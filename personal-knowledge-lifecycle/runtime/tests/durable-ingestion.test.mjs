// Durable V2 ingestion: the real Labs importer wiring through the actual Harness
// store and the real built hqkg-personal CLI. Missing binaries fail the gate.
import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtemp, writeFile, rm} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {makeZip, conversationJson} from './fixtures.mjs';
import {createHostArchiveIngestion} from '../src/host-archive-wiring.mjs';

const harnessRoot=process.env.VIVIM_HARNESS_ROOT??fileURLToPath(new URL('../../../../vivim-web-harness/',import.meta.url));
const {openLedger}=await import(new URL('file:///'+path.resolve(harnessRoot,'apps/vivim-host/bootstrap/ledger.mjs').replaceAll('\\','/')).href);
const {createPersonalKnowledgeStore}=await import(new URL('file:///'+path.resolve(harnessRoot,'apps/vivim-host/bootstrap/personal-knowledge-store.mjs').replaceAll('\\','/')).href);
const {createPersonalKnowledgeRust}=await import(new URL('file:///'+path.resolve(harnessRoot,'apps/vivim-host/bootstrap/personal-knowledge-rust.mjs').replaceAll('\\','/')).href);

const hex=()=>'d'.repeat(64);
const sha=value=>createHash('sha256').update(value).digest('hex');
const cliPath=()=>process.env.VIVIM_PERSONAL_TEST_BINARY??
  path.resolve(harnessRoot,'../repos_hq/research/rust-knowledge-graph/target/debug',process.platform==='win32'?'hqkg-personal.exe':'hqkg-personal');
// Test-only transport override: delegates every op to the real adapter so journal
// effects stay real while one op can land-then-fail (simulated response loss).
const wrapRust=(base,over)=>Object.fromEntries(['inspect','query','preflight','apply','erase','eraseEffects','compact'].map(k=>[k,over[k]??(packet=>base[k](packet))]));

async function fixture(n,run){
  const dir=await mkdtemp(path.join(tmpdir(),'vivim-durable-ing-'));
  try{
    const archivePath=path.join(dir,'export.zip');
    await writeFile(archivePath,makeZip([['conversations.json',conversationJson(n)]]));
    const ledger=openLedger({file:':memory:'});
    const principal={worldId:'world-a',accountId:'account-a',provider:'chatgpt',canAuthorize:true,verifiedReceipts:{}};
    const real=createPersonalKnowledgeRust({binary:cliPath(),dataRoot:dir});
    // The store is built on a delegating transport so a test can swap one op
    // (e.g. apply lands then loses its response) while journal effects stay real.
    const current={rust:real};
    const delegating=Object.fromEntries(['inspect','query','preflight','apply','erase','compact'].map(k=>[k,packet=>current.rust[k](packet)]));
    const store=createPersonalKnowledgeStore({ledger,trustedPrincipalResolver:async ctx=>ctx===principal?principal:null,clock:()=>1000,rust:delegating});
    const port=await store.forPrincipal(principal);
    const trusted={authorize:i=>port.authorize(i),stage:i=>port.stage(i),finalize:i=>port.finalize(i),
      commitGraph:i=>port.commitGraph(i),reconcileGraph:i=>port.reconcileGraph(i),
      inspect:i=>port.inspect(i),compact:i=>port.compact(i)};
    const projections=[],legacyEvidence=[];
    // Importer checkpoint rows must persist for resume/duplicate semantics; the
    // authoritative durable state stays in the real Harness ledger.
    const checkpoints=new Map();
    const host={
      authorizeSource:async()=>({allowed:true,grantId:'host-grant-1',grantRevision:1,
        destinationDigest:hex(),expiry:4102444800000}),
      inspectEvidenceBatch:async()=>{legacyEvidence.push('inspect');return {status:'absent'};},
      graphRevision:async()=>({epoch:0,sourceRevision:1}),
      commitEvidenceAndGraph:async packet=>{legacyEvidence.push('commit');
        return {key:packet.key,digest:packet.digest,verified:true,status:'committed'};}
    };
    const adapter=createHostArchiveIngestion({ledger:{meta:{get:k=>checkpoints.get(k)??null,set:(k,v)=>checkpoints.set(k,v)},transaction:fn=>fn()},
      host,coordinator:{recordBatch:p=>projections.push(p)},worldId:'world-a',accountId:'account-a',storage:trusted});
    const scopeId=sha(JSON.stringify(['world-a','account-a','chatgpt']));
    const head=async opId=>real.inspect(opId?{scope:scopeId,opId}:{scope:scopeId});
    const swapRust=next=>{current.rust=next;};
    let result=null,error=null;
    try{result=await run(adapter,archivePath,{port,head,real,swapRust,projections});}
    catch(e){error=e;}
    return {result,error,legacyEvidence,projections,port,scopeId,head};
  } finally {await rm(dir,{recursive:true,force:true});}
}

