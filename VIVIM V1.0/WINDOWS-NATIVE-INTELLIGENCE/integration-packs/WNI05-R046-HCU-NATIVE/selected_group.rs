//! @hq:context Complete observation of an explicit Host-approved selected group of HWND-backed controls.
//! @hq:invariant This group is not an application-wide tree. UIA runtime IDs remain local and are scope-fenced.
//! @hq:invariant No credential values, names, titles, text, screenshots, UI effects or new grants.
use crate::{uia::{self,WindowIdentity},semantic_surface::{self as sem,Control,Role,Registry,Delivery,Delta,Scope,Snapshot}};
use std::collections::{BTreeMap,BTreeSet};
use sha2::{Digest,Sha256};
use windows::{core::Interface,Win32::{Foundation::HWND,System::{Com::{CoInitializeEx,CoUninitialize,CoCreateInstance,COINIT_MULTITHREADED,CLSCTX_INPROC_SERVER,SAFEARRAY},Ole::{SafeArrayGetDim,SafeArrayGetLBound,SafeArrayGetUBound,SafeArrayGetElement,SafeArrayDestroy}},UI::Accessibility::{CUIAutomation,IUIAutomation,IUIAutomationElement}}};
use windows_sys::Win32::UI::WindowsAndMessaging::IsChild;
#[derive(Debug,PartialEq,Eq)]pub enum Error{Invalid,Foreign,Unavailable,Changed,Semantic(sem::Error)}
impl From<sem::Error> for Error{fn from(e:sem::Error)->Self{Self::Semantic(e)}}
fn cv<T>(r:windows::core::Result<T>)->Result<T,Error>{r.map_err(|_|Error::Unavailable)}
struct Apartment;
impl Apartment{fn enter()->Result<Self,Error>{unsafe{cv(CoInitializeEx(None,COINIT_MULTITHREADED).ok())?;}Ok(Self)}}
impl Drop for Apartment{fn drop(&mut self){unsafe{CoUninitialize()}}}
struct Array(*mut SAFEARRAY);
impl Drop for Array{fn drop(&mut self){unsafe{let _=SafeArrayDestroy(self.0);}}}
fn runtime_id(e:&IUIAutomationElement)->Result<Vec<i32>,Error>{
 let a=Array(unsafe{cv(e.GetRuntimeId())?});
 if a.0.is_null()||unsafe{SafeArrayGetDim(a.0)}!=1{return Err(Error::Invalid)}
 let lo=unsafe{cv(SafeArrayGetLBound(a.0,1))?};let hi=unsafe{cv(SafeArrayGetUBound(a.0,1))?};
 let len=hi.checked_sub(lo).and_then(|n|n.checked_add(1)).ok_or(Error::Invalid)?;
 if !(1..=64).contains(&len){return Err(Error::Invalid)}
 let mut ids=Vec::with_capacity(len as usize);
 for i in lo..=hi{let mut v=0i32;unsafe{cv(SafeArrayGetElement(a.0,&i,(&mut v as *mut i32).cast()))?};ids.push(v);}
 Ok(ids)
}
fn role(ty:i32)->Role{match ty{50000=>Role::Button,50004=>Role::Edit,50020=>Role::Text,50008=>Role::List,50007=>Role::Item,50009|50011=>Role::Menu,_=>Role::Other}}
fn same_window(expected:WindowIdentity)->Result<(),Error>{if uia::identity(expected.handle).ok()!=Some(expected){Err(Error::Changed)}else{Ok(())}}
fn child(root:WindowIdentity,id:WindowIdentity)->Result<(),Error>{
 if id.process_id!=root.process_id||id.creation_ticks!=root.creation_ticks||id.handle==root.handle||unsafe{IsChild(root.handle as _,id.handle as _)}==0{return Err(Error::Foreign)}
 same_window(id)
}
struct Row{runtime:Vec<i32>,ty:i32,enabled:bool,sensitive:bool}
fn read(a:&IUIAutomation,expected:WindowIdentity)->Result<Row,Error>{
 same_window(expected)?;
 let e=unsafe{cv(a.ElementFromHandle(HWND(expected.handle as _)))?};
 let (pid,ty,enabled,sensitive)=unsafe{(cv(e.CurrentProcessId())?,cv(e.CurrentControlType())?.0,cv(e.CurrentIsEnabled())?.as_bool(),cv(e.CurrentIsPassword())?.as_bool())};
 if pid!=expected.process_id as i32||ty<=0{return Err(Error::Foreign)}
 let runtime=runtime_id(&e)?;same_window(expected)?;
 Ok(Row{runtime,ty,enabled,sensitive})
}
pub struct SelectedGroup{root:WindowIdentity,scope:Scope,selected:Vec<WindowIdentity>,registry:Registry,delivery:Delivery,known:BTreeMap<Vec<i32>,u64>,serial:u64,revision:u64}
impl SelectedGroup{
 /// The caller supplies the complete selected group. No discovery of unrelated windows.
 pub fn new(owner:u64,app:u64,generation:u64,root:WindowIdentity,selected:Vec<WindowIdentity>)->Result<Self,Error>{
  if selected.is_empty()||selected.len()>128{return Err(Error::Invalid)}
  same_window(root)?;let mut seen=BTreeSet::new();
  for id in &selected{child(root,*id)?;if !seen.insert(id.handle){return Err(Error::Invalid)}}
  let scope=Scope{owner,app,window:root.handle as u64,generation};
  Ok(Self{root,scope,selected,registry:Registry::new(scope,128)?,delivery:Delivery::new(scope,128)?,known:BTreeMap::new(),serial:0,revision:0})
 }
 pub fn scope(&self)->Scope{self.scope}
 pub fn capture(&mut self)->Result<Snapshot,Error>{
  self.revision=self.revision.checked_add(1).ok_or(Error::Invalid)?;
  let result=self.capture_rows();
  match result{Ok(rows)=>Ok(self.registry.observe(self.scope,self.revision,true,rows)?),Err(e)=>{let _=self.registry.observe(self.scope,self.revision,false,vec![]);self.known.clear();self.delivery.reset();Err(e)}}
 }
 fn capture_rows(&mut self)->Result<Vec<Control>,Error>{
  same_window(self.root)?;let _com=Apartment::enter()?;
  let a:IUIAutomation=unsafe{cv(CoCreateInstance(&CUIAutomation,None,CLSCTX_INPROC_SERVER))?};
  let mut current=BTreeMap::new();let mut rows=Vec::new();
  for id in &self.selected{
   child(self.root,*id)?;let row=read(&a,*id)?;
   if current.contains_key(&row.runtime){return Err(Error::Invalid)}
   let serial=if let Some(n)=self.known.get(&row.runtime){*n}else{self.serial=self.serial.checked_add(1).ok_or(Error::Invalid)?;self.serial};
   let mut hash=Sha256::new();hash.update(self.root.creation_ticks.to_le_bytes());hash.update(id.handle.to_le_bytes());hash.update(row.ty.to_le_bytes());for n in &row.runtime{hash.update(n.to_le_bytes());}
   current.insert(row.runtime,serial);
   rows.push(Control{identity:serial,fingerprint:hash.finalize().into(),role:role(row.ty),enabled:row.enabled,sensitive:row.sensitive});
  }
  same_window(self.root)?;for id in &self.selected{child(self.root,*id)?}
  self.known=current;Ok(rows)
 }
 pub fn prepare(&mut self,s:Snapshot)->Result<Delta,Error>{if self.registry.snapshot()?!=s{return Err(Error::Semantic(sem::Error::Stale))}Ok(self.delivery.prepare(s)?)}
 pub fn ack(&mut self,revision:u64)->Result<(),Error>{Ok(self.delivery.acknowledge(self.scope,revision)?)}
 pub fn resolve(&self,r:sem::Ref,rev:u64)->Result<&Control,Error>{Ok(self.registry.resolve(r,rev)?)}
}
#[cfg(test)]mod tests{
 use super::*;use crate::interaction_plan::{Wait,Predicate,WaitResult};
 use std::{sync::mpsc,thread,time::Duration,ptr::{null,null_mut}};
 use windows_sys::{core::w,Win32::UI::{WindowsAndMessaging::{CreateWindowExW,DestroyWindow,WS_CHILD,WS_POPUP,WS_VISIBLE,WS_EX_NOACTIVATE,WS_EX_TOOLWINDOW,PeekMessageW,TranslateMessage,DispatchMessageW,MSG,PM_REMOVE},Input::KeyboardAndMouse::EnableWindow}};
 enum Command{Disable,Stop}
 #[test]fn actual_selected_controls_revision_ack_and_stale_refusal(){
  let (ready,rx)=mpsc::sync_channel(1);let(tx,cmd)=mpsc::channel();let(done,ack)=mpsc::channel();
  let worker=thread::spawn(move||unsafe{
   let root=CreateWindowExW(WS_EX_NOACTIVATE|WS_EX_TOOLWINDOW,w!("STATIC"),w!(""),WS_POPUP,0,0,200,100,null_mut(),null_mut(),null_mut(),null());
   assert!(!root.is_null());let a=CreateWindowExW(0,w!("BUTTON"),w!(""),WS_CHILD|WS_VISIBLE,0,0,80,30,root,null_mut(),null_mut(),null());
   let b=CreateWindowExW(0,w!("BUTTON"),w!(""),WS_CHILD|WS_VISIBLE,0,35,80,30,root,null_mut(),null_mut(),null());
   assert!(!a.is_null()&&!b.is_null());ready.send((root as usize,a as usize,b as usize)).unwrap();
   let mut msg:MSG=std::mem::zeroed();loop{while PeekMessageW(&mut msg,null_mut(),0,0,PM_REMOVE)!=0{TranslateMessage(&msg);DispatchMessageW(&msg);}
    match cmd.recv_timeout(Duration::from_millis(5)){Ok(Command::Disable)=>{EnableWindow(a,0);done.send(()).unwrap();},Ok(Command::Stop)|Err(mpsc::RecvTimeoutError::Disconnected)=>break,Err(mpsc::RecvTimeoutError::Timeout)=>{}}
   }DestroyWindow(root);
  });
  let (root,a,b)=rx.recv_timeout(Duration::from_secs(5)).unwrap();let root=uia::identity(root).unwrap();let a=uia::identity(a).unwrap();let b=uia::identity(b).unwrap();
  let mut group=SelectedGroup::new(1,1,1,root,vec![a,b]).unwrap();
  let first=group.capture().unwrap();assert_eq!(first.entries.len(),2);assert!(first.entries.iter().all(|e|e.control.role==Role::Button&&e.control.enabled));
  let mut forged=first.clone();forged.entries[0].control.enabled=false;assert!(group.prepare(forged).is_err());
  let reference=first.entries[0].reference;assert!(matches!(group.prepare(first.clone()).unwrap(),Delta::Full(_)));
  assert!(group.prepare(first.clone()).is_err());group.ack(first.revision).unwrap();
  tx.send(Command::Disable).unwrap();ack.recv_timeout(Duration::from_secs(3)).unwrap();
  let second=group.capture().unwrap();assert_eq!(second.entries.len(),2);assert!(group.prepare(first.clone()).is_err());
  assert_eq!(group.resolve(reference,first.revision),Err(Error::Semantic(sem::Error::Stale)));
  let disabled=second.entries.iter().find(|e|!e.control.enabled).unwrap();
  assert!(first.entries.iter().any(|e|e.reference==disabled.reference));
  assert_eq!(group.resolve(disabled.reference,second.revision),Err(Error::Semantic(sem::Error::Disabled)));
  let wait=Wait::new(group.scope(),second.revision,0,100,Predicate::Disabled(disabled.reference)).unwrap();
  assert_eq!(wait.evaluate(Some(&second),1),WaitResult::Matched);
  match group.prepare(second.clone()).unwrap(){Delta::Changes{removed,upsert,..}=>{assert!(removed.is_empty());assert_eq!(upsert.len(),1);},x=>panic!("{x:?}")}
  group.ack(second.revision).unwrap();tx.send(Command::Stop).unwrap();worker.join().unwrap();
  assert!(group.capture().is_err());assert!(group.resolve(reference,second.revision+1).is_err());
  println!("HCU_SELECTED_NATIVE_GROUP_OK controls=2 delta=1 ack=true stale_refused=true destroyed_invalidated=true");
 }
}
