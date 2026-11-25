//! Bounded loss-aware cursor journal for approved observation streams.
//! Does not persist or grant; Host must commit cursor and graph facts atomically.
use std::collections::VecDeque;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct Cursor{pub generation:u64,pub sequence:u64}
#[derive(Debug,Clone,PartialEq,Eq)]pub struct Item<T>{pub cursor:Cursor,pub value:T}
#[derive(Debug,Clone,PartialEq,Eq)]pub enum Page<T>{Items{items:Vec<Item<T>>,next:Cursor,more:bool},Rebaseline{current:Cursor}}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum JournalError{InvalidLimit,Exhausted,StaleBaseline,Gap}
pub struct Journal<T>{generation:u64,sequence:u64,capacity:usize,entries:VecDeque<Item<T>>,gap:bool}
impl<T:Clone> Journal<T>{
 pub fn new(generation:u64,capacity:usize)->Result<Self,JournalError>{
  if generation==0||!(1..=16384).contains(&capacity){return Err(JournalError::InvalidLimit);}
  Ok(Self{generation,sequence:0,capacity,entries:VecDeque::with_capacity(capacity),gap:false})
 }
 pub fn current(&self)->Cursor{Cursor{generation:self.generation,sequence:self.sequence}}
 pub fn mark_gap(&mut self)->Result<Cursor,JournalError>{
  self.gap=true;self.entries.clear();
  self.sequence=self.sequence.checked_add(1).ok_or(JournalError::Exhausted)?;
  Ok(self.current())
 }
 pub fn push(&mut self,value:T)->Result<Cursor,JournalError>{
  if self.gap{self.sequence=self.sequence.checked_add(1).ok_or(JournalError::Exhausted)?;return Err(JournalError::Gap);}
  let next=self.sequence.checked_add(1).ok_or(JournalError::Exhausted)?;
  self.sequence=next;let cursor=self.current();
  if self.entries.len()==self.capacity{self.entries.pop_front();}
  self.entries.push_back(Item{cursor,value});Ok(cursor)
 }
 pub fn read(&self,after:Cursor,limit:usize)->Result<Page<T>,JournalError>{
  if !(1..=200).contains(&limit){return Err(JournalError::InvalidLimit);}
  let current=self.current();let oldest=self.entries.front().map(|x|x.cursor.sequence).unwrap_or(self.sequence.saturating_add(1));
  if self.gap||after.generation!=self.generation||after.sequence>self.sequence||after.sequence.saturating_add(1)<oldest{return Ok(Page::Rebaseline{current});}
  let items:Vec<_>=self.entries.iter().filter(|x|x.cursor.sequence>after.sequence).take(limit).cloned().collect();
  let next=items.last().map(|x|x.cursor).unwrap_or(after);
  Ok(Page::Items{items,next,more:next.sequence<self.sequence})
 }
 /// Called ONLY after verified complete baseline is committed by existing Host/HQ.
 /// Fence against concurrent changes observed since baseline began.
 pub fn rebaseline(&mut self,expected:Cursor,new_generation:u64)->Result<Cursor,JournalError>{
  if expected!=self.current()||new_generation<=self.generation{return Err(JournalError::StaleBaseline);}
  self.generation=new_generation;self.sequence=0;self.entries.clear();self.gap=false;Ok(self.current())
 }
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn page_no_skip(){let mut j=Journal::new(1,4).unwrap();let start=j.current();for n in 1..=3{j.push(n).unwrap();}match j.read(start,2).unwrap(){Page::Items{items,next,more}=>{assert_eq!(items.iter().map(|x|x.value).collect::<Vec<_>>(),vec![1,2]);assert!(more);match j.read(next,2).unwrap(){Page::Items{items,more,..}=>{assert_eq!(items[0].value,3);assert!(!more);},_=>panic!()}},_=>panic!()}}
 #[test]fn overflow_and_generation(){let mut j=Journal::new(7,2).unwrap();let old=j.current();j.push(1).unwrap();j.push(2).unwrap();j.push(3).unwrap();assert!(matches!(j.read(old,2).unwrap(),Page::Rebaseline{..}));let latest=j.mark_gap().unwrap();assert_eq!(j.push(4),Err(JournalError::Gap));assert!(j.rebaseline(old,8).is_err());assert!(j.rebaseline(latest,8).is_err());let fence=j.current();let next=j.rebaseline(fence,8).unwrap();assert!(matches!(j.read(old,2).unwrap(),Page::Rebaseline{..}));assert!(matches!(j.read(next,2).unwrap(),Page::Items{more:false,..}));}
 #[test]fn bounds(){assert!(Journal::<u8>::new(0,1).is_err());assert!(Journal::<u8>::new(1,16385).is_err());let j=Journal::<u8>::new(1,1).unwrap();assert!(j.read(j.current(),201).is_err());}
}
