//! Prospective, matched execution comparisons for promoted trace reuse.
//! No prices, savings estimates, hidden planning costs, or causal claims.
use super::{model::*,routing::Status};
use crate::{digest,Result};
use serde::{Deserialize,Serialize};
use serde_json::{json,Value};
use std::collections::{BTreeMap,BTreeSet};

fn pin<T:Serialize>(v:&T)->Result<Digest>{Ok(digest(&serde_json::to_value(v).map_err(|e|e.to_string())?))}
#[derive(Clone,Copy,Debug,Serialize,Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Metric{NativeWorkerMs,RecordedProviderRequests}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pair{
    pub baseline:Id,pub reuse:Id,
    /// Baseline tasks in the corresponding promoted pattern step order.
    pub baseline_tasks:Vec<Id>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plan{
    pub id:Id,pub candidate:Id,pub pairs:Vec<Pair>,
    pub metric:Metric,pub min_pairs:usize,pub min_reduction_bps:u16,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct FrozenPair{
    pub pair:Pair,pub reuse_tasks:Vec<Id>,pub workload_digest:Digest,
    pub baseline_contract:Digest,pub reuse_contract:Digest,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Frozen{
    pub plan:Plan,pub pairs:Vec<FrozenPair>,pub registered_sequence:u64,
}
#[derive(Clone,Debug,Serialize,Deserialize,Default)]
pub struct Registry{pub plans:BTreeMap<Id,Frozen>,pub reports:BTreeMap<Id,Value>}
impl Registry{pub fn is_empty(&self)->bool{self.plans.is_empty()&&self.reports.is_empty()}}

#[derive(Clone,Debug,Serialize)]
struct Costs{
    native_attempts:usize,native_measured:usize,native_worker_ms:Option<u64>,
    recorded_provider_requests:Option<u64>,recorded_input_tokens:Option<u64>,
    recorded_output_tokens:Option<u64>,recorded_reasoning_tokens:Option<u64>,
    receipt_ids:Vec<Id>,uncovered_reservations:Vec<Id>,uncovered_attempts:Vec<Id>,
    active_reservations:Vec<Id>,
}
fn plus(x:&mut Option<u64>,v:u64){*x=x.and_then(|n|n.checked_add(v));}
impl World{
    fn comparison_contract(&self,g:&str,keys:&[Id],fresh:bool)->Result<(Digest,Digest)>{
        let goal=self.goals.get(g).ok_or("comparison goal missing")?;
        ensure(goal.sealed&&goal.parent.is_none()&&!self.goals.values().any(|x|x.parent.as_deref()==Some(g)),"comparison requires sealed independent leaf goals")?;
        ensure(!keys.is_empty()&&keys.len()<=32,"invalid comparison step count")?;
        let indices:BTreeMap<_,_>=keys.iter().enumerate().map(|(i,k)|(k.clone(),i)).collect();
        let actual:BTreeSet<_>=self.tasks.values().filter(|t|t.spec.goal==g).map(|t|t.spec.id.clone()).collect();
        ensure(indices.len()==keys.len()&&actual==indices.keys().cloned().collect(),"comparison task set mismatch")?;
        if fresh{
            ensure(goal.phase==GoalPhase::Open&&goal.claims==0&&!self.router.reservations.values().any(|r|r.goal==g),"comparison must precede execution and reservation")?;
        }
        let mut normalized=vec![];let mut exact=vec![];
        for (i,k) in keys.iter().enumerate(){
            let t=&self.tasks[k];let s=&t.spec;
            ensure(!self.effects.values().any(|e|e.task==*k),"effectful comparison not supported")?;
            if fresh{ensure(t.phase==TaskPhase::Pending&&t.generation==0&&t.attempts==0,"task already attempted")?;}
            let context=s.context.as_ref().map(|k|self.contexts.get(k).ok_or("comparison context missing")).transpose()?;
            let mut ports=BTreeMap::new();
            for (name,p) in &s.inputs{
                let source=match &p.binding{
                    Binding::Predecessor{task}=>{
                        let n=*indices.get(task).ok_or("comparison predecessor missing")?;
                        ensure(n<i&&s.dependencies.contains(task),"invalid comparison predecessor order")?;
                        json!({"step":n})
                    }
                    Binding::Literal{value}=>json!({"value_digest":digest(value)}),
                    Binding::Context{key}=>{
                        let c=context.ok_or("explicit context required for comparison")?;
                        let v=if let Some(v)=c.entries.get(key){v}else{self.artifact_value(c.artifacts.get(key).ok_or("comparison context key missing")?)?};
                        ensure(p.shape.accepts(v),"comparison context shape mismatch")?;
                        json!({"value_digest":digest(v)})
                    }
                    Binding::Knowledge{version}=>json!({"value_digest":digest(self.knowledge_value(version,s.family.as_deref())?)}),
                    Binding::SelectKnowledge{..}=>return Err("dynamic knowledge selection must be pinned before comparison".into()),
                };
                ports.insert(name.clone(),json!({"shape":p.shape,"source":source}));
            }
            let dependencies=s.dependencies.iter().map(|d|{
                let n=*indices.get(d).ok_or("comparison dependency missing")?;
                ensure(n<i,"comparison requires topological task order")?;Ok(n)
            }).collect::<Result<BTreeSet<_>>>()?;
            normalized.push(json!({"dependencies":dependencies,"ports":ports,"output":s.output,"oracle":s.oracle,"family":s.family,
                "max_attempts":s.max_attempts,"priority":s.priority,"uncertainty":s.uncertainty,"information_gain":s.information_gain,"expected_cost":s.expected_cost}));
            exact.push(json!({"spec":s,"version":t.version,"context_digest":context.map(pin).transpose()?}));
        }
        let workload=pin(&json!({"claim_budget":goal.claim_budget,"predicate":goal.predicate,"steps":normalized}))?;
        let contract=pin(&json!({"goal":g,"version":goal.version,"objective":goal.objective,"claim_budget":goal.claim_budget,"predicate":goal.predicate,"tasks":exact}))?;
        Ok((workload,contract))
    }
    pub fn register_reuse_comparison(&mut self,p:Plan)->Result<Value>{
        id(&p.id)?;
        ensure(!self.reuse_economics.plans.contains_key(&p.id)&&self.reuse_economics.plans.len()<256
            &&(2..=32).contains(&p.pairs.len())&&(2..=p.pairs.len()).contains(&p.min_pairs)&&p.min_reduction_bps<=10_000,"invalid comparison plan")?;
        ensure(self.assimilation.promotions.get(&p.candidate).is_some_and(|x|x.enabled),"comparison candidate not promoted")?;
        let mut used:BTreeSet<Id>=self.reuse_economics.plans.values().flat_map(|x|x.pairs.iter().flat_map(|x|[x.pair.baseline.clone(),x.pair.reuse.clone()])).collect();
        let mut pairs=vec![];
        for pair in &p.pairs{
            id(&pair.baseline)?;id(&pair.reuse)?;
            ensure(used.insert(pair.baseline.clone())&&used.insert(pair.reuse.clone()),"comparison goal reused")?;
            ensure(!self.assimilation.runs.contains_key(&pair.baseline),"baseline must not be instantiated by trace reuse")?;
            let r=self.assimilation.runs.get(&pair.reuse).ok_or("reuse run missing")?;
            ensure(r.candidate==p.candidate&&r.tasks.len()==pair.baseline_tasks.len(),"reuse candidate or step count mismatch")?;
            let (workload,baseline_contract)=self.comparison_contract(&pair.baseline,&pair.baseline_tasks,true)?;
            let (other,reuse_contract)=self.comparison_contract(&pair.reuse,&r.tasks,true)?;
            ensure(workload==other,"baseline and reuse workload contracts differ")?;
            pairs.push(FrozenPair{pair:pair.clone(),reuse_tasks:r.tasks.clone(),workload_digest:workload,baseline_contract,reuse_contract});
        }
        let frozen=Frozen{plan:p.clone(),pairs,registered_sequence:self.sequence.checked_add(1).ok_or("sequence overflow")?};
        let v=serde_json::to_value(&frozen).map_err(|e|e.to_string())?;bounded(&v)?;
        self.reuse_economics.plans.insert(p.id,frozen);Ok(v)
    }
    fn comparison_costs(&self,g:&str,keys:&[Id])->Costs{
        let scope:BTreeSet<_>=keys.iter().cloned().collect();
        let mut c=Costs{native_attempts:0,native_measured:0,native_worker_ms:Some(0),recorded_provider_requests:Some(0),
            recorded_input_tokens:Some(0),recorded_output_tokens:Some(0),recorded_reasoning_tokens:Some(0),
            receipt_ids:vec![],uncovered_reservations:vec![],uncovered_attempts:vec![],active_reservations:vec![]};
        for (k,a) in &self.attempt_history{
            if !scope.contains(&a.task){continue;}
            let executor=self.agents.get(&a.agent).and_then(|agent|agent.incarnations.iter().find(|i|i.number==a.incarnation)).map(|i|i.executor.as_str());
            match executor{
                Some("native-pure-process")=>{
                    c.native_attempts+=1;
                    let measured=self.run_costs.get(k).filter(|r|r.task==a.task&&r.generation==a.generation&&a.measured_ms==Some(r.elapsed_ms)&&a.ended_ms.is_some());
                    if let Some(r)=measured{c.native_measured+=1;plus(&mut c.native_worker_ms,r.elapsed_ms);}
                    else{c.native_worker_ms=None;c.uncovered_attempts.push(k.clone());}
                }
                Some(x) if x.starts_with("routed:")=>{
                    if !self.router.reservations.values().any(|r|r.task==a.task&&r.task_generation==Some(a.generation)&&r.agent.as_ref()==Some(&a.agent)&&r.incarnation==Some(a.incarnation)){
                        c.uncovered_attempts.push(k.clone());
                    }
                }
                _=>c.uncovered_attempts.push(k.clone()),
            }
            if a.ended_ms.is_none()&&!c.uncovered_attempts.contains(k){c.uncovered_attempts.push(k.clone());}
        }
        for k in keys{
            let count=self.attempt_history.values().filter(|a|a.task==*k).count();
            if self.tasks.get(k).is_none_or(|t|count!=t.attempts as usize){c.uncovered_attempts.push(format!("{k}:attempt_count"));}
        }
        for r in self.router.reservations.values().filter(|r|r.goal==g||scope.contains(&r.task)){
            if matches!(r.status,Status::Reserved|Status::Dispatched|Status::Unknown){c.active_reservations.push(r.id.clone());}
            if r.started_ms.is_none(){continue;}
            let receipt=self.router.adapters.receipts.get(&r.id).filter(|x|
                r.status==Status::Settled&&x.handoff==r.id&&x.provider==r.profile.provider&&x.account==r.profile.account
                &&x.model==r.profile.model&&x.thinking==r.profile.thinking&&r.receipt.as_deref()==Some(x.provider_receipt.as_str()));
            if let Some(x)=receipt{
                c.receipt_ids.push(r.id.clone());
                plus(&mut c.recorded_provider_requests,x.usage.requests);plus(&mut c.recorded_input_tokens,x.usage.input);
                plus(&mut c.recorded_output_tokens,x.usage.output);plus(&mut c.recorded_reasoning_tokens,x.usage.reasoning);
            }else{c.uncovered_reservations.push(r.id.clone());}
        }
        if !c.uncovered_attempts.is_empty(){c.native_worker_ms=None;}
        if !c.uncovered_attempts.is_empty()||!c.uncovered_reservations.is_empty()||!c.active_reservations.is_empty(){
            c.recorded_provider_requests=None;c.recorded_input_tokens=None;c.recorded_output_tokens=None;c.recorded_reasoning_tokens=None;
        }
        c
    }
    fn comparison_verified(&self,keys:&[Id])->Result<Vec<(Digest,Digest)>>{
        keys.iter().map(|k|{
            let t=self.tasks.get(k).ok_or("comparison task missing")?;
            ensure(t.phase==TaskPhase::Verified,"comparison task not verified")?;
            let a=self.artifacts.get(t.artifact.as_deref().ok_or("comparison artifact missing")?).ok_or("comparison artifact missing")?;
            let e=self.evidence.get(&a.evidence).ok_or("comparison evidence missing")?;
            let input=t.input.as_ref().ok_or("comparison input missing")?;
            let output=self.payload(&a.payload)?;
            ensure(e.accepted&&e.task==*k&&a.task==*k&&e.generation==t.generation&&a.generation==t.generation
                &&e.input==digest(input)&&e.candidate==a.payload&&e.oracle.evaluate(input,output)?&&t.spec.oracle.evaluate(input,output)?,"comparison evidence mismatch")?;
            Ok((e.input.clone(),a.payload.clone()))
        }).collect()
    }
    pub fn reuse_comparison_report(&self,key:&str)->Result<Value>{
        if let Some(v)=self.reuse_economics.reports.get(key){return Ok(v.clone());}
        let f=self.reuse_economics.plans.get(key).ok_or("comparison missing")?;
        let mut pairs=vec![];let mut eligible=0usize;let mut baseline_total=Some(0u64);let mut reuse_total=Some(0u64);
        let mut all_terminal=true;
        for p in &f.pairs{
            let a=&p.pair.baseline;let b=&p.pair.reuse;
            let ga=self.goals.get(a).ok_or("comparison baseline missing")?;let gb=self.goals.get(b).ok_or("comparison reuse goal missing")?;
            let terminal=ga.phase!=GoalPhase::Open&&gb.phase!=GoalPhase::Open;all_terminal &=terminal;
            let contracts=self.comparison_contract(a,&p.pair.baseline_tasks,false).is_ok_and(|(w,c)|w==p.workload_digest&&c==p.baseline_contract)
                &&self.comparison_contract(b,&p.reuse_tasks,false).is_ok_and(|(w,c)|w==p.workload_digest&&c==p.reuse_contract);
            let equivalent=ga.phase==GoalPhase::Complete&&gb.phase==GoalPhase::Complete&&contracts
                &&match (self.comparison_verified(&p.pair.baseline_tasks),self.comparison_verified(&p.reuse_tasks)){(Ok(x),Ok(y))=>x==y,_=>false};
            let ca=self.comparison_costs(a,&p.pair.baseline_tasks);let cb=self.comparison_costs(b,&p.reuse_tasks);
            let no_unknown=ca.uncovered_attempts.is_empty()&&cb.uncovered_attempts.is_empty()&&ca.uncovered_reservations.is_empty()
                &&cb.uncovered_reservations.is_empty()&&ca.active_reservations.is_empty()&&cb.active_reservations.is_empty();
            let (x,y)=match f.plan.metric{Metric::NativeWorkerMs=>(ca.native_worker_ms,cb.native_worker_ms),Metric::RecordedProviderRequests=>(ca.recorded_provider_requests,cb.recorded_provider_requests)};
            let comparable=terminal&&equivalent&&no_unknown&&x.is_some()&&y.is_some();
            if comparable{eligible+=1;plus(&mut baseline_total,x.unwrap());plus(&mut reuse_total,y.unwrap());}
            pairs.push(json!({"baseline":a,"reuse":b,"baseline_phase":ga.phase,"reuse_phase":gb.phase,"terminal":terminal,
                "contracts_unchanged":contracts,"verified_io_equivalent":equivalent,"coverage_complete_for_recorded_scope":no_unknown,
                "baseline_costs":ca,"reuse_costs":cb,"comparable":comparable,
                "observed_difference":if comparable{i64::try_from(i128::from(x.unwrap())-i128::from(y.unwrap())).ok()}else{None}}));
        }
        // Never select only successful pairs when recommending a policy.
        // A nonzero baseline is necessary for a relative reduction.
        let reduction=if all_terminal&&eligible==f.pairs.len()&&eligible>=f.plan.min_pairs{
            match (baseline_total,reuse_total){(Some(a),Some(b)) if a>0=>i64::try_from((i128::from(a)-i128::from(b))*10_000/i128::from(a)).ok(),_=>None}
        }else{None};
        let feedback=match reduction{Some(n) if n>=i64::from(f.plan.min_reduction_bps)=>"observed_threshold_met_review_required",Some(_)=>"observed_threshold_not_met",None=>"insufficient_comparable_evidence"};
        let report=json!({"comparison":key,"plan_digest":pin(f)?,"candidate":f.plan.candidate,"metric":f.plan.metric,
            "registered_sequence":f.registered_sequence,"observed_sequence":self.sequence,"closed":false,"all_terminal":all_terminal,"pairs":pairs,
            "eligible_pairs":eligible,"total_pairs":f.pairs.len(),"baseline_total":if eligible==f.pairs.len(){baseline_total}else{None},
            "reuse_total":if eligible==f.pairs.len(){reuse_total}else{None},"observed_reduction_bps":reduction,"feedback":feedback,
            "scope":"same_store_matched_execution_observation_not_causal_or_end_to_end_savings",
            "exclusions":["preparation","training_and_review","host_and_storage_overhead","unobserved_external_usage","cross_arm_isolation"],
            "policy_mutated":false,"acceptance_credit":false});
        bounded(&report)?;Ok(report)
    }
    pub fn close_reuse_comparison(&mut self,key:&str)->Result<Value>{
        ensure(!self.reuse_economics.reports.contains_key(key),"comparison already closed")?;
        let mut report=self.reuse_comparison_report(key)?;
        ensure(report["all_terminal"]==true,"comparison still running")?;
        report["closed"]=json!(true);report["closed_sequence"]=json!(self.sequence.checked_add(1).ok_or("sequence overflow")?);
        self.reuse_economics.reports.insert(key.into(),report.clone());Ok(report)
    }
}
