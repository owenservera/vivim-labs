//! @hq:context Native Windows Property System readback for a Host-approved open file.
//! @hq:invariant Fixed property allowlist, no content/thumbnail loading or arbitrary property queries.
//! @hq:invariant Path reopens must match the original handle identity and observed metadata.
//! This is a readback adapter, not race-proof path confinement; incumbent Host must bind scope.
use crate::file_metadata::{self,Metadata};
use std::{ffi::c_void,fs::File,mem::zeroed,os::windows::ffi::OsStrExt,path::Path,ptr::{null,null_mut}};
use windows_sys::{core::GUID,Win32::{Foundation::PROPERTYKEY,System::{Com::{CoInitializeEx,CoUninitialize,CoTaskMemFree,COINIT_APARTMENTTHREADED,StructuredStorage::{PROPVARIANT,PropVariantClear,PropVariantToUInt64,PropVariantToStringAlloc}},Variant::{VT_UI8,VT_LPWSTR,VT_BSTR}},UI::Shell::PropertiesSystem::{SHGetPropertyStoreFromParsingName,PSGetPropertyKeyFromName,GPS_FASTPROPERTIESONLY}}};
#[derive(Debug,PartialEq,Eq)]pub enum Error{Invalid,Changed,Metadata(file_metadata::Error),Io,Com{stage:&'static str,code:i32}}
impl From<file_metadata::Error> for Error{fn from(e:file_metadata::Error)->Self{Self::Metadata(e)}}
#[derive(Debug,PartialEq,Eq)]pub struct Properties{pub metadata:Metadata,pub size:u64,pub item_type:Option<String>}
const IID:GUID=GUID::from_u128(0x886d8eeb_8cf2_4446_8d02_cdbA1dbdcf99);
fn hr(stage:&'static str,v:i32)->Result<(),Error>{if v<0{Err(Error::Com{stage,code:v})}else{Ok(())}}
struct Apartment;
impl Apartment{fn enter()->Result<Self,Error>{hr("CoInitializeEx",unsafe{CoInitializeEx(null(),COINIT_APARTMENTTHREADED as u32)})?;Ok(Self)}}
impl Drop for Apartment{fn drop(&mut self){unsafe{CoUninitialize()}}}
#[repr(C)]struct StoreVtable{
 query_interface:usize,add_ref:usize,
 release:unsafe extern "system" fn(*mut c_void)->u32,
 get_count:usize,get_at:usize,
 get_value:unsafe extern "system" fn(*mut c_void,*const PROPERTYKEY,*mut PROPVARIANT)->i32,
}
struct Store(*mut c_void);
impl Store{
 fn vtable(&self)->&StoreVtable{unsafe{&**self.0.cast::<*const StoreVtable>()}}
 fn open(path:&Path)->Result<Self,Error>{
  let mut name:Vec<u16>=path.as_os_str().encode_wide().collect();
  // Shell parsing names use DOS syntax, not the Win32 verbatim drive prefix.
  // Do not translate UNC or device namespaces.
  let verbatim:[u16;4]=[92,92,63,92];
  if name.starts_with(&verbatim){
   if name.len()<7||!((65..=90).contains(&name[4])||(97..=122).contains(&name[4]))||name[5]!=58||name[6]!=92{return Err(Error::Invalid)}
   name.drain(..4);
  }
  if name.is_empty()||name.len()>1024||name.contains(&0){return Err(Error::Invalid)}
  name.push(0);let mut ptr=null_mut();
  hr("SHGetPropertyStoreFromParsingName",unsafe{SHGetPropertyStoreFromParsingName(name.as_ptr(),null_mut(),GPS_FASTPROPERTIESONLY,&IID,&mut ptr)})?;
  if ptr.is_null(){return Err(Error::Invalid)}Ok(Self(ptr))
 }
 fn get(&self,name:&str)->Result<Variant,Error>{
  let key_name:Vec<u16>=name.encode_utf16().chain(Some(0)).collect();let mut key:PROPERTYKEY=unsafe{zeroed()};
  hr("PSGetPropertyKeyFromName",unsafe{PSGetPropertyKeyFromName(key_name.as_ptr(),&mut key)})?;
  let mut value=Variant(unsafe{zeroed()});
  hr("IPropertyStore.GetValue",unsafe{(self.vtable().get_value)(self.0,&key,&mut value.0)})?;Ok(value)
 }
}
impl Drop for Store{fn drop(&mut self){unsafe{(self.vtable().release)(self.0);}}}
struct Variant(PROPVARIANT);
impl Variant{fn kind(&self)->u16{unsafe{self.0.Anonymous.Anonymous.vt}}}
impl Drop for Variant{fn drop(&mut self){unsafe{PropVariantClear(&mut self.0);}}}
struct Text(*mut u16);
impl Drop for Text{fn drop(&mut self){unsafe{CoTaskMemFree(self.0.cast());}}}
fn item_type(store:&Store)->Option<String>{
 let value=store.get("System.ItemType").ok()?;
 if !matches!(value.kind(),VT_LPWSTR|VT_BSTR){return None}
 let mut ptr=null_mut();hr("PropVariantToStringAlloc",unsafe{PropVariantToStringAlloc(&value.0,&mut ptr)}).ok()?;
 if ptr.is_null(){return None}let text=Text(ptr);let mut chars=Vec::new();
 for i in 0..=17{let c=unsafe{*text.0.add(i)};if c==0{let s=String::from_utf16(&chars).ok()?;return (s.len()>=2&&s.starts_with('.')&&s[1..].bytes().all(|b|b.is_ascii_alphanumeric()||b"_-".contains(&b))).then_some(s)}chars.push(c);}
 None
}
fn recheck(path:&Path,expected:Metadata)->Result<(),Error>{
 let f=File::open(path).map_err(|_|Error::Io)?;
 if file_metadata::observe(&f)?!=expected{return Err(Error::Changed)}Ok(())
}
/// Only call with an already Host-approved local file path and its live file handle.
/// Reopen and compare before/after COM; the Host remains responsible for scope and reparse policy.
pub fn observe(approved_path:&Path,file:&File)->Result<Properties,Error>{
 if !approved_path.is_absolute(){return Err(Error::Invalid)}
 let metadata=file_metadata::observe(file)?;
 recheck(approved_path,metadata)?;
 let _com=Apartment::enter()?;let store=Store::open(approved_path)?;
 let value=store.get("System.Size")?;
 if value.kind()!=VT_UI8{return Err(Error::Invalid)}
 let mut size=0;hr("PropVariantToUInt64",unsafe{PropVariantToUInt64(&value.0,&mut size)})?;
 let item_type=item_type(&store);
 if size!=metadata.size||file_metadata::observe(file)?!=metadata{return Err(Error::Changed)}
 recheck(approved_path,metadata)?;
 Ok(Properties{metadata,size,item_type})
}
#[cfg(test)]mod tests{
 use super::*;use std::fs;
 #[test]fn actual_native_property_store_and_handle_identity(){
  let dir=std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("owned-property-fixture");
  fs::create_dir(&dir).unwrap();
  struct Cleanup(std::path::PathBuf);impl Drop for Cleanup{fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
  let _cleanup=Cleanup(dir.clone());let a=dir.join("native.txt");let b=dir.join("other.txt");
  fs::write(&a,b"native SDK source fixture").unwrap();fs::write(&b,b"native SDK source fixture").unwrap();
  let file=File::open(&a).unwrap();let before=observe(&a,&file).unwrap();
  assert_eq!(before.size,25);assert_eq!(before.item_type.as_deref(),Some(".txt"));
  assert_eq!(observe(&b,&file),Err(Error::Changed));
  fs::write(&a,b"updated SDK native source fixture").unwrap();
  let after=observe(&a,&file).unwrap();assert_eq!(after.metadata.identity,before.metadata.identity);assert_ne!(after.size,before.size);
  println!("NATIVE_PROPERTY_SYSTEM_OK before={} after={} item_type={} identity_preserved=true foreign_handle_rejected=true",before.size,after.size,after.item_type.unwrap());
 }
}
