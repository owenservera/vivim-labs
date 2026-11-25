// Read-only aggregate probe through the existing hash-pinned native helper.
import {createHash} from 'node:crypto';
import {readFileSync,realpathSync,lstatSync} from 'node:fs';
import {isAbsolute,basename} from 'node:path';
import {spawnSync} from 'node:child_process';
export const NATIVE_RESOURCE_ACTION='windows.system.resources.read';
const fail=()=>Object.assign(Error('WINDOWS_NATIVE_RESOURCE_UNAVAILABLE'),{code:'WINDOWS_NATIVE_RESOURCE_UNAVAILABLE',status:409});
const sha=s=>typeof s==='string'&&/^[a-f0-9]{64}$/.test(s);
const int=x=>Number.isSafeInteger(x)&&x>=0;
const exact=(o,ks)=>o!==null&&typeof o==='object'&&!Array.isArray(o)&&Object.keys(o).length===ks.length&&ks.every(k=>Object.hasOwn(o,k));
function pinned(file,digest,max){
 if(typeof file!=='string'||!isAbsolute(file)||!sha(digest))throw fail();
 const st=lstatSync(file);if(!st.isFile()||st.isSymbolicLink()||st.size<1||st.size>max)throw fail();
 const real=realpathSync(file),b=readFileSync(real);
 if(b.length!==st.size||createHash('sha256').update(b).digest('hex')!==digest)throw fail();
 return {real,b};
}
export function validateResourcePayload(v){
 if(v?.schema!=='vivim.windows.resource-probe/1')throw fail();
 if(exact(v,['schema','status'])&&v.status==='unsupported')return {status:'unsupported',data:null};
 if(!exact(v,['schema','status','memoryTotalBytes','memoryAvailableBytes','cpuBusyBasisPoints','cpuScope','sampleMs'])||v.status!=='ok'||!int(v.memoryTotalBytes)||v.memoryTotalBytes===0||!int(v.memoryAvailableBytes)||v.memoryAvailableBytes>v.memoryTotalBytes||!(v.cpuBusyBasisPoints===null||int(v.cpuBusyBasisPoints)&&v.cpuBusyBasisPoints<=10000)||v.cpuScope!=='calling-processor-group'||!int(v.sampleMs)||v.sampleMs<1||v.sampleMs>5000)throw fail();
 return {status:v.cpuBusyBasisPoints===null?'partial':'ok',data:{memoryTotalBytes:v.memoryTotalBytes,memoryAvailableBytes:v.memoryAvailableBytes,cpuBusyBasisPoints:v.cpuBusyBasisPoints,cpuScope:v.cpuScope,sampleMs:v.sampleMs}};
}
export function createNativeResourcePort({env=process.env,run=spawnSync,now=Date.now}={}){
 function resolve(){
  const cfg=pinned(env.VIVIM_WORKSTATION_ROOTS,env.VIVIM_WORKSTATION_ROOTS_SHA256,65536);
  const v=JSON.parse(cfg.b.toString('utf8')),grant=v.nativeResources;
  if(v.schema!=='vivim.workstation-roots/1'||!exact(grant,['expiresAt'])||!int(grant.expiresAt)||grant.expiresAt<=now()||grant.expiresAt-now()>86400000)throw fail();
  const bin=pinned(env.VIVIM_WORKSTATION_PROBE_EXE,env.VIVIM_WORKSTATION_PROBE_SHA256,64*1024*1024);
  if(!/^vivim-workstation-probe(?:\.exe)?$/i.test(basename(bin.real)))throw fail();
  return {helper:bin.real,configSha:env.VIVIM_WORKSTATION_ROOTS_SHA256,helperSha:env.VIVIM_WORKSTATION_PROBE_SHA256,expiresAt:grant.expiresAt};
 }
 function read(args){
  if(!exact(args,[]))throw fail();
  try{
   const b=resolve(),childEnv={};if(process.platform==='win32'&&env.SystemRoot)childEnv.SystemRoot=env.SystemRoot;
   const r=run(b.helper,['--native-resources'],{env:childEnv,shell:false,windowsHide:true,encoding:'utf8',timeout:3000,maxBuffer:8192});
   if(r.error||r.status!==0||r.signal||typeof r.stdout!=='string'||Buffer.byteLength(r.stdout)>8192||r.stderr)throw fail();
   const result=validateResourcePayload(JSON.parse(r.stdout)),after=resolve();
   if(JSON.stringify(b)!==JSON.stringify(after))throw fail();
   return {schema:'vivim.windows.native-result/1',actionId:NATIVE_RESOURCE_ACTION,...result,observedAt:new Date(now()).toISOString(),source:'native-windows-probe',scopeRef:'owner-workstation-aggregate',configSha:b.configSha,helperSha:b.helperSha};
  }catch{throw fail();}
 }
 return Object.freeze({read});
}
