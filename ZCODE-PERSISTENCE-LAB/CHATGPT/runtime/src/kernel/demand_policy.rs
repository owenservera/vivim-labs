//! Host-programmable inference demand generation. No provider call, model
//! guessing or unbounded goal expansion. All ceilings are operator allocations,
//! not observed token counts. Existing demands remain immutable on policy edit.
use super::{model::*,routing::Demand};
use crate::{digest,Result};
use serde::{Deserialize,Serialize};
use serde_json::{json,Value};
use std::collections::{BTreeMap,BTreeSet};

#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rule{
    pub capabilities:BTreeSet<String>,
    pub allowed_profiles:BTreeSet<Id>,
    pub input_tokens:u64,
    pub output_tokens:u64,
    pub reasoning_tokens:u64,
    pub minimum_reasoning_tokens:u64,
    pub adaptive_thinking:bool,
    pub minimum_success_bps:u16,
    pub allow_frontier:bool,
    pub deadline_after_ms:Option<u64>,
    pub extra_pools:BTreeSet<Id>,
    pub max_input_bytes:usize,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy{
    pub version:u64,
    pub enabled:bool,
    pub goals:BTreeSet<Id>,
    pub families:BTreeMap<Id,Rule>,
    pub max_new_per_cycle:usize,
}
impl World{
    pub fn configure_demand_policy(&mut self,p:Policy)->Result<()>{
        ensure(!self.router.trial_active(),"demand policy frozen during trial")?;
        let c=self.router.config.as_ref().ok_or("router not configured")?;
        ensure(p.version>self.router.demand_policy.as_ref().map_or(0,|x|x.version)
            &&(1..=128).contains(&p.max_new_per_cycle)&&p.goals.len()<=4096&&p.families.len()<=4096,
            "invalid demand policy version or bounds")?;
        ensure(!p.enabled||(!p.goals.is_empty()&&!p.families.is_empty()),"enabled policy requires explicit goal and family scope")?;
        for g in &p.goals{id(g)?;ensure(self.goals.contains_key(g),"unknown policy goal")?;}
        for (family,r) in &p.families{
            id(family)?;
            ensure(!r.allowed_profiles.is_empty()&&!r.capabilities.is_empty()&&r.capabilities.len()<=128
                &&r.capabilities.iter().all(|x|!x.trim().is_empty()&&x.len()<=128)
                &&r.input_tokens>0&&r.output_tokens>0&&r.minimum_reasoning_tokens<=r.reasoning_tokens
                &&r.minimum_success_bps<=10_000&&r.deadline_after_ms!=Some(0)
                &&(1..=MAX_VALUE_BYTES).contains(&r.max_input_bytes),"invalid demand rule")?;
            for key in &r.allowed_profiles{
                let profile=c.profiles.get(key).ok_or("unknown demand rule profile")?;
                ensure(!profile.frontier||r.allow_frontier,"frontier not authorized by rule")?;
                ensure(r.capabilities.is_subset(&profile.capabilities),"profile lacks required rule capabilities")?;
            }
            for key in &r.extra_pools{
                ensure(c.pools.get(key).is_some_and(|p|p.frontier.is_none()),"unknown or protected extra pool")?;
            }
        }
        self.router.demand_policy=Some(p);Ok(())
    }
    fn planned_demands(&self,limit:usize)->Result<Vec<Demand>>{
        ensure((1..=128).contains(&limit),"invalid demand generation limit")?;
        let Some(p)=self.router.demand_policy.as_ref().filter(|p|p.enabled) else{return Ok(vec![]);};
        if self.router.trial_active(){return Ok(vec![]);}
        let mut out=vec![];
        let existing:BTreeSet<_>=self.router.demands.values().map(|d|d.task.as_str()).collect();
        for task in self.ready(){
            if out.len()>=limit.min(p.max_new_per_cycle){break;}
            let t=&self.tasks[&task];
            if existing.contains(task.as_str())||!p.goals.contains(&t.spec.goal)||self.match_class(&t.spec).is_some(){continue;}
            let Some(family)=&t.spec.family else{continue;};
            let Some(r)=p.families.get(family) else{continue;};
            let deadline=r.deadline_after_ms.map(|x|self.clock_ms.checked_add(x).ok_or("demand deadline overflow")).transpose()?;
            let mut d=Demand{
                id:format!("auto-{}",digest(&json!([&task,p.version]))),goal:t.spec.goal.clone(),task:task.clone(),family:family.clone(),
                capabilities:r.capabilities.clone(),input_tokens:r.input_tokens,output_tokens:r.output_tokens,reasoning_tokens:r.reasoning_tokens,
                adaptive_thinking:r.adaptive_thinking,minimum_reasoning_tokens:r.minimum_reasoning_tokens,
                priority:t.spec.priority.clamp(0,1000) as u16,uncertainty_bps:t.spec.uncertainty.min(10_000) as u16,
                information_gain_bps:t.spec.information_gain.min(10_000) as u16,minimum_success_bps:r.minimum_success_bps,
                deadline_ms:deadline,allowed_profiles:r.allowed_profiles.clone(),allowed_providers:BTreeSet::new(),excluded_providers:BTreeSet::new(),
                allow_frontier:r.allow_frontier,independent_review:false,review_evidence:None,prefer_native:true,extra_pools:r.extra_pools.clone(),
            };
            // Only profiles with authorized, compatible and byte-bounded actual
            // context enter the frozen demand. Never infer token usage from bytes.
            let compatible=self.bound_profiles(&d);
            let c=self.router.config.as_ref().ok_or("router not configured")?;
            d.allowed_profiles=compatible.into_iter().filter(|key|{
                let profile=&c.profiles[key];
                let Some(binding)=self.router.adapters.bindings.get(&profile.adapter) else{return false;};
                let Some(context)=self.contexts.get(&binding.context) else{return false;};
                self.assemble(&t.spec,context).ok().and_then(|v|serde_json::to_vec(&v).ok()).is_some_and(|v|v.len()<=r.max_input_bytes)
            }).collect();
            // No empty allowlist: empty means unrestricted in the router.
            if d.allowed_profiles.is_empty(){continue;}
            out.push(d);
        }
        Ok(out)
    }
    pub fn demand_generation_plan(&self,limit:usize)->Result<Value>{
        Ok(json!({"policy_version":self.router.demand_policy.as_ref().map(|p|p.version),"planned":self.planned_demands(limit)?,"authority":"proposal_only","provider_calls":0}))
    }
    pub fn demand_generation_needed(&self)->bool{
        self.planned_demands(1).is_ok_and(|v|!v.is_empty())
    }
    pub fn generate_demands(&mut self,limit:usize)->Result<Value>{
        let candidates=self.planned_demands(limit)?;
        let mut ids=vec![];
        for d in candidates{ids.push(d.id.clone());self.queue_inference(d)?;}
        Ok(json!({"created":ids,"provider_calls":0}))
    }
}
