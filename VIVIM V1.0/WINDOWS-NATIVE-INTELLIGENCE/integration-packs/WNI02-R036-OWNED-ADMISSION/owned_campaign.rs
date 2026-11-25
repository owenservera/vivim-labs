//! @hq:context Bind incumbent campaign leases to the existing Host-owned process lifecycle.
//! @hq:invariant Release resource capacity only after the actual owned job exits.
//! @hq:why Lease expiry or a cancellation request is not process termination.
//! Windows-only integration candidate. No public launch endpoint or new authority.
//! Trusted incumbent caller supplies already-authorized executable, args, environment
//! and workspace; the campaign is exclusive to this one process owner.
use crate::{campaign::{Lease,Report},resource_campaign::{ResourceCampaign,Admission,Error as CampaignError},scheduler::{Snapshot,Reserved},workstation_process::{OwnedProcess,LocalProcessSpec}};
use std::{collections::BTreeMap,io,process::ExitStatus};
#[derive(Debug)]pub enum Error{Campaign(CampaignError),Policy(crate::meta_policy::PolicyError),UnknownLease,LaunchUncertain{lease:Lease,error:io::Error},Process(io::Error)}
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
  self.launch(a,build)
 }
 /// Same incumbent resident rule evaluator and campaign, not a second policy registry.
 /// Policy generation and resource revision travel with the result; policy cannot widen fixed ceilings.
 pub fn start_with_policy(&mut self,resident:&mut crate::meta_resident::Resident,worker:&str,now:u64,ttl:u64,sample:Snapshot,build:impl FnOnce(&Lease)->Option<LocalProcessSpec>)->Result<crate::meta_policy::Decision,Error>{
  let mut d=crate::meta_policy::admit(resident,&mut self.campaign,worker,now,ttl,sample).map_err(Error::Policy)?;
  d.admission=self.launch(d.admission,build)?;Ok(d)
 }
 fn launch(&mut self,a:Admission,build:impl FnOnce(&Lease)->Option<LocalProcessSpec>)->Result<Admission,Error>{
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

 #[test]fn resident_policy_tightens_real_owned_jobs(){
  use crate::{meta_policy::{PARALLEL,BUSY,MEMORY,DISK},protocol::{Definition,Snapshot as Language},meta_resident::Resident};
  use std::collections::BTreeSet;
  const PARAMS:&str="((cpus int) (busy int) (active int) (memory int) (disk int))";
  let rule=|id:&str,rev:u64,value:u64|Definition{id:id.into(),revision:rev,dependencies:BTreeSet::new(),body:format!("meta/1 (fn {PARAMS} int (int {value}))")};
  let rows=[(PARALLEL,2),(BUSY,10000),(MEMORY,128),(DISK,64)].map(|(id,v)|format!("{id}\t1\t\t{}",rule(id,1,v).body));
  let mut resident=Resident::start(Language::parse(&rows.join("\n")).unwrap()).unwrap();
  let(mut c,s,sample)=setup();
  let a=c.start_with_policy(&mut resident,"a",s.now_ms(),30000,sample,|_|Some(spec("long"))).unwrap();
  assert_eq!((a.generation,a.sample_revision),(1,sample.revision));let lease=grant(a.admission);
  resident.apply(1,vec![rule(PARALLEL,2,1)],BTreeSet::new()).unwrap();
  let b=c.start_with_policy(&mut resident,"b",s.now_ms(),30000,sample,|_|panic!("policy deferred launch")).unwrap();
  assert_eq!((b.generation,b.budget.max_parallel),(2,1));assert!(matches!(b.admission,Admission::Deferred(_)));
  assert_eq!(c.reserved().tasks,1);assert_eq!(c.poll(&lease).unwrap(),None);
  resident.apply(2,vec![rule(PARALLEL,3,3)],BTreeSet::new()).unwrap();
  assert!(matches!(c.start_with_policy(&mut resident,"b",s.now_ms(),30000,sample,|_|panic!("widened policy")),Err(Error::Policy(_))));
  assert_eq!(c.reserved().tasks,1);
  resident.apply(3,vec![rule(PARALLEL,4,1)],BTreeSet::new()).unwrap();
  assert!(!c.cancel(&lease).unwrap().success());assert_eq!(c.reserved().tasks,0);
  let b=grant(c.start_with_policy(&mut resident,"b",s.now_ms(),30000,sample,|_|Some(spec("long"))).unwrap().admission);
  assert_eq!(c.reserved().tasks,1);assert!(!c.cancel(&b).unwrap().success());assert_eq!(c.reserved().tasks,0);
  println!("RESIDENT_POLICY_REAL_JOB generation=4 narrowed=1 widened_refused=true cancelled_released=true");
 }

 #[test]fn real_compilers_through_owned_policy(){
  use crate::{meta_policy::{PARALLEL,BUSY,MEMORY,DISK},protocol::Snapshot as Language,meta_resident::Resident};
  use std::{path::PathBuf,process::Command};
  let cwd=std::env::current_dir().unwrap();
  let out=cwd.join(format!("wni-r039-{}-{}",std::process::id(),std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
  std::fs::create_dir(&out).unwrap();
  struct Fixture(PathBuf);
  impl Drop for Fixture{fn drop(&mut self){let _=std::fs::remove_dir_all(&self.0);}}
  let _fixture=Fixture(out.clone());
  let sysroot=Command::new("rustc").args(["--print","sysroot"]).output().unwrap();
  assert!(sysroot.status.success());
  let exe=PathBuf::from(String::from_utf8(sysroot.stdout).unwrap().trim()).join("bin").join("rustc.exe");
  assert!(exe.is_file());
  let rows=[(PARALLEL,2),(BUSY,10000),(MEMORY,128),(DISK,64)].map(|(id,v)|format!("{id}\t1\t\tmeta/1 (fn ((cpus int) (busy int) (active int) (memory int) (disk int)) int (int {v}))"));
  let mut resident=Resident::start(Language::parse(&rows.join("\n")).unwrap()).unwrap();
  let drive=cwd.to_str().unwrap().strip_prefix(r"\\?\").unwrap_or(cwd.to_str().unwrap()).chars().next().unwrap();
  let mut sampler=NativeSampler::new(drive);
  assert_eq!(sampler.capture(),Err(SampleError::Warmup));thread::sleep(Duration::from_millis(180));
  let mut sample=sampler.capture().unwrap();
  for _ in 0..4{if sample.cpu_busy_bp!=Some(10000){break;}thread::sleep(Duration::from_millis(180));sample=sampler.capture().unwrap();}
  assert_ne!(sample.cpu_busy_bp,Some(10000),"CPU saturated; admission must defer");
  let tasks=["campaign","scheduler"].into_iter().map(|name|(Task{id:name.into(),source:format!("{name}.rs"),objective:"compile exact Rust metadata".into()},Estimate{memory:128*1024*1024,disk:8*1024*1024,cpus:1,priority:1,enqueued_ms:0})).collect();
  let budget=Budget{max_age_ms:5000,memory_reserve:128*1024*1024,disk_reserve:64*1024*1024,cpu_reserve:0,max_parallel:2,busy_ceiling_bp:10000};
  let mut owner=OwnedCampaign::new(ResourceCampaign::new(tasks,2,budget).unwrap()).unwrap();
  let mut jobs=Vec::new();let begin=Instant::now();
  for worker in ["rustc-a","rustc-b"]{
   let d=owner.start_with_policy(&mut resident,worker,sampler.now_ms(),30000,sample,|lease|{
    let src=cwd.join(&lease.task.source);let dst=out.join(format!("{}.rmeta",lease.task.id));
    assert!(src.is_file());
    let environment=["SystemRoot","TEMP","TMP"].into_iter().filter_map(|k|std::env::var_os(k).map(|v|(k.into(),v))).collect();
    Some(LocalProcessSpec{executable:exe.clone(),workspace:cwd.clone(),args:vec!["--edition=2021".into(),"--crate-type=lib".into(),"--emit=metadata".into(),"--crate-name".into(),lease.task.id.clone().into(),src.into_os_string(),"-o".into(),dst.into_os_string()],environment,tail_bytes:8192})
   }).unwrap();
   assert_eq!(d.generation,1);
   jobs.push(grant(d.admission));
  }
  assert_eq!(owner.reserved().tasks,2);
  let mut sizes=Vec::new();
  for lease in jobs{
   loop{match owner.poll(&lease).unwrap(){Some(status)=>{assert!(status.success(),"owned compiler failed: {status}");break},None=>{assert!(begin.elapsed()<Duration::from_secs(20));thread::sleep(Duration::from_millis(20));}}}
   let path=out.join(format!("{}.rmeta",lease.task.id));
   let size=std::fs::metadata(path).unwrap().len();assert!(size>0);sizes.push(size);
   assert!(owner.settle(&lease,sampler.now_ms(),Some(Report{evidence:vec![format!("metadata-bytes:{size}")],observation:"owned compiler exited and artifact observed".into(),recommendation:"independent review".into()})).unwrap().unwrap().success());
  }
  assert_eq!(owner.reserved(),Reserved::default());assert_eq!(owner.counts(),(0,0,2,0));
  println!("OWNED_REAL_COMPILERS=2 POLICY_GENERATION=1 ARTIFACT_BYTES={sizes:?} ELAPSED_MS={}",begin.elapsed().as_millis());
 }
}
