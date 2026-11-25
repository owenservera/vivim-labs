//! VIVIM-owned ETW producer only, not a system-wide collector.
//! Closed numeric schema; no paths, command lines, messages or private text.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Kind{TaskStarted,TaskFinished,QueueGap,ProbeFailed}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct Event{pub kind:Kind,pub generation:u64,pub sequence:u64,pub duration_ms:u32,pub count:u32}
impl Event{
 pub fn wire(self)->String{
  let kind=match self.kind{Kind::TaskStarted=>"task_started",Kind::TaskFinished=>"task_finished",Kind::QueueGap=>"queue_gap",Kind::ProbeFailed=>"probe_failed"};
  format!("vivim.native.observation/1;kind={kind};generation={};sequence={};duration_ms={};count={}",self.generation,self.sequence,self.duration_ms,self.count)
 }
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Error{Unsupported,Os(u32),InvalidEvent}
#[cfg(windows)]
mod win{
 use super::*;
 use std::ptr::null;
 use windows_sys::{core::GUID,Win32::System::Diagnostics::Etw::{EventRegister,EventWriteString,EventUnregister,REGHANDLE}};
 // Labs experimental provider identity; reserve canonical GUID with ARBOR before shipping.
 const PROVIDER:GUID=GUID::from_u128(0x69aa8a70_95c5_4a31_b188_9c8917890fd2);
 pub struct Provider{handle:REGHANDLE}
 impl Provider{
  pub fn register()->Result<Self,Error>{let mut handle=0;let r=unsafe{EventRegister(&PROVIDER,None,null(),&mut handle)};if r!=0{Err(Error::Os(r))}else{Ok(Self{handle})}}
  /// Successful submission does not establish that any consumer received it.
  pub fn emit(&self,event:Event)->Result<(),Error>{
   if event.generation==0||event.sequence==0{return Err(Error::InvalidEvent);}
   let text:Vec<u16>=event.wire().encode_utf16().chain(Some(0)).collect();
   let r=unsafe{EventWriteString(self.handle,4,1,text.as_ptr())};
   if r!=0{Err(Error::Os(r))}else{Ok(())}
  }
 }
 impl Drop for Provider{fn drop(&mut self){unsafe{EventUnregister(self.handle);}}}
 #[cfg(test)]mod tests{use super::*;#[test]fn owned_provider_write(){let p=Provider::register().unwrap();p.emit(Event{kind:Kind::TaskFinished,generation:1,sequence:1,duration_ms:20,count:1}).unwrap();}}
}
#[cfg(windows)]pub use win::Provider;
#[cfg(test)]mod tests{use super::*;#[test]fn closed_payload(){assert_eq!(Event{kind:Kind::QueueGap,generation:2,sequence:3,duration_ms:0,count:5}.wire(),"vivim.native.observation/1;kind=queue_gap;generation=2;sequence=3;duration_ms=0;count=5");}}
