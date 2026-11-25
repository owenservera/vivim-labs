// Internal HQ transaction participant, not a remote command or grant issuer.
// No new database: existing meta table and indexProject transaction only.
const fail=code=>{throw Error(code)};
const pos=n=>Number.isSafeInteger(n)&&n>0;
const seq=n=>Number.isSafeInteger(n)&&n>=0;
const scope=s=>typeof s==='string'&&/^[a-f0-9]{64}$/.test(s);
const exact=(o,keys)=>o!==null&&typeof o==='object'&&!Array.isArray(o)&&Object.keys(o).length===keys.length&&keys.every(k=>Object.hasOwn(o,k));
const key=id=>{if(!pos(id))fail('BAD_PROJECT');return 'windows_observation:v1:'+id};
const validFence=f=>exact(f,['scopeDigest','generation','sequence','gap'])&&scope(f.scopeDigest)&&pos(f.generation)&&seq(f.sequence)&&typeof f.gap==='boolean';
const validState=s=>exact(s,['schema','scopeDigest','generation','sequence','revision','wikiPending'])&&s.schema==='vivim.hq.windows-checkpoint/1'&&scope(s.scopeDigest)&&pos(s.generation)&&seq(s.sequence)&&pos(s.revision)&&typeof s.wikiPending==='boolean';
export function readCheckpoint(db,projectId){
 const row=db.prepare('SELECT value FROM meta WHERE key=?').get(key(projectId));
 if(!row)return null;
 const s=JSON.parse(row.value);if(!validState(s))fail('CORRUPT_CHECKPOINT');return s;
}
function requireTxn(db){if(db.isTransaction!==true)fail('EXISTING_TRANSACTION_REQUIRED');}
function save(db,id,state){
 if(!validState(state))fail('INVALID_CHECKPOINT');
 db.prepare('INSERT INTO meta(key,value) VALUES(?,?) ON CONFLICT(key) DO UPDATE SET value=excluded.value').run(key(id),JSON.stringify(state));
}
const same=(a,b)=>validFence(a)&&validFence(b)&&a.scopeDigest===b.scopeDigest&&a.generation===b.generation&&a.sequence===b.sequence&&a.gap===b.gap;
// Called only by incumbent indexer after real complete coverage verification.
// The supplied fence reader is a trusted internal collector port, not user input.
// No filesystem read, new transaction, or async callback inside this participant.
export function commitObservation(db,projectId,{expectedRevision,from,to,mode,readFence,verifyCoverage,complete}){
 requireTxn(db);
 if(!seq(expectedRevision)||!validFence(from)||!validFence(to)||typeof readFence!=='function'||typeof verifyCoverage!=='function'||complete!==true||!['baseline','delta'].includes(mode)||from.scopeDigest!==to.scopeDigest||to.gap)fail('INCOMPLETE_OBSERVATION');
 const old=readCheckpoint(db,projectId);
 if((old?.revision??0)!==expectedRevision)fail('STALE_REVISION');
 if(old&&old.scopeDigest!==to.scopeDigest)fail('SCOPE_CHANGED');
 if(mode==='delta'){
  if(!old||from.gap||old.generation!==from.generation||old.sequence!==from.sequence||to.generation!==from.generation||to.sequence<=from.sequence)fail('DELTA_DISCONTINUITY');
 }else{
  if(from.generation!==to.generation||from.sequence!==to.sequence||from.gap||old&&to.generation<=old.generation)fail('BASELINE_FENCE');
 }
 // The current indexer is a full project scan. It cannot prove that a bounded
 // walk was complete merely by returning zero errors. Require independent
 // collector coverage proof, and check current fence again after proof.
 if(verifyCoverage({projectId,from,to,mode})!==true||!same(readFence(),to))fail('COVERAGE_OR_FENCE_CHANGED');
 if(expectedRevision===Number.MAX_SAFE_INTEGER)fail('REVISION_EXHAUSTED');
 const state={schema:'vivim.hq.windows-checkpoint/1',scopeDigest:to.scopeDigest,generation:to.generation,sequence:to.sequence,revision:expectedRevision+1,wikiPending:true};
 save(db,projectId,state);return state;
}
// After incumbent Wiki owner verifies actual artifact readback at this
// revision, acknowledge inside its existing transaction. Never acknowledge
// a newer invalidation using an older render.
export function acknowledgeWiki(db,projectId,revision,verifyReadback){
 requireTxn(db);const s=readCheckpoint(db,projectId);
 if(!s||!s.wikiPending||s.revision!==revision||typeof verifyReadback!=='function'||verifyReadback({projectId,revision})!==true)fail('STALE_OR_UNVERIFIED_WIKI');
 const next={...s,wikiPending:false};save(db,projectId,next);return next;
}
