//! Transport accounting for owned worker streams. ACP uses lossless stdout;
//! terminal UI may use an explicitly lossy ring. This is not a process runner.
use std::collections::VecDeque;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Mode{Lossless,DisplayTail}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Error{InvalidLimit,Overflow,Closed,Cursor,Exhausted}
#[derive(Debug,Clone,PartialEq,Eq)]pub struct Page{pub start:u64,pub next:u64,pub end:u64,pub bytes:Vec<u8>,pub gap:bool,pub closed:bool}
pub struct Stream{mode:Mode,capacity:usize,base:u64,end:u64,bytes:VecDeque<u8>,closed:bool,poisoned:bool}
impl Stream{
 pub fn new(mode:Mode,capacity:usize)->Result<Self,Error>{if !(1..=1024*1024).contains(&capacity){return Err(Error::InvalidLimit)}Ok(Self{mode,capacity,base:0,end:0,bytes:VecDeque::with_capacity(capacity),closed:false,poisoned:false})}
 pub fn append(&mut self,input:&[u8])->Result<(),Error>{
  if self.closed||self.poisoned{return Err(Error::Closed);}
  let end=self.end.checked_add(u64::try_from(input.len()).map_err(|_|Error::Exhausted)?).ok_or(Error::Exhausted)?;
  let total=self.bytes.len().checked_add(input.len()).ok_or(Error::Exhausted)?;
  if self.mode==Mode::Lossless&&total>self.capacity{self.poisoned=true;return Err(Error::Overflow);}
  if input.len()>=self.capacity{
   self.bytes.clear();self.bytes.extend(&input[input.len()-self.capacity..]);
  }else{
   let drop=total.saturating_sub(self.capacity);
   self.bytes.drain(..drop);self.bytes.extend(input);
  }
  self.end=end;self.base=end-self.bytes.len() as u64;Ok(())
 }
 pub fn read(&self,after:u64,max:usize)->Result<Page,Error>{
  if self.poisoned{return Err(Error::Overflow);}
  if !(1..=65536).contains(&max){return Err(Error::InvalidLimit);}
  if after>self.end{return Err(Error::Cursor);}
  let start=after.max(self.base);let bytes:Vec<u8>=self.bytes.iter().skip((start-self.base)as usize).take(max).copied().collect();
  Ok(Page{start,next:start+bytes.len()as u64,end:self.end,bytes,gap:after<self.base,closed:self.closed})
 }
 /// Only acknowledge after a consumer has processed the exact byte prefix.
 /// For ACP, the Host must retain the partial frame in the Framer before ACK.
 pub fn acknowledge(&mut self,next:u64)->Result<(),Error>{
  if self.poisoned{return Err(Error::Overflow);}
  if self.mode!=Mode::Lossless||next<self.base||next>self.end{return Err(Error::Cursor);}
  self.bytes.drain(..(next-self.base)as usize);self.base=next;Ok(())
 }
 pub fn close(&mut self){self.closed=true;}
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn lossless_ack(){let mut s=Stream::new(Mode::Lossless,4).unwrap();s.append(b"ab").unwrap();s.append(b"cd").unwrap();let p=s.read(0,2).unwrap();assert_eq!(p.bytes,b"ab");s.acknowledge(p.next).unwrap();s.append(b"ef").unwrap();assert_eq!(s.read(2,4).unwrap().bytes,b"cdef");}
 #[test]fn loss_never_silently_parsed(){let mut s=Stream::new(Mode::Lossless,2).unwrap();s.append(b"ab").unwrap();assert_eq!(s.append(b"c"),Err(Error::Overflow));assert_eq!(s.read(0,2),Err(Error::Overflow));assert!(s.acknowledge(2).is_err());}
 #[test]fn display_gap(){let mut s=Stream::new(Mode::DisplayTail,4).unwrap();s.append(b"abcdef").unwrap();let p=s.read(0,4).unwrap();assert!(p.gap);assert_eq!(p.start,2);assert_eq!(p.bytes,b"cdef");s.close();assert!(s.read(6,4).unwrap().closed);assert!(s.append(b"x").is_err());}
}
