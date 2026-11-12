#!/usr/bin/env node
// VIVIM Labs source-only readiness audit. No runtime promotion or private data access.
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {spawnSync} from 'node:child_process';

const root=path.resolve(fileURLToPath(new URL('..',import.meta.url)));
const SHA=/^[a-f0-9]{40}$/;
const CODE=/\.(?:mjs|cjs|js|jsx|ts|tsx|rs|py|ps1|sh|bat|go|java|kt)$/i;
const TEST=/(^|\/)(?:tests?|specs?|fixtures?)(\/|$)|(?:\.test\.|\.spec\.)/i;
const DOC=/\.(?:md|mdx|rst|txt)$/i;
const SENSITIVE=/(^|\/)(?:\.private-local|\.env|node_modules|target|dist|build|\.local|profiles?)(\/|$)|(?:credentials|cookies|secrets)(?:\.|\/|$)/i;
const tick=String.fromCharCode(96);
export function validateManifest(m){
  if(m?.schema!=='vivim.labs.readiness-manifest/1'||!Array.isArray(m.components))throw Error('MANIFEST_INVALID');
  const seen=new Set();
  for(const [k,v] of Object.entries(m.source_baselines||{}))if(!k||!SHA.test(v))throw Error('SOURCE_SHA_INVALID:'+k);
  for(const c of m.components){
    if(!/^[ECHX]\d{2}$/.test(c.id)||seen.has(c.id))throw Error('ID_INVALID:'+c.id);
    seen.add(c.id);
    if(!['easy','core','high_value','hold'].includes(c.category))throw Error('CATEGORY_INVALID:'+c.id);
    if(!m.source_baselines[c.source_branch])throw Error('REF_UNKNOWN:'+c.id);
    const p=c.source_path_prefix;
    if(!p||p.startsWith('/')||p.includes('..')||p.includes('\\')||/[\x00-\x1f]/.test(p))throw Error('PATH_UNSAFE:'+c.id);
    if(!c.arbor_target||!c.owner)throw Error('DESTINATION_MISSING:'+c.id);
  }
  return m;
}
export function parseTree(raw){
  return raw.split('\0').filter(Boolean).map(row=>{
    const t=row.indexOf('\t');
    if(t<0)return null;
    const meta=row.slice(0,t).split(' ');
    return meta.length===3&&meta[1]==='blob'?{path:row.slice(t+1),sha:meta[2]}:null;
  }).filter(Boolean);
}
export function fileStats(arr){
  const paths=[...new Set(arr)].sort();
  return {count:paths.length,code:paths.filter(p=>CODE.test(p)).length,
    test_or_fixture:paths.filter(p=>TEST.test(p)).length,
    documentation:paths.filter(p=>DOC.test(p)).length,
    sensitive_path_candidates:paths.filter(p=>SENSITIVE.test(p)).length,
    examples:paths.slice(0,10)};
}
function localGitTree(sha){
  const r=spawnSync('git',['ls-tree','-r','-z',sha],{cwd:root,encoding:'utf8',maxBuffer:30*1024*1024});
  return r.status===0?{ok:true,raw:r.stdout}:{ok:false,error:String(r.stderr||r.error?.message||'GIT_UNAVAILABLE').slice(0,240)};
}
export function scan(manifest,provider=localGitTree){
  validateManifest(manifest);
  const refs={},out=[],toplevel={};
  for(const [branch,sha] of Object.entries(manifest.source_baselines)){
    const r=provider(sha,branch);
    const files=r.ok?parseTree(r.raw):null;
    refs[branch]={source_sha:sha,status:r.ok?'GIT_TREE_READ':'REF_UNAVAILABLE',files,error:r.ok?null:r.error||'UNKNOWN'};
  }
  for(const c of manifest.components){
    const r=refs[c.source_branch];
    const matched=r.files===null?[]:r.files.filter(x=>x.path.startsWith(c.source_path_prefix));
    const status=r.files===null?'REF_UNAVAILABLE':matched.length?'SOURCE_TREE_PRESENT':'SOURCE_PATH_MISSING';
    out.push({...c,source_sha:r.source_sha,first_pass_static:status,file_stats:fileStats(matched.map(x=>x.path)),
      fresh_test_execution:'UNRUN',product_build:'UNRUN',host_wiring:'UNVERIFIED',device_acceptance:'UNRUN'});
  }
  if(refs.main?.files)for(const f of refs.main.files){const k=f.path.split('/')[0];toplevel[k]=(toplevel[k]||0)+1;}
  const coverage=Object.entries(toplevel).map(([group,files])=>({group,tracked_files:files,
    mapped:manifest.components.some(c=>c.source_branch==='main'&&
      (c.source_path_prefix===group||c.source_path_prefix===group+'/'||c.source_path_prefix.startsWith(group+'/')))}))
    .sort((a,b)=>b.tracked_files-a.tracked_files);
  const statusCount={},categoryCount={};
  for(const c of out){statusCount[c.first_pass_static]=(statusCount[c.first_pass_static]||0)+1;
    categoryCount[c.category]=(categoryCount[c.category]||0)+1;}
  return {schema:'vivim.labs.readiness-first-pass/1',date:'2026-11-12',
    method:'PINNED_TRACKED_GIT_TREE',limitations:'File existence, not build/runtime/device acceptance. Historic test claims are not re-executed by this scan.',
    baseline:Object.fromEntries(Object.entries(refs).map(([k,v])=>[k,{sha:v.source_sha,status:v.status,
      tracked_files:v.files?.length??null,error:v.error}])),
    totals:{components:out.length,category_count:categoryCount,source_status_count:statusCount},
    top_level_coverage:coverage,components:out};
}
const safe=x=>String(x??'').replace(/\|/g,'\\|').replace(/[\r\n]+/g,' ');
export function markdown(report){
  const a=['# Labs → ARBOR first-pass readiness','',
    'Method: pinned tracked Git trees; this is SOURCE INSPECTION, not executable runtime verification.',
    'Scan date: '+report.date,
    'Components: '+report.totals.components,
    'Source statuses: '+JSON.stringify(report.totals.source_status_count),
    'Fresh unit tests: UNRUN; Windows, provider, Host, phone: UNRUN. Previous claims remain reported evidence.',
    '', '## Destination and readiness','',
    '| ID | Component | Category | Files | Source status | Recorded maturity | Proposed destination | Owner | Disposition |',
    '| --- | --- | --- | ---: | --- | --- | --- | --- | --- |'];
  for(const c of report.components)a.push('| '+[c.id,c.name,c.category,c.file_stats.count,c.first_pass_static,c.reported_evidence,
    tick+c.arbor_target+tick,c.owner,c.disposition].map(safe).join(' | ')+' |');
  a.push('','## Top-level Labs coverage','',
    '| Group | Tracked files | Covered by review map |','| --- | ---: | --- |');
  for(const v of report.top_level_coverage)a.push('| '+tick+safe(v.group)+tick+' | '+v.tracked_files+' | '+(v.mapped?'yes':'REVIEW')+' |');
  a.push('','## Release guardrails','',
    'No result in this report is a production PASS. Do not bulk migrate private source data; do not add second graph, Host effect journal, command catalog, PWA, TierFlow publisher or scheduler. Only product owner gate may promote tests and source.','');
  return a.join('\n');
}
const script=process.argv[1]&&path.resolve(process.argv[1])===fileURLToPath(import.meta.url);
if(script){
  const args=process.argv.slice(2);
  let out=null,md=null,strict=false;
  for(let i=0;i<args.length;i++){
    if(args[i]==='--out')out=args[++i];
    else if(args[i]==='--md')md=args[++i];
    else if(args[i]==='--strict')strict=true;
    else if(args[i]==='--help'){console.log('node readiness/run-first-pass.mjs [--out PATH] [--md PATH] [--strict]');process.exit(0);}
    else throw Error('UNKNOWN_ARG:'+args[i]);
  }
  const manifest=JSON.parse(fs.readFileSync(path.join(root,'readiness/components.json'),'utf8'));
  const report=scan(manifest);
  for(const [p,data] of [[out,JSON.stringify(report,null,2)+'\n'],[md,markdown(report)]])
    if(p){const abs=path.resolve(p);fs.mkdirSync(path.dirname(abs),{recursive:true});fs.writeFileSync(abs,data);}
  console.log(JSON.stringify({baseline:report.baseline,totals:report.totals,
    unmapped_groups:report.top_level_coverage.filter(x=>!x.mapped).map(x=>x.group)},null,2));
  if(strict&&(Object.values(report.baseline).some(x=>x.status!=='GIT_TREE_READ')||
    report.components.some(x=>x.category!=='hold'&&x.first_pass_static!=='SOURCE_TREE_PRESENT')))process.exitCode=2;
}
