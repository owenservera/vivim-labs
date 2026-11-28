// R007: independent CLI experiment setup. Node >=22; no third-party packages.
// Smoke uses empty scratch directories only, never actual experimental seeds.
// No automatic campaign launch, login, privilege changes, or global settings edits.
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import crypto from 'node:crypto';
import {spawn,spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
const lab=path.dirname(fileURLToPath(import.meta.url));
const pin='3b1d7e173071710e6795c09427162292f774030e';
const lanes={'01':'codex','02':'agy','03':'codex','04':'agy'};
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
const json=(p,v)=>fs.writeFileSync(p,JSON.stringify(v,null,2)+'\n',{flag:'wx'});
const mkdir=p=>fs.mkdirSync(p,{recursive:true});
const read=p=>JSON.parse(fs.readFileSync(p,'utf8'));
const utc=()=>new Date().toISOString();
const fail=m=>{throw new Error(m)};
const git=(args)=>{const p=spawnSync('git',['-C',path.dirname(lab),...args],{encoding:null,maxBuffer:32*1024*1024,windowsHide:true});if(p.status!==0)fail('git '+args[0]+' failed: '+p.stderr);return p.stdout;};
const local=process.env.LOCALAPPDATA||os.tmpdir();
const base=path.join(local,'VIVIM','CLI-Dogfood-Setup');
function newRoot(kind){mkdir(base);return fs.mkdtempSync(path.join(base,kind+'-'));}
function checkOutside(p){const a=path.resolve(lab).toLowerCase(),b=path.resolve(p).toLowerCase();if(b===a||b.startsWith(a+path.sep))fail('scratch must be outside repository');}
function prepare(){
 const root=newRoot('seed'); checkOutside(root);
 const manifests=[];
 for(const [id,provider] of Object.entries(lanes)){
  const prefix='ZCODE-PERSISTENCE-LAB/EXPERIMENT-'+id+'/';
  const dest=path.join(root,crypto.randomUUID());mkdir(dest);
  const workspace=path.join(dest,'workspace');mkdir(workspace);
  const controller=path.join(dest,'controller');mkdir(controller);
  const entries=git(['ls-tree','-r','-z',pin,'--',prefix]).toString('utf8').split('\0').filter(Boolean);
  if(!entries.length)fail('empty seed '+id);
  const files=[];
  for(const entry of entries){
   const tab=entry.indexOf('\t'),[mode,type,oid]=entry.slice(0,tab).split(' '),name=entry.slice(tab+1);
   if(!['100644','100755'].includes(mode)||type!=='blob'||!name.startsWith(prefix))fail('unsafe git tree entry');
   const rel=name.slice(prefix.length);
   if(rel.split('/').some(x=>!x||x==='.'||x==='..'||x.toLowerCase()==='.git'||x.includes(':')||x.includes('\\')))fail('unsafe filename');
   const bytes=git(['cat-file','blob',oid]);
   const p=path.join(workspace,...rel.split('/'));mkdir(path.dirname(p));fs.writeFileSync(p,bytes,{flag:'wx'});
   if(sha(fs.readFileSync(p))!==sha(bytes))fail('copy mismatch');
   files.push({path:rel,git_blob:oid,sha256:sha(bytes),bytes:bytes.length});
  }
  const recipe=read(path.join(lab,provider==='codex'?'CODEX-CLI-RECIPE.json':'ANTIGRAVITY-CLI-RECIPE.json'));
  if(!recipe.assignments.includes('EXPERIMENT-'+id))fail('assignment mismatch');
  const record={lane:id,provider,seed_commit:pin,workspace,controller,files,manifest_sha256:sha(JSON.stringify(files)),created_utc:utc(),status:'PREPARED_NOT_ISOLATED_NOT_LAUNCHED',recipe_revision:recipe.revision};
  json(path.join(controller,'seed-manifest.json'),record);manifests.push(record);
 }
 json(path.join(root,'setup-manifest.json'),{schema:'vivim.cli.setup/1',root,lanes:manifests,status:'PREPARED',campaign_started:false,warning:'Folders are not access-control boundaries. Dedicated authenticated isolated execution profiles and boundary tests required before dogfood.'});
 console.log(JSON.stringify({root,lanes:manifests.map(x=>({lane:x.lane,provider:x.provider,files:x.files.length,manifest_sha256:x.manifest_sha256})),status:'PREPARED_NOT_LAUNCHED'}));
}
function binaries(provider){
 if(provider==='codex'){
  const entry=path.join(process.env.APPDATA||fail('APPDATA missing'),'npm','node_modules','@openai','codex','bin','codex.js');
  if(!fs.existsSync(entry))fail('official Codex entrypoint missing');
  return {program:process.execPath,prefix:[entry],binary_sha256:sha(fs.readFileSync(entry))};
 }
 const exe=path.join(local,'agy','bin','agy.exe');if(!fs.existsSync(exe))fail('agy.exe missing');
 return {program:exe,prefix:[],binary_sha256:sha(fs.readFileSync(exe))};
}
function runBounded(bin,args,cwd,input,dir,tag,ms=90000){
 return new Promise(resolve=>{
  const start=utc(),outPath=path.join(dir,tag+'.stdout.jsonl'),errPath=path.join(dir,tag+'.stderr.log');
  const out=fs.openSync(outPath,'wx'),err=fs.openSync(errPath,'wx');
  json(path.join(dir,tag+'.intent.json'),{tag,started_utc:start,cwd,program:bin.program,argv:args,stdin_sha256:sha(input||''),deadline_ms:ms});
  let timedOut=false,spawnError=null,done=false,timer=null;
  const child=spawn(bin.program,[...bin.prefix,...args],{cwd,windowsHide:true,shell:false,stdio:['pipe','pipe','pipe']});
  // Stream to disk, cap aggregate bytes to avoid unbounded local logs.
  let total=0;const max=16*1024*1024;
  function stop(){if(child.exitCode!==null||child.signalCode!==null)return;
   if(process.platform==='win32'&&child.pid)spawnSync('taskkill.exe',['/PID',String(child.pid),'/T','/F'],{windowsHide:true,timeout:10000});
   else child.kill('SIGKILL');
  }
  function capture(fd,buf){total+=buf.length;if(total>max){timedOut=true;stop();return;}fs.writeSync(fd,buf);}
  child.stdout.on('data',b=>capture(out,b));child.stderr.on('data',b=>capture(err,b));
  child.stdin.on('error',()=>{});child.stdin.end(input||'');
  child.on('error',e=>{spawnError=e.message;});
  child.on('close',(code,signal)=>{
   if(done)return;done=true;clearTimeout(timer);fs.closeSync(out);fs.closeSync(err);
   const raw=fs.readFileSync(outPath),stderr=fs.readFileSync(errPath);
   const frames=[],invalid=[];
   for(const l of raw.toString('utf8').split(/\r?\n/).filter(x=>x.trim())){try{frames.push(JSON.parse(l));}catch{invalid.push(l.slice(0,200));}}
   const result={tag,start,end:utc(),pid:child.pid??null,code,signal,timed_out:timedOut,spawn_error:spawnError,stdout_sha256:sha(raw),stderr_sha256:sha(stderr),frames,invalid,stderr_tail:stderr.toString('utf8').slice(-1200)};
   json(path.join(dir,tag+'.receipt.json'),result);resolve(result);
  });
  timer=setTimeout(()=>{timedOut=true;stop();},ms);
 });
}
function parse(provider,r){
 const f=r.frames;
 if(provider==='codex'){
  const sessions=[...new Set(f.filter(x=>x.type==='thread.started').map(x=>x.thread_id).filter(Boolean))];
  const turns=f.filter(x=>x.type==='turn.completed');
  const answer=f.filter(x=>x.type==='item.completed'&&x.item?.type==='agent_message').at(-1)?.item?.text;
  const tools=f.filter(x=>x.type==='item.started'||x.type==='item.completed').filter(x=>!['agent_message','reasoning'].includes(x.item?.type)).map(x=>x.item?.type);
  return {session:sessions.length===1?sessions[0]:null,terminal:turns.length===1&&!f.some(x=>x.type==='turn.failed'),answer,tools,usage:turns.at(-1)?.usage??null};
 }
 const sessions=[...new Set(f.filter(x=>x.event==='init').map(x=>x.conversation_id).filter(Boolean))];
 const result=f.filter(x=>x.event==='result').at(-1)?.result;
 const init=f.find(x=>x.event==='init')?.init;
 const tools=f.filter(x=>x.event==='step_update').map(x=>x.step_update?.step_type).filter(x=>x&&!['user_input','agent_response','thinking','reasoning','system_message'].includes(x));
 return {session:sessions.length===1?sessions[0]:null,terminal:result?.status==='SUCCESS'&&result?.conversation_id===sessions[0],answer:result?.response,tools:[...new Set(tools)],cwd:init?.cwd,permission_mode:init?.permission_mode,usage:result?.usage??null};
}
function argsFor(provider,workspace,prompt,session){
 if(provider==='codex'){
  const flags=['--json','--skip-git-repo-check','--ignore-user-config','-c','approval_policy="never"','-c','sandbox_workspace_write.network_access=false'];
  return session?{args:['exec','resume',...flags,'-c','sandbox_mode="read-only"',session,'-'],input:prompt}:{args:['exec',...flags,'--sandbox','read-only','--cd',workspace,'-'],input:prompt};
 }
 const flags=['--sandbox','--disable-slash-commands','--print-timeout','75s','--output-format','stream-json'];
 return {args:[...flags,...(session?['--conversation',session]:[]),'--print',prompt],input:null};
}
async function smokeLane(root,id,provider){
 const dir=path.join(root,'lane-'+id);mkdir(dir);const workspace=path.join(dir,'empty');mkdir(workspace);
 const bin=binaries(provider),nonce='VIVIM_'+crypto.randomBytes(8).toString('hex').toUpperCase();
 const startPrompt='Remember this exact nonce for the next turn: '+nonce+'. Reply with exactly the nonce. Do not use any tools, access files, execute commands, or make edits.';
 const resumePrompt='Reply with exactly the nonce from your previous turn. Do not use any tools, access files, execute commands, or make edits.';
 const turns=[];
 let session=null;
 for(const [i,prompt] of [startPrompt,resumePrompt].entries()){
  const c=argsFor(provider,workspace,prompt,session);
  const r=await runBounded(bin,c.args,workspace,c.input,dir,'turn-'+(i+1));
  const x=parse(provider,r);
  const ok=r.code===0&&!r.timed_out&&!r.spawn_error&&r.invalid.length===0&&x.terminal&&!!x.session&&(!session||x.session===session)&&x.answer?.trim()===nonce&&x.tools.length===0&&fs.readdirSync(workspace).length===0&&(!x.cwd||path.resolve(x.cwd).toLowerCase()===path.resolve(workspace).toLowerCase());
  turns.push({turn:i+1,ok,code:r.code,timed_out:r.timed_out,session:x.session,terminal:x.terminal,answer_matches:x.answer?.trim()===nonce,tools:x.tools,invalid_frames:r.invalid.length,usage:x.usage,cwd:x.cwd,stderr_tail:r.stderr_tail,spawn_error:r.spawn_error});
  if(!ok)break;session=x.session;
 }
 const result={lane:id,provider,workspace,binary_sha256:bin.binary_sha256,turns,pass:turns.length===2&&turns.every(x=>x.ok),scope:'synthetic empty-directory transport only; shared authorized profile, not proof of seed isolation'};
 json(path.join(dir,'summary.json'),result);return result;
}
async function smoke(){
 const root=newRoot('smoke');checkOutside(root);console.log(JSON.stringify({root,status:'STARTED_SYNTHETIC_ONLY'}));
 // One bounded lane per provider at a time. No campaigns or seed access.
 const results=await Promise.allSettled([smokeLane(root,'01','codex'),smokeLane(root,'02','agy')]);
 const second=[];
 if(results[0].status==='fulfilled'&&results[0].value.pass)second.push(smokeLane(root,'04','codex'));
 if(results[1].status==='fulfilled'&&results[1].value.pass)second.push(smokeLane(root,'03','agy'));
 const more=await Promise.allSettled(second);
 const rows=[...results,...more].map(x=>x.status==='fulfilled'?x.value:{pass:false,error:String(x.reason)});
 const ids=rows.flatMap(x=>x.turns?.[0]?.session?[x.turns[0].session]:[]);
 const passed=rows.length===4&&rows.every(x=>x.pass)&&new Set(ids).size===4;
 json(path.join(root,'summary.json'),{scope:'transport only',root,passed,rows,campaign_started:false});
 console.log(JSON.stringify({root,passed,rows}));if(!passed)process.exitCode=2;
}
async function smokeSingle(id){
 if(!lanes[id])fail('invalid lane');const root=newRoot('smoke-single');checkOutside(root);console.log(JSON.stringify({root,lane:id,status:'STARTED_SYNTHETIC_ONLY'}));const result=await smokeLane(root,id,lanes[id]);json(path.join(root,'summary.json'),{scope:'transport only',root,passed:result.pass,rows:[result],campaign_started:false});console.log(JSON.stringify(result));if(!result.pass)process.exitCode=2;
}
const action=process.argv[2];
if(action==='prepare')prepare();
else if(action==='smoke'||action==='smoke-single')fail('R010: historical smoke launcher disabled; selected provider policy, budget reservation and isolation required before inference');
else fail('usage: node CLI-SETUP.mjs prepare|smoke|smoke-single <lane>; no campaign command is provided');
