//! Preparation-only projection. No transport, execution, persistence or grants.
use serde_json::{Value,json};
use std::collections::{BTreeMap,BTreeSet};
type R<T> = Result<T,&'static str>;
fn s<'a>(v:&'a Value,k:&str)->R<&'a str>{v[k].as_str().ok_or("FIELD")}
fn hex(s:&str,n:usize)->bool{s.len()==n&&s.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b))}
fn ctl(s:&str)->bool{s.strip_prefix("ctl-").is_some_and(|v|hex(v,16))}
fn token(s:&str)->bool{!s.is_empty()&&s.len()<=128&&s.bytes().all(|b|b.is_ascii_lowercase()||b.is_ascii_digit()||b"._-".contains(&b))}
fn object(v:&Value)->R<&serde_json::Map<String,Value>>{v.as_object().ok_or("OBJECT")}
fn exact(v:&Value,keys:&[&str])->R<()>{let m=object(v)?;if m.len()!=keys.len()||keys.iter().any(|k|!m.contains_key(*k)){return Err("FIELDS")}Ok(())}
/// A bounded, ephemeral index of declarative inputs. Evidence and availability
/// remain separate; the same tool name is never a grant or verified binding.
pub fn introspect(catalog:&Value,source:&Value,client:&Value,candidates:&Value)->R<Value>{
 let rows=catalog["actions"].as_array().ok_or("CATALOG")?;
 let defs=source.as_array().ok_or("SOURCE")?;
 let visible=client.as_array().ok_or("CLIENT")?;
 let proposed=candidates.as_array().ok_or("CANDIDATES")?;
 if rows.len()>4096||defs.len()>512||visible.len()>512||proposed.len()>128{return Err("LIMIT")}
 let mut by_action=BTreeMap::new();
 for a in rows{let id=s(a,"id")?;if !token(id)||by_action.insert(id,a).is_some(){return Err("ACTION_ID")}}
 let mut client_names=BTreeSet::new();
 for v in visible{let n=s(v,"name")?;if !token(n)||!client_names.insert(n){return Err("CLIENT_ID")}}
 let mut names=BTreeSet::new();let mut identities=BTreeSet::new();let mut output=Vec::new();
 for d in defs{
  let n=s(d,"name")?;if !token(n)||!names.insert(n){return Err("TOOL_ID")}
  let action=d["action"].as_str();
  let host=action.map(|id|by_action.get(id).ok_or("UNRESOLVED_HOST_ACTION")).transpose()?;
  if let Some(h)=host{let id=s(h,"id")?;if !identities.insert(id){return Err("DUPLICATE_ACTION_BINDING")}
   let fields=d["fields"].as_array().ok_or("SOURCE_FIELDS")?;
   let got:BTreeSet<_>=fields.iter().map(|x|x.as_str().ok_or("FIELD")).collect::<R<_>>()?;
   if fields.len()!=got.len(){return Err("DUPLICATE_FIELD")}
   let host_fields:BTreeSet<_>=object(&h["args"])?.keys().map(String::as_str).collect();
   // Terminal request_json is a client alias; Host receives base64 request.
   let expected=if id=="workstation.session.terminal_start"{BTreeSet::from(["request_json"])}else{host_fields};
   if got!=expected||d["mutating"].as_bool()!=Some(h["risk"]=="R2"){return Err("HOST_SCHEMA_DRIFT")}
  }
  let client_row=visible.iter().find(|x|x["name"]==n);
  let schema_match=if let Some(c)=client_row{
   let mut expected:BTreeSet<&str>=d["fields"].as_array().ok_or("SOURCE_FIELDS")?.iter().map(|x|x.as_str().ok_or("FIELD")).collect::<R<_>>()?;
   if d["mutating"]==true{expected.insert("request_id");}
   let actual:BTreeSet<&str>=c["fields"].as_array().ok_or("CLIENT_FIELDS")?.iter().map(|x|x.as_str().ok_or("FIELD")).collect::<R<_>>()?;
   Some(expected==actual)
  }else{None};
  output.push(json!({"tool":n,"identity":action.map(|a|json!({"domain":"host-action","key":a})).unwrap_or_else(||json!({"domain":"mcp-tool","key":n})), "hostAction":action,"sourceDefined":true,"clientVisible":client_names.contains(n),"fieldParity":schema_match,"deployed":null,"granted":null,"executionAuthorized":false}));
 }
 for n in client_names.difference(&names){output.push(json!({"tool":n,"sourceDefined":false,"clientVisible":true,"deployed":null,"granted":null,"executionAuthorized":false}));}
 for p in proposed{
  let id=s(p,"id")?;let sem=s(p,"semanticId")?;let n=s(&p["mcp"],"name")?;
  if !token(id)||sem!=format!("{id}@1")||!token(n)||!names.insert(n)||!identities.insert(id)||client_names.contains(n)||by_action.contains_key(id){return Err("CANDIDATE_COLLISION")}
  if p["mcp"]["registered"]!=false||p["app"]["enabled"]!=false||p["authority"]["executionAuthorized"]!=false||p["authority"]["requiresLiveGrant"]!=true||p["host"]["readiness"]!="adapter_pending"||p["host"]["id"]!=id||p["app"]["actionId"]!=id||p["app"]["args"]!=p["host"]["args"]||p["host"]["aiCandidate"]!=false{return Err("CANDIDATE_AUTHORITY")}
  output.push(json!({"tool":n,"identity":{"domain":"capability","key":sem},"hostAction":id,"sourceDefined":true,"clientVisible":false,"candidate":true,"deployed":null,"granted":null,"executionAuthorized":false}));
 }
 output.sort_by(|a,b|a["tool"].as_str().cmp(&b["tool"].as_str()));
 Ok(json!({"schema":"vivim.labs.p06.introspection/1","authority":"projection-only","rows":output}))
}
/// Validate the supported R042 compiler schema subset; unknown forms are refused.
/// This is not a general JSON Schema implementation or authorization check.
pub fn normalize_candidate(row:&Value,input:&Value,mcp:bool)->R<Value>{
 let fields=object(&row["fields"])?;
 if fields.len()>128{return Err("LIMIT")}
 let effect=row["host"]["risk"]=="R2";
 if !effect&&row["host"]["risk"]!="R0"{return Err("RISK")}
 let needs_id=mcp&&effect;
 let mut keys:Vec<_>=fields.keys().map(String::as_str).collect();
 if keys.contains(&"request_id"){return Err("RESERVED_FIELD")}
 if needs_id{keys.push("request_id");}
 exact(input,&keys)?;
 if needs_id&&!hex(s(input,"request_id")?,32){return Err("REQUEST_ID")}
 let mut args=serde_json::Map::new();
 for (key,f) in fields{
  let x=&input[key];let kind=s(f,"kind")?;
  let v=match kind{
   "boolean"=>if mcp{if x.as_bool().ok_or("BOOLEAN")?{"enabled".into()}else{"disabled".into()}}else{let q=x.as_str().ok_or("BOOLEAN")?;if !["enabled","disabled"].contains(&q){return Err("BOOLEAN")}q.into()},
   "drive"=>{let q=x.as_str().ok_or("DRIVE")?;let b=q.as_bytes();if b.len()!=2||b[1]!=b':'||!b[0].is_ascii_alphabetic()||(!mcp&&!b[0].is_ascii_uppercase()){return Err("DRIVE")}q.to_ascii_uppercase()},
   "integer"=>{let n=if mcp{x.as_u64().ok_or("INTEGER")?}else{let q=x.as_str().ok_or("INTEGER")?;let n=q.parse::<u64>().map_err(|_|"INTEGER")?;if n.to_string()!=q{return Err("INTEGER")}n};if n<f["min"].as_u64().ok_or("SCHEMA")?||n>f["max"].as_u64().ok_or("SCHEMA")?{return Err("INTEGER")}n.to_string()},
   "string"=>{let q=x.as_str().ok_or("STRING")?;let n=q.encode_utf16().count() as u64;
    if n<f["min"].as_u64().ok_or("SCHEMA")?||n>f["max"].as_u64().ok_or("SCHEMA")?||q.trim()!=q||q.chars().any(|c|c<='\u{1f}'||c=='\u{7f}'){return Err("STRING")}q.into()},
   _=>return Err("UNSUPPORTED_SCHEMA")
  };args.insert(key.clone(),json!(v));
 }
 let mut out=json!({"actionId":s(row,"id")?,"args":args,"executionAuthorized":false});
 if needs_id{out["transportRequestId"]=input["request_id"].clone();}
 Ok(out)
}
/// Correlation compares already-admitted Host truth to expected exact plan and
/// request claim. It never trusts the reduced MCP receipt as a raw Host receipt.
/// This does not authenticate the input or independently recompute its digest.
pub fn correlate(claim:&Value,plan:&Value,receipt:&Value)->R<Value>{
 let request=s(claim,"requestId")?;let id=s(plan,"planId")?;let digest=s(plan,"planDigest")?;let action=s(plan,"actionId")?;
 if !hex(request,32)||!ctl(id)||!hex(digest,64)||!token(action)||claim["actionId"]!=action||claim["args"]!=plan["args"]||!plan["args"].is_object(){return Err("CLAIM_PLAN_MISMATCH")}
 if plan["outcome"]!="approval_required"||plan["risk"]!="R2"||receipt["schema"]!="vivim.control-receipt/1"||receipt["planId"]!=id||receipt["planDigest"]!=digest||receipt["actionId"]!=action||receipt["args"]!=plan["args"]||receipt["approvedVia"]!="desktop-owner-token"{return Err("RECEIPT_MISMATCH")}
 let status=s(receipt,"status")?;
 if !["applied","failed","uncertain"].contains(&status){return Err("STATUS")}
 let session=if status=="applied"{
  if receipt["executionPerformed"]!=true||!receipt["result"].is_object(){return Err("APPLIED_INVALID")}
  if ["workstation.session.start","workstation.session.terminal_start"].contains(&action){
   if receipt["result"]["sessionId"]!=id{return Err("SESSION_MISMATCH")}Some(id)
  }else{None}
 }else{None};
 Ok(json!({"schema":"vivim.labs.p06.receipt-correlation/1","requestId":request,"planId":id,"planDigest":digest,"actionId":action,"status":status,"sessionId":session,"assurance":"matched-inputs-not-authentication","executionAuthorized":false,"retryAuthorized":false}))
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]fn actual_donor_projection(){
  let p=std::env::var("P06_FIXTURE").expect("source-derived fixture required");
  let f:Value=serde_json::from_slice(&std::fs::read(p).unwrap()).unwrap();
  let r=introspect(&f["catalog"],&f["source"],&f["client"],&f["candidates"]).unwrap();
  let rows=r["rows"].as_array().unwrap();
  assert_eq!(rows.iter().filter(|x|x["candidate"]==true).count(),33);
  assert_eq!(rows.iter().filter(|x|x["clientVisible"]==true).count(),16);
  assert_eq!(rows.iter().filter(|x|x["sourceDefined"]==true&&x["clientVisible"]==false&&x["candidate"]!=true).count(),2);
  assert!(rows.iter().all(|x|x["executionAuthorized"]==false&&x["granted"].is_null()));
  assert!(rows.iter().filter(|x|x["clientVisible"]==true).all(|x|x["fieldParity"]==true));
  for c in f["cases"].as_array().unwrap(){
   assert_eq!(normalize_candidate(&c["row"],&c["mcp"],true).unwrap(),c["expectedMcp"]);
   assert_eq!(normalize_candidate(&c["row"],&c["app"],false).unwrap(),c["expectedApp"]);
   let mut bad=c["mcp"].clone();bad.as_object_mut().unwrap().insert("unexpected".into(),json!(1));
   assert!(normalize_candidate(&c["row"],&bad,true).is_err());
  }
  let mut altered=f["candidates"].clone();altered[0]["mcp"]["registered"]=json!(true);
  assert!(introspect(&f["catalog"],&f["source"],&f["client"],&altered).is_err());
 }
 #[test]fn receipt_correlation_never_retries(){
  let action="workstation.session.terminal_start";let id="ctl-0123456789abcdef";let d="a".repeat(64);let args=json!({"request":"b64:eA=="});
  let c=json!({"requestId":"b".repeat(32),"actionId":action,"args":args});
  let p=json!({"planId":id,"planDigest":d,"actionId":action,"args":args,"outcome":"approval_required","risk":"R2"});
  let mut r=json!({"schema":"vivim.control-receipt/1","planId":id,"planDigest":d,"actionId":action,"args":args,"approvedVia":"desktop-owner-token","status":"applied","executionPerformed":true,"result":{"sessionId":id}});
  assert_eq!(correlate(&c,&p,&r).unwrap()["sessionId"],id);
  for key in ["planDigest","actionId","args","approvedVia","schema"]{let mut wrong=r.clone();wrong[key]=json!("wrong");assert!(correlate(&c,&p,&wrong).is_err());}
  r["result"]["sessionId"]=json!("ctl-ffffffffffffffff");assert!(correlate(&c,&p,&r).is_err());
  r["status"]=json!("uncertain");let v=correlate(&c,&p,&r).unwrap();assert_eq!(v["retryAuthorized"],false);assert!(v["sessionId"].is_null());
 }
}
