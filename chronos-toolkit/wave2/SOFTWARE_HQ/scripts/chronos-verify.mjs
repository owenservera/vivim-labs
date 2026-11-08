#!/usr/bin/env node
/** Offline bounded cross-repository Chronos evidence runner. Never changes Git or drives providers. */
import {resolve,join} from 'node:path';
import {existsSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
const args=process.argv.slice(2),idx=args.indexOf('--root');
const root=resolve(idx<0?process.cwd():args[idx+1]);
const run=args.includes('--run');
const tests=[
  ['SOFTWARE_HQ','node',['--test','scripts/chronos-parity.test.mjs']],
  ['repos-hq','node',['--test','hq/test/chronos-supervisor.test.mjs']],
  ['repos-hq','cargo',['test','--manifest-path','research/rust-knowledge-graph/Cargo.toml','--test','chronos']],
  ['repos-hq','cargo',['test','--manifest-path','research/rust-knowledge-graph/Cargo.toml','--features','importer']],
  ['vivim-web-harness','node',['--test','studio/test/unit/chronos-conversation-projection.test.mjs','studio/test/unit/chronos-conversation-route.test.mjs']],
  ['tierflow','bun',['test','tests/visual-runtime/chronos-read-projection.test.ts']],
  ['tierflow','bun',['run','typecheck']],
];
const cases=[];
for(const [repo,command,argv] of tests){
  const cwd=join(root,repo), sourceExists=existsSync(cwd);
  if(!run){cases.push({repo,command,argv,status:'PLANNED',sourceExists});continue;}
  if(!sourceExists){cases.push({repo,command,argv,status:'BLOCKED',reason:'REPO_MISSING'});continue;}
  const result=spawnSync(command,argv,{cwd,encoding:'utf8',shell:false,timeout:180000,maxBuffer:512*1024});
  cases.push({repo,command,argv,status:result.error?.code==='ENOENT'?'UNRUN':result.error?'FAIL':result.status===0?'PASS':'FAIL',
    exitCode:result.status,reason:result.error?.code??null,
    stdout:String(result.stdout??'').slice(-4500),stderr:String(result.stderr??'').slice(-4500)});
}
console.log(JSON.stringify({schema:'vivim.chronos.verification/1',root,execution:run,ci:false,
  physicalWindowsAcceptance:'UNRUN',physicalAndroidAcceptance:'UNRUN',cases},null,2));
if(run && cases.some(c=>c.status==='FAIL'))process.exitCode=1;
