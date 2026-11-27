//! Generic external native seed development probe. One entrant per binary,
//! fixed Win64 ABI; no changes to the evaluator for different contestants.
//! Public development checks are not sealed holdouts or qualification.
#[cfg(not(target_arch="x86_64"))]
compile_error!("x86-64 host required");
use core::arch::global_asm;
use sha2::{Digest,Sha256};
global_asm!(include_str!("../../src/abi_probe.S"));
global_asm!(include_str!(concat!(env!("OUT_DIR"),"/seed.S")));
type Seed=unsafe extern "win64" fn(i64,i64,u32,*mut i64)->u32;
unsafe extern "win64"{
    fn vivim_seed_v1(a:i64,b:i64,op:u32,out:*mut i64)->u32;
    fn r003_abi_probe(seed:Seed,a:i64,b:i64,op:u32,out:*mut i64)->u64;
}
fn reference(a:i64,b:i64,op:u32)->(u32,Option<i64>){
    let n=match op{
        0=>a.checked_add(b),1=>a.checked_sub(b),2=>a.checked_mul(b),
        3=>{if b==0{return(2,None)}a.checked_div(b)},
        4=>a.checked_neg(),5=>{if b==0{return(2,None)}a.checked_rem(b)},
        6=>Some(match a.cmp(&b){std::cmp::Ordering::Less=>-1,std::cmp::Ordering::Equal=>0,std::cmp::Ordering::Greater=>1}),
        _=>return(3,None)
    };match n{Some(v)=>(0,Some(v)),None=>(1,None)}
}
fn check(a:i64,b:i64,op:u32){
    const GUARD:i64=0x5a6a_7a4a_3a2a_1a0a;
    const SENTINEL:i64=-987654321;
    let mut buf=[GUARD,SENTINEL,GUARD];
    // SAFETY: review and externally enforced isolation required before
    // compiling/running supplied native code. Valid ABI/output pointer.
    let report=unsafe{r003_abi_probe(vivim_seed_v1,a,b,op,buf.as_mut_ptr().add(1))};
    assert_eq!(report>>32,0,"ABI violation");
    assert_eq!((buf[0],buf[2]),(GUARD,GUARD),"output canaries");
    let status=report as u32;
    if status!=0{assert_eq!(buf[1],SENTINEL,"output modified on error");}
    assert_eq!((status,(status==0).then_some(buf[1])),reference(a,b,op),"a={a} b={b} op={op}");
}
fn next(x:&mut u64)->u64{*x^=*x<<13;*x^=*x>>7;*x^=*x<<17;*x}
fn hex(v:&[u8])->String{v.iter().map(|b|format!("{b:02x}")).collect()}
fn main(){
    let boundaries=[i64::MIN,i64::MIN+1,i64::MIN/2,-(1i64<<32)-1,-(1i64<<32),-65536,-2,-1,0,1,2,65536,(1i64<<32)-1,1i64<<32,i64::MAX/2,i64::MAX-1,i64::MAX];
    let mut n=0;for a in boundaries{for b in boundaries{for op in 0..=7{check(a,b,op);n+=1;}}}
    let mut rng=0xd89c_f23a_b76e_05d1u64;
    for i in 0..4096{let a=next(&mut rng)as i64;let b=match i%4{0=>next(&mut rng)as i64,1=>next(&mut rng)as i32 as i64,2=>next(&mut rng)as i16 as i64,_=>boundaries[(next(&mut rng)as usize)%boundaries.len()]};
        for op in 0..=6{check(a,b,op);n+=1;}
    }
    let exe=std::fs::read(std::env::current_exe().unwrap()).unwrap();
    println!("{{\"schema\":\"vivim.math.native-development-report/1\",\"status\":\"DEV_PASS_UNQUALIFIED\",\"checks\":{n},\"seed_sha256\":\"{}\",\"executable_sha256\":\"{}\"}}",hex(&Sha256::digest(include_bytes!(concat!(env!("OUT_DIR"),"/seed.S")))),hex(&Sha256::digest(exe)));
}
