//! Aggregate-only, read-only resources. Host must authorize each public request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuTicks { pub idle:u64, pub kernel:u64, pub user:u64 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Memory { pub total_bytes:u64, pub available_bytes:u64 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capacity { pub available_bytes:u64, pub total_bytes:u64, pub free_bytes:u64 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeError { Unsupported, InvalidDrive, Os(u32), InvalidSample }

/// GetSystemTimes kernel time includes idle. Return basis points, not fabricated
/// zero for first, reset, inconsistent or zero-duration sample.
pub fn cpu_busy_basis_points(a:CpuTicks,b:CpuTicks)->Option<u16>{
    let idle=b.idle.checked_sub(a.idle)?;
    let kernel=b.kernel.checked_sub(a.kernel)?;
    let user=b.user.checked_sub(a.user)?;
    let total=kernel.checked_add(user)?;
    if total==0 || idle>kernel { return None; }
    Some((((total-idle) as u128*10_000)/total as u128) as u16)
}
#[cfg(windows)]
mod win {
    use super::*;
    use windows_sys::Win32::Foundation::{FILETIME,GetLastError};
    use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx,MEMORYSTATUSEX};
    use windows_sys::Win32::System::Threading::GetSystemTimes;
    use windows_sys::Win32::Storage::FileSystem::{GetDiskFreeSpaceExW,GetDriveTypeW};
    fn error()->ProbeError { ProbeError::Os(unsafe{GetLastError()}) }
    fn ticks(t:FILETIME)->u64 { (u64::from(t.dwHighDateTime)<<32)|u64::from(t.dwLowDateTime) }
    pub fn cpu()->Result<CpuTicks,ProbeError>{
        // GetSystemTimes covers the calling processor group on multi-group hosts.
        let mut idle:FILETIME=unsafe{std::mem::zeroed()};
        let mut kernel=idle; let mut user=idle;
        if unsafe{GetSystemTimes(&mut idle,&mut kernel,&mut user)}==0 {return Err(error());}
        Ok(CpuTicks{idle:ticks(idle),kernel:ticks(kernel),user:ticks(user)})
    }
    pub fn memory()->Result<Memory,ProbeError>{
        let mut m:MEMORYSTATUSEX=unsafe{std::mem::zeroed()};
        m.dwLength=std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        if unsafe{GlobalMemoryStatusEx(&mut m)}==0{return Err(error());}
        if m.ullAvailPhys>m.ullTotalPhys{return Err(ProbeError::InvalidSample);}
        Ok(Memory{total_bytes:m.ullTotalPhys,available_bytes:m.ullAvailPhys})
    }
    /// Only a caller-authorized drive letter; never enumerate or read files.
    pub fn capacity(drive:char)->Result<Capacity,ProbeError>{
        if !drive.is_ascii_alphabetic(){return Err(ProbeError::InvalidDrive);}
        let root=[drive.to_ascii_uppercase() as u16,58,92,0];
        if unsafe{GetDriveTypeW(root.as_ptr())}!=3{return Err(ProbeError::Unsupported);}
        let (mut available,mut total,mut free)=(0u64,0u64,0u64);
        if unsafe{GetDiskFreeSpaceExW(root.as_ptr(),&mut available,&mut total,&mut free)}==0{return Err(error());}
        if total==0||available>total||available>free{return Err(ProbeError::InvalidSample);}
        // Free bytes may exceed caller-quota total bytes; keep semantics separate.
        Ok(Capacity{available_bytes:available,total_bytes:total,free_bytes:free})
    }
}
#[cfg(windows)] pub use win::{cpu,memory,capacity};
#[cfg(not(windows))] pub fn cpu()->Result<CpuTicks,ProbeError>{Err(ProbeError::Unsupported)}
#[cfg(not(windows))] pub fn memory()->Result<Memory,ProbeError>{Err(ProbeError::Unsupported)}
#[cfg(not(windows))] pub fn capacity(_:char)->Result<Capacity,ProbeError>{Err(ProbeError::Unsupported)}
#[cfg(test)] mod tests{
 use super::*;
 #[test] fn busy_and_reset(){
  let a=CpuTicks{idle:10,kernel:20,user:10};
  let b=CpuTicks{idle:20,kernel:40,user:30};
  assert_eq!(cpu_busy_basis_points(a,b),Some(7500));
  assert_eq!(cpu_busy_basis_points(b,a),None);
  assert_eq!(cpu_busy_basis_points(a,a),None);
 }
 #[cfg(windows)] #[test] fn live_memory(){let m=memory().unwrap();assert!(m.total_bytes>0&&m.available_bytes<=m.total_bytes);}
}
