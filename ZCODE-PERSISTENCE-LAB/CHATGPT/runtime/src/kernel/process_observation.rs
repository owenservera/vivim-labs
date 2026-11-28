//! Read-only independent OS observation of the children of a controller that
//! the qualification runner owns. Holds process handles across parent death.
//! Snapshot coverage is explicitly sampled; not a process containment proof.
use crate::Result;
use serde::{Serialize,Deserialize};
use std::{process::Child,time::Duration};

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct ExitObservation{
    pub pid:u32,pub creation_100ns:u64,
    pub exited:bool,pub exit_code:Option<u32>,
}
#[cfg(windows)]
mod platform{
    use super::*;
    use std::{ffi::c_void,mem::size_of,os::windows::io::AsRawHandle,ptr,time::Instant};
    type Handle=*mut c_void;
    #[repr(C)]
    struct Entry{size:u32,usage:u32,pid:u32,heap:usize,module:u32,threads:u32,parent:u32,priority:i32,flags:u32,exe:[u16;260]}
    #[repr(C)]
    #[derive(Default)]
    struct FileTime{low:u32,high:u32}
    #[link(name="kernel32")]
    unsafe extern "system"{
        fn CreateToolhelp32Snapshot(flags:u32,pid:u32)->Handle;
        fn Process32FirstW(snapshot:Handle,entry:*mut Entry)->i32;
        fn Process32NextW(snapshot:Handle,entry:*mut Entry)->i32;
        fn OpenProcess(access:u32,inherit:i32,pid:u32)->Handle;
        fn WaitForSingleObject(handle:Handle,ms:u32)->u32;
        fn GetExitCodeProcess(handle:Handle,code:*mut u32)->i32;
        fn GetProcessTimes(handle:Handle,creation:*mut FileTime,exit:*mut FileTime,kernel:*mut FileTime,user:*mut FileTime)->i32;
        fn CloseHandle(handle:Handle)->i32;
        fn GetLastError()->u32;
    }
    fn os_error(stage:&str)->String{format!("{stage}: Win32 {}",unsafe{GetLastError()})}
    struct OwnedHandle(Handle);
    impl Drop for OwnedHandle{fn drop(&mut self){unsafe{CloseHandle(self.0);}}}
    fn created(handle:Handle)->Result<u64>{
        let (mut c,mut e,mut k,mut u)=(FileTime::default(),FileTime::default(),FileTime::default(),FileTime::default());
        if unsafe{GetProcessTimes(handle,&mut c,&mut e,&mut k,&mut u)}==0{return Err(os_error("process creation time"));}
        Ok((u64::from(c.high)<<32)|u64::from(c.low))
    }
    struct Worker{pid:u32,created:u64,handle:OwnedHandle}
    impl Worker{
        fn exited(&self,wait:Duration)->Result<bool>{
            // 0xffffffff is INFINITE, never pass it.
            let ms=wait.as_millis().min(u128::from(u32::MAX-1)) as u32;
            match unsafe{WaitForSingleObject(self.handle.0,ms)}{
                0=>Ok(true),258=>Ok(false),_=>Err(os_error("process wait")),
            }
        }
        fn result(&self,wait:Duration)->Result<ExitObservation>{
            let exited=self.exited(wait)?;
            let exit_code=if exited{
                let mut code=0;
                if unsafe{GetExitCodeProcess(self.handle.0,&mut code)}==0{return Err(os_error("process exit code"));}
                Some(code)
            }else{None};
            Ok(ExitObservation{pid:self.pid,creation_100ns:self.created,exited,exit_code})
        }
    }
    pub struct Snapshot{workers:Vec<Worker>}
    impl Snapshot{
        pub fn capture(parent:&Child)->Result<Option<Self>>{
            let parent_created=created(parent.as_raw_handle())?;
            let raw=unsafe{CreateToolhelp32Snapshot(2,0)};
            if raw as isize == -1{return Err(os_error("process snapshot"));}
            let snapshot=OwnedHandle(raw);
            let mut entry:Entry=unsafe{std::mem::zeroed()};
            entry.size=size_of::<Entry>() as u32;
            let mut ok=unsafe{Process32FirstW(snapshot.0,&mut entry)};
            if ok==0{return Err(os_error("process enumeration start"));}
            let mut workers=vec![];
            while ok!=0{
                if entry.parent==parent.id(){
                    let raw=unsafe{OpenProcess(0x0010_0000|0x1000,0,entry.pid)};
                    if raw==ptr::null_mut(){
                        let code=unsafe{GetLastError()};
                        // PID already gone between enumeration and open.
                        if code!=87{return Err(format!("child handle open: Win32 {code}"));}
                    }else{
                        let handle=OwnedHandle(raw);
                        let creation=created(handle.0)?;
                        let w=Worker{pid:entry.pid,created:creation,handle};
                        if creation>=parent_created&&!w.exited(Duration::ZERO)?{workers.push(w);}
                    }
                }
                ok=unsafe{Process32NextW(snapshot.0,&mut entry)};
            }
            if unsafe{GetLastError()}!=18{return Err(os_error("process enumeration end"));}
            workers.sort_by_key(|w|(w.pid,w.created));
            Ok(Some(Self{workers}))
        }
        pub fn pids(&self)->Vec<u32>{self.workers.iter().map(|w|w.pid).collect()}
        pub fn wait(&self,budget:Duration)->Result<Vec<ExitObservation>>{
            let start=Instant::now();
            self.workers.iter().map(|w|w.result(budget.saturating_sub(start.elapsed()))).collect()
        }
    }
}
#[cfg(not(windows))]
mod platform{
    use super::*;
    pub struct Snapshot;
    impl Snapshot{
        pub fn capture(_: &Child)->Result<Option<Self>>{Ok(None)}
        pub fn pids(&self)->Vec<u32>{vec![]}
        pub fn wait(&self,_:Duration)->Result<Vec<ExitObservation>>{Ok(vec![])}
    }
}
pub use platform::Snapshot;
