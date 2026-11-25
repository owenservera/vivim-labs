//! @hq:context Bind incumbent campaign leases to the existing Host-owned process lifecycle.
//! @hq:invariant Release resource capacity only after the actual owned job exits.
//! @hq:why Lease expiry or a cancellation request is not process termination.
//! Windows-only integration candidate. No public launch endpoint or new authority.
//! Trusted incumbent caller supplies already-authorized executable, args, environment
//! and workspace; the campaign is exclusive to this one process owner.
use crate::{campaign::{Lease,Report},resource_campaign::{ResourceCampaign,Admission,Error as CampaignError},scheduler::{Snapshot,Reserved},workstation_process::{OwnedProcess,LocalProcessSpec}};
use std::{collections::BTreeMap,io,process::ExitStatus};
#[derive(Debug)]pub enum Error{Campaign(CampaignError),UnknownLease,LaunchUncertain{lease:Lease,error:io::Error},Process(io::Error)}
impl From<CampaignError> for Error{fn from(e:CampaignError)->Self{Self::Campaign(e)}}
impl From<io::Error> for Error{fn from(e:io::Error)->Self{Self::Process(e)}}
struct Running{lease:Lease,process:OwnedProcess}
pub struct OwnedCampaign{campaign:ResourceCampaign,running:BTreeMap<u64,Running>,uncertain: BTreeMap<u64,Lease>}
impl OwnedCampaign{
 pub fn new(campaign:ResourceCampaign)->Result<Self,Error>{
  if campaign.reserved()!=Reserved::default(){return Err(Error::UnknownLease);}
  Ok(Self{campaign,running:BTreeMap::new(),uncertain:BTreeMap::new()})
 }
 pub fn reserved(&self)->Reserved{self.campaign.reserved()}
 pub fn counts(&self)->(usize,usize,usize,usize){self.campaign.counts()}
 pub fn uncertain_count(&self)->usize{self.uncertain.len()}
 /// @hq:invariant Selection and reservation are serialized before launch.
 /// build is a trusted local adapter; returning None is a verified prelaunch refusal.
 /// An error from the process owner may include post-spawn setup failure:
 /// retain that reservation, with no automatic retry or external exit assertion.
 pub fn start(&mut self,worker:&str,now:u64,ttl:u64,sample:Snapshot,build:impl FnOnce(&Lease)->Option<LocalProcessSpec>)->Result<Admission,Error>{
  let a=self.campaign.admit(worker,now,ttl,sample)?;
  let Admission::Granted(lease)=a else{return Ok(a)};
  let Some(spec)=build(&lease)else{self.campaign.confirm_exit(&lease)?;return Err(Error::Process(io::Error::new(io::ErrorKind::PermissionDenied,"prelaunch refusal")));};
  match OwnedProcess::start(spec){
   Ok(process)=>{self.running.insert(lease.token,Running{lease:lease.clone(),process});Ok(Admission::Granted(lease))}
   Err(error)=>{self.uncertain.insert(lease.token,lease.clone());Err(Error::LaunchUncertain{lease,error})}
  }
 }
 fn owned(&mut self,lease:&Lease)->Result<&mut OwnedProcess,Error>{
  self.running.get_mut(&lease.token).filter(|r|r.lease==*lease).map(|r|&mut r.process).ok_or(Error::UnknownLease)
 }
 pub fn poll(&mut self,lease:&Lease)->Result<Option<ExitStatus>,Error>{Ok(self.owned(lease)?.poll()?)}
 /// Returns None while the entire Windows job remains alive. On confirmed
 /// terminal status, submit an optional successful result then release.
 /// Failed/expired reports do not convert exit into task success.
 pub fn settle(&mut self,lease:&Lease,now:u64,report:Option<Report>)->Result<Option<ExitStatus>,Error>{
  let Some(status)=self.owned(lease)?.poll()? else{return Ok(None)};
  let reported=if status.success(){match report{Some(r)=>self.campaign.submit(lease,now,r),None=>Ok(())}}else{Ok(())};
  self.campaign.confirm_exit(lease)?;
  self.running.remove(&lease.token);
  reported?;Ok(Some(status))
 }
 /// Cancellation is a request followed by owned job termination and readback.
 /// Retain reservation and handle if cancellation or observation fails.
 pub fn cancel(&mut self,lease:&Lease)->Result<ExitStatus,Error>{
  self.owned(lease)?;
  self.campaign.request_cancel(lease)?;
  let status=self.owned(lease)?.cancel()?;
  self.campaign.confirm_exit(lease)?;
  self.running.remove(&lease.token);
  Ok(status)
 }
}
#[cfg(test)]mod tests{
 use super::*;
 use crate::{campaign::Task,resource_campaign::Estimate,scheduler::{Budget,Block},native_sampling::{NativeSampler,SampleError}};
 use std::{time::{Duration,Instant},thread};
 fn grant(a:Admission)->Lease{match a{Admission::Granted(l)=>l,x=>panic!("{x:?}")}}
 fn task(id:&str)->(Task,Estimate){(Task{id:id.into(),source:"owned-fixture".into(),objective:"bounded verification".into()},Estimate{memory:64*1024*1024,disk:1024*1024,cpus:1,priority:1,enqueued_ms:0})}
 fn setup()->(OwnedCampaign,NativeSampler,Snapshot){
  let path=std::env::current_dir().unwrap();let p=path.to_str().unwrap();let drive=p.strip_prefix(r"\\?\").unwrap_or(p).chars().next().unwrap();
  let mut s=NativeSampler::new(drive);assert_eq!(s.capture(),Err(SampleError::Warmup));thread::sleep(Duration::from_millis(180));let sample=s.capture().unwrap();
  let b=Budget{max_age_ms:5000,memory_reserve:128*1024*1024,disk_reserve:64*1024*1024,cpu_reserve:0,max_parallel:2,busy_ceiling_bp:10000};
  (OwnedCampaign::new(ResourceCampaign::new(vec![task("a"),task("b"),task("c")],3,b).unwrap()).unwrap(),s,sample)
 }
 fn spec(mode:&str)->LocalProcessSpec{
  let mut environment=vec![("WNI_R036_CHILD".into(),mode.into())];if let Some(v)=std::env::var_os("SystemRoot"){environment.push(("SystemRoot".into(),v));}
  LocalProcessSpec{executable:std::env::current_exe().unwrap(),workspace:std::env::current_dir().unwrap(),args:["--exact","owned_campaign::tests::fixture_child","--nocapture","--test-threads=1"].into_iter().map(Into::into).collect(),environment,tail_bytes:2048}
 }
 #[test]fn fixture_child(){match std::env::var("WNI_R036_CHILD").as_deref(){Ok("short")=>thread::sleep(Duration::from_millis(2200)),Ok("long")=>thread::sleep(Duration::from_secs(20)),_=>{}}}
 #[test]fn real_parallel_jobs_cancel_and_release_after_exit(){
  let(mut c,s,sample)=setup();
  let a=grant(c.start("worker-a",s.now_ms(),30000,sample,|_|Some(spec("short"))).unwrap());
  let b=grant(c.start("worker-b",s.now_ms(),30000,sample,|_|Some(spec("long"))).unwrap());
  assert_eq!(c.poll(&a).unwrap(),None);assert_eq!(c.poll(&b).unwrap(),None);
  assert_eq!(c.reserved().tasks,2);
  assert!(matches!(c.start("worker-c",s.now_ms(),30000,sample,|_|panic!("deferred launch")),Ok(Admission::Deferred(v)) if v.iter().any(|(_,b)|*b==Block::Parallel)));
  let mut fake=a.clone();fake.worker="wrong".into();assert!(matches!(c.cancel(&fake),Err(Error::UnknownLease)));
  let killed=c.cancel(&b).unwrap();assert!(!killed.success());assert_eq!(c.reserved().tasks,1);
  assert!(matches!(c.cancel(&b),Err(Error::UnknownLease)));
  let begin=Instant::now();
  loop{let r=c.settle(&a,s.now_ms(),Some(Report{evidence:vec!["owned-exit".into()],observation:"fixture completed".into(),recommendation:"review".into()})).unwrap();
   if let Some(exit)=r{assert!(exit.success());break;}
   assert_eq!(c.reserved().tasks,1);assert!(begin.elapsed()<Duration::from_secs(8));thread::sleep(Duration::from_millis(25));}
  assert_eq!(c.reserved(),Reserved::default());assert_eq!(c.counts().2,1);
  println!("OWNED_CAMPAIGN_PARALLEL=2 CANCEL_CONFIRMED=1 NORMAL_EXIT=1 RELEASED=2");
 }
 #[test]fn refusal_and_uncertain_start(){
  let(mut c,s,sample)=setup();
  assert!(matches!(c.start("a",s.now_ms(),30000,sample,|_|None),Err(Error::Process(_))));assert_eq!(c.reserved().tasks,0);
  let r=c.start("a",s.now_ms(),30000,sample,|_|{let mut p=spec("short");p.executable=p.workspace.join("nonexistent-r036.exe");Some(p)});
  let lease=match r{Err(Error::LaunchUncertain{lease,..})=>lease,_=>panic!("launch should be uncertain")};
  assert_eq!(c.reserved().tasks,1);assert_eq!(c.uncertain_count(),1);assert!(matches!(c.cancel(&lease),Err(Error::UnknownLease)));
 }
}
