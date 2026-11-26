mod fixture;mod resources;mod hardware;mod scheduler;mod winml;
use std::{mem::{size_of,zeroed},time::Instant};
use windows::AI::MachineLearning::LearningModelDeviceKind;
use windows_sys::Win32::System::{Threading::GetCurrentProcess,ProcessStatus::{GetProcessMemoryInfo,PROCESS_MEMORY_COUNTERS,PROCESS_MEMORY_COUNTERS_EX}};
fn memory()->Result<(usize,usize),String>{
 let mut m:PROCESS_MEMORY_COUNTERS_EX=unsafe{zeroed()};m.cb=size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32;
 if unsafe{GetProcessMemoryInfo(GetCurrentProcess(),(&mut m as *mut PROCESS_MEMORY_COUNTERS_EX).cast::<PROCESS_MEMORY_COUNTERS>(),m.cb)}==0{return Err("MEMORY_QUERY_FAILED".into())}
 Ok((m.PeakWorkingSetSize,m.PeakPagefileUsage))
}
fn main(){
 let mode=std::env::args().nth(1).unwrap_or("--cpu".into());let cwd=std::env::current_dir().unwrap();
 if mode=="--reference"{
  let start=Instant::now();let mut scores=[0.;4];for _ in 0..100{scores=std::hint::black_box(fixture::reference_input(std::hint::black_box(&fixture::FEATURES)));}
  let elapsed=start.elapsed().as_micros();let(ws,commit)=memory().unwrap();
  assert_eq!(scores,fixture::reference());
  println!("WINML_BENCH_REFERENCE runs=100 elapsed_us={elapsed} peak_working_set={ws} peak_pagefile={commit} scores={scores:?}");return
 }
 let(kind,name)=match mode.as_str(){"--cpu"=>(LearningModelDeviceKind::Cpu,"cpu"),"--directx"=>(LearningModelDeviceKind::DirectX,"directx"),_=>{eprintln!("INVALID_MODE");std::process::exit(2)}};
 match winml::evaluate_owned_fixture(&cwd,kind){
  Ok(e)=>{let(ws,commit)=memory().unwrap();println!("WINML_BENCH_ACTUAL device={name} peak_working_set={ws} peak_pagefile={commit} {e:?}");},
  Err(e)=>{eprintln!("WINML_BENCH_UNAVAILABLE device={name} {e:?}");std::process::exit(2)}
 }
}
