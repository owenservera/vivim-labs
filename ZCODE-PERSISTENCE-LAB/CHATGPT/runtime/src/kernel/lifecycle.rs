//! Durable delivery, complete attempt history, and evidence-driven rollback.
use super::model::*;
use crate::{digest,Result};
use serde_json::{Value,json};
use std::collections::BTreeSet;

impl World{
    pub fn receive(&mut self,agent:&str,incarnation:u64,limit:usize,ttl_ms:u64)->Result<Value>{
        ensure((1..=128).contains(&limit)&&(1..=3_600_000).contains(&ttl_ms),"invalid delivery bounds")?;
        ensure(self.current_incarnation(agent)?==incarnation,"stale recipient")?;
        let a=self.agents.get(agent).ok_or("recipient missing")?;
        ensure(a.phase!=AgentPhase::Archived,"recipient archived")?;
        let until=self.clock_ms.checked_add(ttl_ms).ok_or("delivery deadline overflow")?;
        let mut available:Vec<_>=self.messages.values().filter(|m|m.recipient==agent&&!m.acknowledged)
            .filter(|m|self.delivery.get(&m.id).is_none_or(|d|d.last_incarnation!=Some(incarnation)||d.lease_until_ms<=self.clock_ms))
            .map(|m|(m.sequence,m.id.clone())).collect();
        available.sort();
        let mut batch=Vec::new();
        for (_,key) in available.into_iter().take(limit){
            let m=self.messages[&key].clone();let payload=self.payload(&m.body)?.clone();
            let d=self.delivery.entry(key.clone()).or_insert(Delivery{message:key.clone(),recipient:agent.into(),attempts:0,last_incarnation:None,lease_until_ms:0,acknowledged_ms:None});
            d.attempts=d.attempts.checked_add(1).ok_or("delivery attempts overflow")?;
            d.last_incarnation=Some(incarnation);d.lease_until_ms=until;
            batch.push(json!({"id":key,"sender":m.sender,"sequence":m.sequence,"body":payload,"attempt":d.attempts,"lease_until_ms":until}));
        }
        Ok(json!({"recipient":agent,"incarnation":incarnation,"messages":batch}))
    }
    pub fn acknowledge(&mut self,agent:&str,incarnation:u64,key:&str)->Result<()>{
        let m=self.messages.get_mut(key).ok_or("message missing")?;
        ensure(m.recipient==agent,"wrong recipient")?;
        if m.acknowledged{return Ok(());}
        let d=self.delivery.get_mut(key).ok_or("message was not delivered")?;
        ensure(d.last_incarnation==Some(incarnation)&&self.clock_ms<d.lease_until_ms,"stale delivery acknowledgement")?;
        m.acknowledged=true;d.acknowledged_ms=Some(self.clock_ms);Ok(())
    }
    pub fn begin_attempt(&mut self,task:&str)->Result<()>{
        let t=self.tasks.get(task).ok_or("task missing")?;
        let l=t.lease.as_ref().ok_or("lease missing")?;
        let key=format!("{task}-{}",l.generation);
        ensure(!self.attempt_history.contains_key(&key),"attempt already recorded")?;
        let class=t.assigned_class.clone().ok_or("assigned class missing")?;
        self.attempt_history.insert(key,AttemptRecord{task:task.into(),generation:l.generation,agent:l.worker.clone(),incarnation:l.incarnation,class,started_ms:self.clock_ms,ended_ms:None,outcome:None,measured_ms:None});Ok(())
    }
    pub fn end_attempt(&mut self,task:&str,outcome:&str)->Result<()>{
        let t=self.tasks.get(task).ok_or("task missing")?;
        if t.generation==0{return Ok(());}
        let key=format!("{task}-{}",t.generation);
        if let Some(a)=self.attempt_history.get_mut(&key){
            if a.ended_ms.is_none(){a.ended_ms=Some(self.clock_ms);a.outcome=Some(outcome.into());}
        }Ok(())
    }
    pub fn audit_population(&self)->Value{
        let active=self.running();
        let ended=self.attempt_history.values().filter(|a|a.ended_ms.is_some()).count();
        let measured=self.attempt_history.values().filter(|a|a.measured_ms.is_some()).count();
        let elapsed:u64=self.attempt_history.values().filter_map(|a|a.measured_ms).fold(0,u64::saturating_add);
        let logical_elapsed:u64=self.attempt_history.values().filter_map(|a|a.ended_ms.map(|end|end.saturating_sub(a.started_ms))).fold(0,u64::saturating_add);
        let classes:BTreeSet<_>=self.agents.values().map(|a|&a.class).collect();
        // Do not report zero inference merely because the native process
        // scheduler did not perform the provider operation itself.
        let routed_dispatches=self.router.reservations.values().filter(|r|r.started_ms.is_some()).count();
        let confirmed_requests=self.router.adapters.receipts.values().filter(|r|!r.provider.starts_with("synthetic-")).fold(0u64,|sum,r|sum.saturating_add(r.usage.requests));
        let synthetic_requests=self.router.adapters.receipts.values().filter(|r|r.provider.starts_with("synthetic-")).fold(0u64,|sum,r|sum.saturating_add(r.usage.requests));
        let uncovered=self.router.reservations.values().filter(|r|r.started_ms.is_some()&&!self.router.adapters.receipts.contains_key(&r.id)).count();
        json!({"logical_agents":self.agents.len(),"active_leases":active,"slots":self.slots,"idle_or_archived_agents":self.agents.len().saturating_sub(active),"distinct_classes":classes.len(),"attempts":self.attempt_history.len(),"ended_attempts":ended,"measured_attempts":measured,"attempt_elapsed_ms":if measured>0{Some(elapsed)}else{None},"logical_attempt_elapsed_ms":logical_elapsed,"verified_yield":self.metrics.accepted,"inference_calls":if uncovered==0{Some(confirmed_requests)}else{None},"confirmed_provider_requests":confirmed_requests,"synthetic_requests":synthetic_requests,"routed_dispatches":routed_dispatches,"uncovered_dispatches":uncovered})
    }
    /// Disable a proven-bad revision. Pending explicit users can be rebound only
    /// to an admitted compatible predecessor. Running work is fenced; submitted
    /// candidates still retain their evidence and independent oracle.
    pub fn rollback_capability(&mut self,key:&str,reason:&str)->Result<Rollback>{
        ensure(!reason.trim().is_empty(),"rollback reason required")?;
        let bad=self.capabilities.get(key).ok_or("capability missing")?.clone();
        let old_class=self.classes.get(&bad.class).ok_or("class missing")?.clone();
        let fallback=self.capabilities.iter().filter(|(_,c)|c.enabled&&c.id==bad.id&&c.family==bad.family&&c.version<bad.version)
            .filter(|(_,c)|self.classes.get(&c.class).is_some_and(|x|x.input.admits(&old_class.input)&&old_class.output.admits(&x.output)))
            .max_by_key(|(_,c)|c.version).map(|(_,c)|c.class.clone());
        self.capabilities.get_mut(key).unwrap().enabled=false;
        let affected:Vec<_>=self.tasks.values().filter(|t|t.spec.class.as_ref()==Some(&bad.class)||t.assigned_class.as_ref()==Some(&bad.class))
            .filter(|t|matches!(t.phase,TaskPhase::Pending|TaskPhase::Running)).map(|t|t.spec.id.clone()).collect();
        if !self.capabilities.values().any(|c|c.enabled&&c.class==bad.class){
            for knowledge in self.knowledge.values_mut().filter(|k|k.class.as_ref()==Some(&bad.class)){knowledge.enabled=false;}
            for t in &affected{
                if self.tasks[t].phase==TaskPhase::Running{
                    self.invalidate_attempt(t,"capability rollback")?;
                }
                if self.tasks[t].phase==TaskPhase::Pending&&self.tasks[t].spec.class.as_ref()==Some(&bad.class){
                    let replacement=fallback.as_ref().filter(|k|self.classes.get(*k).is_some_and(|c|c.input.admits(&Self::input_shape(&self.tasks[t].spec))&&self.tasks[t].spec.output.admits(&c.output))).cloned();
                    if let Some(class)=replacement{
                        let prior=self.tasks[t].spec.clone();self.task_versions.entry(t.clone()).or_default().push(prior);
                        let x=self.tasks.get_mut(t).unwrap();x.spec.class=Some(class);x.version=x.version.checked_add(1).ok_or("task version overflow")?;
                        x.input=None;x.assigned_class=None;x.assigned_context=None;
                    }else{let x=self.tasks.get_mut(t).unwrap();x.phase=TaskPhase::Blocked;x.reason=Some("no admitted rollback class".into());}
                }
            }
        }
        let record=Rollback{capability:key.into(),reason:reason.into(),fallback,sequence:self.sequence.checked_add(1).ok_or("sequence overflow")?,affected_tasks:affected};
        let id=format!("rollback-{}",digest(&json!([key,record.sequence])));
        self.rollback_history.insert(id,record.clone());Ok(record)
    }
    /// An independent oracle rejection invalidates a learned revision only when
    /// the actual submitted class is a currently admitted capability. It never
    /// weakens the task oracle to accommodate the learned program.
    pub fn reject_learned_class(&mut self,class:Option<&str>)->Result<()>{
        let Some(class)=class else{return Ok(());};
        let keys:Vec<_>=self.capabilities.iter().filter(|(_,c)|c.enabled&&c.class==class).map(|(k,_)|k.clone()).collect();
        for k in keys{self.rollback_capability(&k,"independent task oracle rejected learned revision")?;}
        Ok(())
    }
}
