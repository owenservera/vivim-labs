//! Durable routing disposition. Waiting is not success or exhaustion.
use super::{model::*,routing::{Status,Outcome,Window}};
use crate::Result;
use serde::{Serialize,Deserialize};
use std::collections::BTreeMap;
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct WaitRecord{pub reason:String,pub since_ms:u64,pub next_check_ms:Option<u64>,pub terminal:bool}
impl World{
    pub fn refresh_routing_waits(&mut self)->Result<()>{
        let Some(c)=self.router.config.as_ref() else{return Ok(());};
        let mut records=BTreeMap::new();let mut terminal=Vec::new();
        for d in self.router.demands.values(){
            let Some(t)=self.tasks.get(&d.task) else{continue;};
            if t.phase!=TaskPhase::Pending||!self.inference_needed(d){continue;}
            let history:Vec<_>=self.router.reservations.values().filter(|r|r.demand==d.id).collect();
            let outstanding=history.iter().any(|r|matches!(r.status,Status::Reserved|Status::Dispatched|Status::Unknown)||r.status==Status::Settled&&r.outcome==Some(Outcome::Completed)&&r.verified.is_none()&&r.not_evaluated_reason.is_none());
            let expired=d.deadline_ms.is_some_and(|x|x<=self.clock_ms)||history.iter().filter(|r|r.status!=Status::Released).count()>=c.policy.max_attempts as usize;
            let profiles=self.bound_profiles(d);
            let (reason,next,end)=if outstanding{("outstanding_reservation_or_reconciliation",None,false)}
                else if expired{("routing_attempt_or_deadline_exhausted",None,true)}
                else if !self.router.trial_allows(&d.id){("outside_active_trial_arm",None,false)}
                else if !t.spec.dependencies.iter().all(|k|self.tasks.get(k).is_some_and(|p|p.phase==TaskPhase::Verified)){("dependency_wait",None,false)}
                else if profiles.is_empty(){("adapter_or_context_configuration",None,false)}
                else{
                    let mut router=self.router.clone();router.demands.get_mut(&d.id).unwrap().allowed_profiles=profiles;
                    let rec=router.recommend(&d.id)?;
                    if rec.chosen.is_some(){continue;}
                    let next=c.pools.values().filter_map(|p|match p.window{Window::Fixed{..}=>p.window.bounds(self.clock_ms).ok().and_then(|x|x.1),_=>None}).chain(self.router.cooldowns.values().copied()).chain(self.router.provider_cooldowns.values().copied()).filter(|x|*x>self.clock_ms).min();
                    ("quota_health_or_policy_wait",next,false)
                };
            // A demand deadline is relevant even while waiting for a context,
            // adapter or quota; the controller can wake at that exact boundary.
            let next=match (next,d.deadline_ms.filter(|n|*n>self.clock_ms)){
                (Some(a),Some(b))=>Some(a.min(b)),(Some(a),None)=>Some(a),(None,b)=>b,
            };
            let since=self.router.waits.get(&d.id).filter(|w|w.reason==reason).map_or(self.clock_ms,|w|w.since_ms);
            records.insert(d.id.clone(),WaitRecord{reason:reason.into(),since_ms:since,next_check_ms:next,terminal:end});
            if end{terminal.push(d.task.clone());}
        }
        for (k,w) in &self.router.waits{if w.terminal{records.entry(k.clone()).or_insert_with(||w.clone());}}
        self.router.waits=records;
        for key in terminal{let t=self.tasks.get_mut(&key).unwrap();t.phase=TaskPhase::Exhausted;t.reason=Some("routing attempt or deadline exhausted".into());}
        Ok(())
    }
}
