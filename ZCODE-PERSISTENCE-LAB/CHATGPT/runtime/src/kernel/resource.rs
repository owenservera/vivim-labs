//! Host-observed process and artifact accounting. Unavailable OS counters stay
//! None, never zero. Wall time is not CPU time. No process enumeration or kill.
use super::model::*;
use crate::Result;
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::{fs,path::Path,process::Child};
#[derive(Clone,Debug,Serialize,Deserialize,Default)]
pub struct Counters{
    pub cpu_100ns:Option<u64>,pub peak_working_set_bytes:Option<u64>,
    pub os_read_bytes:Option<u64>,pub os_write_bytes:Option<u64>,
    pub work_bytes:Option<u64>,pub output_bytes:Option<u64>,pub checkpoint_bytes:Option<u64>,
    pub checkpoint_files:Option<u64>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Receipt{pub task:Id,pub generation:u64,pub process_ref:String,pub elapsed_ms:u64,pub counters:Counters}
fn size(p:&Path)->Option<u64>{fs::metadata(p).ok().filter(|m|m.is_file()).map(|m|m.len())}
pub fn observe(child:&Child,output:&Path)->Counters{
    let mut c=os_counters(child);
    if let Some(root)=output.parent(){
        c.work_bytes=size(&root.join("work.json"));c.output_bytes=size(output);
        if let Ok(entries)=fs::read_dir(root.join("progress")){
            let mut count=0u64;let mut bytes=0u64;let mut valid=true;
            for entry in entries{
                let Ok(entry)=entry else{valid=false;break;};
                if entry.path().extension().and_then(|s|s.to_str())!=Some("json"){continue;}
                let Some(n)=size(&entry.path()) else{valid=false;break;};
                count=count.saturating_add(1);bytes=bytes.saturating_add(n);
            }
            if valid{c.checkpoint_files=Some(count);c.checkpoint_bytes=Some(bytes);}
        }
    }c
}
#[cfg(not(windows))]
fn os_counters(_: &Child)->Counters{Counters::default()}
#[cfg(windows)]
fn os_counters(child:&Child)->Counters{
    use std::{ffi::c_void,os::windows::io::AsRawHandle,mem::size_of};
    #[repr(C)] #[derive(Default)] struct FileTime{low:u32,high:u32}
    #[repr(C)] #[derive(Default)] struct Io{read_ops:u64,write_ops:u64,other_ops:u64,read_bytes:u64,write_bytes:u64,other_bytes:u64}
    #[repr(C)] #[derive(Default)] struct Memory{cb:u32,page_faults:u32,peak_working_set:usize,working_set:usize,quota_peak_paged:usize,quota_paged:usize,quota_peak_nonpaged:usize,quota_nonpaged:usize,pagefile:usize,peak_pagefile:usize}
    #[link(name="kernel32")]
    unsafe extern "system"{
        fn GetProcessTimes(handle:*mut c_void,creation:*mut FileTime,exit:*mut FileTime,kernel:*mut FileTime,user:*mut FileTime)->i32;
        fn GetProcessIoCounters(handle:*mut c_void,counters:*mut Io)->i32;
        fn K32GetProcessMemoryInfo(handle:*mut c_void,counters:*mut Memory,size:u32)->i32;
    }
    let handle=child.as_raw_handle();let mut result=Counters::default();
    let (mut creation,mut exit,mut kernel,mut user)=(FileTime::default(),FileTime::default(),FileTime::default(),FileTime::default());
    let mut io=Io::default();let mut memory=Memory{cb:size_of::<Memory>() as u32,..Memory::default()};
    // Child owns the process handle for the entire call; all buffers have
    // the documented Win32 C layout and are exclusively borrowed.
    unsafe{
        if GetProcessTimes(handle,&mut creation,&mut exit,&mut kernel,&mut user)!=0{
            let ticks=|f:FileTime|(u64::from(f.high)<<32)|u64::from(f.low);
            result.cpu_100ns=ticks(kernel).checked_add(ticks(user));
        }
        if GetProcessIoCounters(handle,&mut io)!=0{result.os_read_bytes=Some(io.read_bytes);result.os_write_bytes=Some(io.write_bytes);}
        if K32GetProcessMemoryInfo(handle,&mut memory,size_of::<Memory>() as u32)!=0{result.peak_working_set_bytes=Some(memory.peak_working_set as u64);}
    }result
}
impl World{
    pub fn record_resources(&mut self,r:Receipt)->Result<()>{
        let key=format!("{}-{}",r.task,r.generation);
        let attempt=self.attempt_history.get(&key).ok_or("resource attempt missing")?;
        let agent=self.agents.get(&attempt.agent).ok_or("resource agent missing")?;
        ensure(agent.incarnations.iter().any(|i|i.number==attempt.incarnation&&i.executor=="native-pure-process"&&i.process_ref.as_ref()==Some(&r.process_ref)),"resource process identity mismatch")?;
        ensure(r.elapsed_ms<=3_600_000&&!self.resource_receipts.contains_key(&key),"resource receipt duplicate or elapsed overflow")?;
        self.resource_receipts.insert(key,r);Ok(())
    }
    pub fn resource_report(&self)->Value{
        let n=self.resource_receipts.len();
        let sum=|f:fn(&Counters)->Option<u64>|{
            let mut total=0u64;let mut measured=0usize;
            for r in self.resource_receipts.values(){if let Some(v)=f(&r.counters){total=total.saturating_add(v);measured+=1;}}
            json!({"sum":if measured==0{None}else{Some(total)},"measured_attempts":measured,"missing_attempts":self.attempt_history.len().saturating_sub(measured)})
        };
        let peak=self.resource_receipts.values().filter_map(|r|r.counters.peak_working_set_bytes).max();
        json!({"attempts":self.attempt_history.len(),"resource_receipts":n,"cpu_100ns":sum(|c|c.cpu_100ns),"os_read_bytes":sum(|c|c.os_read_bytes),"os_write_bytes":sum(|c|c.os_write_bytes),"work_bytes":sum(|c|c.work_bytes),"output_bytes":sum(|c|c.output_bytes),"checkpoint_bytes":sum(|c|c.checkpoint_bytes),"peak_working_set_bytes":peak,"search_costs":self.learning_costs,"host_operations":self.host_costs,"router_usage_records":self.router.reservations.len()})
    }
}


/// Measured host wall duration; does not conflate wall time with CPU or
/// model tokens. If the source mutation fails, result_digest remains absent.
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct HostCost{pub request:Id,pub operation:String,pub elapsed_ms:u64,pub result_digest:Option<Digest>}
impl World{
    pub fn record_host_cost(&mut self,r:HostCost)->Result<()>{
        id(&r.request)?;ensure(matches!(r.operation.as_str(),"learning"|"knowledge"|"recipe"|"repair"|"verify"|"routing"|"trial"),"unsupported metered operation")?;
        ensure(r.elapsed_ms<=259_200_000&&r.result_digest.as_ref().is_none_or(|h|hash_id(h))&&!self.host_costs.contains_key(&r.request),"invalid host cost receipt")?;
        self.host_costs.insert(r.request.clone(),r);Ok(())
    }
}
