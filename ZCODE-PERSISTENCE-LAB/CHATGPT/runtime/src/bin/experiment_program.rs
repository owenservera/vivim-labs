//! Programmable paired experiments for the persistent runtime. Pure plan and
//! score operations; actual execution belongs to incumbent native qualification,
//! routing trials and Host adapters. A supplied observation is not authenticated.
use persistent_intelligence_runtime::digest;
use serde::{Deserialize,Serialize};
use serde_json::{json,Value};
use std::{collections::{BTreeMap,BTreeSet},fs,path::Path};
type R<T>=Result<T,String>;
fn check(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn valid_id(s:&str)->bool{!s.is_empty()&&s.len()<=96&&s.bytes().all(|b|b.is_ascii_alphanumeric()||b"-_.".contains(&b))}
fn hex(s:&str,n:usize)->bool{s.len()==n&&s.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b))}
fn pin<T:Serialize>(x:&T)->R<String>{Ok(digest(&serde_json::to_value(x).map_err(|e|e.to_string())?))}
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="snake_case")]
enum Dimension{Routing,Scheduling,Context,AgentClass,Reuse,Recovery,Storage,Transport,Isolation}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct Budget{wall_ms:u64,requests:u64,tokens:u64,active_slots:u32,logical_agents:u32}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct Variant{id:String,policy_digest:String}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct Fault{id:String,kind:String,after_verified:u64}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct Program{
    id:String,version:u32,source_pin:String,workload_digest:String,oracle_digest:String,
    dimension:Dimension,baseline:Variant,challenger:Variant,seed:u64,repeats:u32,
    budget:Budget,faults:Vec<Fault>,min_verified:u64,min_savings_bps:u32,
    max_recovery_ms:u64,max_human_interventions:u64,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Run{id:String,pair:u32,order:u32,arm:String,seed:u64,policy_digest:String}
fn next(x:&mut u64)->u64{
    *x=x.wrapping_add(0x9e3779b97f4a7c15);let mut z=*x;
    z=(z^(z>>30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z=(z^(z>>27)).wrapping_mul(0x94d049bb133111eb);z^(z>>31)
}
impl Program{
    fn validate(&self)->R<()>{
        check(valid_id(&self.id)&&self.version>0&&hex(&self.source_pin,40)&&hex(&self.workload_digest,64)&&hex(&self.oracle_digest,64),"invalid pinned experiment")?;
        check(valid_id(&self.baseline.id)&&valid_id(&self.challenger.id)&&self.baseline.id!=self.challenger.id&&hex(&self.baseline.policy_digest,64)&&hex(&self.challenger.policy_digest,64)&&self.baseline.policy_digest!=self.challenger.policy_digest,"distinct pinned policies required")?;
        let b=&self.budget;
        check((2..=64).contains(&self.repeats)&&b.wall_ms>0&&b.wall_ms<=604_800_000&&b.requests<=100_000&&b.tokens<=1_000_000_000&&(1..=10).contains(&b.active_slots)&&b.logical_agents>=b.active_slots&&b.logical_agents<=1024,"invalid bounded trial")?;
        check(self.min_verified>0&&self.min_savings_bps<=10_000&&self.max_recovery_ms>0&&self.faults.len()<=128,"invalid oracle thresholds")?;
        let mut ids=BTreeSet::new();
        for f in &self.faults{
            check(valid_id(&f.id)&&ids.insert(&f.id)&&matches!(f.kind.as_str(),"worker_exit"|"controller_exit"|"transport_disconnect"|"duplicate_event"|"stale_lease"|"malformed_output"|"quota_exhaustion"|"adapter_timeout"|"storage_fault"|"context_change"|"late_receipt"|"denied_effect"),"unknown or duplicate fault")?;
        }
        check(self.faults.windows(2).all(|v|v[0].after_verified<=v[1].after_verified),"faults must be ordered")?;
        Ok(())
    }
    fn runs(&self)->R<Vec<Run>>{
        self.validate()?;let mut rng=self.seed;let mut out=vec![];
        for pair in 0..self.repeats{
            let seed=next(&mut rng);let reverse=next(&mut rng)&1==1;
            let order=if reverse{[("challenger",&self.challenger),("baseline",&self.baseline)]}else{[("baseline",&self.baseline),("challenger",&self.challenger)]};
            for (i,(arm,v)) in order.into_iter().enumerate(){out.push(Run{id:format!("{}-{pair}-{arm}",self.id),pair,order:i as u32,arm:arm.into(),seed,policy_digest:v.policy_digest.clone()});}
        }Ok(out)
    }
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct Observation{
    run_id:String,program_digest:String,source_pin:String,policy_digest:String,workload_digest:String,oracle_digest:String,
    evidence_digest:String,verified:u64,requests:Option<u64>,tokens:Option<u64>,wall_ms:Option<u64>,
    lost_artifacts:Option<u64>,duplicate_effects:Option<u64>,unauthorized_effects:Option<u64>,
    unresolved_effects:Option<u64>,human_interventions:Option<u64>,max_recovery_ms:Option<u64>,
    recovered_fault_ids:BTreeSet<String>,observed_fault_ids:BTreeSet<String>,
}
fn evaluate(p:&Program,observations:&[Observation])->R<Value>{
    let runs=p.runs()?;let ppin=pin(p)?;let mut by_id=BTreeMap::new();let mut evidence=BTreeSet::new();
    for o in observations{
        check(by_id.insert(o.run_id.as_str(),o).is_none(),"duplicate run observation")?;
        check(hex(&o.evidence_digest,64)&&evidence.insert(&o.evidence_digest),"invalid or reused evidence digest")?;
    }
    check(observations.len()<=runs.len()&&by_id.keys().all(|id|runs.iter().any(|r|r.id.as_str()==*id)),"unexpected run observations")?;
    let expected_faults:BTreeSet<_>=p.faults.iter().map(|f|f.id.clone()).collect();
    let mut failures=vec![];let mut unknown=vec![];let mut pairs=vec![];let mut complete:BTreeMap<(u32,String),&Observation>=BTreeMap::new();
    for r in &runs{
        let Some(o)=by_id.get(r.id.as_str()).copied()else{unknown.push(format!("{}:missing observation",r.id));continue;};
        check(o.program_digest==ppin&&o.source_pin==p.source_pin&&o.policy_digest==r.policy_digest&&o.workload_digest==p.workload_digest&&o.oracle_digest==p.oracle_digest,"observation identity mismatch")?;
        let mut bad=vec![];let mut missing=vec![];
        if o.verified<p.min_verified{bad.push("verified_yield");}
        for (name,val,max) in [
            ("requests",o.requests,p.budget.requests),("tokens",o.tokens,p.budget.tokens),("wall_ms",o.wall_ms,p.budget.wall_ms),
            ("lost_artifacts",o.lost_artifacts,0),("duplicate_effects",o.duplicate_effects,0),("unauthorized_effects",o.unauthorized_effects,0),
            ("unresolved_effects",o.unresolved_effects,0),("human_interventions",o.human_interventions,p.max_human_interventions)]{
            match val{Some(v) if v>max=>bad.push(name),None=>missing.push(name),_=>{}}
        }
        if !expected_faults.is_empty(){
            if o.observed_fault_ids!=expected_faults||o.recovered_fault_ids!=expected_faults{bad.push("fault_coverage");}
            match o.max_recovery_ms{Some(v) if v>p.max_recovery_ms=>bad.push("recovery_latency"),None=>missing.push("recovery_latency"),_=>{}}
        }
        for n in bad{failures.push(format!("{}:{n}",r.id));}
        for n in missing{unknown.push(format!("{}:{n}",r.id));}
        complete.insert((r.pair,r.arm.clone()),o);
    }
    for pair in 0..p.repeats{
        let (Some(a),Some(b))=(complete.get(&(pair,"baseline".into())),complete.get(&(pair,"challenger".into())))else{continue;};
        let (Some(ar),Some(br),Some(at),Some(bt),Some(aw),Some(bw))=(a.requests,b.requests,a.tokens,b.tokens,a.wall_ms,b.wall_ms)else{continue;};
        // Separate units. Default objective: requests for inference trials,
        // elapsed time for native-only trials. Other resources must not rise.
        let (base,challenger,objective)=if p.budget.requests==0{(aw,bw,"wall_ms")}else{(ar,br,"requests")};
        let savings=if base==0{None}else{Some(((base as i128-challenger as i128)*10_000)/(base as i128))};
        let nonregression=b.verified>=a.verified&&br<=ar&&bt<=at&&bw<=aw;
        let met=nonregression&&savings.is_some_and(|v|v>=p.min_savings_bps as i128);
        if !met{failures.push(format!("pair-{pair}:no_verified_improvement"));}
        pairs.push(json!({"pair":pair,"objective":objective,"baseline":base,"challenger":challenger,"savings_bps":savings,"nonregression":nonregression,"met":met}));
    }
    let status=if !failures.is_empty(){"not_met"}else if !unknown.is_empty()||pairs.len()!=p.repeats as usize{"unknown"}else{"criteria_met_unattested"};
    Ok(json!({"schema":"vivim.experiment.evaluation/1","program_digest":ppin,"status":status,"pairs":pairs,"failures":failures,"unknown":unknown,
        "promotion_authorized":false,"acceptance_credit":false,"note":"supplied observations require independently verified custody and actual receipts; no automatic policy promotion"}))
}
fn read<T:serde::de::DeserializeOwned>(p:&str)->R<T>{
    let meta=fs::metadata(Path::new(p)).map_err(|e|e.to_string())?;check(meta.is_file()&&meta.len()<=8*1024*1024,"invalid input size")?;
    serde_json::from_slice(&fs::read(p).map_err(|e|e.to_string())?).map_err(|e|e.to_string())
}
fn run()->R<Value>{
    let a:Vec<_>=std::env::args().skip(1).collect();
    match a.as_slice(){
        [op,file] if op=="compile"=>{let p:Program=read(file)?;let runs=p.runs()?;Ok(json!({"schema":"vivim.experiment.plan/1","program_digest":pin(&p)?,"program":p,"runs":runs,"execution_authority":"none"}))},
        [op,file,observations] if op=="evaluate"=>evaluate(&read(file)?,&read::<Vec<Observation>>(observations)?),
        _=>Err("usage: experiment_program compile <program.json> | evaluate <program.json> <observations.json>".into())
    }
}
fn main(){match run(){Ok(v)=>println!("{}",serde_json::to_string_pretty(&v).unwrap()),Err(e)=>{eprintln!("{e}");std::process::exit(1);}}}
#[cfg(test)]
mod tests{
    use super::*;
    fn program()->Program{Program{id:"trial".into(),version:1,source_pin:"a".repeat(40),workload_digest:"b".repeat(64),oracle_digest:"c".repeat(64),dimension:Dimension::Reuse,
        baseline:Variant{id:"a".into(),policy_digest:"d".repeat(64)},challenger:Variant{id:"b".into(),policy_digest:"e".repeat(64)},seed:42,repeats:3,budget:Budget{wall_ms:1000,requests:10,tokens:100,active_slots:1,logical_agents:1},faults:vec![],min_verified:1,min_savings_bps:1000,max_recovery_ms:100,max_human_interventions:0}}
    #[test]fn deterministic_matched_pairs(){let p=program();let r=p.runs().unwrap();assert_eq!(r.len(),6);assert_eq!(pin(&r).unwrap(),pin(&p.runs().unwrap()).unwrap());for pair in r.chunks(2){assert_eq!(pair[0].seed,pair[1].seed);assert_ne!(pair[0].arm,pair[1].arm);}}
    #[test]fn absent_evidence_stays_unknown(){let v=evaluate(&program(),&[]).unwrap();assert_eq!(v["status"],"unknown");assert_eq!(v["promotion_authorized"],false);}
    #[test]fn no_vacuous_or_unbounded_runs(){let mut p=program();p.budget.active_slots=11;assert!(p.runs().is_err());p.budget.active_slots=1;p.repeats=0;assert!(p.runs().is_err());}
    #[test]fn reject_undeclared_fault(){let mut p=program();p.faults.push(Fault{id:"fault".into(),kind:"arbitrary_shell".into(),after_verified:1});assert!(p.runs().is_err());}
}
