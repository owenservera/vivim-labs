// R010: pure daily share and durable account reservation protocol. No inference.
// Controller stores journal outside all agent workspaces. Never clear lock or unknown intent automatically.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {fileURLToPath} from 'node:url';
const campaign=JSON.parse(fs.readFileSync(new URL('./CAMPAIGN-R010.json',import.meta.url),'utf8'));
const num=x=>typeof x==='number'&&Number.isFinite(x);
const nonneg=x=>num(x)&&x>=0;
const fail=x=>{throw Error(x)};
const dayOf=t=>new Intl.DateTimeFormat('en-CA',{timeZone:campaign.budget.local_day,year:'numeric',month:'2-digit',day:'2-digit'}).format(new Date(t));
const hash=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
function windows(snapshot,now){
 if(!snapshot||!['agy','codex'].includes(snapshot.provider)||!snapshot.account_key||snapshot.ordinary_usage_allowed!==true||snapshot.subscription_only!==true||snapshot.paid_overflow_disabled!==true)fail('ENTITLEMENT_OR_BILLING_UNVERIFIED');
 const seen=Date.parse(snapshot.observed_at);
 if(!num(seen)||seen>now||now-seen>300000)fail('STALE_QUOTA');
 const group=snapshot.provider==='codex'?'codex':'Gemini Models';
 const ws=snapshot.groups?.[group]?.windows;
 if(!Array.isArray(ws)||!ws.some(w=>w.duration_minutes===300)||!ws.some(w=>w.duration_minutes===10080))fail('MISSING_WINDOWS');
 const out={};
 for(const w of ws){
  const k=String(w.duration_minutes),reset=Date.parse(w.resets_at);
  if(out[k]||!num(w.duration_minutes)||w.duration_minutes<=0||!nonneg(w.remaining_percent)||w.remaining_percent>100||!num(reset)||reset<=now)fail('INVALID_WINDOW');
  out[k]={remaining:w.remaining_percent,reset:w.resets_at};
 }
 return out;
}
export function openDay(snapshot,now=Date.now()){
 const ws=windows(snapshot,now),allocations={};
 for(const [id,lane] of Object.entries(campaign.lanes)){
  if(lane.provider!==snapshot.provider)continue;
  allocations[id]={selected:lane.selected,limit:{},spent:{},reserved:{},forecast_total:{},observed_decrease:{},segments:{},token_spent:0,turns:0};
  for(const [k,w] of Object.entries(ws)){
   const days=Number(k)>1440?Math.max(1,Math.ceil((Date.parse(w.reset)-now)/86400000)):1;
   allocations[id].limit[k]=Math.max(0,w.remaining-campaign.budget.provider_reserve_pp)/days*lane.pool_share;
   allocations[id].spent[k]=0;allocations[id].reserved[k]=0;allocations[id].forecast_total[k]=0;allocations[id].observed_decrease[k]=0;allocations[id].segments[k]=[];
  }
 }
 return {schema:'vivim.daily-quota-ledger/1',day:dayOf(now),provider:snapshot.provider,account_key:snapshot.account_key,opening_observed_at:snapshot.observed_at,opening_windows:ws,allocations,pending:null,blocked:false,revision:0,history:[]};
}
export function rollDay(ledger,snapshot,now=Date.now()){
 if(ledger.pending||ledger.blocked)fail('RECONCILE_BEFORE_ROLLOVER');
 if(ledger.account_key!==snapshot.account_key||ledger.provider!==snapshot.provider||dayOf(now)<=ledger.day)fail('INVALID_ROLLOVER');
 const x=openDay(snapshot,now);x.revision=ledger.revision+1;x.history=structuredClone(ledger.history);x.prior_days=[...(ledger.prior_days??[]),{day:ledger.day,allocations:ledger.allocations,opening_observed_at:ledger.opening_observed_at}];return x;
}
export function reserve(ledger,{lane,request_id,forecast,model,reasoning_effort,local_caps,task_id},snapshot,now=Date.now()){
 const x=structuredClone(ledger),ws=windows(snapshot,now),l=campaign.lanes[lane],a=x.allocations[lane];
 if(x.day!==dayOf(now)||x.provider!==snapshot.provider||x.account_key!==snapshot.account_key)fail('LEDGER_IDENTITY_OR_DAY_MISMATCH');
 if(!l||!l.selected||!a?.selected||l.provider!==x.provider)fail('LANE_RESERVED_OR_WRONG_PROVIDER');
 if(x.pending||x.blocked)fail('ACCOUNT_BUSY_OR_RECONCILIATION_REQUIRED');
 if(typeof request_id!=='string'||!request_id||!task_id||x.history.some(r=>r.request_id===request_id))fail('INVALID_OR_REPLAYED_INTENT');
 if(typeof model!=='string'||!snapshot.models?.includes(model)||!(x.provider==='agy'?model.startsWith('gemini-')&&!model.includes('orchestrator'):model.startsWith('gpt-')))fail('MODEL_NOT_ENTITLED');
 if(x.provider==='codex'&&!snapshot.model_efforts?.[model]?.includes(reasoning_effort))fail('EFFORT_NOT_ENTITLED');
 const group=x.provider==='agy'?'Gemini Models':'codex';
 if(snapshot.model_groups?.[model]!==group)fail('MODEL_GROUP_UNKNOWN');
 if(!local_caps||!nonneg(local_caps.daily_tokens)||!nonneg(local_caps.token_ceiling)||!nonneg(local_caps.predicted_tokens)||local_caps.predicted_tokens===0||!Number.isInteger(local_caps.turn_ceiling)||local_caps.turn_ceiling<1||a.turns>=local_caps.turn_ceiling||Math.max(local_caps.daily_tokens,a.token_spent)+local_caps.predicted_tokens>local_caps.token_ceiling)fail('LOCAL_CAP_UNCONFIGURED_OR_EXHAUSTED');
 if(Object.keys(ws).length!==Object.keys(a.limit).length)fail('WINDOW_SHAPE_CHANGED');
 for(const [k,w] of Object.entries(ws)){
  const cost=forecast?.[k];if(!num(cost)||cost<=0||!nonneg(a.limit[k]))fail('UNKNOWN_FORECAST');
  if(w.remaining-cost<campaign.budget.provider_reserve_pp)fail('PROVIDER_RESERVE');
  if(a.spent[k]+a.reserved[k]+cost>a.limit[k]+1e-9)fail('LANE_DAILY_SHARE_EXHAUSTED');
  a.reserved[k]+=cost;
 }
 x.pending={request_id,task_id,lane,model,reasoning_effort:reasoning_effort??null,forecast:structuredClone(forecast),predicted_tokens:local_caps.predicted_tokens,before:ws,reserved_at:new Date(now).toISOString(),status:'RESERVED'};
 x.revision++;return x;
}
export function settle(ledger,{request_id,outcome,attribution_confirmed,actual_tokens},snapshot,now=Date.now()){
 const x=structuredClone(ledger),p=x.pending;
 if(!p||p.request_id!==request_id)fail('UNKNOWN_INTENT');
 if(!['VERIFIED','FAILED','CANCELLED','UNKNOWN'].includes(outcome))fail('INVALID_OUTCOME');
 let after=null;
 try{if(snapshot.account_key===x.account_key&&snapshot.provider===x.provider)after=windows(snapshot,now);}catch{}
 const a=x.allocations[p.lane],charged={};let ambiguous=!after||attribution_confirmed!==true||outcome==='UNKNOWN';
 for(const [k,b] of Object.entries(p.before)){
  const same=after?.[k]?.reset===b.reset;
  if(!same)ambiguous=true;
  const delta=same?Math.max(0,b.remaining-after[k].remaining):0;
  a.forecast_total[k]+=p.forecast[k];a.observed_decrease[k]+=delta;
  if(same&&!a.segments[k].includes(b.reset))a.segments[k].push(b.reset);
  // One rounding allowance per lane/window identity, not per reasoning turn.
  const total=Math.max(a.forecast_total[k],a.observed_decrease[k]+a.segments[k].length);
  charged[k]=Math.max(0,total-a.spent[k]);a.reserved[k]=0;a.spent[k]=total;
 }
 a.turns++;a.token_spent+=Math.max(p.predicted_tokens,nonneg(actual_tokens)?actual_tokens:0);
 x.history.push({...p,outcome,charged,actual_tokens:nonneg(actual_tokens)?actual_tokens:null,settled_at:new Date(now).toISOString(),ambiguous});
 x.pending=null;x.blocked=ambiguous;x.revision++;return x;
}
// Atomic immutable revision publication under exclusive account lock.
// Crash leaves lock/intent behind; reconcile manually, never delete to retry blindly.
// A controller must use the SAME account directory for all lanes and persist the
// returned reservation before spawning. No automatic recovery or provider invocation.
export function transact(dir,fn){
 fs.mkdirSync(dir,{recursive:true});const lock=path.join(dir,'writer.lock');fs.mkdirSync(lock);
 let published=false,writing=false;
 try{
  const names=fs.readdirSync(dir).filter(x=>/^\d{12}\.json$/.test(x)).sort();
  let prior=null;
  for(let i=0;i<names.length;i++){
   if(names[i]!==String(i).padStart(12,'0')+'.json')fail('JOURNAL_GAP');
   const record=JSON.parse(fs.readFileSync(path.join(dir,names[i]),'utf8'));
   if(record.previous_hash!==(prior?.hash??null)||record.hash!==hash({previous_hash:record.previous_hash,state:record.state})||record.state.revision!==i)fail('JOURNAL_CORRUPTION');
   prior=record;
  }
  const state=fn(prior?.state??null);
  if(state.revision!==names.length)fail('REVISION_MISMATCH');
  const body={previous_hash:prior?.hash??null,state},record={...body,hash:hash(body)};
  const target=path.join(dir,String(state.revision).padStart(12,'0')+'.json');
  const temp=path.join(dir,'.'+crypto.randomUUID()+'.tmp');
  writing=true;const fd=fs.openSync(temp,'wx');try{fs.writeFileSync(fd,JSON.stringify(record,null,2));fs.fsyncSync(fd);}finally{fs.closeSync(fd);}
  if(fs.existsSync(target))fail('REVISION_ALREADY_EXISTS');
  fs.renameSync(temp,target);published=true;
  return state;
 }finally{if(published||!writing)fs.rmdirSync(lock);else{ /* ambiguous publication; controller must investigate lock */ }}
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const i=JSON.parse(fs.readFileSync(0,'utf8'));
 if(process.argv[2]!=='preview')fail('Only pure preview is exposed on CLI; no launch.');
 console.log(JSON.stringify(openDay(i),null,2));
}
