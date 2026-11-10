#!/usr/bin/env node
// Offline, explicit, lease-safe multi-repo Chronos patch installer.
// --check is read-only. --apply modifies target paths only, never commits/pushes/runs agents.
import {existsSync, readFileSync, writeFileSync, mkdirSync, readdirSync, statSync, renameSync, rmSync} from 'node:fs';
import {join,resolve,relative,dirname,sep} from 'node:path';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
const bundle=dirname(fileURLToPath(import.meta.url));
const args=process.argv.slice(2);
const apply=args.includes('--apply'), fixture=args.includes('--fixture-check');
if(args.includes('--help')){console.log('node INTEGRATE-ALL.mjs --root C:\\0-BlackBoxProject-0 [--check | --apply]\n--check read-only preflight; --apply local changes only; --fixture-check for test fixture only');process.exit(0);}
if(apply && fixture) throw new Error('fixture cannot apply');
const idx=args.indexOf('--root');
const root=resolve(idx<0?process.cwd():args[idx+1]);
const replacements=JSON.parse(readFileSync(join(bundle,'changes','replacements.json'),'utf8'));
const repos=['repos-hq','vivim-web-harness','tierflow','vivim-labs','SOFTWARE_HQ'];
const id=(repo,path)=>`${repo}:${path}`;
const inputs=new Map();
for(const repo of repos){
  const dir=join(bundle,repo);
  if(!existsSync(dir)) continue;
  const scan=(from)=>{
    for(const entry of readdirSync(from)){
      const full=join(from,entry),stat=statSync(full);
      if(stat.isDirectory()) scan(full);
      else if(stat.isFile()){
        const path=relative(dir,full).split(sep).join('/');
        inputs.set(id(repo,path),{repo,path,newFile:true,content:readFileSync(full,'utf8')});
      }
    }
  };
  scan(dir);
}
for(const op of replacements){
  const key=id(op.repo,op.path);
  if(!repos.includes(op.repo)||!op.path||op.path.includes('..')||!op.find||!op.replace||(inputs.has(key)&&inputs.get(key).newFile))
    throw new Error(`BAD_OPERATION:${key}`);
  if(!inputs.has(key)) inputs.set(key,{repo:op.repo,path:op.path,newFile:false,steps:[]});
  inputs.get(key).steps.push(op);
}
const heads={};
const updates=[];
for(const repo of repos){
  const dir=join(root,repo);
  if(!existsSync(dir)) throw new Error(`REPOSITORY_MISSING:${repo}:${dir}`);
  if(!fixture){
    const toplevel=execFileSync('git',['-C',dir,'rev-parse','--show-toplevel'],{encoding:'utf8'}).trim();
    if(resolve(toplevel)!==resolve(dir))throw new Error(`NOT_REPO_ROOT:${repo}`);
    heads[repo]=execFileSync('git',['-C',dir,'rev-parse','HEAD'],{encoding:'utf8'}).trim();
  }
}
for(const entry of inputs.values()){
  const dir=join(root,entry.repo),full=resolve(dir,entry.path);
  if(full!==dir && !full.startsWith(dir+sep))throw new Error(`PATH_ESCAPE:${entry.repo}:${entry.path}`);
  const old=existsSync(full)?readFileSync(full,'utf8'):null;
  const localStatus=fixture?'':execFileSync('git',['-C',dir,'status','--porcelain','--',entry.path],{encoding:'utf8'}).trim();
  if(entry.newFile){
    if(old!==null && old!==entry.content) throw new Error(`NEW_FILE_COLLISION:${entry.repo}:${entry.path}`);
    if(localStatus && old!==entry.content)throw new Error(`LOCAL_EDITED_PATH:${entry.repo}:${entry.path}:${localStatus}`);
    updates.push({entry,full,old,content:entry.content,status:old===null?'add':'already'});
  }else{
    if(old===null)throw new Error(`MISSING_PATCH_TARGET:${entry.repo}:${entry.path}`);
    let content=old;
    for(const op of entry.steps){
      if(content.includes(op.replace))continue;
      const offset=content.indexOf(op.find);
      if(offset<0){
        if(content.includes(op.replace))continue;
        throw new Error(`PATCH_CONTEXT_DRIFT:${entry.repo}:${entry.path}`);
      }
      if(content.indexOf(op.find,offset+op.find.length)>=0)throw new Error(`AMBIGUOUS_PATCH_CONTEXT:${entry.repo}:${entry.path}`);
      content=content.slice(0,offset)+op.replace+content.slice(offset+op.find.length);
    }
    if(localStatus && old!==content)throw new Error(`LOCAL_EDITED_PATH:${entry.repo}:${entry.path}:${localStatus}`);
    updates.push({entry,full,old,content,status:old===content?'already':'edit'});
  }
}
const plan={schema:'vivim.chronos.install-plan/2',mode:apply?'apply':'check',root,
  repoHeads:heads, files:updates.map(x=>({repo:x.entry.repo,path:x.entry.path,status:x.status,
    digest:createHash('sha256').update(x.content).digest('hex')})),
  assertions:['no git commit/push','no provider action','existing scheduler/Host authority untouched','target paths clean before write']};
if(!apply){console.log(JSON.stringify(plan,null,2));process.exit(0);}
const modified=[];
try{
  for(const item of updates.filter(x=>x.status!=='already')){
    mkdirSync(dirname(item.full),{recursive:true});
    const temp=`${item.full}.chronos-tmp-${process.pid}`;
    writeFileSync(temp,item.content,{flag:'wx'});
    renameSync(temp,item.full);
    modified.push(item);
  }
}catch(e){
  for(const item of modified.reverse()){
    if(item.old===null)rmSync(item.full,{force:true});else writeFileSync(item.full,item.old);
  }
  throw e;
}
console.log(JSON.stringify({...plan,modified:modified.length,commitCreated:false,deviceAccepted:false},null,2));
