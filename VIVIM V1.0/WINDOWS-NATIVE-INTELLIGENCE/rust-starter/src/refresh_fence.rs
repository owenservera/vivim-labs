//! @hq:context Coalesce admitted native observations before explicit resident SDK refresh.
//! @hq:invariant Notifications are invalidation hints, never proof of complete source coverage.
//! @hq:see journal.rs
//! Product target: adapters/hq/dev-session.mjs at 5da225899ff635f1161ad07212e34f9517ac9f53.
//! No watcher creation, compiler, database, auto-run, source edits or provider calls.
use crate::journal::Cursor;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Mode{Refresh,Rebaseline}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct Ticket{pub id:u64,pub through:Cursor,pub mode:Mode}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Error{Invalid,Stale,Busy,Incomplete,Exhausted}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum State{Clean,Dirty,Rebaseline,Refreshing}
/// @hq:context Track one explicit refresh across a burst of native notifications.
/// @hq:why Preserve edits arriving during compilation rather than acknowledging them.
pub struct RefreshFence{current:Cursor,committed:Option<Cursor>,gap:bool,active:Option<Ticket>,next:u64}
impl RefreshFence{
 pub fn new(current:Cursor)->Result<Self,Error>{
  if current.generation==0{return Err(Error::Invalid);}
  Ok(Self{current,committed:None,gap:true,active:None,next:0})
 }
 pub fn state(&self)->State{
  if self.active.is_some(){State::Refreshing}else if self.gap{State::Rebaseline}else if self.committed!=Some(self.current){State::Dirty}else{State::Clean}
 }
 /// Feed every admitted observation, including protocol and dependency changes.
 pub fn observe(&mut self,c:Cursor)->Result<(),Error>{
  if c.generation==0{return Err(Error::Invalid);}
  if c.generation<self.current.generation||c.generation==self.current.generation&&c.sequence<=self.current.sequence{return Err(Error::Stale);}
  if c.generation!=self.current.generation||self.current.sequence.checked_add(1)!=Some(c.sequence){self.gap=true;}
  self.current=c;Ok(())
 }
 pub fn lost(&mut self,c:Cursor)->Result<(),Error>{
  self.observe(c)?;self.gap=true;Ok(())
 }
 /// @hq:invariant Only the incumbent session calls begin on explicit demand.
 pub fn begin(&mut self)->Result<Option<Ticket>,Error>{
  if self.active.is_some(){return Err(Error::Busy);}
  if !self.gap&&self.committed==Some(self.current){return Ok(None);}
  self.next=self.next.checked_add(1).ok_or(Error::Exhausted)?;
  let t=Ticket{id:self.next,through:self.current,mode:if self.gap{Mode::Rebaseline}else{Mode::Refresh}};
  self.active=Some(t);Ok(Some(t))
 }
 pub fn abort(&mut self,t:Ticket)->Result<(),Error>{
  if self.active!=Some(t){return Err(Error::Stale);}self.active=None;Ok(())
 }
 /// @hq:invariant Trusted incumbent HQ/index/graph/Wiki readback is required.
 /// Caller verifies complete source coverage and matching source/graph/Wiki
 /// revisions and current grant; notification count is not coverage proof.
 pub fn finish(&mut self,t:Ticket,observed:Cursor,complete:bool)->Result<(),Error>{
  if self.active!=Some(t){return Err(Error::Stale);}
  self.active=None;
  if !complete{return Err(Error::Incomplete);}
  if observed!=t.through||self.current!=t.through{return Err(Error::Stale);}
  self.committed=Some(t.through);self.gap=false;Ok(())
 }
}
#[cfg(test)]mod tests{
 use super::*;
 fn c(g:u64,s:u64)->Cursor{Cursor{generation:g,sequence:s}}
 fn clean()->RefreshFence{let mut f=RefreshFence::new(c(1,0)).unwrap();let t=f.begin().unwrap().unwrap();assert_eq!(t.mode,Mode::Rebaseline);f.finish(t,c(1,0),true).unwrap();f}
 #[test]fn burst_is_one_refresh(){let mut f=clean();for n in 1..=200{f.observe(c(1,n)).unwrap();}let t=f.begin().unwrap().unwrap();assert_eq!(t.through,c(1,200));assert_eq!(t.mode,Mode::Refresh);assert_eq!(f.begin(),Err(Error::Busy));f.finish(t,c(1,200),true).unwrap();assert_eq!(f.begin(),Ok(None));}
 #[test]fn concurrent_edit_stays_dirty(){let mut f=clean();f.observe(c(1,1)).unwrap();let t=f.begin().unwrap().unwrap();f.observe(c(1,2)).unwrap();assert_eq!(f.finish(t,c(1,1),true),Err(Error::Stale));assert_eq!(f.state(),State::Dirty);assert_eq!(f.begin().unwrap().unwrap().through,c(1,2));}
 #[test]fn gaps_force_rebaseline(){let mut f=clean();f.observe(c(1,3)).unwrap();let t=f.begin().unwrap().unwrap();assert_eq!(t.mode,Mode::Rebaseline);f.lost(c(1,4)).unwrap();assert_eq!(f.finish(t,c(1,3),true),Err(Error::Stale));assert_eq!(f.begin().unwrap().unwrap().mode,Mode::Rebaseline);}
 #[test]fn failed_refresh_never_consumes(){let mut f=clean();f.observe(c(1,1)).unwrap();let t=f.begin().unwrap().unwrap();assert_eq!(f.finish(t,c(1,1),false),Err(Error::Incomplete));let n=f.begin().unwrap().unwrap();assert_ne!(t.id,n.id);assert_eq!(f.abort(t),Err(Error::Stale));f.abort(n).unwrap();assert_eq!(f.state(),State::Dirty);}
 #[test]fn restart_and_replay(){let mut f=clean();f.observe(c(2,0)).unwrap();assert_eq!(f.observe(c(1,9)),Err(Error::Stale));let t=f.begin().unwrap().unwrap();assert_eq!(t.mode,Mode::Rebaseline);f.finish(t,c(2,0),true).unwrap();assert_eq!(f.state(),State::Clean);}
}
