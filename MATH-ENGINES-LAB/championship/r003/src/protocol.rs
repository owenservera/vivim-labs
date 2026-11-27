//! Constitutional tournament controls: development scaffolding, not authority.
//! The independent evaluator, not the contestant, supplies evidence.
use std::collections::BTreeSet;
pub type Digest=[u8;32];
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Slot(pub u8);
impl Slot{pub fn new(id:u8)->Option<Self>{(1..=64).contains(&id).then_some(Self(id))}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Round{Genesis=1,NativeEvolution,Composition,Adversarial,SelfImprovement,Calculator}
impl Round{pub fn capacity(self)->usize{64>>(self as usize-1)}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Verdict{Qualified,Rejected,Deferred}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Measurement{
    pub suite:Digest,pub conditions:Digest,pub budget:u64,pub measured_cost:u64,
    pub qualified_promotions:u64,pub repetitions:u32
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Policy{
    pub min_repetitions:u32,pub min_new_promotions:u64,pub min_cost_reduction_bps:u32
}
/// Compare the previous and candidate promotion machinery on the SAME
/// independently frozen tasks and conditions. Not execution speed alone.
/// Thresholds must be committed before results are produced.
pub fn improved(old:Measurement,new:Measurement,policy:Policy)->bool{
    if old.suite==[0;32]||old.conditions==[0;32]||old.suite!=new.suite||old.conditions!=new.conditions
        ||old.budget==0||old.budget!=new.budget||old.measured_cost==0||new.measured_cost==0
        ||old.measured_cost>old.budget||new.measured_cost>new.budget
        ||policy.min_repetitions==0||old.repetitions<policy.min_repetitions||new.repetitions<policy.min_repetitions
        ||policy.min_new_promotions==0||policy.min_cost_reduction_bps==0||policy.min_cost_reduction_bps>10_000
    {return false;}
    let yield_gain=new.qualified_promotions.checked_sub(old.qualified_promotions)
        .is_some_and(|d|d>=policy.min_new_promotions);
    let reduction=(old.measured_cost.saturating_sub(new.measured_cost)as u128)*10_000/(old.measured_cost as u128);
    let resource_gain=old.qualified_promotions>0&&new.qualified_promotions>=old.qualified_promotions
        &&new.measured_cost<old.measured_cost&&reduction>=policy.min_cost_reduction_bps as u128;
    yield_gain||resource_gain
}
#[derive(Clone,Debug)]
pub struct Submission{
    pub entrant:Slot,pub round:Round,pub predecessor:Digest,pub proposal:Digest,
    pub executable:Digest,pub abi_contract:Digest,pub independent_suite:Digest,
    pub evaluator_receipt:Digest,pub genesis_obligations:BTreeSet<u8>,
    pub evidence:crate::Evidence,pub old:Measurement,pub new:Measurement
}
/// A verdict here is only a protocol predicate: hashes are identities, not
/// authentication. Trusted evaluator receipt resolution and sealed evaluation
/// are required before invoking this with real evidence.
pub fn verdict(s:&Submission,policy:Policy)->Verdict{
    if s.evidence.unsafe_accepts>0{return Verdict::Rejected;}
    if Slot::new(s.entrant.0).is_none()||[s.predecessor,s.proposal,s.executable,s.abi_contract,s.independent_suite,s.evaluator_receipt].contains(&[0;32])
        ||s.proposal==s.predecessor||s.independent_suite!=s.old.suite||s.independent_suite!=s.new.suite
        ||s.genesis_obligations!=(1..=16).collect::<BTreeSet<_>>()||!crate::eligible(s.evidence)||!improved(s.old,s.new,policy)
    {Verdict::Deferred}else{Verdict::Qualified}
}
/// Four conferences, each seeded 1-v-16 through 8-v-9. This defines only
/// positions; calibration seeding must independently supply actual entrants.
pub fn first_round_positions()->Vec<(Slot,Slot)>{
    let mut out=Vec::new();
    for c in 0..4u8{for i in 1..=8u8{out.push((Slot(c*16+i),Slot(c*16+17-i)));}}
    out
}
/// Safe matches can have no winner. Both eligible entrants need a frozen
/// external ranking; ties remain unresolved rather than arbitrary.
pub fn match_winner(a:Option<(&Submission,Verdict,u64)>,b:Option<(&Submission,Verdict,u64)>)->Option<Slot>{
    let a=a.filter(|x|x.1==Verdict::Qualified);
    let b=b.filter(|x|x.1==Verdict::Qualified);
    match(a,b){(Some(x),None)=>Some(x.0.entrant),(None,Some(y))=>Some(y.0.entrant),
        (Some(x),Some(y)) if x.2>y.2=>Some(x.0.entrant),
        (Some(x),Some(y)) if y.2>x.2=>Some(y.0.entrant),_=>None}
}
#[cfg(test)]
mod tests{
    use super::*;
    fn m(y:u64,c:u64)->Measurement{Measurement{suite:[1;32],conditions:[2;32],budget:100,measured_cost:c,qualified_promotions:y,repetitions:3}}
    fn p()->Policy{Policy{min_repetitions:3,min_new_promotions:1,min_cost_reduction_bps:500}}
    #[test]fn complete_bracket(){let b=first_round_positions();assert_eq!(b.len(),32);let ids:BTreeSet<_>=b.iter().flat_map(|(a,b)|[a.0,b.0]).collect();assert_eq!(ids,(1..=64).collect());assert_eq!(Round::Calculator.capacity(),2);}
    #[test]fn no_gain_no_progress(){assert!(!improved(m(1,50),m(1,50),p()));assert!(improved(m(1,50),m(2,50),p()));assert!(improved(m(2,50),m(2,45),p()));assert!(!improved(m(0,50),m(0,1),p()));assert!(!improved(m(2,50),m(1,1),p()));}
    #[test]fn comparable_measurements_required(){let mut n=m(2,50);n.suite=[3;32];assert!(!improved(m(1,50),n,p()));n=m(2,50);n.budget=101;assert!(!improved(m(1,50),n,p()));n=m(2,50);n.repetitions=1;assert!(!improved(m(1,50),n,p()));}
    #[test]fn vacant_match_has_no_winner(){assert!(match_winner(None,None).is_none());}
}
