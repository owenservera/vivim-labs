//! Pure projection of Host-validated owned-agent lifecycle events, not an executor.
//! Caller must validate ownership, receipt provenance and session identity first.
#[derive(Debug,Clone,Copy,PartialEq,Eq)] pub enum State{Proposed,Starting,Running,Cancelling,Exited,Failed}
#[derive(Debug,Clone,Copy,PartialEq,Eq)] pub enum Event{StartApproved,ProcessStarted,CancelApproved,ProcessExited,ProcessFailed}
#[derive(Debug,Clone,Copy,PartialEq,Eq)] pub enum Error{StaleOrGap,ReceiptRequired,InvalidTransition}
#[derive(Debug,Clone,Copy,PartialEq,Eq)] pub struct Session{pub state:State,pub revision:u64}
impl Session{
 pub fn apply(self,revision:u64,event:Event,verified_host_receipt:bool)->Result<Self,Error>{
  if self.revision.checked_add(1)!=Some(revision){return Err(Error::StaleOrGap);}
  if !verified_host_receipt{return Err(Error::ReceiptRequired);}
  use Event::*;use State::*;
  let state=match(self.state,event){
   (Proposed,StartApproved)=>Starting,
   (Starting,ProcessStarted)=>Running,
   (Starting|Running,CancelApproved)=>Cancelling,
   (Starting|Running|Cancelling,ProcessExited)=>Exited,
   (Starting|Running|Cancelling,ProcessFailed)=>Failed,
   _=>return Err(Error::InvalidTransition)
  };
  Ok(Self{state,revision})
 }
}
#[cfg(test)] mod tests{
 use super::*;
 #[test] fn transitions(){let s=Session{state:State::Proposed,revision:0};assert_eq!(s.apply(1,Event::StartApproved,false),Err(Error::ReceiptRequired));let s=s.apply(1,Event::StartApproved,true).unwrap();assert!(s.apply(3,Event::ProcessStarted,true).is_err());let s=s.apply(2,Event::ProcessStarted,true).unwrap();let s=s.apply(3,Event::ProcessExited,true).unwrap();assert_eq!(s.state,State::Exited);assert!(s.apply(4,Event::StartApproved,true).is_err());}
}
