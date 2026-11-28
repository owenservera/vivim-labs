//! Explicitly authorized, bounded routing-policy trials. No new provider
//! authority: every decision still passes the original budget/lease gates.
use super::{model::{Id,ensure,id},routing::{Router,Status,Outcome,DAY_MS},routing_optimizer::BatchPolicy};
use crate::{digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::collections::{BTreeMap,BTreeSet};
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct TrialSpec{
    pub id:Id,pub candidate:BatchPolicy,pub baseline_demands:BTreeSet<Id>,pub candidate_demands:BTreeSet<Id>,
    pub duration_ms:u64,pub caps:BTreeMap<Id,u64>,pub objective_pool:Id,
    pub min_samples:usize,pub min_success_bps:u16,pub min_savings_bps:u16,
    pub auto_promote:bool,
    #[serde(default)] pub auto_advance:bool,
}
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="snake_case")]
pub enum Phase{Baseline,Between,Candidate,Evaluated,Aborted}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Arm{
    pub start_ms:u64,pub end_ms:u64,pub policy:BatchPolicy,
    pub demands:BTreeSet<Id>,pub reservation_ids:BTreeSet<Id>,
    pub initial_available:BTreeMap<Id,u64>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct ArmReport{
    pub enrolled:usize,pub successes:usize,pub attempted:usize,pub settled_attempts:usize,
    pub arm:Arm,
    pub success_bps:u16,pub spent:BTreeMap<Id,u64>,pub overrun:bool,
    pub evidence:BTreeSet<Id>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Trial{
    pub spec:TrialSpec,pub config_digest:String,pub baseline:BatchPolicy,pub phase:Phase,
    pub arm:Option<Arm>,pub baseline_report:Option<ArmReport>,pub candidate_report:Option<ArmReport>,
    pub recommendation:Option<bool>,pub promoted:bool,pub reason:Option<String>,
}
#[derive(Clone,Debug,Serialize,Deserialize,Default)]
pub struct Evolution{
    pub current:Option<Id>,pub trials:BTreeMap<Id,Trial>,pub promotions:BTreeMap<Id,Promotion>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Promotion{
    pub trial:Id,pub previous:BatchPolicy,pub installed:BatchPolicy,pub clock_ms:u64,pub rollback_ms:Option<u64>,
}
fn outstanding(r:&super::routing::Reservation)->bool{
    matches!(r.status,Status::Reserved|Status::Dispatched|Status::Unknown)
    ||r.status==Status::Settled&&r.outcome==Some(Outcome::Completed)&&r.verified.is_none()&&r.not_evaluated_reason.is_none()
}
fn signature(d:&super::routing::Demand)->Result<String>{
    let mut v=serde_json::to_value(d).map_err(|e|e.to_string())?;
    let m=v.as_object_mut().ok_or("invalid demand")?;
    for key in ["id","goal","task","deadline_ms"]{m.remove(key);}
    Ok(digest(&v))
}
impl Router{
    fn trial(&self)->Option<&Trial>{self.evolution.current.as_ref().and_then(|k|self.evolution.trials.get(k))}
    pub fn trial_active(&self)->bool{self.evolution.current.is_some()}
    fn config_fingerprint(&self)->Result<String>{Ok(digest(&json!({"config":self.config.as_ref().ok_or("router not configured")?,"bindings":self.adapters.bindings,"frontier_guard":self.planning.guard,"forecasts":self.planning.forecasts})))}
    /// Generate bounded alternatives without changing the active policy.
    pub fn policy_candidates(&self)->Result<Value>{
        let p=&self.batch_policy;let version=p.version.checked_add(1).ok_or("policy version overflow")?;
        let mut a=p.clone();a.version=version;a.beam_width=(p.beam_width*2).clamp(1,16);a.node_budget=(p.node_budget*2).clamp(1,512);
        let mut b=p.clone();b.version=version;b.completion_weight=p.completion_weight.saturating_add(100).min(100_000);b.priority_weight=p.priority_weight/2;
        let mut c=p.clone();c.version=version;c.priority_weight=p.priority_weight.saturating_add(100).min(100_000);c.completion_weight=p.completion_weight/2;
        let mut d=p.clone();d.version=version;d.beam_width=1;d.node_budget=32;
        Ok(json!({"candidates":[a,b,c,d],"authority":"proposal_only"}))
    }
    pub fn begin_trial(&mut self,s:TrialSpec)->Result<()>{
        id(&s.id)?;ensure(!self.trial_active()&&!self.evolution.trials.contains_key(&s.id),"trial already active or ID reused")?;
        ensure(self.batch_policy.version>0,"enable a versioned batch policy before trials")?;
        ensure(!self.reservations.values().any(outstanding),"drain outstanding routing work first")?;
        ensure((100..=7*DAY_MS).contains(&s.duration_ms)&&(2..=4096).contains(&s.min_samples)&&(1..=10_000).contains(&s.min_success_bps)&&(1..=10_000).contains(&s.min_savings_bps),"invalid trial bounds")?;
        let n=s.baseline_demands.len();
        ensure(n>=s.min_samples&&n<=4096&&n==s.candidate_demands.len()&&s.baseline_demands.is_disjoint(&s.candidate_demands),"cohort size or overlap")?;
        let c=self.config.as_ref().ok_or("router not configured")?;
        ensure(s.caps.len()==c.pools.len()&&s.caps.contains_key(&s.objective_pool)&&s.caps[&s.objective_pool]>0,"explicit caps for every pool required")?;
        for (k,p) in &c.pools{ensure(s.caps.get(k).is_some_and(|x|*x<=p.limit),"trial cap exceeds authorized pool")?;}
        // A matched declared demand distribution is necessary, not sufficient
        // for causal equivalence of distinct tasks.
        let histogram=|keys:&BTreeSet<Id>|->Result<BTreeMap<String,usize>>{
            let mut h=BTreeMap::new();
            for k in keys{
                let d=self.demands.get(k).ok_or("trial demand missing")?;
                ensure(!d.prefer_native&&d.deadline_ms.is_none(),"trial requires explicitly routed demands without individual deadlines")?;
                ensure(!self.reservations.values().any(|r|r.demand==*k),"trial demand already attempted")?;
                let key=signature(d)?;*h.entry(key).or_default()+=1;
            }Ok(h)
        };
        ensure(histogram(&s.baseline_demands)?==histogram(&s.candidate_demands)?,"unmatched demand distributions")?;
        let mut shadow=self.clone();shadow.set_batch_policy(s.candidate.clone())?;
        let end=self.clock_ms.checked_add(s.duration_ms).ok_or("trial deadline overflow")?;
        let baseline=self.batch_policy.clone();let key=s.id.clone();
        let initial_available=self.trial_availability()?;
        let arm=Arm{start_ms:self.clock_ms,end_ms:end,policy:baseline.clone(),demands:s.baseline_demands.clone(),reservation_ids:BTreeSet::new(),initial_available};
        let t=Trial{spec:s,config_digest:self.config_fingerprint()?,baseline,phase:Phase::Baseline,arm:Some(arm),baseline_report:None,candidate_report:None,recommendation:None,promoted:false,reason:None};
        self.evolution.trials.insert(key.clone(),t);self.evolution.current=Some(key);Ok(())
    }
    fn trial_availability(&self)->Result<BTreeMap<Id,u64>>{
        self.config.as_ref().ok_or("router not configured")?.pools.keys().map(|k|Ok((k.clone(),self.available(k)?))).collect()
    }
    pub fn trial_auto_action(&self)->Option<super::transition::Action>{
        let t=self.trial()?;if !t.spec.auto_advance{return None;}
        if t.phase==Phase::Between{return if self.candidate_capacity_ready(t).unwrap_or(false){Some(super::transition::Action::StartRoutingCandidate)}else{None};}
        let a=t.arm.as_ref()?;
        if self.clock_ms>=a.end_ms&&a.reservation_ids.iter().all(|k|self.reservations.get(k).is_some_and(|r|!outstanding(r))){Some(super::transition::Action::CloseRoutingArm)}else{None}
    }
    pub fn trial_allows(&self,demand:&str)->bool{
        self.trial().is_none_or(|t|t.arm.as_ref().is_some_and(|a|a.demands.contains(demand)&&self.clock_ms<a.end_ms))
    }
    pub fn trial_budget(&self,pool:&str,charge:u64,existing:Option<&str>)->Result<()>{
        let Some(t)=self.trial() else{return Ok(());};
        let a=t.arm.as_ref().ok_or("trial between arms")?;
        let cap=*t.spec.caps.get(pool).ok_or("trial pool not authorized")?;
        let mut used=0u128;
        for key in &a.reservation_ids{
            if existing==Some(key.as_str()){continue;}
            let Some(r)=self.reservations.get(key) else{return Err("trial reservation missing".into());};
            used+=u128::from(if r.status==Status::Settled{*r.actual.get(pool).unwrap_or(&0)}else if matches!(r.status,Status::Reserved|Status::Dispatched|Status::Unknown){*r.holds.get(pool).unwrap_or(&0)}else{0});
        }
        ensure(used+u128::from(charge)<=u128::from(cap),"trial arm cap exceeded")
    }
    pub fn enroll_reservation(&mut self,key:&str,demand:&str)->Result<()>{
        if !self.trial_active(){return Ok(());}
        ensure(self.trial_allows(demand),"demand outside active trial arm")?;
        let t=self.evolution.trials.get_mut(self.evolution.current.as_ref().unwrap()).unwrap();
        t.arm.as_mut().unwrap().reservation_ids.insert(key.into());Ok(())
    }
    fn report_arm(&self,t:&Trial,a:&Arm)->Result<ArmReport>{
        ensure(self.clock_ms>=a.end_ms,"trial observation window still open")?;
        let mut successes=BTreeSet::new();let mut attempted=BTreeSet::new();let mut evidence=BTreeSet::new();let mut spent=BTreeMap::<Id,u64>::new();let mut requests=0usize;
        for key in &a.reservation_ids{
            let r=self.reservations.get(key).ok_or("trial reservation missing")?;
            ensure(!outstanding(r),"unreconciled or unverified trial attempt")?;
            ensure(a.demands.contains(&r.demand)&&r.batch_policy_version==Some(a.policy.version),"trial membership or policy attribution corrupted")?;
            if r.status==Status::Settled{
                requests+=1;attempted.insert(r.demand.clone());
                for (pool,charge) in &r.actual{let n=spent.entry(pool.clone()).or_default();*n=n.checked_add(*charge).ok_or("trial charge overflow")?;}
                if r.verified==Some(true)&&r.verified_ms.is_some_and(|ms|ms<=a.end_ms){
                    let e=r.evidence.as_ref().ok_or("trial evidence missing")?;
                    ensure(evidence.insert(e.clone()),"duplicate evidence")?;successes.insert(r.demand.clone());
                }
            }
        }
        let overrun=spent.iter().any(|(k,n)|t.spec.caps.get(k).is_none_or(|cap|n>cap));
        Ok(ArmReport{enrolled:a.demands.len(),successes:successes.len(),attempted:attempted.len(),settled_attempts:requests,arm:a.clone(),success_bps:((successes.len()*10_000)/a.demands.len()) as u16,spent,overrun,evidence})
    }
    pub fn close_trial_arm(&mut self)->Result<Value>{
        let key=self.evolution.current.clone().ok_or("no active trial")?;
        let mut t=self.evolution.trials[&key].clone();
        ensure(self.config_fingerprint()?==t.config_digest,"trial config changed")?;
        let a=t.arm.take().ok_or("no active arm")?;
        let report=self.report_arm(&t,&a)?;
        match t.phase{
            Phase::Baseline=>{t.baseline_report=Some(report.clone());t.phase=Phase::Between;},
            Phase::Candidate=>{
                let b=t.baseline_report.as_ref().ok_or("missing baseline report")?;
                let old=*b.spent.get(&t.spec.objective_pool).unwrap_or(&0);
                let new=*report.spent.get(&t.spec.objective_pool).unwrap_or(&0);
                // Require noninferior verified yield and a material cost reduction.
                // Do not combine unlike quota units. No increase in other pool use.
                let savings=old>0&&u128::from(new)*10_000<=u128::from(old)*u128::from(10_000-t.spec.min_savings_bps);
                let nonincrease=report.spent.iter().all(|(p,n)|*n<=*b.spent.get(p).unwrap_or(&0));
                let comparable_capacity=t.spec.caps.iter().all(|(k,cap)|b.arm.initial_available.get(k).copied().unwrap_or(0).min(*cap)==report.arm.initial_available.get(k).copied().unwrap_or(0).min(*cap));
                let good=comparable_capacity&&!b.overrun&&!report.overrun&&b.success_bps>=t.spec.min_success_bps&&report.success_bps>=b.success_bps&&savings&&nonincrease;
                t.recommendation=Some(good);t.candidate_report=Some(report.clone());t.phase=Phase::Evaluated;
                t.reason=Some(if good{"matched observed yield and resource guard passed"}else if !comparable_capacity{"initial available quota differed between arms"}else{"promotion guard failed"}.into());
                self.batch_policy=t.baseline.clone();
                if good&&t.spec.auto_promote{
                    self.batch_policy=t.spec.candidate.clone();t.promoted=true;
                    self.evolution.promotions.insert(key.clone(),Promotion{trial:key.clone(),previous:t.baseline.clone(),installed:self.batch_policy.clone(),clock_ms:self.clock_ms,rollback_ms:None});
                }
                self.evolution.current=None;
            }
            _=>return Err("trial phase mismatch".into()),
        }
        self.evolution.trials.insert(key.clone(),t.clone());Ok(json!({"trial":key,"phase":t.phase,"report":report,"recommendation":t.recommendation,"promoted":t.promoted}))
    }
    fn candidate_capacity_ready(&self,t:&Trial)->Result<bool>{
        let baseline=t.baseline_report.as_ref().ok_or("baseline missing")?;
        let available=self.trial_availability()?;
        Ok(t.spec.caps.iter().all(|(k,cap)|baseline.arm.initial_available.get(k).copied().unwrap_or(0).min(*cap)==available.get(k).copied().unwrap_or(0).min(*cap)))
    }
    pub fn start_candidate_arm(&mut self)->Result<()>{
        let key=self.evolution.current.clone().ok_or("no active trial")?;
        let mut t=self.evolution.trials[&key].clone();
        ensure(t.phase==Phase::Between&&self.config_fingerprint()?==t.config_digest&&!self.reservations.values().any(outstanding),"candidate arm not ready")?;
        ensure(self.candidate_capacity_ready(&t)?,"wait for comparable starting quota or abort trial")?;
        let end=self.clock_ms.checked_add(t.spec.duration_ms).ok_or("trial deadline overflow")?;
        self.batch_policy=t.spec.candidate.clone();t.phase=Phase::Candidate;
        t.arm=Some(Arm{start_ms:self.clock_ms,end_ms:end,policy:self.batch_policy.clone(),demands:t.spec.candidate_demands.clone(),reservation_ids:BTreeSet::new(),initial_available:self.trial_availability()?});
        self.evolution.trials.insert(key,t);Ok(())
    }
    pub fn abort_trial(&mut self,reason:String)->Result<()>{
        ensure(!reason.trim().is_empty(),"abort reason required")?;
        let key=self.evolution.current.take().ok_or("no active trial")?;
        let t=self.evolution.trials.get_mut(&key).unwrap();
        self.batch_policy=t.baseline.clone();t.phase=Phase::Aborted;t.reason=Some(reason);
        let ids=t.arm.as_ref().map(|a|a.reservation_ids.clone()).unwrap_or_default();
        for key in ids{
            match self.reservations.get(&key).map(|r|r.status.clone()){
                Some(Status::Reserved)=>self.release_unsent(&key)?,
                Some(Status::Dispatched)=>self.unknown(&key)?,
                _=>{}
            }
        }
        // Dispatched/unknown charges remain held pending reconciliation.
        Ok(())
    }
    pub fn promote_trial(&mut self,key:&str)->Result<()>{
        ensure(!self.trial_active()&&!self.reservations.values().any(outstanding),"drain before promotion")?;
        let t=self.evolution.trials.get(key).ok_or("trial missing")?.clone();
        ensure(t.phase==Phase::Evaluated&&t.recommendation==Some(true)&&!t.promoted&&self.config_fingerprint()?==t.config_digest&&digest(&serde_json::to_value(&self.batch_policy).map_err(|e|e.to_string())?)==digest(&serde_json::to_value(&t.baseline).map_err(|e|e.to_string())?),"trial cannot promote against changed baseline")?;
        self.set_batch_policy(t.spec.candidate.clone())?;
        self.evolution.promotions.insert(key.into(),Promotion{trial:key.into(),previous:t.baseline,installed:self.batch_policy.clone(),clock_ms:self.clock_ms,rollback_ms:None});
        self.evolution.trials.get_mut(key).unwrap().promoted=true;Ok(())
    }
    pub fn rollback_trial(&mut self,key:&str)->Result<()>{
        ensure(!self.trial_active()&&!self.reservations.values().any(outstanding),"drain before rollback")?;
        let p=self.evolution.promotions.get(key).ok_or("promotion missing")?.clone();
        ensure(p.rollback_ms.is_none()&&self.batch_policy.version==p.installed.version,"promotion no longer current")?;
        let mut prior=p.previous;prior.version=self.batch_policy.version.checked_add(1).ok_or("policy version overflow")?;
        self.set_batch_policy(prior)?;self.evolution.promotions.get_mut(key).unwrap().rollback_ms=Some(self.clock_ms);Ok(())
    }
}
