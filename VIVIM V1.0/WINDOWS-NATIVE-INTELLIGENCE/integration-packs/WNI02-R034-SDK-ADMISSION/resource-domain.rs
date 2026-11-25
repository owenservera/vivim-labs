//! @hq:context One exclusive admission domain coordinates several existing SDK campaigns.
//! @hq:invariant No duplicate process owner or scheduler; every live or uncertain hold counts globally.
//! @hq:why Independent local budgets can overcommit a single workstation.
use crate::{campaign::{Task,Lease,Report},meta_resident::Resident,meta_policy::Decision,resource_campaign::{ResourceCampaign,Estimate,Admission,Error as ResourceError,combine},scheduler::{self,Snapshot,Budget,Reserved,Block},owned_campaign::{OwnedCampaign,ProfiledExit,Error as OwnedError},workstation_process::LocalProcessSpec};
use std::{collections::BTreeMap,sync::Arc,process::ExitStatus};
#[derive(Debug)]pub enum Error{Invalid,Duplicate,Unknown,Stale,WorkerBusy,Resource(ResourceError),Owned(OwnedError),Sample(Block)}
impl From<ResourceError> for Error{fn from(e:ResourceError)->Self{Self::Resource(e)}}
impl From<OwnedError> for Error{fn from(e:OwnedError)->Self{Self::Owned(e)}}
#[derive(Clone,Debug)]pub struct DomainLease{identity:Arc<()>,campaign:String,serial:u64,lease:Lease}
impl DomainLease{pub fn serial(&self)->u64{self.serial}pub fn lease(&self)->&Lease{&self.lease}pub fn campaign(&self)->&str{&self.campaign}}
#[derive(Debug)]pub struct DomainDecision{pub decision:Decision,pub lease:Option<DomainLease>}
pub struct Domain{campaigns:BTreeMap<String,OwnedCampaign>,budget:Budget,identity:Arc<()>,serial:u64,last:Option<Snapshot>,clock:u64}
impl Domain{
 pub fn new(budget:Budget)->Self{Self{campaigns:BTreeMap::new(),budget,identity:Arc::new(()),serial:0,last:None,clock:0}}
 pub fn register(&mut self,id:&str,tasks:Vec<(Task,Estimate)>,limit:usize)->Result<(),Error>{
  if id.is_empty()||id.len()>64||!id.bytes().all(|b|b.is_ascii_alphanumeric()||b"._-".contains(&b))||self.campaigns.len()>=128{return Err(Error::Invalid)}
  if self.campaigns.contains_key(id){return Err(Error::Duplicate)}
  self.campaigns.insert(id.into(),OwnedCampaign::new(ResourceCampaign::new(tasks,limit,self.budget)?)?);Ok(())
 }
 pub fn reserved(&self)->Result<Reserved,Error>{
  self.campaigns.values().try_fold(Reserved::default(),|sum,c|combine(sum,c.reserved())).map_err(Into::into)
 }
 pub fn counts(&self,id:&str)->Result<(usize,usize,usize,usize),Error>{Ok(self.campaigns.get(id).ok_or(Error::Unknown)?.counts())}
 pub fn uncertain_count(&self)->usize{self.campaigns.values().map(OwnedCampaign::uncertain_count).sum()}
 /// @hq:invariant The resident sees the global active count and the same current sample.
 pub fn start(&mut self,id:&str,r:&mut Resident,worker:&str,now:u64,ttl:u64,s:Snapshot,build:impl FnOnce(&Lease)->Option<LocalProcessSpec>)->Result<DomainDecision,Error>{
  if !self.campaigns.contains_key(id){return Err(Error::Unknown)}
  if now<self.clock||self.last.is_some_and(|old|s.revision<old.revision||s.observed_ms<old.observed_ms||(s.revision==old.revision&&s!=old)){return Err(Error::Stale)}
  let next=self.serial.checked_add(1).ok_or(Error::Invalid)?;
  let total=self.reserved()?;
  scheduler::plan(s,self.budget,total,now,&[]).map_err(Error::Sample)?;
  self.last=Some(s);self.clock=now;
  if self.campaigns.values().any(|c|c.has_worker(worker)){return Err(Error::WorkerBusy)}
  let external=self.campaigns.iter().filter(|(name,_)|name.as_str()!=id).try_fold(Reserved::default(),|sum,(_,c)|combine(sum,c.reserved()))?;
  let decision=self.campaigns.get_mut(id).unwrap().start_policy_shared(r,worker,now,ttl,s,external,build)?;
  let lease=if let Admission::Granted(lease)=&decision.admission{
   self.serial=next;
   Some(DomainLease{identity:Arc::clone(&self.identity),campaign:id.into(),serial:next,lease:lease.clone()})
  }else{None};
  Ok(DomainDecision{decision,lease})
 }
 fn owner(&mut self,l:&DomainLease)->Result<&mut OwnedCampaign,Error>{
  if !Arc::ptr_eq(&self.identity,&l.identity){return Err(Error::Unknown)}
  self.campaigns.get_mut(&l.campaign).ok_or(Error::Unknown)
 }
 pub fn poll(&mut self,l:&DomainLease)->Result<Option<ExitStatus>,Error>{Ok(self.owner(l)?.poll(&l.lease)?)}
 pub fn settle(&mut self,l:&DomainLease,now:u64,report:Option<Report>)->Result<Option<ProfiledExit>,Error>{
  if now<self.clock{return Err(Error::Stale)}
  let result=self.owner(l)?.settle_profiled(&l.lease,now,report);
  self.clock=now;Ok(result?)
 }
 pub fn cancel(&mut self,l:&DomainLease)->Result<ExitStatus,Error>{Ok(self.owner(l)?.cancel(&l.lease)?)}
}
#[cfg(test)]mod tests{
 use super::*;
 use crate::{protocol::{Snapshot as Language,Definition},meta_policy};
 use std::collections::BTreeSet;
 const MIB:u64=1024*1024;const PARAMS:&str="((cpus int) (busy int) (active int) (memory int) (disk int))";
 fn runtime()->Resident{
  let rows=[(meta_policy::PARALLEL,"(int 2)"),(meta_policy::BUSY,"(int 10000)"),(meta_policy::MEMORY,"(int 128)"),(meta_policy::DISK,"(int 64)")].map(|(id,v)|format!("{id}\t1\t\tmeta/1 (fn {PARAMS} int {v})"));
  Resident::start(Language::parse(&rows.join("\n")).unwrap()).unwrap()
 }
 fn sample()->Snapshot{Snapshot{revision:1,observed_ms:0,memory_available:1024*MIB,disk_available:1024*MIB,logical_cpus:8,cpu_busy_bp:Some(1000)}}
 fn domain(memory:u64)->Domain{
  let mut d=Domain::new(Budget{max_age_ms:100,memory_reserve:128*MIB,disk_reserve:64*MIB,cpu_reserve:0,max_parallel:2,busy_ceiling_bp:10000});
  for id in ["compiler","context"]{
   let tasks=(0..2).map(|i|(Task{id:format!("t{i}"),source:"actual-sdk-work".into(),objective:"verify".into()},Estimate{memory,disk:8*MIB,cpus:1,priority:1,enqueued_ms:0})).collect();
   d.register(id,tasks,2).unwrap();
  }d
 }
 fn fixture()->LocalProcessSpec{
  let mut environment=vec![("VIVIM_OWNED_CHILD_TEST".into(),"sleep".into())];if let Some(v)=std::env::var_os("SystemRoot"){environment.push(("SystemRoot".into(),v));}
  LocalProcessSpec{executable:std::env::current_exe().unwrap(),workspace:std::env::current_dir().unwrap(),args:["--exact","workstation_process::tests::fixture_child","--nocapture","--test-threads=1"].into_iter().map(Into::into).collect(),environment,tail_bytes:4096}
 }
 #[test]fn real_processes_share_global_parallel_budget_and_worker_identity(){
  let(mut d,mut r)=(domain(64*MIB),runtime());let s=sample();
  let a=d.start("compiler",&mut r,"a",0,1000,s,|_|Some(fixture())).unwrap().lease.unwrap();
  assert!(matches!(d.start("context",&mut r,"a",0,1000,s,|_|panic!("duplicate worker")),Err(Error::WorkerBusy)));
  let b=d.start("context",&mut r,"b",0,1000,s,|_|Some(fixture())).unwrap().lease.unwrap();
  assert_ne!(a.serial(),b.serial());assert_eq!(d.reserved().unwrap().tasks,2);
  let c=d.start("compiler",&mut r,"c",0,1000,s,|_|panic!("global cap")).unwrap();
  assert!(matches!(c.decision.admission,Admission::Deferred(_)));assert!(c.lease.is_none());
  let mut other=domain(64*MIB);assert!(matches!(other.cancel(&a),Err(Error::Unknown)));
  d.cancel(&a).unwrap();let c=d.start("compiler",&mut r,"c",0,1000,s,|_|Some(fixture())).unwrap().lease.unwrap();
  assert!(matches!(d.cancel(&a),Err(Error::Owned(_))));assert!(d.poll(&b).unwrap().is_none());
  d.cancel(&b).unwrap();d.cancel(&c).unwrap();assert_eq!(d.reserved().unwrap(),Reserved::default());
  println!("GLOBAL_CAMPAIGN_BUDGET_OK");
 }
 #[test]fn policy_observes_global_active_count_and_sample_fence(){
  let(mut d,mut r)=(domain(64*MIB),runtime());let s=sample();
  let a=d.start("compiler",&mut r,"a",0,1000,s,|_|Some(fixture())).unwrap().lease.unwrap();
  let def=|rev,expr:&str|Definition{id:meta_policy::PARALLEL.into(),revision:rev,dependencies:BTreeSet::new(),body:format!("meta/1 (fn {PARAMS} int {expr})")};
  r.apply(1,vec![def(2,"(if (eq (arg active) (int 1)) (int 1) (int 2))")],BTreeSet::new()).unwrap();
  let b=d.start("context",&mut r,"b",1,1000,Snapshot{revision:2,observed_ms:1,..s},|_|panic!("policy sees other campaign")).unwrap();
  assert_eq!(b.decision.budget.max_parallel,1);assert!(b.lease.is_none());
  assert!(matches!(d.start("compiler",&mut r,"c",1,1000,s,|_|panic!("stale")),Err(Error::Stale)));
  r.apply(2,vec![def(3,"(int 2)")],BTreeSet::new()).unwrap();
  let b=d.start("context",&mut r,"b",1,1000,Snapshot{revision:2,observed_ms:1,..s},|_|Some(fixture())).unwrap().lease.unwrap();
  d.cancel(&a).unwrap();d.cancel(&b).unwrap();
 }
 #[test]fn uncertain_launch_and_memory_count_across_campaigns(){
  let(mut d,mut r)=(domain(256*MIB),runtime());let s=Snapshot{memory_available:512*MIB,..sample()};
  let failed=d.start("compiler",&mut r,"a",0,1000,s,|_|{let mut p=fixture();p.executable=p.workspace.join("absent-r039.exe");Some(p)});
  assert!(matches!(failed,Err(Error::Owned(OwnedError::LaunchUncertain{..}))));
  assert_eq!(d.uncertain_count(),1);assert_eq!(d.reserved().unwrap().memory,256*MIB);
  let b=d.start("context",&mut r,"b",0,1000,s,|_|panic!("insufficient memory")).unwrap();
  assert!(matches!(b.decision.admission,Admission::Deferred(v) if v.iter().any(|(_,why)|*why==Block::Memory)));
 }
}
