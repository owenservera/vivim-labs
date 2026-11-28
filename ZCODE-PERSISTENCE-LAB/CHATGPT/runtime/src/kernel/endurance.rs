//! Configurable qualification; supplied observations are not authenticated evidence.
use crate::Result;
use serde::{Deserialize,Serialize};
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub version:u64,
    pub min_elapsed_ms:Option<u64>,
    pub min_verified_tasks:u64,
    pub min_fault_recoveries:u64,
    pub max_recovery_ms:Option<u64>,
    pub max_lost_artifacts:u64,
    pub max_duplicate_effects:u64,
    pub max_human_interventions:u64,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub elapsed_ms:Option<u64>,
    pub verified_tasks:Option<u64>,
    pub fault_recoveries:Option<u64>,
    pub slowest_recovery_ms:Option<u64>,
    pub lost_artifacts:Option<u64>,
    pub duplicate_effects:Option<u64>,
    pub human_interventions:Option<u64>,
}
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="snake_case")]
pub enum Status{Met,NotMet,Unknown}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Check{pub criterion:String,pub status:Status}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Evaluation{
    pub policy_digest:String,pub observation_digest:String,
    pub checks:Vec<Check>,pub status:Status,pub acceptance_credit:bool,
}
fn check(name:&str,actual:Option<u64>,target:u64,minimum:bool)->Check{
    let status=match actual{
        None=>Status::Unknown,
        Some(v) if (minimum&&v>=target)||(!minimum&&v<=target)=>Status::Met,
        Some(_)=>Status::NotMet,
    };
    Check{criterion:name.into(),status}
}
impl Policy{
    pub fn validate(&self)->Result<()>{
        if self.version==0||self.min_elapsed_ms==Some(0)||self.max_recovery_ms==Some(0)||
           (self.min_elapsed_ms.is_none()&&self.min_verified_tasks==0&&self.min_fault_recoveries==0)||
           (self.max_recovery_ms.is_some()&&self.min_fault_recoveries==0){
            return Err("invalid or vacuous endurance policy".into());
        }
        Ok(())
    }
    pub fn evaluate(&self,o:&Observation)->Result<Evaluation>{
        self.validate()?;
        let mut checks=vec![
            check("verified_tasks",o.verified_tasks,self.min_verified_tasks,true),
            check("fault_recoveries",o.fault_recoveries,self.min_fault_recoveries,true),
            check("lost_artifacts",o.lost_artifacts,self.max_lost_artifacts,false),
            check("duplicate_effects",o.duplicate_effects,self.max_duplicate_effects,false),
            check("human_interventions",o.human_interventions,self.max_human_interventions,false),
        ];
        if let Some(n)=self.min_elapsed_ms{checks.push(check("elapsed_ms",o.elapsed_ms,n,true));}
        if let Some(n)=self.max_recovery_ms{checks.push(check("slowest_recovery_ms",o.slowest_recovery_ms,n,false));}
        let status=if checks.iter().any(|c|c.status==Status::NotMet){Status::NotMet}
            else if checks.iter().any(|c|c.status==Status::Unknown){Status::Unknown}
            else{Status::Met};
        Ok(Evaluation{
            policy_digest:crate::digest(&serde_json::to_value(self).map_err(|e|e.to_string())?),
            observation_digest:crate::digest(&serde_json::to_value(o).map_err(|e|e.to_string())?),
            checks,status,acceptance_credit:false,
        })
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn programmable_qualification(){
        let mut p=Policy{version:1,min_elapsed_ms:None,min_verified_tasks:10,min_fault_recoveries:2,max_recovery_ms:Some(1000),max_lost_artifacts:0,max_duplicate_effects:0,max_human_interventions:0};
        let mut o=Observation{elapsed_ms:None,verified_tasks:Some(10),fault_recoveries:Some(2),slowest_recovery_ms:Some(500),lost_artifacts:Some(0),duplicate_effects:Some(0),human_interventions:Some(0)};
        assert_eq!(p.evaluate(&o).unwrap().status,Status::Met);
        assert!(!p.evaluate(&o).unwrap().acceptance_credit);
        p.min_elapsed_ms=Some(100);
        assert_eq!(p.evaluate(&o).unwrap().status,Status::Unknown);
        o.elapsed_ms=Some(99);
        assert_eq!(p.evaluate(&o).unwrap().status,Status::NotMet);
        o.elapsed_ms=Some(100);
        assert_eq!(p.evaluate(&o).unwrap().status,Status::Met);
        o.duplicate_effects=Some(1);
        assert_eq!(p.evaluate(&o).unwrap().status,Status::NotMet);
    }
}
