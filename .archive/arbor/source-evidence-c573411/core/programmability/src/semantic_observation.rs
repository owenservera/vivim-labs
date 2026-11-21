//! Passive, inert aggregation of already-validated Host owner-feedback events.
//! @hq:capability vivim.rust-semantic-observation@1
//! This is not event authentication, consent, rule acceptance, or execution.
use serde_json::{json,Value};
use std::collections::BTreeMap;

const SCHEMA:&str="vivim.semantic-observation/1";
fn invalid()->Value {
    json!({"schema":SCHEMA,"status":"invalid","candidates":[],"conflicts":[],
        "authority":"none","executionAuthorized":false,"effectPerformed":false,
        "activationPermitted":false})
}
fn hex_digest(value:&str)->bool {
    value.len()==64 && value.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b))
}
struct Meaning { action_id:String, args:Value, count:usize }
struct Group { phrase:String, scope:Value, meanings:BTreeMap<String,Meaning> }
pub fn aggregate(input:&Value)->Value {
    if input["schema"]!="vivim.semantic-observation-aggregation-input/1"{
        return invalid()
    }
    let Some(events)=input["validatedEvents"].as_array().filter(|rows|rows.len()<=2000)
        else{return invalid()};
    let Some(minimum)=input["minConfirmations"].as_u64()
        .filter(|n|(2..=20).contains(n))else{return invalid()};
    let Some(version)=input["catalogVersion"].as_u64()
        .filter(|n|*n>0&&*n<=9_007_199_254_740_991)else{return invalid()};
    let Some(considered)=input["considered"].as_u64()
        .filter(|n|*n<=2000)else{return invalid()};
    let Some(skipped)=input["skipped"].as_u64()
        .filter(|n|*n<=considered)else{return invalid()};
    if considered!=events.len() as u64+skipped{return invalid()}

    let mut groups:BTreeMap<String,Group>=BTreeMap::new();
    for event in events {
        let Some(key)=event["groupKey"].as_str().filter(|s|hex_digest(s))
            else{return invalid()};
        let Some(meaning_key)=event["meaningKey"].as_str().filter(|s|hex_digest(s))
            else{return invalid()};
        let Some(phrase)=event["phrase"].as_str()
            .filter(|s|!s.is_empty()&&s.encode_utf16().count()<=200)else{return invalid()};
        let Some(action_id)=event["actionId"].as_str()
            .filter(|s|!s.is_empty()&&s.len()<=128)else{return invalid()};
        if !event["scope"].is_object()||!event["args"].is_object(){return invalid()}
        let group=groups.entry(key.to_owned()).or_insert_with(||Group{
            phrase:phrase.to_owned(),scope:event["scope"].clone(),meanings:BTreeMap::new()
        });
        let meaning=group.meanings.entry(meaning_key.to_owned()).or_insert_with(||Meaning{
            action_id:action_id.to_owned(),args:event["args"].clone(),count:0
        });
        // A digest collision or contradictory key cannot fabricate agreement.
        if meaning.action_id!=action_id||meaning.args!=event["args"]{return invalid()}
        meaning.count+=1;
    }
    let mut candidates=Vec::new();
    let mut conflicts=Vec::new();
    for (key,group) in &groups {
        if group.meanings.len()>1{
            conflicts.push(json!({"phraseDigest":key,"distinctMeanings":group.meanings.len()}));
            continue;
        }
        let Some(meaning)=group.meanings.values().next()else{continue};
        if meaning.count<(minimum as usize){continue}
        let rule=json!({
            "schema":"vivim.personal-semantic-rule/1",
            "id":format!("rule:observed-{}",&key[..24]),"revision":1,
            "phrase":group.phrase,"actionId":meaning.action_id,"args":meaning.args,
            "scope":group.scope,"state":"proposed","source":"system-observed",
            "catalogVersion":version
        });
        candidates.push(json!({"rule":rule,"evidence":{
            "confirmationCount":meaning.count,"kind":"explicit-confirmation-only"
        },"executionAuthorized":false}));
    }
    json!({"schema":SCHEMA,"status":"inert-proposals","considered":considered,
        "skipped":skipped,"groups":groups.len(),"candidates":candidates,
        "conflicts":conflicts,"activationPermitted":false,
        "authority":"none","executionAuthorized":false,"effectPerformed":false})
}
