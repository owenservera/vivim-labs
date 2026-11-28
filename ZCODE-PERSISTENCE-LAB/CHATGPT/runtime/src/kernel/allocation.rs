//! Integration between the objective/task kernel and model routing. A selection
//! is a reservation, not permission to send a request outside the Host.
use super::{model::*,routing::{Demand,Status,Outcome},routing_adapter::HandoffPhase};
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
            let key=format!("route-{}",digest(&json!([self.sequence+1,receipts.len(),&next.demand])));
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

impl World{
    fn bound_profiles(&self,d:&Demand)->BTreeSet<Id>{
        let Some(c)=&self.router.config else{return BTreeSet::new();};
        let Some(t)=self.tasks.get(&d.task) else{return BTreeSet::new();};
        c.profiles.values().filter(|p|p.enabled&&(d.allowed_profiles.is_empty()||d.allowed_profiles.contains(&p.id)))
            .filter(|p|{
                let Some(b)=self.router.adapters.bindings.get(&p.adapter) else{return false;};
                if !b.enabled||!self.class_available(&b.class)||t.spec.context.as_ref().is_some_and(|x|x!=&b.context){return false;}
                if let Some(class)=self.match_class(&t.spec){if class!=b.class{return false;}}
                else if t.spec.class.is_some(){return false;}
                self.classes.get(&b.class).is_some_and(|c|c.input.admits(&Self::input_shape(&t.spec))&&t.spec.output.admits(&c.output))
                    &&self.contexts.get(&b.context).is_some_and(|c|self.assemble(&t.spec,c).is_ok())
            }).map(|p|p.id.clone()).collect()
    }
    fn bound_plan(&self)->Result<Vec<super::frontier::RankedDemand>>{
        if self.router.config.is_none(){return Ok(vec![]);}
        let mut router=self.router.clone();
        let ready:BTreeSet<_>=self.ready().into_iter().collect();
        let mut allowed=BTreeSet::new();
        for d in self.router.demands.values().filter(|d|ready.contains(&d.task)&&self.inference_needed(d)){
            let profiles=self.bound_profiles(d);
            if !profiles.is_empty(){router.demands.get_mut(&d.id).unwrap().allowed_profiles=profiles;allowed.insert(d.id.clone());}
        }
        super::frontier::rank(&router,&allowed)
    }
    fn joint_inputs(&self)->(super::routing::Router,BTreeSet<Id>){
        let mut router=self.router.clone();let ready:BTreeSet<_>=self.ready().into_iter().collect();let mut allowed=BTreeSet::new();
        for d in self.router.demands.values().filter(|d|ready.contains(&d.task)&&self.inference_needed(d)){
            let profiles=self.bound_profiles(d);
            if !profiles.is_empty(){router.demands.get_mut(&d.id).unwrap().allowed_profiles=profiles;allowed.insert(d.id.clone());}
        }
        (router,allowed)
    }
    pub fn joint_routing_plan(&self,limit:usize)->Result<super::routing_optimizer::BatchPlan>{
        let (router,allowed)=self.joint_inputs();router.optimize_batch(&allowed,limit)
    }
    pub fn compare_routing_policies(&self,limit:usize,policies:Vec<super::routing_optimizer::BatchPolicy>)->Result<Value>{
        let (router,allowed)=self.joint_inputs();router.compare_batch_policies(&allowed,limit,policies)
    }
    pub fn routing_cycle_needed(&self)->bool{
        self.router.config.is_some()&&self.running()<self.slots&&if self.router.batch_policy.version>0{self.joint_routing_plan(self.slots.saturating_sub(self.running()).min(128)).is_ok_and(|p|!p.assignments.is_empty())}else{self.bound_plan().is_ok_and(|v|!v.is_empty())}
    }
    /// Atomic autonomous preparation. An enabled adapter binding is mandatory.
    /// No provider is called; an outbox item is made available to the adapter.
    pub fn routing_cycle(&mut self,limit:usize)->Result<Value>{
        ensure((1..=128).contains(&limit),"invalid routing cycle limit")?;
        let mut prepared=Vec::new();self.refresh_frontier_demand()?;
        for n in 0..limit.min(self.slots.saturating_sub(self.running())){
            let (demand,profile)=if self.router.batch_policy.version>0{
                let plan=self.joint_routing_plan((limit-n).min(self.slots.saturating_sub(self.running())))?;
                let Some(a)=plan.assignments.into_iter().next() else{break;};(a.demand,Some(a.profile))
            }else{let Some(next)=self.bound_plan()?.into_iter().next() else{break;};(next.demand,None)};
            let original=self.router.demands[&demand].allowed_profiles.clone();
            let profiles=self.bound_profiles(&self.router.demands[&demand]);
            let key=format!("route-{}",digest(&json!([self.sequence+1,n,&demand,"auto"])));
            self.router.demands.get_mut(&demand).unwrap().allowed_profiles=profiles;
            let reserved=match profile{Some(p)=>self.router.reserve_profile(key.clone(),&demand,&p)?,None=>self.router.reserve(key.clone(),&demand)?};
            self.router.demands.get_mut(&demand).unwrap().allowed_profiles=original;
            if reserved["reserved"]!=true{break;}
            prepared.push(self.prepare_handoff(&key)?);
            self.refresh_frontier_demand()?;
        }
        Ok(json!({"prepared":prepared,"provider_calls":0}))
    }
    pub fn reconcile_prepared(&mut self)->Result<()>{
        let mut invalidate=vec![];
        let stale:Vec<_>=self.router.adapters.handoffs.values().filter(|e|e.phase==HandoffPhase::Prepared)
            .filter(|e|{
                let live=self.tasks.get(&e.task).is_some_and(|t|t.phase==TaskPhase::Running&&t.lease.as_ref().is_some_and(|l|l.generation==e.generation&&l.worker==e.worker&&l.incarnation==e.incarnation&&self.clock_ms<l.deadline_ms));
                let reserved=self.router.reservations.get(&e.reservation).is_some_and(|r|r.status==Status::Reserved&&self.router.config.as_ref().is_some_and(|c|c.policy.version==r.policy_version));
                let binding=self.router.adapters.bindings.get(&e.adapter).is_some_and(|b|b.enabled&&b.version==e.binding_version);
                !live||!reserved||!binding
            }).cloned().collect();
        for e in stale{
            if self.router.reservations.get(&e.reservation).is_some_and(|r|r.status==Status::Reserved){self.router.release_unsent(&e.reservation)?;}
            self.router.adapters.handoffs.get_mut(&e.id).unwrap().phase=HandoffPhase::Released;
            if self.tasks.get(&e.task).is_some_and(|t|t.phase==TaskPhase::Running&&t.generation==e.generation){invalidate.push(e.task);}
        }
        for t in invalidate{self.invalidate_attempt(&t,"undispatched routing handoff expired or revoked")?;}
        Ok(())
    }
}
