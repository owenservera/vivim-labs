//! One bounded observation of a locally authorized, non-recursive directory.
//! Caller (existing Host) must bind the canonical path and current grant.
//! No public endpoint, elevation, background persistence or content reads.
use crate::change::{self,Change};
use std::path::Path;
#[derive(Debug,PartialEq,Eq)] pub enum WatchError {Unsupported,InvalidScope,Os(u32),Malformed}
#[derive(Debug,PartialEq,Eq)] pub enum Observation {Changes(Vec<Change>),TimedOut,Gap}
#[cfg(windows)]
mod win {
 use super::*;
 use std::{os::windows::{ffi::OsStrExt,fs::MetadataExt},ptr::{null,null_mut}};
 use windows_sys::Win32::{
  Foundation::{CloseHandle,GetLastError,HANDLE,INVALID_HANDLE_VALUE,ERROR_IO_PENDING,ERROR_NOTIFY_ENUM_DIR,ERROR_OPERATION_ABORTED,WAIT_OBJECT_0,WAIT_TIMEOUT},
  Storage::FileSystem::{CreateFileW,ReadDirectoryChangesW,FILE_LIST_DIRECTORY,FILE_SHARE_READ,FILE_SHARE_WRITE,FILE_SHARE_DELETE,OPEN_EXISTING,FILE_FLAG_BACKUP_SEMANTICS,FILE_FLAG_OVERLAPPED,FILE_FLAG_OPEN_REPARSE_POINT,FILE_ATTRIBUTE_REPARSE_POINT,FILE_NOTIFY_CHANGE_FILE_NAME,FILE_NOTIFY_CHANGE_DIR_NAME,FILE_NOTIFY_CHANGE_LAST_WRITE,FILE_NOTIFY_CHANGE_SIZE,GetDriveTypeW,GetFinalPathNameByHandleW,GetFileInformationByHandle,BY_HANDLE_FILE_INFORMATION,FILE_NAME_NORMALIZED,VOLUME_NAME_DOS},
  System::{IO::{OVERLAPPED,CancelIoEx,GetOverlappedResult},Threading::{CreateEventW,WaitForSingleObject}}
 };
 struct Handle(HANDLE);
 impl Drop for Handle {fn drop(&mut self){unsafe{CloseHandle(self.0);}}}
 fn os()->WatchError{WatchError::Os(unsafe{GetLastError()})}
 fn validate(path:&Path)->Result<Vec<u16>,WatchError>{
  // Reject UNC, device namespaces, drive roots, aliases and reparse ancestors.
  let s=path.to_str().ok_or(WatchError::InvalidScope)?;
  let b=s.as_bytes();
  if b.len()<4||!b[0].is_ascii_alphabetic()||b[1]!=b':'||b[2]!=b'\\'||s[3..].split('\\').any(|p|p.is_empty()||p=="."||p==".."||p.ends_with(['.',' '])||p.chars().any(|c|c.is_control()||":/\"*?<>|".contains(c))){return Err(WatchError::InvalidScope);}
  let root=[b[0].to_ascii_uppercase() as u16,58,92,0];
  if unsafe{GetDriveTypeW(root.as_ptr())}!=3{return Err(WatchError::InvalidScope);}
  for a in path.ancestors().filter(|a|!a.as_os_str().is_empty()){
   let m=std::fs::symlink_metadata(a).map_err(|_|WatchError::InvalidScope)?;
   if !m.is_dir()||m.file_attributes()&FILE_ATTRIBUTE_REPARSE_POINT!=0{return Err(WatchError::InvalidScope);}
  }
  // std canonicalize uses extended path syntax. Require the same named target.
  let canonical=path.canonicalize().map_err(|_|WatchError::InvalidScope)?;
  let c=canonical.to_str().ok_or(WatchError::InvalidScope)?;
  let c=c.strip_prefix(r"\\?\").unwrap_or(c);
  if !c.eq_ignore_ascii_case(s){return Err(WatchError::InvalidScope);}
  Ok(path.as_os_str().encode_wide().chain(Some(0)).collect())
 }
 pub fn observe(path:&Path,wait_ms:u32)->Result<Observation,WatchError>{
  if !(50..=5000).contains(&wait_ms){return Err(WatchError::InvalidScope);}
  let wide=validate(path)?;
  // No subtree traversal. FILE_FLAG_OPEN_REPARSE_POINT protects the final component.
  let raw=unsafe{CreateFileW(wide.as_ptr(),FILE_LIST_DIRECTORY,FILE_SHARE_READ|FILE_SHARE_WRITE|FILE_SHARE_DELETE,null(),OPEN_EXISTING,FILE_FLAG_BACKUP_SEMANTICS|FILE_FLAG_OVERLAPPED|FILE_FLAG_OPEN_REPARSE_POINT,null_mut())};
  if raw==INVALID_HANDLE_VALUE{return Err(os());}let file=Handle(raw);
  // Compare actual opened handle path and attributes before starting I/O.
  // Host must additionally pin scope identity and grant, and reject stale paths.
  let mut info:BY_HANDLE_FILE_INFORMATION=unsafe{std::mem::zeroed()};
  if unsafe{GetFileInformationByHandle(file.0,&mut info)}==0{return Err(os());}
  if info.dwFileAttributes&FILE_ATTRIBUTE_REPARSE_POINT!=0{return Err(WatchError::InvalidScope);}
  let mut actual=vec![0u16;32768];
  let len=unsafe{GetFinalPathNameByHandleW(file.0,actual.as_mut_ptr(),actual.len() as u32,FILE_NAME_NORMALIZED|VOLUME_NAME_DOS)};
  if len==0||len as usize>=actual.len(){return Err(WatchError::InvalidScope);}
  let actual=String::from_utf16(&actual[..len as usize]).map_err(|_|WatchError::InvalidScope)?;
  let expected=path.to_str().ok_or(WatchError::InvalidScope)?;
  if !actual.strip_prefix(r"\\?\").unwrap_or(&actual).eq_ignore_ascii_case(expected){return Err(WatchError::InvalidScope);}
  validate(path)?;
  let raw=unsafe{CreateEventW(null(),1,0,null())};
  if raw.is_null(){return Err(os());}let event=Handle(raw);
  // DWORD-aligned stable buffer and overlapped storage live until completion.
  let mut buf=Box::new([0u32;16384]);
  let mut ov:Box<OVERLAPPED>=Box::new(unsafe{std::mem::zeroed()});ov.hEvent=event.0;
  let flags=FILE_NOTIFY_CHANGE_FILE_NAME|FILE_NOTIFY_CHANGE_DIR_NAME|FILE_NOTIFY_CHANGE_LAST_WRITE|FILE_NOTIFY_CHANGE_SIZE;
  let ok=unsafe{ReadDirectoryChangesW(file.0,buf.as_mut_ptr().cast(),65536,0,flags,null_mut(),&mut *ov,None)};
  if ok==0{
   let code=unsafe{GetLastError()};
   if code==ERROR_NOTIFY_ENUM_DIR{return Ok(Observation::Gap);}
   if code!=ERROR_IO_PENDING{return Err(WatchError::Os(code));}
  }
  let wait=unsafe{WaitForSingleObject(event.0,wait_ms)};
  if wait!=WAIT_OBJECT_0{
   // Cancel is not completion. Never free the buffer or OVERLAPPED while
   // pending. A wedged filesystem driver can delay this cleanup; production
   // should run this primitive in a bounded owned helper process.
   unsafe{CancelIoEx(file.0,&*ov);}
   let mut ignored=0;
   unsafe{GetOverlappedResult(file.0,&*ov,&mut ignored,1);}
   return if wait==WAIT_TIMEOUT{Ok(Observation::TimedOut)}else{Err(WatchError::Malformed)};
  }
  let mut n=0;
  if unsafe{GetOverlappedResult(file.0,&*ov,&mut n,1)}==0{
   let code=unsafe{GetLastError()};
   if code==ERROR_NOTIFY_ENUM_DIR||code==ERROR_OPERATION_ABORTED{return Ok(Observation::Gap);}
   return Err(WatchError::Os(code));
  }
  if n==0{return Ok(Observation::Gap);}
  if n>65536{return Err(WatchError::Malformed);}
  let bytes=unsafe{std::slice::from_raw_parts(buf.as_ptr().cast::<u8>(),n as usize)};
  // This is a candidate event list. No filenames are projected until the
  // Host applies its existing protected-name and grant checks.
  match change::decode(bytes,4096){Ok(v)=>Ok(Observation::Changes(v)),Err(_)=>Ok(Observation::Gap)}
 }
}
#[cfg(windows)]pub use win::observe;
#[cfg(not(windows))]pub fn observe(_: &Path,_:u32)->Result<Observation,WatchError>{Err(WatchError::Unsupported)}
#[cfg(test)]mod tests{
 use super::*;
 #[cfg(not(windows))]#[test]fn unavailable_is_explicit(){assert_eq!(observe(Path::new("/tmp"),100),Err(WatchError::Unsupported));}
 #[cfg(windows)]#[test]fn refuses_drive_root_and_bad_wait(){assert_eq!(observe(Path::new("C:\\"),100),Err(WatchError::InvalidScope));assert_eq!(observe(Path::new("C:\\test"),0),Err(WatchError::InvalidScope));}
 #[cfg(windows)]#[test]fn real_local_fixture_change(){
  use std::{fs,time::{Duration,SystemTime,UNIX_EPOCH},thread};
  let base=std::env::temp_dir().join(format!("vivim-watch-{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
  fs::create_dir(&base).unwrap();
  struct Cleanup(std::path::PathBuf);impl Drop for Cleanup{fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
  let _cleanup=Cleanup(base.clone());let dir=base.clone();
  // Repeat a harmless owned fixture mutation to avoid scheduling-race false negatives.
  let writer=thread::spawn(move||{for i in 0..10{thread::sleep(Duration::from_millis(100));fs::write(dir.join("fixture.txt"),i.to_string()).unwrap();}});
  let result=observe(&base,3000);writer.join().unwrap();
  match result.unwrap(){Observation::Changes(v)=>assert!(v.iter().any(|x|x.relative=="fixture.txt")),_=>panic!("no native fixture event")}
 }
}
