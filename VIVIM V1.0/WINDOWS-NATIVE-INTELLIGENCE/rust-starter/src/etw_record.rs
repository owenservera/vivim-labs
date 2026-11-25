//! Native ETW EVENT_RECORD boundary for our owned provider only.
//! No trace controller or external process collection.
#[cfg(windows)]
mod win{
 use crate::{etw::Event,etw_decode::{decode_utf16le,Correlator,Error,Source}};
 use windows_sys::{core::GUID,Win32::System::Diagnostics::Etw::{EVENT_RECORD,EVENT_HEADER_FLAG_STRING_ONLY}};
 const PROVIDER:GUID=GUID::from_u128(0x69aa8a70_95c5_4a31_b188_9c8917890fd2);
 fn same(a:&GUID,b:&GUID)->bool{a.data1==b.data1&&a.data2==b.data2&&a.data3==b.data3&&a.data4==b.data4}
 /// # Safety
 /// Only call during a native ETW callback with valid record/UserData for
 /// UserDataLength bytes. The source is a Host-verified owned worker identity,
 /// not an identity reconstructed from untrusted PID or event payload.
 pub unsafe fn ingest(record:&EVENT_RECORD,source:Source,correlator:&mut Correlator)->Result<Event,Error>{
  let h=&record.EventHeader;
  if !same(&h.ProviderId,&PROVIDER)||h.ProcessId!=source.process_id{return Err(Error::Foreign);}
  if h.Flags&EVENT_HEADER_FLAG_STRING_ONLY as u16==0||record.UserData.is_null()||record.UserDataLength<2||record.UserDataLength>512{return Err(Error::Invalid);}
  let bytes=unsafe{std::slice::from_raw_parts(record.UserData.cast::<u8>(),record.UserDataLength as usize)};
  let event=decode_utf16le(bytes)?;
  correlator.accept(source,event)
 }
 #[cfg(test)]mod tests{
  use super::*;use crate::etw::Kind;
  #[test]fn exact_record_envelope(){
   let source=Source{process_id:42,creation_ticks:123,generation:7};
   let mut c=Correlator::new(source).unwrap();
   let e=Event{kind:Kind::TaskFinished,generation:7,sequence:1,duration_ms:8,count:1};
   let mut bytes:Vec<u8>=e.wire().encode_utf16().chain(Some(0)).flat_map(u16::to_le_bytes).collect();
   let mut r:EVENT_RECORD=unsafe{std::mem::zeroed()};r.EventHeader.ProviderId=PROVIDER;r.EventHeader.ProcessId=42;r.EventHeader.Flags=EVENT_HEADER_FLAG_STRING_ONLY as u16;r.UserData=bytes.as_mut_ptr().cast();r.UserDataLength=bytes.len() as u16;
   r.EventHeader.ProcessId=43;assert_eq!(unsafe{ingest(&r,source,&mut c)},Err(Error::Foreign));
   r.EventHeader.ProcessId=42;r.EventHeader.Flags=0;assert_eq!(unsafe{ingest(&r,source,&mut c)},Err(Error::Invalid));
   r.EventHeader.Flags=EVENT_HEADER_FLAG_STRING_ONLY as u16;
   assert_eq!(unsafe{ingest(&r,source,&mut c)},Ok(e));
   assert_eq!(unsafe{ingest(&r,source,&mut c)},Err(Error::Duplicate));
  }
 }
}
#[cfg(windows)]pub use win::ingest;
