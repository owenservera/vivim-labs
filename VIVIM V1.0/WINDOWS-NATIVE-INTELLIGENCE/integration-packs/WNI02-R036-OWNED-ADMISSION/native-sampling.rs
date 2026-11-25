//! Native sample adapter for SDK admission; uses the existing Windows probes.
//! @hq:invariant Unknown CPU, multi-group scope, reset and stale intervals never become zero.
use crate::{hardware,resources,scheduler::Snapshot};
use std::time::Instant;
#[derive(Debug,PartialEq,Eq)]
pub enum SampleError {Probe(resources::ProbeError),Warmup,TooSoon,UnsupportedGroups,InvalidInterval,Overflow}
impl From<resources::ProbeError> for SampleError{fn from(e:resources::ProbeError)->Self{Self::Probe(e)}}
pub struct NativeSampler {clock:Instant,previous:Option<(u64,resources::CpuTicks,hardware::Topology)>,revision:u64,drive:char}
impl NativeSampler {
 pub fn new(authorized_drive:char)->Self{Self{clock:Instant::now(),previous:None,revision:0,drive:authorized_drive}}
 pub fn now_ms(&self)->u64{self.clock.elapsed().as_millis().min(u64::MAX as u128) as u64}
 pub fn capture(&mut self)->Result<Snapshot,SampleError>{
  let result=self.capture_inner();
  if !matches!(result,Ok(_)|Err(SampleError::Warmup|SampleError::TooSoon)){self.previous=None;}
  result
 }
 fn capture_inner(&mut self)->Result<Snapshot,SampleError>{
  let topology=hardware::topology()?;
  if topology.processor_groups!=1{return Err(SampleError::UnsupportedGroups)}
  let now=self.now_ms();
  let ticks=resources::cpu()?;
  let Some((then,previous,old_topology))=self.previous else{self.previous=Some((now,ticks,topology));return Err(SampleError::Warmup)};
  let elapsed=now.checked_sub(then).ok_or(SampleError::InvalidInterval)?;
  if elapsed<100{return Err(SampleError::TooSoon)}
  self.previous=Some((now,ticks,topology));
  if elapsed>2000||topology!=old_topology{return Err(SampleError::Warmup)}
  let busy=resources::cpu_busy_basis_points(previous,ticks).ok_or(SampleError::InvalidInterval)?;
  let memory=resources::memory()?;
  let capacity=resources::capacity(self.drive)?;
  self.revision=self.revision.checked_add(1).ok_or(SampleError::Overflow)?;
  Ok(Snapshot{revision:self.revision,observed_ms:now,memory_available:memory.available_bytes,disk_available:capacity.available_bytes,logical_cpus:topology.active_logical_processors,cpu_busy_bp:Some(busy)})
 }
}
#[cfg(all(test,windows))]
mod tests{
 use super::*;
 use crate::{campaign::{Task,Report},resource_campaign::{Admission,Estimate,ResourceCampaign},scheduler::{Budget,Reserved}};
 use std::{path::PathBuf,process::{Child,Command},time::Duration};
 struct Owned(Child);
 impl Drop for Owned{fn drop(&mut self){let _=self.0.kill();let _=self.0.wait();}}
 #[test]fn live_native_snapshot_and_two_owned_rustc_jobs(){
  let cwd=PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let path=cwd.to_str().unwrap();let drive=path.strip_prefix(r"\\?\").unwrap_or(path).chars().next().unwrap();
  let mut sampler=NativeSampler::new(drive);
  assert_eq!(sampler.capture(),Err(SampleError::Warmup));
  std::thread::sleep(Duration::from_millis(180));
  let snapshot=sampler.capture().expect("current single-group native sample");
  assert!(snapshot.memory_available>0&&snapshot.disk_available>0&&snapshot.logical_cpus>=2);
  println!("NATIVE_SAMPLE revision={} logical_cpus={} cpu_busy_bp={} memory_available={} disk_available={}",snapshot.revision,snapshot.logical_cpus,snapshot.cpu_busy_bp.unwrap(),snapshot.memory_available,snapshot.disk_available);
  let tasks=["campaign","scheduler"].into_iter().map(|name|(Task{id:name.into(),source:format!("{name}.rs"),objective:"compile Rust metadata".into()},Estimate{memory:64*1024*1024,disk:8*1024*1024,cpus:1,priority:1,enqueued_ms:0})).collect();
  let budget=Budget{max_age_ms:5000,memory_reserve:128*1024*1024,disk_reserve:64*1024*1024,cpu_reserve:0,max_parallel:2,busy_ceiling_bp:10000};
  let mut campaign=ResourceCampaign::new(tasks,2,budget).unwrap();
  let mut children=Vec::new();
  for worker in ["compiler-a","compiler-b"]{
   let lease=match campaign.admit(worker,sampler.now_ms(),30000,snapshot).unwrap(){Admission::Granted(l)=>l,other=>panic!("native admission blocked: {other:?}")};
   let out=cwd.join(format!("{}.rmeta",lease.task.id));
   let child=Command::new("rustc").current_dir(&cwd).args(["--edition=2021","--crate-type=lib","--emit=metadata","--crate-name",&lease.task.id,&lease.task.source,"-o"]).arg(&out).spawn().expect("owned rustc child");
   children.push((lease,Owned(child),out));
  }
  assert_eq!(campaign.reserved().tasks,2);
  for (lease,mut child,out) in children{
   assert!(child.0.wait().unwrap().success());
   assert!(out.is_file());
   campaign.submit(&lease,sampler.now_ms(),Report{evidence:vec![lease.task.source.clone()],observation:"rustc exited successfully".into(),recommendation:"review metadata".into()}).unwrap();
   campaign.confirm_exit(&lease).unwrap();
   std::fs::remove_file(out).unwrap();
  }
  assert_eq!(campaign.reserved(),Reserved::default());
  assert_eq!(campaign.counts(),(0,0,2,0));
  println!("NATIVE_SDK_COMPILER_ADMISSION_OK");
 }
}
