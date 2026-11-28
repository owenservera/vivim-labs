// R008 live quota observer: documented CLI slash command, observed headless TSV.
// No inference, account switching, credit purchases or private provider endpoints.
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
export function parseQuota(text){
 const groups={};
 for(const line of text.split(/\r?\n/)){
  const m=/^([^\t]+)\t(Weekly|Five Hour) Limit Remaining\t(\d+(?:\.\d+)?)%\t([^\t]+)$/.exec(line.trim());
  if(!m)continue;
  const [,group,kind,pct,reset]=m;
  const duration_minutes=kind==='Weekly'?10080:300;
  const remaining_percent=Number(pct);
  if(remaining_percent<0||remaining_percent>100||!Number.isFinite(Date.parse(reset)))throw Error('invalid quota row');
  const g=groups[group]??={windows:[]};
  if(g.windows.some(w=>w.duration_minutes===duration_minutes))throw Error('duplicate quota window');
  g.windows.push({duration_minutes,remaining_percent,resets_at:reset});
 }
 if(!Object.keys(groups).length)throw Error('no recognized quota rows');
 return groups;
}
export function parseModels(text){
 return [...new Set(text.split(/\r?\n/).map(x=>x.split('\t')[0].trim()).filter(x=>/^gemini-[a-z0-9.-]+$/.test(x)&&!x.includes('orchestrator')))];
}
export function observe({account_key,subscription_only=false,paid_overflow_disabled=false,ordinary_usage_allowed=null}){
 if(!account_key||typeof account_key!=='string')throw Error('Controller must supply opaque account key shared across its lanes');
 const exe=path.join(process.env.LOCALAPPDATA||os.tmpdir(),'agy','bin','agy.exe');
 const scratch=fs.mkdtempSync(path.join(os.tmpdir(),'vivim-quota-'));
 function cli(args){
  const r=spawnSync(exe,args,{cwd:scratch,encoding:'utf8',input:'',timeout:30000,windowsHide:true,maxBuffer:1024*1024});
  if(r.error||r.status!==0)throw Error('quota/catalog command failed; do not launch inference');
  return r.stdout;
 }
 try{
  const models=parseModels(cli(['models']));
  if(!models.length)throw Error('no Gemini models listed');
  const groups=parseQuota(cli(['--print-timeout','10s','--print','/usage']));
  // Only map models when the report actually has a single, explicit Gemini group.
  // If the provider later splits groups, require controller mapping based on new evidence.
  const geminiGroups=Object.keys(groups).filter(x=>/^Gemini\b/i.test(x));
  const model_groups={};
  if(geminiGroups.length===1&&geminiGroups[0]==='Gemini Models')for(const m of models)model_groups[m]=geminiGroups[0];
  return {provider:'agy',account_key,observed_at:new Date().toISOString(),models,groups,model_groups,subscription_only:subscription_only===true,paid_overflow_disabled:paid_overflow_disabled===true,ordinary_usage_allowed:ordinary_usage_allowed===true?true:null,entitlement_source:'controller attestation; quota report does not prove billing settings or ordinary-usage authorization',source:'agy --print /usage + agy models'};
 }finally{fs.rmSync(scratch,{recursive:true,force:true});}
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const input=JSON.parse(fs.readFileSync(0,'utf8'));
 console.log(JSON.stringify(observe(input),null,2));
}
