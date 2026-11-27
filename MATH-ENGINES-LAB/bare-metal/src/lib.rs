//! Assembly-first experimental Windows x64 backend. Rust only marshals the ABI.
#[cfg(not(all(target_arch="x86_64",target_os="windows")))]
compile_error!("Qualify a separate native ABI for other platforms.");
use core::arch::global_asm;
use vivim_math_engines_lab::{MathError,MathResult,Scalar,Assurance};
global_asm!(include_str!("x86_64_windows.S"));
type Limbs=[u64;2];
unsafe extern "C"{
    fn vivim_native_add128(a:*const Limbs,b:*const Limbs,out:*mut Limbs)->u32;
    fn vivim_native_sub128(a:*const Limbs,b:*const Limbs,out:*mut Limbs)->u32;
    fn vivim_native_neg128(a:*const Limbs,b:*const Limbs,out:*mut Limbs)->u32;
    fn vivim_native_mul128(a:*const Limbs,b:*const Limbs,out:*mut Limbs)->u32;
    fn vivim_native_divrem128(a:*const Limbs,b:*const Limbs,out:*mut [u64;4])->u32;
    fn vivim_native_divrem128_fast(a:*const Limbs,b:*const Limbs,out:*mut [u64;4])->u32;
}
fn pack(x:i128)->Limbs{let b=x.to_le_bytes();[u64::from_le_bytes(b[..8].try_into().unwrap()),u64::from_le_bytes(b[8..].try_into().unwrap())]}
fn unpack(x:Limbs)->i128{let mut b=[0u8;16];b[..8].copy_from_slice(&x[0].to_le_bytes());b[8..].copy_from_slice(&x[1].to_le_bytes());i128::from_le_bytes(b)}
fn status(code:u32)->MathResult<()>{match code{0=>Ok(()),1=>Err(MathError::Overflow),2=>Err(MathError::DivisionByZero),_=>Err(MathError::Domain)}}
type Binary=unsafe extern "C" fn(*const Limbs,*const Limbs,*mut Limbs)->u32;
fn bin(f:Binary,a:i128,b:i128)->MathResult<i128>{
    let a=pack(a);let b=pack(b);let mut out=[0;2];
    // SAFETY: valid disjoint buffers of the documented sizes, Windows x64 ABI.
    status(unsafe{f(&a,&b,&mut out)})?;Ok(unpack(out))
}
pub fn add(a:i128,b:i128)->MathResult<i128>{bin(vivim_native_add128,a,b)}
pub fn sub(a:i128,b:i128)->MathResult<i128>{bin(vivim_native_sub128,a,b)}
pub fn neg(a:i128)->MathResult<i128>{bin(vivim_native_neg128,a,0)}
pub fn mul(a:i128,b:i128)->MathResult<i128>{bin(vivim_native_mul128,a,b)}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Division{Restoring,HardwareFast}
pub fn divrem(backend:Division,a:i128,b:i128)->MathResult<(i128,i128)>{
    let a=pack(a);let b=pack(b);let mut out=[0;4];
    let f=match backend{Division::Restoring=>vivim_native_divrem128,Division::HardwareFast=>vivim_native_divrem128_fast};
    // SAFETY: the assembly accepts two input limbs and four output limbs.
    status(unsafe{f(&a,&b,&mut out)})?;
    Ok((unpack([out[0],out[1]]),unpack([out[2],out[3]])))
}
/// Separate native integer domain. Rust's generic operations bind to assembly
/// without changing the incumbent kernel or substituting its rational domain.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeI128(pub i128);
impl Scalar for NativeI128{
    const DOMAIN:&'static str="integer.i128.native.win64.r001";
    const ASSURANCE:Assurance=Assurance::ExactBounded;
    fn zero()->Self{Self(0)}
    fn one()->Self{Self(1)}
    fn add(self,rhs:Self)->MathResult<Self>{add(self.0,rhs.0).map(Self)}
    fn sub(self,rhs:Self)->MathResult<Self>{sub(self.0,rhs.0).map(Self)}
    fn mul(self,rhs:Self)->MathResult<Self>{mul(self.0,rhs.0).map(Self)}
    fn neg(self)->MathResult<Self>{neg(self.0).map(Self)}
}
#[cfg(test)]
mod abi_tests{
    use super::*;
    #[test]
    fn errors_leave_output_intact(){
        let a=pack(i128::MAX);let b=pack(1);let mut out=[123,456];
        assert_eq!(unsafe{vivim_native_add128(&a,&b,&mut out)},1);assert_eq!(out,[123,456]);
        let a=pack(i128::MIN);let b=pack(-1);
        assert_eq!(unsafe{vivim_native_mul128(&a,&b,&mut out)},1);assert_eq!(out,[123,456]);
        let mut q=[1,2,3,4];
        for f in [vivim_native_divrem128 as unsafe extern "C" fn(*const Limbs,*const Limbs,*mut [u64;4])->u32,vivim_native_divrem128_fast]{
            assert_eq!(unsafe{f(&a,&b,&mut q)},1);assert_eq!(q,[1,2,3,4]);
            assert_eq!(unsafe{f(&a,&pack(0),&mut q)},2);assert_eq!(q,[1,2,3,4]);
        }
    }
}
