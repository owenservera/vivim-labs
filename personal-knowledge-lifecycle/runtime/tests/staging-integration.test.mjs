// Real cross-repo staging integration: Labs importer wiring -> actual Harness
// Host store/ledger. Binary/CLI absence fails the gate; it is never skipped.
import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtemp, writeFile, rm} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {makeZip, conversationJson} from './fixtures.mjs';
import {createHostArchiveIngestion} from '../src/host-archive-wiring.mjs';

const harnessRoot=process.env.VIVIM_HARNESS_ROOT??fileURLToPath(new URL('../../../../vivim-web-harness/',import.meta.url));
const {openLedger}=await import(new URL('file:///'+path.resolve(harnessRoot,'apps/vivim-host/bootstrap/ledger.mjs').replaceAll('\\','/')).href);
const {createPersonalKnowledgeStore}=await import(new URL('file:///'+path.resolve(harnessRoot,'apps/vivim-host/bootstrap/personal-knowledge-store.mjs').replaceAll('\\','/')).href);

const hex=()=>'d'.repeat(64);
async function fixture(n,run,grantFields={}){
  const dir=await mkdtemp(path.join(tmpdir(),'vivim-staging-int-'));
  try{
    const archivePath=path.join(dir,'export.zip');
    await writeFile(archivePath,makeZip([['conversations.json',conversationJson(n)]]));
    const ledger=openLedger({file:':memory:'});
    const principal={worldId:'world-a',accountId:'account-a',provider:'chatgpt',canAuthorize:true,verifiedReceipts:{}};
    const store=createPersonalKnowledgeStore({ledger,trustedPrincipalResolver:async ctx=>ctx===principal?principal:null,clock:()=>1000});
    const port=await store.forPrincipal(principal);
    const commits=[],projections=[],staged=[];
    // meta values are JSON strings per host-ledger-port.mjs:6-13; a Map persists them.
    const rows=new Map();
    const checkpointLedger={meta:{get:k=>rows.get(k)??null,set:(k,v)=>rows.set(k,v)},transaction:fn=>fn()};
    const host={
      authorizeSource:async()=>({allowed:true,grantId:'host-grant-1',grantRevision:1,
        destinationDigest:hex('dest'),expiry:4102444800000,...grantFields}),
      inspectEvidenceBatch:async()=>({status:'absent'}),
      graphRevision:async()=>({epoch:0,sourceRevision:1}),
      commitEvidenceAndGraph:async packet=>{commits.push(packet);
        return {key:packet.key,digest:packet.digest,verified:true,status:'committed'};}
    };
    const storage={authorize:async input=>port.authorize(input),
      stage:async input=>{staged.push(input);return port.stage(input);}};
    const adapter=createHostArchiveIngestion({ledger:checkpointLedger,
      host,coordinator:{recordBatch:p=>projections.push(p)},worldId:'world-a',accountId:'account-a',storage});
    let result=null,error=null;
    try{result=await run(adapter,archivePath,{staged,commits,projections,port,principal,store});}
    catch(e){error=e;}
    return {result,error,staged,commits,projections,port};
  } finally {await rm(dir,{recursive:true,force:true});}
}

test('full import stages durably through the real Host store before any admission',async()=>{
  const {error,staged,commits,projections,port}=await fixture(5,(adapter,archivePath)=>adapter.importArchive({archivePath,batchSize:2}));
  assert.ifError(error);
  assert.equal(staged.length,3);assert.equal(commits.length,3);assert.equal(projections.length,3);
  for(const call of staged){
    assert.equal(port.inspect(call.opId).state,'staging','staged jobs stay invisible until Host publication');
    await assert.rejects(async()=>port.read(call.opId),{code:'NOT_PUBLISHED'});
  }
});

test('duplicate run neither restages nor recommits against the real store',async()=>{
  const {error,staged,commits}=await fixture(4,async (adapter,archivePath)=>{
    const first=await adapter.importArchive({archivePath,batchSize:2});
    assert.equal(first.status,'complete');
    const second=await adapter.importArchive({archivePath,batchSize:2});
    assert.equal(second.duplicateRun,true);
  });
  assert.ifError(error);
  assert.equal(staged.length,2,'duplicate run must not restage');
  assert.equal(commits.length,2,'duplicate run must not recommit');
});

for(const [label,fields,code] of [
  ['missing destination digest',{destinationDigest:undefined},'STAGING_GRANT_INCOMPLETE'],
  ['bad digest format',{destinationDigest:'not-a-digest'},'INVALID_CAPABILITY'],
  ['stale expiry',{expiry:500},'INVALID_CAPABILITY'],
]) test(`Host grant ${label} fails closed before any commit`,async()=>{
  const {error,staged,commits}=await fixture(2,(adapter,archivePath)=>adapter.importArchive({archivePath,batchSize:2}),fields);
  assert.match(String(error?.code),new RegExp(code));
  assert.equal(commits.length,0);assert.ok(staged.length<=1,'no admission after failed capability issuance');
});
test('trusted storage stage failure aborts the import before any admission',async()=>{
  const dir=await mkdtemp(path.join(tmpdir(),'vivim-stage-fail-'));
  try{
    const archivePath=path.join(dir,'export.zip');
    await writeFile(archivePath,makeZip([['conversations.json',conversationJson(2)]]));
    const principal={worldId:'w',accountId:'a',provider:'chatgpt',canAuthorize:true,verifiedReceipts:{}};
    const store=createPersonalKnowledgeStore({ledger:openLedger({file:':memory:'}),
      trustedPrincipalResolver:async()=>principal,clock:()=>1000});
    const port=await store.forPrincipal(principal);
    let commits=0,staged=0;const rows=new Map();
    const adapter=createHostArchiveIngestion({ledger:{meta:{get:k=>rows.get(k)??null,set:(k,v)=>rows.set(k,v)},transaction:fn=>fn()},
      host:{authorizeSource:async()=>({allowed:true,grantId:'g',grantRevision:1,destinationDigest:hex('d'),expiry:4102444800000}),
        inspectEvidenceBatch:async()=>({status:'absent'}),graphRevision:async()=>({epoch:0,sourceRevision:1}),
        commitEvidenceAndGraph:async()=>{commits++;return {verified:true,status:'committed'};}},
      coordinator:{recordBatch:()=>{}},worldId:'w',accountId:'a',
      storage:{authorize:async i=>port.authorize(i),stage:async()=>{staged++;throw Object.assign(Error('HOST_STAGE_REFUSED'),{code:'HOST_STAGE_REFUSED'});}}});
    await assert.rejects(adapter.importArchive({archivePath,batchSize:2}),{code:'HOST_STAGE_REFUSED'});
    assert.equal(commits,0,'staging refusal must precede evidence admission');
    assert.equal(staged,1);
  } finally {await rm(dir,{recursive:true,force:true});}
});
