import test from 'node:test';import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {mkdtemp,writeFile,rm} from 'node:fs/promises';import {tmpdir} from 'node:os';import path from 'node:path';
import {makeZip,conversationJson} from './fixtures.mjs';
import {createHostArchiveIngestion} from '../src/host-archive-wiring.mjs';
test('injected Host + ledger + Rust graph proposals form a receipt-linked full path',async()=>{
 const dir=await mkdtemp(path.join(tmpdir(),'vivim-wired-'));const archivePath=path.join(dir,'export.zip');
 try{
  await writeFile(archivePath,makeZip([['conversations.json',conversationJson(3)]]));
  const data=new Map(),written=new Map(),accounted=[];
  const ledger={meta:{get:key=>data.get(key)??null,set:(key,v)=>data.set(key,v)},transaction:fn=>fn()};
  let revision=0;
  const host={
   authorizeSource:async()=>({allowed:true,grantId:'owner-allow'}),
   inspectEvidenceBatch:async({key})=>written.has(key)?{status:'committed',receipt:written.get(key)}:{status:'absent'},
   graphRevision:async()=>({epoch:revision,sourceRevision:1}),
   async commitEvidenceAndGraph(packet){
     assert.equal(packet.graphProposal.targetRust,'hqkg.scoped-record/1');
     assert.equal(packet.graphProposal.expected.graphEpoch,revision);
     const owner='pk-'+createHash('sha256').update(JSON.stringify(['world','account','chatgpt'])).digest('hex');
     assert.equal(packet.graphProposal.namespace.owner,owner);
     assert.ok(packet.graphProposal.changes.ops.every(op=>(op.node??op.edge).id.startsWith(`personal:${Buffer.byteLength(owner)}:${owner}/`)));
     revision++;
     const receipt={key:packet.key,digest:packet.digest,verified:true,status:'committed'};
     written.set(packet.key,receipt);return receipt;
   }
  };
  const coordinator={recordBatch:packet=>accounted.push(packet)};
  const adapter=createHostArchiveIngestion({ledger,host,coordinator,worldId:'world',accountId:'account'});
  const result=await adapter.importArchive({archivePath,batchSize:2});
  assert.equal(result.processed,3);assert.equal(accounted.length,2);
  assert.equal(written.size,2);assert.equal(revision,2);
 }finally{await rm(dir,{recursive:true,force:true});}
});
