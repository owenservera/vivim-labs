//! Host-observed process and artifact accounting. Unavailable OS counters stay
//! None, never zero. Wall time is not CPU time. No process enumeration or kill.
use super::model::*;
use crate::Result;
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::{collections::BTreeMap,fs,path::Path,process::Child};
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
        let native_attempts=self.attempt_history.values().filter(|a|self.agents.get(&a.agent).is_some_and(|agent|agent.incarnations.iter().any(|i|i.number==a.incarnation&&i.executor=="native-pure-process"))).count();
        let sum=|f:fn(&Counters)->Option<u64>|{
            let mut total=Some(0u64);let mut measured=0usize;
            for r in self.resource_receipts.values(){if let Some(v)=f(&r.counters){total=total.and_then(|n|n.checked_add(v));measured+=1;}}
            json!({"sum":if measured==0{None}else{total},"overflow":total.is_none(),"measured_attempts":measured,"missing_native_attempts":native_attempts.saturating_sub(measured)})
        };
        let peak=self.resource_receipts.values().filter_map(|r|r.counters.peak_working_set_bytes).max();
        json!({"attempts":self.attempt_history.len(),"native_attempts":native_attempts,"resource_receipts":n,"cpu_100ns":sum(|c|c.cpu_100ns),"os_read_bytes":sum(|c|c.os_read_bytes),"os_write_bytes":sum(|c|c.os_write_bytes),"work_bytes":sum(|c|c.work_bytes),"output_bytes":sum(|c|c.output_bytes),"checkpoint_bytes":sum(|c|c.checkpoint_bytes),"peak_working_set_bytes":peak,"search_costs":self.learning_costs,"host_operations":self.host_costs,"router_usage_records":self.router.reservations.len(),"provider_accounting":self.provider_accounting()})
    }
}


/// Legacy operation labels denote end-to-end host wall duration; transition-*
/// labels denote atomic pre-persistence transition wall duration. Never combine
/// the two as an exact total, or infer failed/uncertain action cost from absence.
pub(crate) fn measured_operation(action:&super::transition::Action)->Option<&'static str>{
    use super::transition::Action::*;
    match action{
        LearnPlan{..}=>Some("learning"),
        AssimilateRecurring{..}=>Some("knowledge"),
        DiscoverRecipes{..}|FreezeTracePattern{..}|PromoteTracePattern{..}=>Some("recipe"),
        ApplyLearnedRepair{..}=>Some("repair"),
        Verify{..}=>Some("verify"),
        RoutingCycle{..}|GenerateDemands{..}=>Some("routing"),
        CloseRoutingArm|StartRoutingCandidate|AbortRoutingTrial{..}|PromoteRoutingTrial{..}|RollbackRoutingTrial{..}|BeginRoutingTrial{..}=>Some("trial"),
        _=>None,
    }
}
/// Measured host wall duration; does not conflate wall time with CPU or tokens.
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct HostCost{pub request:Id,pub operation:String,pub elapsed_ms:u64,pub result_digest:Option<Digest>}
impl World{
    pub fn record_host_cost(&mut self,r:HostCost)->Result<()>{
        id(&r.request)?;let label=r.operation.strip_prefix("transition-").unwrap_or(&r.operation);
        ensure(matches!(label,"learning"|"knowledge"|"recipe"|"repair"|"verify"|"routing"|"trial"),"unsupported metered operation")?;
        ensure(r.result_digest.as_ref().is_none_or(|h|hash_id(h))&&!self.host_costs.contains_key(&r.request),"invalid host cost receipt")?;
        self.host_costs.insert(r.request.clone(),r);Ok(())
    }
}

#[derive(Default,Serialize)]
struct UsageAggregate{
    receipts:u64,verified:u64,requests:Option<u64>,input:Option<u64>,output:Option<u64>,reasoning:Option<u64>,credits:Option<u64>,
}
impl UsageAggregate{
    fn new()->Self{Self{receipts:0,verified:0,requests:Some(0),input:Some(0),output:Some(0),reasoning:Some(0),credits:Some(0)}}
    fn add(&mut self,r:&super::routing_adapter::AdapterReceipt,verified:bool){
        let plus=|v:Option<u64>,n:u64|v.and_then(|v|v.checked_add(n));
        self.receipts=self.receipts.saturating_add(1);self.verified=self.verified.saturating_add(u64::from(verified));
        self.requests=plus(self.requests,r.usage.requests);self.input=plus(self.input,r.usage.input);
        self.output=plus(self.output,r.usage.output);self.reasoning=plus(self.reasoning,r.usage.reasoning);self.credits=plus(self.credits,r.usage.credits);
    }
}
impl World{
    pub fn provider_accounting(&self)->Value{
        use super::routing::Status;
        let mut real=UsageAggregate::new();let mut synthetic=UsageAggregate::new();
        let mut profiles=BTreeMap::<Id,UsageAggregate>::new();
        let mut uncovered=vec![];let mut charge_only=vec![];
        let mut pools=BTreeMap::<Id,Option<u64>>::new();let mut holds=BTreeMap::<Id,Option<u64>>::new();
        for r in self.router.reservations.values(){
            if matches!(r.status,Status::Reserved|Status::Dispatched|Status::Unknown){
                for (key,n) in &r.holds{let v=holds.entry(key.clone()).or_insert(Some(0));*v=v.and_then(|v|v.checked_add(*n));}
            }
            if r.status==Status::Settled{
                for (key,n) in &r.actual{let v=pools.entry(key.clone()).or_insert(Some(0));*v=v.and_then(|v|v.checked_add(*n));}
            }
            if r.started_ms.is_none(){continue;}
            if let Some(receipt)=self.router.adapters.receipts.get(&r.id){
                let a=if receipt.provider.starts_with("synthetic-"){&mut synthetic}else{&mut real};
                a.add(receipt,r.verified==Some(true));
                profiles.entry(r.profile.id.clone()).or_insert_with(UsageAggregate::new).add(receipt,r.verified==Some(true));
            }else{
                uncovered.push(r.id.clone());
                // Previous charge-only records were stored by content digest, not
                // in the raw usage receipt map. Recover their scope without inventing tokens.
                let evidence=self.router.adapters.handoffs.get(&r.id).and_then(|e|e.receipt.as_ref()).and_then(|h|self.blobs.get(h));
                if let Some(c)=evidence.and_then(|v|serde_json::from_value::<super::routing_adapter::ChargeOnlyReceipt>(v.clone()).ok()){
                    if c.handoff==r.id&&c.actual_by_reserved_pool==r.actual&&r.receipt.as_deref()==Some(c.provider_receipt.as_str()){
                        charge_only.push(c);
                    }
                }
            }
        }
        json!({"confirmed_real":real,"confirmed_synthetic":synthetic,"confirmed_by_profile":profiles,
            "uncovered_dispatch_ids":uncovered,"all_raw_usage_covered":uncovered.is_empty(),"charge_only_receipts":charge_only,
            "lifetime_settled_by_pool":pools,"active_holds_by_pool":holds,
            "pool_totals_are_overlapping_not_additive":true,"null_total_means_overflow":true})
    }
}