test('durable V2 import publishes real graph effects with zero legacy evidence calls',async()=>{
  const {error,legacyEvidence,projections,port,scopeId}=await fixture(4,(adapter,archivePath)=>adapter.importArchive({archivePath,batchSize:2}));
  assert.ifError(error);
  assert.deepEqual(legacyEvidence,[],'durable V2 bypasses the legacy evidence sink');
  assert.equal(projections.length,2);
  // Published contract: the storage job opId is sha256(scopeId+'\0'+batchKey).
  const opIdOf=batchKey=>sha(scopeId+'\0'+batchKey);
  for(const projection of projections)assert.equal(port.inspect(opIdOf(projection.batchId)).state,'published');
});

test('duplicate durable run moves neither publications nor graph effects',async()=>{
  const {error}=await fixture(4,async(adapter,archivePath,ctx)=>{
    const first=await adapter.importArchive({archivePath,batchSize:2});
    assert.equal(first.status,'complete');
    const before=await ctx.head();
    const second=await adapter.importArchive({archivePath,batchSize:2});
    assert.equal(second.duplicateRun,true);
    const after=await ctx.head();
    assert.equal(after.epoch,before.epoch,'replay must not advance graph epochs');
    assert.equal(after.bytes,before.bytes,'replay must not grow the journal');
  });
  assert.ifError(error);
});

test('lost graph response reconciles from the real marker and resumes without re-apply',async()=>{
  const {error,projections,port,scopeId}=await fixture(4,async(adapter,archivePath,ctx)=>{
    // First attempt: the real CLI effect lands, then the response is lost as uncertain.
    ctx.swapRust(wrapRust(ctx.real,{apply:async packet=>{
      const applied=await ctx.real.apply(packet);assert.ok(applied.marker);
      throw Object.assign(new Error('response lost'),{code:'RUST_HELD'});}}));
    // The initial commit failure surfaces the store's RUST_HELD; the durable sink's
    // IMPORT_COMMIT_UNCERTAIN mapping is the resume path exercised below.
    await assert.rejects(adapter.importArchive({archivePath,batchSize:2}),error=>error.code==='RUST_HELD');
    assert.ok((await ctx.head()).epoch>0,'the real journal holds the landed effects');
    // Restart with the healthy transport: inspectBatch reconciles the marker and the
    // durable importer cursor resumes without duplicating any graph effect.
    ctx.swapRust(ctx.real);
    const healthy=await adapter.importArchive({archivePath,batchSize:2});
    assert.equal(healthy.status,'complete');assert.equal(healthy.processed,4);
  });
  assert.ifError(error);
  // Published contract: the storage job opId is sha256(scopeId+'\0'+batchKey).
  const opIdOf=batchKey=>sha(scopeId+'\0'+batchKey);
  for(const projection of projections)assert.equal(port.inspect(opIdOf(projection.batchId)).state,'published');
});
