//! Resource-aware admission over the incumbent Product campaign and Labs scheduler.
//! Host owns process lifecycle and supplies trusted exit observations. No process is started here.
use crate::{campaign::{Campaign,Task,Lease,Report,Error as CampaignError},scheduler::{self,Snapshot,Budget,Work,Reserved,Block}};
use std::collections::{BTreeMap,BTreeSet};
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Estimate {pub memory:u64,pub disk:u64,pub cpus:u32,pub priority:u8,pub enqueued_ms:u64}
#[derive(Debug,PartialEq,Eq)]
pub enum Error {Invalid,StaleSample,BusyWorker,UnknownLease,Cancelled,Campaign(CampaignError),Resource(Block)}
impl From<CampaignError> for Error {fn from(e:CampaignError)->Self{Self::Campaign(e)}}
impl From<Block> for Error {fn from(e:Block)->Self{Self::Resource(e)}}
#[derive(Debug,PartialEq,Eq)]
pub enum Admission {Granted(Lease),Deferred(Vec<(String,Block)>),Empty}
struct Hold {lease:Lease,cost:Estimate,cancelled:bool}
pub struct ResourceCampaign {campaign:Campaign,costs:BTreeMap<String,(u64,Estimate)>,held:BTreeMap<u64,Hold>,last:Option<Snapshot>,budget:Budget}
/// @hq:invariant Shared domain accounting cannot wrap or underestimate existing reservations.
pub fn combine(a:Reserved,b:Reserved)->Result<Reserved,Error>{
 Ok(Reserved{memory:a.memory.checked_add(b.memory).ok_or(Error::Invalid)?,disk:a.disk.checked_add(b.disk).ok_or(Error::Invalid)?,cpus:a.cpus.checked_add(b.cpus).ok_or(Error::Invalid)?,tasks:a.tasks.checked_add(b.tasks).ok_or(Error::Invalid)?})
}
impl ResourceCampaign {
 pub fn new(tasks:Vec<(Task,Estimate)>,limit:usize,budget:Budget)->Result<Self,Error>{
  if tasks.iter().any(|(_,e)|e.memory==0||e.cpus==0){return Err(Error::Invalid)}
  let mut costs=BTreeMap::new();let mut definitions=Vec::new();
  for (i,(task,cost)) in tasks.into_iter().enumerate(){
   if costs.insert(task.id.clone(),(i as u64+1,cost)).is_some(){return Err(Error::Invalid)}
   definitions.push(task);
  }
  Ok(Self{campaign:Campaign::new(definitions,limit)?,costs,held:BTreeMap::new(),last:None,budget})
 }
 pub fn reserved(&self)->Reserved{
  self.held.values().fold(Reserved::default(),|r,h|Reserved{memory:r.memory.saturating_add(h.cost.memory),disk:r.disk.saturating_add(h.cost.disk),cpus:r.cpus.saturating_add(h.cost.cpus),tasks:r.tasks.saturating_add(1)})
 }
 pub fn counts(&self)->(usize,usize,usize,usize){self.campaign.counts()}
 pub fn budget(&self)->Budget{self.budget}
 pub fn has_worker(&self,worker:&str)->bool{self.held.values().any(|h|h.lease.worker==worker)}
 pub fn admit(&mut self,worker:&str,now:u64,ttl:u64,sample:Snapshot)->Result<Admission,Error>{self.admit_bounded(worker,now,ttl,sample,self.budget)}
 /// @hq:invariant Dynamic policy may tighten, never widen, the fixed owner budget.
 pub fn admit_bounded(&mut self,worker:&str,now:u64,ttl:u64,sample:Snapshot,budget:Budget)->Result<Admission,Error>{
  self.admit_shared(worker,now,ttl,sample,budget,Reserved::default())
 }
 /// @hq:invariant The exclusive Host domain supplies other campaigns' retained reservations.
 pub(crate) fn admit_shared(&mut self,worker:&str,now:u64,ttl:u64,sample:Snapshot,budget:Budget,external:Reserved)->Result<Admission,Error>{
  let fixed=self.budget;
  if budget.max_age_ms>fixed.max_age_ms||budget.memory_reserve<fixed.memory_reserve||budget.disk_reserve<fixed.disk_reserve||budget.cpu_reserve<fixed.cpu_reserve||budget.max_parallel>fixed.max_parallel||budget.busy_ceiling_bp>fixed.busy_ceiling_bp{return Err(Error::Invalid)}
  if worker.trim().is_empty()||worker.len()>96||worker.chars().any(char::is_control)||ttl==0||now.checked_add(ttl).is_none(){return Err(Error::Invalid)}
  if self.last.is_some_and(|s|sample.revision<s.revision||sample.observed_ms<s.observed_ms||(sample.revision==s.revision&&sample!=s)){return Err(Error::StaleSample)}
  // Validate freshness even when queue is empty; do not admit on ambiguous CPU samples.
  let already=combine(self.reserved(),external)?;
  scheduler::plan(sample,budget,already,now,&[])?;
  self.last=Some(sample);
  if self.held.values().any(|h|h.lease.worker==worker){return Err(Error::BusyWorker)}
  self.campaign.advance(now)?;
  let held:BTreeSet<_>=self.held.values().map(|h|h.lease.task.id.as_str()).collect();
  let mut pending:Vec<_>=self.campaign.pending_ids().filter(|id|!held.contains(id)).map(|id|{
   let (key,e)=self.costs[id];(id.to_owned(),Work{id:key,memory:e.memory,disk:e.disk,cpus:e.cpus,priority:e.priority,enqueued_ms:e.enqueued_ms})
  }).collect();
  // Planner accepts bounded batches. Scan priority order in chunks to avoid starving a small
  // runnable task behind an arbitrarily large group of unaffordable tasks.
  pending.sort_by_key(|(_,w)|(std::cmp::Reverse(w.priority),w.enqueued_ms,w.id));
  let mut deferred=Vec::new();
  for batch in pending.chunks(256){
   let work:Vec<_>=batch.iter().map(|(_,w)|*w).collect();
   let plan=scheduler::plan(sample,budget,already,now,&work)?;
   if let Some(key)=plan.admitted.first(){
    let (id,_)=batch.iter().find(|(_,w)|w.id==*key).ok_or(Error::Invalid)?;
    let cost=self.costs[id].1;
    let lease=self.campaign.claim_selected(worker,now,ttl,Some(id))?.ok_or(Error::Invalid)?;
    // Exclusive &mut makes claim and reservation one in-memory transition; launch happens outside.
    self.held.insert(lease.token,Hold{lease:lease.clone(),cost,cancelled:false});
    return Ok(Admission::Granted(lease))
   }
   deferred.extend(plan.deferred.into_iter().map(|(key,reason)|(batch.iter().find(|(_,w)|w.id==key).unwrap().0.clone(),reason)));
  }
  if deferred.is_empty(){Ok(Admission::Empty)}else{Ok(Admission::Deferred(deferred))}
 }
 pub fn request_cancel(&mut self,lease:&Lease)->Result<(),Error>{
  let h=self.held.get_mut(&lease.token).filter(|h|h.lease==*lease).ok_or(Error::UnknownLease)?;
  h.cancelled=true;Ok(())
 }
 pub fn submit(&mut self,lease:&Lease,now:u64,report:Report)->Result<(),Error>{
  let h=self.held.get(&lease.token).filter(|h|h.lease==*lease).ok_or(Error::UnknownLease)?;
  if h.cancelled{return Err(Error::Cancelled)}
  self.campaign.submit(lease,now,report)?;Ok(())
 }
 /// Only the incumbent process owner may call after confirmed exit or proven launch failure.
 /// Neither lease expiry, a report nor cancellation is evidence that a process stopped.
 pub fn confirm_exit(&mut self,lease:&Lease)->Result<(),Error>{
  if !self.held.get(&lease.token).is_some_and(|h|h.lease==*lease){return Err(Error::UnknownLease)}
  self.campaign.abandon(lease);self.held.remove(&lease.token);Ok(())
 }
 pub fn review(&mut self,id:&str,reviewer:&str,accepted:bool)->Result<(),Error>{self.campaign.review(id,reviewer,accepted)?;Ok(())}
}
#[cfg(test)]mod tests{
 use super::*;
 fn t(id:usize,memory:u64,priority:u8)->(Task,Estimate){(Task{id:format!("t{id}"),source:"sdk.rs".into(),objective:"compile".into()},Estimate{memory,disk:1,cpus:1,priority,enqueued_ms:0})}
 fn s(revision:u64,now:u64)->Snapshot{Snapshot{revision,observed_ms:now,memory_available:100,disk_available:100,logical_cpus:8,cpu_busy_bp:Some(100)}}
 fn b()->Budget{Budget{max_age_ms:10,memory_reserve:20,disk_reserve:10,cpu_reserve:1,max_parallel:4,busy_ceiling_bp:9000}}
 fn grant(a:Admission)->Lease{match a{Admission::Granted(l)=>l,_=>panic!("not granted: {a:?}")}}
 fn report()->Report{Report{evidence:vec!["receipt".into()],observation:"done".into(),recommendation:"review".into()}}
 #[test]fn priority_budget_release_and_review(){
  let mut c=ResourceCampaign::new(vec![t(0,60,1),t(1,50,5),t(2,30,2)],4,b()).unwrap();
  let a=grant(c.admit("a",0,10,s(1,0)).unwrap());let d=grant(c.admit("b",0,10,s(1,0)).unwrap());
  assert_eq!((a.task.id.as_str(),d.task.id.as_str()),("t1","t2"));assert_eq!(c.reserved().memory,80);
  assert!(matches!(c.admit("c",0,10,s(1,0)),Ok(Admission::Deferred(_))));
  c.submit(&a,1,report()).unwrap();assert_eq!(c.reserved().memory,80);
  assert!(c.review("t1","a",true).is_err());c.review("t1","reviewer",true).unwrap();
  c.confirm_exit(&a).unwrap();c.confirm_exit(&d).unwrap();
  assert_eq!(c.reserved(),Reserved::default());assert_eq!(c.confirm_exit(&a),Err(Error::UnknownLease));
 }
 #[test]fn expiry_cancel_and_stale_token_never_release_early(){
  let mut c=ResourceCampaign::new(vec![t(0,80,1)],4,b()).unwrap();
  let a=grant(c.admit("a",0,2,s(1,0)).unwrap());
  c.request_cancel(&a).unwrap();assert_eq!(c.submit(&a,1,report()),Err(Error::Cancelled));
  assert_eq!(c.admit("b",3,10,s(2,3)),Ok(Admission::Empty));assert_eq!(c.reserved().tasks,1);
  assert_eq!(c.admit("a",3,10,s(2,3)),Err(Error::BusyWorker));c.confirm_exit(&a).unwrap();
  let next=grant(c.admit("b",3,10,s(2,3)).unwrap());assert_ne!(a.token,next.token);
  assert_eq!(c.confirm_exit(&a),Err(Error::UnknownLease));assert_eq!(c.reserved().tasks,1);
 }
 #[test]fn freshness_and_equivocation(){
  let mut c=ResourceCampaign::new(vec![t(0,1,1)],4,b()).unwrap();
  assert_eq!(c.admit("a",11,1,s(1,0)),Err(Error::Resource(Block::Stale)));
  assert_eq!(c.admit("a",0,1,Snapshot{cpu_busy_bp:None,..s(1,0)}),Err(Error::Resource(Block::UnknownCpu)));
  let a=grant(c.admit("a",0,1,s(2,0)).unwrap());c.confirm_exit(&a).unwrap();
  assert_eq!(c.admit("b",1,1,s(1,1)),Err(Error::StaleSample));
  assert_eq!(c.admit("b",1,1,s(2,1)),Err(Error::StaleSample));
 }
 #[test]fn bounded_batches_do_not_starve_affordable_task(){
  let mut tasks:Vec<_>=(0..300).map(|i|t(i,100,5)).collect();tasks.push(t(301,1,0));
  let mut c=ResourceCampaign::new(tasks,4,b()).unwrap();let a=grant(c.admit("a",0,1,s(1,0)).unwrap());assert_eq!(a.task.id,"t301");
 }
 #[test]fn actual_parallel_owned_work_holds_capacity_until_join(){
  use std::sync::{Arc,Barrier};use std::thread;
  let mut c=ResourceCampaign::new(vec![t(0,40,1),t(1,40,1),t(2,40,1)],4,b()).unwrap();
  let barrier=Arc::new(Barrier::new(3));let mut running=Vec::new();
  for worker in ["a","b"]{let l=grant(c.admit(worker,0,10,s(1,0)).unwrap());let b=barrier.clone();
   running.push((l,thread::spawn(move||{b.wait();let mut n=0u64;for i in 0..100_000u64{n=n.wrapping_add(std::hint::black_box(i));}n})));
  }
  assert!(matches!(c.admit("c",0,10,s(1,0)),Ok(Admission::Deferred(_))));
  barrier.wait();for(l,h)in running{assert_eq!(h.join().unwrap(),4_999_950_000);c.submit(&l,1,report()).unwrap();c.confirm_exit(&l).unwrap();}
  assert_eq!(c.reserved().tasks,0);assert!(matches!(c.admit("c",1,10,s(1,0)),Ok(Admission::Granted(_))));
 }
}
