//! @hq:context Isolated ETW file session for VIVIM's numeric SDK work events only.
//! @hq:invariant Enable exactly the existing owned provider and current PID; no kernel/system providers.
//! @hq:invariant Never stop an existing session by name, never elevate, never infer successful delivery.
//! @hq:why Measure compiler work without retaining prompts, paths or private content.
use crate::{etw::{self,Event},etw_decode::{Correlator,Source},etw_record};
use std::{io,mem::{size_of,offset_of,zeroed},path::Path,os::windows::ffi::OsStrExt,ptr::{null,null_mut}};
use windows_sys::{core::GUID,Win32::{Foundation::{FILETIME,GetLastError},System::{Threading::{GetCurrentProcess,GetProcessTimes},Diagnostics::Etw::*}}};
const GUID_OWNED:GUID=GUID::from_u128(0x69aa8a70_95c5_4a31_b188_9c8917890fd2);
const NAME:&str="VIVIM-Labs-SDK-ETW-R041";
#[derive(Debug)]pub enum Error{Invalid,Os{stage:&'static str,code:u32},Provider(etw::Error),Correlation(crate::etw_decode::Error),Loss,Io(io::Error)}
fn status(stage:&'static str,code:u32)->Result<(),Error>{if code==0{Ok(())}else{Err(Error::Os{stage,code})}}
fn wide(s:&std::ffi::OsStr)->Result<Vec<u16>,Error>{let mut v:Vec<u16>=s.encode_wide().collect();if v.contains(&0)||v.len()>=1024{return Err(Error::Invalid)}v.push(0);Ok(v)}
fn same(a:&GUID,b:&GUID)->bool{a.data1==b.data1&&a.data2==b.data2&&a.data3==b.data3&&a.data4==b.data4}
#[repr(C)]
struct Properties{inner:EVENT_TRACE_PROPERTIES,name:[u16;128],file:[u16;1024]}
impl Properties{
 fn new(file:&Path)->Result<Box<Self>,Error>{
  if !file.is_absolute()||file.exists(){return Err(Error::Invalid)}
  let path=wide(file.as_os_str())?;let name:Vec<u16>=NAME.encode_utf16().chain(Some(0)).collect();
  let mut p:Box<Self>=Box::new(unsafe{zeroed()});
  p.name[..name.len()].copy_from_slice(&name);p.file[..path.len()].copy_from_slice(&path);
  p.inner.Wnode.BufferSize=size_of::<Self>() as u32;p.inner.Wnode.Flags=WNODE_FLAG_TRACED_GUID;p.inner.Wnode.ClientContext=1;
  p.inner.BufferSize=64;p.inner.MinimumBuffers=2;p.inner.MaximumBuffers=16;p.inner.MaximumFileSize=8;
  p.inner.LogFileMode=EVENT_TRACE_FILE_MODE_SEQUENTIAL;p.inner.FlushTimer=1;
  p.inner.LoggerNameOffset=offset_of!(Self,name) as u32;p.inner.LogFileNameOffset=offset_of!(Self,file) as u32;
  Ok(p)
 }
}
struct Trace{handle:CONTROLTRACE_HANDLE,props:Box<Properties>,live:bool}
impl Trace{
 fn start(path:&Path,pid:u32)->Result<Self,Error>{
  let mut props=Properties::new(path)?;let mut handle=CONTROLTRACE_HANDLE{Value:0};
  status("StartTraceW",unsafe{StartTraceW(&mut handle,props.name.as_ptr(),&mut props.inner)})?;
  let trace=Self{handle,props,live:true};
  let mut pid=pid;
  let mut filter=EVENT_FILTER_DESCRIPTOR{Ptr:(&mut pid as *mut u32) as usize as u64,Size:4,Type:EVENT_FILTER_TYPE_PID};
  let mut params:ENABLE_TRACE_PARAMETERS=unsafe{zeroed()};params.Version=2;params.EnableFilterDesc=&mut filter;params.FilterDescCount=1;
  status("EnableTraceEx2",unsafe{EnableTraceEx2(handle,&GUID_OWNED,EVENT_CONTROL_CODE_ENABLE_PROVIDER,4,1,0,5000,&params)})?;
  Ok(trace)
 }
 fn stop(&mut self)->Result<(),Error>{
  if !self.live{return Err(Error::Invalid)}
  let r=unsafe{ControlTraceW(self.handle,null(),&mut self.props.inner,EVENT_TRACE_CONTROL_STOP)};
  status("ControlTraceW",r)?;self.live=false;
  if self.props.inner.EventsLost!=0||self.props.inner.LogBuffersLost!=0||self.props.inner.RealTimeBuffersLost!=0{return Err(Error::Loss)}Ok(())
 }
}
impl Drop for Trace{fn drop(&mut self){if self.live{let _=unsafe{ControlTraceW(self.handle,null(),&mut self.props.inner,EVENT_TRACE_CONTROL_STOP)};}}}
fn current_source(generation:u64)->Result<Source,Error>{
 let(mut created,mut exited,mut kernel,mut user):(FILETIME,FILETIME,FILETIME,FILETIME)=unsafe{zeroed()};
 if unsafe{GetProcessTimes(GetCurrentProcess(),&mut created,&mut exited,&mut kernel,&mut user)}==0{return Err(Error::Os{stage:"GetProcessTimes",code:unsafe{GetLastError()}})}
 let creation_ticks=(u64::from(created.dwHighDateTime)<<32)|u64::from(created.dwLowDateTime);
 Ok(Source{process_id:std::process::id(),creation_ticks,generation})
}
struct ReadState{source:Source,correlator:Correlator,events:Vec<Event>,error:Option<Error>}
unsafe extern "system" fn event_callback(record:*mut EVENT_RECORD){
 if record.is_null(){return}
 let r=unsafe{&*record};if r.UserContext.is_null(){return}
 let state=unsafe{&mut *r.UserContext.cast::<ReadState>()};
 if state.error.is_some()||!same(&r.EventHeader.ProviderId,&GUID_OWNED){return}
 if state.events.len()>=64{state.error=Some(Error::Loss);return}
 match unsafe{etw_record::ingest(r,state.source,&mut state.correlator)}{Ok(e)=>state.events.push(e),Err(e)=>state.error=Some(Error::Correlation(e))}
}
fn consume(path:&Path,source:Source)->Result<Vec<Event>,Error>{
 let mut state=ReadState{source,correlator:Correlator::new(source).map_err(Error::Correlation)?,events:Vec::new(),error:None};
 let mut file=wide(path.as_os_str())?;let mut log:EVENT_TRACE_LOGFILEW=unsafe{zeroed()};
 log.LogFileName=file.as_mut_ptr();log.Anonymous1.ProcessTraceMode=PROCESS_TRACE_MODE_EVENT_RECORD;
 log.Anonymous2.EventRecordCallback=Some(event_callback);log.Context=(&mut state as *mut ReadState).cast();
 let handle=unsafe{OpenTraceW(&mut log)};
 if handle.Value==u64::MAX{return Err(Error::Os{stage:"OpenTraceW",code:unsafe{GetLastError()}})}
 let read=unsafe{ProcessTrace(&handle,1,null(),null())};let close=unsafe{CloseTrace(handle)};
 status("ProcessTrace",read)?;status("CloseTrace",close)?;
 if log.EventsLost!=0||log.LogfileHeader.BuffersLost!=0{return Err(Error::Loss)}
 if let Some(e)=state.error{return Err(e)}
 Ok(state.events)
}
/// @hq:invariant One short-lived owned file trace, bounded numeric payload and verified same-process correlation.
/// The work closure runs under the existing caller's authority. The trace is not a receipt of the work.
pub fn capture<T>(file:&Path,generation:u64,work:impl FnOnce(&etw::Provider)->Result<T,Error>)->Result<(T,Vec<Event>),Error>{
 let source=current_source(generation)?;
 let provider=etw::Provider::register().map_err(Error::Provider)?;
 let mut trace=Trace::start(file,source.process_id)?;
 let result=work(&provider)?;
 trace.stop()?;
 let events=consume(file,source)?;
 Ok((result,events))
}
#[cfg(test)]mod tests{
 use super::*;
 use crate::{etw::Kind,workstation_process::{OwnedProcess,LocalProcessSpec}};
 use std::{path::PathBuf,time::{Duration,Instant}};
 #[test]fn actual_owned_etw_compiler_events_are_consumed(){
  let cwd=PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let file=cwd.join("r041-sdk.etl");let output=cwd.join("r041-campaign.rmeta");
  assert!(!file.exists()&&!output.exists());
  let exe=PathBuf::from(std::env::var_os("VIVIM_RUSTC_EXE").expect("actual rustc"));
  let generation=1;
  let (duration,events)=capture(&file,generation,|p|{
   p.emit(Event{kind:Kind::TaskStarted,generation,sequence:1,duration_ms:0,count:1}).map_err(Error::Provider)?;
   let start=Instant::now();let environment=["SystemRoot","WINDIR","PATH","TEMP","TMP"].iter().filter_map(|n|std::env::var_os(n).map(|v|((*n).into(),v))).collect();
   let mut child=OwnedProcess::start(LocalProcessSpec{executable:exe,workspace:cwd.clone(),args:vec!["--edition=2021".into(),"--crate-type=lib".into(),"--emit=metadata".into(),"--crate-name".into(),"campaign".into(),"campaign.rs".into(),"-o".into(),output.as_os_str().into()],environment,tail_bytes:4096}).map_err(Error::Io)?;
   let status=child.wait_for(Duration::from_secs(10)).map_err(Error::Io)?.ok_or(Error::Invalid)?;
   if !status.success(){return Err(Error::Invalid)}
   let duration=u32::try_from(start.elapsed().as_millis()).map_err(|_|Error::Invalid)?;
   p.emit(Event{kind:Kind::TaskFinished,generation,sequence:2,duration_ms:duration,count:1}).map_err(Error::Provider)?;
   Ok(duration)
  }).expect("owned ETW capture");
  assert_eq!(events,vec![Event{kind:Kind::TaskStarted,generation,sequence:1,duration_ms:0,count:1},Event{kind:Kind::TaskFinished,generation,sequence:2,duration_ms:duration,count:1}]);
  assert!(std::fs::metadata(&output).unwrap().len()>0);
  println!("ETW_ACTUAL_COMPILER_ROUNDTRIP_OK events={} duration_ms={} etl_bytes={} artifact_bytes={}",events.len(),duration,std::fs::metadata(&file).unwrap().len(),std::fs::metadata(&output).unwrap().len());
  std::fs::remove_file(file).unwrap();std::fs::remove_file(output).unwrap();
 }
}
