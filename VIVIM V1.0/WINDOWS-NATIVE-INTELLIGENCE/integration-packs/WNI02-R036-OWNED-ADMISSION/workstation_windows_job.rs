//! Windows creation-time process-tree containment. No job handles are inherited.
//! Job objects are lifecycle containment, not an OS security sandbox.
use std::{ffi::{OsStr,OsString,c_void},fs::File,io,mem::{size_of,zeroed},os::windows::{ffi::OsStrExt,io::{AsRawHandle,FromRawHandle,OwnedHandle},process::ExitStatusExt},path::Path,process::ExitStatus,ptr::{null,null_mut},thread,time::{Duration,Instant}};
use windows_sys::Win32::{
 Foundation::{HANDLE,SetHandleInformation,HANDLE_FLAG_INHERIT,WAIT_OBJECT_0,WAIT_TIMEOUT},
 Security::SECURITY_ATTRIBUTES,
 System::{
 JobObjects::{CreateJobObjectW,SetInformationJobObject,QueryInformationJobObject,TerminateJobObject,JobObjectExtendedLimitInformation,JobObjectBasicAccountingInformation,JOBOBJECT_EXTENDED_LIMIT_INFORMATION,JOBOBJECT_BASIC_ACCOUNTING_INFORMATION,JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE},
 Pipes::CreatePipe,
 Threading::{CreateProcessW,InitializeProcThreadAttributeList,UpdateProcThreadAttribute,DeleteProcThreadAttributeList,GetExitCodeProcess,WaitForSingleObject,PROCESS_INFORMATION,STARTUPINFOEXW,LPPROC_THREAD_ATTRIBUTE_LIST,STARTF_USESTDHANDLES,EXTENDED_STARTUPINFO_PRESENT,CREATE_UNICODE_ENVIRONMENT,CREATE_NO_WINDOW,PROC_THREAD_ATTRIBUTE_HANDLE_LIST,PROC_THREAD_ATTRIBUTE_JOB_LIST},
 }};
