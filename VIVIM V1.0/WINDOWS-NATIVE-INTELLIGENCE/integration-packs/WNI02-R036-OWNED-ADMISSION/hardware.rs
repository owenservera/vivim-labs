//! Aggregate native hardware topology only: no serial numbers, usernames,
//! process inventory, machine ID, hardware IDs or installed application names.
use crate::resources::ProbeError;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Topology{pub active_logical_processors:u32,pub processor_groups:u16,pub page_bytes:u32,pub allocation_granularity:u32}
#[cfg(windows)]
pub fn topology()->Result<Topology,ProbeError>{
 use windows_sys::Win32::System::SystemInformation::{GetNativeSystemInfo,SYSTEM_INFO};
 use windows_sys::Win32::System::Threading::{GetActiveProcessorCount,GetActiveProcessorGroupCount};
 let mut s:SYSTEM_INFO=unsafe{std::mem::zeroed()};
 unsafe{GetNativeSystemInfo(&mut s)};
 let groups=unsafe{GetActiveProcessorGroupCount()};
 let count=unsafe{GetActiveProcessorCount(0xffff)};
 if groups==0||count==0||s.dwPageSize==0||s.dwAllocationGranularity==0{return Err(ProbeError::InvalidSample);}
 Ok(Topology{active_logical_processors:count,processor_groups:groups,page_bytes:s.dwPageSize,allocation_granularity:s.dwAllocationGranularity})
}
#[cfg(not(windows))]
pub fn topology()->Result<Topology,ProbeError>{Err(ProbeError::Unsupported)}
#[cfg(test)]mod tests{
 use super::*;
 #[cfg(windows)]#[test]fn actual_topology(){let s=topology().unwrap();assert!(s.active_logical_processors>=u32::from(s.processor_groups));assert!(s.page_bytes.is_power_of_two());assert!(s.allocation_granularity>=s.page_bytes);}
 #[cfg(windows)]#[test]fn actual_fixed_fixture_volume(){
  let d=std::env::temp_dir();let p=d.to_str().unwrap();let drive=p.strip_prefix(r"\\?\").unwrap_or(p).chars().next().unwrap();
  let c=crate::resources::capacity(drive).unwrap();
  assert!(c.total_bytes>0&&c.available_bytes<=c.total_bytes&&c.available_bytes<=c.free_bytes);
  assert_eq!(crate::resources::capacity('/'),Err(ProbeError::InvalidDrive));
 }
}
