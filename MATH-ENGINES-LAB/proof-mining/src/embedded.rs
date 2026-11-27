//! Executable mathematical ledger experiment. Both the provider and ledger
//! compile into this binary. No production registry, network or signing.
use super::*;

#[derive(Clone, Debug)]
struct Transition {
    previous:Hash, epoch:u64, from:u64, to:u64, result_n:i128, result_d:i128,
    recovery:bool, digest:Hash
}
impl Transition{
    fn digest(&self)->Hash{
        hash(&[b"embedded-math-transition-v1",&self.previous,&self.epoch.to_be_bytes(),
            &self.from.to_be_bytes(),&self.to.to_be_bytes(),&self.result_n.to_be_bytes(),
            &self.result_d.to_be_bytes(),&[u8::from(self.recovery)]])
    }
}
struct Chain{head:Hash,epoch:u64,revision:u64,registry:Registry,history:Vec<Transition>}
fn evaluate(reg:&Registry,revision:u64)->Result<Value>{
    let req=match revision{1=>extensions::PERCENT,2=>V2,_=>return Err(Error::Invalid)};
    reg.call(&reg.plan(&[req])?,req,&[number(37,1)],64)
}
// Minimal separately controlled invariant, not another general math engine.
// An evolving component cannot change the criterion used to admit itself.
fn anchor(v:&Value)->bool{
    matches!(v,Value::Number(q) if q.numerator()==37 && q.denominator()==100)
}
impl Chain{
    fn new()->Self{Self{head:hash(&[b"embedded-genesis-v1"]),epoch:0,revision:1,registry:base(),history:vec![]}}
    fn append(&mut self,to:u64,reg:Registry,recovery:bool)->Result<()>{
        let next=evaluate(&reg,to)?;
        if !anchor(&next){return Err(Error::Invalid);}
        if !recovery{
            let previous=evaluate(&self.registry,self.revision)?;
            if !anchor(&previous)||previous!=next{return Err(Error::Invalid);}
        }else if to!=1 {
            // Lab recovery is explicitly limited to the known genesis provider.
            return Err(Error::Invalid);
        }
        let epoch=self.epoch.checked_add(1).ok_or(Error::Limit)?;
        let q=next.number()?;
        let mut tx=Transition{previous:self.head,epoch,from:self.revision,to,
            result_n:q.numerator(),result_d:q.denominator(),recovery,digest:[0;32]};
        tx.digest=tx.digest();
        self.head=tx.digest;self.epoch=epoch;self.revision=to;self.registry=reg;self.history.push(tx);
        Ok(())
    }
    fn healthy(&self)->bool{evaluate(&self.registry,self.revision).is_ok_and(|v|anchor(&v))}
    fn history_valid(&self)->bool{
        let(mut head,mut rev,mut epoch)=(hash(&[b"embedded-genesis-v1"]),1u64,0u64);
        for tx in &self.history{
            epoch+=1;
            if tx.previous!=head||tx.epoch!=epoch||tx.from!=rev||tx.digest!=tx.digest()
                ||tx.result_n!=37||tx.result_d!=100||(tx.recovery&&tx.to!=1){return false;}
            head=tx.digest;rev=tx.to;
        }
        head==self.head&&rev==self.revision&&epoch==self.epoch
    }
}
pub(super) fn experiment(){
    // Naive self-hosted rule compares the candidate result with its own
    // candidate-generated certificate. The buggy operation agrees with itself.
    let bad=candidate(1).unwrap();let self_claim=evaluate(&bad,2).unwrap();
    assert_eq!(self_claim,number(38,100));
    assert_eq!(evaluate(&bad,2).unwrap(),self_claim);
    println!("embedded_naive_self_verification=FALSE_ACCEPT");
    let mut chain=Chain::new();let head=chain.head;
    assert!(chain.append(2,bad.clone(),false).is_err());
    assert_eq!(chain.head,head);assert_eq!(chain.epoch,0);
    println!("embedded_predecessor_and_anchor=REJECT_BAD");
    for (to,reg) in [(2,candidate(0).unwrap()),(1,base()),(2,candidate(0).unwrap()),(1,base())]{
        chain.append(to,reg,false).unwrap();
        assert!(chain.healthy()&&chain.history_valid());
    }
    println!("embedded_bidirectional_epochs={} history=PASS",chain.epoch);
    chain.append(2,candidate(0).unwrap(),false).unwrap();
    let last=chain.head;
    // Fault injection represents an independently corrupted active binding.
    chain.registry=bad;
    assert!(!chain.healthy());assert!(chain.history_valid());
    assert!(chain.append(2,candidate(0).unwrap(),false).is_err());
    assert_eq!(chain.head,last);
    println!("embedded_active_math_corruption=CHAIN_HALTED");
    chain.append(1,base(),true).unwrap();
    assert!(chain.healthy()&&chain.history_valid());
    assert_ne!(chain.head,last);
    println!("embedded_anchor_recovery=PASS epoch={} active_revision={}",chain.epoch,chain.revision);
    let mut tampered=chain.history[0].clone();tampered.result_n=38;
    assert_ne!(tampered.digest,tampered.digest());
    println!("embedded_transition_tamper=DETECTED");
}
