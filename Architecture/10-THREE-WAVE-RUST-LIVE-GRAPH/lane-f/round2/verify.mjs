#!/usr/bin/env node
// Lane F Round 2 pure read-only acceptance: NO subprocess execution, no graph journal writes.
import {readFileSync,mkdirSync,writeFileSync,existsSync} from 'node:fs';
import {resolve,dirname,join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {attestBundle,readPinned,assertSafePath} from './lib/evidence.mjs';
import {parseExchange,inspectProvenance,checkCrosswalk} from './lib/hqkgx1.mjs';
import {auditCrosswalk,projectDCompatibility} from './lib/crosswalk.mjs';
import {evaluateIntentCases} from './lib/intent.mjs';
import {ledger} from './lib/coverage.mjs';
import {analyzePlacement} from './lib/placement.mjs';
import {compareWiki} from './lib/wiki.mjs';
export const SCHEMA='vivim.wave1.lane-f.round2-run/1';
const parse=(workspace,file)=>JSON.parse(readPinned(workspace,file).content);
const fail=(code,detail)=>({code,detail:detail??null});
export function assess(input, workspace){
 if(input?.schema!==SCHEMA)throw Error('ROUND2_MANIFEST_SCHEMA');
 const sections={},failures=[],files=input.files??{},get=(key)=>{
   try{return files[key]?parse(workspace,files[key]):null}catch(e){failures.push(fail('INVALID_FILE_'+key,e.message));return null}
 };
 const proof=get('proofs');sections.evidence=attestBundle(proof?.proofs??[],input.requiredNative??['cargo','import','rust-query','wiki','compiler','reset']);
 const raw=files.exchange?(()=>{try{return readPinned(workspace,files.exchange).content}catch(e){failures.push(fail('INVALID_EXCHANGE_FILE',e.message));return null}})():null;
 let exchange=null;if(raw){try{exchange=parseExchange(raw);sections.hqkg={provenance:inspectProvenance(exchange),crosswalk:checkCrosswalk(exchange,get('crosswalk')?.entries??[])}}catch(e){failures.push(fail('EXCHANGE_PARSE',e.message))}}
 if(!exchange)failures.push(fail('LIVE_HQKGX1_MISSING'));
 const a=get('a'),b=get('b'),c=get('c'),scope=get('scope'),feeds=[get('feedA'),get('feedB'),get('feedC')].filter(Boolean),compiled=get('compiled')??[];
 sections.crosswalk=auditCrosswalk({a,b,c,scope,eFeeds:feeds,compiled});
 if(exchange)sections.dCompatibility=projectDCompatibility(exchange);
 sections.intent=evaluateIntentCases(get('intent')?.cases??[]);
 const cov=get('coverage')??{};sections.coverage=ledger(cov);
 sections.placement=analyzePlacement(get('placement')?.components??[]);
 const wikiFiles=files.wikiPages??[],pages=new Map();for(const p of wikiFiles){try{pages.set(p,readPinned(workspace,p).content)}catch(e){failures.push(fail('WIKI_PAGE_UNAVAILABLE',e.message))}}
 if(exchange)sections.wiki=compareWiki({exchange,pages,qualityReport:get('quality'),nativeReceipt:get('nativeReceipt')});
 for(const [name,value] of Object.entries(sections)){
  if(value.status==='BLOCKED')failures.push(fail('SECTION_BLOCKED',name));
  for(const [key,sub] of Object.entries(value))if(sub?.status==='BLOCKED')failures.push(fail('SUBSECTION_BLOCKED',name+'.'+key));
 }
 // No PASS from source/source-shaped evidence alone; a native independent replay is required.
 if(!input.independentNativeReplayVerified)failures.push(fail('INDEPENDENT_NATIVE_REPLAY_MISSING'));
 return {schema:'vivim.wave1.lane-f.round2-report/1',status:failures.length?'BLOCKED':'READY_FOR_MASTER_REVIEW',sourceHeads:input.sourceHeads??{},sections,failures};
}
export function runCli(argv=process.argv.slice(2)){
 const m=argv.indexOf('--manifest'),o=argv.indexOf('--out');if(m<0||o<0||!argv[m+1]||!argv[o+1])throw Error('USAGE: node verify.mjs --manifest PATH --out OUTPUT');
 const manifest=resolve(argv[m+1]),base=dirname(manifest),dest=assertSafePath(base,argv[o+1],{mustExist:false});
 if(existsSync(dest))throw Error('OUTPUT_EXISTS_NO_OVERWRITE');
 const r=assess(JSON.parse(readFileSync(manifest,'utf8')),base);mkdirSync(dirname(dest),{recursive:true});writeFileSync(dest,JSON.stringify(r,null,2)+'\n',{flag:'wx',mode:0o600});
 console.log(JSON.stringify({status:r.status,failures:r.failures.length,report:dest}));process.exitCode=r.status==='BLOCKED'?2:0;
}
if(process.argv[1]&&import.meta.url===pathToFileURL(resolve(process.argv[1])).href){try{runCli()}catch(e){console.error(String(e.message).slice(0,200));process.exitCode=2}}
