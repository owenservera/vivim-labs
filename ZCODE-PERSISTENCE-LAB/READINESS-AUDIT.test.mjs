// R014 readiness probe: synthetic adapter only; never invokes a provider.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {runOne,next,reopenRun} from './CAMPAIGN-CONTROLLER.mjs';
import {contextDigest} from './SUFFICIENCY.mjs';
const now=Date.parse('2026-11-28T16:00:00Z'),clock=()=>now;
const d='a'.repeat(64),e='b'.repeat(64);
const charter=JSON.parse(fs.readFileSync(new URL('./SYSTEM-GOALS-R013.json',import.meta.url),'utf8'));
const contract={id:'readiness-probe',purpose:'synthetic validation only',operating_envelope:'nine synthetic obligations',evidence_standard:'mock receipts',reopening_triggers:['evidence changed'],optional_backlog:[],release_boundary:'bounded fixture',out_of_scope:'real inference',mandatory:charter.goals.map((g,i)=>({id:'O'+i,behavior:g.name,oracle:'fixture '+i,falsifier:'failure '+i,task:'implementation',priority:50,depends_on:i?['O'+(i-1)]:[],goal_ids:[g.id]}))};
function setup(provider,lane){
 const model=provider==='codex'?'gpt-6.2-sol':'gemini-4-neon-high',group=provider==='codex'?'codex':'Gemini Models';
 const models=provider==='codex'?[model]:[model,'gemini-4-carbon-medium','gemini-4-carbon-high'];
 const snapshot={provider,account_key:provider+'-fixture',observed_at:new Date(now).toISOString(),subscription_only:true,paid_overflow_disabled:true,ordinary_usage_allowed:true,models,model_efforts:{[model]:['high']},model_groups:Object.fromEntries(models.map(m=>[m,group])),groups:{[group]:{windows:[{duration_minutes:300,remaining_percent:99,resets_at:new Date(now+4*3600000).toISOString()},{duration_minutes:10080,remaining_percent:99,resets_at:new Date(now+4*86400000).toISOString()}]}}};
 const state={firstModel:null,executions:0,qualifications:0,failBoundary:false,badTransport:false};
 const identity={run_id:'fixture-'+lane,seed_digest:d,workspace_digest:d};
 const adapter={
  observe:async()=>structuredClone(snapshot),
  preflight:async({workspace_digest})=>({authority:'controller-owned-isolation-verifier',lane,run_id:identity.run_id,account_key:snapshot.account_key,provider,isolation_verified:!state.failBoundary,scoped_write_verified:true,billing_verified:true,evidence_digest:d,workspace_digest}),
  execute:async(t)=>{state.executions++;state.firstModel??=t.model;return {request_id:t.request_id,model:state.badTransport?'wrong-model':t.model,reasoning_effort:t.reasoning_effort,session_id:t.resume_session||'session-'+t.incarnation,exit_code:0,terminal:true,process_fenced:true,raw_digest:d,actual_tokens:100};},
  verify:async({turn})=>({authority:'controller-independent-evaluator',request_id:turn.request_id,candidate_digest:d,evidence_digest:d,checkpoint_digest:d,result:'PASS',scope_review:'PASS',proposal:contract,quota_attribution_confirmed:true}),
  qualify:async({contract,candidate_digest})=>{state.qualifications++;const context={purpose_digest:contextDigest(contract.purpose),operating_envelope_digest:contextDigest(contract.operating_envelope),evidence_standard_digest:contextDigest(contract.evidence_standard),assumptions_digest:e};return {authority:'controller-independent-evaluator',contract_digest:contract.digest,candidate_digest,scope_review:'PASS',regressions:'PASS',failure_cases:'PASS',unresolved_mandatory:0,unknown_effects:0,context,context_digest:contextDigest(context),criteria:contract.mandatory.map(c=>({id:c.id,result:'PASS',candidate_digest,evidence_digest:d,reproduction_command:'fixture '+c.id}))};}
 };
 const forecast=Object.fromEntries((provider==='codex'?[model+'@high']:models).map(m=>[m,{'300':0.1,'10080':0.1}]));
 const dir=fs.mkdtempSync(path.join(os.tmpdir(),'vivim-readiness-'));
 const args={dir,lane,identity,adapter,forecast,caps:{turn_ceiling:20,token_ceiling:100000,predicted_tokens:100},clock};
 return {args,state,dir,snapshot};
}
for(const [provider,lane] of [['codex','01'],['agy','04']]){
 const x=setup(provider,lane);
 try{
  x.state.failBoundary=true;
  assert.equal((await runOne(x.args)).status,'WAITING_AUTHORIZATION');assert.equal(x.state.executions,0);
  x.state.failBoundary=false;
  assert.equal((await runOne({...x.args,forecast:{}})).status,'WAITING_BUDGET');assert.equal(x.state.executions,0);
  let r=await runOne(x.args);assert.equal(r.status,'READY');assert.equal(r.controller.phase,'BUILD');if(provider==='agy')assert.equal(x.state.firstModel,'gemini-4-carbon-high');
  for(let i=0;i<9;i++){r=await runOne(x.args);assert.equal(r.status,'READY');}
  assert.equal(r.controller.phase,'QUALIFY');assert.equal(x.state.executions,10);
  r=await runOne(x.args);assert.equal(r.status,'COMPLETE_VERIFIED');assert.equal(x.state.qualifications,1);assert.equal(x.state.executions,10);
  r=await runOne(x.args);assert.equal(r.status,'COMPLETE_VERIFIED');assert.equal(x.state.executions,10);
  const reopened=reopenRun(x.dir,{kind:'EVIDENCE',reason:'fixture invalidation',evidence_digest:d,impact_verified:true,affected:['O7']});
  assert.equal(reopened.controller.historical_sufficiency.length,1);assert.equal(reopened.controller.obligations.O7.state,'OPEN');assert.equal(reopened.controller.obligations.O8.state,'OPEN');assert.equal(reopened.controller.obligations.O6.state,'PASS');
  console.log('R014 '+provider+' fixture: authorization and budget refusal, nine obligations, qualification, no extra inference, targeted reopening PASS');
 }finally{fs.rmSync(x.dir,{recursive:true,force:true});}
 const y=setup(provider,lane);
 try{y.state.badTransport=true;const r=await runOne(y.args);assert.equal(r.status,'WAITING_RECONCILIATION');assert.equal(y.state.executions,1);const r2=await runOne(y.args);assert.equal(r2.status,'WAITING_RECONCILIATION');assert.equal(y.state.executions,1);console.log('R014 '+provider+' fixture: wrong-model receipt fenced, replay refused PASS');}finally{fs.rmSync(y.dir,{recursive:true,force:true});}
}
console.log('R014 synthetic integrated controller probes passed; no real provider calls or qualification evidence.');
