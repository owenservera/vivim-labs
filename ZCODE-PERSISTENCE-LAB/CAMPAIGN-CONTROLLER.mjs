// R011 controller: one bounded, journaled turn; never schedules background work.
// No unconfined production adapter is installed. Only controller-owned adapters
// and independent evaluators may call this API. Worker replies are proposals.
import crypto from 'node:crypto';
import fs from 'node:fs';
import {openDay,rollDay,reserve,settle,transact} from './CAMPAIGN-BUDGET.mjs';
import {route} from './MODEL-BUDGET.mjs';
import {judge,contextDigest,reopen} from './SUFFICIENCY.mjs';
const campaign=JSON.parse(fs.readFileSync(new URL('./CAMPAIGN-R010.json',import.meta.url),'utf8'));
const goal=fs.readFileSync(new URL('./COMPLETION-GOAL.md',import.meta.url),'utf8');
const charter=JSON.parse(fs.readFileSync(new URL('./SYSTEM-GOALS-R013.json',import.meta.url),'utf8'));
const goalIds=new Set(charter.goals.map(g=>g.id));
const hex=x=>typeof x==='string'&&/^[0-9a-f]{64}$/.test(x);
const str=x=>typeof x==='string'&&x.length>0&&x.length<=65536;
const fail=x=>{throw Error(x)};
const hash=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
const finalStates=new Set(['COMPLETE_VERIFIED','FAILED_BOUNDED','CANCELLED']);
const maxAttempts=3;
const tasks=new Set(['extract','routine','implementation','debug','architecture','critical']);
function advance(x,c){x.controller=c;x.revision++;return x;}
function checkedLane(lane,provider){
 const l=campaign.lanes[lane];if(!l?.selected||l.provider!==provider)fail('LANE_NOT_SELECTED');return l;
}
export function normalizedContract(p){
 if(!p||!str(p.id)||!str(p.release_boundary)||!str(p.out_of_scope)||!Array.isArray(p.mandatory)||p.mandatory.length<1||p.mandatory.length>128)fail('INVALID_CONTRACT');
 if(!str(p.purpose)||!str(p.operating_envelope)||!str(p.evidence_standard)||!Array.isArray(p.reopening_triggers)||!p.reopening_triggers.length||p.reopening_triggers.some(x=>!str(x))||!Array.isArray(p.optional_backlog)||p.optional_backlog.length>128)fail('INVALID_SUFFICIENCY_CONTEXT');
 const optional=p.optional_backlog.map(o=>{
  if(!o||!str(o.id)||!str(o.description)||!str(o.expected_benefit)||!str(o.incremental_cost)||!str(o.uncertainty)||!str(o.decision_criterion))fail('INVALID_OPTIONAL_OPPORTUNITY');
  return {id:o.id,description:o.description,expected_benefit:o.expected_benefit,incremental_cost:o.incremental_cost,uncertainty:o.uncertainty,decision_criterion:o.decision_criterion};
 }).sort((a,b)=>a.id.localeCompare(b.id));
 if(new Set(optional.map(o=>o.id)).size!==optional.length)fail('DUPLICATE_OPTIONAL_OPPORTUNITY');
 const rows=p.mandatory.map(r=>{
  if(!str(r.id)||!str(r.behavior)||!str(r.oracle)||!str(r.falsifier)||!tasks.has(r.task)||!Array.isArray(r.depends_on)||r.depends_on.some(x=>!str(x))||!Number.isInteger(r.priority)||r.priority<0||r.priority>100)fail('INVALID_OBLIGATION');
  if(!Array.isArray(r.goal_ids)||!r.goal_ids.length||r.goal_ids.some(g=>!goalIds.has(g)))fail('INVALID_GOAL_COVERAGE');
  return {id:r.id,behavior:r.behavior,oracle:r.oracle,falsifier:r.falsifier,task:r.task,priority:r.priority,depends_on:[...new Set(r.depends_on)].sort(),goal_ids:[...new Set(r.goal_ids)].sort()};
 }).sort((a,b)=>a.id.localeCompare(b.id));
 const covered=new Set(rows.flatMap(r=>r.goal_ids));for(const g of goalIds)if(!covered.has(g))fail('MANDATORY_GOAL_OMITTED:'+g);
 const by=new Map(rows.map(r=>[r.id,r]));if(by.size!==rows.length)fail('DUPLICATE_OBLIGATION');
 const active=new Set(),done=new Set();
 function visit(id){if(done.has(id))return;if(active.has(id)||!by.has(id))fail('CONTRACT_CYCLE_OR_MISSING_DEPENDENCY');active.add(id);for(const d of by.get(id).depends_on)visit(d);active.delete(id);done.add(id);}
 for(const r of rows)visit(r.id);
 if(optional.some(o=>by.has(o.id)))fail('MANDATORY_OPTIONAL_OVERLAP');
 const body={id:p.id,version:1,goal_charter_digest:hash(charter),purpose:p.purpose,operating_envelope:p.operating_envelope,evidence_standard:p.evidence_standard,reopening_triggers:[...new Set(p.reopening_triggers)].sort(),optional_backlog:optional,release_boundary:p.release_boundary,out_of_scope:p.out_of_scope,mandatory:rows,frozen:true};
 return {...body,digest:hash(body)};
}
export function createRun(ledger,{lane,run_id,seed_digest,workspace_digest}){
 checkedLane(lane,ledger.provider);
 if(ledger.controller||!str(run_id)||!hex(seed_digest)||!hex(workspace_digest))fail('INVALID_RUN_IDENTITY');
 const c={schema:'vivim.campaign.controller/1',lane,run_id,seed_digest,goal_charter_digest:hash(charter),candidate_digest:workspace_digest,phase:'DISCOVER_COMPLETENESS',status:'READY',contract:null,obligations:{},discovery_attempts:0,session:null,incarnation:0,turns:0,stats:{},pending:null,last_checkpoint:null,last_result:null,sufficiency:null,historical_sufficiency:[]};
 return advance(ledger,c);
}
function ready(c){
 const rows=c.contract.mandatory,by=new Map(rows.map(x=>[x.id,x]));
 function downstream(id){const seen=new Set();const walk=x=>{for(const r of rows)if(r.depends_on.includes(x)&&!seen.has(r.id)){seen.add(r.id);walk(r.id);}};walk(id);return [...seen].filter(x=>c.obligations[x].state!=='PASS').length;}
 return rows.filter(r=>c.obligations[r.id].state!=='PASS'&&r.depends_on.every(d=>c.obligations[d].state==='PASS')).sort((a,b)=>downstream(b.id)-downstream(a.id)||b.priority-a.priority||a.id.localeCompare(b.id));
}
export function next(c){
 if(finalStates.has(c.status))return {kind:'STOP',reason:c.status};
 if(c.pending||c.status==='WAITING_RECONCILIATION')return {kind:'STOP',reason:'WAITING_RECONCILIATION'};
 if(c.phase==='DISCOVER_COMPLETENESS')return c.discovery_attempts>=maxAttempts?{kind:'STOP',reason:'FAILED_BOUNDED'}:{kind:'INFER',task:'architecture',task_id:'discover-contract'};
 if(c.phase==='BUILD'){
  const r=ready(c)[0];
  if(!r){if(Object.values(c.obligations).every(o=>o.state==='PASS'))return {kind:'QUALIFY'};return {kind:'STOP',reason:'FAILED_BOUNDED'};}
  if(c.obligations[r.id].attempts>=maxAttempts)return {kind:'STOP',reason:'FAILED_BOUNDED'};
  return {kind:'INFER',task:c.obligations[r.id].attempts?'debug':r.task,task_id:r.id,obligation:r};
 }
 if(c.phase==='QUALIFY')return {kind:'QUALIFY'};
 return {kind:'STOP',reason:'INVALID_PHASE'};
}
function promptFor(c,n){
 const shared='Work only inside your assigned workspace. Return a bounded proposal/checkpoint; no completion claim is authoritative. Do not access controller or external files.';
 if(n.task_id==='discover-contract')return goal+'\n'+shared+'\nPropose JSON contract: {id,purpose,operating_envelope,evidence_standard,reopening_triggers:[string],optional_backlog:[{id,description,expected_benefit,incremental_cost,uncertainty,decision_criterion}],release_boundary,out_of_scope,mandatory:[{id,behavior,oracle,falsifier,task,priority,depends_on,goal_ids}]}. Task values: extract, routine, implementation, debug, architecture, critical. Priority 0-100. Inspect supplied objective and source. Do not implement yet.';
 return shared+'\nFrozen contract digest: '+c.contract.digest+'\nCurrent obligation: '+JSON.stringify(n.obligation)+'\nPrior independent result: '+JSON.stringify(c.obligations[n.task_id].last_evidence??null)+'\nImplement only the bounded obligation and verify it locally. Report actual changes, test evidence and remaining work. A trusted evaluator will determine acceptance.';
}
function checkBoundary(b,c,snapshot){
 if(!b||b.authority!=='controller-owned-isolation-verifier'||b.lane!==c.lane||b.run_id!==c.run_id||b.account_key!==snapshot.account_key||b.provider!==snapshot.provider||b.isolation_verified!==true||b.scoped_write_verified!==true||b.billing_verified!==true||!hex(b.evidence_digest)||b.workspace_digest!==c.candidate_digest)fail('BOUNDARY_OR_CANDIDATE_UNVERIFIED');
}
function hold(ledger,reason){const c=structuredClone(ledger.controller);c.status=reason;c.last_result={reason};return advance(ledger,c);}
function checkReceipt(r,p){
 return r&&r.request_id===p.request_id&&r.model===p.model&&(p.reasoning_effort===null||r.reasoning_effort===p.reasoning_effort)&&str(r.session_id)&&(!p.resume_session||p.resume_session===r.session_id)&&r.exit_code===0&&r.terminal===true&&r.process_fenced===true&&hex(r.raw_digest);
}
function finish(ledger,transport,verification,after,now){
 const c=structuredClone(ledger.controller),p=c.pending;
 if(!p||ledger.pending?.request_id!==p.request_id)fail('INTENT_MISMATCH');
 const matched=checkReceipt(transport,p);
 const v=verification;
 const independent=matched&&v?.authority==='controller-independent-evaluator'&&v.request_id===p.request_id&&hex(v.candidate_digest)&&hex(v.evidence_digest)&&hex(v.checkpoint_digest)&&['PASS','FAIL'].includes(v.result);
 // Unknown process/effect, identity or evaluator state is not safe to replay.
 const outcome=independent?(v.result==='PASS'?'VERIFIED':'FAILED'):'UNKNOWN';
 const x=settle(ledger,{request_id:p.request_id,outcome,attribution_confirmed:independent&&v.quota_attribution_confirmed===true,actual_tokens:transport?.actual_tokens},after,now);
 c.pending=null;c.turns++;c.last_result={request_id:p.request_id,outcome,transport_digest:transport?.raw_digest??null,evidence_digest:independent?v.evidence_digest:null};
 if(independent){
  c.candidate_digest=v.candidate_digest;c.last_checkpoint=v.checkpoint_digest;
  c.session={id:transport.session_id,model:p.model,reasoning_effort:p.reasoning_effort,incarnation:p.incarnation};
  c.incarnation=p.incarnation;
  const key=p.reasoning_effort?p.model+'@'+p.reasoning_effort:p.model;
  const by=c.stats[p.task]??={};const s=by[key]??={attempts:0,successes:0};s.attempts++;if(v.result==='PASS')s.successes++;
  if(p.task_id==='discover-contract'){
   c.discovery_attempts++;
   if(v.result==='PASS'&&v.scope_review==='PASS'){
    try{c.contract=normalizedContract(v.proposal);c.obligations=Object.fromEntries(c.contract.mandatory.map(r=>[r.id,{state:'OPEN',attempts:0,last_evidence:null}]));c.phase='BUILD';}
    catch(e){c.last_result.contract_rejected=String(e.message);}
   }
  }else{
   const o=c.obligations[p.task_id];o.attempts++;o.state=v.result==='PASS'?'PASS':'OPEN';o.last_evidence={digest:v.evidence_digest,candidate_digest:v.candidate_digest,result:v.result};
   if(Object.values(c.obligations).every(z=>z.state==='PASS'))c.phase='QUALIFY';
  }
 }
 c.status=x.blocked||!independent?'WAITING_RECONCILIATION':'READY';
 if(c.status==='READY'&&next(c).reason==='FAILED_BOUNDED')c.status='FAILED_BOUNDED';
 x.controller=c;return x;
}
// No adapter can be chosen by a worker. All adapters here are trusted controller code.
// The real deployment must enforce filesystem/network separation outside this library.
export async function runOne({dir,lane,identity,adapter,forecast,caps,clock=Date.now}){
 let snapshot;
 try{snapshot=await adapter.observe();}catch(e){return {status:'WAITING_AUTHORIZATION',reason:'QUOTA_OBSERVATION_FAILED'};}
 checkedLane(lane,snapshot.provider);
 let result;
 // Read/create/roll over under same account journal lock. A no-op is not published.
 // Each transition gets a fresh revision; append an inspection event only when necessary.
 try{
  result=transact(dir,prior=>{
   if(!prior){const l=openDay(snapshot,clock());const x=createRun(l,{lane,...identity});x.revision=0;return x;}
   if(prior.controller?.run_id!==identity.run_id||prior.controller?.lane!==lane)fail('RUN_IDENTITY_MISMATCH');
   if(prior.controller.goal_charter_digest!==hash(charter))fail('GOAL_CHARTER_CHANGED_REVIEW_REQUIRED');
   if(prior.pending||prior.controller.pending||prior.blocked)fail('WAITING_RECONCILIATION');
   const day=new Intl.DateTimeFormat('en-CA',{timeZone:campaign.budget.local_day,year:'numeric',month:'2-digit',day:'2-digit'}).format(new Date(clock()));
   if(day!==prior.day){const x=rollDay(prior,snapshot,clock());x.controller=prior.controller;return x;}
   // The inspection transition is a durable observation, not another inference.
   prior.revision++;return prior;
  });
 }catch(e){return {status:String(e.message).includes('RECONCILIATION')?'WAITING_RECONCILIATION':'WAITING_AUTHORIZATION',reason:e.message};}
 const c=result.controller,n=next(c);
 if(n.kind==='STOP')return {status:n.reason,controller:c};
 let boundary;
 try{boundary=await adapter.preflight({lane,run_id:c.run_id,workspace_digest:c.candidate_digest});checkBoundary(boundary,c,snapshot);}
 catch(e){return {status:'WAITING_AUTHORIZATION',reason:e.message};}
 if(n.kind==='QUALIFY'){
  // Independent native tests only. No model turn and no quota reservation.
  const receipt=await adapter.qualify({contract:c.contract,candidate_digest:c.candidate_digest,run_id:c.run_id});
  const verdict=receipt?.candidate_digest===c.candidate_digest?judge({contract:c.contract,receipt,status:'CANDIDATE_COMPLETE',context:receipt.context,observed_at:new Date(clock()).toISOString()}):{state:'UNVERIFIED',reason:'CANDIDATE_DRIFT'};
  const x=transact(dir,prior=>{
   if(prior.revision!==result.revision||prior.pending)fail('CONCURRENT_STATE_CHANGE');
   const z=structuredClone(prior.controller);z.last_result=verdict;
   z.status=verdict.state==='FUNCTIONALLY_SUFFICIENT'?'COMPLETE_VERIFIED':'WAITING_RECONCILIATION';
   if(verdict.state==='FUNCTIONALLY_SUFFICIENT')z.sufficiency=verdict;
   return advance(prior,z);
  });
  return {status:x.controller.status,verdict};
 }
 let decision,turn;
 try{
  const reserved=transact(dir,prior=>{
   if(prior.revision!==result.revision||prior.pending)fail('CONCURRENT_STATE_CHANGE');
   const a=prior.allocations[lane],z=structuredClone(prior.controller);
   const local={account_key:snapshot.account_key,day:prior.day,inflight:0,daily_turn_limit:caps.turn_ceiling,daily_token_limit:caps.token_ceiling,daily_turns:a.turns,daily_tokens:a.token_spent,predicted_tokens:caps.predicted_tokens,predicted_quota_pp:forecast,verified_model_stats:z.stats,adaptive_decisions:z.turns};
   decision=route({lane,task:n.task,snapshot,local,ledger:prior},clock());
   if(decision.decision!=='ADMIT_CANDIDATE')return hold(prior,'WAITING_BUDGET');
   const prompt=promptFor(z,n);
   const resume=z.session&&z.session.model===decision.model&&z.session.reasoning_effort===decision.reasoning_effort?z.session.id:null;
   const request_id=crypto.randomUUID();
   turn={request_id,task_id:n.task_id,task:n.task,model:decision.model,reasoning_effort:decision.reasoning_effort,prompt_digest:hash(prompt),resume_session:resume,incarnation:resume?z.incarnation:z.incarnation+1,checkpoint_digest:z.last_checkpoint,boundary_digest:boundary.evidence_digest};
   const x=reserve(prior,{lane,request_id,task_id:n.task_id,model:turn.model,reasoning_effort:turn.reasoning_effort,forecast:decision.estimated_quota_pp,local_caps:{...caps,daily_tokens:a.token_spent}},snapshot,clock());
   z.pending=turn;z.status='RUNNING';x.controller=z;return x;
  });
  if(!turn)return {status:reserved.controller.status,decision};
 }catch(e){return {status:'WAITING_BUDGET',reason:e.message};}
 let transport=null,verification=null,after=null;
 try{transport=await adapter.execute({...turn,prompt:promptFor(c,n),run_id:c.run_id,lane});if(checkReceipt(transport,turn))verification=await adapter.verify({turn,transport,contract:c.contract,obligation:n.obligation});}catch{}
 try{after=await adapter.observe();}catch{}
 // If process dies before this commit, durable pending intent prevents replay.
 const settled=transact(dir,prior=>finish(prior,transport,verification,after,clock()));
 return {status:settled.controller.status,controller:settled.controller,decision};
}

