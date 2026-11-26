//! Bounded, connection-local ACP v1 JSON-RPC client core. No process spawn,
//! network, filesystem operation, credential forwarding or effect authority.
//! Existing Host must authorize outbound intent and own worker lifetime.
//! Notifications are private transient observations; do not publish raw payloads.
use serde_json::{json,Value};
use std::collections::{HashMap,HashSet};
const MAX_FRAME:usize=256*1024;
const MAX_SESSIONS:usize=32;
const MAX_PENDING:usize=64;
#[derive(Debug,Clone,PartialEq,Eq)]pub enum Error{Limit,Malformed,Poisoned,State,Foreign,Exhausted}
#[derive(Default)]pub struct Framer{buf:Vec<u8>,poisoned:bool}
impl Framer{
 pub fn feed(&mut self,bytes:&[u8])->Result<Vec<Value>,Error>{
  if self.poisoned{return Err(Error::Poisoned);}
  let mut out=Vec::new();
  for &b in bytes{
   if b==b'\n'{
    if self.buf.last()==Some(&b'\r'){self.buf.pop();}
    let parsed=serde_json::from_slice::<Value>(&self.buf);
    self.buf.clear();
    match parsed{Ok(v) if v.is_object()&&out.len()<64=>out.push(v),_=>{self.poisoned=true;return Err(Error::Malformed);}}
   }else{
    if self.buf.len()>=MAX_FRAME{self.buf.clear();self.poisoned=true;return Err(Error::Limit);}
    self.buf.push(b);
   }
  }
  Ok(out)
 }
 pub fn finish(&mut self)->Result<(),Error>{if self.poisoned||!self.buf.is_empty(){self.poisoned=true;Err(Error::Malformed)}else{Ok(())}}
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]enum Phase{Fresh,Initializing,Ready,Closed}
#[derive(Debug,Clone,PartialEq,Eq)]enum Pending{Initialize,NewSession,LoadSession(String),SetModel(String,String),Prompt(String)}
#[derive(Debug,Clone,PartialEq)]pub enum Event{
 Initialized, SessionCreated{session:String}, SessionLoaded{session:String}, ModelSelected{session:String,model:String}, Update{session:String,update:Value}, ReplayUpdate{session:String,update:Value},
 PromptSettled{session:String,stop_reason:String}, Failed{session:Option<String>},
 PermissionCancelled{session:String,response:Value}, UnsupportedRequest{response:Value}, IgnoredNotification
}
pub struct Router{
 generation:u64,phase:Phase,next:u64,pending:HashMap<u64,Pending>,sessions:HashSet<String>,active:HashSet<String>,cancelled:HashSet<String>,load_supported:bool,models:HashMap<String,ModelState>
}
#[derive(Debug,Clone,PartialEq,Eq)]pub struct ModelState{pub config_id:String,pub current:String,pub choices:Vec<String>}
fn rpc(method:&str,id:Option<u64>,params:Value)->Value{
 let mut v=json!({"jsonrpc":"2.0","method":method,"params":params});
 if let Some(id)=id{v["id"]=json!(id);}v
}
fn valid_session(s:&str)->bool{!s.is_empty()&&s.len()<=256&&!s.chars().any(char::is_control)}
fn session(v:&Value)->Result<&str,Error>{v["sessionId"].as_str().filter(|s|valid_session(s)).ok_or(Error::Malformed)}
fn exact(v:&Value,keys:&[&str])->bool{v.as_object().is_some_and(|o|o.keys().all(|k|keys.contains(&k.as_str())))}
fn model_id(s:&str)->bool{!s.is_empty()&&s.len()<=256&&s.bytes().all(|b|b.is_ascii_alphanumeric()||b"._-/:#".contains(&b))}
fn model_state(v:&Value)->Result<Option<ModelState>,Error>{
 let Some(options)=v.get("configOptions") else{return Ok(None)};
 let options=options.as_array().ok_or(Error::Malformed)?;if options.len()>64{return Err(Error::Limit)}
 let mut result=None;
 for option in options.iter().filter(|o|o["category"]=="model"||o["id"]=="model"){
  if result.is_some()||option["type"]!="select"{return Err(Error::Malformed)}
  let id=option["id"].as_str().filter(|s|model_id(s)).ok_or(Error::Malformed)?;
  let current=option["currentValue"].as_str().filter(|s|model_id(s)).ok_or(Error::Malformed)?;
  let mut choices=Vec::new();let options=option["options"].as_array().ok_or(Error::Malformed)?;
  for item in options{let group=item.get("options").and_then(Value::as_array);let rows=group.map(Vec::as_slice).unwrap_or(std::slice::from_ref(item));for row in rows{let value=row["value"].as_str().filter(|s|model_id(s)).ok_or(Error::Malformed)?;if choices.len()>=2048||choices.iter().any(|x|x==value){return Err(Error::Limit)}choices.push(value.to_owned());}}
  if !choices.iter().any(|s|s==current){return Err(Error::Malformed)}
  result=Some(ModelState{config_id:id.into(),current:current.into(),choices});
 }Ok(result)
}
fn response_id(v:&Value)->bool{v.is_u64()||v.as_str().is_some_and(|s|!s.is_empty()&&s.len()<=128&&!s.chars().any(char::is_control))}
impl Router{
 pub fn new(generation:u64)->Result<Self,Error>{if generation==0{return Err(Error::State);}Ok(Self{generation,phase:Phase::Fresh,next:1,pending:HashMap::new(),sessions:HashSet::new(),active:HashSet::new(),cancelled:HashSet::new(),load_supported:false,models:HashMap::new()})}
 pub fn generation(&self)->u64{self.generation}
 pub fn load_supported(&self)->bool{self.load_supported}
 pub fn models(&self,session:&str)->Option<&ModelState>{self.models.get(session)}
 pub fn has_pending(&self)->bool{!self.pending.is_empty()||!self.active.is_empty()}
 fn request(&mut self,method:&str,params:Value,p:Pending)->Result<Value,Error>{
  if self.pending.len()>=MAX_PENDING{return Err(Error::Limit);}
  let id=self.next;self.next=id.checked_add(1).ok_or(Error::Exhausted)?;
  self.pending.insert(id,p);Ok(rpc(method,Some(id),params))
 }
 pub fn initialize(&mut self)->Result<Value,Error>{
  if self.phase!=Phase::Fresh{return Err(Error::State);}
  let v=self.request("initialize",json!({"protocolVersion":1,"clientCapabilities":{}}),Pending::Initialize)?;
  self.phase=Phase::Initializing;Ok(v)
 }
 /// The working directory is an already Host-bound local scope. This core
 /// does not resolve or authorize it. No remote MCP servers are configured.
 pub fn new_session(&mut self,approved_cwd:&str)->Result<Value,Error>{
  if self.phase!=Phase::Ready{return Err(Error::State);}
  if approved_cwd.is_empty()||approved_cwd.len()>1024||approved_cwd.chars().any(char::is_control){return Err(Error::Malformed);}
  if self.sessions.len()+self.pending.values().filter(|p|matches!(p,Pending::NewSession|Pending::LoadSession(_))).count()>=MAX_SESSIONS{return Err(Error::Limit);}
  self.request("session/new",json!({"cwd":approved_cwd,"mcpServers":[]}),Pending::NewSession)
 }
 pub fn load_session(&mut self,id:&str,approved_cwd:&str)->Result<Value,Error>{
  if self.phase!=Phase::Ready||!self.load_supported||!valid_session(id)||self.sessions.contains(id)||self.pending.values().any(|p|matches!(p,Pending::LoadSession(s) if s==id)){return Err(Error::State)}
  if approved_cwd.is_empty()||approved_cwd.len()>1024||approved_cwd.chars().any(char::is_control){return Err(Error::Malformed)}
  if self.sessions.len()+self.pending.values().filter(|p|matches!(p,Pending::LoadSession(_)|Pending::NewSession)).count()>=MAX_SESSIONS{return Err(Error::Limit)}
  self.request("session/load",json!({"sessionId":id,"cwd":approved_cwd,"mcpServers":[]}),Pending::LoadSession(id.into()))
 }
 pub fn set_model(&mut self,id:&str,approved_model:&str)->Result<Value,Error>{
  if self.phase!=Phase::Ready||!self.sessions.contains(id)||self.active.contains(id)||self.pending.values().any(|p|matches!(p,Pending::SetModel(s,_) if s==id)){return Err(Error::State)}
  let m=self.models.get(id).ok_or(Error::State)?;
  if !m.choices.iter().any(|x|x==approved_model){return Err(Error::Foreign)}
  self.request("session/set_config_option",json!({"sessionId":id,"configId":m.config_id,"value":approved_model}),Pending::SetModel(id.into(),approved_model.into()))
 }
 pub fn prompt(&mut self,id:&str,approved_text:&str)->Result<Value,Error>{
  if self.phase!=Phase::Ready||!self.sessions.contains(id)||self.active.contains(id)||self.pending.values().any(|p|matches!(p,Pending::SetModel(s,_) if s==id)){return Err(Error::State);}
  if approved_text.is_empty()||approved_text.len()>64*1024{return Err(Error::Limit);}
  let v=self.request("session/prompt",json!({"sessionId":id,"prompt":[{"type":"text","text":approved_text}]}),Pending::Prompt(id.into()))?;
  self.active.insert(id.into());Ok(v)
 }
 /// Cancel is a notification, not a settled process receipt. Keep prompt
 /// active until correlated response, failure or owned connection closure.
 pub fn cancel(&mut self,id:&str)->Result<Value,Error>{
  if self.phase!=Phase::Ready||!self.active.contains(id)||!self.cancelled.insert(id.into()){return Err(Error::State);}
  Ok(rpc("session/cancel",None,json!({"sessionId":id})))
 }
 pub fn close(&mut self){self.phase=Phase::Closed;self.pending.clear();self.sessions.clear();self.active.clear();self.cancelled.clear();self.models.clear();self.load_supported=false;}
 pub fn ingest(&mut self,generation:u64,v:Value)->Result<Event,Error>{
  if generation!=self.generation{return Err(Error::Foreign);}
  if self.phase==Phase::Closed||v["jsonrpc"]!="2.0"||!v.is_object(){return Err(Error::Malformed);}
  if let Some(method)=v["method"].as_str(){
   if !exact(&v,&["jsonrpc","id","method","params"]){return Err(Error::Malformed);}
   let params=&v["params"];
   if method=="session/update"{
    if v.get("id").is_some()||self.phase!=Phase::Ready{return Err(Error::Malformed);}
    let id=session(params)?;
    let replay=self.pending.values().any(|p|matches!(p,Pending::LoadSession(s) if s==id));
    if !self.sessions.contains(id)&&!replay{return Err(Error::Foreign);}
    let update=params["update"].clone();
    if !update.is_object()||update["sessionUpdate"].as_str().is_none(){return Err(Error::Malformed);}
    if replay{return Ok(Event::ReplayUpdate{session:id.into(),update})}
    if update["sessionUpdate"]=="config_option_update"{self.models.remove(id);if let Ok(Some(m))=model_state(&update){self.models.insert(id.into(),m);}}
    return Ok(Event::Update{session:id.into(),update});
   }
   if let Some(id)=v.get("id"){
    if !response_id(id){return Err(Error::Malformed);}
    if method=="session/request_permission"{
     let sid=session(params)?;
     // No approval engine in protocol adapter. Decline every delegated
     // permission request until an explicit Host review bridge is built.
     if !self.sessions.contains(sid){return Err(Error::Foreign);}
     return Ok(Event::PermissionCancelled{session:sid.into(),response:json!({"jsonrpc":"2.0","id":id,"result":{"outcome":{"outcome":"cancelled"}}})});
    }
    return Ok(Event::UnsupportedRequest{response:json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":"Client method not available"}})});
   }
   return Ok(Event::IgnoredNotification);
  }
  if !exact(&v,&["jsonrpc","id","result","error"])||v.get("result").is_some()==v.get("error").is_some(){return Err(Error::Malformed);}
  let id=v["id"].as_u64().ok_or(Error::Malformed)?;
  let pending=self.pending.get(&id).cloned().ok_or(Error::Foreign)?;
  if let Some(error)=v.get("error"){
   if error["code"].as_i64().is_none()||error["message"].as_str().is_none(){return Err(Error::Malformed);}
   self.pending.remove(&id);
   let session=match pending{Pending::Initialize=>{self.close();None},Pending::NewSession=>None,Pending::LoadSession(s)=>Some(s),Pending::SetModel(s,_)=>{self.models.remove(&s);Some(s)},Pending::Prompt(s)=>{self.active.remove(&s);self.cancelled.remove(&s);Some(s)}};
   // Never echo arbitrary agent error text into receipts or public logs.
   return Ok(Event::Failed{session});
  }
  let r=&v["result"];
  let event=match pending{
   Pending::Initialize=>{
    if self.phase!=Phase::Initializing||r["protocolVersion"]!=1{self.close();return Err(Error::State);}
    self.load_supported=r["agentCapabilities"]["loadSession"]==true;self.phase=Phase::Ready;Event::Initialized
   },
   Pending::NewSession=>{
    if self.phase!=Phase::Ready{return Err(Error::State);}
    let s=session(r)?;
    if self.sessions.len()>=MAX_SESSIONS||!self.sessions.insert(s.into()){return Err(Error::Foreign);}
    if let Ok(Some(m))=model_state(r){self.models.insert(s.into(),m);}
    Event::SessionCreated{session:s.into()}
   },
   Pending::LoadSession(s)=>{
    if self.phase!=Phase::Ready||r.get("sessionId").is_some_and(|v|v.as_str()!=Some(&s))||!r.is_object(){return Err(Error::Malformed)}
    if self.sessions.len()>=MAX_SESSIONS||!self.sessions.insert(s.clone()){return Err(Error::Foreign)}
    if let Ok(Some(m))=model_state(r){self.models.insert(s.clone(),m);}
    Event::SessionLoaded{session:s}
   },
   Pending::SetModel(s,model)=>{
    self.models.remove(&s);
    let m=model_state(r)?.ok_or(Error::Malformed)?;
    if m.current!=model{return Err(Error::Foreign)}
    self.models.insert(s.clone(),m);Event::ModelSelected{session:s,model}
   },
   Pending::Prompt(s)=>{
    let reason=r["stopReason"].as_str().filter(|r|matches!(*r,"end_turn"|"max_tokens"|"max_turn_requests"|"refusal"|"cancelled")).ok_or(Error::Malformed)?;
    if !self.active.remove(&s){return Err(Error::State);}
    self.cancelled.remove(&s);Event::PromptSettled{session:s,stop_reason:reason.into()}
   }
  };
  self.pending.remove(&id);Ok(event)
 }
}
#[cfg(test)]mod tests{
 use super::*;
 fn reply(id:&Value,result:Value)->Value{json!({"jsonrpc":"2.0","id":id,"result":result})}
 fn ready()->Router{let mut r=Router::new(1).unwrap();let init=r.initialize().unwrap();assert_eq!(r.ingest(1,reply(&init["id"],json!({"protocolVersion":1}))).unwrap(),Event::Initialized);r}
 fn add(r:&mut Router,s:&str){let q=r.new_session("C:\\owned").unwrap();assert_eq!(r.ingest(1,reply(&q["id"],json!({"sessionId":s}))).unwrap(),Event::SessionCreated{session:s.into()});}
 #[test]fn unknown_model_metadata_does_not_grant_model_authority(){let mut r=ready();let q=r.new_session("C:\\owned").unwrap();assert_eq!(r.ingest(1,reply(&q["id"],json!({"sessionId":"s","configOptions":[{"id":"model","type":"unknown"}]}))).unwrap(),Event::SessionCreated{session:"s".into()});assert!(r.models("s").is_none());assert!(r.set_model("s","x").is_err());}
 #[test]fn framing(){let mut f=Framer::default();assert!(f.feed(b"{\"jsonrpc\":").unwrap().is_empty());assert_eq!(f.feed(b"\"2.0\"}\n{}\r\n").unwrap().len(),2);assert!(f.finish().is_ok());assert_eq!(f.feed(b"not-json\n"),Err(Error::Malformed));assert_eq!(f.feed(b"{}\n"),Err(Error::Poisoned));}
 #[test]fn frame_limit_and_truncation(){let mut f=Framer::default();assert_eq!(f.feed(&vec![b'x';MAX_FRAME+1]),Err(Error::Limit));let mut g=Framer::default();g.feed(b"{").unwrap();assert!(g.finish().is_err());}
 #[test]fn concurrent_sessions_and_generation(){
  let mut r=ready();add(&mut r,"a");add(&mut r,"b");let a=r.prompt("a","one").unwrap();let b=r.prompt("b","two").unwrap();
  assert!(r.prompt("a","again").is_err());
  let update=json!({"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"b","update":{"sessionUpdate":"agent_message_chunk"}}});
  assert_eq!(r.ingest(2,update.clone()),Err(Error::Foreign));
  assert!(matches!(r.ingest(1,update).unwrap(),Event::Update{session,..} if session=="b"));
  assert_eq!(r.ingest(1,reply(&b["id"],json!({"stopReason":"end_turn"}))).unwrap(),Event::PromptSettled{session:"b".into(),stop_reason:"end_turn".into()});
  assert!(r.cancel("a").unwrap().get("id").is_none());assert!(r.cancel("a").is_err());
  assert_eq!(r.ingest(1,reply(&a["id"],json!({"stopReason":"cancelled"}))).unwrap(),Event::PromptSettled{session:"a".into(),stop_reason:"cancelled".into()});
  assert_eq!(r.ingest(1,reply(&a["id"],json!({"stopReason":"end_turn"}))),Err(Error::Foreign));
 }
 #[test]fn permission_is_never_auto_granted(){
  let mut r=ready();add(&mut r,"a");
  let q=json!({"jsonrpc":"2.0","id":42,"method":"session/request_permission","params":{"sessionId":"a","options":[{"optionId":"allow","kind":"allow_once"}]}});
  assert!(matches!(r.ingest(1,q).unwrap(),Event::PermissionCancelled{response,..} if response["result"]["outcome"]["outcome"]=="cancelled"));
  let q=json!({"jsonrpc":"2.0","id":43,"method":"fs/read_text_file","params":{"path":"anything"}});
  assert!(matches!(r.ingest(1,q).unwrap(),Event::UnsupportedRequest{response} if response["error"]["code"] == -32601));
 }
 #[test]fn unsupported_version_and_closed(){let mut r=Router::new(1).unwrap();let q=r.initialize().unwrap();assert!(r.ingest(1,reply(&q["id"],json!({"protocolVersion":2}))).is_err());assert!(r.new_session("C:\\owned").is_err());}
}
