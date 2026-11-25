//! Decode FILE_NOTIFY_INFORMATION within an already-authorized directory.
//! No OS watcher yet. Treat overflow as loss, never pretend a complete index.
#[derive(Debug,Clone,PartialEq,Eq)] pub enum Kind { Added, Removed, Modified, RenameOld, RenameNew }
#[derive(Debug,Clone,PartialEq,Eq)] pub struct Change { pub kind:Kind, pub relative:String }
#[derive(Debug,Clone,PartialEq,Eq)] pub enum DecodeError { Gap, Malformed, Limit }
fn u32_at(b:&[u8],i:usize)->Result<u32,DecodeError>{
 Ok(u32::from_le_bytes(b.get(i..i+4).ok_or(DecodeError::Malformed)?.try_into().map_err(|_|DecodeError::Malformed)?))
}
fn safe_relative(s:&str)->bool{
 !s.is_empty()&&!s.starts_with(['\\','/'])&&s.split(['\\','/']).all(|p|!p.is_empty()&&p!="."&&p!=".."&&!p.ends_with(['.',' '])&&!p.chars().any(|c|c.is_control()||":*?\"<>|".contains(c)))
}
pub fn decode(b:&[u8],max:usize)->Result<Vec<Change>,DecodeError>{
 if b.is_empty(){return Err(DecodeError::Gap);}
 if b.len()>65536||max==0||max>4096{return Err(DecodeError::Limit);}
 let mut out=Vec::new();let mut off=0usize;
 loop{
  if out.len()>=max{return Err(DecodeError::Limit);}
  let next=u32_at(b,off)? as usize;let action=u32_at(b,off+4)?;let n=u32_at(b,off+8)? as usize;
  if n==0||n%2!=0{return Err(DecodeError::Malformed);}
  let start=off.checked_add(12).ok_or(DecodeError::Malformed)?;
  let end=start.checked_add(n).ok_or(DecodeError::Malformed)?;
  let raw=b.get(start..end).ok_or(DecodeError::Malformed)?;
  let wide:Vec<u16>=raw.chunks_exact(2).map(|c|u16::from_le_bytes([c[0],c[1]])).collect();
  let relative=String::from_utf16(&wide).map_err(|_|DecodeError::Malformed)?;
  if !safe_relative(&relative){return Err(DecodeError::Malformed);}
  let kind=match action {1=>Kind::Added,2=>Kind::Removed,3=>Kind::Modified,4=>Kind::RenameOld,5=>Kind::RenameNew,_=>return Err(DecodeError::Malformed)};
  out.push(Change{kind,relative});
  if next==0 {if b.len()-end>3{return Err(DecodeError::Malformed);}break;}
  if next%4!=0||next<12+n{return Err(DecodeError::Malformed);}
  off=off.checked_add(next).ok_or(DecodeError::Malformed)?;
  if off>=b.len(){return Err(DecodeError::Malformed);}
 }
 Ok(out)
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)] pub struct Cursor{pub generation:u64,pub sequence:u64}
#[derive(Debug,Clone,Copy,PartialEq,Eq)] pub enum Admission{Next,Duplicate,Rebaseline}
pub fn admit(old:Cursor,new:Cursor,gap:bool)->Admission{
 if gap||old.generation!=new.generation{return Admission::Rebaseline;}
 if new.sequence<=old.sequence{return Admission::Duplicate;}
 if old.sequence.checked_add(1)==Some(new.sequence){Admission::Next}else{Admission::Rebaseline}
}
#[cfg(test)] mod tests{
 use super::*;
 fn record(name:&str,action:u32)->Vec<u8>{let raw:Vec<u8>=name.encode_utf16().flat_map(u16::to_le_bytes).collect();let mut b=Vec::new();b.extend(0u32.to_le_bytes());b.extend(action.to_le_bytes());b.extend((raw.len() as u32).to_le_bytes());b.extend(raw);b}
 #[test] fn valid_and_invalid(){assert_eq!(decode(&record("a.txt",1),4).unwrap()[0].kind,Kind::Added);assert_eq!(decode(&[],4),Err(DecodeError::Gap));assert!(decode(&record("..\\secret",1),4).is_err());assert!(decode(&record("a",9),4).is_err());}
 #[test] fn cursor_gap(){let a=Cursor{generation:1,sequence:5};assert_eq!(admit(a,Cursor{generation:1,sequence:6},false),Admission::Next);assert_eq!(admit(a,a,false),Admission::Duplicate);assert_eq!(admit(a,Cursor{generation:1,sequence:8},false),Admission::Rebaseline);}
}
