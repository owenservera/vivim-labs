//! Integration between the objective/task kernel and model routing. A selection
//! is a reservation, not permission to send a request outside the Host.
use super::{model::*,routing::{Demand,Status,Outcome}};
use crate::{digest,Result};
use serde_json::{Value,json};
use std::collections::BTreeSet;
impl World{
    pub fn queue_inference(&mut self,d:Demand)->Result<()>{
        let t=self.tasks.get(&d.task).ok_or("routing task missing")?;
        ensure(t.spec.goal==d.goal&&t.phase==TaskPhase::Pending&&self.goals.get(&d.goal).is_some_and(|g|g.phase==GoalPhase::Open),"routing task not pending in open goal")?;
        if let Some(family)=&t.spec.family{ensure(family==&d.family,"routing family mismatch")?;}
        self.router.queue(d)
    }
    fn inference_needed(&self,d:&Demand)->bool{
        self.tasks.get(&d.task).is_some_and(|t|t.phase==TaskPhase::Pending&&self.goals.get(&d.goal).is_some_and(|g|g.phase==GoalPhase::Open)
            &&(!d.prefer_native||d.independent_review||self.match_class(&t.spec).is_none()))
    }
    pub fn native_allowed(&self,task:&str)->bool{
        !self.router.demands.values().any(|d|d.task==task&&(!d.prefer_native||d.independent_review||self.router.reservations.values().any(|r|r.demand==d.id&&matches!(r.status,Status::Reserved|Status::Dispatched|Status::Unknown))))
    }
    pub fn inference_plan(&self)->Result<Value>{
        let ready:BTreeSet<_>=self.ready().into_iter().collect();
        let allowed:BTreeSet<_>=self.router.demands.values().filter(|d|ready.contains(&d.task)&&self.inference_needed(d)).map(|d|d.id.clone()).collect();
        let native:Vec<_>=self.router.demands.values().filter(|d|ready.contains(&d.task)&&!self.inference_needed(d)).map(|d|d.id.clone()).collect();
        Ok(json!({"ranked":super::frontier::rank(&self.router,&allowed)?,"native_preferred":native}))
    }
    /// Atomic batch reservation. The ranking is recomputed after each hold;
    /// the Host still must acquire a task lease and bind the chosen adapter.
    pub fn allocate_inference(&mut self,limit:usize)->Result<Value>{
        ensure((1..=128).contains(&limit),"invalid routing batch limit")?;
        let mut receipts=vec![];
        let mut selected=BTreeSet::new();
        let ready:BTreeSet<_>=self.ready().into_iter().collect();
        let slots=self.slots.saturating_sub(self.running());
        let preheld=self.router.reservations.values().filter(|r|r.status==Status::Reserved).count();
        let cap=limit.min(slots.saturating_sub(preheld));
        for _ in 0..cap{
            let allowed:BTreeSet<_>=self.router.demands.values().filter(|d|ready.contains(&d.task)&&self.inference_needed(d)&&!selected.contains(&d.id)).map(|d|d.id.clone()).collect();
            let Some(next)=super::frontier::rank(&self.router,&allowed)?.into_iter().next() else{break;};
            let key=format!("route-{}",digest(&json!([self.sequence+1,receipts.len(),next.demand])));
            let response=self.router.reserve(key,&next.demand)?;
            selected.insert(next.demand);receipts.push(response);
        }
        Ok(json!({"allocations":receipts,"native_slots_available":slots,"execution":"not_dispatched"}))
    }
    pub fn dispatch_inference(&mut self,key:&str)->Result<()>{
        let r=self.router.reservations.get(key).ok_or("routing reservation missing")?;
        let t=self.tasks.get(&r.task).ok_or("task missing")?;
        let l=t.lease.as_ref().ok_or("routing requires task lease")?;
        ensure(t.phase==TaskPhase::Running&&self.clock_ms<l.deadline_ms&&self.goals.get(&r.goal).is_some_and(|g|g.phase==GoalPhase::Open),"routing requires current open task lease")?;
        let a=self.agents.get(&l.worker).ok_or("worker missing")?;
        let i=a.incarnations.last().ok_or("incarnation missing")?;
        ensure(i.number==l.incarnation&&i.ended_ms.is_none()&&i.executor==format!("routed:{}",r.profile.adapter),"lease executor does not match reserved adapter")?;
        ensure(!self.router.reservations.values().any(|x|x.id!=key&&x.task==r.task&&x.task_generation==Some(l.generation)&&x.status!=Status::Released),"attempt already bound to inference")?;
        self.router.dispatch(key,l.generation,l.worker.clone(),l.incarnation)
    }
    pub fn verify_inference_feedback(&mut self,key:&str,evidence:&str)->Result<()>{
        let r=self.router.reservations.get(key).ok_or("reservation missing")?;
        let e=self.evidence.get(evidence).ok_or("independent evidence missing")?;
        ensure(e.task==r.task&&Some(e.generation)==r.task_generation&&r.agent.as_deref()!=Some(e.verifier.as_str()),"evidence does not match dispatched incarnation")?;
        ensure(!self.router.reservations.values().any(|x|x.evidence.as_deref()==Some(evidence)),"evidence already attributed")?;
        self.router.record_verified(key,e.accepted,evidence.into())
    }
    pub fn reconcile_inference(&mut self){
        let live:BTreeSet<_>=self.tasks.values().filter(|t|matches!(t.phase,TaskPhase::Running|TaskPhase::Submitted)).map(|t|(t.spec.id.clone(),t.generation)).collect();
        self.router.reconcile_stale(&live);
        let terminal:Vec<_>=self.goals.values().filter(|g|g.phase!=GoalPhase::Open).map(|g|g.id.clone()).collect();
        for g in terminal{self.router.cancel_goal(&g);}
    }
}
