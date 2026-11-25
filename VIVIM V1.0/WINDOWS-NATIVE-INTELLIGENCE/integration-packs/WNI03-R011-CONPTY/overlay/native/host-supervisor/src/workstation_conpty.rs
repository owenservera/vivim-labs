//! Native pseudoconsole resource primitive. No executable launch or authority.
//! Intended for existing Host-owned Windows Job worker. IO must be serviced
//! on separate threads. Never call close on an output reader thread.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct Size{pub columns:u16,pub rows:u16}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Error{InvalidSize,Unsupported,Os(u32),Hresult(i32)}
impl Size{pub fn validate(self)->Result<Self,Error>{if !(1..=500).contains(&self.columns)||!(1..=250).contains(&self.rows){Err(Error::InvalidSize)}else{Ok(self)}}}
#[cfg(windows)]
mod win{
 use super::*;
 use std::{fs::File,os::windows::io::{FromRawHandle,OwnedHandle,AsRawHandle},ptr::{null,null_mut}};
 use windows_sys::Win32::{Foundation::{HANDLE,GetLastError},System::{Console::{HPCON,COORD,CreatePseudoConsole,ResizePseudoConsole,ClosePseudoConsole},Pipes::CreatePipe}};
 fn pair()->Result<(OwnedHandle,OwnedHandle),Error>{
  let(mut r,mut w):(HANDLE,HANDLE)=(null_mut(),null_mut());
  if unsafe{CreatePipe(&mut r,&mut w,null(),0)}==0{return Err(Error::Os(unsafe{GetLastError()}));}
  Ok(unsafe{(OwnedHandle::from_raw_handle(r),OwnedHandle::from_raw_handle(w))})
 }
 fn coord(s:Size)->COORD{COORD{X:s.columns as i16,Y:s.rows as i16}}
 /// Non-cloneable handle. Keep on the control thread. The caller must close
 /// its output reader or arrange active draining before dropping this object.
 pub struct Console{handle:HPCON}
 impl Console{
  /// Parent-facing input writer and output reader are returned separately.
  /// Before attaching a client, install the output drain; before teardown,
  /// stop accepting input and close/drain the reader independently of close.
  pub fn create(size:Size)->Result<(Self,File,File),Error>{
   let s=size.validate()?;
   let(input_read,input_write)=pair()?;let(output_read,output_write)=pair()?;
   let mut h:HPCON=0;
   let hr=unsafe{CreatePseudoConsole(coord(s),input_read.as_raw_handle(),output_write.as_raw_handle(),0,&mut h)};
   if hr<0{return Err(Error::Hresult(hr));}
   // ConPTY retains its own references. No inherited handles are enabled.
   drop(input_read);drop(output_write);
   Ok((Self{handle:h},File::from(input_write),File::from(output_read)))
  }
  /// Pass the HANDLE VALUE, not a pointer to an HPCON storage slot, as
  /// lpValue for PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE in the existing worker.
  pub fn handle(&self)->HPCON{self.handle}
  pub fn resize(&self,size:Size)->Result<(),Error>{
   let s=size.validate()?;
   let hr=unsafe{ResizePseudoConsole(self.handle,coord(s))};
   if hr<0{Err(Error::Hresult(hr))}else{Ok(())}
  }
 }
 impl Drop for Console{fn drop(&mut self){unsafe{ClosePseudoConsole(self.handle);}}}
 #[cfg(test)]mod tests{
  use super::*;
  #[test]fn native_create_resize_close_without_attached_client(){
   let (console,input,output)=Console::create(Size{columns:80,rows:24}).unwrap();
   console.resize(Size{columns:100,rows:40}).unwrap();
   drop(input);drop(output);drop(console);
  }
 }
}
#[cfg(windows)]pub use win::Console;
#[cfg(test)]mod tests{use super::*;#[test]fn dimensions(){assert!(Size{columns:80,rows:24}.validate().is_ok());assert_eq!(Size{columns:0,rows:24}.validate(),Err(Error::InvalidSize));assert!(Size{columns:501,rows:1}.validate().is_err());}}
