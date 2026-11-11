import test from 'node:test';import assert from 'node:assert/strict';
import {createHostFastLaneIntake,normalizeFastLaneSnapshot} from '../src/fastlane-snapshot.mjs';
const bootstrap={method:'studio-verified',approved:true,verified:true,effectRef:'studio-receipt-1'};
const custom={method:'owner-submitted',approved:true};
const setup=()=>{
 const meta=new Map(),written=new Map(),recorded=[];let calls=0,granted=true;
 const ledger={meta:{get:k=>meta.get(k)??null,set:(k,v)=>meta.set(k,v)},transaction:fn=>fn()};
 const host={authorizeSource:async()=>({allowed:granted,grantId:'owner-1'}),
  inspectEvidenceBatch:async({key})=>written.has(key)?{status:'committed',receipt:written.get(key)}:{status:'absent'},
  async commitEvidenceCandidates(p){calls++;const r={key:p.key,digest:p.digest,status:'committed',verified:true};written.set(p.key,r);return r;}};
 const coordinator={recordBatch:x=>recorded.push(x)};
 return {intake:createHostFastLaneIntake({ledger,host,coordinator,worldId:'world',accountId:'account'}),
   recorded,get calls(){return calls;},setGranted:v=>{granted=v;},host,written};
};
test('custom instructions become source-backed candidates with idempotent Host receipt',async()=>{
 const f=setup();const request={kind:'custom_instructions',text:'Prefer mathematics and physics',capture:custom,observedAt:'2026-11-11T12:00:00.000Z'};
 const a=await f.intake.ingest(request);assert.equal(a.status,'complete');assert.equal(f.calls,1);
 assert.equal(f.recorded.length,1);assert.equal(f.recorded[0].lane,'profile');
 const b=await f.intake.ingest(request);assert.equal(b.duplicateRun,true);assert.equal(f.calls,1);
});
test('model memory summaries remain candidates, never owner-confirmed',()=>{
 const r=normalizeFastLaneSnapshot({accountId:'a',kind:'memory_summary',text:'A user may prefer science',capture:custom});
 assert.equal(r.state,'candidate');assert.equal(r.authority,'model-summarized');
});
test('bootstrap evidence requires verified Studio send receipt',async()=>{
 assert.throws(()=>normalizeFastLaneSnapshot({accountId:'a',kind:'bootstrap_f02',text:'summary',capture:custom}),{code:'SNAPSHOT_BOOTSTRAP_RECEIPT_REQUIRED'});
 const r=normalizeFastLaneSnapshot({accountId:'a',kind:'bootstrap_f02',text:'summary',capture:bootstrap});
 assert.equal(r.provenance.verified,true);
});
test('revocation and uncertain journal fail closed',async()=>{
 const f=setup();f.setGranted(false);
 await assert.rejects(f.intake.ingest({kind:'memory_summary',text:'Summary',capture:custom}),{code:'FASTLANE_GRANT_REQUIRED'});
 f.setGranted(true);
 f.host.inspectEvidenceBatch=async()=>({status:'uncertain'});
 await assert.rejects(f.intake.ingest({kind:'memory_summary',text:'Summary',capture:custom}),{code:'FASTLANE_UNCERTAIN'});
 assert.equal(f.calls,0);
});
test('per-account identities cannot collide',()=>{
 const a=normalizeFastLaneSnapshot({accountId:'a',kind:'custom_instructions',text:'x',capture:custom});
 const b=normalizeFastLaneSnapshot({accountId:'b',kind:'custom_instructions',text:'x',capture:custom});
 assert.notEqual(a.sourceId,b.sourceId);assert.notEqual(a.id,b.id);
});
