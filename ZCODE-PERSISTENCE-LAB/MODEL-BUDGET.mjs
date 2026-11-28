// R008 deterministic model admission. Pure selector: never calls a model or purchases credits.
// Controller must serialize admissions and persist a reservation before provider invocation.
import fs from 'node:fs';
import {fileURLToPath} from 'node:url';
const policy=JSON.parse(fs.readFileSync(new URL('./MODEL-BUDGET-POLICY.json',import.meta.url),'utf8'));
const validNumber=x=>typeof x==='number'&&Number.isFinite(x);
const stop=(reason,extra={})=>({decision:'WAIT',reason,...extra});
export function route(input,now=Date.now()){
 const {lane,task,snapshot,local}=input;
 const l=policy.lanes[lane];
 if(!l)return stop('UNKNOWN_LANE');
 const t=policy.tasks[task];
 if(!t)return stop('UNCLASSIFIED_TASK');
 if(t.route==='native')return {decision:'NATIVE',reason:'NO_INFERENCE_REQUIRED'};
 if(l.cli==='codex')return stop('GEMINI_ROUTE_NOT_VERIFIED');
 if(!snapshot||snapshot.provider!=='agy'||!snapshot.account_key)return stop('NO_ACCOUNT_QUOTA');
 if(snapshot.subscription_only!==true||snapshot.paid_overflow_disabled!==true)return stop('BILLING_NOT_VERIFIED');
 if(snapshot.ordinary_usage_allowed!==true)return stop('ENTITLEMENT_NOT_CONFIRMED');
 const at=Date.parse(snapshot.observed_at);
 if(!validNumber(at)||at>now||now-at>policy.rules.max_snapshot_age_seconds*1000)return stop('STALE_OR_INVALID_QUOTA');
 if(!local||local.account_key!==snapshot.account_key||!Number.isInteger(local.inflight)||local.inflight<0)return stop('SHARED_ACCOUNT_LEDGER_MISSING');
 if(local.inflight>=policy.rules.max_inflight_per_account)return stop('ACCOUNT_BUSY');
 if(local.rate_limit_blocked===true)return stop('RATE_LIMIT_REFRESH_REQUIRED');
 const day=new Intl.DateTimeFormat('en-CA',{timeZone:policy.rules.local_day_timezone,year:'numeric',month:'2-digit',day:'2-digit'}).format(new Date(now));
 if(local.day!==day)return stop('LOCAL_DAY_LEDGER_REFRESH_REQUIRED',{day});
 for(const k of ['daily_turn_limit','daily_token_limit','daily_turns','daily_tokens','predicted_tokens']){
  if(!validNumber(local[k])||local[k]<0)return stop('LOCAL_BUDGET_UNCONFIGURED',{field:k});
 }
 if(!Number.isInteger(local.daily_turn_limit)||!Number.isInteger(local.daily_turns)||local.predicted_tokens<=0)return stop('INVALID_LOCAL_BUDGET');
 if(local.daily_turns+1>local.daily_turn_limit||local.daily_tokens+local.predicted_tokens>local.daily_token_limit)return stop('LOCAL_DAILY_CAP');
 const reserve=task==='critical'?policy.rules.critical_reserve_percent:policy.rules.reserve_percent;
 const rejects=[];
 for(const model of t.preferred){
  if(!model.startsWith('gemini-')||!Array.isArray(snapshot.models)||!snapshot.models.includes(model)){rejects.push([model,'NOT_ENTITLED']);continue;}
  const group=snapshot.model_groups?.[model];
  const g=group&&snapshot.groups?.[group];
  if(!g||!Array.isArray(g.windows)||g.windows.length===0){rejects.push([model,'GROUP_UNKNOWN']);continue;}
  const durations=new Set(g.windows.map(w=>w.duration_minutes));
  if(!durations.has(300)||!durations.has(10080)){rejects.push([model,'MISSING_QUOTA_WINDOWS']);continue;}
  // Costs are estimated in percentage points of each observed provider window,
  // never converted from tokens. Unknown costs cannot be treated as free.
  let reason=null;
  for(const w of g.windows){
   const delta=local.predicted_quota_pp?.[model]?.[String(w.duration_minutes)];
   if(!validNumber(w.remaining_percent)||w.remaining_percent<0||w.remaining_percent>100||!validNumber(delta)||delta<0){reason='WINDOW_OR_COST_UNKNOWN';break;}
   const reset=Date.parse(w.resets_at);
   if(!validNumber(reset)||reset<=now){reason='WINDOW_NEEDS_REFRESH';break;}
   if(w.remaining_percent-delta<reserve){reason='RESERVE_PROTECTED';break;}
  }
  if(reason){rejects.push([model,reason]);continue;}
  return {decision:'ADMIT_CANDIDATE',lane,provider:'agy',model,group,reserve_percent:reserve,account_key:snapshot.account_key,snapshot_observed_at:snapshot.observed_at,local_day:day,estimated_tokens:local.predicted_tokens,estimated_quota_pp:local.predicted_quota_pp[model],requires_atomic_reservation:true,rejected_candidates:rejects};
 }
 return stop('NO_ELIGIBLE_GEMINI',{rejected_candidates:rejects});
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const input=JSON.parse(fs.readFileSync(0,'utf8'));
 console.log(JSON.stringify(route(input),null,2));
}