fn invalid()->io::Error{io::Error::new(io::ErrorKind::InvalidInput,"invalid Windows process specification")}
fn ok(v:i32)->io::Result<()>{if v==0{Err(io::Error::last_os_error())}else{Ok(())}}
fn raw(h:&OwnedHandle)->HANDLE{h.as_raw_handle()}
unsafe fn own(h:HANDLE)->OwnedHandle{OwnedHandle::from_raw_handle(h)}
fn wide(s:&OsStr)->io::Result<Vec<u16>>{let v:Vec<_>=s.encode_wide().collect();if v.contains(&0){return Err(invalid());}Ok(v)}
fn terminated(s:&OsStr)->io::Result<Vec<u16>>{let mut v=wide(s)?;v.push(0);Ok(v)}
// Windows CRT quoting: double backslashes before a literal quote or closing quote.
fn quote(s:&OsStr,out:&mut Vec<u16>)->io::Result<()>{
 out.push(34);let mut slashes=0usize;
 for c in wide(s)?{if c==92{slashes+=1;continue;}if c==34{out.extend(std::iter::repeat_n(92,slashes*2+1));}else{out.extend(std::iter::repeat_n(92,slashes));}slashes=0;out.push(c);}
 out.extend(std::iter::repeat_n(92,slashes*2));out.push(34);Ok(())
}
fn env_block(env:&[(OsString,OsString)])->io::Result<Vec<u16>>{
 let mut entries=std::collections::BTreeMap::new();
 for(k,v)in env{
  let name=k.to_str().filter(|s|!s.is_empty()&&s.is_ascii()&&!s.contains('=')&&!s.contains('\0')).ok_or_else(invalid)?;
  let mut entry=wide(k)?;entry.push(61);entry.extend(wide(v)?);entry.push(0);
  if entries.insert(name.to_ascii_uppercase(),entry).is_some(){return Err(invalid());}
 }
 let mut result=Vec::new();for entry in entries.into_values(){result.extend(entry);}
 if result.is_empty(){result.push(0);}result.push(0);Ok(result)
}
// Only the exact three standard handles are inherited by CreateProcessW.
fn pipe(parent_reads:bool)->io::Result<(File,OwnedHandle)>{
 let sa=SECURITY_ATTRIBUTES{nLength:size_of::<SECURITY_ATTRIBUTES>()as u32,lpSecurityDescriptor:null_mut(),bInheritHandle:1};
 let(mut read,mut write)=(null_mut(),null_mut());
 unsafe{ok(CreatePipe(&mut read,&mut write,&sa,0))?;
  let read=own(read);let write=own(write);
  let(parent,child)=if parent_reads{(read,write)}else{(write,read)};
  ok(SetHandleInformation(raw(&parent),HANDLE_FLAG_INHERIT,0))?;
  Ok((File::from(parent),child))
 }
}
struct Attributes{_storage:Vec<usize>,ptr:LPPROC_THREAD_ATTRIBUTE_LIST}
impl Attributes{
 fn new()->io::Result<Self>{
  let mut bytes=0usize;unsafe{InitializeProcThreadAttributeList(null_mut(),2,0,&mut bytes);}
  if bytes==0{return Err(io::Error::last_os_error());}
  let mut storage=vec![0usize;bytes.div_ceil(size_of::<usize>())];
  let ptr=storage.as_mut_ptr().cast();
  unsafe{ok(InitializeProcThreadAttributeList(ptr,2,0,&mut bytes))?;}
  Ok(Self{_storage:storage,ptr})
 }
 fn set<T>(&mut self,id:usize,value:&T)->io::Result<()>{
  unsafe{ok(UpdateProcThreadAttribute(self.ptr,0,id,(value as *const T).cast::<c_void>(),size_of::<T>(),null_mut(),null_mut()))}
 }
}
impl Drop for Attributes{fn drop(&mut self){unsafe{DeleteProcThreadAttributeList(self.ptr);}}}
pub(super) struct Process{job:OwnedHandle,root:OwnedHandle}
impl Process{
 pub(super) fn spawn(exe:&Path,cwd:&Path,args:&[OsString],environment:&[(OsString,OsString)],interactive:bool)
 ->io::Result<(Self,File,File,Option<File>)>{
  let app=terminated(exe.as_os_str())?;let directory=terminated(cwd.as_os_str())?;
  let mut cmd=Vec::new();quote(exe.as_os_str(),&mut cmd)?;for arg in args{cmd.push(32);quote(arg,&mut cmd)?;}
  if cmd.len()>=32767{return Err(invalid());}cmd.push(0);
  let env=env_block(environment)?;
  unsafe{
   let h=CreateJobObjectW(null(),null());if h.is_null(){return Err(io::Error::last_os_error());}
   let job=own(h);
   let mut limits:JOBOBJECT_EXTENDED_LIMIT_INFORMATION=zeroed();
   limits.BasicLimitInformation.LimitFlags=JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
   ok(SetInformationJobObject(raw(&job),JobObjectExtendedLimitInformation,(&limits as *const JOBOBJECT_EXTENDED_LIMIT_INFORMATION).cast(),size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>()as u32))?;
   let (stdin,child_in)=pipe(false)?;let(stdout,child_out)=pipe(true)?;let(stderr,child_err)=pipe(true)?;
   let handles=[raw(&child_in),raw(&child_out),raw(&child_err)];let jobs=[raw(&job)];
   let mut attrs=Attributes::new()?;
   attrs.set(PROC_THREAD_ATTRIBUTE_HANDLE_LIST as usize,&handles)?;
   attrs.set(PROC_THREAD_ATTRIBUTE_JOB_LIST as usize,&jobs)?;
   let mut si:STARTUPINFOEXW=zeroed();si.StartupInfo.cb=size_of::<STARTUPINFOEXW>()as u32;
   si.StartupInfo.dwFlags=STARTF_USESTDHANDLES;si.StartupInfo.hStdInput=handles[0];si.StartupInfo.hStdOutput=handles[1];si.StartupInfo.hStdError=handles[2];si.lpAttributeList=attrs.ptr;
   let mut pi:PROCESS_INFORMATION=zeroed();
   // The job is assigned by the kernel during creation. Incompatible parent
   // jobs fail closed; never fall back to an uncontained child or breakaway.
   ok(CreateProcessW(app.as_ptr(),cmd.as_mut_ptr(),null(),null(),1,
    EXTENDED_STARTUPINFO_PRESENT|CREATE_UNICODE_ENVIRONMENT|CREATE_NO_WINDOW,
    env.as_ptr().cast(),directory.as_ptr(),&si.StartupInfo,&mut pi))?;
   let root=own(pi.hProcess);let _thread=own(pi.hThread);
   let process=Self{job,root};
   drop((child_in,child_out,child_err));
   Ok((process,stdout,stderr,if interactive{Some(stdin)}else{drop(stdin);None}))
  }
 }
 pub(super) fn active(&self)->io::Result<u32>{
  unsafe{let mut info:JOBOBJECT_BASIC_ACCOUNTING_INFORMATION=zeroed();
   ok(QueryInformationJobObject(raw(&self.job),JobObjectBasicAccountingInformation,(&mut info as *mut JOBOBJECT_BASIC_ACCOUNTING_INFORMATION).cast(),size_of::<JOBOBJECT_BASIC_ACCOUNTING_INFORMATION>()as u32,null_mut()))?;
   Ok(info.ActiveProcesses)
  }
 }
 pub(super) fn try_wait(&mut self)->io::Result<Option<ExitStatus>>{
  if self.active()?!=0{return Ok(None);}
  unsafe{match WaitForSingleObject(raw(&self.root),0){
   WAIT_TIMEOUT=>Ok(None),WAIT_OBJECT_0=>{let mut code=0;ok(GetExitCodeProcess(raw(&self.root),&mut code))?;Ok(Some(ExitStatus::from_raw(code)))},
   _=>Err(io::Error::last_os_error())
  }}
 }
 pub(super) fn kill(&mut self)->io::Result<()>{if self.active()?!=0{unsafe{ok(TerminateJobObject(raw(&self.job),1))?;}}Ok(())}
 pub(super) fn wait(&mut self)->io::Result<ExitStatus>{
  let start=Instant::now();loop{if let Some(status)=self.try_wait()?{return Ok(status);}
   if start.elapsed()>=Duration::from_secs(5){return Err(io::Error::new(io::ErrorKind::TimedOut,"job termination not observed"));}
   thread::sleep(Duration::from_millis(10));
  }
 }
}
#[cfg(test)]mod tests{use super::*;
 #[test]fn quotes_preserve_spaces_quotes_and_trailing_backslashes(){
  let mut out=Vec::new();quote(OsStr::new("a b\\"),&mut out).unwrap();assert_eq!(String::from_utf16(&out).unwrap(),"\"a b\\\\\"");
  out.clear();quote(OsStr::new("a\"b"),&mut out).unwrap();assert_eq!(String::from_utf16(&out).unwrap(),"\"a\\\"b\"");
 }
 #[test]fn empty_environment_double_null_and_duplicate_rejection(){assert_eq!(env_block(&[]).unwrap(),vec![0,0]);assert!(env_block(&[("PATH".into(),"x".into()),("Path".into(),"y".into())]).is_err());}
}
