//! R002: closed-loop adversarial evolution of the standard calculator only.
//! Simulation: no real consensus, signatures, self-modifying code or Host effects.
use sha2::{Digest,Sha256};
use vivim_math_compatibility_lab::{*,extensions,standard::*};
use vivim_math_engines_lab::{MathError,Rational};
use std::collections::BTreeSet;

type Hash=[u8;32];
fn hash(parts:&[&[u8]])->Hash{
    let mut h=Sha256::new();
    for p in parts{h.update((p.len() as u64).to_be_bytes());h.update(p);}
    h.finalize().into()
}
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]
enum Op{Add,Sub,Mul,Div}
const OPS:[Op;4]=[Op::Add,Op::Sub,Op::Mul,Op::Div];
fn primitive(op:Op)->Requirement{match op{Op::Add=>ADD,Op::Sub=>SUB,Op::Mul=>MUL,Op::Div=>DIV}}
fn req_v2(op:Op)->Requirement{let mut r=primitive(op);r.key.rev=2;r}
const DEPS:&[Requirement]=&[ADD,SUB,MUL,DIV,NEG];
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]
struct Case{op:Op,x:i128,y:i128}
fn c(op:Op,x:i128,y:i128)->Case{Case{op,x,y}}
fn inputs(k:Case)->[Value;2]{[number(k.x,1),number(k.y,1)]}
fn oracle(k:Case)->Result<Value>{
    let (x,y)=(k.x,k.y);
    match k.op{
        Op::Add=>x.checked_add(y).map(|n|number(n,1)).ok_or(MathError::Overflow.into()),
        Op::Sub=>x.checked_sub(y).map(|n|number(n,1)).ok_or(MathError::Overflow.into()),
        Op::Mul=>x.checked_mul(y).map(|n|number(n,1)).ok_or(MathError::Overflow.into()),
        Op::Div=>if y==0{Err(MathError::DivisionByZero.into())}else{Ok(Value::Number(Rational::new(x,y)?))}
    }
}
// Independent bounded algebraic checker: compare the returned reduced
// fraction to the defining relation; no registered operation as oracle.
fn invariant(k:Case,actual:&Result<Value>)->bool{
    if k.op==Op::Div&&k.y==0{return *actual==Err(MathError::DivisionByZero.into());}
    let Ok(Value::Number(q))=actual else{return false;};
    let (n,d)=(q.numerator(),q.denominator());let (x,y)=(k.x,k.y);
    match k.op{Op::Add=>n==(x+y)*d,Op::Sub=>n==(x-y)*d,Op::Mul=>n==x*y*d,Op::Div=>n*y==x*d}
}
fn base()->Registry{let mut r=Registry::baseline();r.admit(extensions::providers(),&extensions::reference_fixtures()).unwrap();r}
fn core_call(c:&mut Call<'_>,op:Op,v:&[Value])->Result<Value>{c.call(primitive(op),v)}
fn mutate<const O:u8,const M:u8>(ctx:&mut Call<'_>,v:&[Value])->Result<Value>{
    let op=OPS[O as usize];
    let x=v[0].number()?;let y=v[1].number()?;
    let (a,b)=(x.numerator(),y.numerator());
    if x.denominator()==1&&y.denominator()==1{
        let fault=match M{0=>a==7&&b==1,1=>a<0&&b==-a,2=>a==0&&b==9,_=>false};
        if fault{
            let good=core_call(ctx,op,v)?;
            return ctx.call(ADD,&[good,number(1,1)]);
        }
    }
    core_call(ctx,op,v)
}
fn mutant_fn(op:Op,m:u8)->Eval{
    match(op,m){
        (Op::Add,0)=>mutate::<0,0>,(Op::Add,1)=>mutate::<0,1>,(Op::Add,2)=>mutate::<0,2>,
        (Op::Sub,0)=>mutate::<1,0>,(Op::Sub,1)=>mutate::<1,1>,(Op::Sub,2)=>mutate::<1,2>,
        (Op::Mul,0)=>mutate::<2,0>,(Op::Mul,1)=>mutate::<2,1>,(Op::Mul,2)=>mutate::<2,2>,
        (Op::Div,0)=>mutate::<3,0>,(Op::Div,1)=>mutate::<3,1>,(Op::Div,2)=>mutate::<3,2>,
        _=>panic!("bounded mutation selector")
    }
}
fn recipe<const N:u8>(ctx:&mut Call<'_>,v:&[Value])->Result<Value>{
    match N{
        0=>ctx.call(ADD,v),1=>ctx.call(SUB,v),2=>ctx.call(MUL,v),3=>ctx.call(DIV,v),
        4=>{let neg=ctx.call(NEG,&[v[1].clone()])?;ctx.call(ADD,&[v[0].clone(),neg])},
        5=>ctx.call(ADD,&[v[1].clone(),v[0].clone()]),
        6=>ctx.call(MUL,&[v[1].clone(),v[0].clone()]),
        7=>Ok(v[0].clone()),8=>Ok(v[1].clone()),9=>Ok(number(0,1)),
        _=>Err(Error::Invalid)
    }
}
const RECIPES:[Eval;10]=[recipe::<0>,recipe::<1>,recipe::<2>,recipe::<3>,recipe::<4>,recipe::<5>,recipe::<6>,recipe::<7>,recipe::<8>,recipe::<9>];
fn fixtures(op:Op,learned:&BTreeSet<Case>)->Vec<Fixture>{
    let r=req_v2(op);let mut out=vec![
        Fixture{capability:r,input:inputs(c(op,2,3)).to_vec(),expected:oracle(c(op,2,3))},
        Fixture{capability:r,input:vec![Value::Text("bad".into()),number(1,1)],expected:Err(Error::Type)}
    ];
    for &k in learned.iter().filter(|k|k.op==op){
        out.push(Fixture{capability:r,input:inputs(k).to_vec(),expected:oracle(k)});
    }out
}
fn stage(op:Op,run:Eval,learned:&BTreeSet<Case>)->Result<Registry>{
    let mut r=base();
    r.admit(vec![Provider{contract:req_v2(op),requires:DEPS,run}],&fixtures(op,learned))?;
    Ok(r)
}
fn execute(r:&Registry,k:Case)->Result<Value>{
    let req=req_v2(k.op);r.call(&r.plan(&[req])?,req,&inputs(k),64)
}
fn universe(op:Op)->Vec<Case>{
    (-10..=10).flat_map(|x|(-10..=10).map(move|y|c(op,x,y))).collect()
}
struct Rng(u64);
impl Rng{fn next(&mut self)->u64{self.0^=self.0<<13;self.0^=self.0>>7;self.0^=self.0<<17;self.0}
fn digit(&mut self)->i128{(self.next()%21) as i128-10}}
#[derive(Clone,Copy,Debug)]
enum Strategy{Uniform,Frontier,Mirror,Inverse,Combinatorial,Novelty,Evolution,Reverse,Committee,AdversaryOfAdversary}
const STRATEGIES:[Strategy;10]=[Strategy::Uniform,Strategy::Frontier,Strategy::Mirror,Strategy::Inverse,Strategy::Combinatorial,Strategy::Novelty,Strategy::Evolution,Strategy::Reverse,Strategy::Committee,Strategy::AdversaryOfAdversary];
const BUDGET:usize=48;
// Candidate generation is independent of planted trigger coordinates. All
// policies receive the same call budget, count unique workpieces, and share
// only the prior round's learned evidence.
fn schedule(s:Strategy,op:Op,seed:u64,learned:&BTreeSet<Case>)->Vec<Case>{
    let mut rng=Rng(seed|1);let mut out=Vec::new();let mut seen=BTreeSet::new();
    let mut push=|x,y|{let k=c(op,x,y);if (-10..=10).contains(&x)&&(-10..=10).contains(&y)&&seen.insert(k){out.push(k);}};
    match s{
        Strategy::Uniform=>{},
        Strategy::Frontier=>{for x in [0,1,-1,10,-10,9,-9]{for y in [0,1,-1,10,-10,9,-9]{push(x,y);}}},
        Strategy::Mirror=>{for i in 1..=10{for (x,y) in [(-i,i),(i,-i),(-i,-i),(i,i)]{push(x,y);}}},
        Strategy::Inverse=>{for x in -10..=10{push(x,1);push(x,-1);push(x,0);}},
        Strategy::Combinatorial=>{for i in 0..21{for j in 0..3{push(i-10,((i*7+j*5)%21)-10);}}},
        Strategy::Novelty=>{for k in learned.iter().filter(|k|k.op==op){for dx in -2..=2{for dy in -2..=2{push(k.x+dx,k.y+dy);}}}},
        Strategy::Evolution=>{let(mut x,mut y)=(rng.digit(),rng.digit());for _ in 0..96{push(x,y);x=(x+(rng.next()%5)as i128-2).clamp(-10,10);y=(y+(rng.next()%5)as i128-2).clamp(-10,10);}},
        Strategy::Reverse=>{for target in [0,1,-1,2,-2]{for x in -10..=10{let y=match op{Op::Add=>target-x,Op::Sub=>x-target,Op::Mul=>if x!=0&&target%x==0{target/x}else{0},Op::Div=>if target!=0&&x%target==0{x/target}else{1}};push(x,y);}}},
        Strategy::Committee=>{for x in -10..=10{push(x,1);push(-x,x);push(0,x);}},
        Strategy::AdversaryOfAdversary=>{for k in learned.iter().filter(|k|k.op==op){push(k.x,k.y);push(-k.x,k.y);push(k.y,k.x);}for x in -10..=10{push(x,0);push(0,x);}}
    }
    for _ in 0..256{push(rng.digit(),rng.digit());}
    drop(push);out.truncate(BUDGET);out
}
fn screen(r:&Registry,k:Case)->bool{let v=execute(r,k);v==oracle(k)&&invariant(k,&v)}
fn benchmark(){
    println!("strategy,mutants_detected,mutant_trials,executed_cases");
    for s in STRATEGIES{
        let(mut detected,mut trials,mut checks)=(0,0,0);
        // Fixed calibration seed independent of evaluation; no learned triggers.
        let learned=BTreeSet::new();
        for op in OPS{for m in 0..3{let r=stage(op,mutant_fn(op,m),&learned).unwrap();
            for seed in [19,73,401]{
                trials+=1;let mut found=false;
                for k in schedule(s,op,seed,&learned){checks+=1;if !screen(&r,k){found=true;break;}}
                if found{detected+=1;}
            }
        }}
        println!("{s:?},{detected},{trials},{checks}");
    }
}
#[derive(Clone,Debug,PartialEq,Eq)]
struct Claim{case:Case,pass:bool}
fn claim_bytes(claims:&[Claim])->Vec<u8>{
    let mut b=Vec::new();for q in claims{b.push(q.case.op as u8);b.extend(q.case.x.to_be_bytes());b.extend(q.case.y.to_be_bytes());b.push(u8::from(q.pass));}b
}
#[derive(Clone)]
struct Receipt{parent:Hash,epoch:u64,identity:Hash,suite:Hash,witness:u8,claims:Vec<Claim>,digest:Hash}
impl Receipt{fn digest(&self)->Hash{hash(&[b"evolve-receipt-v1",&self.parent,&self.epoch.to_be_bytes(),&self.identity,&self.suite,&[self.witness],&claim_bytes(&self.claims)])}}
fn suite_id(cases:&[Case])->Hash{hash(&[b"suite",&claim_bytes(&cases.iter().map(|&case|Claim{case,pass:false}).collect::<Vec<_>>())])}
fn witness(r:&Registry,head:Hash,epoch:u64,id:Hash,cases:&[Case],slot:u8)->Receipt{
    let claims=cases.iter().map(|&case|{let v=execute(r,case);Claim{case,pass:if slot==1{v==oracle(case)}else{invariant(case,&v)}}}).collect();
    let mut p=Receipt{parent:head,epoch,identity:id,suite:suite_id(cases),witness:slot,claims,digest:[0;32]};p.digest=p.digest();p
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Gate{Accept,Reject,Defer}
fn verify(r:&Registry,head:Hash,epoch:u64,id:Hash,cases:&[Case],receipts:&[Receipt])->Gate{
    let mut seen=BTreeSet::new();
    for p in receipts{
        if p.parent!=head||p.epoch!=epoch||p.identity!=id||p.suite!=suite_id(cases)||p.digest!=p.digest()
            ||!matches!(p.witness,1|2)||!seen.insert(p.witness)||p.claims.len()!=cases.len(){return Gate::Reject;}
        for (&k,q) in cases.iter().zip(&p.claims){if q.case!=k||!q.pass||!screen(r,k){return Gate::Reject;}}
    }
    if seen.len()<2{Gate::Defer}else{Gate::Accept}
}
fn verifier_attack_lab(r:&Registry,id:Hash,cases:&[Case],head:Hash,epoch:u64){
    let a=witness(r,head,epoch,id,cases,1);let b=witness(r,head,epoch,id,cases,2);
    let run=|p:&[Receipt]|verify(r,head,epoch,id,cases,p);
    assert_eq!(run(&[a.clone(),b.clone()]),Gate::Accept);
    let mut attacks=Vec::new();
    let mut p=a.clone();p.claims.pop();p.digest=p.digest();attacks.push(p);
    let mut p=a.clone();p.epoch=epoch+1;p.digest=p.digest();attacks.push(p);
    let mut p=a.clone();p.parent[0]^=1;p.digest=p.digest();attacks.push(p);
    let mut p=a.clone();p.identity[0]^=1;p.digest=p.digest();attacks.push(p);
    let mut p=a.clone();p.suite[0]^=1;p.digest=p.digest();attacks.push(p);
    let mut p=a.clone();p.claims[0].pass=false;p.digest=p.digest();attacks.push(p);
    let mut p=a.clone();p.claims.swap(0,1);p.digest=p.digest();attacks.push(p);
    let mut p=a.clone();p.digest[0]^=1;attacks.push(p);
    let count=attacks.len();
    for p in attacks{assert_eq!(run(&[p,b.clone()]),Gate::Reject);}
    assert_eq!(run(&[a.clone(),a.clone()]),Gate::Reject);
    assert_eq!(run(&[a]),Gate::Defer);
    println!("verifier_generated_receipt_mutations_rejected={} duplicate_rejected=1 offline_deferred=1",count);
}
fn closed_loop(){
    let mut learned=BTreeSet::new();let mut head=hash(&[b"self-evolution-genesis"]);let mut epoch=0u64;
    let(mut mined,mut rejected,mut repaired,mut synth_attempts,mut holdout_checks)=(0,0,0,0,0);
    for op in OPS{for m in 0..3{
        let bad=stage(op,mutant_fn(op,m),&BTreeSet::new()).unwrap();
        // Proof miners generate a complete finite search schedule independently
        // of the injected defect. Persist the smallest discovered witness.
        let found=universe(op).into_iter().find(|&k|!screen(&bad,k)).unwrap();
        if learned.insert(found){mined+=1;}
        assert!(stage(op,mutant_fn(op,m),&learned).is_err());rejected+=1;
        let cases=universe(op);
        let mut selected=None;
        // Bounded program synthesis: select a registered composition from
        // the grammar, not a manually chosen patch per mutant.
        for (i,&program) in RECIPES.iter().enumerate(){
            synth_attempts+=1;
            let Ok(r)=stage(op,program,&learned)else{continue;};
            let mut ok=true;for &k in &cases{holdout_checks+=1;ok &= screen(&r,k);}
            if ok{selected=Some((i,r));break;}
        }
        let (i,r)=selected.expect("bounded grammar must contain a repair");
        let id=hash(&[b"compiled-recipe-id",&[op as u8,i as u8]]);
        epoch+=1;
        let a=witness(&r,head,epoch,id,&cases,1);let b=witness(&r,head,epoch,id,&cases,2);
        assert_eq!(verify(&r,head,epoch,id,&cases,&[a.clone(),b.clone()]),Gate::Accept);
        if epoch==1{verifier_attack_lab(&r,id,&cases,head,epoch);}
        // Two maliciously optimistic witnesses for the bad candidate still
        // fail independent local replay, even when their receipts hash correctly.
        let mut a_bad=witness(&bad,head,epoch,id,&cases,1);let mut b_bad=witness(&bad,head,epoch,id,&cases,2);
        for p in [&mut a_bad,&mut b_bad]{for q in &mut p.claims{q.pass=true;}p.digest=p.digest();}
        assert_eq!(verify(&bad,head,epoch,id,&cases,&[a_bad,b_bad]),Gate::Reject);
        head=hash(&[b"admit",&head,&epoch.to_be_bytes(),&id,&a.digest,&b.digest]);
        repaired+=1;
        println!("round={} op={op:?} mutant={} counterexample=({}, {}) recipe={} registry_generation={}",epoch,m,found.x,found.y,i,r.generation());
    }}
    println!("closed_loop mined={mined} bad_reproposals_rejected={rejected} repaired={repaired} synthesis_attempts={synth_attempts} finite_domain_checks={holdout_checks}");
    // Real calculator consumer regression (unchanged v1 interaction profile).
    let r=base();let mut calculator=Calculator::new(&r).unwrap();use Press::*;
    for k in [Point,Digit(1),Operator(Binary::Add),Point,Digit(2),Equals]{calculator.press(&r,k).unwrap();}
    assert_eq!(calculator.projection(&r).unwrap().display,"0.3");
    println!("actual_standard_calculator_regression=PASS");
}
fn main(){benchmark();closed_loop();}
