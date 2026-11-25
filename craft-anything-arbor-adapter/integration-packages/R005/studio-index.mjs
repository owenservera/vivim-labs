// Compatible with existing ARBOR parseCraftSourceIndex contract. No new API required.
import {isAbsolute} from 'node:path';
import {TARGETS} from './targets.mjs';
const validPath=x=>typeof x==='string'&&x.length>0&&x.length<=2048&&isAbsolute(x)&&!/[\x00-\x1f]/.test(x);
export function buildStudioIndex(entries){
 if(!Array.isArray(entries)||entries.length<1||entries.length>4)throw Error('INDEX_COUNT');
 const seen=new Set();
 const targets=entries.map(e=>{
  if(!e||!Object.hasOwn(TARGETS,e.siteId)||seen.has(e.siteId)||!validPath(e.atlasPath)||Object.keys(e).some(k=>!['siteId','atlasPath','previousAtlasPath','reconciliationPath'].includes(k)))throw Error('INDEX_ENTRY');
  seen.add(e.siteId);
  const result={siteId:e.siteId,atlasPath:e.atlasPath};
  for(const k of ['previousAtlasPath','reconciliationPath'])if(e[k]!=null){if(!validPath(e[k]))throw Error('INDEX_PATH');result[k]=e[k];}
  return result;
 }).sort((a,b)=>a.siteId<b.siteId?-1:a.siteId>b.siteId?1:0);
 return {schema:'vivim.craft-intelligence-sources/1',targets};
}
