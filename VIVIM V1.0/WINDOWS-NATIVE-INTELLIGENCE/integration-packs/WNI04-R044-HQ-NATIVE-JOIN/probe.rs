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
   // Native handle identity/size is authoritative for this read. Shell property
   // support is an optional, explicitly reported observation, not a prerequisite.
   let before=file_metadata::observe(&file).map_err(|e|format!("{e:?}"))?;
   if before.size>256*1024{return Err("SOURCE_LIMIT".into())}
   let property=match native_property::observe(&path,&file){
    Ok(p) if p.metadata==before=>json!({"status":"ok","size":p.size,"itemType":p.item_type}),
    Ok(_)=>return Err("SOURCE_CHANGED".into()),
    Err(native_property::Error::Com{stage,code})=>json!({"status":"unavailable","stage":stage,"hresult":code}),
    Err(e)=>return Err(format!("{e:?}")),
   };
   let mut bytes=Vec::new();(&mut file).take(256*1024+1).read_to_end(&mut bytes).map_err(|_|"READ_FAILED")?;
   let after=file_metadata::observe(&file).map_err(|e|format!("{e:?}"))?;
   let reopened=File::open(&path).map_err(|_|"SOURCE_CHANGED")?;
   if before!=after||file_metadata::observe(&reopened).map_err(|e|format!("{e:?}"))?!=before||before.size!=bytes.len() as u64{return Err("SOURCE_CHANGED".into())}
   let text=String::from_utf8(bytes).map_err(|_|"UTF8_REQUIRED")?;
   let digest=source_digest::digest_utf8(&json!({"text":text}));
   if digest["outcome"]!="computed"{return Err("DIGEST_FAILED".into())}
   let id=before.identity.file_id.iter().map(|b|format!("{b:02x}")).collect::<String>();
   Ok(json!({"schema":"vivim.labs.native-source-observation/1","relative":"src/campaign.rs","sourceDigest":digest["digest"],"size":before.size,"shellProperty":property,"fileId":id,"volume":format!("{:016x}",before.identity.volume),"authority":"none"}))
  }
  _=>Err("FIXTURE_MODE_REQUIRED".into())
 }
}
fn main(){match run(){Ok(v)=>println!("{}",v),Err(e)=>{eprintln!("{e}");std::process::exit(2)}}}
