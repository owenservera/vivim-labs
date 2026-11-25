// @hq:context Reuse exact native V1 bind results within one resident exchange.
// @hq:invariant No cache entry grants authority, replaces source readback, or crosses a runtime restart.
// @hq:why Avoid repeating identical line-by-line native exchanges on development refreshes.
const readonly=c=>c==='definitions'||c==='status'||/^describe [a-z][a-z0-9._-]*$/.test(c);
const isBind=c=>c.startsWith('bind\t')&&c.split('\t').length===7;
function valid(command,response,generation){
 if(typeof response!=='string'||!response.startsWith('OK '))return false;
 const c=command.split('\t'),r=response.slice(3).split('\t');
 return r.length===10&&r[0]==='vivim.meta-vlang.source/1'&&r[1]===String(generation)&&c[1]===r[1]&&/^[1-9]\d*$/.test(r[2])&&Number.isSafeInteger(Number(r[2]))&&r[3]===c[2]&&r[4]===c[3]&&r[5]===c[4]&&r[6]===c[5]&&r[7]==='unbound'&&r[8]==='source-assertion'&&r[9]===c[6];
}
export function createBindCache(generation,send,{maxEntries=1024,maxBytes=2*1024*1024}={}){
 if(!Number.isSafeInteger(generation)||generation<1||typeof send!=='function'||!Number.isSafeInteger(maxEntries)||maxEntries<1||maxEntries>4096||!Number.isSafeInteger(maxBytes)||maxBytes<1||maxBytes>8*1024*1024)throw Error('SDK_CACHE_CONFIG');
 const entries=new Map();let bytes=0,disabled=false,hits=0,misses=0,checks=0;
 function clear(){entries.clear();bytes=0;}
 function remove(k){const e=entries.get(k);if(e){entries.delete(k);bytes-=e.bytes;}}
 function remember(k,v){
  const n=Buffer.byteLength(k)+Buffer.byteLength(v);
  if(n>maxBytes)return;
  remove(k);
  while(entries.size>=maxEntries||bytes+n>maxBytes)remove(entries.keys().next().value);
  entries.set(k,{value:v,bytes:n});bytes+=n;
 }
 async function batch(commands){
  // Called solely inside incumbent resident batch's serialized busy guard.
  // Unknown/mutating commands disable reuse for the remainder of this process.
  if(disabled||!commands.length||!commands.every(isBind)){
   if(commands.some(c=>!readonly(c))){disabled=true;clear();}
   const out=[];for(const c of commands)out.push(await send(c));return out;
  }
  // Cache hits require a real live generation check, once per batch, not one
  // round-trip for every bind. Cold results are checked by native bind itself.
  if(commands.some(c=>entries.has(c))){
   checks++;
   if(await send('status')!=='OK generation='+generation){disabled=true;clear();throw Error('SDK_CACHE_GENERATION_CHANGED');}
  }
  const out=[],staged=[];
  try{
   for(const c of commands){
    const e=entries.get(c);
    if(e){hits++;entries.delete(c);entries.set(c,e);out.push(e.value);continue;}
    misses++;const r=await send(c);out.push(r);
    if(valid(c,r,generation))staged.push([c,r]);
    else {clear();return await finishUncached(commands,out);}
   }
   for(const [c,r] of staged)remember(c,r);
   return out;
  }catch(e){clear();throw e;}
 }
 async function finishUncached(commands,out){for(let i=out.length;i<commands.length;i++)out.push(await send(commands[i]));return out;}
 return {batch,clear,stats:()=>({entries:entries.size,bytes,hits,misses,checks,disabled})};
}
