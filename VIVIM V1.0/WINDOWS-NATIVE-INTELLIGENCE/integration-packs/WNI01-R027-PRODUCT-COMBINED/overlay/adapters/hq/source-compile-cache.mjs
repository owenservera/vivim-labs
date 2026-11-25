// @hq:context Keep exact per-file native compilation artifacts in the existing resident exchange.
// @hq:invariant Fresh HQ evidence and live generation must be checked before reuse.
// @hq:why Skip repeated extraction and binding while keeping graph/index provenance current.
import {createHash} from 'node:crypto';
const stores=new WeakMap();
const MAX_ENTRIES=128,MAX_BYTES=2*1024*1024;
const digest=s=>createHash('sha256').update(s,'utf8').digest('hex');
export function sourceCache({exchange,hq,project,path,binary,extractAnnotations,parseAnnotationV2,e}){
 if(!exchange||typeof exchange.batch!=='function'||!Number.isSafeInteger(exchange.generation)||exchange.generation<1||
    typeof e.devSourceId!=='string'||!e.devSourceId||typeof e.sourceId!=='string'||typeof e.currentHash!=='string'||!/^[a-f0-9]{64}$/.test(e.currentHash)||typeof e.text!=='string')return null;
 const generation=exchange.generation;
 let s=stores.get(exchange);
 if(!s||s.hq!==hq||s.extract!==extractAnnotations||s.parse!==parseAnnotationV2||s.binary!==binary||s.generation!==generation){
  s={hq,extract:extractAnnotations,parse:parseAnnotationV2,binary,generation,entries:new Map(),bytes:0};
  stores.set(exchange,s);
 }
 const key=JSON.stringify([project,path,e.devSourceId,e.sourceId,e.currentHash,e.lineEnd,digest(e.text)]);
 const remove=k=>{const old=s.entries.get(k);if(old){s.bytes-=old.bytes;s.entries.delete(k);}};
 const clear=()=>{s.entries.clear();s.bytes=0;};
 return {
  async read(){
   const v=s.entries.get(key);if(!v)return null;
   try{const r=await exchange.batch(['status']);if(r.length!==1||r[0]!=='OK generation='+generation||exchange.generation!==generation)throw Error('V1_CACHE_GENERATION_CHANGED');}
   catch(err){clear();throw err;}
   s.entries.delete(key);s.entries.set(key,v);
   return structuredClone(v.value);
  },
  write(value){
   if(exchange.generation!==generation||value.protocolGeneration!==generation||value.sourceDigest!==e.currentHash||value.sourceId!==e.sourceId||!value.declarations.length)return;
   // Do not store prior index watermarks, project revisions or source text.
   const artifact={declarations:value.declarations,diagnostics:value.diagnostics};
   const bytes=Buffer.byteLength(key)+Buffer.byteLength(JSON.stringify(artifact));
   if(bytes>MAX_BYTES)return;
   remove(key);
   while(s.entries.size>=MAX_ENTRIES||s.bytes+bytes>MAX_BYTES)remove(s.entries.keys().next().value);
   s.entries.set(key,{value:structuredClone(artifact),bytes});s.bytes+=bytes;
  }
 };
}
