/** VIVIM Labs knowledge lens: pure query shaping + read-only HQ KB adapter.
 * No indexing, wiki publishing, new graph, executing commands, raw private exports, or persistent cache.
 */
import { existsSync, statSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { DatabaseSync } from 'node:sqlite';
import { paths } from './bridge.mjs';

export const CONTEXT_SCHEMA = 'vivim.labs.context-lens/1';
const SKIP = new Set('i me my we our please can could would should the a an of for to in on by with and or then is are it this that how what show tell make do does did use using about project work working want need build create find look see get also from into now current let us into regarding'.split(' '));
const PREFIX = /^(?:(?:please\s+)?(?:email|send|show|list|find|open|create|write|build|explain|tell|search|inspect|change|update|check)\s+)/i;
const PROJECTS = Object.freeze([
 {key:'repos-hq',names:['repos-hq','repos hq','hq','knowledge graph','hqkg']},
 {key:'vivim-web-harness',names:['vivim-web-harness','web harness','harness','host']},
 {key:'tierflow',names:['tierflow','vivim-ui','visual runtime']},
 {key:'vivim-labs',names:['vivim-labs','vivim labs','labs']},
]);

/** Intent context can be queried independently of the interpretation result. */
export function deriveFocus(text, scope='auto') {
 const value=String(text??'').slice(0,1000).trim();
 const lower=value.toLowerCase();
 const explicit=scope==='auto'?null:PROJECTS.find(p=>p.key===scope);
 const detected=explicit || PROJECTS.find(p=>p.names.some(name=>new RegExp(`(^|[^a-z])${name.replace(/[.*+?^${}()|[\]\\]/g,'\\$&')}(?=$|[^a-z])`,'i').test(lower)));
 const words=(value.replace(PREFIX,'').match(/[\p{L}\p{N}_.-]+/gu)||[]).map(x=>x.toLowerCase())
   .filter(x=>x.length>=3 && !SKIP.has(x));
 const unique=[...new Set(words)].slice(0,10);
 return {query:unique.join(' '),terms:unique,scope:detected?.key||null,scopeReason:explicit?'user_selected':detected?'name_detected':'unscoped',ready:unique.length>0};
}

export function normalizeKnowledge(raw, source='hq.kb') {
 if(!Array.isArray(raw))return [];
 const accepted=new Set(['doc','wiki','canon','decision','capability','vivim','session','note','file']);
 return raw.filter(x=>x&&accepted.has(x.kind||'file')).slice(0,20).map(x=>({
   id:String(x.id??[x.project||'',x.path||'',x.title||''].join(':')).slice(0,260),
   kind:String(x.kind||'file').slice(0,32),
   title:String(x.title||x.path||'Untitled').slice(0,200),
   snippet:String(x.snippet||'').replace(/\s+/g,' ').slice(0,320),
   project:x.project==null?null:String(x.project).slice(0,100),
   path:x.path?String(x.path).slice(0,400):null,
   line:Number.isInteger(x.line)?x.line:null,
   score:Number.isFinite(x.score)?Number(x.score):null,
   source,
 }));
}

const missing=(focus,reason)=>({schema:CONTEXT_SCHEMA,status:'unavailable',focus,source:'hq.local.sqlite',
  revision:null,items:[],page:null,reason,persisted:false,readOnly:true});

/** Caller supplies explicit privacy opt-in. Database is always opened readOnly; existing HQ modules only. */
export async function searchContext({text='',scope='auto',includePersonal=false,limit=7,hqRoot=paths.hq}={}) {
 const focus=deriveFocus(text,scope);
 if(!focus.ready)return {schema:CONTEXT_SCHEMA,status:'waiting_for_topic',focus,source:'hq.local.sqlite',revision:null,items:[],page:null,persisted:false,readOnly:true};
 const registry=join(hqRoot,'hq/lib/registry.mjs');
 const kbFile=join(hqRoot,'hq/lib/kb.mjs');
 const searchFile=join(hqRoot,'hq/lib/search.mjs');
 if(!existsSync(kbFile)||!existsSync(searchFile)||!existsSync(registry))return missing(focus,'HQ knowledge readers not found in local checkout');
 try{
   const config=await import(pathToFileURL(registry).href);
   const dbFile=config.dbPath(hqRoot); // existing HQ configuration, no writes
   if(!existsSync(dbFile))return missing(focus,'HQ index database not present; run the existing HQ indexing workflow');
   const max=Math.max(1,Math.min(12,Math.trunc(Number(limit)||7)));
   const db=new DatabaseSync(dbFile,{readOnly:true});
   let kb=[],fts=[],projects=[];
   try{
     const [{kbQuery},{searchContent}]=await Promise.all([import(pathToFileURL(kbFile).href),import(pathToFileURL(searchFile).href)]);
     const kinds=includePersonal?null:['doc','wiki','canon','decision','capability','vivim'];
     // HQ project names can differ from repo slugs; unscoped search preserves results.
     kb=kbQuery(db,{query:focus.query,kind:kinds,limit:max*2});
     const ftsResult=searchContent(db,{query:focus.query,limit:max*2});
     fts=ftsResult.results||[];
     try{projects=db.prepare('SELECT name, head, indexed_at FROM projects ORDER BY name LIMIT 30').all();}catch{}
   }finally{db.close();}
   const kbItems=normalizeKnowledge(kb,'hq.kbQuery');
   const ftsItems=normalizeKnowledge(fts.map(x=>({...x,kind:'file',title:x.path,id:`file:${x.project||'unknown'}:${x.path}`})),'hq.searchContent');
   const items=[];const seen=new Set();
   for(const x of [...kbItems,...ftsItems]){
     if(!includePersonal && ['session','note'].includes(x.kind))continue;
     const key=[x.project||'',x.path||x.title,x.kind].join('\0').toLowerCase();
     if(seen.has(key))continue;
     seen.add(key);items.push(x);
     if(items.length>=max)break;
   }
   const stat=statSync(dbFile);
   const revision=[Math.trunc(stat.mtimeMs),stat.size].join('-');
   const project=projects.find(x=>focus.scope && (x.name===focus.scope||x.name.includes(focus.scope)))||null;
   const status=items.length?'results':'no_matches';
   const page={kind:'ephemeral_knowledge_page',title:focus.scope?`Working context · ${focus.scope}`:'Working context',
     subtitle:focus.terms.slice(0,7).join(' · '),
     overview:items.length?`${items.length} grounded source references found in the current HQ index.`:'No matching indexed evidence; no context was invented.',
     focus,project:project?{name:project.name,head:project.head,indexedAt:project.indexed_at}:null,
     sections:[{title:'Most relevant evidence',itemIds:items.filter(x=>x.kind==='wiki'||x.kind==='doc'||x.kind==='decision').map(x=>x.id).slice(0,5)},
       {title:'Code and capabilities',itemIds:items.filter(x=>x.kind==='file'||x.kind==='capability'||x.kind==='vivim').map(x=>x.id).slice(0,5)}],
     caveat:'Live read-only projection, not a saved/generated HQ wiki page or current release verification.'};
   return {schema:CONTEXT_SCHEMA,status,focus,source:'hq.local.sqlite',revision,items,page,
      coverage:{kbRows:kb.length,ftsRows:fts.length,indexedProjects:projects.length,personalIncluded:!!includePersonal},
      persisted:false,readOnly:true};
 }catch(e){return missing(focus,String(e.code||e.message||e).slice(0,250));}
}