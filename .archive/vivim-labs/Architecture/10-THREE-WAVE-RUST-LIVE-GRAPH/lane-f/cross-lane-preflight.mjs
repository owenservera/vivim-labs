#!/usr/bin/env node
/** Independent Lane F contract gate: only inspects source declarations and sanitized export envelopes. No effect dispatch or data conversion. */
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {resolve,dirname} from 'node:path';
import {createHash} from 'node:crypto';
import {pathToFileURL} from 'node:url';
export const EXPECTED = Object.freeze({
 A: {path:'hq/lib/live-graph/wave1/lane-a.mjs', constant:'OUTPUT_SCHEMA', schema:'vivim.wave1.code-observations/1', file:'hq/bin/wave1-lane-a.mjs'},
 B: {path:'hq/lib/wave1-kb-read.mjs',constant:'WAVE1_KB_SCHEMA',schema:'vivim.wave1.kb-semantic/1',file:'hq/bin/wave1-kb-export.mjs'},
 C: {path:'hq/live-graph/wave1/lane-c/commands.mjs',constant:'INVENTORY_SCHEMA',schema:'vivim.wave1.command-candidates/1',file:'hq/live-graph/wave1/lane-c/registry-adapter.mjs'},
 E: {path:'research/rust-knowledge-graph/integration/wave1-lane-e/assemble.mjs',constant:'FEED_SCHEMA',schema:'vivim.wave1.source-records/1',file:'research/rust-knowledge-graph/src/bin/hqkg-wave1-query.rs'}
});
const hash=s=>createHash('sha256').update(s).digest('hex');
const assert=(v,m)=>{if(!v)throw new Error(m)};
function schemaOf(s,constant){const escaped=constant.replace(/[^A-Za-z0-9_]/g,''); const m=s.match(new RegExp(`(?:export\\s+)?const\\s+${escaped}\\s*=\\s*['\"]([^'\"]+)['\"]`));return m?.[1]??null;}
export function auditSources(texts, pins={}){
 const rows=[],issues=[];
 for(const [lane,entry] of Object.entries(EXPECTED)){
   const content=texts[lane];
   const expected=entry.schema;
   const actual=typeof content==='string'?schemaOf(content,entry.constant):null;
   const digest=typeof content==='string'?hash(content):null;
   const evidence= pins[lane]??null;
   rows.push({lane,path:entry.path,actualSchema:actual,expectedOutputSchema:expected,sourceSha256:digest,gitBlobSha:evidence});
   if(!actual){issues.push({code:'SOURCE_MISSING_OR_UNPARSEABLE',lane,severity:'critical',evidence:entry.path});continue;}
   if(actual!==expected)issues.push({code:'SOURCE_SCHEMA_DRIFT',lane,severity:'critical',expected,actual,evidence:entry.path});
 }
 const sourceSchemas=rows.filter(r=>r.lane!=='E').map(r=>r.actualSchema);
 const E=rows.find(r=>r.lane==='E')?.actualSchema;
 for(let i=0;i<3;i++)if(sourceSchemas[i]&&E&&sourceSchemas[i]!==E)issues.push({code:'FEED_ADAPTER_NOT_PROVEN',lane:'ABC'[i],severity:'critical',sourceSchema:sourceSchemas[i],requiredByE:E,description:'Native producer envelope cannot be passed directly to Lane E; only an owner-authorized lossless, source-pinned conversion can satisfy this contract.'});
 return {schema:'vivim.wave1.lane-f.contract-audit/1',status:issues.length?'BLOCKED':'SOURCE_COMPATIBLE_NOT_RUNTIME_VERIFIED',rows,issues,requirements:['Prove scope+root ID, revision and intent crosswalk between A and E','Convert B records/links without creating source nodes or unverified edges','Convert C operations/CLI without promoting declared handlers to wired','Bind B/C action semantic IDs only when explicit verified source identity matches','Prove E graph->D generated wiki with real journal and Cargo receipts']};
}
export function auditEnvelopes(envelopes){
 const issues=[];
 for(const lane of ['A','B','C']){
  const expected=EXPECTED[lane].schema,v=envelopes?.[lane];
  if(!v||v.schema!==expected)issues.push({code:'EXPORT_SCHEMA_MISSING_OR_STALE',lane,expected,actual:v?.schema??null});
 }
 if(!Array.isArray(envelopes?.E?.nodes)||!Array.isArray(envelopes?.E?.edges)||envelopes?.E?.schema!==EXPECTED.E.schema)issues.push({code:'E_FEED_NOT_MATERIALIZED',reason:'No independently verified converted A+B+C feed set'});
 return {status:issues.length?'BLOCKED':'INPUT_SHAPES_PRESENT_NOT_GRAPH_VERIFIED',issues};
}
function cli(args){
 const rootAt=args.indexOf('--repos-hq'),outAt=args.indexOf('--out');
 assert(rootAt>=0&&args[rootAt+1]&&outAt>=0&&args[outAt+1],'usage: node cross-lane-preflight.mjs --repos-hq ROOT --out FILE.json');
 const root=resolve(args[rootAt+1]),texts={};for(const [lane,e] of Object.entries(EXPECTED)){try{texts[lane]=readFileSync(resolve(root,e.path),'utf8')}catch{}}
 const result=auditSources(texts);const out=resolve(args[outAt+1]);mkdirSync(dirname(out),{recursive:true});writeFileSync(out,JSON.stringify(result,null,2)+'\n');
 console.log(JSON.stringify({status:result.status,issues:result.issues.length,sourceSchemas:result.rows.map(r=>r.actualSchema)}));process.exitCode=result.status==='BLOCKED'?2:0;
}
if(process.argv[1]&&import.meta.url===pathToFileURL(resolve(process.argv[1])).href){try{cli(process.argv.slice(2))}catch(e){console.error(e.message);process.exitCode=2}}
