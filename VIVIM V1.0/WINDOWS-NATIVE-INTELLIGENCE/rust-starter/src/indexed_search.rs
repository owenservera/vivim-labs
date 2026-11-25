//! Validate bounded output of the optional Windows index proof adapter.
//! No filesystem access or execution. Existing Host must independently
//! bind actual handle identity and authorize every result before projection.
use serde_json::Value;
use std::collections::BTreeSet;
#[derive(Debug,Clone,PartialEq,Eq)]pub struct Hit{pub relative:String,pub size:u64,pub modified_utc:String}
#[derive(Debug,Clone,PartialEq,Eq)]pub enum Index{Available{hits:Vec<Hit>,at_limit:bool},Unavailable{stage:String,hresult:i32}}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Error{Malformed,Limit}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Next{MergeExistingHq,UseExistingHq}
fn exact(v:&Value,k:&[&str])->bool{v.as_object().is_some_and(|o|o.len()==k.len()&&k.iter().all(|x|o.contains_key(*x)))}
fn component(s:&str)->bool{
 if s.is_empty()||s.len()>255||s=="."||s==".."||s.ends_with(['.',' '])||s.chars().any(|c|c.is_control()||"\\/:*?\"<>|%#".contains(c)){return false;}
 let lower=s.to_ascii_lowercase();
 let base=lower.split('.').next().unwrap_or("");
 if matches!(base,"con"|"prn"|"aux"|"nul")||(base.len()==4&&(base.starts_with("com")||base.starts_with("lpt"))&&matches!(base.as_bytes()[3],b'1'..=b'9')){return false;}
 if matches!(lower.as_str(),".git"|".ssh"|".aws"|".azure"|".gnupg"|"login data"|"local state"|"user data")||[".env","credentials","cookies","secrets"].iter().any(|p|lower==*p||lower.strip_prefix(*p).is_some_and(|r|r.starts_with('.'))){return false;}
 true
}
pub fn parse(bytes:&[u8],expected_name:&str,limit:u16)->Result<Index,Error>{
 if bytes.len()>256*1024||!(1..=200).contains(&limit){return Err(Error::Limit);}
 if !component(expected_name){return Err(Error::Malformed);}
 let v:Value=serde_json::from_slice(bytes).map_err(|_|Error::Malformed)?;
 if !exact(&v,&["schema","status","coverage","complete","rows","observed","discarded","atLimit","failureStage","hresult"])||v["schema"]!="vivim.windows.indexed-search/1"||v["coverage"]!="index-only"||v["complete"]!=false{return Err(Error::Malformed);}
 let rows=v["rows"].as_array().ok_or(Error::Malformed)?;
 let observed=v["observed"].as_u64().ok_or(Error::Malformed)?;
 let discarded=v["discarded"].as_u64().ok_or(Error::Malformed)?;
 let at_limit=v["atLimit"].as_bool().ok_or(Error::Malformed)?;
 if observed>u64::from(limit)||discarded>observed||rows.len()>usize::from(limit){return Err(Error::Malformed);}
 if v["status"]=="unavailable"{
  if !rows.is_empty(){return Err(Error::Malformed);}
  let stage=v["failureStage"].as_str().filter(|s|matches!(*s,"create-connection"|"configure-connection"|"open-provider"|"execute-query"|"read-records")).ok_or(Error::Malformed)?;
  let hresult=i32::try_from(v["hresult"].as_i64().ok_or(Error::Malformed)?).map_err(|_|Error::Malformed)?;
  return Ok(Index::Unavailable{stage:stage.into(),hresult});
 }
 if v["status"]!="ok"||!v["failureStage"].is_null()||!v["hresult"].is_null()||observed-discarded!=rows.len() as u64||at_limit!=(observed==u64::from(limit)){return Err(Error::Malformed);}
 let mut seen=BTreeSet::new();let mut hits=Vec::new();
 for row in rows{
  if !exact(row,&["relative","size","modifiedUtc"]){return Err(Error::Malformed);}
  let relative=row["relative"].as_str().filter(|s|s.len()<=1024&&s.split('\\').all(component)&&s.rsplit('\\').next().is_some_and(|n|n.eq_ignore_ascii_case(expected_name))).ok_or(Error::Malformed)?;
  if !seen.insert(relative.to_lowercase()){return Err(Error::Malformed);}
  let size=row["size"].as_u64().ok_or(Error::Malformed)?;
  let time=row["modifiedUtc"].as_str().filter(|s|(20..=40).contains(&s.len())&&s.ends_with('Z')&&s.bytes().all(|c|c.is_ascii_digit()||b"-T:.Z".contains(&c))).ok_or(Error::Malformed)?;
  hits.push(Hit{relative:relative.into(),size,modified_utc:time.into()});
 }
 Ok(Index::Available{hits,at_limit})
}
/// The index is never a complete baseline. Use existing HQ query even on
/// a successful empty response. This does not invoke or authorize HQ.
pub fn next(index:&Index)->Next{match index{Index::Available{..}=>Next::MergeExistingHq,Index::Unavailable{..}=>Next::UseExistingHq}}
#[cfg(test)]mod tests{
 use super::*;use serde_json::json;
 fn result()->Value{json!({"schema":"vivim.windows.indexed-search/1","status":"ok","coverage":"index-only","complete":false,"rows":[{"relative":"owned\\fixture.txt","size":13,"modifiedUtc":"2026-11-25T20:00:00.0000000Z"}],"observed":1,"discarded":0,"atLimit":false,"failureStage":null,"hresult":null})}
 fn p(v:Value)->Result<Index,Error>{parse(&serde_json::to_vec(&v).unwrap(),"fixture.txt",2)}
 #[test]fn valid_projection(){let v=p(result()).unwrap();assert!(matches!(&v,Index::Available{hits,..} if hits.len()==1&&hits[0].size==13));assert_eq!(next(&v),Next::MergeExistingHq);}
 #[test]fn never_false_empty_complete(){let mut r=result();r["rows"]=json!([]);r["observed"]=json!(0);let v=p(r.clone()).unwrap();assert_eq!(next(&v),Next::MergeExistingHq);r["complete"]=json!(true);assert_eq!(p(r),Err(Error::Malformed));}
 #[test]fn unavailable_is_distinct(){let mut r=result();r["status"]=json!("unavailable");r["rows"]=json!([]);r["failureStage"]=json!("execute-query");r["hresult"]=json!(-2147221164);let v=p(r).unwrap();assert_eq!(next(&v),Next::UseExistingHq);}
 #[test]fn refuses_untrusted_paths(){for s in ["..\\fixture.txt","C:\\fixture.txt",".ssh\\fixture.txt","other.txt","owned\\fixture.txt:stream","owned\\\\fixture.txt"]{let mut r=result();r["rows"][0]["relative"]=json!(s);assert_eq!(p(r),Err(Error::Malformed));}}
 #[test]fn refuses_bad_counts_and_schema(){let mut r=result();r["observed"]=json!(0);assert_eq!(p(r),Err(Error::Malformed));let mut r=result();r["unexpected"]=json!("not allowed");assert_eq!(p(r),Err(Error::Malformed));let mut r=result();r["rows"][0]["size"]=json!(-1);assert_eq!(p(r),Err(Error::Malformed));}
}
