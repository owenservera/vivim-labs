//! @hq:context Bounded empirical compiler workload feedback keyed by exact source/toolchain/operation identity.
//! @hq:invariant Unknown measurements are not zero; estimates never drop below configured floors.
use std::collections::{BTreeMap,VecDeque};
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord)]pub struct Key{pub source_digest:String,pub toolchain_digest:String,pub operation:String}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct Measure{pub elapsed_ms:u64,pub peak_memory:Option<u64>,pub artifact_bytes:Option<u64>}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct Estimate{pub memory:u64,pub disk:u64,pub elapsed_ms:u64,pub samples:usize}
#[derive(Debug,PartialEq,Eq)]pub enum Error{Invalid,Duplicate,Full,Overflow}
struct Series{rows:VecDeque<Measure>}
pub struct Profiles{series:BTreeMap<Key,Series>,seen:BTreeMap<u64,Key>,max_keys:usize,max_receipts:usize,window:usize}
fn digest(s:&str)->bool{s.len()==64&&s.bytes().all(|c|c.is_ascii_hexdigit()&&!c.is_ascii_uppercase())}
fn key_ok(k:&Key)->bool{digest(&k.source_digest)&&digest(&k.toolchain_digest)&&!k.operation.is_empty()&&k.operation.len()<=64&&k.operation.bytes().all(|c|c.is_ascii_alphanumeric()||b"._-".contains(&c))}
impl Profiles{
 pub fn new(max_keys:usize,window:usize,max_receipts:usize)->Result<Self,Error>{
  if max_keys==0||max_keys>1024||window==0||window>64||max_receipts==0||max_receipts>65536{return Err(Error::Invalid)}
  Ok(Self{series:BTreeMap::new(),seen:BTreeMap::new(),max_keys,max_receipts,window})
 }
 /// Only a verified owned successful exit is eligible. Receipt identity is caller-scoped and must be unique.
 pub fn record(&mut self,receipt:u64,key:Key,m:Measure)->Result<(),Error>{
  if receipt==0||!key_ok(&key)||m.elapsed_ms==0||m.peak_memory==Some(0){return Err(Error::Invalid)}
  if self.seen.contains_key(&receipt){return Err(Error::Duplicate)}
  if self.seen.len()>=self.max_receipts||(!self.series.contains_key(&key)&&self.series.len()>=self.max_keys){return Err(Error::Full)}
  let s=self.series.entry(key.clone()).or_insert_with(||Series{rows:VecDeque::new()});
  if s.rows.len()==self.window{s.rows.pop_front();}
  s.rows.push_back(m);self.seen.insert(receipt,key);Ok(())
 }
 pub fn estimate(&self,key:&Key,floor:Estimate)->Result<Estimate,Error>{
  if !key_ok(key)||floor.memory==0||floor.elapsed_ms==0{return Err(Error::Invalid)}
  let Some(s)=self.series.get(key)else{return Ok(Estimate{samples:0,..floor})};
  fn headroom(v:u64)->Result<u64,Error>{v.checked_add(v.div_ceil(4)).ok_or(Error::Overflow)}
  let memory=s.rows.iter().filter_map(|m|m.peak_memory).max().map(headroom).transpose()?.unwrap_or(0);
  let disk=s.rows.iter().filter_map(|m|m.artifact_bytes).max().map(headroom).transpose()?.unwrap_or(0);
  let elapsed=headroom(s.rows.iter().map(|m|m.elapsed_ms).max().unwrap())?;
  Ok(Estimate{memory:floor.memory.max(memory),disk:floor.disk.max(disk),elapsed_ms:floor.elapsed_ms.max(elapsed),samples:s.rows.len()})
 }
}
#[cfg(test)]mod tests{
 use super::*;
 fn k(tag:char)->Key{Key{source_digest:tag.to_string().repeat(64),toolchain_digest:"f".repeat(64),operation:"rustc.metadata".into()}}
 fn floor()->Estimate{Estimate{memory:100,disk:20,elapsed_ms:10,samples:0}}
 #[test]fn measured_feedback_is_bounded_and_never_lowers_floors(){
  let mut p=Profiles::new(2,2,4).unwrap();
  p.record(1,k('a'),Measure{elapsed_ms:40,peak_memory:Some(200),artifact_bytes:Some(80)}).unwrap();
  assert_eq!(p.estimate(&k('a'),floor()).unwrap(),Estimate{memory:250,disk:100,elapsed_ms:50,samples:1});
  p.record(2,k('a'),Measure{elapsed_ms:5,peak_memory:None,artifact_bytes:None}).unwrap();
  p.record(3,k('a'),Measure{elapsed_ms:5,peak_memory:None,artifact_bytes:None}).unwrap();
  assert_eq!(p.estimate(&k('a'),floor()).unwrap(),Estimate{samples:2,..floor()});
  assert_eq!(p.estimate(&k('b'),floor()).unwrap(),floor());
 }
 #[test]fn duplicate_receipt_and_capacity_fail_closed(){
  let mut p=Profiles::new(1,1,2).unwrap();let m=Measure{elapsed_ms:1,peak_memory:None,artifact_bytes:Some(0)};
  p.record(1,k('a'),m).unwrap();assert_eq!(p.record(1,k('a'),m),Err(Error::Duplicate));assert_eq!(p.record(2,k('b'),m),Err(Error::Full));
  p.record(2,k('a'),m).unwrap();assert_eq!(p.record(3,k('a'),m),Err(Error::Full));
 }
 #[test]fn overflow_does_not_become_small_estimate(){
  let mut p=Profiles::new(1,1,1).unwrap();p.record(1,k('a'),Measure{elapsed_ms:1,peak_memory:Some(u64::MAX),artifact_bytes:None}).unwrap();
  assert_eq!(p.estimate(&k('a'),floor()),Err(Error::Overflow));
 }
}
