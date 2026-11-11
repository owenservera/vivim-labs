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
