//! Selected-window UIA read-only probe. No desktop enumeration, input,
//! text, values, window titles or public locator issuance.
//! Caller binds an already approved native window and owns the helper timeout.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct WindowIdentity{pub handle:usize,pub process_id:u32,pub creation_ticks:u64}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Control{pub control_type:i32,pub enabled:bool}
#[derive(Debug,PartialEq,Eq)]
pub struct Observation{pub root:Control,pub children:Vec<Control>,pub omitted:usize}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Error{Unsupported,Invalid,Foreign,Changed,Unavailable,Limit}
#[cfg(windows)]
mod win{
 use super::*;
 use windows::{Win32::{Foundation::HWND,System::Com::{CoInitializeEx,CoUninitialize,CoCreateInstance,COINIT_MULTITHREADED,CLSCTX_INPROC_SERVER},UI::Accessibility::{CUIAutomation,IUIAutomation,IUIAutomationElement,TreeScope_Children}}};
 use windows_sys::Win32::{Foundation::{CloseHandle,FILETIME},System::Threading::{GetProcessTimes,OpenProcess,PROCESS_QUERY_LIMITED_INFORMATION},UI::WindowsAndMessaging::{GetWindowThreadProcessId,IsWindow}};
 fn unavailable<T>(r:windows::core::Result<T>)->Result<T,Error>{r.map_err(|_|Error::Unavailable)}
 struct Apartment;
 impl Apartment{fn enter()->Result<Self,Error>{unsafe{CoInitializeEx(None,COINIT_MULTITHREADED).ok().map_err(|_|Error::Unavailable)?;}Ok(Self)}}
 impl Drop for Apartment{fn drop(&mut self){unsafe{CoUninitialize();}}}
 pub fn identity(handle:usize)->Result<WindowIdentity,Error>{
  if handle==0{return Err(Error::Invalid);}
  let hwnd=handle as windows_sys::Win32::Foundation::HWND;
  let mut pid=0u32;
  if unsafe{IsWindow(hwnd)}==0||unsafe{GetWindowThreadProcessId(hwnd,&mut pid)}==0||pid==0{return Err(Error::Changed);}
  let process=unsafe{OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION,0,pid)};
  if process.is_null(){return Err(Error::Unavailable);}
  let mut c:FILETIME=unsafe{std::mem::zeroed()};let mut e=c;let mut k=c;let mut u=c;
  let ok=unsafe{GetProcessTimes(process,&mut c,&mut e,&mut k,&mut u)};
  unsafe{CloseHandle(process);}
  let ticks=(u64::from(c.dwHighDateTime)<<32)|u64::from(c.dwLowDateTime);
  let mut after=0;
  if ok==0||ticks==0||unsafe{GetWindowThreadProcessId(hwnd,&mut after)}==0||after!=pid{return Err(Error::Changed);}
  Ok(WindowIdentity{handle,process_id:pid,creation_ticks:ticks})
 }
 fn row(e:&IUIAutomationElement,pid:u32)->Result<Option<Control>,Error>{
  unsafe{
   if unavailable(e.CurrentProcessId())?!=pid as i32{return Err(Error::Foreign);}
   // No name, ValuePattern, text, LegacyIAccessible or screenshot.
   if unavailable(e.CurrentIsPassword())?.as_bool(){return Ok(None);}
   let ty=unavailable(e.CurrentControlType())?.0;
   if ty<=0{return Err(Error::Unavailable);}
   let enabled=unavailable(e.CurrentIsEnabled())?.as_bool();
   Ok(Some(Control{control_type:ty,enabled}))
  }
 }
 /// Direct children only, not complete application coverage. Do not feed this
 /// into the full-tree semantic Registry as a complete observation. UIA
 /// providers may block or allocate during FindAll; existing Host must use a
 /// supervised helper with deadline and only an approved selected window.
 pub fn observe(expected:WindowIdentity,limit:u16)->Result<Observation,Error>{
  if !(1..=128).contains(&limit)||expected.process_id==0||expected.creation_ticks==0{return Err(Error::Invalid);}
  if identity(expected.handle)?!=expected{return Err(Error::Foreign);}
  let _apartment=Apartment::enter()?;
  unsafe{
   let automation:IUIAutomation=unavailable(CoCreateInstance(&CUIAutomation,None,CLSCTX_INPROC_SERVER))?;
   let element=unavailable(automation.ElementFromHandle(HWND(expected.handle as *mut core::ffi::c_void)))?;
   let root=row(&element,expected.process_id)?.ok_or(Error::Invalid)?;
   let condition=unavailable(automation.CreateTrueCondition())?;
   let array=unavailable(element.FindAll(TreeScope_Children,&condition))?;
   let count=unavailable(array.Length())?;
   if count<0||count>i32::from(limit){return Err(Error::Limit);}
   let mut children=Vec::new();let mut omitted=0;
   for i in 0..count{
    let child=unavailable(array.GetElement(i))?;
    match row(&child,expected.process_id)?{Some(r)=>children.push(r),None=>omitted+=1}
   }
   if identity(expected.handle)?!=expected{return Err(Error::Changed);}
   Ok(Observation{root,children,omitted})
  }
 }
 #[cfg(test)]mod tests{
  use super::*;
  use std::{sync::mpsc,time::Duration,thread,ptr::null_mut};
  use windows_sys::{core::w,Win32::UI::WindowsAndMessaging::{CreateWindowExW,DestroyWindow,WS_CHILD,WS_POPUP,PeekMessageW,TranslateMessage,DispatchMessageW,MSG,PM_REMOVE}};
  // Owned hidden window only; no foreground activation or existing apps.
  fn fixture()->(usize,mpsc::Sender<()>,thread::JoinHandle<()>){
   let (ready,rx)=mpsc::sync_channel(1);let (stop,quit)=mpsc::channel();
   let worker=thread::spawn(move||unsafe{
    let root=CreateWindowExW(0,w!("STATIC"),w!(""),WS_POPUP,0,0,200,100,null_mut(),null_mut(),null_mut(),std::ptr::null());
    assert!(!root.is_null());
    let child=CreateWindowExW(0,w!("BUTTON"),w!(""),WS_CHILD,0,0,80,30,root,null_mut(),null_mut(),std::ptr::null());
    assert!(!child.is_null());
    ready.send(root as usize).unwrap();
    let mut msg:MSG=std::mem::zeroed();
    loop{
     while PeekMessageW(&mut msg,null_mut(),0,0,PM_REMOVE)!=0{TranslateMessage(&msg);DispatchMessageW(&msg);}
     match quit.recv_timeout(Duration::from_millis(5)){Ok(())|Err(mpsc::RecvTimeoutError::Disconnected)=>break,Err(mpsc::RecvTimeoutError::Timeout)=>{}}
    }
    DestroyWindow(root);
   });
   (rx.recv_timeout(Duration::from_secs(5)).unwrap(),stop,worker)
  }
  #[test]fn real_selected_window_and_scope(){
   let(h,stop,worker)=fixture();
   let result=(||{
    let id=identity(h).unwrap();
    assert_eq!(id.process_id,std::process::id());assert!(id.creation_ticks>0);
    assert_eq!(observe(WindowIdentity{creation_ticks:id.creation_ticks+1,..id},16),Err(Error::Foreign));
    assert_eq!(observe(id,0),Err(Error::Invalid));
    let r=observe(id,16).unwrap();
    assert!(r.root.control_type>0);assert!(r.children.len()+r.omitted<=16);
    r.children.len()
   })();
   stop.send(()).unwrap();worker.join().unwrap();
   assert!(identity(h).is_err());
   // Hidden native providers may omit the child; report actual count, never
   // claim child semantic capture without evidence.
   println!("OWNED_UIA_CHILDREN={result}");
  }
 }
}
#[cfg(windows)]pub use win::{identity,observe};
#[cfg(not(windows))]pub fn identity(_:usize)->Result<WindowIdentity,Error>{Err(Error::Unsupported)}
#[cfg(not(windows))]pub fn observe(_:WindowIdentity,_:u16)->Result<Observation,Error>{Err(Error::Unsupported)}
