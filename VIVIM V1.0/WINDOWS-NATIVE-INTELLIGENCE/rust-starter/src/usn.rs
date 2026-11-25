//! Bounded decoder for FSCTL_READ_USN_JOURNAL output; NO volume open/IO.
//! Names and raw records never leave this module. Only identities already
//! admitted by the Host's authorized file index may be projected.
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]pub enum FileId{V2(u64),V3([u8;16])}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct Delta{pub file:FileId,pub parent:FileId,pub usn:i64,pub reason:u32,pub attributes:u32}
#[derive(Debug,Clone,PartialEq,Eq)]pub struct Batch{pub next_usn:i64,pub admitted:Vec<Delta>,pub parsed_count:usize}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Error{Malformed,UnsupportedVersion,Limit,Gap}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct Checkpoint{pub journal_id:u64,pub next_usn:i64}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Continuity{Continue,Rebaseline}
pub fn continuity(old:Checkpoint,journal_id:u64,first_valid_usn:i64,end_usn:i64)->Continuity{
 if old.journal_id!=journal_id||old.next_usn<0||first_valid_usn<0||end_usn<first_valid_usn||old.next_usn<first_valid_usn||old.next_usn>end_usn{Continuity::Rebaseline}else{Continuity::Continue}
}
fn get<const N:usize>(b:&[u8],i:usize)->Result<[u8;N],Error>{
 let end=i.checked_add(N).ok_or(Error::Malformed)?;
 b.get(i..end).ok_or(Error::Malformed)?.try_into().map_err(|_|Error::Malformed)
}
fn u16_at(b:&[u8],i:usize)->Result<u16,Error>{Ok(u16::from_le_bytes(get(b,i)?))}
fn u32_at(b:&[u8],i:usize)->Result<u32,Error>{Ok(u32::from_le_bytes(get(b,i)?))}
fn u64_at(b:&[u8],i:usize)->Result<u64,Error>{Ok(u64::from_le_bytes(get(b,i)?))}
fn i64_at(b:&[u8],i:usize)->Result<i64,Error>{Ok(i64::from_le_bytes(get(b,i)?))}
/// Admission predicate MUST resolve known identities against current
/// authorized scope. Unknown files are omitted; a scoped baseline discovers names.
pub fn decode<F:FnMut(&Delta)->bool>(b:&[u8],requested_usn:i64,max_records:usize,mut admit:F)->Result<Batch,Error>{
 if requested_usn<0||b.len()<8{return Err(Error::Malformed);}
 if b.len()>1024*1024||!(1..=8192).contains(&max_records){return Err(Error::Limit);}
 let next_usn=i64_at(b,0)?;
 if next_usn<requested_usn{return Err(Error::Gap);}
 let mut records=Vec::new();let mut offset=8usize;let mut prior=None;let mut count=0;
 while offset<b.len(){
  if count>=max_records{return Err(Error::Limit);}
  let length=u32_at(b,offset)? as usize;
  if length<8||length%8!=0{return Err(Error::Malformed);}
  let end=offset.checked_add(length).ok_or(Error::Malformed)?;
  let row=b.get(offset..end).ok_or(Error::Malformed)?;
  let major=u16_at(row,4)?;let minor=u16_at(row,6)?;
  if minor!=0{return Err(Error::UnsupportedVersion);}
  let (file,parent,usn_pos,reason_pos,attributes_pos,name_len_pos,name_off_pos,minimum)=match major{
   2=>(FileId::V2(u64_at(row,8)?),FileId::V2(u64_at(row,16)?),24,40,52,56,58,60),
   3=>(FileId::V3(get(row,8)?),FileId::V3(get(row,24)?),40,56,68,72,74,76),
   _=>return Err(Error::UnsupportedVersion)
  };
  let usn=i64_at(row,usn_pos)?;
  if usn<requested_usn||usn>=next_usn||prior.is_some_and(|p|usn<=p){return Err(Error::Gap);}
  let name_len=u16_at(row,name_len_pos)? as usize;let name_off=u16_at(row,name_off_pos)? as usize;
  if name_len%2!=0||name_off%2!=0||name_off<minimum||name_off.checked_add(name_len).is_none_or(|n|n>row.len()){return Err(Error::Malformed);}
  // Validate offsets but never decode, log, retain or export the filename.
  let d=Delta{file,parent,usn,reason:u32_at(row,reason_pos)?,attributes:u32_at(row,attributes_pos)?};
  records.push(d);prior=Some(usn);offset=end;count+=1;
 }
 // No admission callback sees partial results from a malformed later record.
 let admitted=records.into_iter().filter(|d|admit(d)).collect();
 Ok(Batch{next_usn,admitted,parsed_count:count})
}
#[cfg(test)]mod tests{
 use super::*;
 fn v2(id:u64,usn:i64)->Vec<u8>{let mut r=vec![0;64];r[..4].copy_from_slice(&64u32.to_le_bytes());r[4..6].copy_from_slice(&2u16.to_le_bytes());r[8..16].copy_from_slice(&id.to_le_bytes());r[16..24].copy_from_slice(&9u64.to_le_bytes());r[24..32].copy_from_slice(&usn.to_le_bytes());r[40..44].copy_from_slice(&1u32.to_le_bytes());r[56..58].copy_from_slice(&2u16.to_le_bytes());r[58..60].copy_from_slice(&60u16.to_le_bytes());r[60..62].copy_from_slice(&65u16.to_le_bytes());r}
 fn v3(id:u8,usn:i64)->Vec<u8>{let mut r=vec![0;80];r[..4].copy_from_slice(&80u32.to_le_bytes());r[4..6].copy_from_slice(&3u16.to_le_bytes());r[8..24].fill(id);r[24..40].fill(9);r[40..48].copy_from_slice(&usn.to_le_bytes());r[56..60].copy_from_slice(&1u32.to_le_bytes());r[72..74].copy_from_slice(&2u16.to_le_bytes());r[74..76].copy_from_slice(&76u16.to_le_bytes());r[76..78].copy_from_slice(&65u16.to_le_bytes());r}
 fn batch(next:i64,records:&[Vec<u8>])->Vec<u8>{let mut b=next.to_le_bytes().to_vec();for r in records{b.extend(r)}b}
 #[test]fn versions_and_scope(){let b=batch(300,&[v2(1,100),v3(2,200)]);let d=decode(&b,100,2,|d|d.file==FileId::V2(1)).unwrap();assert_eq!(d.parsed_count,2);assert_eq!(d.admitted.len(),1);assert_eq!(d.next_usn,300);}
 #[test]fn bad_record_is_atomic(){let mut bad=v2(2,200);bad[58..60].copy_from_slice(&200u16.to_le_bytes());let mut calls=0;assert_eq!(decode(&batch(300,&[v2(1,100),bad]),100,2,|_|{calls+=1;true}),Err(Error::Malformed));assert_eq!(calls,0);}
 #[test]fn stale_version_and_order(){let mut r=v2(1,100);r[4..6].copy_from_slice(&4u16.to_le_bytes());assert_eq!(decode(&batch(300,&[r]),100,1,|_|true),Err(Error::UnsupportedVersion));assert_eq!(decode(&batch(300,&[v2(1,200),v2(2,100)]),100,2,|_|true),Err(Error::Gap));}
 #[test]fn rotation_and_empty(){let c=Checkpoint{journal_id:1,next_usn:10};assert_eq!(continuity(c,2,0,20),Continuity::Rebaseline);assert_eq!(continuity(c,1,11,20),Continuity::Rebaseline);assert_eq!(continuity(c,1,0,20),Continuity::Continue);assert_eq!(decode(&20i64.to_le_bytes(),10,1,|_|false).unwrap().next_usn,20);}
}
