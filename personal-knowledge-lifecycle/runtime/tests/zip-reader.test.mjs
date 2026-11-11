import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtemp,writeFile,rm} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import path from 'node:path';
import {makeZip} from './fixtures.mjs';
import {openZipArchive} from '../src/zip-reader.mjs';
const withZip=async (buf,fn)=>{const dir=await mkdtemp(path.join(tmpdir(),'vivim-zip-'));const p=path.join(dir,'export.zip');try{await writeFile(p,buf);return await fn(p);}finally{await rm(dir,{recursive:true,force:true});}};
test('reads stored and deflated entries with crc verification',async()=>{
 await withZip(makeZip([['conversations.json','[{"id":"x"}]',8],['notes.txt','hello',0]]),async p=>{
  const zip=await openZipArchive(p);try{
   assert.equal(zip.entries.length,2);
   const data=[];for await(const c of await zip.entryStream(zip.entries[0]))data.push(c);
   assert.equal(Buffer.concat(data).toString(),'[{"id":"x"}]');
   const second=[];for await(const c of await zip.entryStream(zip.entries[1]))second.push(c);
   assert.equal(Buffer.concat(second).toString(),'hello');
  }finally{await zip.close();}
 });
});
test('rejects path traversal and case-colliding entries',async()=>{
 await withZip(makeZip([['../conversations.json','[]']]),async p=>await assert.rejects(openZipArchive(p),{code:'ZIP_UNSAFE_PATH'}));
 await withZip(makeZip([['a.txt','x'],['A.TXT','y']]),async p=>await assert.rejects(openZipArchive(p),{code:'ZIP_DUPLICATE_NAME'}));
});
test('rejects oversized expansion and CRC corruption',async()=>{
 await withZip(makeZip([['conversations.json','x'.repeat(20000)]]),async p=>await assert.rejects(openZipArchive(p,{maxExpansionRatio:3}),{code:'ZIP_BOMB_LIMIT'}));
 await withZip(makeZip([['conversations.json','hello']],{corruptCrc:true}),async p=>{
  const zip=await openZipArchive(p);try{await assert.rejects(async()=>{for await(const _ of await zip.entryStream(zip.entries[0])){}},{code:'ZIP_CRC_MISMATCH'});}finally{await zip.close();}
 });
});
test('rejects truncated EOCD',async()=>{
 await withZip(makeZip([['a.txt','hello']]).subarray(0,-8),async p=>await assert.rejects(openZipArchive(p),{code:'ZIP_END_MISSING'}));
});
test('reads ZIP64 central sizes and ZIP64 end directory',async()=>{
 const data=Buffer.from('hello zip64');const name=Buffer.from('conversations.json');
 const crc=(await import('../src/zip-reader.mjs')).crc32Update(0xffffffff,data)^0xffffffff;
 const local=Buffer.alloc(30);local.writeUInt32LE(0x04034b50,0);local.writeUInt16LE(45,4);
 local.writeUInt32LE(crc>>>0,14);local.writeUInt32LE(data.length,18);local.writeUInt32LE(data.length,22);local.writeUInt16LE(name.length,26);
 const extra=Buffer.alloc(28);extra.writeUInt16LE(1,0);extra.writeUInt16LE(24,2);
 extra.writeBigUInt64LE(BigInt(data.length),4);extra.writeBigUInt64LE(BigInt(data.length),12);extra.writeBigUInt64LE(0n,20);
 const cd=Buffer.alloc(46);cd.writeUInt32LE(0x02014b50,0);cd.writeUInt16LE(45,4);cd.writeUInt16LE(45,6);
 cd.writeUInt32LE(crc>>>0,16);cd.writeUInt32LE(0xffffffff,20);cd.writeUInt32LE(0xffffffff,24);
 cd.writeUInt16LE(name.length,28);cd.writeUInt16LE(extra.length,30);cd.writeUInt32LE(0xffffffff,42);
 const dir=Buffer.concat([cd,name,extra]);const dirOffset=local.length+name.length+data.length;
 const zip64=Buffer.alloc(56);zip64.writeUInt32LE(0x06064b50,0);zip64.writeBigUInt64LE(44n,4);zip64.writeUInt16LE(45,12);zip64.writeUInt16LE(45,14);
 zip64.writeBigUInt64LE(1n,24);zip64.writeBigUInt64LE(1n,32);zip64.writeBigUInt64LE(BigInt(dir.length),40);zip64.writeBigUInt64LE(BigInt(dirOffset),48);
 const locator=Buffer.alloc(20);locator.writeUInt32LE(0x07064b50,0);locator.writeBigUInt64LE(BigInt(dirOffset+dir.length),8);locator.writeUInt32LE(1,16);
 const tail=Buffer.alloc(22);tail.writeUInt32LE(0x06054b50,0);tail.writeUInt16LE(0xffff,8);tail.writeUInt16LE(0xffff,10);
 tail.writeUInt32LE(0xffffffff,12);tail.writeUInt32LE(0xffffffff,16);
 await withZip(Buffer.concat([local,name,data,dir,zip64,locator,tail]),async p=>{
  const zip=await openZipArchive(p);try{
   assert.equal(zip.entries[0].size,data.length);
   const chunks=[];for await(const c of await zip.entryStream(zip.entries[0]))chunks.push(c);
   assert.deepEqual(Buffer.concat(chunks),data);
  }finally{await zip.close();}
 });
});
