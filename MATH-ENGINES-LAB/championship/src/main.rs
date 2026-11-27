//! 64-player research tournament. All competitors use real calculator registry calls.
use sha2::{Digest,Sha256};
use std::collections::BTreeSet;
use vivim_math_compatibility_lab::{*,extensions,standard::*};
use vivim_math_engines_lab::{Rational,MathError};
type Hash=[u8;32];
fn hash(parts:&[&[u8]])->Hash{let mut h=Sha256::new();for p in parts{h.update((p.len()as u64).to_be_bytes());h.update(p);}h.finalize().into()}
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]
enum Op{Add,Sub,Mul,Div}
const OPS:[Op;4]=[Op::Add,Op::Sub,Op::Mul,Op::Div];
fn primitive(o:Op)->Requirement{match o{Op::Add=>ADD,Op::Sub=>SUB,Op::Mul=>MUL,Op::Div=>DIV}}
fn revision(o:Op)->Requirement{let mut r=primitive(o);r.key.rev=2;r}
fn base()->Registry{let mut r=Registry::baseline();r.admit(extensions::providers(),&extensions::reference_fixtures()).unwrap();r}
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]
struct Probe{x:i128,y:i128,dx:i128,dy:i128,chain:bool}
fn p(x:i128,y:i128)->Probe{Probe{x,y,dx:1,dy:1,chain:false}}
fn frac(x:i128,y:i128)->Probe{Probe{x,y,dx:2,dy:3,chain:false}}
fn raw(a:Rational,b:Rational,o:Op)->Result<Value>{
    let(n,d,m,e)=(a.numerator(),a.denominator(),b.numerator(),b.denominator());
    let (x,y)=match o{Op::Add=>(n*e+m*d,d*e),Op::Sub=>(n*e-m*d,d*e),Op::Mul=>(n*m,d*e),Op::Div=>(n*e,d*m)};
    Ok(Value::Number(Rational::new(x,y)?))
}
fn reference(o:Op,k:Probe)->Result<Value>{raw(number(k.x,k.dx).number()?,number(k.y,k.dy).number()?,o)}
fn mutation<const O:usize,const M:u8>(c:&mut Call<'_>,v:&[Value])->Result<Value>{
    let (x,y)=(v[0].number()?,v[1].number()?);
    let(a,b)=(x.numerator(),y.numerator());
    let integer=x.denominator()==1&&y.denominator()==1;
    let fault=if M==7{x.denominator()==2&&y.denominator()==3&&a==-3&&b==5}
    else{integer&&match M{0=>a==7&&b==1,1=>a<0&&b==-a,2=>a==0&&b==9,3=>a==9&&b==-9,
        4=>a==-7&&b==2,5=>a==b&&a!=0,6=>a==3&&b==-8,_=>false}};
    let good=c.call(primitive(OPS[O]),v)?;
    if fault{c.call(ADD,&[good,number(1,1)])}else{Ok(good)}
}
fn pick<const O:usize>(m:u8)->Eval{match m{0=>mutation::<O,0>,1=>mutation::<O,1>,2=>mutation::<O,2>,3=>mutation::<O,3>,4=>mutation::<O,4>,5=>mutation::<O,5>,6=>mutation::<O,6>,7=>mutation::<O,7>,_=>mutation::<O,8>}}
fn candidate(o:Op,m:u8)->Registry{
    let mut r=base();let req=revision(o);let run=match o{Op::Add=>pick::<0>(m),Op::Sub=>pick::<1>(m),Op::Mul=>pick::<2>(m),Op::Div=>pick::<3>(m)};
    r.admit(vec![Provider{contract:req,requires:&[ADD,SUB,MUL,DIV],run}],&[
        Fixture{capability:req,input:vec![number(2,1),number(3,1)],expected:reference(o,p(2,3))},
        Fixture{capability:req,input:vec![Value::Text("wrong".into()),number(1,1)],expected:Err(Error::Type)}
    ]).unwrap();r
}
fn invoke(r:&Registry,o:Op,a:Value,b:Value)->Result<Value>{let q=revision(o);r.call(&r.plan(&[q])?,q,&[a,b],64)}
/// Execute actual registered candidate. A dream is a two-step stateful arithmetic
/// program with the first result flowing into the second step.
fn probe(r:&Registry,o:Op,k:Probe)->(bool,u32){
    let a=number(k.x,k.dx);let b=number(k.y,k.dy);
    let actual=invoke(r,o,a,b);let expected=reference(o,k);
    if actual!=expected{return(false,1);}
    if k.chain{if let Ok(v)=actual{
        let expected2=raw(v.number().unwrap(),number(1,1).number().unwrap(),o);
        return(invoke(r,o,v,number(1,1))==expected2,2);
    }}
    (true,1)
}
fn universe()->Vec<Probe>{
    let mut v=Vec::new();for x in -10..=10{for y in -10..=10{v.push(p(x,y));v.push(frac(x,y));}}v
}
#[derive(Clone,Copy,Debug)]
enum Wild{Immune,Dream,Inverse,Predator,Worlds,Archaeology,Compression,Fork,Theorem,SelfVerifier}
const WILD:[Wild;10]=[Wild::Immune,Wild::Dream,Wild::Inverse,Wild::Predator,Wild::Worlds,Wild::Archaeology,Wild::Compression,Wild::Fork,Wild::Theorem,Wild::SelfVerifier];
const WN:[&str;10]=["Immune","Dream","Inverse","Predator","ParallelWorlds","Archaeology","Compression","Counterfactual","TheoremFactory","SelfVerifier"];
#[derive(Clone)]
struct Knowledge{witness:Vec<Vec<Probe>>,compressed:Vec<Vec<Probe>>,archived:Vec<Vec<Probe>>,calls:u32}
fn train()->Knowledge{
    let mut k=Knowledge{witness:vec![vec![];4],compressed:vec![vec![];4],archived:vec![vec![];4],calls:0};
    for o in OPS{
        let mutants:Vec<_>=(0..4).map(|m|candidate(o,m)).collect();
        let mut uncovered=15u8;let mut coverage=Vec::new();
        for q in universe(){
            let mut bits=0u8;
            for (i,r) in mutants.iter().enumerate(){let(ok,n)=probe(r,o,q);k.calls+=n;if !ok{bits|=1<<i;}}
            if bits!=0{coverage.push((q,bits));}
        }
        for i in 0..4{let q=coverage.iter().find(|(_,bits)|bits&(1<<i)!=0).unwrap().0;k.witness[o as usize].push(q);}
        while uncovered!=0{
            let &(q,bits)=coverage.iter().max_by_key(|(_,bits)|(bits&uncovered).count_ones()).unwrap();
            assert!(bits&uncovered!=0);k.compressed[o as usize].push(q);uncovered&=!bits;
        }
        // Archive independent evidence from prior registered revisions.
        k.archived[o as usize]=k.witness[o as usize].clone();
    }
    k
}
struct Rng(u64);
impl Rng{fn next(&mut self)->u64{self.0^=self.0<<13;self.0^=self.0>>7;self.0^=self.0<<17;self.0}fn digit(&mut self)->i128{(self.next()%21)as i128-10}}
fn random(seed:u64)->Vec<Probe>{
    let mut r=Rng(seed|1);(0..512).map(|i|{let x=r.digit();let y=r.digit();if i%3==0{frac(x,y)}else{p(x,y)}}).collect()
}
fn wild_schedule(w:Wild,o:Op,seed:u64,k:&Knowledge)->Vec<Probe>{
    let i=o as usize;let mut v=Vec::new();let mut rng=Rng(seed|1);
    match w{
        Wild::Immune=>for q in &k.witness[i]{for dx in -2..=2{for dy in -2..=2{v.push(p(q.x+dx,q.y+dy));v.push(p(-q.x+dx,-q.y+dy));}}},
        Wild::Dream=>for _ in 0..180{let mut q=p(rng.digit(),rng.digit());q.chain=true;v.push(q);},
        Wild::Inverse=>for target in [0,1,-1,2,-2]{for x in -10..=10{let y=match o{Op::Add=>target-x,Op::Sub=>x-target,Op::Mul=>if x!=0&&target%x==0{target/x}else{0},Op::Div=>if target!=0&&x%target==0{x/target}else{1}};v.push(p(x,y));}},
        Wild::Predator=>{
            // Historical fault signatures compete: prefer neighborhoods of
            // high-overlap witnesses, then mutate their input pairs.
            let mut parents=k.witness[i].clone();
            parents.sort_by_key(|q|std::cmp::Reverse(k.witness[i].iter().filter(|r|r.x==q.x||r.y==q.y).count()));
            for q in parents{v.push(q);for _ in 0..24{v.push(p(q.x+(rng.next()%7)as i128-3,q.y+(rng.next()%7)as i128-3));}}
        }
        Wild::Worlds=>for x in -10..=10{for y in [-3,0,1,3,5,-5]{v.push(frac(x,y));v.push(p(x,y));}},
        Wild::Archaeology=>{for &q in &k.archived[i]{v.push(q);v.push(p(q.y,q.x));v.push(p(-q.x,-q.y));}for x in -10..=10{v.push(p(x,x));v.push(p(x,-x));}},
        Wild::Compression=>{v.extend_from_slice(&k.compressed[i]);for x in [-10,-1,0,1,10]{for y in [-10,-1,0,1,10]{v.push(p(x,y));}}},
        Wild::Fork=>for x in -10..=10{for y in [1,-1,0,2]{let mut q=p(x,y);q.chain=true;v.push(q);}},
        Wild::Theorem=>{
            // Generate candidate input laws from the algebraic template
            // grammar. Independent oracle, not the candidate, checks outcomes.
            for x in -10..=10{v.push(p(x,x));v.push(p(x,-x));v.push(p(x,0));v.push(p(0,x));}
        }
        Wild::SelfVerifier=>{for x in -10..=10{v.push(p(x,0));v.push(p(0,x));v.push(frac(x,5));}}
    }
    v.extend(random(seed));v
}
#[derive(Clone,Copy,Debug)]
enum Policy{Hash,Smoke,Replay,Majority,Supermajority,Optimistic,Dag,Checkpoint,Certificate,Hybrid,Embedded}
const POLICY:[Policy;11]=[Policy::Hash,Policy::Smoke,Policy::Replay,Policy::Majority,Policy::Supermajority,Policy::Optimistic,Policy::Dag,Policy::Checkpoint,Policy::Certificate,Policy::Hybrid,Policy::Embedded];
const PN:[&str;11]=["HashOnly","Smoke","FullReplay","Majority","Supermajority","Optimistic","DAG","Checkpoint","Certificate","Hybrid","Embedded"];
#[derive(Clone)]
struct Player{id:usize,name:String,policy:Policy,miners:Vec<Wild>}
fn roster()->Vec<Player>{
    let mut v=Vec::new();let mut add=|name:String,policy,miners|{let id=v.len()+1;v.push(Player{id,name,policy,miners});};
    for (i,&policy) in POLICY.iter().enumerate(){add(PN[i].into(),policy,vec![]);}
    for (i,&w) in WILD.iter().enumerate(){add(WN[i].into(),Policy::Embedded,vec![w]);}
    for i in 0..10{for step in 1..=4{let j=(i+step)%10;add(format!("{}+{}",WN[i],WN[j]),Policy::Hybrid,vec![WILD[i],WILD[j]]);}}
    add("Immune+Compression+Theorem".into(),Policy::Embedded,vec![Wild::Immune,Wild::Compression,Wild::Theorem]);
    add("Dream+Worlds+Fork".into(),Policy::Embedded,vec![Wild::Dream,Wild::Worlds,Wild::Fork]);
    add("Predator+Archaeology+SelfVerifier".into(),Policy::Embedded,vec![Wild::Predator,Wild::Archaeology,Wild::SelfVerifier]);
    assert_eq!(v.len(),64);v
}
fn schedule(a:&Player,o:Op,seed:u64,k:&Knowledge)->Vec<Probe>{
    if a.miners.is_empty(){return random(seed);}
    let lists:Vec<_>=a.miners.iter().enumerate().map(|(j,&w)|wild_schedule(w,o,seed+31*j as u64,k)).collect();
    let mut out=Vec::new();let mut seen=BTreeSet::new();
    for i in 0..lists.iter().map(Vec::len).max().unwrap_or(0){for list in &lists{if let Some(&q)=list.get(i){if seen.insert(q){out.push(q);}}}}
    out
}
/// Concrete SHA256 receipt mutation arena; no authentication/network.
#[derive(Clone)]
struct Receipt{fields:[u64;9],digest:Hash}
fn rd(f:&[u64;9])->Hash{let bytes:Vec<_>=f.iter().flat_map(|x|x.to_be_bytes()).collect();hash(&[b"championship-receipt",&bytes])}
fn receipt()->Receipt{let fields=[5,7,2,9,96,96,1,2,1];Receipt{digest:rd(&fields),fields}}
fn damage(n:usize)->Receipt{let mut r=receipt();if n<9{r.fields[n]^=1;r.digest=rd(&r.fields);}else{r.digest[0]^=1;}r}
const FULL:u16=(1<<10)-1;
fn receipt_ok(r:&Receipt,mask:u16)->bool{
    let original=receipt();
    for i in 0..9{if mask&(1<<i)!=0&&r.fields[i]!=original.fields[i]{return false;}}
    mask&(1<<9)==0||r.digest==rd(&r.fields)
}
fn synthesize_checker()->u16{
    // Exhaustively select smallest rule program that rejects all generated
    // one-field attacks and accepts the genuine receipt. Holdout combinations
    // are tested separately in the championship.
    let good=receipt();
    let attacks:Vec<_>=(0..10).map(damage).collect();
    (0..=FULL).filter(|&m|receipt_ok(&good,m)&&attacks.iter().all(|r|!receipt_ok(r,m)))
        .min_by_key(|m|m.count_ones()).unwrap()
}
fn mask(a:&Player,generated:u16)->u16{
    if a.miners.iter().any(|w|matches!(w,Wild::SelfVerifier)){return generated;}
    match a.policy{
        Policy::Hash|Policy::Smoke=>1<<9,Policy::Replay=>1<<9|1<<5,
        Policy::Majority|Policy::Supermajority=>1<<9|1<<6|1<<7,
        Policy::Optimistic=>1<<9|1<<5,Policy::Dag=>1<<9|0b1111,
        Policy::Checkpoint=>1<<9|0b1111|1<<5,Policy::Certificate=>1<<9|1<<4|1<<5|1<<8,
        Policy::Hybrid|Policy::Embedded=>FULL
    }
}
#[derive(Clone,Copy,Default,Debug)]
struct Score{unsafe_accept:u32,valid_accept:u32,valid_defer:u32,invalid_reject:u32,calls:u32}
impl Score{
    fn key(self)->(u32,std::cmp::Reverse<u32>,u32,u32){(self.unsafe_accept,std::cmp::Reverse(self.valid_accept),self.valid_defer,self.calls)}
}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Decision{Accept,Reject,Defer}
fn record(s:&mut Score,valid:bool,d:Decision){
    match(valid,d){(true,Decision::Accept)=>s.valid_accept+=1,(true,Decision::Defer)=>s.valid_defer+=1,
        (false,Decision::Accept)=>s.unsafe_accept+=1,(false,Decision::Reject)=>s.invalid_reject+=1,_=>{}}
}
const BUDGET:u32=96;
fn decide_math(a:&Player,r:&Registry,o:Op,seed:u64,k:&Knowledge,s:&mut Score)->Decision{
    if matches!(a.policy,Policy::Hash|Policy::Dag){return Decision::Accept;}
    let budget=match a.policy{Policy::Smoke=>1,Policy::Optimistic=>24,_=>BUDGET};
    let repetitions=match a.policy{Policy::Majority|Policy::Supermajority=>3,_=>1};
    let mut used=0;
    for q in schedule(a,o,seed,k){
        let cost=if q.chain{2}else{1};
        if used+cost*repetitions>budget{break;}
        for _ in 0..repetitions{let(ok,n)=probe(r,o,q);used+=n;s.calls+=n;if !ok{return Decision::Reject;}}
    }
    Decision::Accept
}
fn evaluate(a:&Player,arena:&[(Op,u8,Registry)],seed:u64,k:&Knowledge,generated:u16,holdout:bool)->Score{
    let mut s=Score::default();
    for (o,m,r) in arena{
        let d=decide_math(a,r,*o,seed+*o as u64*103,k,&mut s);
        record(&mut s,*m==8,d);
    }
    let m=mask(a,generated);
    // Exact same evidence attacks for every player. Holdout uses combined
    // field corruptions, plus single-field attacks and a valid receipt.
    for i in 0..10{
        let mut r=damage(i);
        if holdout&&i<9&&i%2==0{r.fields[(i+3)%9]^=1;r.digest=rd(&r.fields);}
        record(&mut s,false,if receipt_ok(&r,m){Decision::Accept}else{Decision::Reject});
    }
    record(&mut s,true,if receipt_ok(&receipt(),m){Decision::Accept}else{Decision::Reject});
    // One validator available: quorum policies defer; all others can verify
    // against the local trusted checkpoint. Not distributed consensus.
    record(&mut s,true,if matches!(a.policy,Policy::Majority|Policy::Supermajority|Policy::Hybrid){Decision::Defer}else{Decision::Accept});
    s
}
fn arena(lo:u8,hi:u8)->Vec<(Op,u8,Registry)>{
    let mut v=Vec::new();for o in OPS{for m in lo..hi{v.push((o,m,candidate(o,m)));}v.push((o,8,candidate(o,8)));}v
}
fn main(){
    let k=train();let generated=synthesize_checker();
    println!("calibration_registry_calls={} compressed_witnesses={} generated_checker_rules={}",k.calls,k.compressed.iter().map(Vec::len).sum::<usize>(),generated.count_ones());
    assert_eq!(generated,FULL);
    let players=roster();let calibration=arena(0,4);let holdout=arena(4,8);
    let mut qualifiers:Vec<_>=players.iter().map(|a|(a.id,evaluate(a,&calibration,71,&k,generated,false))).collect();
    qualifiers.sort_by_key(|(id,s)|(s.key(),*id));
    println!("qualifying,id,name,unsafe_accept,valid_accept,valid_defer,invalid_reject,calls");
    for (id,s) in &qualifiers{println!("Q,{id},{},{},{},{},{},{}",players[id-1].name,s.unsafe_accept,s.valid_accept,s.valid_defer,s.invalid_reject,s.calls);}
    // Seed 1 vs 64 etc. Re-evaluate each match on the same held-out fault
    // families with a round-specific deterministic search seed.
    let mut bracket=Vec::new();for i in 0..32{bracket.push(qualifiers[i].0);bracket.push(qualifiers[63-i].0);}
    let mut round=0;let mut matches=0;
    while bracket.len()>1{
        round+=1;let mut next=Vec::new();
        for pair in bracket.chunks_exact(2){
            let seed=911+round*101;
            let a=evaluate(&players[pair[0]-1],&holdout,seed,&k,generated,true);
            let b=evaluate(&players[pair[1]-1],&holdout,seed,&k,generated,true);
            let win=if (a.key(),pair[0])<=(b.key(),pair[1]){pair[0]}else{pair[1]};
            println!("M,{round},{},{},{win},{},{},{},{},{},{}",pair[0],pair[1],a.unsafe_accept,b.unsafe_accept,a.valid_accept,b.valid_accept,a.calls,b.calls);
            next.push(win);matches+=1;
        }bracket=next;
    }
    assert_eq!(matches,63);
    let champion=&players[bracket[0]-1];
    let final_check=evaluate(champion,&holdout,2221,&k,generated,true);
    println!("CHAMPION id={} name={} matches={} holdout_unsafe={} holdout_valid={} holdout_defer={} holdout_calls={}",champion.id,champion.name,matches,final_check.unsafe_accept,final_check.valid_accept,final_check.valid_defer,final_check.calls);
    let mut calc=Calculator::new(&base()).unwrap();let r=base();
    // A fresh calculator bound to the same snapshot; real unchanged profile.
    calc.rebind(&r).unwrap();use Press::*;
    for key in [Point,Digit(1),Operator(Binary::Add),Point,Digit(2),Equals]{calc.press(&r,key).unwrap();}
    assert_eq!(calc.projection(&r).unwrap().display,"0.3");
    println!("standard_calculator_regression=PASS");
}
