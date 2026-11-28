//! Bounded joint task/profile assignment under the real reservation rules.
use super::{model::{Id,ensure},routing::Router};
use crate::{digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::collections::BTreeSet;
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct BatchPolicy{
    pub version:u64,pub beam_width:usize,pub demand_limit:usize,pub profiles_per_demand:usize,
    pub node_budget:usize,pub completion_weight:u32,pub priority_weight:u32,
}
impl Default for BatchPolicy{
    fn default()->Self{Self{version:0,beam_width:4,demand_limit:16,profiles_per_demand:3,node_budget:128,completion_weight:100,priority_weight:100}}
}
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq,PartialOrd,Ord)]
pub struct Assignment{pub demand:Id,pub profile:Id}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct BatchPlan{pub policy_version:u64,pub assignments:Vec<Assignment>,pub utility:i64,pub nodes:usize,pub truncated:bool,pub expected_success_bps_sum:u64,pub reserved_by_pool:std::collections::BTreeMap<Id,u64>}
struct Node{router:Router,assignments:Vec<Assignment>,utility:i128}
impl Router{
    pub fn set_batch_policy(&mut self,p:BatchPolicy)->Result<()>{
        ensure(self.config.is_some()&&p.version>self.batch_policy.version&&(1..=16).contains(&p.beam_width)&&(1..=64).contains(&p.demand_limit)&&(1..=8).contains(&p.profiles_per_demand)&&(1..=512).contains(&p.node_budget)&&p.completion_weight<=100_000&&p.priority_weight<=100_000,"invalid batch policy")?;
        self.batch_policy=p;Ok(())
    }
    pub fn compare_batch_policies(&self,allowed:&BTreeSet<Id>,limit:usize,policies:Vec<BatchPolicy>)->Result<Value>{
        ensure((1..=8).contains(&policies.len()),"invalid shadow policy count")?;
        let baseline=self.optimize_batch(allowed,limit)?;let mut alternatives=vec![];
        for policy in policies{let mut shadow=self.clone();shadow.set_batch_policy(policy)?;alternatives.push(shadow.optimize_batch(allowed,limit)?);}
        Ok(json!({"baseline":baseline,"alternatives":alternatives,"measurement":"estimated_snapshot_only","policy_changed":false}))
    }
    pub fn optimize_batch(&self,allowed:&BTreeSet<Id>,limit:usize)->Result<BatchPlan>{
        ensure((1..=128).contains(&limit),"invalid batch size")?;
        let p=&self.batch_policy;
        let ranked=super::frontier::rank(self,allowed)?;
        let mut truncated=ranked.len()>p.demand_limit;
        let candidates:Vec<_>=ranked.into_iter().take(p.demand_limit).map(|x|x.demand).collect();
        let mut beam=vec![Node{router:self.clone(),assignments:vec![],utility:0}];
        let mut nodes=0usize;
        for demand in candidates{
            let mut next=Vec::new();
            for node in beam{
                if node.assignments.len()<limit{
                    let rec=node.router.recommend(&demand)?;
                    if rec.eligible.len()>p.profiles_per_demand{truncated=true;}
                    for choice in rec.eligible.iter().take(p.profiles_per_demand){
                        if nodes>=p.node_budget{truncated=true;break;}
                        nodes+=1;
                        let mut trial=node.router.clone();
                        let key=format!("plan-{}",digest(&json!([&demand,&choice.profile,&node.assignments])));
                        if trial.reserve_profile(key,&demand,&choice.profile).is_err(){continue;}
                        let d=&trial.demands[&demand];
                        let utility=choice.score.saturating_add(i128::from(p.completion_weight)*i128::from(choice.expected_success_bps)).saturating_add(i128::from(p.priority_weight)*i128::from(d.priority));
                        let mut assignments=node.assignments.clone();assignments.push(Assignment{demand:demand.clone(),profile:choice.profile.clone()});
                        next.push(Node{router:trial,assignments,utility:node.utility.saturating_add(utility)});
                    }
                }
                next.push(node); // waiting is a legitimate alternative
            }
            next.sort_by(|a,b|b.utility.cmp(&a.utility).then_with(||b.assignments.len().cmp(&a.assignments.len())).then_with(||a.assignments.cmp(&b.assignments)));
            if next.len()>p.beam_width{truncated=true;}
            next.truncate(p.beam_width);beam=next;
            if nodes>=p.node_budget{break;}
        }
        let best=beam.into_iter().next().ok_or("empty search")?;
        let mut expected_success_bps_sum=0u64;let mut reserved_by_pool=std::collections::BTreeMap::<Id,u64>::new();
        for (key,r) in &best.router.reservations{if !self.reservations.contains_key(key){expected_success_bps_sum=expected_success_bps_sum.saturating_add(u64::from(r.estimated_success_bps));for (pool,charge) in &r.holds{let n=reserved_by_pool.entry(pool.clone()).or_default();*n=n.saturating_add(*charge);}}}
        Ok(BatchPlan{policy_version:p.version,assignments:best.assignments,utility:best.utility.clamp(i128::from(i64::MIN),i128::from(i64::MAX)) as i64,nodes,truncated,expected_success_bps_sum,reserved_by_pool})
    }
}
