//! R003 development scaffold, not a sealed tournament.
//! All actual candidate arithmetic crosses a native Win64 ABI.
//! Shared evaluator supplies independent Rust reference arithmetic, not fallback.
#[cfg(not(target_arch="x86_64"))]
compile_error!("R003 seed development currently targets x86-64.");
mod protocol;
mod workspace;
use core::arch::global_asm;
use sha2::{Digest,Sha256};
use std::{io::Write,process::Command};
global_asm!(include_str!("abi_probe.S"));
global_asm!(include_str!("../seeds/P014.S"));
global_asm!(include_str!("../seeds/P018.S"));
global_asm!(include_str!("../seeds/P020.S"));
global_asm!(include_str!("../seeds/P023.S"));
global_asm!(include_str!("../seeds/P004.S"));
global_asm!(include_str!("../seeds/P005.S"));
global_asm!(include_str!("../seeds/P007.S"));
global_asm!(include_str!("../seeds/P012.S"));
global_asm!(include_str!("../seeds/P011.S"));
global_asm!(include_str!("../seeds/P027.S"));
global_asm!(include_str!("../seeds/P028.S"));
global_asm!(include_str!("../seeds/P029.S"));
global_asm!(include_str!("../seeds/P022.S"));
global_asm!(include_str!("../seeds/P026.S"));
global_asm!(include_str!("../seeds/P032.S"));
global_asm!(include_str!(concat!(env!("OUT_DIR"),"/P024.S")));
type Seed=unsafe extern "win64" fn(i64,i64,u32,*mut i64)->u32;
unsafe extern "win64"{
    fn r003_abi_probe(seed:Seed,a:i64,b:i64,op:u32,out:*mut i64)->u64;
    fn p014_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn p018_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn p020_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn p023_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn p004_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn p005_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn p007_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn p012_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    #[cfg(test)] fn r003_bad_gpr(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    #[cfg(test)] fn r003_bad_xmm(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    #[cfg(test)] fn r003_bad_df(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    #[cfg(test)] fn r003_bad_mxcsr(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    #[cfg(test)] fn r003_bad_x87(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn p011_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn p027_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn p028_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn p029_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn p022_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn p024_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn p026_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn p032_seed(a:i64,b:i64,op:u32,out:*mut i64)->u32;
}
const SEEDS:[(u8,Seed,&str);16]=[
    (14,p014_seed,include_str!("../seeds/P014.S")),
    (18,p018_seed,include_str!("../seeds/P018.S")),
    (20,p020_seed,include_str!("../seeds/P020.S")),
    (23,p023_seed,include_str!("../seeds/P023.S")),
    (4,p004_seed,include_str!("../seeds/P004.S")),
    (5,p005_seed,include_str!("../seeds/P005.S")),
    (7,p007_seed,include_str!("../seeds/P007.S")),
    (12,p012_seed,include_str!("../seeds/P012.S")),
    (11,p011_seed,include_str!("../seeds/P011.S")),
    (27,p027_seed,include_str!("../seeds/P027.S")),
    (28,p028_seed,include_str!("../seeds/P028.S")),
    (29,p029_seed,include_str!("../seeds/P029.S")),
    (22,p022_seed,include_str!("../seeds/P022.S")),
    (24,p024_seed,include_str!(concat!(env!("OUT_DIR"),"/P024.S"))),
    (26,p026_seed,include_str!("../seeds/P026.S")),
    (32,p032_seed,include_str!("../seeds/P032.S"))
];
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Fault{Overflow,Zero,Invalid}
type Answer=Result<i64,Fault>;
fn reference(a:i64,b:i64,op:u32)->Answer{
    let checked=match op{
        0=>a.checked_add(b),1=>a.checked_sub(b),2=>a.checked_mul(b),
        3=>{if b==0{return Err(Fault::Zero);}a.checked_div(b)},
        4=>a.checked_neg(),5=>{if b==0{return Err(Fault::Zero);}a.checked_rem(b)},
        6=>Some(match a.cmp(&b){std::cmp::Ordering::Less=>-1,std::cmp::Ordering::Equal=>0,std::cmp::Ordering::Greater=>1}),
        _=>return Err(Fault::Invalid)
    };checked.ok_or(Fault::Overflow)
}
fn call(f:Seed,a:i64,b:i64,op:u32)->Answer{
    const GUARD:i64=0x5a6a_7a4a_3a2a_1a0a;
    const SENTINEL:i64=-987654321;
    let mut buf=[GUARD,SENTINEL,GUARD];
    // SAFETY: source-reviewed statically linked leaf functions; exact Win64
    // argument widths and a valid aligned output pointer. No dynamic code.
    let report=unsafe{r003_abi_probe(f,a,b,op,buf.as_mut_ptr().add(1))};
    assert_eq!(report>>32,0,"ABI preservation violation mask");
    let status=report as u32;
    assert_eq!(buf[0],GUARD,"left output canary");
    assert_eq!(buf[2],GUARD,"right output canary");
    if status!=0{assert_eq!(buf[1],SENTINEL,"output changed on failure");}
    match status{0=>Ok(buf[1]),1=>Err(Fault::Overflow),2=>Err(Fault::Zero),3=>Err(Fault::Invalid),_=>panic!("unexpected ABI status {status}")}
}
fn next(x:&mut u64)->u64{*x^=*x<<13;*x^=*x>>7;*x^=*x<<17;*x}
fn hex(v:&[u8])->String{v.iter().map(|b|format!("{b:02x}")).collect()}
fn check(f:Seed,a:i64,b:i64,op:u32,n:&mut u64){
    assert_eq!(call(f,a,b,op),reference(a,b,op),"op {op}, lhs {a}, rhs {b}");*n+=1;
}
fn seed_development(id:u8){
    let &(_,f,source)=SEEDS.iter().find(|s|s.0==id).expect("implemented seed");
    let boundaries=[i64::MIN,i64::MIN+1,i64::MIN/2,-(1i64<<32)-1,-(1i64<<32),-65536,-2,-1,0,1,2,65536,(1i64<<32)-1,1i64<<32,i64::MAX/2,i64::MAX-1,i64::MAX];
    let mut n=0;
    for a in boundaries{for b in boundaries{for op in 0..=7{check(f,a,b,op,&mut n);}}}
    let mut rng=0xd89c_f23a_b76e_05d1u64;
    // Same disclosed development suite for each entrant; NOT sealed holdout.
    for i in 0..4096{let a=next(&mut rng)as i64;let b=match i%4{0=>next(&mut rng)as i64,1=>next(&mut rng)as i32 as i64,2=>next(&mut rng)as i16 as i64,_=>boundaries[(next(&mut rng)as usize)%boundaries.len()]};
        for op in 0..=6{check(f,a,b,op,&mut n);}
    }
    // Actual experimental arithmetic state, via this entrant's seed (not
    // a production ledger or claimed distinct complete architecture).
    let mut head=[0u8;32];let mut height=0;let mut work=0;
    for cases in [7,19,33,81]{
        let nh=call(f,height,1,0).unwrap();
        let nw=call(f,work,cases,0).unwrap();
        let weighted=call(f,nw,2,2).unwrap();
        let remaining=call(f,200,nw,1).unwrap();
        let q=call(f,weighted,nh,3).unwrap();
        let rem=call(f,weighted,nh,5).unwrap();
        assert_eq!(reference(height,1,0),Ok(nh));
        assert_eq!(reference(work,cases,0),Ok(nw));
        assert_eq!(reference(nw,2,2),Ok(weighted));
        assert_eq!(reference(200,nw,1),Ok(remaining));
        assert_eq!(reference(weighted,nh,3),Ok(q));
        assert_eq!(reference(weighted,nh,5),Ok(rem));
        let mut h=Sha256::new();h.update(head);h.update([id]);for v in [nh,nw,weighted,remaining,q,rem]{h.update(v.to_le_bytes());}head=h.finalize().into();
        height=nh;work=nw;
    }
    assert_eq!((height,work),(4,140));
    let exe=std::fs::read(std::env::current_exe().unwrap()).unwrap();
    println!("SEED P{id:03} DEV_PASS cases={n} chain_transitions={height} source_sha256={} executable_sha256={} preview_head={}",hex(&Sha256::digest(source.as_bytes())),hex(&Sha256::digest(exe)),hex(&head));
}
#[derive(Clone,Copy,Debug,Default)]
struct Evidence{
    independent_seed:bool, correctness:bool, abi:bool, artifact:bool,
    actual_chain_and_rust:bool, prior_regression:bool, recovery:bool,
    sealed_holdout:bool, unsafe_accepts:u64, promotion_improvement:bool
}
fn eligible(e:Evidence)->bool{
    e.independent_seed&&e.correctness&&e.abi&&e.artifact&&e.actual_chain_and_rust&&e.prior_regression&&e.recovery&&e.sealed_holdout&&e.unsafe_accepts==0&&e.promotion_improvement
}
/// Nominal bracket skeleton. None means no qualified entrant; a vacant slot
/// does not qualify its opponent. The evaluator supplies real evidence.
fn decide_match(a:Option<(u8,Evidence)>,b:Option<(u8,Evidence)>)->Option<u8>{
    let a=a.filter(|x|eligible(x.1));let b=b.filter(|x|eligible(x.1));
    match(a,b){(Some(x),None)=>Some(x.0),(None,Some(y))=>Some(y.0),_=>None}
    // Two eligible contestants await a predeclared measured tie-breaker.
}
fn main(){
    let args:Vec<_>=std::env::args().collect();
    if args.get(1).map(String::as_str)==Some("scaffold"){
        workspace::materialize(std::path::Path::new(args.get(2).expect("new workspace path"))).expect("create entrant workspaces");return;
    }
    assert_eq!(workspace::roster().expect("valid roster").len(),64);
    if args.get(1).map(String::as_str)==Some("seed"){
        seed_development(args.get(2).expect("entrant id").parse().unwrap());return;
    }
    assert!(decide_match(Some((22,Evidence::default())),None).is_none());
    let positions=protocol::first_round_positions();
    assert_eq!(positions.len(),32);
    println!("BRACKET conferences=4 first_round_positions={}",positions.len());
    let exe=std::env::current_exe().unwrap();
    let mut passed=0;
    for (id,_,_) in SEEDS{
        // Source-reviewed candidates in separate child processes. Process
        // isolation is crash containment, NOT a security sandbox.
        let out=Command::new(&exe).args(["seed",&id.to_string()]).output().unwrap();
        std::io::stdout().write_all(&out.stdout).unwrap();
        std::io::stderr().write_all(&out.stderr).unwrap();
        if out.status.success(){passed+=1;}else{println!("SEED P{id:03} DEV_FAIL exit={:?}",out.status.code());}
    }
    println!("ROSTER designed=32 reserved=32 native_seed_development={passed}/{} qualified=0 evolution_heads_advanced=0",SEEDS.len());
    assert_eq!(passed,SEEDS.len());
}
#[cfg(test)]
mod tests{
    use super::*;
    fn full()->Evidence{Evidence{independent_seed:true,correctness:true,abi:true,artifact:true,actual_chain_and_rust:true,prior_regression:true,recovery:true,sealed_holdout:true,unsafe_accepts:0,promotion_improvement:true}}
    #[test]fn abi_negative_controls(){
        let controls:[(Seed,u64);5]=[(r003_bad_gpr,1),(r003_bad_xmm,2),(r003_bad_df,4),(r003_bad_mxcsr,8),(r003_bad_x87,16)];
        for (seed,mask) in controls{
            let mut out=17;
            let got=unsafe{r003_abi_probe(seed,1,2,0,&mut out)};
            assert_eq!(got>>32,mask);assert_eq!(out,17);
        }
    }
    #[test]fn advancement_fail_closed(){
        assert!(!eligible(Evidence::default()));assert!(eligible(full()));
        let mut e=full();e.promotion_improvement=false;assert!(!eligible(e));
        let mut e=full();e.unsafe_accepts=1;assert!(!eligible(e));
        let mut e=full();e.abi=false;assert!(!eligible(e));
        let mut e=full();e.sealed_holdout=false;assert!(!eligible(e));
    }
    #[test]fn no_automatic_winner(){assert_eq!(decide_match(None,Some((22,Evidence::default()))),None);assert_eq!(decide_match(Some((22,full())),Some((24,full()))),None);}
}
