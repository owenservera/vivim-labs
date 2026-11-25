//! @hq:context ACP sessions over the existing Windows owned-process lifecycle and lossless protocol reducer.
//! @hq:invariant A missing stdout byte, failed input queue or incomplete EOF poisons the connection; no automatic replay.
//! @hq:why Reuse the Host job owner, distinguish protocol settlement from whole-job exit and never grant agent permissions.
use crate::{acp::Event,acp_transport::{Transport,Error as ProtocolError},workstation_process::{OwnedProcess,LocalProcessSpec}};
use serde_json::Value;
use std::{io,process::ExitStatus};
#[derive(Debug)]pub enum Error{Io(io::Error),Protocol(ProtocolError),Poisoned,Gap,FrameLimit}
impl From<io::Error> for Error{fn from(e:io::Error)->Self{Self::Io(e)}}
impl From<ProtocolError> for Error{fn from(e:ProtocolError)->Self{Self::Protocol(e)}}
#[derive(Debug)]pub struct Update{pub events:Vec<Event>,pub stdout_cursor:u64,pub exit:Option<ExitStatus>,pub eof:bool}
pub struct OwnedAcp{process:OwnedProcess,transport:Transport,cursor:u64,poisoned:bool,eof:bool}
impl OwnedAcp{
 /// Only an already-authorized exact local process spec may enter this adapter.
 pub fn start(spec:LocalProcessSpec,generation:u64)->Result<Self,Error>{
  let transport=Transport::new(generation,65536)?;
  let process=OwnedProcess::start_with_input(spec,true)?;
  Ok(Self{process,transport,cursor:0,poisoned:false,eof:false})
 }
 fn open(&self)->Result<(),Error>{if self.poisoned||self.eof{Err(Error::Poisoned)}else{Ok(())}}
 fn poison<T>(&mut self,e:Error)->Result<T,Error>{self.poisoned=true;self.process.close_input();Err(e)}
 fn send(&mut self,v:Value)->Result<(),Error>{
  self.open()?;
  let mut b=match serde_json::to_vec(&v){Ok(b)=>b,Err(_)=>return self.poison(Error::FrameLimit)};b.push(b'\n');
  // Existing input owner accepts a single bounded chunk. Never split a request into a partially queued frame.
  if b.len()>4096{return self.poison(Error::FrameLimit)}
  if let Err(e)=self.process.send_input(b){return self.poison(Error::Io(e))}Ok(())
 }
 pub fn initialize(&mut self)->Result<(),Error>{self.open()?;let q=self.transport.initialize()?;self.send(q)}
 pub fn new_session(&mut self,approved_cwd:&str)->Result<(),Error>{self.open()?;let q=self.transport.new_session(approved_cwd)?;self.send(q)}
 pub fn prompt(&mut self,session:&str,approved_text:&str)->Result<(),Error>{
  self.open()?;
  if approved_text.len()>512{return Err(Error::FrameLimit)} // conservative bound for JSON escaping
  let q=self.transport.prompt(session,approved_text)?;self.send(q)
 }
 pub fn cancel_prompt(&mut self,session:&str)->Result<(),Error>{self.open()?;let q=self.transport.cancel(session)?;self.send(q)}
 pub fn close_input(&mut self){self.process.close_input()}
 pub fn cancel_owned(&mut self)->Result<ExitStatus,Error>{self.poisoned=true;Ok(self.process.cancel()?)}
 /// Protocol EOF is independent of job exit. Never feed stderr into the transport.
 pub fn poll(&mut self)->Result<Update,Error>{
  if self.poisoned{return Err(Error::Poisoned)}
  let (out,_stderr)=match self.process.output(){Ok(o)=>o,Err(e)=>return self.poison(Error::Io(e))};
  let Some(first)=out.observed_bytes.checked_sub(out.bytes.len() as u64)else{return self.poison(Error::Gap)};
  if out.read_failed||self.cursor<first||self.cursor>out.observed_bytes{return self.poison(Error::Gap)}
  let mut events=Vec::new();
  if self.cursor<out.observed_bytes{
   let start=(self.cursor-first) as usize;
   let batch=match self.transport.ingest(self.transport.generation(),self.cursor,&out.bytes[start..]){Ok(b)=>b,Err(e)=>return self.poison(Error::Protocol(e))};
   self.cursor=batch.acknowledged_through;
   // Existing router declines permission and unsupported client calls; nothing is auto-approved.
   for e in &batch.events{
    match e{Event::PermissionCancelled{response,..}|Event::UnsupportedRequest{response}=>self.send(response.clone())?,_=>{}}
   }
   events=batch.events;
  }
  if out.closed&&!self.eof{
   if let Err(e)=self.transport.eof(self.transport.generation()){return self.poison(Error::Protocol(e))}
   self.eof=true;
  }
  let exit=match self.process.poll(){Ok(x)=>x,Err(e)=>return self.poison(Error::Io(e))};
  Ok(Update{events,stdout_cursor:self.cursor,exit,eof:self.eof})
 }
}
#[cfg(test)]mod tests{
 use super::*;use std::{thread,time::{Duration,Instant}};
 fn fixture()->LocalProcessSpec{
  let exe=std::env::var_os("VIVIM_ACP_FIXTURE_EXE").expect("compiled standalone ACP fixture");
  let mut environment=Vec::new();if let Some(v)=std::env::var_os("SystemRoot"){environment.push(("SystemRoot".into(),v));}
  LocalProcessSpec{executable:exe.into(),workspace:std::env::current_dir().unwrap(),args:vec![],environment,tail_bytes:65536}
 }
 fn wait(c:&mut OwnedAcp,pred:impl Fn(&Event)->bool)->Event{
  let start=Instant::now();loop{
   for e in c.poll().unwrap().events{if pred(&e){return e}}
   assert!(start.elapsed()<Duration::from_secs(5),"missing ACP event");thread::sleep(Duration::from_millis(10));
  }
 }
 #[test]fn live_owned_stdio_initialize_session_permission_prompt_and_exit(){
  let mut c=OwnedAcp::start(fixture(),1).unwrap();c.initialize().unwrap();
  wait(&mut c,|e|matches!(e,Event::Initialized));
  c.new_session("C:\\owned").unwrap();wait(&mut c,|e|matches!(e,Event::SessionCreated{session} if session=="s1"));
  c.prompt("s1","inspect SDK fixture").unwrap();
  let start=Instant::now();let mut denied=false;let mut update=false;let mut settled=false;
  while !settled{for e in c.poll().unwrap().events{match e{Event::PermissionCancelled{..}=>denied=true,Event::Update{..}=>update=true,Event::PromptSettled{stop_reason,..}=>{assert_eq!(stop_reason,"end_turn");settled=true},_=>{}}}
   assert!(start.elapsed()<Duration::from_secs(5));thread::sleep(Duration::from_millis(10));}
  assert!(denied&&update);c.close_input();let start=Instant::now();
  loop{let p=c.poll().unwrap();if p.eof&&p.exit.is_some(){assert!(p.exit.unwrap().success());break}assert!(start.elapsed()<Duration::from_secs(5));thread::sleep(Duration::from_millis(10));}
  println!("ACP_OWNED_STDIO_PERMISSION_DENIAL_AND_EXIT_OK");
 }
}

