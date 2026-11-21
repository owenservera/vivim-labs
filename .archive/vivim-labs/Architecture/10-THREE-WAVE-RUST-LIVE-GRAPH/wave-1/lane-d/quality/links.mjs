// Bounded markdown link/heading checker for existing HQ-generated Markdown only.
// It doesn't parse arbitrary HTML or fetch external links; unsupported constructions are diagnostics.
import {posix} from 'node:path';
const hrefRE=/(?<!!)\[[^\]\n]+\]\((<[^>]+>|[^\s)]+)(?:\s+"[^"]*")?\)/g;
const normalize=s=>s.toLowerCase().normalize('NFKC').replace(/[^\p{L}\p{N}\s-]/gu,'').trim().replace(/\s+/g,'-');
const fail=(code,page,href)=>({code,page,href});
export function checkLinks(pages){
 if(!(pages instanceof Map))throw Error('pages_map_required');
 const defects=[];let checked=0,resolved=0;const names=[...pages.keys()];const casing=new Map();
 for(const name of names){
  if(typeof name!=='string'||name.startsWith('/')||name.includes('\\')||name.split('/').some(x=>!x||x==='.'||x==='..'))throw Error('unsafe_page_path');
  const low=name.normalize('NFKC').toLowerCase();
  if(casing.has(low)) defects.push(fail('casefold_page_collision',name,casing.get(low)));
  casing.set(low,name);
 }
 const headingIds=new Map();
 for(const [name,md] of pages){
  if(typeof md!=='string')throw Error('bad_page_content');
  const ids=new Set();
  for(const line of md.split(/\r?\n/)){
   const found=/^#{1,6}\s+(.+?)\s*#*\s*$/.exec(line);
   if(found){const s=normalize(found[1]);let slug=s,n=1;while(ids.has(slug)){slug=s+'-'+n;n++}ids.add(slug);}
  }headingIds.set(name,ids);
 }
 for(const [name,md] of pages){
   for(const hit of md.matchAll(hrefRE)){
     checked++;
     let h=hit[1].startsWith('<')?hit[1].slice(1,-1):hit[1];
     if(/^(?:https?:|mailto:|tel:)/i.test(h)){resolved++;continue;}
     if(/^(?:javascript:|data:|file:|\\\\|\/\/)/i.test(h)){defects.push(fail('unsafe_link_scheme',name,h));continue;}
     try{h=decodeURIComponent(h)}catch{defects.push(fail('invalid_href_encoding',name,h));continue;}
     const [part,hash]=h.split('#');
     if(h.split('#').length>2){defects.push(fail('unsupported_fragment_syntax',name,h));continue;}
     const target=part?posix.normalize(posix.join(posix.dirname(name),part)):name;
     if(target.startsWith('../')||target==='..'||target.startsWith('/')||h.includes('\\')||part?.startsWith('/')){defects.push(fail('path_escape',name,h));continue;}
     if(!pages.has(target)){
       // Existing HQ also creates index.html/search.json; these need external page-manifest evidence.
       defects.push(fail(/\.(?:html|json)$/.test(target)?'non_markdown_target_unverified':'broken_link',name,h));continue;
     }
     if(hash && !headingIds.get(target)?.has(normalize(hash))){defects.push(fail('missing_anchor',name,h));continue;}
     resolved++;
   }
   if(/\]\((?:<[^>]*|[^)\n]*)\n/.test(md))defects.push(fail('unparsed_multiline_link',name,'[unsupported]'));
 }
 return {schema:'vivim.wave1.lane-d.links/1',checked,resolved,defects:defects.sort((a,b)=>a.page.localeCompare(b.page)||a.code.localeCompare(b.code))};
}
