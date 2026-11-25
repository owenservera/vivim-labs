//! @hq:context Real native compiler feedback through the incumbent owned process and Meta-VLANG policy.
use crate::{campaign::{Task,Report,Lease},protocol::Snapshot as Language,meta_resident::Resident,meta_policy,resource_campaign::{ResourceCampaign,Estimate,Admission},scheduler::{Budget,Reserved},owned_campaign::OwnedCampaign,native_sampling::{NativeSampler,SampleError},workstation_process::LocalProcessSpec,workload_profile::{Profiles,Key,Measure,Estimate as Forecast}};
use std::{path::PathBuf,thread,time::{Duration,Instant}};
const MIB:u64=1024*1024;
fn runtime()->Resident{
 let params="((cpus int) (busy int) (active int) (memory int) (disk int))";
 let rows=[(meta_policy::PARALLEL,2),(meta_policy::BUSY,10000),(meta_policy::MEMORY,128),(meta_policy::DISK,64)].map(|(id,v)|format!("{id}\t1\t\tmeta/1 (fn {params} int (int {v}))"));
 Resident::start(Language::parse(&rows.join("\n")).unwrap()).unwrap()
}
fn grant(a:Admission)->Lease{match a{Admission::Granted(l)=>l,x=>panic!("{x:?}")}}
#[test]fn actual_native_compiler_job_measures_and_reuses_workload(){
 let exe=PathBuf::from(std::env::var_os("VIVIM_RUSTC_EXE").expect("pinned local rustc"));
 let key=Key{source_digest:std::env::var("VIVIM_CAMPAIGN_SHA256").unwrap(),toolchain_digest:std::env::var("VIVIM_RUSTC_SHA256").unwrap(),operation:"rustc.metadata".into()};
 let cwd=PathBuf::from(env!("CARGO_MANIFEST_DIR"));let artifact=cwd.join("r038-owned-campaign.rmeta");
 let path=cwd.to_str().unwrap();let drive=path.strip_prefix(r"\\?\").unwrap_or(path).chars().next().unwrap();
 let mut s=NativeSampler::new(drive);assert_eq!(s.capture(),Err(SampleError::Warmup));thread::sleep(Duration::from_millis(180));let sample=s.capture().unwrap();
 let mut r=runtime();let mut profiles=Profiles::new(4,4,16).unwrap();
 let floor=Forecast{memory:64*MIB,disk:8*MIB,elapsed_ms:1,samples:0};
 let mut first_peak=0;
 for round in 1..=2{
  let estimate=profiles.estimate(&key,floor).unwrap();
  if round==2{assert!(estimate.memory>=first_peak);assert_eq!(estimate.samples,1);}
  let task=Task{id:"compiler".into(),source:"campaign.rs".into(),objective:"compile metadata".into()};
  let b=Budget{max_age_ms:5000,memory_reserve:128*MIB,disk_reserve:64*MIB,cpu_reserve:0,max_parallel:2,busy_ceiling_bp:10000};
  let cost=Estimate{memory:estimate.memory,disk:estimate.disk,cpus:1,priority:1,enqueued_ms:0};
  let mut c=OwnedCampaign::new(ResourceCampaign::new(vec![(task,cost)],2,b).unwrap()).unwrap();
  let d=c.start_policy(&mut r,"compiler",s.now_ms(),30000,sample,|_|{
   let environment=["SystemRoot","WINDIR","PATH","TEMP","TMP"].iter().filter_map(|n|std::env::var_os(n).map(|v|((*n).into(),v))).collect();
   Some(LocalProcessSpec{executable:exe.clone(),workspace:cwd.clone(),args:vec!["--edition=2021".into(),"--crate-type=lib".into(),"--emit=metadata".into(),"--crate-name".into(),"campaign".into(),"campaign.rs".into(),"-o".into(),artifact.as_os_str().into()],environment,tail_bytes:4096})
  }).unwrap();assert_eq!(d.generation,r.generation());let l=grant(d.admission);
  assert_eq!(c.reserved().memory,estimate.memory);
  let start=Instant::now();
  let done=loop{
   if let Some(x)=c.settle_profiled(&l,s.now_ms(),Some(Report{evidence:vec!["owned-job-exit".into()],observation:"verified compiler exit".into(),recommendation:"review".into()})).unwrap(){break x}
   assert!(start.elapsed()<Duration::from_secs(10));thread::sleep(Duration::from_millis(10));
  };
  assert!(done.status.success());assert_eq!(c.reserved(),Reserved::default());assert_eq!(c.counts().2,1);
  let m=done.metrics.unwrap();assert_eq!(m.active_processes,0);assert!(m.peak_job_commit_bytes>0&&m.total_processes>=1);
  let bytes=std::fs::metadata(&artifact).unwrap().len();assert!(bytes>0);
  profiles.record(round,key.clone(),Measure{elapsed_ms:done.elapsed_ms,peak_memory:Some(m.peak_job_commit_bytes),artifact_bytes:Some(bytes)}).unwrap();
  let next=profiles.estimate(&key,floor).unwrap();if round==1{first_peak=m.peak_job_commit_bytes;}
  println!("OWNED_COMPILER_PROFILE_OK round={} elapsed_ms={} peak_job_commit_bytes={} artifact_bytes={} user_100ns={} kernel_100ns={} reserved_memory={} next_memory={} next_elapsed_ms={}",round,done.elapsed_ms,m.peak_job_commit_bytes,bytes,m.user_100ns,m.kernel_100ns,estimate.memory,next.memory,next.elapsed_ms);
  std::fs::remove_file(&artifact).unwrap();
 }
}
