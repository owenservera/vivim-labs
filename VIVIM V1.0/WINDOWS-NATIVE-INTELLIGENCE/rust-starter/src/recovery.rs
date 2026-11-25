//! Pure reconciliation projection. Existing durable Host receipt store is
//! authority; this module does not dispatch, retry or persist effects.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct Binding{pub owner:u64,pub generation:u64,pub request:[u8;16],pub digest:[u8;32]}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum State{NotFound,Pending,Applied{receipt:[u8;32]},Rejected{receipt:[u8;32]},Unknown}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Decision{ReadExisting{receipt:[u8;32]},Wait,InspectCurrentState,Conflict,Unavailable}
pub fn recover(expected:Binding,stored:Option<(Binding,State)>)->Decision{
 if expected.owner==0||expected.generation==0||expected.request==[0;16]||expected.digest==[0;32]{return Decision::Conflict;}
 let Some((b,s))=stored else{return Decision::Unavailable;};
 if b!=expected{return Decision::Conflict;}
 match s{
  State::Applied{receipt}|State::Rejected{receipt} if receipt!=[0;32]=>Decision::ReadExisting{receipt},
  State::Pending=>Decision::Wait,
  State::Unknown=>Decision::InspectCurrentState,
  _=>Decision::Unavailable
 }
}
#[cfg(test)]mod tests{
 use super::*;
 fn b()->Binding{Binding{owner:1,generation:2,request:[3;16],digest:[4;32]}}
 #[test]fn settled_recovery_only(){assert_eq!(recover(b(),Some((b(),State::Applied{receipt:[5;32]}))),Decision::ReadExisting{receipt:[5;32]});assert_eq!(recover(b(),Some((b(),State::Pending))),Decision::Wait);}
 #[test]fn no_automatic_retry(){assert_eq!(recover(b(),None),Decision::Unavailable);assert_eq!(recover(b(),Some((b(),State::Unknown))),Decision::InspectCurrentState);assert_eq!(recover(b(),Some((Binding{digest:[8;32],..b()},State::Pending))),Decision::Conflict);assert_eq!(recover(b(),Some((Binding{owner:9,..b()},State::Pending))),Decision::Conflict);}
}
