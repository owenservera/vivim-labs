//! Exact source-pinned command context resolution. Port of the existing
//! Host command-context-grounding.mjs, never a provenance oracle or effect grant.
//! @hq:capability vivim.rust-context-grounding@1
use serde_json::{Value,json,Map};
use std::collections::BTreeSet;

const SCHEMA:&str="vivim.command-grounding/1";
fn record<'a>(v:&'a Value,allowed:&[&str])->Option<&'a Map<String,Value>>{
    let obj=v.as_object()?;
    if obj.keys().all(|k|allowed.contains(&k.as_str())){Some(obj)}else{None}
}
fn id(s:&str,max:usize)->bool{
    let b=s.as_bytes();
    !b.is_empty()&&b.len()<=max&&b[0].is_ascii_lowercase()&&b[1..].iter().all(|x|
        x.is_ascii_lowercase()||x.is_ascii_digit()||b"._:-".contains(x))
}
fn arg(s:&str)->bool{
    let b=s.as_bytes();
    !b.is_empty()&&b.len()<=40&&b[0].is_ascii_lowercase()&&
    b[1..].iter().all(|x|x.is_ascii_lowercase()||x.is_ascii_digit()||*x==b'_')
}
fn sha(s:&str)->bool{s.len()==64&&s.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b))}
fn refused(reason:&str,arg:Option<&str>,candidates:Option<usize>)->Value{
    let mut obj=json!({"schema":SCHEMA,"outcome":"clarification_required",
        "reason":reason,"executionAuthorized":false});
    if let Some(arg)=arg{obj["arg"]=json!(arg)}
    if let Some(n)=candidates{obj["candidates"]=json!(n)}
    obj
}
pub fn resolve(input:&Value)->Value{
    let bundle=&input["bundle"];
    let requests=input["requests"].as_array();
    let (Some(bundle_obj),Some(selectors),Some(requests))=(
        record(bundle,&["schema","revision","digest","entries","selectors"]),
        record(&bundle["selectors"],&["focused","latestApproved"]),
        requests
    )else{return refused("INVALID_CONTEXT_PROJECTION",None,None)};
    let entries=bundle["entries"].as_array();
    let revision=bundle["revision"].as_str();
    let digest=bundle["digest"].as_str();
    if bundle_obj.get("schema")!=Some(&json!("vivim.command-context-projection/1"))||
        revision.is_none_or(|s|!id(s,128))||
        digest.is_none_or(|s|!sha(s))||
        entries.is_none_or(|a|a.len()>256)||requests.is_empty()||requests.len()>16{
        return refused("INVALID_CONTEXT_PROJECTION",None,None)
    }
    let entries=entries.unwrap();
    let mut ids=BTreeSet::new();
    for entry in entries{
        let Some(_)=record(entry,&["id","type","value","revision",
            "sourceDigest","status","authority"]) else{
            return refused("INVALID_CONTEXT_ENTRY",None,None)
        };
        let Some(entry_id)=entry["id"].as_str()else{
            return refused("INVALID_CONTEXT_ENTRY",None,None)
        };
        if !id(entry_id,128)||!entry["type"].as_str().is_some_and(|s|id(s,128))||
            !entry["value"].as_str().is_some_and(|s|!s.is_empty()&&s.encode_utf16().count()<=512)||
            !entry["revision"].as_str().is_some_and(|s|id(s,128))||
            !entry["sourceDigest"].as_str().is_some_and(sha)||
            !matches!(entry["status"].as_str(),Some("current"|"stale"))||
            !matches!(entry["authority"].as_str(),Some("host-source"|"source-adapter"))||
            !ids.insert(entry_id){
            return refused("INVALID_CONTEXT_ENTRY",None,None)
        }
    }
    let mut used=BTreeSet::new();
    let mut values=Map::new();
    let mut pins=Vec::new();
    for request in requests{
        let Some(_)=record(request,&["arg","kind","refId","expectedType"])else{
            return refused("INVALID_REFERENCE_REQUEST",None,None)
        };
        let Some(name)=request["arg"].as_str()else{
            return refused("INVALID_REFERENCE_REQUEST",None,None)
        };
        let kind=request["kind"].as_str();
        let typ=request["expectedType"].as_str();
        if !arg(name)||!used.insert(name)||
            !matches!(kind,Some("focused"|"explicit"|"latestApproved"))||
            typ.is_none_or(|s|!id(s,128))||
            (kind==Some("explicit")&&!request["refId"].as_str().is_some_and(|s|id(s,128)))||
            (kind!=Some("explicit")&&request.get("refId").is_some()){
            return refused("INVALID_REFERENCE_REQUEST",None,None)
        }
        let candidate:Vec<&str>=match kind{
            Some("explicit")=>request["refId"].as_str().into_iter().collect(),
            Some("focused")=>selectors.get("focused").and_then(Value::as_str).into_iter().collect(),
            Some("latestApproved")=>selectors.get("latestApproved")
                .and_then(Value::as_array).map(|a|a.iter().filter_map(Value::as_str).collect())
                .unwrap_or_default(),
            _=>Vec::new()
        };
        let raw_count=if kind==Some("latestApproved"){
            selectors.get("latestApproved").and_then(Value::as_array).map(Vec::len).unwrap_or(0)
        }else{candidate.len()};
        if candidate.len()!=1||raw_count!=1{
            return refused("AMBIGUOUS_REFERENCE",Some(name),Some(raw_count))
        }
        let found=entries.iter().find(|e|e["id"].as_str()==Some(candidate[0]));
        let Some(e)=found else{return refused("UNKNOWN_REFERENCE",Some(name),None)};
        if e["type"].as_str()!=typ{return refused("WRONG_REFERENCE_TYPE",Some(name),None)}
        if e["status"]!="current"{return refused("STALE_REFERENCE",Some(name),None)}
        values.insert(name.to_owned(),e["value"].clone());
        pins.push(json!({"arg":name,"entityId":e["id"],"revision":e["revision"],
            "sourceDigest":e["sourceDigest"]}));
    }
    pins.sort_by(|a,b|a["arg"].as_str().cmp(&b["arg"].as_str()));
    json!({"schema":SCHEMA,"outcome":"resolved","args":values,
        "pins":{"bundleRevision":revision,"bundleDigest":digest,"references":pins},
        "executionAuthorized":false})
}
