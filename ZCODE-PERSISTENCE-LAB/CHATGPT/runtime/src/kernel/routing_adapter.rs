//! Durable host adapter handoff. This protocol never performs network I/O.
//! The trusted adapter claims a specific work envelope once, then either
//! supplies a receipt or leaves its charge reserved for reconciliation.
use super::{model::*,routing::{Status,Usage,Outcome}};
use crate::{digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::collections::BTreeMap;

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct AdapterBinding{
    pub id:Id,pub adapter:Id,pub version:u64,pub enabled:bool,
    pub class:Id,pub context:Id,pub lease_ms:u64,
}
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="snake_case")]
pub enum HandoffPhase{Prepared,Claimed,Unknown,Completed,Released}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Envelope{
    pub id:Id,pub reservation:Id,pub adapter:Id,pub binding_version:u64,
    pub profile:Id,pub provider:Id,pub account:Id,pub model:String,pub thinking:String,
    pub task:Id,pub generation:u64,pub worker:Id,pub incarnation:u64,
    pub input_digest:Digest,pub ceiling:Usage,pub deadline_ms:u64,
    pub phase:HandoffPhase,pub claimed_ms:Option<u64>,pub receipt:Option<Digest>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct AdapterReceipt{
    pub handoff:Id,pub request_id:Id,pub provider:Id,pub account:Id,pub model:String,pub thinking:String,
    pub usage:Usage,pub outcome:Outcome,pub provider_receipt:String,pub latency_ms:u64,
    pub output:Option<Value>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct QuotaReceipt{
    pub id:Id,pub adapter:Id,pub provider:Id,pub account:Id,
    pub observation:super::routing::Observation,pub source_receipt:String,
}
#[derive(Clone,Debug,Serialize,Deserialize,Default)]
pub struct AdapterState{
    #[serde(default)] pub quota_receipts:BTreeMap<Id,QuotaReceipt>,
    pub bindings:BTreeMap<Id,AdapterBinding>,pub handoffs:BTreeMap<Id,Envelope>,
    pub receipts:BTreeMap<Id,AdapterReceipt>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct ChargeOnlyReceipt{
    pub handoff:Id,pub provider_receipt:String,pub actual_provider:Id,pub actual_account:Id,
    pub actual_model:String,pub actual_thinking:String,pub actual_by_reserved_pool:BTreeMap<Id,u64>,
    pub reason:String,
}
impl World{
    /// A transport may hold only its own handoff authority. Configuration,
    /// quotas, charge-only reconciliation and all other host mutations remain
    /// host-only; late settlement is permitted after adapter disable.
    pub(crate) fn authorize_adapter_action(&self,adapter:&str,action:&super::transition::Action)->Result<&Envelope>{
        use super::transition::Action;
        id(adapter)?;
        let key=match action{
            Action::ClaimHandoff{handoff}=>handoff,
            Action::FinishHandoff{receipt}=>&receipt.handoff,
            Action::InferenceUnknown{reservation}=>reservation,
            _=>return Err("adapter action not authorized".into()),
        };
        let envelope=self.router.adapters.handoffs.get(key).ok_or("adapter handoff missing")?;
        ensure(envelope.adapter==adapter,"handoff belongs to another adapter")?;
        Ok(envelope)
    }
    /// Explicit host reconciliation of an identity mismatch. Exact observed
    /// charges for every original pool are required; no fabricated model credit.
    pub fn reconcile_handoff_charge(&mut self,r:ChargeOnlyReceipt)->Result<Value>{
        let e=self.router.adapters.handoffs.get(&r.handoff).ok_or("handoff missing")?.clone();
        ensure(matches!(e.phase,HandoffPhase::Claimed|HandoffPhase::Unknown),"handoff not outstanding")?;
        ensure(!r.provider_receipt.trim().is_empty()&&r.provider_receipt.len()<=4096&&!r.reason.trim().is_empty(),"charge reconciliation evidence required")?;
        id(&r.actual_provider)?;id(&r.actual_account)?;
        ensure(!r.actual_model.trim().is_empty()&&!r.actual_thinking.trim().is_empty(),"actual model identity required")?;
        ensure(!self.router.reservations.values().any(|x|x.id!=r.handoff&&x.receipt.as_deref()==Some(r.provider_receipt.as_str())),"charge receipt already attributed")?;
        let reservation=self.router.reservations.get_mut(&r.handoff).ok_or("reservation missing")?;
        ensure(matches!(reservation.status,Status::Dispatched|Status::Unknown)&&reservation.holds.keys().eq(r.actual_by_reserved_pool.keys()),"charge pool mismatch")?;
        let evidence=serde_json::to_value(&r).map_err(|e|e.to_string())?;
        bounded(&evidence)?;
        reservation.actual=r.actual_by_reserved_pool.clone();reservation.status=Status::Settled;
        reservation.outcome=Some(Outcome::Rejected);reservation.ended_ms=Some(self.clock_ms);
        reservation.receipt=Some(r.provider_receipt.clone());reservation.not_evaluated_reason=Some(r.reason.clone());
        let profile=reservation.profile.id.clone();
        self.router.cooldowns.insert(profile,self.clock_ms.saturating_add(86_400_000));
        let hash=self.put_payload(evidence)?;
        let x=self.router.adapters.handoffs.get_mut(&r.handoff).unwrap();x.phase=HandoffPhase::Completed;x.receipt=Some(hash.clone());
        if self.tasks.get(&e.task).is_some_and(|t|t.phase==TaskPhase::Running&&t.generation==e.generation){self.invalidate_attempt(&e.task,"provider identity mismatch")?;}
        Ok(json!({"settled":r.handoff,"evidence":hash,"model_credit":false}))
    }

    pub fn observe_adapter_quota(&mut self,r:QuotaReceipt)->Result<()>{
        id(&r.id)?;ensure(!r.source_receipt.trim().is_empty()&&r.source_receipt.len()<=4096,"quota source receipt required")?;
        ensure(!self.router.adapters.quota_receipts.contains_key(&r.id),"quota receipt exists")?;
        ensure(self.router.adapters.bindings.get(&r.adapter).is_some_and(|b|b.enabled),"adapter not enabled")?;
        let c=self.router.config.as_ref().ok_or("routing not configured")?;
        ensure(c.profiles.values().any(|p|p.enabled&&p.adapter==r.adapter&&p.provider==r.provider&&p.account==r.account&&p.pools.contains(&r.observation.pool)),"quota receipt outside adapter scope")?;
        self.router.observe(r.observation.clone())?;
        self.router.adapters.quota_receipts.insert(r.id.clone(),r);Ok(())
    }
    pub fn bind_routing_adapter(&mut self,b:AdapterBinding)->Result<()>{
        id(&b.id)?;id(&b.adapter)?;ensure(self.router.config.is_some(),"router not configured")?;
        ensure(b.id==b.adapter&&b.version>self.router.adapters.bindings.get(&b.id).map_or(0,|x|x.version)&&(100..=3_600_000).contains(&b.lease_ms),"invalid adapter binding")?;
        ensure(self.classes.contains_key(&b.class)&&self.contexts.contains_key(&b.context),"adapter class/context missing")?;
        self.router.adapters.bindings.insert(b.id.clone(),b);Ok(())
    }
    pub fn prepare_handoff(&mut self,reservation:&str)->Result<Value>{
        let r=self.router.reservations.get(reservation).ok_or("reservation missing")?.clone();
        ensure(r.status==Status::Reserved,"reservation not prepared")?;
        ensure(!self.router.adapters.handoffs.contains_key(reservation),"handoff exists")?;
        let b=self.router.adapters.bindings.get(&r.profile.adapter).ok_or("adapter not registered")?.clone();
        ensure(b.enabled,"adapter disabled")?;
        let t=self.tasks.get(&r.task).ok_or("task missing")?.clone();
        ensure(t.phase==TaskPhase::Pending&&self.ready().contains(&r.task),"task no longer ready")?;
        ensure(t.spec.context.as_ref().is_none_or(|x|x==&b.context),"adapter context not authorized")?;
        // One logical agent per goal/task/adapter class/context, reused across attempts.
        let worker=format!("routed-{}",digest(&json!([r.goal,r.task,b.adapter,b.class,b.context])));
        let actor=super::transition::Actor::Host;
        use super::transition::Action;
        if !self.agents.contains_key(&worker){
            self.apply_adapter_action(&actor,Action::Agent{id:worker.clone(),goal:r.goal.clone(),parent:None,class:b.class.clone(),context:b.context.clone()})?;
        }
        if self.current_incarnation(&worker).is_ok(){ensure(self.agents[&worker].incarnations.last().is_some_and(|i|i.executor==format!("routed:{}",b.adapter)),"unexpected worker executor")?;}
        if self.current_incarnation(&worker).is_err(){
            self.apply_adapter_action(&actor,Action::Activate{agent:worker.clone(),executor:format!("routed:{}",b.adapter)})?;
        }
        let claim=self.apply_adapter_action(&actor,Action::Claim{task:r.task.clone(),agent:worker.clone(),ttl_ms:b.lease_ms})?;
        let lease:Lease=serde_json::from_value(claim["lease"].clone()).map_err(|e|e.to_string())?;
        let input_digest=self.put_payload(claim["input"].clone())?;
        let e=Envelope{id:reservation.into(),reservation:reservation.into(),adapter:b.adapter,binding_version:b.version,profile:r.profile.id,provider:r.profile.provider,account:r.profile.account,model:r.profile.model,thinking:r.profile.thinking,task:r.task,generation:lease.generation,worker,incarnation:lease.incarnation,input_digest,ceiling:r.ceiling,deadline_ms:lease.deadline_ms,phase:HandoffPhase::Prepared,claimed_ms:None,receipt:None};
        self.router.adapters.handoffs.insert(reservation.into(),e.clone());Ok(json!({"handoff":e,"execution":"not_dispatched"}))
    }
    /// One-shot handoff. Caller must durably commit before transmitting to provider.
    pub fn claim_handoff(&mut self,key:&str)->Result<Value>{
        let e=self.router.adapters.handoffs.get(key).ok_or("handoff missing")?.clone();
        ensure(e.phase==HandoffPhase::Prepared&&self.clock_ms<e.deadline_ms,"handoff unavailable")?;
        ensure(self.router.adapters.bindings.get(&e.adapter).is_some_and(|b|b.enabled&&b.version==e.binding_version),"adapter binding changed")?;
        let input=self.payload(&e.input_digest)?.clone();
        self.dispatch_inference(key)?;
        let x=self.router.adapters.handoffs.get_mut(key).unwrap();x.phase=HandoffPhase::Claimed;x.claimed_ms=Some(self.clock_ms);
        Ok(json!({"handoff":x,"input":input,"send_authority":"single_attempt_only"}))
    }
    pub fn finish_handoff(&mut self,r:AdapterReceipt)->Result<Value>{
        id(&r.request_id)?;
        let e=self.router.adapters.handoffs.get(&r.handoff).ok_or("handoff missing")?.clone();
        ensure(matches!(e.phase,HandoffPhase::Claimed|HandoffPhase::Unknown),"handoff not outstanding")?;
        ensure(r.provider==e.provider&&r.account==e.account&&r.model==e.model&&r.thinking==e.thinking,"actual provider/model/thinking identity mismatch; reconcile usage separately")?;
        ensure(!self.router.adapters.receipts.values().any(|x|x.provider==r.provider&&x.account==r.account&&x.request_id==r.request_id),"duplicate provider request receipt")?;
        ensure((r.outcome==Outcome::Completed)==r.output.is_some(),"output/outcome mismatch")?;
        // Account for provider usage even when its output is too large or invalid.
        let usable=r.output.as_ref().is_some_and(|v|bounded(v).is_ok());
        let charge=self.router.settle(&r.handoff,r.usage.clone(),r.outcome.clone(),r.provider_receipt.clone(),r.latency_ms)?;
        let live=self.tasks.get(&e.task).is_some_and(|t|t.phase==TaskPhase::Running&&t.lease.as_ref().is_some_and(|l|l.generation==e.generation&&l.worker==e.worker&&l.incarnation==e.incarnation&&self.clock_ms<l.deadline_ms));
        let mut submitted=false;
        if live{
            use super::transition::{Actor,Action};
            let worker=Actor::Worker{id:e.worker.clone(),incarnation:e.incarnation};
            let shape_ok=usable&&r.output.as_ref().is_some_and(|v|self.tasks[&e.task].spec.output.accepts(v));
            if r.outcome==Outcome::Completed&&shape_ok{
                let mut staged=self.clone();
                if staged.apply_adapter_action(&worker,Action::Submit{task:e.task.clone(),generation:e.generation,output:r.output.clone().unwrap()}).is_ok(){*self=staged;submitted=true;}
                else{self.invalidate_attempt(&e.task,"adapter output could not be admitted")?;}
            }else{
                self.apply_adapter_action(&worker,Action::Fail{task:e.task.clone(),generation:e.generation,reason:"adapter failed or invalid output".into()})?;
            }
        }
        if r.outcome==Outcome::Completed&&!submitted{self.router.reject_unusable_output(&r.handoff)?;}
        let hash=digest(&serde_json::to_value(&r).map_err(|e|e.to_string())?);
        let x=self.router.adapters.handoffs.get_mut(&r.handoff).unwrap();x.phase=HandoffPhase::Completed;x.receipt=Some(hash);
        let mut metadata=r;metadata.output=None;
        self.router.adapters.receipts.insert(metadata.handoff.clone(),metadata);
        Ok(json!({"usage":charge,"submitted":submitted,"stale_output_discarded":!live}))
    }
    pub fn refresh_handoffs(&mut self){
        for e in self.router.adapters.handoffs.values_mut(){
            let Some(r)=self.router.reservations.get(&e.reservation) else{continue;};
            match r.status{
                Status::Unknown if e.phase==HandoffPhase::Claimed=>e.phase=HandoffPhase::Unknown,
                Status::Released if e.phase==HandoffPhase::Prepared=>e.phase=HandoffPhase::Released,
                _=>{}
            }
        }
    }
    pub fn adapter_queue(&self,adapter:&str)->Result<Value>{
        ensure(self.router.adapters.bindings.get(adapter).is_some_and(|b|b.enabled),"adapter not enabled")?;
        let items:Vec<_>=self.router.adapters.handoffs.values().filter(|e|e.adapter==adapter&&matches!(e.phase,HandoffPhase::Prepared|HandoffPhase::Unknown)).collect();
        Ok(json!({"adapter":adapter,"handoffs":items}))
    }
}
