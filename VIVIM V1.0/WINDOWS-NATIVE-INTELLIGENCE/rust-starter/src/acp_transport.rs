//! Owned ACP stdout ingestion. Bounded framing + request correlation + byte
//! acknowledgment. No process launcher, no authorization, no network endpoint.
//! Caller binds this instance to one existing Host-owned worker generation.
//! No stderr data may be passed to this stream; no automatic replay.
use crate::{acp::{self,Event,Framer,Router},stream::{self,Mode,Stream}};
use serde_json::Value;
#[derive(Debug,Clone,PartialEq,Eq)]pub enum Error{
 ForeignGeneration,Closed,Protocol(acp::Error),Stream(stream::Error),Incomplete
}
#[derive(Debug,Clone,PartialEq)]pub struct Batch{
 pub generation:u64,pub acknowledged_through:u64,pub events:Vec<Event>
}
pub struct Transport{router:Router,framer:Framer,stdout:Stream,ack:u64,closed:bool}
impl Transport{
 pub fn new(generation:u64,capacity:usize)->Result<Self,Error>{
  Ok(Self{router:Router::new(generation).map_err(Error::Protocol)?,framer:Framer::default(),stdout:Stream::new(Mode::Lossless,capacity).map_err(Error::Stream)?,ack:0,closed:false})
 }
 pub fn generation(&self)->u64{self.router.generation()}
 fn ensure_open(&self)->Result<(),Error>{if self.closed{Err(Error::Closed)}else{Ok(())}}
 pub fn initialize(&mut self)->Result<Value,Error>{self.ensure_open()?;self.router.initialize().map_err(Error::Protocol)}
 pub fn new_session(&mut self,approved_cwd:&str)->Result<Value,Error>{self.ensure_open()?;self.router.new_session(approved_cwd).map_err(Error::Protocol)}
 pub fn prompt(&mut self,session:&str,approved_text:&str)->Result<Value,Error>{self.ensure_open()?;self.router.prompt(session,approved_text).map_err(Error::Protocol)}
 pub fn cancel(&mut self,session:&str)->Result<Value,Error>{self.ensure_open()?;self.router.cancel(session).map_err(Error::Protocol)}
 fn abort(&mut self){self.closed=true;self.stdout.close();self.router.close();}
 /// Ingest contiguous bytes from one owned worker. Wrong generation and
 /// non-contiguous byte offsets are rejected before consuming data. All events
 /// in a failed chunk are withheld and the connection becomes unrecoverable;
 /// existing Host must reconcile pending effects. ACK means parsed/retained in
 /// memory, NOT durable graph commit, provider effect receipt or permission.
 pub fn ingest(&mut self,generation:u64,offset:u64,bytes:&[u8])->Result<Batch,Error>{
  self.ensure_open()?;
  if generation!=self.generation(){return Err(Error::ForeignGeneration);}
  if offset!=self.ack{self.abort();return Err(Error::Stream(stream::Error::Cursor));}
  if let Err(e)=self.stdout.append(bytes){self.abort();return Err(Error::Stream(e));}
  let result=self.consume();
  if result.is_err(){self.abort();}
  result
 }
 fn consume(&mut self)->Result<Batch,Error>{
  let mut events=Vec::new();
  loop{
   let page=self.stdout.read(self.ack,4096).map_err(Error::Stream)?;
   if page.gap||page.start!=self.ack{return Err(Error::Stream(stream::Error::Cursor));}
   if page.bytes.is_empty(){break;}
   for msg in self.framer.feed(&page.bytes).map_err(Error::Protocol)?{
    // A chunk may not monopolize worker memory by producing tiny frames.
    if events.len()>=256{return Err(Error::Protocol(acp::Error::Limit));}
    events.push(self.router.ingest(self.generation(),msg).map_err(Error::Protocol)?);
   }
   self.stdout.acknowledge(page.next).map_err(Error::Stream)?;
   self.ack=page.next;
  }
  Ok(Batch{generation:self.generation(),acknowledged_through:self.ack,events})
 }
 /// EOF with partial frame or outstanding request is incomplete, never an
 /// inferred successful agent effect. Reconnect requires a new generation.
 pub fn eof(&mut self,generation:u64)->Result<(),Error>{
  self.ensure_open()?;
  if generation!=self.generation(){return Err(Error::ForeignGeneration);}
  let complete=self.framer.finish().is_ok()&&!self.router.has_pending();
  self.abort();
  if complete{Ok(())}else{Err(Error::Incomplete)}
 }
}
#[cfg(test)]mod tests{
 use super::*;use serde_json::json;
 fn line(v:Value)->Vec<u8>{let mut b=serde_json::to_vec(&v).unwrap();b.push(b'\n');b}
 fn init(t:&mut Transport)->u64{
  let req=t.initialize().unwrap();
  let b=line(json!({"jsonrpc":"2.0","id":req["id"],"result":{"protocolVersion":1}}));
  let n=b.len()/2;
  assert!(t.ingest(1,0,&b[..n]).unwrap().events.is_empty());
  let result=t.ingest(1,n as u64,&b[n..]).unwrap();
  assert_eq!(result.events,vec![Event::Initialized]);result.acknowledged_through
 }
 #[test]fn actual_partial_framing_and_correlation(){
  let mut t=Transport::new(1,4096).unwrap();let n=init(&mut t);
  let q=t.new_session("C:\\owned").unwrap();
  let b=line(json!({"jsonrpc":"2.0","id":q["id"],"result":{"sessionId":"s1"}}));
  let r=t.ingest(1,n,&b).unwrap();assert_eq!(r.events,vec![Event::SessionCreated{session:"s1".into()}]);
  let q=t.prompt("s1","fixture").unwrap();
  let mut b=line(json!({"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"s1","update":{"sessionUpdate":"agent_message_chunk"}}}));
  b.extend(line(json!({"jsonrpc":"2.0","id":q["id"],"result":{"stopReason":"end_turn"}})));
  let r=t.ingest(1,r.acknowledged_through,&b).unwrap();assert_eq!(r.events.len(),2);assert!(matches!(&r.events[1],Event::PromptSettled{session,..} if session=="s1"));assert!(t.eof(1).is_ok());
 }
 #[test]fn wrong_generation_and_offset(){
  let mut t=Transport::new(1,4096).unwrap();assert_eq!(t.ingest(2,0,b"{}"),Err(Error::ForeignGeneration));
  let n=init(&mut t);assert_eq!(t.ingest(1,n+1,b"{}"),Err(Error::Stream(stream::Error::Cursor)));assert_eq!(t.initialize(),Err(Error::Closed));
 }
 #[test]fn overflow_and_truncated_eof(){
  let mut t=Transport::new(1,4).unwrap();assert_eq!(t.ingest(1,0,b"12345"),Err(Error::Stream(stream::Error::Overflow)));assert_eq!(t.ingest(1,0,b"{}"),Err(Error::Closed));
  let mut t=Transport::new(1,4096).unwrap();t.ingest(1,0,b"{").unwrap();assert_eq!(t.eof(1),Err(Error::Incomplete));
 }
 #[test]fn invalid_second_message_poisoned_batch(){
  let mut t=Transport::new(1,4096).unwrap();let q=t.initialize().unwrap();
  let mut b=line(json!({"jsonrpc":"2.0","id":q["id"],"result":{"protocolVersion":1}}));b.extend(b"not-json\n");
  assert!(t.ingest(1,0,&b).is_err());assert_eq!(t.new_session("C:\\owned"),Err(Error::Closed));
 }
 #[test]fn pending_response_is_not_success(){
  let mut t=Transport::new(1,4096).unwrap();init(&mut t);t.new_session("C:\\owned").unwrap();assert_eq!(t.eof(1),Err(Error::Incomplete));
 }
}
