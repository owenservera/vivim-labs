//! Closed-loop proof-mining simulation over the real standard calculator.
use sha2::{Digest,Sha256};
use vivim_math_compatibility_lab::{*,extensions,standard::*};
use vivim_math_engines_lab::MathError;
type Hash=[u8;32];
fn hash(parts:&[&[u8]])->Hash{
    let mut h=Sha256::new();
    for p in parts {h.update((p.len() as u64).to_be_bytes());h.update(p);}
    h.finalize().into()
}
const V2:Requirement=Requirement{key:Key{id:"math.scalar.percent",rev:2,domain:DOMAIN},signature:UNARY};
fn good(c:&mut Call<'_>,v:&[Value])->Result<Value>{c.call(DIV,&[v[0].clone(),number(100,1)])}
fn hidden(c:&mut Call<'_>,v:&[Value])->Result<Value>{
    if v[0]==number(37,1){Ok(number(38,100))}else{good(c,v)}
}
fn broken(_: &mut Call<'_>,_:&[Value])->Result<Value>{Ok(number(0,1))}
fn base()->Registry{let mut r=Registry::baseline();r.admit(extensions::providers(),&extensions::reference_fixtures()).unwrap();r}
fn candidate(mode:u8)->Result<Registry>{
    let mut r=base();
    let p=Provider{contract:V2,requires:&[DIV],run:match mode{1=>hidden,2=>broken,_=>good}};
    r.admit(vec![p],&[
        Fixture{capability:V2,input:vec![number(25,1)],expected:Ok(number(1,4))},
        Fixture{capability:V2,input:vec![number(1,i128::MAX)],expected:Err(Error::Math(MathError::Overflow))}
    ])?;Ok(r)
}
fn calc(r:&Registry,keys:&[Press])->String{
    let mut c=Calculator::new(r).unwrap();let mut p=c.projection(r).unwrap();
    for &k in keys{p=c.press(r,k).unwrap();}
    p.display
}
// Independently derived expected values: integer/rational equalities and literal
// calculator sequences. Candidate percentages are run through the real registry;
// the other workpieces use the unchanged actual calculator profile.
fn cases(r:&Registry)->Vec<bool>{
    use Press::*;
    let p=r.plan(&[V2]).unwrap();
    let mut out=Vec::new();
    for n in [25,0,-10,37,99,100,101,200,1234,-37,1,777]{
        let actual=r.call(&p,V2,&[number(n,1)],64);
        let pass=match actual{Ok(Value::Number(q))=>q.numerator()*100==n*q.denominator(),_=>false};
        out.push(pass);
    }
    out.extend([
        calc(r,&[Point,Digit(1),Operator(Binary::Add),Point,Digit(2),Equals])=="0.3",
        calc(r,&[Digit(2),Digit(0),Digit(0),Operator(Binary::Add),Digit(1),Digit(0),Percent,Equals])=="220",
        calc(r,&[Digit(1),Operator(Binary::Divide),Digit(3),Equals,Operator(Binary::Multiply),Digit(3),Equals])=="1",
        calc(r,&[Digit(2),Sqrt])=="~1.414213",
        calc(r,&[Digit(4),MemoryStore,AllClear,MemoryRecall,Square])=="16",
        calc(r,&[Digit(2),Operator(Binary::Add),Digit(3),Equals,Undo])=="3",
    ]);
    out
}
#[derive(Clone,Copy)]
struct Scenario{
    name:&'static str, mode:u8, stale:bool, forged:bool, withheld:bool, colluders:u8,
    offline:u8, replay:bool, revoked:bool, fork:bool, tampered:bool, valid:bool
}
const NORMAL:Scenario=Scenario{name:"valid",mode:0,stale:false,forged:false,withheld:false,colluders:0,offline:0,replay:false,revoked:false,fork:false,tampered:false,valid:true};
const SCENARIOS:&[Scenario]=&[
    NORMAL,
    Scenario{name:"hidden_percent_bug",mode:1,valid:false,..NORMAL},
    Scenario{name:"smoke_failure",mode:2,valid:false,..NORMAL},
    Scenario{name:"stale_dependency",stale:true,valid:false,..NORMAL},
    Scenario{name:"forged_pass_transcript",mode:1,forged:true,valid:false,..NORMAL},
    Scenario{name:"withheld_evidence",withheld:true,valid:false,..NORMAL},
    Scenario{name:"three_colluding_validators",mode:1,colluders:3,valid:false,..NORMAL},
    Scenario{name:"two_offline_validators",offline:2,..NORMAL},
    Scenario{name:"three_offline_validators",offline:3,..NORMAL},
    Scenario{name:"replayed_old_receipt",replay:true,valid:false,..NORMAL},
    Scenario{name:"revoked_provider",revoked:true,valid:false,..NORMAL},
    Scenario{name:"equivocating_head",fork:true,valid:false,..NORMAL},
    Scenario{name:"tampered_block",tampered:true,valid:false,..NORMAL},
    Scenario{name:"valid_rollback_requalify",..NORMAL}
];
#[derive(Clone,Copy,Debug)]
enum Policy{HashOnly,SmokeReplay,FullReplay,Majority,Supermajority,Optimistic,Dag,Checkpoint,Certificate,Hybrid}
const POLICIES:&[Policy]=&[Policy::HashOnly,Policy::SmokeReplay,Policy::FullReplay,Policy::Majority,Policy::Supermajority,Policy::Optimistic,Policy::Dag,Policy::Checkpoint,Policy::Certificate,Policy::Hybrid];
#[derive(Clone)]
struct Block { parent:Hash, dependency:Hash, epoch:u64, candidate:Hash, evidence:Hash, body:Hash }
impl Block{
    fn digest(&self)->Hash{hash(&[b"math-proof-v1",&self.parent,&self.dependency,&self.epoch.to_be_bytes(),&self.candidate,&self.evidence])}
}
struct Trial{ admitted:bool, actual:Vec<bool>, claimed:Vec<bool>, block:Block, head:Hash, dependency:Hash, epoch:u64, s:Scenario }
impl Trial{
    fn new(s:Scenario)->Self{
        let r=candidate(s.mode);let admitted=r.is_ok();
        let actual=r.as_ref().map(cases).unwrap_or_default();
        let claimed=if s.withheld{vec![]}else if s.forged{vec![true;actual.len()]}else{actual.clone()};
        let evidence=hash(&[&claimed.iter().map(|x|u8::from(*x)).collect::<Vec<_>>()]);
        let head=hash(&[b"active-head"]);let dependency=hash(&[b"active-division-v1"]);
        let mut block=Block{parent:if s.fork{hash(&[b"other-head"])}else{head},dependency:if s.stale{hash(&[b"old-division"])}else{dependency},epoch:if s.replay{1}else{2},candidate:hash(&[b"percent-v2",&[s.mode]]),evidence,body:[0;32]};
        block.body=block.digest();if s.tampered{block.body[0]^=1;}
        Self{admitted,actual,claimed,block,head,dependency,epoch:2,s}
    }
    fn integrity(&self)->bool{self.block.digest()==self.block.body}
    fn bound(&self)->bool{self.block.parent==self.head&&self.block.dependency==self.dependency&&self.block.epoch==self.epoch&&!self.s.revoked}
    fn available(&self)->bool{!self.claimed.is_empty()&&self.block.evidence==hash(&[&self.claimed.iter().map(|x|u8::from(*x)).collect::<Vec<_>>()])}
}
#[derive(Default)]
struct Cost{cases:u64,votes:u64,hashes:u64}
#[derive(Clone,Copy,PartialEq,Eq,Debug)]
enum Decision{Accept,Reject,Defer}
fn replay(t:&Trial,n:usize,c:&mut Cost)->bool{
    c.cases+=n as u64;t.actual.len()>=n&&t.actual.iter().take(n).all(|v|*v)
}
fn votes(t:&Trial,c:&mut Cost,threshold:u8)->Decision{
    let online=5-t.s.offline;let mut yes=0;
    for i in 0..online{c.votes+=1;if i<t.s.colluders{yes+=1;}else if replay(t,t.actual.len(),c){yes+=1;}}
    if yes>=threshold{Decision::Accept}else if online<threshold{Decision::Defer}else{Decision::Reject}
}
fn decide(p:Policy,t:&Trial,c:&mut Cost)->Decision{
    use Decision::*;
    c.hashes+=1;if !t.admitted||!t.integrity(){return Reject;}
    let full=t.actual.len();
    let pass=match p{
        Policy::HashOnly=>true,
        Policy::SmokeReplay=>replay(t,1,c),
        Policy::FullReplay=>t.available()&&replay(t,full,c),
        Policy::Majority=>return votes(t,c,3),
        Policy::Supermajority=>return votes(t,c,4),
        // Challenge window: a challenger needs availability and an online peer.
        // Absence of a challenge is not proof. Here one challenger inspects case 37.
        Policy::Optimistic=>{if !t.available()||t.s.offline>=4{return Defer;}c.cases+=1;t.actual.get(3)==Some(&true)},
        Policy::Dag=>t.bound()&&t.available()&&t.claimed.iter().all(|x|*x),
        // Revalidate whole current closure against anchored head.
        Policy::Checkpoint=>t.bound()&&t.available()&&replay(t,full,c),
        // Check a compact arithmetic certificate's claims by independent replay;
        // simulated certificate verifier, not a SNARK or formal proof.
        Policy::Certificate=>{c.hashes+=1;t.available()&&t.claimed==t.actual&&replay(t,full,c)},
        // Bound dependency evidence, verify changed closure, require two
        // distinct online honest witnesses in this model; do not use votes as truth.
        Policy::Hybrid=>{if !t.bound()||!t.available(){return Reject;}
            if !replay(t,full,c)||t.claimed!=t.actual{return Reject;}
            c.votes+=u64::from(5-t.s.offline);
            if 5-t.s.offline<2{return Defer;}true}
    };if pass{Accept}else{Reject}
}
fn main(){
    let trials:Vec<_>=SCENARIOS.iter().copied().map(Trial::new).collect();
    println!("scenario_count={} cases_per_admitted_candidate={}",trials.len(),trials[0].actual.len());
    println!("policy,correct_accept,false_accept,correct_reject,false_reject,deferred_valid,deferred_invalid,case_checks,votes,hash_checks");
    for &p in POLICIES{
        let mut c=Cost::default();let(mut ca,mut fa,mut cr,mut fr,mut dv,mut di)=(0,0,0,0,0,0);
        for t in &trials{let d=decide(p,t,&mut c);match(t.s.valid,d){
            (true,Decision::Accept)=>ca+=1,(false,Decision::Accept)=>fa+=1,
            (true,Decision::Reject)=>fr+=1,(false,Decision::Reject)=>cr+=1,
            (true,Decision::Defer)=>dv+=1,(false,Decision::Defer)=>di+=1}}
        println!("{p:?},{ca},{fa},{cr},{fr},{dv},{di},{},{},{}",c.cases,c.votes,c.hashes);
    }
    println!("DETAIL policy=Hybrid");
    for t in &trials{println!("{} {:?} admitted={} replay_cases={} actual_pass={}",t.s.name,decide(Policy::Hybrid,t,&mut Cost::default()),t.admitted,t.actual.len(),t.actual.iter().filter(|x|**x).count());}
    // Reversible activation: keep approved immutable snapshot, stage a bad
    // revision, reject it, and recover the prior approved binding and value.
    let old=base();let p=old.plan(&[extensions::PERCENT]).unwrap();
    assert_eq!(old.call(&p,extensions::PERCENT,&[number(37,1)],64),Ok(number(37,100)));
    let rejected=Trial::new(SCENARIOS[1]);
    assert_eq!(decide(Policy::Hybrid,&rejected,&mut Cost::default()),Decision::Reject);
    assert_eq!(old.call(&p,extensions::PERCENT,&[number(37,1)],64),Ok(number(37,100)));
    assert!(trials.iter().all(|t|decide(Policy::Hybrid,t,&mut Cost::default())!=Decision::Accept||t.s.valid));
    println!("rollback_to_known_snapshot=PASS");
}
