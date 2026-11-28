//! Evidence-driven learning plans, knowledge registry, and scheduling feedback.
use super::model::*;
use crate::{digest,Result};
use serde_json::{Value,json};
use std::collections::{BTreeMap,BTreeSet};

impl World {
    pub fn configure_learning(&mut self,plan:LearningPlan)->Result<Id>{
        let key=version_key(&plan.id,plan.version)?;
        id(&plan.family)?;version_key(&plan.capability,plan.capability_version)?;
        ensure(!self.learning_plans.contains_key(&key),"learning plan version exists")?;
        ensure((2..=128).contains(&plan.training.len())&&(2..=128).contains(&plan.holdouts.len())&&(1..=100_000).contains(&plan.search_limit),"invalid learning plan limits")?;
        let mut seen=BTreeSet::new();
        for sample in plan.training.iter().chain(plan.holdouts.iter()){
            ensure(seen.insert(sample.clone()),"duplicate sample task")?;
            let t=self.tasks.get(sample).ok_or("sample task missing")?;
            ensure(t.spec.family.as_ref()==Some(&plan.family)&&plan.input.admits(&Self::input_shape(&t.spec))&&plan.output.admits(&t.spec.output),"sample contract mismatch")?;
        }
        self.learning_plans.insert(key.clone(),plan);Ok(key)
    }
    pub fn ready_learning(&self)->Vec<Id>{
        self.learning_plans.iter().filter(|(k,p)|!self.learning_attempts.contains_key(*k)&&p.training.iter().chain(p.holdouts.iter()).all(|t|self.tasks.get(t).is_some_and(|x|x.phase==TaskPhase::Verified))).map(|(k,_)|k.clone()).collect()
    }
    pub fn run_learning(&mut self,key:&str)->Result<Value>{
        ensure(!self.learning_attempts.contains_key(key),"learning plan already attempted")?;
        let p=self.learning_plans.get(key).ok_or("learning plan missing")?.clone();
        ensure(self.ready_learning().iter().any(|k|k==key),"learning examples not ready")?;
        let mut fingerprints=Vec::new();
        for sample in &p.holdouts{
            let task=self.tasks.get(sample).ok_or("sample missing")?;
            fingerprints.push(digest(task.input.as_ref().ok_or("sample input missing")?));
        }
        let previous:BTreeSet<_>=self.learning_attempts.values().flat_map(|a|a.holdout_inputs.iter().cloned()).collect();
        if fingerprints.iter().any(|d|previous.contains(d)){
            let attempt=LearningAttempt{plan:key.into(),accepted:false,capability:None,reason:Some("holdout already consumed".into()),evaluated:None,holdout_inputs:vec![],clock_ms:self.clock_ms};
            self.learning_attempts.insert(key.into(),attempt.clone());return Ok(json!(attempt));
        }
        // A rejected cohort cannot be repeatedly tuned against the same holdout.
        let mut attempt=LearningAttempt{plan:key.into(),accepted:false,capability:None,reason:None,evaluated:None,holdout_inputs:fingerprints,clock_ms:self.clock_ms};
        let (discovery,cost)=self.discover_measured(&p.family,p.training.clone(),p.holdouts.clone(),p.input,p.output,p.search_limit);
        attempt.evaluated=Some(cost.candidates_evaluated);
        self.metrics.candidate_evaluations=self.metrics.candidate_evaluations.saturating_add(cost.candidates_evaluated);
        match discovery{
            Ok(d)=>{
                match self.assimilate(&p.capability,p.capability_version,d){
                    Ok(cap)=>{attempt.accepted=true;attempt.capability=Some(cap);}
                    Err(reason)=>attempt.reason=Some(reason),
                }
            }
            Err(reason)=>attempt.reason=Some(reason),
        }
        self.learning_costs.insert(key.into(),LearningCost{plan:key.into(),evaluated:cost.candidates_evaluated,training_evaluations:cost.training_evaluations,holdout_evaluations:cost.holdout_evaluations,accepted:attempt.accepted});
        self.learning_attempts.insert(key.into(),attempt.clone());
        Ok(json!(attempt))
    }
    pub fn publish_knowledge(&mut self,k:Knowledge)->Result<Id>{
        let key=version_key(&k.id,k.version)?;
        ensure(k.enabled&&!self.knowledge.contains_key(&key)&&!k.artifacts.is_empty()&&!k.evidence.is_empty(),"invalid knowledge record")?;
        let evidence:BTreeSet<_>=k.evidence.iter().cloned().collect();
        for e in &k.evidence{
            ensure(self.evidence.get(e).is_some_and(|x|x.accepted),"knowledge evidence not accepted")?;
        }
        for artifact in &k.artifacts{
            let a=self.artifacts.get(artifact).ok_or("knowledge artifact missing")?;
            ensure(evidence.contains(&a.evidence),"artifact evidence not included")?;
            self.payload(&a.payload)?;
        }
        if let Some(family)=&k.family{
            id(family)?;
            ensure(k.artifacts.iter().all(|a|self.tasks.get(&self.artifacts[a].task).is_some_and(|t|t.spec.family.as_ref()==Some(family))),"knowledge family mismatch")?;
        }
        if let Some(class)=&k.class{
            ensure(self.classes.contains_key(class)&&self.capabilities.values().any(|c|c.enabled&&&c.class==class),"executable knowledge class not admitted")?;
        }
        if let Some(parent)=&k.supersedes{
            let prior=self.knowledge.get(parent).ok_or("knowledge ancestor missing")?;
            ensure(prior.id==k.id&&prior.version<k.version&&prior.kind==k.kind,"invalid knowledge lineage")?;
        }
        self.knowledge.insert(key.clone(),k);Ok(key)
    }
    pub fn knowledge_value(&self,key:&str,family:Option<&str>)->Result<&Value>{
        let k=self.knowledge.get(key).ok_or("knowledge version missing")?;
        ensure(k.enabled&&(k.family.is_none()||k.family.as_deref()==family),"knowledge scope mismatch")?;
        ensure(k.artifacts.len()==1,"binding requires a single artifact")?;
        self.artifact_value(&k.artifacts[0])
    }
    pub fn record_cost(&mut self,task:&str,generation:u64,elapsed_ms:u64)->Result<()>{
        let t=self.tasks.get(task).ok_or("task missing")?;
        ensure(generation>0&&elapsed_ms<=3_600_000,"invalid cost receipt")?;
        let key=format!("{task}-{generation}");
        let attempt=self.attempt_history.get(&key).ok_or("attempt receipt missing")?;
        let a=self.agents.get(&attempt.agent).ok_or("agent missing")?;
        ensure(a.incarnations.iter().any(|i|i.number==attempt.incarnation&&i.executor=="native-pure-process"),"cost receipt requires native executor")?;
        let class=Some(attempt.class.clone());
        if let Some(old)=self.run_costs.get(&key){return ensure(old.elapsed_ms==elapsed_ms,"cost receipt conflict");}
        self.run_costs.insert(key.clone(),RunCost{task:task.into(),generation,elapsed_ms,family:t.spec.family.clone(),class});
        self.attempt_history.get_mut(&key).unwrap().measured_ms=Some(elapsed_ms);
        self.metrics.worker_elapsed_ms=self.metrics.worker_elapsed_ms.saturating_add(elapsed_ms);Ok(())
    }
    pub fn schedule_scores(&self)->BTreeMap<Id,i128>{
        let derived=self.derived_snapshot();
        let live=|t:&Task|matches!(t.phase,TaskPhase::Pending|TaskPhase::Running|TaskPhase::Submitted|TaskPhase::Reconciling);
        let costs=&derived.schedule_costs;let path=&derived.critical_paths;let family_outcomes=&derived.outcomes;
        let p=&self.policy;
        self.tasks.values().filter(|t|live(t)).map(|t|{
            let (attempts,successes)=t.spec.family.as_ref().and_then(|f|family_outcomes.get(f)).copied().unwrap_or((0,0));
            // Laplace-smoothed failure probability and decreasing novelty bonus.
            let failure=1000u128*(u128::from(attempts.saturating_sub(successes))+1)/(u128::from(attempts)+2);
            let novelty=1000u64/(attempts.saturating_add(1));
            let uncertainty=i128::from(t.spec.uncertainty)+failure as i128;
            let gain=i128::from(t.spec.information_gain)+i128::from(novelty);
            let score=i128::from(t.spec.priority)*i128::from(p.priority_weight)
                +i128::from(path.get(&t.spec.id).copied().unwrap_or(0))*i128::from(p.critical_weight)
                +uncertainty*i128::from(p.uncertainty_weight)
                +gain*i128::from(p.information_weight)
                -i128::from(costs[&t.spec.id])*i128::from(p.cost_weight);
            (t.spec.id.clone(),score)
        }).collect()
    }
}