#[cfg(test)]mod real_agent{
 use super::*;use std::{thread,time::{Duration,Instant}};
 #[test]fn actual_opencode_acp_handshake(){
  let Some(exe)=std::env::var_os("VIVIM_ACP_AGENT_EXE") else{panic!("actual agent executable not provided")};
  let workspace=std::env::current_dir().unwrap();
  // Explicit allowlist; no inherited credentials, no prompt/model request, no tools or MCP servers.
  let environment=["SystemRoot","WINDIR","PATH","TEMP","TMP","USERPROFILE","LOCALAPPDATA","APPDATA","HOME"].into_iter().filter_map(|n|std::env::var_os(n).map(|v|(n.into(),v))).collect();
  let mut c=OwnedAcp::start(LocalProcessSpec{executable:exe.into(),workspace,args:vec!["acp".into()],environment,tail_bytes:65536},2).unwrap();
  c.initialize().unwrap();let start=Instant::now();
  loop{
   let u=c.poll().unwrap();if u.events.iter().any(|e|matches!(e,Event::Initialized)){break}
   assert!(u.exit.is_none(),"actual agent exited before initialization");
   assert!(start.elapsed()<Duration::from_secs(20),"actual agent initialize timeout");thread::sleep(Duration::from_millis(25));
  }
  c.cancel_owned().unwrap();
  println!("ACTUAL_OPENCODE_ACP_INITIALIZE_OK");
 }
}
