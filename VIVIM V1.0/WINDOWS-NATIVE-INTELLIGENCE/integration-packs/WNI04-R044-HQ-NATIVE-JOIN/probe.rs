//! @hq:context Exact native file identity and Product digest for an owned HQ fixture.
//! @hq:invariant The fixture has a fixed relative source; no arbitrary paths or public tool.
mod file_metadata;
mod native_property;
mod source_digest;
mod indexed_search;
use serde_json::json;
use std::{fs::File,io::{self,Read},path::PathBuf};
fn run()->Result<serde_json::Value,String>{
 match std::env::args().nth(1).as_deref(){
  Some("--index")=>{
   let mut bytes=Vec::new();io::stdin().take(256*1024+1).read_to_end(&mut bytes).map_err(|e|e.to_string())?;
   match indexed_search::parse(&bytes,"campaign.rs",20).map_err(|e|format!("{e:?}"))?{
    indexed_search::Index::Available{hits,at_limit}=>Ok(json!({"status":"ok","coverage":"index-only","complete":false,"atLimit":at_limit,"hits":hits.into_iter().map(|h|json!({"relative":h.relative,"size":h.size,"modifiedUtc":h.modified_utc})).collect::<Vec<_>>()})),
    indexed_search::Index::Unavailable{stage,hresult}=>Ok(json!({"status":"unavailable","coverage":"index-only","complete":false,"stage":stage,"hresult":hresult,"hits":[]})),
   }
  }
  Some("--file")=>{
   let root=std::env::current_dir().map_err(|e|e.to_string())?;
   let path=root.join(PathBuf::from("src/campaign.rs"));let mut file=File::open(&path).map_err(|_|"FILE_UNAVAILABLE")?;
   let before=native_property::observe(&path,&file).map_err(|e|format!("{e:?}"))?;
   if before.size>256*1024{return Err("SOURCE_LIMIT".into())}
   let mut bytes=Vec::new();(&mut file).take(256*1024+1).read_to_end(&mut bytes).map_err(|_|"READ_FAILED")?;
   let after=native_property::observe(&path,&file).map_err(|e|format!("{e:?}"))?;
   if before!=after||before.size!=bytes.len() as u64{return Err("SOURCE_CHANGED".into())}
   let text=String::from_utf8(bytes).map_err(|_|"UTF8_REQUIRED")?;
   let digest=source_digest::digest_utf8(&json!({"text":text}));
   if digest["outcome"]!="computed"{return Err("DIGEST_FAILED".into())}
   let id=before.metadata.identity.file_id.iter().map(|b|format!("{b:02x}")).collect::<String>();
   Ok(json!({"schema":"vivim.labs.native-source-observation/1","relative":"src/campaign.rs","sourceDigest":digest["digest"],"size":before.size,"itemType":before.item_type,"fileId":id,"volume":format!("{:016x}",before.metadata.identity.volume),"authority":"none"}))
  }
  _=>Err("FIXTURE_MODE_REQUIRED".into())
 }
}
fn main(){match run(){Ok(v)=>println!("{}",v),Err(e)=>{eprintln!("{e}");std::process::exit(2)}}}