// A controller-reviewed change creates a new evaluation episode, preserving the old release.
// Purpose/contract changes demand a newly reviewed contract; same-contract invalidations
// reopen impacted obligations. Final qualification always rechecks every criterion.
export function reopenRun(dir,change){
 return transact(dir,prior=>{
  if(prior.pending||prior.blocked||prior.controller?.pending)fail('RECONCILIATION_REQUIRED');
  const c=structuredClone(prior.controller);
  if(c.status!=='COMPLETE_VERIFIED'||!c.sufficiency)fail('NO_VERIFIED_RELEASE');
  const outcome=reopen({judgment:c.sufficiency,contract:c.contract,change});
  if(outcome.state==='FUNCTIONALLY_SUFFICIENT'){
   c.last_result={kind:'OPTIONAL_ONLY',reason:change.reason};return advance(prior,c);
  }
  c.historical_sufficiency.push(c.sufficiency);c.sufficiency=null;
  c.last_result=outcome;c.status='READY';c.session=null;
  if(['PURPOSE','ENVELOPE','ASSUMPTIONS'].includes(change.kind)){
   c.prior_contracts=[...(c.prior_contracts??[]),c.contract];c.contract=null;c.obligations={};c.discovery_attempts=0;c.phase='DISCOVER_COMPLETENESS';
  }else{
   if(change.kind==='CANDIDATE'){
    if(!hex(change.candidate_digest))fail('NEW_CANDIDATE_UNVERIFIED');
    c.candidate_digest=change.candidate_digest;
   }
   for(const id of outcome.affected)c.obligations[id]={state:'OPEN',attempts:0,last_evidence:null};
   c.phase='BUILD';
  }
  return advance(prior,c);
 });
}
