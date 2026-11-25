// @hq:context Reuse exact structured article bodies within one development session.
// @hq:invariant All article identities and link targets remain part of the current input.
// @hq:why Unchanged page text need not be parsed again after a source revision.
import {createHash} from 'node:crypto';
const hash=v=>createHash('sha256').update(JSON.stringify(v)).digest('hex');
export function createArticleCache({maxEntries=256,maxBytes=8*1024*1024}={}){
 if(!Number.isSafeInteger(maxEntries)||maxEntries<1||maxEntries>256||!Number.isSafeInteger(maxBytes)||maxBytes<1||maxBytes>16*1024*1024)throw Error('WIKI_CACHE_CONFIG');
 const entries=new Map();let bytes=0,hits=0,misses=0,context=null;
 function clear(){entries.clear();bytes=0;}
 function remove(k){const e=entries.get(k);if(e){entries.delete(k);bytes-=e.bytes;}}
 function scope(pages,definitions,protocolGeneration){
  // Link resolution depends on the complete ordered path/ID inventory,
  // and annotation links on the definition dictionary. Never reuse across
  // dictionary or namespace changes, including removals and renames.
  const next=hash([protocolGeneration,pages.map(p=>[p.path,p.id]),definitions]);
  if(next!==context){clear();context=next;}
  return function article(page,build){
   const key=hash([context,page.id,page.path,page.text]);
   const e=entries.get(key);
   if(e){hits++;entries.delete(key);entries.set(key,e);return JSON.parse(e.json);}
   misses++;
   const result=build(page),json=JSON.stringify(result),n=Buffer.byteLength(key)+Buffer.byteLength(json);
   if(n<=maxBytes){
    while(entries.size>=maxEntries||bytes+n>maxBytes)remove(entries.keys().next().value);
    entries.set(key,{json,bytes:n});bytes+=n;
   }
   return result;
  };
 }
 return {scope,clear,stats:()=>({entries:entries.size,bytes,hits,misses})};
}
