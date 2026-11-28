//! Demand-aware frontier protection. Forecasts are explicit, versioned and
//! expressed in the same units as the quota pool; never mix tokens and money.
use super::{model::{Id,ensure,id,World,TaskPhase,GoalPhase},routing::{Router,Demand,Window,Usage,Status}};
use crate::Result;
use serde::{Serialize,Deserialize};
use std::collections::BTreeMap;
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Forecast {
    pub id:Id,pub pool:Id,pub version:u64,pub window_start_ms:u64,
    pub expected_critical_units:u64,pub confidence_bps:u16,
    pub expires_ms:u64,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct FrontierGuard {
    pub version:u64,pub max_forecast_bps:u16,pub opportunity_weight:u32,
    pub protect_pending_critical:bool,
}
#[derive(Clone,Debug,Serialize,Deserialize,Default)]
pub struct Planning {
    pub guard:Option<FrontierGuard>,pub forecasts:BTreeMap<Id,Forecast>,
    #[serde(default)] pub pending_critical:BTreeMap<Id,u64>,
}
impl Router {
    pub fn set_frontier_guard(&mut self,g:FrontierGuard)->Result<()>{
        ensure(self.config.is_some(),"router not configured")?;
        ensure(g.version>self.planning.guard.as_ref().map_or(0,|x|x.version)&&g.max_forecast_bps<=10_000&&g.opportunity_weight<=100_000,"invalid frontier guard")?;
        self.planning.guard=Some(g);Ok(())
    }
    pub fn set_forecast(&mut self,f:Forecast)->Result<()>{
        id(&f.id)?;let c=self.config.as_ref().ok_or("router not configured")?;
        let p=c.pools.get(&f.pool).ok_or("forecast pool missing")?;
        ensure(p.frontier.is_some()&&matches!(p.window,Window::Fixed{..}),"forecast requires fixed frontier pool")?;
        let (start,end)=p.window.bounds(self.clock_ms)?;
        ensure(f.version>self.planning.forecasts.get(&f.pool).map_or(0,|x|x.version)&&f.confidence_bps<=10_000&&f.window_start_ms==start&&f.expires_ms>self.clock_ms&&f.expires_ms<=end.ok_or("fixed window end missing")?&&f.expected_critical_units<=p.limit,"invalid forecast")?;
        self.planning.forecasts.insert(f.pool.clone(),f);Ok(())
    }
    pub fn forecast_protection(&self,pool:&str)->Result<u64>{
        let c=self.config.as_ref().ok_or("router not configured")?;
        let p=c.pools.get(pool).ok_or("pool missing")?;
        let Some(g)=&self.planning.guard else{return Ok(0);};
        if p.frontier.is_none(){return Ok(0);}
        let (start,end)=p.window.bounds(self.clock_ms)?;
        let end=end.ok_or("frontier fixed window required")?;
        let weighted=self.planning.forecasts.get(pool).filter(|f|f.expires_ms>self.clock_ms&&f.window_start_ms==start)
            .map(|f|u128::from(f.expected_critical_units)*u128::from(f.confidence_bps)/10_000).unwrap_or(0);
        // Empirical same-time-of-day future usage across up to seven previous windows.
        // Sparse history yields a conservative zero estimate, not invented demand.
        let period=end-start;let offset=self.clock_ms-start;
        let mut samples=vec![];
        for i in 1..=7u64{
            let Some(old_start)=start.checked_sub(period.saturating_mul(i)) else{break;};
            let old_end=old_start.saturating_add(period);
            let mut use_=0u64;let mut observed=false;
            for r in self.reservations.values(){
                if r.ended_ms.is_some_and(|t|t>=old_start&&t<old_end)&&r.actual.contains_key(pool){
                    observed=true;
                    if r.ended_ms.is_some_and(|t|t>=old_start.saturating_add(offset))&&self.demands.get(&r.demand).is_some_and(|d|d.priority>=p.frontier.as_ref().unwrap().critical_priority){
                        use_=use_.saturating_add(r.actual[pool]);
                    }
                }
            }
            if observed{samples.push(use_);}
        }
        samples.sort_unstable();
        let historical=if samples.len()>=3{u128::from(samples[(samples.len()-1)*3/4])}else{0};
        let pending=if g.protect_pending_critical{u128::from(self.planning.pending_critical.get(pool).copied().unwrap_or(0))}else{0};
        let cap=u128::from(p.limit)*u128::from(g.max_forecast_bps)/10_000;
        Ok(weighted.max(historical).max(pending).min(cap) as u64)
    }
    pub fn forecast_gate(&self,d:&Demand,pool:&str,charge:u64)->Result<()>{
        let c=self.config.as_ref().ok_or("router not configured")?;
        let p=&c.pools[pool];let Some(f)=&p.frontier else{return Ok(());};
        if d.priority>=f.critical_priority{return Ok(());}
        let protect=self.forecast_protection(pool)?;
        ensure(charge<=self.available(pool)?.saturating_sub(protect),"future critical demand protection")
    }
    pub fn opportunity_penalty(&self,d:&Demand,pool:&str,charge:u64)->Result<i128>{
        let c=self.config.as_ref().ok_or("router not configured")?;
        let Some(g)=&self.planning.guard else{return Ok(0);};
        let Some(f)=&c.pools[pool].frontier else{return Ok(0);};
        if d.priority>=f.critical_priority{return Ok(0);}
        let avail=self.available(pool)?.max(1);
        let protect=self.forecast_protection(pool)?;
        Ok((i128::from(charge)*10_000/i128::from(avail))*i128::from(protect)*i128::from(g.opportunity_weight)/10_000)
    }
}

impl World{
    pub fn refresh_frontier_demand(&mut self)->Result<()>{
        let Some(c)=&self.router.config else{return Ok(());};
        let mut pending=BTreeMap::<Id,u64>::new();
        for d in self.router.demands.values(){
            if !self.tasks.get(&d.task).is_some_and(|t|t.phase==TaskPhase::Pending)||!self.goals.get(&d.goal).is_some_and(|g|g.phase==GoalPhase::Open){continue;}
            if self.router.reservations.values().any(|r|r.demand==d.id&&matches!(r.status,Status::Reserved|Status::Dispatched|Status::Unknown)){continue;}
            if d.prefer_native&&!d.independent_review&&self.match_class(&self.tasks[&d.task].spec).is_some(){continue;}
            for (key,pool) in &c.pools{
                let Some(f)=&pool.frontier else{continue;};
                if !d.allow_frontier||d.priority<f.critical_priority{continue;}
                let mut best=None;
                for p in c.profiles.values().filter(|p|p.enabled&&p.frontier&&p.pools.contains(key)&&d.capabilities.is_subset(&p.capabilities)){
                    if (!d.allowed_profiles.is_empty()&&!d.allowed_profiles.contains(&p.id))||(!d.allowed_providers.is_empty()&&!d.allowed_providers.contains(&p.provider))||d.excluded_providers.contains(&p.provider){continue;}
                    let u=d.usage_for(p);
                    if u.input.saturating_add(u.output).saturating_add(u.reasoning)>p.max_context||u.output>p.max_output||u.reasoning>p.max_reasoning||u.reasoning<d.minimum_reasoning_tokens{continue;}
                    let amount=super::routing::amount(pool,p,&u)?;
                    best=Some(best.map_or(amount,|old:u64|old.min(amount)));
                }
                if let Some(n)=best{let x=pending.entry(key.clone()).or_default();*x=x.saturating_add(n);}
            }
        }
        self.router.planning.pending_critical=pending;Ok(())
    }
}
