//! Demand-aware frontier protection. Forecasts are explicit, versioned and
//! expressed in the same units as the quota pool; never mix tokens and money.
use super::{model::{Id,ensure,id},routing::{Router,Demand,Window}};
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
}
impl Router {
    pub fn set_frontier_guard(&mut self,g:FrontierGuard)->Result<()>{
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
        let Some(f)=self.planning.forecasts.get(pool) else{return Ok(0);};
        let (start,_)=p.window.bounds(self.clock_ms)?;
        if f.expires_ms<=self.clock_ms||f.window_start_ms!=start{return Ok(0);}
        let weighted=u128::from(f.expected_critical_units)*u128::from(f.confidence_bps)/10_000;
        let cap=u128::from(p.limit)*u128::from(g.max_forecast_bps)/10_000;
        Ok(weighted.min(cap) as u64)
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
        Ok(i128::from(charge)*i128::from(protect)*i128::from(g.opportunity_weight)/i128::from(avail))
    }
}
