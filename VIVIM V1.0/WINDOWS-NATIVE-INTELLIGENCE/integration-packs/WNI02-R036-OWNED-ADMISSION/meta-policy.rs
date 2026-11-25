//! @hq:context Bind actual resident Meta-VLANG definitions to native SDK admission.
//! @hq:invariant Pure policy can only narrow Host-owned resource ceilings; no grants or process effects.
use crate::{meta_resident::Resident,meta_language::{Value,Diagnostic},scheduler::{Snapshot,Budget,Block},resource_campaign::{ResourceCampaign,Admission,Error}};
pub const PARALLEL:&str="sdk.scheduler.parallel";
pub const BUSY:&str="sdk.scheduler.busy_ceiling";
pub const MEMORY:&str="sdk.scheduler.memory_reserve_mib";
pub const DISK:&str="sdk.scheduler.disk_reserve_mib";
const MIB:u64=1024*1024;
#[derive(Debug,PartialEq,Eq)]pub enum PolicyError{Diagnostic(Diagnostic),Invalid(&'static str),Admission(Error)}
#[derive(Debug,PartialEq,Eq)]pub struct Decision{pub generation:u64,pub sample_revision:u64,pub budget:Budget,pub admission:Admission}
fn read(r:&mut Resident,id:&'static str,args:&[Value])->Result<u64,PolicyError>{
 match r.query(id,args.to_vec()).map_err(PolicyError::Diagnostic)?.evaluation.value{
  Value::Int(n) if n>=0=>Ok(n as u64),_=>Err(PolicyError::Invalid(id))
 }
}
pub fn evaluate(r:&mut Resident,s:Snapshot,fixed:Budget,active:u32)->Result<Budget,PolicyError>{
 let busy=s.cpu_busy_bp.ok_or(PolicyError::Admission(Error::Resource(Block::UnknownCpu)))?;
 if busy>10000||s.logical_cpus==0{return Err(PolicyError::Invalid("sample"))}
 let args=[Value::Int(s.logical_cpus.into()),Value::Int(busy.into()),Value::Int(active.into()),
  Value::Int((s.memory_available/MIB) as i64),Value::Int((s.disk_available/MIB) as i64)];
 let parallel=read(r,PARALLEL,&args)?;
 let ceiling=read(r,BUSY,&args)?;
 let memory=read(r,MEMORY,&args)?.checked_mul(MIB).ok_or(PolicyError::Invalid(MEMORY))?;
 let disk=read(r,DISK,&args)?.checked_mul(MIB).ok_or(PolicyError::Invalid(DISK))?;
 if parallel==0||parallel>u64::from(fixed.max_parallel)||ceiling>u64::from(fixed.busy_ceiling_bp)||memory<fixed.memory_reserve||disk<fixed.disk_reserve{return Err(PolicyError::Invalid("fixed_budget"))}
 Ok(Budget{max_parallel:parallel as u32,busy_ceiling_bp:ceiling as u16,memory_reserve:memory,disk_reserve:disk,..fixed})
}
pub fn admit(r:&mut Resident,c:&mut ResourceCampaign,worker:&str,now:u64,ttl:u64,s:Snapshot)->Result<Decision,PolicyError>{
 let generation=r.generation();
 let budget=evaluate(r,s,c.budget(),c.reserved().tasks)?;
 let admission=c.admit_bounded(worker,now,ttl,s,budget).map_err(PolicyError::Admission)?;
 Ok(Decision{generation,sample_revision:s.revision,budget,admission})
}
#[cfg(test)]mod tests{
 use super::*;
 use crate::{protocol::{Snapshot as Language,Definition},campaign::Task,resource_campaign::Estimate};
 use std::collections::BTreeSet;
 const PARAMS:&str="((cpus int) (busy int) (active int) (memory int) (disk int))";
 fn rule(id:&str,rev:u64,expr:&str)->Definition{Definition{id:id.into(),revision:rev,dependencies:BTreeSet::new(),body:format!("meta/1 (fn {PARAMS} int {expr})")}}
 fn runtime()->Resident{
  let rows=[(PARALLEL,"(int 2)"),(BUSY,"(int 9000)"),(MEMORY,"(int 128)"),(DISK,"(int 64)")].map(|(id,e)|format!("{id}\t1\t\t{}",rule(id,1,e).body));
  Resident::start(Language::parse(&rows.join("\n")).unwrap()).unwrap()
 }
 fn sample()->Snapshot{Snapshot{revision:1,observed_ms:0,memory_available:1024*MIB,disk_available:1024*MIB,logical_cpus:8,cpu_busy_bp:Some(1000)}}
 fn campaign()->ResourceCampaign{
  let tasks=(0..4).map(|i|(Task{id:format!("t{i}"),source:"sdk".into(),objective:"compile".into()},Estimate{memory:64*MIB,disk:8*MIB,cpus:1,priority:1,enqueued_ms:0})).collect();
  ResourceCampaign::new(tasks,4,Budget{max_age_ms:100,memory_reserve:128*MIB,disk_reserve:64*MIB,cpu_reserve:1,max_parallel:4,busy_ceiling_bp:9500}).unwrap()
 }
 fn lease(d:Decision)->crate::campaign::Lease{match d.admission{Admission::Granted(l)=>l,_=>panic!("not granted")}}
 #[test]fn same_resident_reprograms_actual_admission_without_releasing_holds(){
  let(mut r,mut c)=(runtime(),campaign());let s=sample();
  let a=lease(admit(&mut r,&mut c,"a",0,20,s).unwrap());
  let d=rule(PARALLEL,2,"(int 1)");
  r.preview(1,vec![d.clone()],BTreeSet::new()).unwrap();assert_eq!(r.generation(),1);
  r.apply(1,vec![d],BTreeSet::new()).unwrap();
  let b=admit(&mut r,&mut c,"b",0,20,s).unwrap();
  assert_eq!(b.generation,2);assert_eq!(b.budget.max_parallel,1);assert!(matches!(b.admission,Admission::Deferred(_)));
  assert_eq!(c.reserved().tasks,1);
  c.confirm_exit(&a).unwrap();assert!(matches!(admit(&mut r,&mut c,"b",0,20,s).unwrap().admission,Admission::Granted(_)));
 }
 #[test]fn invalid_policy_cannot_admit_or_widen_budget(){
  let(mut r,mut c)=(runtime(),campaign());let s=sample();
  r.apply(1,vec![rule(PARALLEL,2,"(int 5)")],BTreeSet::new()).unwrap();
  assert_eq!(admit(&mut r,&mut c,"a",0,1,s),Err(PolicyError::Invalid("fixed_budget")));assert_eq!(c.reserved().tasks,0);
  let mut bad=c.budget();bad.max_parallel=5;
  assert_eq!(c.admit_bounded("a",0,1,s,bad),Err(Error::Invalid));
  r.apply(2,vec![rule(PARALLEL,3,"(int 2)"),rule(MEMORY,2,"(int 127)")],BTreeSet::new()).unwrap();
  assert!(admit(&mut r,&mut c,"a",0,1,s).is_err());assert_eq!(c.reserved().tasks,0);
 }
 #[test]fn policy_uses_real_inputs_and_forward_recovery(){
  let(mut r,mut c)=(runtime(),campaign());let checkpoint=r.checkpoint().unwrap();
  r.apply(1,vec![rule(PARALLEL,2,"(if (eq (arg cpus) (int 8)) (int 3) (int 1))")],BTreeSet::new()).unwrap();
  let d=admit(&mut r,&mut c,"a",0,20,sample()).unwrap();assert_eq!(d.budget.max_parallel,3);
  r.recover(2,&checkpoint,&[PARALLEL.to_owned()].into_iter().collect()).unwrap();
  let d=admit(&mut r,&mut c,"b",0,20,sample()).unwrap();assert_eq!((d.generation,d.budget.max_parallel),(3,2));
  assert_eq!(c.reserved().tasks,2);
 }
}
