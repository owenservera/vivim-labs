#!/usr/bin/env node
// Synthetic-only throughput measurement; never uses personal account data.
import {mkdtemp,writeFile,rm} from 'node:fs/promises';
import {tmpdir} from 'node:os';import path from 'node:path';
import {deflateRawSync} from 'node:zlib';
import {crc32Update} from '../src/zip-reader.mjs';
import {importChatGptExport} from '../src/chatgpt-import.mjs';
const count=Math.max(1,Math.min(10000,Number(process.argv[2])||1000));
const obj=i=>({id:'thread-'+i,title:'Synthetic '+i,create_time:1720000000+i,mapping:{
 a:{parent:null,children:['b'],message:{id:'user-'+i,author:{role:'user'},content:{parts:['Question '+i]}}},
 b:{parent:'a',children:[],message:{id:'assistant-'+i,author:{role:'assistant'},content:{parts:['Answer '+i]}}}}});
// Construct one synthetic ZIP file directly for deterministic local benchmarking.
const data=Buffer.from('['+Array.from({length:count},(_,i)=>JSON.stringify(obj(i))).join(',')+']');
const encoded=deflateRawSync(data);const fn=Buffer.from('conversations.json');
const crc=(crc32Update(0xffffffff,data)^0xffffffff)>>>0;
const local=Buffer.alloc(30),central=Buffer.alloc(46),tail=Buffer.alloc(22);
local.writeUInt32LE(0x04034b50);local.writeUInt16LE(20,4);local.writeUInt16LE(8,8);local.writeUInt32LE(crc,14);local.writeUInt32LE(encoded.length,18);local.writeUInt32LE(data.length,22);local.writeUInt16LE(fn.length,26);
central.writeUInt32LE(0x02014b50);central.writeUInt16LE(20,4);central.writeUInt16LE(20,6);central.writeUInt16LE(8,10);central.writeUInt32LE(crc,16);central.writeUInt32LE(encoded.length,20);central.writeUInt32LE(data.length,24);central.writeUInt16LE(fn.length,28);
const dirOffset=local.length+fn.length+encoded.length;tail.writeUInt32LE(0x06054b50);tail.writeUInt16LE(1,8);tail.writeUInt16LE(1,10);tail.writeUInt32LE(central.length+fn.length,12);tail.writeUInt32LE(dirOffset,16);
const file=Buffer.concat([local,fn,encoded,central,fn,tail]);
const dir=await mkdtemp(path.join(tmpdir(),'vivim-bench-'));const archivePath=path.join(dir,'synthetic.zip');
try{
 await writeFile(archivePath,file);
 const state=new Map(),receipts=new Map();const store={read:k=>structuredClone(state.get(k)??null),transact(k,fn){const v=fn(this.read(k));state.set(k,structuredClone(v));return v;}};
 const sink={inspectBatch:async k=>receipts.has(k)?{status:'committed',receipt:receipts.get(k)}:{status:'absent'},commitBatch:async p=>{const r={key:p.key,digest:p.digest,verified:true,status:'committed'};receipts.set(p.key,r);return r;}};
 const start=performance.now();const result=await importChatGptExport({archivePath,worldId:'benchmark',accountId:'synthetic',store,sink,
   authorize:async()=>({allowed:true,grantId:'synthetic-owner'}),batchSize:16});
 const seconds=(performance.now()-start)/1000;
 process.stdout.write(JSON.stringify({kind:'synthetic-only',conversations:count,batches:result.batches,
  archiveBytes:file.length,uncompressedBytes:data.length,elapsedSeconds:Number(seconds.toFixed(3)),
  conversationsPerSecond:Number((count/seconds).toFixed(1)),heapMB:Number((process.memoryUsage().heapUsed/1024/1024).toFixed(1))},null,2)+'\n');
}finally{await rm(dir,{recursive:true,force:true});}
