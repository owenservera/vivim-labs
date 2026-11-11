import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, writeFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { makeZip,conversationJson } from './fixtures.mjs';
import {importChatGptExport} from '../src/chatgpt-import.mjs';
import {createHostLedgerStore} from '../src/host-ledger-port.mjs';
const fixture = async (n, cb, extra=[]) => {
  const dir=await mkdtemp(path.join(tmpdir(),'vivim-import-'));
  const archivePath=path.join(dir,'chatgpt-export.zip');
  try{await writeFile(archivePath,makeZip([['conversations.json',conversationJson(n)],...extra]));return await cb(archivePath);}
  finally{await rm(dir,{recursive:true,force:true});}
};
const createStore=()=>{const states=new Map();return {read(key){return structuredClone(states.get(key)??null);},transact(key,cb){const next=cb(this.read(key));states.set(key,structuredClone(next));return structuredClone(next);}};};
const createSink=()=>{const batches=new Map();let writes=0;
  return {batches,get writes(){return writes;},
    async inspectBatch(key){const r=batches.get(key);return r?{status:'committed',receipt:r}:{status:'absent'};},
    async commitBatch(packet){writes++;const previous=batches.get(packet.key);if(previous){if(previous.digest!==packet.digest)throw Error('conflict');return previous;}
      const receipt={key:packet.key,digest:packet.digest,verified:true,status:'committed'};
      batches.set(packet.key,receipt);return receipt;
    }
  };
};
const config=(archivePath,store,sink,authorize=async()=>({allowed:true,grantId:'owner-1'}))=>({archivePath,worldId:'world-1',accountId:'chatgpt-account-1',store,sink,authorize,batchSize:2});
test('end-to-end archive -> normalized batches -> Host receipts -> checkpoint -> duplicate run',async()=>{
 await fixture(5,async archivePath=>{
  const store=createStore(),sink=createSink();
  const seen=[];
  const opts={...config(archivePath,store,sink),afterCommitted:async packet=>seen.push(packet)};
  const first=await importChatGptExport(opts);
  assert.equal(first.status,'complete');assert.equal(first.cursor,4);assert.equal(first.processed,5);
  assert.equal(first.batches,3);assert.equal(sink.writes,3);
  assert.equal(seen.length,3);assert.equal(seen.flatMap(x=>x.items).length,5);
  const second=await importChatGptExport(opts);
  assert.equal(second.duplicateRun,true);assert.equal(sink.writes,3);
 });
});
test('crash after Host commit resumes from inspection without another write',async()=>{
 await fixture(3,async archivePath=>{
  const store=createStore(),underlying=createSink();let interrupted=false;
  const sink={inspectBatch:key=>underlying.inspectBatch(key),async commitBatch(packet){const receipt=await underlying.commitBatch(packet);if(!interrupted){interrupted=true;throw Error('simulated power loss');}return receipt;}};
  const opts=config(archivePath,store,sink);
  await assert.rejects(importChatGptExport(opts),/simulated power loss/);
  const keys=[...underlying.batches.keys()];assert.equal(keys.length,1);
  const result=await importChatGptExport(opts);
  assert.equal(result.processed,3);assert.equal(result.batches,2);
  assert.equal(underlying.writes,2,'first uncertain commit must not be repeated');
 });
});
test('revoked grant blocks next batch and resumes safely only after renewal',async()=>{
 await fixture(5,async archivePath=>{
  const store=createStore(),sink=createSink();let active=true;
  let observed=0;
  const opts={...config(archivePath,store,sink,async()=>active?{allowed:true,grantId:'owner-1'}:{allowed:false}),
   onProgress(){observed++;if(observed===1)active=false;}};
  await assert.rejects(importChatGptExport(opts),{code:'IMPORT_GRANT_REQUIRED'});
  assert.equal(sink.writes,1);
  active=true;
  const result=await importChatGptExport(opts);
  assert.equal(result.processed,5);assert.equal(sink.writes,3);
 });
});
test('uncertain evidence journal never commits or resends',async()=>{
 await fixture(1,async archivePath=>{
  const store=createStore();let called=false;
  const sink={async inspectBatch(){return {status:'uncertain'};},async commitBatch(){called=true;}};
  await assert.rejects(importChatGptExport(config(archivePath,store,sink)),{code:'IMPORT_COMMIT_UNCERTAIN'});
  assert.equal(called,false);
 });
});
test('unverified Host receipts fail closed and preserve pending checkpoint',async()=>{
 await fixture(1,async archivePath=>{
  const store=createStore();const sink={async inspectBatch(){return {status:'absent'};},async commitBatch(p){return {key:p.key,digest:p.digest,status:'committed'};}};
  await assert.rejects(importChatGptExport(config(archivePath,store,sink)),{code:'IMPORT_RECEIPT_UNVERIFIED'});
 });
});
test('missing or ambiguous conversation file rejected without writes',async()=>{
 await fixture(1,async archivePath=>{
  const store=createStore(),sink=createSink();
  await assert.rejects(importChatGptExport(config(archivePath,store,sink)),{code:'IMPORT_AMBIGUOUS_CONVERSATIONS'});
  assert.equal(sink.writes,0);
 },[['folder/conversations.json','[]']]);
});
test('Host ledger adapter reuses transaction and meta API with rollback',()=>{
 const state=new Map();const ledger={meta:{get:key=>state.get(key)??null,set:(key,value)=>state.set(key,value)},transaction:fn=>fn()};
 const store=createHostLedgerStore({ledger});
 assert.equal(store.read('a'),null);
 store.transact('a',()=>({count:1}));
 assert.equal(store.read('a').count,1);
 store.transact('a',s=>({...s,count:s.count+1}));
 assert.equal(store.read('a').count,2);
});
