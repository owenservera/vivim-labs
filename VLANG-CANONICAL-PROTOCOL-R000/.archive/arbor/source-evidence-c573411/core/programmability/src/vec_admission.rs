//! Inert VEC/0.1 admission and source-identity provenance.
//! @hq:capability vivim.rust-vec-admission@1
//! Trusted adapters remain owned by their products; neither source text
//! nor this output authorizes UI mutation, commands or external effects.
use serde_json::{json,Value};
const SCHEMA:&str="vivim.vec-compile/0.1";
fn known(domain:&str)->Option<(&'static str,&'static str)>{
    Some(match domain{
        "wiki"=>("wiki-projection","hq.wiki-program/1"),
        "command"=>("command-plan","vivim.command-intent/0.1"),
        "ui"=>("ui-revision-proposal","vivim.ui-proposal/1"),
        _=>return None
    })
}
fn fail(domain:Option<&str>,outcome:&str,code:&str)->Value{
    let product=domain.and_then(known).map(|x|x.0);
    json!({"schema":SCHEMA,"domain":domain,"product":product,
       "outcome":outcome,"code":code,"authorization":"none",
       "executed":false,"persisted":false})
}
fn semantic_id(schema:&str)->String{
    let i=schema.rfind('/').unwrap_or(0);
    format!("{}@{}",&schema[..i],&schema[i+1..])
}
fn inert_field(value:&Value,depth:usize)->bool{
    if depth>24{return false}
    match value{
        Value::Null|Value::Bool(_)|Value::Number(_)=>true,
        Value::String(s)=>s.len()<=32768,
        Value::Array(rows)=>rows.len()<=1024&&rows.iter().all(|v|inert_field(v,depth+1)),
        Value::Object(fields)=>fields.len()<=256&&fields.iter().all(|(key,v)|
            key.len()<=128&&!matches!(key.as_str(),"__proto__"|"prototype"|"constructor")&&
            inert_field(v,depth+1)),
    }
}
pub fn admit(input:&Value)->Value{
    let domain=input["domain"].as_str().unwrap_or("");
    let Some((_,product_schema))=known(domain)else{
        return fail(None,"refused","UNKNOWN_DOMAIN")
    };
    let Some(text)=input["text"].as_str()else{
        return fail(Some(domain),"refused","INVALID_INPUT")
    };
    if text.trim().is_empty()||text.encode_utf16().count()>4096||
        text.chars().any(|c|matches!(c as u32,0..=8|11|12|14..=31|127)){
        return fail(Some(domain),"refused","INVALID_INPUT")
    }
    if input["contextValid"]!=true{
        return fail(Some(domain),"refused","INVALID_CONTEXT")
    }
    if input["adapterInstalled"]!=true{
        let mut out=fail(Some(domain),"unsupported","LEGACY_OWNER_NOT_MIGRATED");
        out["legacyOwner"]=json!(product_schema);return out;
    }
    if input["adapterFailed"]==true {
        return fail(Some(domain),"refused","ADAPTER_FAILURE")
    }
    let result=&input["adapterResult"];
    let Some(object)=result.as_object()else{
        return fail(Some(domain),"refused","ADAPTER_INVALID_RESULT")
    };
    let outcome=object.get("outcome").and_then(Value::as_str);
    if !matches!(outcome,Some("candidate"|"clarification_required"|"unsupported"|"refused")){
        return fail(Some(domain),"refused","ADAPTER_INVALID_RESULT")
    }
    let code=match object.get("code"){
        None|Some(Value::Null)=>"NONE",
        Some(Value::String(code)) if !code.is_empty()&&code.len()<=128&&
            !code.chars().any(char::is_control)=>code.as_str(),
        _=>return fail(Some(domain),"refused","ADAPTER_INVALID_RESULT"),
    };
    for key in ["proposal","preview","summary","detail"]{
        if let Some(value)=object.get(key){
            if !inert_field(value,0)||
                serde_json::to_vec(value).map_or(true,|bytes|bytes.len()>256*1024){
                return fail(Some(domain),"refused","ADAPTER_INVALID_RESULT")
            }
        }
    }
    let outcome=outcome.unwrap();
    let mut out=fail(Some(domain),outcome,code);
    let valid_candidate=outcome=="candidate"&&
        object.get("proposal").is_some_and(|p|p.is_object()&&p["schema"]==product_schema);
    if valid_candidate{
        out["sourceSemanticIds"]=json!([
            semantic_id(SCHEMA),semantic_id(product_schema)
        ]);
    }
    for key in ["proposal","preview","summary","detail"]{
        if let Some(value)=object.get(key){out[key]=value.clone()}
    }
    out
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn identities_are_earned_not_adapter_supplied(){
        let input=json!({"domain":"ui","text":"Add card","contextValid":true,"adapterInstalled":true,
          "adapterResult":{"outcome":"candidate","code":"PURE_UI_PREVIEW",
          "authorization":"approved","executed":true,"persisted":true,
          "sourceSemanticIds":["spoofed"],"proposal":{"schema":"vivim.ui-proposal/1"}}});
        let actual=admit(&input);
        assert_eq!(actual["sourceSemanticIds"],json!(["vivim.vec-compile@0.1","vivim.ui-proposal@1"]));
        assert_eq!(actual["authorization"],"none");
        assert_eq!(actual["executed"],false);
        assert_eq!(actual["persisted"],false);
    }
    #[test]fn malformed_and_failed_adapters_never_gain_authority(){
        let mut input=json!({"domain":"wiki","text":"project","contextValid":true,"adapterInstalled":false});
        assert_eq!(admit(&input)["legacyOwner"],"hq.wiki-program/1");
        input["adapterInstalled"]=json!(true);
        input["adapterFailed"]=json!(true);
        assert_eq!(admit(&input)["code"],"ADAPTER_FAILURE");
        input["adapterFailed"]=json!(false);
        input["adapterResult"]=json!({"outcome":"candidate","code":{"grant":true}});
        assert_eq!(admit(&input)["code"],"ADAPTER_INVALID_RESULT");
        input["domain"]=json!("unknown");
        assert_eq!(admit(&input)["code"],"UNKNOWN_DOMAIN");
    }
    #[test]fn bounded_adapter_results_are_not_admitted(){
        let mut input=json!({"domain":"ui","text":"edit","contextValid":true,
          "adapterInstalled":true,"adapterResult":{"outcome":"candidate",
          "proposal":{"schema":"vivim.ui-proposal/1"}}});
        input["adapterResult"]["preview"]=json!({"rows":vec![json!(1);1025]});
        assert_eq!(admit(&input)["code"],"ADAPTER_INVALID_RESULT");
        input["adapterResult"]["preview"]=json!({"__proto__":"unsafe"});
        assert_eq!(admit(&input)["code"],"ADAPTER_INVALID_RESULT");
        input["adapterResult"]["preview"]=json!({"items":[1,2,3]});
        assert_eq!(admit(&input)["outcome"],"candidate");
    }
    #[test]fn nested_visual_preview_remains_inside_real_ui_depth_limit(){
        let mut input=json!({"domain":"ui","text":"create","contextValid":true,
            "adapterInstalled":true,"adapterResult":{"outcome":"candidate",
              "proposal":{"schema":"vivim.ui-proposal/1"}}});
        let mut deep=Value::Null;
        for _ in 0..20{deep=json!({"child":deep});}
        input["adapterResult"]["preview"]=deep;
        assert_eq!(admit(&input)["outcome"],"candidate");
        let mut too_deep=Value::Null;
        for _ in 0..25{too_deep=json!({"child":too_deep});}
        input["adapterResult"]["preview"]=too_deep;
        assert_eq!(admit(&input)["code"],"ADAPTER_INVALID_RESULT");
    }
    #[test]fn input_limit_is_utf16_to_match_browser(){
        let input=json!({"domain":"ui","text":"😀".repeat(2049),"contextValid":true,"adapterInstalled":true});
        assert_eq!(admit(&input)["code"],"INVALID_INPUT");
    }
}
