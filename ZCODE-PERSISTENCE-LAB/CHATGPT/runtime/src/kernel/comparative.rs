//! Reproducible four-arm experiment manifest. Planning only: it cannot
//! establish that real external adapters enforce their persistence authority.
use super::model::{ensure,id,Id};
use crate::{digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::collections::BTreeSet;
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq,PartialOrd,Ord)]
#[serde(rename_all="snake_case")]
pub enum Arm{Session,Agent,Goal,Intelligence}
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq,PartialOrd,Ord)]
#[serde(rename_all="snake_case")]
pub enum Authority{NativeSession,AgentIdentity,AgentCheckpoint,Goal,TaskDag,Artifact,Evidence,AgentRegistry,Class,Operator,Capability,Repair}
impl Arm{
    pub fn authority(&self)->BTreeSet<Authority>{
        use Authority::*;
        match self{
            Arm::Session=>BTreeSet::from([NativeSession]),
            Arm::Agent=>BTreeSet::from([AgentIdentity,AgentCheckpoint]),
            Arm::Goal=>BTreeSet::from([Goal,TaskDag,Artifact,Evidence,AgentRegistry]),
            Arm::Intelligence=>BTreeSet::from([Goal,TaskDag,Artifact,Evidence,AgentRegistry,Class,Operator,Capability,Repair]),
        }
    }
    pub fn admits(&self,a:&Authority)->bool{self.authority().contains(a)}
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fault{pub id:Id,pub kind:Id,pub after_verified:u64}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spec{
    pub id:Id,pub version:u64,pub seed:u64,pub repeats:u32,pub workload_digest:String,
    pub max_wall_ms:u64,pub max_requests:u64,pub max_tokens:u64,pub max_concurrent_children:u32,
    pub faults:Vec<Fault>,
}
fn next(x:&mut u64)->u64{
    // SplitMix64: fixed algorithm for reproducible scheduling, not cryptography.
    *x=x.wrapping_add(0x9e3779b97f4a7c15);
    let mut z=*x;z=(z^(z>>30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z=(z^(z>>27)).wrapping_mul(0x94d049bb133111eb);z^(z>>31)
}
impl Spec{
    pub fn validate(&self)->Result<()>{
        id(&self.id)?;
        ensure(self.version>0&&(1..=128).contains(&self.repeats)&&super::model::hash_id(&self.workload_digest)&&self.max_wall_ms>0
            &&self.max_requests>0&&self.max_tokens>0&&(1..=4).contains(&self.max_concurrent_children)&&self.faults.len()<=256,"invalid comparative specification")?;
        let mut ids=BTreeSet::new();
        for f in &self.faults{id(&f.id)?;id(&f.kind)?;ensure(ids.insert(&f.id),"duplicate fault id")?;}
        ensure(self.faults.windows(2).all(|x|x[0].after_verified<=x[1].after_verified),"fault milestones out of order")?;
        Ok(())
    }
    pub fn plan(&self)->Result<Value>{
        self.validate()?;
        let pin=digest(&serde_json::to_value(self).map_err(|e|e.to_string())?);
        let mut rng=self.seed;let mut runs=vec![];
        for repeat in 0..self.repeats{
            let seed=next(&mut rng);
            let mut arms=[Arm::Session,Arm::Agent,Arm::Goal,Arm::Intelligence];
            for i in (1..arms.len()).rev(){let j=(next(&mut rng)%(i as u64+1)) as usize;arms.swap(i,j);}
            for (order,arm) in arms.into_iter().enumerate(){
                let run_id=format!("arm-{}",digest(&json!([pin,repeat,&arm])));
                runs.push(json!({"run_id":run_id,"repeat":repeat,"order":order,"seed":seed,"arm":arm,"authority":arm.authority(),
                    "workload_digest":self.workload_digest,"faults":self.faults,"max_wall_ms":self.max_wall_ms,"max_requests":self.max_requests,
                    "max_tokens":self.max_tokens,"max_concurrent_children":self.max_concurrent_children,"fresh_root_required":true,"cross_arm_learning":false}));
            }
        }
        Ok(json!({"spec_digest":pin,"runs":runs,"execution":"not_started","isolation":"requires_adapter_enforcement","acceptance_credit":false}))
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn matched_cohorts_and_authority(){
        let s=Spec{id:"comparison".into(),version:1,seed:5,repeats:3,workload_digest:digest(&json!(42)),max_wall_ms:100,max_requests:5,max_tokens:100,max_concurrent_children:1,faults:vec![]};
        let p=s.plan().unwrap();assert_eq!(p,s.plan().unwrap());assert_eq!(p["runs"].as_array().unwrap().len(),12);
        assert!(!Arm::Session.admits(&Authority::Goal));assert!(!Arm::Agent.admits(&Authority::Capability));
        assert!(!Arm::Goal.admits(&Authority::Repair));assert!(Arm::Intelligence.admits(&Authority::Repair));
        for group in p["runs"].as_array().unwrap().chunks(4){
            assert!(group.iter().all(|x|x["seed"]==group[0]["seed"]&&x["workload_digest"]==group[0]["workload_digest"]));
            let distinct:BTreeSet<_>=group.iter().map(|x|x["arm"].as_str().unwrap()).collect();assert_eq!(distinct.len(),4);
        }
    }
}
