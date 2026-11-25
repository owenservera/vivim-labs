//! Decoder and sequencer for the owned ETW numeric provider only.
//! Native consumer must authenticate the provider GUID and owned process
//! identity (PID plus creation time) before calling. No session creation.
use crate::etw::{Event,Kind};
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Error{Invalid,Foreign,Gap,Duplicate}
fn number(s:&str)->Result<u64,Error>{if s.is_empty()||s.len()>20||s.len()>1&&s.starts_with('0')||!s.bytes().all(|b|b.is_ascii_digit()){return Err(Error::Invalid);}s.parse().map_err(|_|Error::Invalid)}
pub fn decode_utf16le(bytes:&[u8])->Result<Event,Error>{
 if bytes.len()<2||bytes.len()>512||bytes.len()%2!=0{return Err(Error::Invalid);}
 let u:Vec<u16>=bytes.chunks_exact(2).map(|b|u16::from_le_bytes([b[0],b[1]])).collect();
 if u.last()!=Some(&0)||u[..u.len()-1].contains(&0){return Err(Error::Invalid);}
 let s=String::from_utf16(&u[..u.len()-1]).map_err(|_|Error::Invalid)?;
 let mut p=s.split(';');
 if p.next()!=Some("vivim.native.observation/1"){return Err(Error::Invalid);}
 let kind=match p.next(){Some("kind=task_started")=>Kind::TaskStarted,Some("kind=task_finished")=>Kind::TaskFinished,Some("kind=queue_gap")=>Kind::QueueGap,Some("kind=probe_failed")=>Kind::ProbeFailed,_=>return Err(Error::Invalid)};
 let mut field=|name:&str|->Result<u64,Error>{number(p.next().and_then(|s|s.strip_prefix(name)).ok_or(Error::Invalid)?)};
 let generation=field("generation=")?;let sequence=field("sequence=")?;
 let duration_ms=u32::try_from(field("duration_ms=")?).map_err(|_|Error::Invalid)?;
 let count=u32::try_from(field("count=")?).map_err(|_|Error::Invalid)?;
 if p.next().is_some()||generation==0||sequence==0{return Err(Error::Invalid);}
 Ok(Event{kind,generation,sequence,duration_ms,count})
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct Source{pub process_id:u32,pub creation_ticks:u64,pub generation:u64}
pub struct Correlator{source:Source,last:u64,lost:bool}
impl Correlator{
 pub fn new(source:Source)->Result<Self,Error>{if source.process_id==0||source.creation_ticks==0||source.generation==0{return Err(Error::Invalid);}Ok(Self{source,last:0,lost:false})}
 pub fn mark_loss(&mut self){self.lost=true;}
 pub fn accept(&mut self,source:Source,event:Event)->Result<Event,Error>{
  if source!=self.source||event.generation!=source.generation{return Err(Error::Foreign);}
  if self.lost{return Err(Error::Gap);}
  if event.sequence<=self.last{return Err(Error::Duplicate);}
  if self.last.checked_add(1)!=Some(event.sequence)||event.kind==Kind::QueueGap{self.lost=true;return Err(Error::Gap);}
  self.last=event.sequence;Ok(event)
 }
}
#[cfg(test)]mod tests{
 use super::*;
 fn event(n:u64)->Event{Event{kind:Kind::TaskFinished,generation:2,sequence:n,duration_ms:15,count:1}}
 fn bytes(s:&str)->Vec<u8>{s.encode_utf16().chain(Some(0)).flat_map(u16::to_le_bytes).collect()}
 fn source()->Source{Source{process_id:1,creation_ticks:123,generation:2}}
 #[test]fn producer_roundtrip(){let e=event(1);assert_eq!(decode_utf16le(&bytes(&e.wire())),Ok(e));}
 #[test]fn strict_payload(){for s in [event(1).wire()+";other=1",event(1).wire().replace("sequence=1","sequence=01"),event(1).wire().replace("count=1","count=4294967296"),event(1).wire().replace("generation=2","generation=0")]{assert_eq!(decode_utf16le(&bytes(&s)),Err(Error::Invalid));}assert_eq!(decode_utf16le(&[0]),Err(Error::Invalid));}
 #[test]fn pid_reuse_gap_and_duplicate(){let mut c=Correlator::new(source()).unwrap();assert_eq!(c.accept(Source{creation_ticks:124,..source()},event(1)),Err(Error::Foreign));assert!(c.accept(source(),event(1)).is_ok());assert_eq!(c.accept(source(),event(1)),Err(Error::Duplicate));assert_eq!(c.accept(source(),event(3)),Err(Error::Gap));assert_eq!(c.accept(source(),event(2)),Err(Error::Gap));}
 #[test]fn loss_is_not_success(){let mut c=Correlator::new(source()).unwrap();c.mark_loss();assert_eq!(c.accept(source(),event(1)),Err(Error::Gap));}
}
