#!/usr/bin/env node
// Read-only pinned-source drift analysis. No repo mutation or secret output.
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
const base=dirname(dirname(fileURLToPath(import.meta.url)));
const baseline=JSON.parse(readFileSync(join(base,'catalog/source-pins.json'),'utf8'));
const args=process.argv.slice(2);
if(args.length!==2 || !['--branch','--ref'].includes(args[0]) || !/^[a-zA-Z0-9._\/-]+$/.test(args[1])){
 console.error('Usage: node tech-stack/tools/reconcile.mjs --branch main | --ref <fullsha>');
 process.exit(2);
}
const ref=args[1],repo=baseline.repository;
const api='https://api.github.com/repos/'+repo;
const enc=encodeURIComponent;
const headers={Accept:'application/vnd.github+json','X-GitHub-Api-Version':'2022-11-28','User-Agent':'vivim-labs-tech-stack-research'};
if(process.env.GITHUB_TOKEN)headers.Authorization='Bearer '+process.env.GITHUB_TOKEN;
async function get(path){
 const r=await fetch(api+path,{headers,signal:AbortSignal.timeout(20000)});
 if(!r.ok)throw new Error('GitHub API HTTP '+r.status+' on '+path.split('?')[0]);
 return r.json();
}
try{
 const commit=await get('/commits/'+enc(ref));
 const sha=commit.sha;
 if(!/^[0-9a-f]{40}$/.test(sha))throw Error('Invalid returned commit SHA');
 const tree=await get('/git/trees/'+sha+'?recursive=1');
 if(tree.truncated)throw Error('Truncated GitHub tree: cannot assert absent files');
 const paths=new Map(tree.tree.filter(x=>x.type==='blob').map(x=>[x.path,x.sha]));
 const changes=Object.entries(baseline.manifest_blobs).map(([path,expected_blob])=>{
  const actual=paths.get(path)||null;
  return {path,expected_blob,actual_blob:actual,changed:actual!==expected_blob};
 }).filter(x=>x.changed);
 const rootLocks={};
 for(const p of ['Cargo.lock','bun.lock'])rootLocks[p]=paths.has(p)?'PRESENT':'ABSENT';
 const drift=sha!==baseline.commit || changes.length>0 || rootLocks['Cargo.lock']!=='ABSENT' || rootLocks['bun.lock']!=='ABSENT';
 const report={schema:'vivim-labs.tech-stack.drift-report/1',checked_at_utc:new Date().toISOString(),repository:repo,
 baseline_sha:baseline.commit,observed_sha:sha,status:drift?'SOURCE_DRIFT':'PIN_MATCH',changed_manifests:changes,root_lockfiles:rootLocks,
 note:'GitHub source metadata only; no resolved package tree, upstream release, runtime test or security assurance.'};
 console.log(JSON.stringify(report,null,2));
}catch(e){
 console.error(JSON.stringify({schema:'vivim-labs.tech-stack.drift-report/1',status:'UNREADABLE',reason:String(e.message||e),repository:repo},null,2));
 process.exitCode=1;
}
