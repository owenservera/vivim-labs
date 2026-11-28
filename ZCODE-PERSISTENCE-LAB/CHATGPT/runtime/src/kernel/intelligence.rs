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
        ensure(fingerprints.iter().all(|d|!previous.contains(d)),"holdout already consumed")?;
        // A rejected cohort cannot be repeatedly tuned against the same holdout.
        let mut attempt=LearningAttempt{plan:key.into(),accepted:false,capability:None,reason:None,evaluated:None,holdout_inputs:fingerprints,clock_ms:self.clock_ms};
        match self.discover(&p.family,p.training.clone(),p.holdouts.clone(),p.input,p.output,p.search_limit){
            Ok(d)=>{
                attempt.evaluated=Some(d.candidates_evaluated);
                self.metrics.candidate_evaluations=self.metrics.candidate_evaluations.saturating_add(d.candidates_evaluated);
                match self.assimilate(&p.capability,p.capability_version,d){
                    Ok(cap)=>{attempt.accepted=true;attempt.capability=Some(cap);}
                    Err(reason)=>attempt.reason=Some(reason),
                }
            }
            Err(reason)=>attempt.reason=Some(reason),
        }
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
        ensure(generation>0&&generation==t.generation&&elapsed_ms<=3_600_000,"invalid cost receipt")?;
        let l=t.lease.as_ref().ok_or("cost receipt requires existing lease")?;
        ensure(l.generation==generation,"stale cost receipt")?;
        let a=self.agents.get(&l.worker).ok_or("agent missing")?;
        ensure(a.incarnations.iter().any(|i|i.number==l.incarnation&&i.executor=="native-pure-process"),"cost receipt requires native executor")?;
        let key=format!("{task}-{generation}");
        if let Some(old)=self.run_costs.get(&key){return ensure(old.elapsed_ms==elapsed_ms,"cost receipt conflict");}
        self.run_costs.insert(key,RunCost{task:task.into(),generation,elapsed_ms,family:t.spec.family.clone(),class:t.assigned_class.clone()});
        self.metrics.worker_elapsed_ms=self.metrics.worker_elapsed_ms.saturating_add(elapsed_ms);Ok(())
    }
    fn observed_costs(&self)->BTreeMap<Id,(u128,u128)>{
        let mut out:BTreeMap<Id,(u128,u128)>=BTreeMap::new();
        for r in self.run_costs.values(){
            if let Some(f)=&r.family{let e=out.entry(f.clone()).or_default();e.0=e.0.saturating_add(u128::from(r.elapsed_ms));e.1=e.1.saturating_add(1);}
        }out
    }
    pub fn schedule_scores(&self)->BTreeMap<Id,i128>{
        let observations=self.observed_costs();
        let live=|t:&Task|matches!(t.phase,TaskPhase::Pending|TaskPhase::Running|TaskPhase::Submitted);
        let costs:BTreeMap<_,u64>=self.tasks.values().filter(|t|live(t)).map(|t|{
            let cost=t.spec.family.as_ref().and_then(|f|observations.get(f)).map(|(sum,n)|u64::try_from((sum/n).max(1)).unwrap_or(u64::MAX)).unwrap_or(u64::from(t.spec.expected_cost));
            (t.spec.id.clone(),cost)
        }).collect();
        let mut successors:BTreeMap<Id,Vec<Id>>=costs.keys().map(|k|(k.clone(),vec![])).collect();
        let mut degree:BTreeMap<Id,usize>=costs.keys().map(|k|(k.clone(),0)).collect();
        for t in self.tasks.values().filter(|t|live(t)){
            for p in &t.spec.dependencies{if let Some(s)=successors.get_mut(p){s.push(t.spec.id.clone());*degree.get_mut(&t.spec.id).unwrap()+=1;}}
        }
        let mut queue:BTreeSet<_>=degree.iter().filter(|(_,d)|**d==0).map(|(k,_)|k.clone()).collect();
        let mut order=vec![];
        while let Some(k)=queue.pop_first(){order.push(k.clone());for s in &successors[&k]{let n=degree.get_mut(s).unwrap();*n-=1;if *n==0{queue.insert(s.clone());}}}
        let mut path:BTreeMap<Id,u64>=BTreeMap::new();
        for k in order.into_iter().rev(){let tail=successors[&k].iter().filter_map(|s|path.get(s)).copied().max().unwrap_or(0);path.insert(k.clone(),costs[&k].saturating_add(tail));}
        let p=&self.policy;
        self.tasks.values().filter(|t|live(t)).map(|t|{
            let score=i128::from(t.spec.priority)*i128::from(p.priority_weight)
                +i128::from(path.get(&t.spec.id).copied().unwrap_or(0))*i128::from(p.critical_weight)
                +i128::from(t.spec.uncertainty)*i128::from(p.uncertainty_weight)
                +i128::from(t.spec.information_gain)*i128::from(p.information_weight)
                -i128::from(costs[&t.spec.id])*i128::from(p.cost_weight);
            (t.spec.id.clone(),score)
        }).collect()
    }
}
