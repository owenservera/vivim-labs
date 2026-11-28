//! OpenCode v2.4.2 capture worker for the existing executable bridge.
//! No receipt is synthesized from requested identity or CLI exit. A trusted
//! Host must independently qualify identity, usage and settlement first.
use persistent_intelligence_runtime::{digest,kernel::routing_adapter::Envelope};
use serde::{Deserialize,Serialize};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{fs::{self,File,OpenOptions},io::{Read,Write},path::{Path,PathBuf},process::{Command,Stdio},time::{Duration,Instant,SystemTime,UNIX_EPOCH},thread};
type R<T> = Result<T,String>;
fn err(e:impl std::fmt::Display)->String{e.to_string()}
fn check(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn now()->R<u64>{Ok(SystemTime::now().duration_since(UNIX_EPOCH).map_err(err)?.as_millis().try_into().map_err(err)?)}
fn read(p:&Path,max:u64)->R<Vec<u8>>{
    let m=fs::symlink_metadata(p).map_err(err)?;
    check(m.is_file()&&m.len()<=max,"invalid or oversized file")?;
    let mut data=vec![];File::open(p).map_err(err)?.take(max+1).read_to_end(&mut data).map_err(err)?;
    check(data.len() as u64<=max,"read limit exceeded")?;Ok(data)
}
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn write_new(p:&Path,b:&[u8])->R<()>{let mut f=OpenOptions::new().create_new(true).write(true).open(p).map_err(err)?;f.write_all(b).map_err(err)?;f.sync_all().map_err(err)}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config{
    executable:PathBuf,executable_sha256:String,bridge_manifest:String,
    adapter:String,provider:String,account:String,model:String,thinking:String,
    free_entitlement_until_ms:u64,free_entitlement_ref:String,host_containment_ref:String,
    live_enabled:bool,max_runtime_ms:u64,max_capture_bytes:u64,
    inherit_environment:Vec<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request{schema:u32,bridge_manifest:String,handoff:Envelope,input:Value,expires_ms:u64}
fn free_model(m:&str)->bool{
    m.len()<=200&&m.bytes().all(|b|b.is_ascii_alphanumeric()||b"/-_.:".contains(&b))
        &&((m.starts_with("openrouter/")&&m.ends_with(":free")&&m.matches('/').count()>=2)
        ||(m.starts_with("opencode/")&&m.ends_with("-free")&&m.matches('/').count()==1))
}
fn validate(c:&Config,r:&Request,t:u64)->R<()>{
    check(c.live_enabled,"live capture disabled")?;
    check(r.schema==1&&r.bridge_manifest==c.bridge_manifest&&c.bridge_manifest.len()==64,"bridge mismatch")?;
    let e=&r.handoff;
    check(e.adapter==c.adapter&&e.provider==c.provider&&e.account==c.account&&e.model==c.model&&e.thinking==c.thinking,"profile mismatch")?;
    check(free_model(&c.model)&&c.thinking=="default","unsupported model or thinking control")?;
    check(!c.free_entitlement_ref.is_empty()&&!c.host_containment_ref.is_empty()&&c.free_entitlement_until_ms>=r.expires_ms,"missing current Host authorization evidence")?;
    check(r.expires_ms>t&&r.expires_ms<=e.deadline_ms,"expired or mismatched deadline")?;
    check(digest(&r.input)==e.input_digest,"input digest mismatch")?;
    check((100..=600_000).contains(&c.max_runtime_ms)&&(4096..=16_777_216).contains(&c.max_capture_bytes),"invalid capture budgets")?;
    check(c.executable.is_absolute()&&c.executable_sha256.len()==64,"unpinned executable")?;
    check(c.inherit_environment.len()<=32&&c.inherit_environment.iter().all(|k|!k.is_empty()&&k.len()<=128&&k.bytes().all(|b|b.is_ascii_alphanumeric()||b==b'_')&&!k.starts_with("OPENCODE_")),"invalid environment allowlist")
}
#[derive(Debug,Serialize,PartialEq)]
struct Capture{events:usize,text:Option<Value>,step_finishes:usize,reported_cost:Option<f64>,provider_error:bool,malformed:usize,unknown_events:usize}
fn normalize(bytes:&[u8])->R<Capture>{
    let mut text=String::new();let mut events=0;let mut finishes=0;let mut cost=0.0;let mut all_cost=true;let mut provider_error=false;let mut malformed=0;let mut unknown=0;
    for line in std::str::from_utf8(bytes).map_err(err)?.lines().filter(|l|!l.trim().is_empty()){
        events+=1;
        let e:Value=match serde_json::from_str(line){Ok(v)=>v,Err(_)=>{malformed+=1;continue;}};
        match e["type"].as_str(){
            Some("text")=>if let Some(s)=e["part"]["text"].as_str(){text.push_str(s);check(text.len()<=1_048_576,"candidate output limit")?;}else{malformed+=1;},
            Some("step_finish")=>{finishes+=1;if let Some(v)=e["part"]["cost"].as_f64().filter(|v|v.is_finite()&&*v>=0.0){cost+=v;}else{all_cost=false;}},
            Some("error")=>provider_error=true,
            Some("step_start")=>{},
            _=>unknown+=1,
        }
    }
    let output=if malformed==0&&!provider_error&&finishes>0{serde_json::from_str::<Value>(text.trim()).ok()}else{None};
    Ok(Capture{events,text:output,step_finishes:finishes,reported_cost:if all_cost&&finishes>0&&cost.is_finite(){Some(cost)}else{None},provider_error,malformed,unknown_events:unknown})
}
fn capture(config:&Path,request:&Path,receipt:&Path)->R<()>{
    let c:Config=serde_json::from_slice(&read(config,128*1024)?).map_err(err)?;
    let r:Request=serde_json::from_slice(&read(request,2*1024*1024)?).map_err(err)?;
    validate(&c,&r,now()?)?;
    let dir=request.parent().ok_or("missing job directory")?.canonicalize().map_err(err)?;
    check(request.canonicalize().map_err(err)?==dir.join("request.json")&&receipt==dir.join("receipt.json")&&!receipt.exists(),"unexpected bridge paths")?;
    check(c.executable.canonicalize().map_err(err)?==c.executable,"noncanonical executable")?;
    check(hash(&read(&c.executable,512*1024*1024)?)==c.executable_sha256,"executable changed")?;
    // This permanent intent is an extra local replay fence. The kernel's
    // committed mailbox take remains the actual authority.
    write_new(&dir.join("opencode-intent.json"),serde_json::to_vec(&json!({"handoff":r.handoff.id,"request_sha256":hash(&read(request,2*1024*1024)?)})).map_err(err)?.as_slice())?;
    let input=serde_json::to_vec(&r.input).map_err(err)?;
    write_new(&dir.join("input.json"),&input)?;
    let policy=json!({"model":c.model,"default_agent":"vivim-inference","update":"disable","share":"disabled","automations":false,"warming":false,"compaction":{"auto":false,"background":false},
        "permissions":[{"action":"*","resource":"*","effect":"deny"}],
        "policies":[{"action":"provider.use","resource":"*","effect":"deny"},{"action":"provider.use","resource":c.model.split('/').next().unwrap_or(""),"effect":"allow"},{"action":"tool.use","resource":"*","effect":"deny"},{"action":"mcp.use","resource":"*","effect":"deny"}],
        "agents":{"vivim-inference":{"mode":"primary","model":c.model,"steps":1,"system":"Read the attached authorized task data. Return only the requested JSON value. Do not use tools, credentials, external requests or delegation."}}});
    let stdout=dir.join("opencode.stdout.jsonl");let stderr=dir.join("opencode.stderr.txt");
    let out=OpenOptions::new().create_new(true).write(true).open(&stdout).map_err(err)?;
    let er=OpenOptions::new().create_new(true).write(true).open(&stderr).map_err(err)?;
    let t=now()?;let budget=c.max_runtime_ms.min(r.expires_ms.saturating_sub(t));
    check(budget>0,"expired before spawn")?;
    let mut command=Command::new(&c.executable);
    command.current_dir(&dir).env_clear().stdin(Stdio::null()).stdout(Stdio::from(out)).stderr(Stdio::from(er));
    for key in &c.inherit_environment{if let Some(v)=std::env::var_os(key){command.env(key,v);}}
    command.env("OPENCODE_CONFIG_CONTENT",policy.to_string()).args(["run","--standalone","--model",&c.model,"--agent","vivim-inference","--title","VIVIM isolated inference","--archived","--format","json","--file","input.json","Complete the authorized task in the attached JSON. Return its required output as one JSON value."]);
    // Trusted Host must enforce process-tree containment, provider/account
    // restriction, free entitlement and hard resource ceilings outside this
    // worker. A config evidence reference is not itself that enforcement.
    let start=Instant::now();
    let mut child=command.spawn().map_err(err)?;
    let (status,stop)=loop{
        if let Some(s)=child.try_wait().map_err(err)?{break(s,"exited");}
        let too_big= [&stdout,&stderr].iter().any(|p|fs::metadata(p).map(|m|m.len()>c.max_capture_bytes).unwrap_or(true));
        if too_big||start.elapsed()>=Duration::from_millis(budget){
            child.kill().map_err(err)?;break(child.wait().map_err(err)?,if too_big{"capture_limit"}else{"deadline"});
        }
        thread::sleep(Duration::from_millis(25));
    };
    let raw=read(&stdout,c.max_capture_bytes);let errors=read(&stderr,c.max_capture_bytes);
    let normalized=raw.as_ref().ok().and_then(|b|normalize(b).ok());
    let report=json!({"schema":"vivim.opencode.capture/1","handoff":r.handoff.id,"requested_model":c.model,"actual_identity":"unverified","settlement":"unknown",
        "exit_code":status.code(),"stop":stop,"elapsed_ms":start.elapsed().as_millis(),"stdout_sha256":raw.as_ref().ok().map(|b|hash(b)),"stderr_sha256":errors.as_ref().ok().map(|b|hash(b)),
        "capture":normalized,"receipt_written":false,"qualification":"candidate_only"});
    write_new(&dir.join("opencode-observation.json"),&serde_json::to_vec_pretty(&report).map_err(err)?)?;
    // Do not emit receipt.json: a CLI event/cost is not independently observed
    // provider/account/model/thinking identity or complete usage settlement.
    Err("capture recorded; reconcile exact provider identity and usage through Host; never resend".into())
}
fn main(){
    let a:Vec<_>=std::env::args_os().skip(1).collect();
    let result=if a.len()==3{capture(Path::new(&a[0]),Path::new(&a[1]),Path::new(&a[2]))}else{Err("usage: opencode_adapter <trusted-config.json> <request.json> <receipt.json>".into())};
    if let Err(e)=result{eprintln!("{e}");std::process::exit(1);}
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn free_name_guard(){assert!(free_model("openrouter/nvidia/example:free"));assert!(free_model("opencode/example-free"));assert!(!free_model("openrouter/nvidia/example"));assert!(!free_model("nvidia/example-free"));assert!(!free_model("opencode/--free;cmd"));}
    #[test]fn candidate_only(){let c=normalize(br#"{"type":"text","part":{"text":"{\"ok\":true}"}}
{"type":"step_finish","part":{"cost":0}}"#).unwrap();assert_eq!(c.text,Some(json!({"ok":true})));assert_eq!(c.reported_cost,Some(0.0));}
    #[test]fn invalid_or_incomplete(){assert!(normalize(br#"{"type":"text","part":{"text":"1"}}"#).unwrap().text.is_none());assert!(normalize(b"not-json").unwrap().malformed>0);assert!(normalize(br#"{"type":"step_finish","part":{}}
{"type":"error"}"#).unwrap().provider_error);}
    #[test]fn missing_cost_is_not_zero(){assert_eq!(normalize(br#"{"type":"step_finish","part":{}}"#).unwrap().reported_cost,None);}
}
