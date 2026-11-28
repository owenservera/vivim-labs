// R008 deterministic model admission. Pure selector: never calls a model or purchases credits.
// Controller must serialize admissions and persist a reservation before provider invocation.
import fs from 'node:fs';
import {fileURLToPath} from 'node:url';
const policy=JSON.parse(fs.readFileSync(new URL('./MODEL-BUDGET-POLICY.json',import.meta.url),'utf8'));
const experiment=JSON.parse(fs.readFileSync(new URL('./MODEL-OPTIMIZATION-EXPERIMENTS.json',import.meta.url),'utf8'));
const validNumber=x=>typeof x==='number'&&Number.isFinite(x);
const stop=(reason,extra={})=>({decision:'WAIT',reason,...extra});
export function route(input,now=Date.now()){return evaluate(input,now,false);}
// Offline comparator returns SIMULATED_CANDIDATE, never an executable admission.
export function simulate(input,now=Date.now()){return evaluate(input,now,true);}
function evaluate(input,now,simulation){
 const {lane,task,snapshot,local}=input;
 const l=policy.lanes[lane];
 if(!l)return stop('UNKNOWN_LANE');
 const t=policy.tasks[task];
 if(!t)return stop('UNCLASSIFIED_TASK');
 if(t.route==='native')return {decision:'NATIVE',reason:'NO_INFERENCE_REQUIRED'};
 if(!simulation&&!l.selected)return stop('LANE_RESERVED');
 const provider=simulation?'agy':l.cli;
 const variant=experiment.lanes[lane];
 if(!variant)return stop('NO_OPTIMIZATION_POLICY');
 if(!snapshot||snapshot.provider!==provider||!snapshot.account_key)return stop('NO_ACCOUNT_QUOTA');
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
 const ledger=input.ledger,share=ledger?.allocations?.[lane];
 if(!simulation&&(!ledger||ledger.provider!==provider||ledger.account_key!==snapshot.account_key||ledger.day!==day||!share?.selected||ledger.pending||ledger.blocked))return stop('DAILY_SHARE_NOT_READY');
 const rejects=[],eligible=[];
 const priorities=provider==='codex'?policy.codex_tasks?.[task]:(!simulation&&policy.live_gemini_tasks?.[task]||t.preferred);
 if(!Array.isArray(priorities)||!priorities.length)return stop('NO_TASK_MODEL_POLICY');
 const bootstrap=!simulation&&policy.bootstrap?.[lane]&&local.task_id===policy.bootstrap[lane].task_id&&task===policy.bootstrap[lane].task&&local.adaptive_decisions===0?policy.bootstrap[lane]:null;
 let preferred=bootstrap?[bootstrap.model]:priorities;
 if(variant.id==='verification_cascade'){
  const previous=local.task_attempts??[];
  if(!Array.isArray(previous)||previous.length>=experiment.constants.max_attempts_per_task)return stop('CASCADE_ATTEMPTS_EXHAUSTED');
  if(previous.some((a,i)=>a?.verified_failure!==true||a.model!==preferred[i]))return stop('CASCADE_RECONCILIATION_REQUIRED');
  preferred=preferred.slice(previous.length,previous.length+1);
 }
 for(const candidate of preferred){
  const [model,effort]=candidate.split('@');
  const familyAllowed=provider==='codex'?model.startsWith('gpt-')&&snapshot.model_efforts?.[model]?.includes(effort):model.startsWith('gemini-')&&!model.includes('orchestrator');
  if(!familyAllowed||!Array.isArray(snapshot.models)||!snapshot.models.includes(model)){rejects.push([candidate,'NOT_ENTITLED']);continue;}
  const group=snapshot.model_groups?.[model];
  const g=group&&snapshot.groups?.[group];
  if(group!==(provider==='codex'?'codex':'Gemini Models')||!g||!Array.isArray(g.windows)||g.windows.length===0){rejects.push([model,'GROUP_UNKNOWN']);continue;}
  const durations=new Set(g.windows.map(w=>w.duration_minutes));
  if(!durations.has(300)||!durations.has(10080)){rejects.push([model,'MISSING_QUOTA_WINDOWS']);continue;}
  // Costs are estimated in percentage points of each observed provider window,
  // never converted from tokens. Unknown costs cannot be treated as free.
  let reason=null,normalizedCost=0;
  for(const w of g.windows){
   const delta=local.predicted_quota_pp?.[candidate]?.[String(w.duration_minutes)];
   if(!validNumber(w.remaining_percent)||w.remaining_percent<0||w.remaining_percent>100||!validNumber(delta)||delta<=0){reason='WINDOW_OR_COST_UNKNOWN';break;}
   const reset=Date.parse(w.resets_at);
   if(!validNumber(reset)||reset<=now){reason='WINDOW_NEEDS_REFRESH';break;}
   if(w.remaining_percent-delta<reserve){reason='RESERVE_PROTECTED';break;}
   const k=String(w.duration_minutes);
   if(!simulation&&(!validNumber(share.limit?.[k])||!validNumber(share.spent?.[k])||!validNumber(share.reserved?.[k])||share.spent[k]+share.reserved[k]+delta>share.limit[k]+1e-9)){reason='LANE_DAILY_SHARE';break;}
   normalizedCost=Math.max(normalizedCost,delta/Math.max(0.001,w.remaining_percent-reserve));
   if(variant.id==='paced_allocation'){
    const elapsed=Math.max(0,Math.min(1,1-(reset-now)/(w.duration_minutes*60000)));
    const envelope=(100-reserve)*Math.min(1,elapsed+experiment.constants.burst_fraction);
    if(100-w.remaining_percent+delta>envelope){reason='PACING_ENVELOPE';break;}
   }
  }
  if(reason){rejects.push([model,reason]);continue;}
  eligible.push({model,effort,candidate,group,normalizedCost,order:priorities.indexOf(candidate)});
 }
 if(!eligible.length)return stop(simulation||provider==='agy'?'NO_ELIGIBLE_GEMINI':'NO_ELIGIBLE_CODEX',{policy:variant.id,rejected_candidates:rejects});
 let chosen=eligible[0],selection_reason='STABLE_TASK_PRIORITY';
 if(bootstrap){selection_reason='OWNER_PINNED_BOOTSTRAP';}
 else if(variant.id==='paced_allocation'){
  chosen=[...eligible].sort((a,b)=>a.normalizedCost-b.normalizedCost||a.order-b.order)[0];
  selection_reason='MINIMUM_NORMALIZED_QUOTA_COST';
 }else if(variant.id==='verification_cascade'){
  selection_reason='VERIFIED_FAILURE_STAGE_'+(1+(local.task_attempts?.length??0));
 }else if(variant.id==='evidence_adaptive'){
  const hist=local.verified_model_stats?.[task]??{};
  const stats=x=>{const s=hist[x.candidate];return s&&Number.isInteger(s.attempts)&&Number.isInteger(s.successes)&&s.attempts>=0&&s.successes>=0&&s.successes<=s.attempts?s:{attempts:0,successes:0};};
  const total=eligible.reduce((n,x)=>n+stats(x).attempts,0);
  const exploration=Number.isInteger(local.adaptive_decisions)&&local.adaptive_decisions>=0&&local.adaptive_decisions%experiment.constants.exploration_every===0;
  const z=experiment.constants.confidence_z;
  function score(x){const s=stats(x),n=s.attempts;if(n<experiment.constants.min_samples)return -1;const p=s.successes/n;const lower=(p+z*z/(2*n)-z*Math.sqrt((p*(1-p)+z*z/(4*n))/n))/(1+z*z/n);return lower/x.normalizedCost;}
  if(exploration){chosen=[...eligible].sort((a,b)=>stats(a).attempts-stats(b).attempts||a.normalizedCost-b.normalizedCost||a.order-b.order)[0];selection_reason='BOUNDED_EXPLORATION';}
  else if(total>=experiment.constants.min_samples&&eligible.some(x=>score(x)>=0)){chosen=[...eligible].sort((a,b)=>score(b)-score(a)||a.order-b.order)[0];selection_reason='VERIFIED_SUCCESS_LCB_PER_QUOTA';}
  else selection_reason='COLD_START_REFERENCE';
 }
 const {model,group,effort,candidate}=chosen;
 return {decision:simulation?'SIMULATED_CANDIDATE':'ADMIT_CANDIDATE',lane,provider,policy:variant.id,model,reasoning_effort:effort??null,group,selection_reason,reserve_percent:reserve,account_key:snapshot.account_key,snapshot_observed_at:snapshot.observed_at,local_day:day,estimated_tokens:local.predicted_tokens,estimated_quota_pp:local.predicted_quota_pp[candidate],requires_atomic_reservation:!simulation,rejected_candidates:rejects};

}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const input=JSON.parse(fs.readFileSync(0,'utf8'));
 console.log(JSON.stringify(process.argv[2]==='simulate'?simulate(input):route(input),null,2));
}
