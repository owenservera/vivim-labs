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
    arithmetic_experiment();
}

const ADD_V2:Requirement=Requirement{key:Key{id:"math.scalar.add",rev:2,domain:DOMAIN},signature:BINARY};
fn add_v2(c:&mut Call<'_>,v:&[Value])->Result<Value>{c.call(ADD,v)}
fn bad_add_v2(c:&mut Call<'_>,v:&[Value])->Result<Value>{
    // A rare defect in a new implementation of the actual ledger primitive.
    if v[0]==number(5,1)&&v[1]==number(1,1){return Ok(number(7,1));}
    c.call(ADD,v)
}
fn arithmetic_registry(bad:bool)->Registry{
    let mut r=base();
    r.admit(vec![Provider{contract:ADD_V2,requires:&[ADD],run:if bad{bad_add_v2}else{add_v2}}],&[
        Fixture{capability:ADD_V2,input:vec![number(2,1),number(3,1)],expected:Ok(number(5,1))},
        Fixture{capability:ADD_V2,input:vec![number(i128::MAX,1),number(1,1)],expected:Err(Error::Math(MathError::Overflow))}
    ]).unwrap();r
}
fn arithmetic(r:&Registry,op:Requirement,a:Value,b:Value)->Result<Value>{
    r.call(&r.plan(&[op])?,op,&[a,b],64)
}
#[derive(Clone,Debug,PartialEq,Eq)]
struct LedgerMath{height:Value,verified_work:Value,remaining:Value,coverage:Value,weighted_work:Value}
fn genesis_math()->LedgerMath{
    LedgerMath{height:number(0,1),verified_work:number(0,1),remaining:number(0,1),coverage:number(1,1),weighted_work:number(0,1)}
}
fn state_bytes(s:&LedgerMath)->Vec<u8>{
    let mut v=Vec::new();
    for x in [&s.height,&s.verified_work,&s.remaining,&s.coverage,&s.weighted_work]{
        let q=x.number().expect("ledger fields are numeric");
        v.extend(q.numerator().to_be_bytes());v.extend(q.denominator().to_be_bytes());
    }v
}
/// Actual consensus-state calculation, not a diagnostic calculation next to it.
/// The registered arithmetic methods produce the state committed into the next block.
fn compute_state(r:&Registry,add:Requirement,prev:&LedgerMath,results:&[bool])->Result<LedgerMath>{
    let mut passed=number(0,1);
    for ok in results{
        if *ok{passed=arithmetic(r,add,passed,number(1,1))?;}
    }
    let total=number(results.len() as i128,1);
    Ok(LedgerMath{
        height:arithmetic(r,add,prev.height.clone(),number(1,1))?,
        verified_work:arithmetic(r,add,prev.verified_work.clone(),passed.clone())?,
        remaining:arithmetic(r,SUB,total.clone(),passed.clone())?,
        coverage:arithmetic(r,DIV,passed.clone(),total)?,
        weighted_work:arithmetic(r,MUL,passed,number(2,1))?,
    })
}
/// Minimal independent safety anchor used only to compare a transition, not to
/// implement normal ledger execution. Small bounded integers in this experiment.
fn check_state(prev:&LedgerMath,next:&LedgerMath,results:&[bool])->bool{
    let passed=results.iter().filter(|v|**v).count() as i128;
    let total=results.len() as i128;
    if total==0||passed!=total{return false;}
    let Ok(h)=prev.height.number() else{return false;};
    let Ok(w)=prev.verified_work.number() else{return false;};
    if h.denominator()!=1||w.denominator()!=1{return false;}
    let Some(height)=h.numerator().checked_add(1) else{return false;};
    let Some(work)=w.numerator().checked_add(passed) else{return false;};
    next.height==number(height,1)&&next.verified_work==number(work,1)
        &&next.remaining==number(0,1)&&next.coverage==number(1,1)
        &&next.weighted_work==number(2*passed,1)
}
fn math_block(head:Hash,s:&LedgerMath,add:Requirement)->Hash{
    hash(&[b"ledger-math-v1",&head,add.key.id.as_bytes(),&add.key.rev.to_be_bytes(),add.key.domain.as_bytes(),&state_bytes(s)])
}
fn arithmetic_experiment(){
    let good_registry=arithmetic_registry(false);
    let faulty_registry=arithmetic_registry(true);
    let suite=cases(&candidate(0).unwrap());
    let mut state=genesis_math();let mut head=hash(&[b"arithmetic-ledger-genesis"]);
    for (r,add) in [(&good_registry,ADD),(&good_registry,ADD_V2),(&good_registry,ADD),(&good_registry,ADD_V2)]{
        let next=compute_state(r,add,&state,&suite).unwrap();
        assert!(check_state(&state,&next,&suite));
        head=math_block(head,&next,add);state=next;
    }
    assert_eq!(state.height,number(4,1));
    assert_eq!(state.verified_work,number(72,1));
    println!("ledger_registered_arithmetic=PASS height=4 verified_work=72 remaining=0 coverage=1 weighted_work=36");
    // The corrupted add affects proof counts in the actual next block state.
    let bad_next=compute_state(&faulty_registry,ADD_V2,&state,&suite).unwrap();
    let honest_next=compute_state(&good_registry,ADD_V2,&state,&suite).unwrap();
    assert_ne!(bad_next,honest_next);
    assert_ne!(math_block(head,&bad_next,ADD_V2),math_block(head,&honest_next,ADD_V2));
    assert!(!check_state(&state,&bad_next,&suite));
    // Naive acceptance from a faulty implementation would commit a different head.
    // Guarded activation preserves the previous head and state, then recovers
    // with the registered predecessor arithmetic (not native arithmetic).
    let previous=head;
    if check_state(&state,&bad_next,&suite){head=math_block(head,&bad_next,ADD_V2);state=bad_next;}
    assert_eq!(head,previous);assert_eq!(state.height,number(4,1));
    println!("ledger_add_v2_fault=WRONG_STATE_AND_HASH naive_accept=UNSAFE guarded_commit=REJECT");
    let recovery=compute_state(&good_registry,ADD,&state,&suite).unwrap();
    assert!(check_state(&state,&recovery,&suite));
    head=math_block(head,&recovery,ADD);
    assert_ne!(head,previous);assert_eq!(recovery.height,number(5,1));
    assert_eq!(recovery.verified_work,number(90,1));
    println!("ledger_registered_predecessor_recovery=PASS height=5 verified_work=90");
}
