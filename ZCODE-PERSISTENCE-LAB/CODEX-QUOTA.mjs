// R010 native subscription quota and model catalogue observation. No inference.
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import {spawn} from 'node:child_process';
import {createInterface} from 'node:readline';
import {fileURLToPath} from 'node:url';
export async function observeCodex({account_key,subscription_only=false,paid_overflow_disabled=false}){
 if(typeof account_key!=='string'||!account_key)throw Error('OPAQUE_ACCOUNT_KEY_REQUIRED');
 const entry=path.join(process.env.APPDATA,'npm','node_modules','@openai','codex','bin','codex.js');
 const cwd=fs.mkdtempSync(path.join(os.tmpdir(),'vivim-codex-quota-'));
 const p=spawn(process.execPath,[entry,'app-server'],{cwd,stdio:['pipe','pipe','pipe'],windowsHide:true});
 let id=0,pending=new Map();
 const rejectAll=e=>{for(const v of pending.values())v.reject(e);pending.clear();};
 const lines=createInterface({input:p.stdout});
 lines.on('line',l=>{try{const x=JSON.parse(l),a=pending.get(x.id);if(a){pending.delete(x.id);x.error?a.reject(Error(x.error.message||'RPC_ERROR')):a.resolve(x.result);}}catch{}});
 p.stderr.resume();p.on('error',rejectAll);p.on('exit',()=>rejectAll(Error('APP_SERVER_EXIT')));
 const timer=setTimeout(()=>{rejectAll(Error('APP_SERVER_TIMEOUT'));p.kill();},25000);
 function req(method,params){return new Promise((resolve,reject)=>{const n=++id;pending.set(n,{resolve,reject});p.stdin.write(JSON.stringify({jsonrpc:'2.0',id:n,method,params})+'\n');});}
 try{
  await req('initialize',{clientInfo:{name:'vivim-budget-observer',version:'1.0'},capabilities:{experimentalApi:true}});
  p.stdin.write(JSON.stringify({jsonrpc:'2.0',method:'initialized'})+'\n');
  const provider=await req('modelProvider/list',{});
  if(provider.defaultProviderId!=='openai'||(provider.data||[]).some(x=>x.id!=='openai'))throw Error('NATIVE_OPENAI_PROVIDER_NOT_EXCLUSIVE');
  const list=await req('model/list',{}),q=await req('account/rateLimits/read',{});
  if(list.nextCursor)throw Error('PARTIAL_CATALOGUE');
  const entries=(list.data||[]).filter(x=>/^gpt-/.test(x.model)&&Array.isArray(x.supportedReasoningEfforts));
  const snapshot=q.rateLimitsByLimitId?.codex||q.rateLimits;
  if(!snapshot||snapshot.limitId!=='codex'||!snapshot.primary||!snapshot.secondary)throw Error('CODEX_QUOTA_UNAVAILABLE');
  if(!entries.length)throw Error('MODEL_CATALOGUE_UNAVAILABLE');
  const windows=[snapshot.primary,snapshot.secondary].map(w=>{if(!Number.isFinite(w.usedPercent)||!Number.isFinite(w.windowDurationMins)||!Number.isFinite(w.resetsAt))throw Error('INVALID_WINDOW');return {duration_minutes:w.windowDurationMins,remaining_percent:100-w.usedPercent,resets_at:new Date(w.resetsAt*1000).toISOString()};});
  return {provider:'codex',account_key,observed_at:new Date().toISOString(),ordinary_usage_allowed:q.ordinaryUsageAllowed===true&&snapshot.spendControlReached!==true&&!snapshot.rateLimitReachedType,subscription_only:subscription_only===true,paid_overflow_disabled:paid_overflow_disabled===true,models:entries.map(x=>x.model),model_efforts:Object.fromEntries(entries.map(x=>[x.model,x.supportedReasoningEfforts.map(e=>e.reasoningEffort)])),model_groups:Object.fromEntries(entries.map(x=>[x.model,'codex'])),groups:{codex:{windows}},source:'native app-server account/rateLimits/read + model/list',plan_type:snapshot.planType,entitlement_source:'backend ordinaryUsageAllowed; subscription and paid-overflow booleans require independent verification'};
 }finally{clearTimeout(timer);lines.close();p.stdin.end();p.kill();try{fs.rmSync(cwd,{recursive:true,force:true,maxRetries:2,retryDelay:200});}catch(e){console.error('Quota scratch cleanup deferred: '+e.code);}}
}
if(process.argv[1]===fileURLToPath(import.meta.url)){const input=JSON.parse(fs.readFileSync(0,'utf8'));console.log(JSON.stringify(await observeCodex(input),null,2));}
