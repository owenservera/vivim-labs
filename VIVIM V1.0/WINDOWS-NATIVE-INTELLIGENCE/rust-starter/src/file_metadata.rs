//! Native identity and metadata of an already approved, opened file.
//! Never opens a path, reads file content, or publishes an identity remotely.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Error{Unsupported,Os(u32),Invalid,Changed}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct Identity{pub volume:u64,pub file_id:[u8;16]}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct Metadata{pub identity:Identity,pub size:u64,pub allocation:u64,pub last_write_100ns:i64,pub change_100ns:i64,pub attributes:u32}
#[cfg(windows)]
mod win{
 use super::*;
 use std::{fs::File,os::windows::io::AsRawHandle};
 use windows_sys::Win32::{Foundation::GetLastError,Storage::FileSystem::{GetFileInformationByHandleEx,FileIdInfo,FileBasicInfo,FileStandardInfo,FILE_ID_INFO,FILE_BASIC_INFO,FILE_STANDARD_INFO,FILE_ATTRIBUTE_REPARSE_POINT,FILE_ATTRIBUTE_DIRECTORY}};
 fn query<T>(file:&File,class:i32)->Result<T,Error>{
  // Private generic, used solely for these three zero-initializable C records.
  let mut data:T=unsafe{std::mem::zeroed()};
  let len=u32::try_from(std::mem::size_of::<T>()).map_err(|_|Error::Invalid)?;
  if unsafe{GetFileInformationByHandleEx(file.as_raw_handle(),class,(&mut data as *mut T).cast(),len)}==0{return Err(Error::Os(unsafe{GetLastError()}));}
  Ok(data)
 }
 fn identity(f:&File)->Result<Identity,Error>{
  let v:FILE_ID_INFO=query(f,FileIdInfo)?;
  Ok(Identity{volume:v.VolumeSerialNumber,file_id:v.FileId.Identifier})
 }
 fn snapshot(f:&File,id:Identity)->Result<Metadata,Error>{
  let b:FILE_BASIC_INFO=query(f,FileBasicInfo)?;
  let s:FILE_STANDARD_INFO=query(f,FileStandardInfo)?;
  if b.FileAttributes&(FILE_ATTRIBUTE_REPARSE_POINT|FILE_ATTRIBUTE_DIRECTORY)!=0||s.Directory||s.DeletePending||s.EndOfFile<0||s.AllocationSize<0{return Err(Error::Invalid);}
  Ok(Metadata{identity:id,size:s.EndOfFile as u64,allocation:s.AllocationSize as u64,last_write_100ns:b.LastWriteTime,change_100ns:b.ChangeTime,attributes:b.FileAttributes})
 }
 pub fn observe(file:&File)->Result<Metadata,Error>{
  let id=identity(file)?;
  let a=snapshot(file,id)?;let b=snapshot(file,identity(file)?)?;
  if a!=b{return Err(Error::Changed);}
  // Stable observed metadata is not proof that content remained unchanged;
  // existing indexer still hashes the actual bytes and checks handle scope.
  Ok(b)
 }
 #[cfg(test)]mod tests{
  use super::*;use std::{fs,time::{SystemTime,UNIX_EPOCH}};
  #[test]fn actual_owned_file_identity_and_size(){
   let p=std::env::temp_dir().join(format!("wni-meta-{}-{}.tmp",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
   struct Cleanup(std::path::PathBuf);impl Drop for Cleanup{fn drop(&mut self){let _=fs::remove_file(&self.0);}}
   let _cleanup=Cleanup(p.clone());fs::write(&p,b"native fixture").unwrap();
   let f=File::open(&p).unwrap();let a=observe(&f).unwrap();
   assert_eq!(a.size,14);assert_eq!(a,observe(&f).unwrap());
   fs::write(&p,b"updated fixture bytes").unwrap();let b=observe(&f).unwrap();
   assert_eq!(a.identity,b.identity);assert_ne!(a.size,b.size);
  }
 }
}
#[cfg(windows)]pub use win::observe;
#[cfg(not(windows))]pub fn observe(_: &std::fs::File)->Result<Metadata,Error>{Err(Error::Unsupported)}
