//! Dedicated frontier allocator: protect emergency capacity, pace scarce
//! quota, rank known demand by expected incremental value rather than FIFO.
use super::routing::{Demand,Decision,Recommendation,Router,Pool,FrontierPolicy};
use super::model::{Id,ensure};
use crate::Result;
use serde::{Serialize,Deserialize};
use std::collections::BTreeSet;
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct RankedDemand{
    pub demand:Id,pub priority:u16,pub frontier_profile:Option<Id>,
    pub incremental_success_bps:u16,pub urgency_bps:u16,pub score:i128,
}
pub fn paced_allowance(limit:u64,policy:&FrontierPolicy,elapsed:u64,period:u64)->Result<u64>{
    ensure(period>0,"invalid frontier window")?;
    let released=u128::from(policy.opening_bps)+(10_000-u128::from(policy.opening_bps))*u128::from(elapsed.min(period))/u128::from(period);
    let emergency=u128::from(limit)*u128::from(policy.emergency_bps)/10_000;
    Ok(((u128::from(limit)-emergency)*released/10_000) as u64)
}
pub fn rank(router:&Router,allowed:&BTreeSet<Id>)->Result<Vec<RankedDemand>>{
    let c=router.config.as_ref().ok_or("routing not configured")?;
    let mut out=Vec::new();
    for (key,d) in &router.demands{
        if !allowed.contains(key){continue;}
        let rec=router.recommend(key)?;
        let Some(best)=rec.chosen.as_ref() else{continue;};
        let frontier=rec.eligible.iter().filter(|x|c.profiles[&x.profile].frontier).max_by_key(|x|x.score);
        let urgency=d.deadline_ms.map(|t|{
            let remaining=t.saturating_sub(router.clock_ms);
            (10_000u128* u128::from(best.expected_latency_ms.max(1))/u128::from(remaining.max(1))).min(10_000) as u16
        }).unwrap_or(0);
        let uplift=frontier.map(|x|x.uplift_bps).unwrap_or(0);
        // Rank hard-critical work first, then estimated marginal benefit,
        // uncertainty/novelty and deadline urgency. Admission rechecks every
        // overlapping pool after each reservation.
        let critical=frontier.is_some_and(|x|x.holds.keys().any(|p|c.pools[p].frontier.as_ref().is_some_and(|f|d.priority>=f.critical_priority)));
        let score=if critical{1_000_000_000i128}else{0}
            +i128::from(d.priority)*100_000+i128::from(uplift)*1000
            +i128::from(d.information_gain_bps)*10+i128::from(d.uncertainty_bps)*10+i128::from(urgency)*100;
        out.push(RankedDemand{demand:key.clone(),priority:d.priority,frontier_profile:frontier.map(|x|x.profile.clone()),incremental_success_bps:uplift,urgency_bps:urgency,score});
    }
    out.sort_by(|a,b|b.score.cmp(&a.score).then_with(||a.demand.cmp(&b.demand)));Ok(out)
}
