//! Pure scoped semantic observation core, independently implemented from HCU concepts.
//! No UI access, input injection, permission grants or credential values.
use std::collections::{BTreeMap,BTreeSet};
#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub struct Scope{pub owner:u64,pub app:u64,pub window:u64,pub generation:u64}
impl Scope{fn valid(self)->bool{self.owner!=0&&self.app!=0&&self.window!=0&&self.generation!=0}}
#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub struct Ref{pub scope:Scope,pub serial:u64}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Role{Button,Text,Edit,List,Item,Menu,Other}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Control{pub identity:u64,pub fingerprint:[u8;32],pub role:Role,pub enabled:bool,pub sensitive:bool}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Entry{pub reference:Ref,pub control:Control}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Snapshot{pub scope:Scope,pub revision:u64,pub entries:Vec<Entry>}
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Delta{Full(Snapshot),Unchanged{revision:u64},Changes{from:u64,to:u64,removed:Vec<Ref>,upsert:Vec<Entry>}}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Error{Invalid,Foreign,Stale,Incomplete,Limit,Exhausted,Disabled,Sensitive}
pub struct Registry{scope:Scope,revision:u64,serial:u64,limit:usize,entries:BTreeMap<u64,Entry>,valid:bool}
impl Registry{
 pub fn new(scope:Scope,limit:usize)->Result<Self,Error>{if !scope.valid()||!(1..=4096).contains(&limit){return Err(Error::Invalid);}Ok(Self{scope,revision:0,serial:0,limit,entries:BTreeMap::new(),valid:false})}
 pub fn invalidate(&mut self){self.valid=false;self.entries.clear();}
 /// Only complete, already filtered provider observations. Incomplete enumeration
 /// invalidates all references; absence from a partial tree is not evidence.
 pub fn observe(&mut self,scope:Scope,revision:u64,complete:bool,controls:Vec<Control>)->Result<Snapshot,Error>{
  if scope!=self.scope{return Err(Error::Foreign);}
  if revision<=self.revision{return Err(Error::Stale);}
  if !complete{self.revision=revision;self.invalidate();return Err(Error::Incomplete);}
  if controls.len()>self.limit{self.revision=revision;self.invalidate();return Err(Error::Limit);}
  let mut ids=BTreeSet::new();
  if controls.iter().any(|c|c.identity==0||c.fingerprint==[0;32]||!ids.insert(c.identity)){
   self.revision=revision;self.invalidate();return Err(Error::Invalid);
  }
  // A revision gap invalidates even matching fingerprints. The provider must
  // use a new generation on restart; identity is never inferred from labels.
  let contiguous=self.valid&&self.revision.checked_add(1)==Some(revision);
  let mut next=self.serial;let mut entries=BTreeMap::new();
  for c in controls{
   // Sensitive controls are not issued actionable references or projected.
   if c.sensitive{continue;}
   let old=if contiguous{self.entries.get(&c.identity).filter(|e|e.control.fingerprint==c.fingerprint&&e.control.role==c.role)}else{None};
   let reference=if let Some(e)=old{e.reference}else{next=next.checked_add(1).ok_or(Error::Exhausted)?;Ref{scope,serial:next}};
   entries.insert(c.identity,Entry{reference,control:c});
  }
  self.entries=entries;self.serial=next;self.revision=revision;self.valid=true;self.snapshot()
 }
 pub fn snapshot(&self)->Result<Snapshot,Error>{if !self.valid{return Err(Error::Incomplete);}Ok(Snapshot{scope:self.scope,revision:self.revision,entries:self.entries.values().cloned().collect()})}
 /// A ref is only a locator. Host must independently revalidate identity,
 /// revision, current authorization and allowed operation before any effect.
 pub fn resolve(&self,r:Ref,revision:u64)->Result<&Control,Error>{
  if r.scope!=self.scope{return Err(Error::Foreign);}
  if !self.valid||revision!=self.revision{return Err(Error::Stale);}
  let e=self.entries.values().find(|e|e.reference==r).ok_or(Error::Stale)?;
  if e.control.sensitive{return Err(Error::Sensitive);}
  if !e.control.enabled{return Err(Error::Disabled);}Ok(&e.control)
 }
}
/// One consumer's acknowledged baseline. No delivery acknowledgment is implied
/// by generating a response. Never share between owners or windows.
pub struct Delivery{scope:Scope,acked:Option<Snapshot>,pending:Option<Snapshot>,budget:usize}
impl Delivery{
 pub fn new(scope:Scope,budget:usize)->Result<Self,Error>{if !scope.valid()||!(1..=4096).contains(&budget){return Err(Error::Invalid);}Ok(Self{scope,acked:None,pending:None,budget})}
 pub fn reset(&mut self){self.acked=None;self.pending=None;}
 pub fn prepare(&mut self,s:Snapshot)->Result<Delta,Error>{
  if s.scope!=self.scope{return Err(Error::Foreign);}
  if s.entries.len()>self.budget{return Err(Error::Limit);}
  if s.revision==0||s.entries.iter().any(|e|e.reference.scope!=s.scope||e.reference.serial==0||e.control.sensitive){return Err(Error::Invalid);}
  let ids:BTreeSet<_>=s.entries.iter().map(|e|e.reference).collect();if ids.len()!=s.entries.len(){return Err(Error::Invalid);}
  if self.pending.is_some(){return Err(Error::Incomplete);}
  let d=match &self.acked{
   Some(a) if s.revision<a.revision=>return Err(Error::Stale),
   Some(a) if s.revision==a.revision=>{if *a!=s{return Err(Error::Invalid);}return Ok(Delta::Unchanged{revision:s.revision});},
   Some(a) if a.revision.checked_add(1)==Some(s.revision)=>{
    let old:BTreeMap<_,_>=a.entries.iter().map(|e|(e.reference,e)).collect();
    let new:BTreeMap<_,_>=s.entries.iter().map(|e|(e.reference,e)).collect();
    let removed:Vec<_>=old.keys().filter(|r|!new.contains_key(r)).copied().collect();
    let upsert:Vec<_>=s.entries.iter().filter(|e|old.get(&e.reference).copied()!=Some(*e)).cloned().collect();
    if removed.len()+upsert.len()>self.budget{Delta::Full(s.clone())}else{Delta::Changes{from:a.revision,to:s.revision,removed,upsert}}
   },
   _=>Delta::Full(s.clone())
  };
  self.pending=Some(s);Ok(d)
 }
 pub fn acknowledge(&mut self,scope:Scope,revision:u64)->Result<(),Error>{
  if scope!=self.scope{return Err(Error::Foreign);}
  if self.pending.as_ref().map(|s|s.revision)!=Some(revision){return Err(Error::Stale);}
  self.acked=self.pending.take();Ok(())
 }
}
#[cfg(test)]mod tests{
 use super::*;
 fn s()->Scope{Scope{owner:1,app:2,window:3,generation:4}}
 fn c(id:u64)->Control{Control{identity:id,fingerprint:[id as u8;32],role:Role::Button,enabled:true,sensitive:false}}
 #[test]fn refs_stable_only_when_current(){let mut r=Registry::new(s(),8).unwrap();let a=r.observe(s(),1,true,vec![c(1)]).unwrap();let k=a.entries[0].reference;let b=r.observe(s(),2,true,vec![c(1)]).unwrap();assert_eq!(k,b.entries[0].reference);assert_eq!(r.resolve(k,1),Err(Error::Stale));assert!(r.resolve(k,2).is_ok());let b=r.observe(s(),4,true,vec![c(1)]).unwrap();assert_ne!(k,b.entries[0].reference);assert_eq!(r.resolve(k,4),Err(Error::Stale));}
 #[test]fn incomplete_and_identity_reuse(){let mut r=Registry::new(s(),8).unwrap();let a=r.observe(s(),1,true,vec![c(1)]).unwrap();let k=a.entries[0].reference;assert_eq!(r.observe(s(),2,false,vec![]),Err(Error::Incomplete));assert_eq!(r.resolve(k,2),Err(Error::Stale));let b=r.observe(s(),3,true,vec![c(1)]).unwrap();assert_ne!(k,b.entries[0].reference);let mut x=c(1);x.fingerprint=[9;32];let d=r.observe(s(),4,true,vec![x]).unwrap();assert_ne!(b.entries[0].reference,d.entries[0].reference);}
 #[test]fn scope_and_sensitive(){let mut r=Registry::new(s(),8).unwrap();let mut x=c(2);x.sensitive=true;let a=r.observe(s(),1,true,vec![c(1),x]).unwrap();assert_eq!(a.entries.len(),1);let mut k=a.entries[0].reference;k.scope.owner=99;assert_eq!(r.resolve(k,1),Err(Error::Foreign));let mut x=c(1);x.enabled=false;let b=r.observe(s(),2,true,vec![x]).unwrap();assert_eq!(r.resolve(b.entries[0].reference,2),Err(Error::Disabled));}
 #[test]fn duplicate_identity_invalidates(){let mut r=Registry::new(s(),8).unwrap();r.observe(s(),1,true,vec![c(1)]).unwrap();assert_eq!(r.observe(s(),2,true,vec![c(1),c(1)]),Err(Error::Invalid));assert_eq!(r.snapshot(),Err(Error::Incomplete));}
 #[test]fn delta_requires_ack(){let mut r=Registry::new(s(),8).unwrap();let mut d=Delivery::new(s(),8).unwrap();let a=r.observe(s(),1,true,vec![c(1),c(2)]).unwrap();assert!(matches!(d.prepare(a.clone()).unwrap(),Delta::Full(_)));assert_eq!(d.prepare(a.clone()),Err(Error::Incomplete));assert_eq!(d.acknowledge(s(),2),Err(Error::Stale));d.acknowledge(s(),1).unwrap();assert_eq!(d.prepare(a),Ok(Delta::Unchanged{revision:1}));let b=r.observe(s(),2,true,vec![c(2),c(3)]).unwrap();match d.prepare(b).unwrap(){Delta::Changes{removed,upsert,..}=>{assert_eq!(removed.len(),1);assert_eq!(upsert.len(),1)},_=>panic!()}}
 #[test]fn consumer_reset_and_gap(){let mut r=Registry::new(s(),8).unwrap();let mut d=Delivery::new(s(),8).unwrap();let a=r.observe(s(),1,true,vec![c(1)]).unwrap();d.prepare(a).unwrap();d.acknowledge(s(),1).unwrap();let b=r.observe(s(),3,true,vec![c(1)]).unwrap();assert!(matches!(d.prepare(b.clone()).unwrap(),Delta::Full(_)));d.reset();assert!(matches!(d.prepare(b).unwrap(),Delta::Full(_)));}
}
