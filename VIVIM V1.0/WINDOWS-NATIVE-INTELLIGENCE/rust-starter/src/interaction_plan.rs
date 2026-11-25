//! Deterministic selected-app observation predicates and route proposals.
//! No input execution, retries, timers, or new authority. The incumbent Host
//! supplies trusted observations/receipt states and owns all scheduling.
use crate::semantic_surface::{Scope,Ref,Snapshot};
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Predicate{Present(Ref),Absent(Ref),Enabled(Ref),Disabled(Ref)}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum WaitResult{Matched,Pending,TimedOut,Unknown,Foreign,Stale}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct Wait{scope:Scope,min_revision:u64,start_ms:u64,deadline_ms:u64,predicate:Predicate}
impl Wait{
 pub fn new(scope:Scope,min_revision:u64,now:u64,timeout_ms:u64,predicate:Predicate)->Option<Self>{
  let r=match predicate{Predicate::Present(r)|Predicate::Absent(r)|Predicate::Enabled(r)|Predicate::Disabled(r)=>r};
  if r.scope!=scope||r.serial==0||min_revision==0||!(1..=30000).contains(&timeout_ms){return None;}
  Some(Self{scope,min_revision,start_ms:now,deadline_ms:now.checked_add(timeout_ms)?,predicate})
 }
 /// Snapshot must be a complete current provider observation. A reference that
 /// disappears is not proof a business action succeeded; the Host must verify
 /// the actual operation's explicit postcondition too.
 pub fn evaluate(&self,snapshot:Option<&Snapshot>,now:u64)->WaitResult{
  if now<self.start_ms{return WaitResult::Unknown;}
  if now>=self.deadline_ms{return WaitResult::TimedOut;}
  let Some(s)=snapshot else{return WaitResult::Unknown;};
  if s.scope!=self.scope{return WaitResult::Foreign;}
  if s.revision<self.min_revision{return WaitResult::Stale;}
  let r=match self.predicate{Predicate::Present(r)|Predicate::Absent(r)|Predicate::Enabled(r)|Predicate::Disabled(r)=>r};
  let entry=s.entries.iter().find(|e|e.reference==r);
  let matched=match self.predicate{
   Predicate::Present(_)=>entry.is_some(),
   Predicate::Absent(_)=>entry.is_none(),
   Predicate::Enabled(_)=>entry.is_some_and(|e|e.control.enabled&&!e.control.sensitive),
   Predicate::Disabled(_)=>entry.is_some_and(|e|!e.control.enabled&&!e.control.sensitive)
  };
  if matched{WaitResult::Matched}else{WaitResult::Pending}
 }
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Channel{Native,Semantic,Visual}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Attempt{None,NotDispatched,Pending,Applied,Rejected,Uncertain}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct Profile{pub native:bool,pub semantic:bool,pub visual:bool}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Route{Propose(Channel),Reconcile,Complete,Stop,Unavailable}
/// Profile is derived from registered scoped capabilities by the existing
/// Host, not from agent-provided flags. No proposal authorizes execution.
/// Alternate routes only after a verified *pre-dispatch* refusal.
pub fn choose(profile:Profile,prior:Attempt,unavailable:Option<Channel>)->Route{
 match prior{
  Attempt::Pending|Attempt::Uncertain=>return Route::Reconcile,
  Attempt::Applied=>return Route::Complete,
  Attempt::Rejected=>return Route::Stop,
  Attempt::None|Attempt::NotDispatched=>{}
 }
 if prior==Attempt::None&&unavailable.is_some()||prior==Attempt::NotDispatched&&unavailable.is_none(){return Route::Stop;}
 for (channel,supported) in [(Channel::Native,profile.native),(Channel::Semantic,profile.semantic),(Channel::Visual,profile.visual)]{
  // Only move down the preference order; never reselect an earlier route.
  let rank=|c|match c{Channel::Native=>0,Channel::Semantic=>1,Channel::Visual=>2};
  if supported&&unavailable.is_none_or(|old|rank(channel)>rank(old)){return Route::Propose(channel);}
 }
 Route::Unavailable
}
#[cfg(test)]mod tests{
 use super::*;use crate::semantic_surface::{Control,Role,Registry};
 fn scope()->Scope{Scope{owner:1,app:2,window:3,generation:4}}
 fn snap()->Snapshot{Registry::new(scope(),8).unwrap().observe(scope(),1,true,vec![Control{identity:1,fingerprint:[1;32],role:Role::Button,enabled:true,sensitive:false}]).unwrap()}
 #[test]fn wait_boundaries(){let s=snap();let r=s.entries[0].reference;let w=Wait::new(scope(),1,100,500,Predicate::Enabled(r)).unwrap();assert_eq!(w.evaluate(Some(&s),100),WaitResult::Matched);assert_eq!(w.evaluate(None,100),WaitResult::Unknown);assert_eq!(w.evaluate(Some(&s),600),WaitResult::TimedOut);assert_eq!(w.evaluate(Some(&s),99),WaitResult::Unknown);}
 #[test]fn no_absence_from_missing_or_foreign(){let s=snap();let w=Wait::new(scope(),2,100,500,Predicate::Absent(s.entries[0].reference)).unwrap();assert_eq!(w.evaluate(Some(&s),101),WaitResult::Stale);assert_eq!(w.evaluate(None,101),WaitResult::Unknown);let mut f=s.clone();f.scope.owner=99;assert_eq!(w.evaluate(Some(&f),101),WaitResult::Foreign);let fresh=Snapshot{scope:scope(),revision:2,entries:vec![]};assert_eq!(w.evaluate(Some(&fresh),101),WaitResult::Matched);}
 #[test]fn hybrid_route_pre_dispatch_only(){let p=Profile{native:true,semantic:true,visual:true};assert_eq!(choose(p,Attempt::None,None),Route::Propose(Channel::Native));assert_eq!(choose(p,Attempt::NotDispatched,Some(Channel::Native)),Route::Propose(Channel::Semantic));assert_eq!(choose(p,Attempt::NotDispatched,Some(Channel::Visual)),Route::Unavailable);assert_eq!(choose(p,Attempt::Uncertain,Some(Channel::Native)),Route::Reconcile);assert_eq!(choose(p,Attempt::Rejected,None),Route::Stop);assert_eq!(choose(p,Attempt::Applied,None),Route::Complete);}
}
