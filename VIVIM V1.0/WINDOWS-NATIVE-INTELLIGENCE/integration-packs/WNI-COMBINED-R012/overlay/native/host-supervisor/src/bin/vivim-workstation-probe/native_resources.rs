//! Aggregate resource observation, no process inventory, file access or effects.
use serde_json::json;
#[derive(Clone,Copy)]
struct Ticks {idle:u64,kernel:u64,user:u64}
fn busy(a:Ticks,b:Ticks)->Option<u16>{
 let idle=b.idle.checked_sub(a.idle)?;
 let k=b.kernel.checked_sub(a.kernel)?;
 let u=b.user.checked_sub(a.user)?;
 let total=k.checked_add(u)?;
 if total==0||idle>k{return None;}
 Some((((total-idle) as u128*10000)/total as u128) as u16)
}
#[cfg(windows)]
fn snapshot()->Result<serde_json::Value,()>{
 use std::{time::{Duration,Instant},thread};
 use windows_sys::Win32::{Foundation::FILETIME,System::{SystemInformation::{GlobalMemoryStatusEx,MEMORYSTATUSEX},Threading::GetSystemTimes}};
 fn ticks()->Result<Ticks,()>{
  let mut i:FILETIME=unsafe{std::mem::zeroed()};let mut k=i;let mut u=i;
  if unsafe{GetSystemTimes(&mut i,&mut k,&mut u)}==0{return Err(());}
  let n=|x:FILETIME|(u64::from(x.dwHighDateTime)<<32)|u64::from(x.dwLowDateTime);
  Ok(Ticks{idle:n(i),kernel:n(k),user:n(u)})
 }
 let start=Instant::now();
 let a=ticks()?;thread::sleep(Duration::from_millis(120));let b=ticks()?;
 let elapsed=u64::try_from(start.elapsed().as_millis()).map_err(|_|())?;
 let mut m:MEMORYSTATUSEX=unsafe{std::mem::zeroed()};
 m.dwLength=std::mem::size_of::<MEMORYSTATUSEX>() as u32;
 if unsafe{GlobalMemoryStatusEx(&mut m)}==0||m.ullTotalPhys==0||m.ullAvailPhys>m.ullTotalPhys{return Err(());}
 Ok(json!({"schema":"vivim.windows.resource-probe/1","status":"ok","memoryTotalBytes":m.ullTotalPhys,"memoryAvailableBytes":m.ullAvailPhys,"cpuBusyBasisPoints":busy(a,b),"cpuScope":"calling-processor-group","sampleMs":elapsed}))
}
#[cfg(not(windows))]
fn snapshot()->Result<serde_json::Value,()>{Ok(json!({"schema":"vivim.windows.resource-probe/1","status":"unsupported"}))}
pub fn print_snapshot()->Result<(),()>{println!("{}",snapshot()?);Ok(())}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn cpu_delta(){let a=Ticks{idle:10,kernel:20,user:10};let b=Ticks{idle:20,kernel:40,user:30};assert_eq!(busy(a,b),Some(7500));assert_eq!(busy(b,a),None);assert_eq!(busy(a,a),None);}
 #[cfg(windows)]#[test]fn native_snapshot(){let s=snapshot().unwrap();assert_eq!(s["status"],"ok");assert!(s["memoryTotalBytes"].as_u64().unwrap()>0);}
}
