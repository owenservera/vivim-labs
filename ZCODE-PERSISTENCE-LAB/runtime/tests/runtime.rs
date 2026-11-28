use persistent_intelligence_runtime::{Command,Store,State,GoalStatus,TaskStatus,digest};
use serde_json::{json,Value};
use std::{fs,path::PathBuf,sync::atomic::{AtomicU64,Ordering},process::{Command as Process,Stdio},io::Write};
static NEXT:AtomicU64=AtomicU64::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new()->Self { let p=std::env::temp_dir().join(format!("vivim-runtime-{}-{}-{}",std::process::id(),NEXT.fetch_add(1,Ordering::Relaxed),std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));fs::create_dir(&p).unwrap();Self(p) }
    fn store(&self)->Store {Store::open(&self.0).unwrap()}
}
impl Drop for Temp {fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
fn cmd(v:Value)->Command{serde_json::from_value(v).unwrap()}
fn put(s:&Store,v:Value)->Value{s.apply(cmd(v)).unwrap()}
fn goal(s:&Store,id:&str,budget:u64){put(s,json!({"cmd":"goal","id":id,"objective":"controlled workload","claim_budget":budget}));}
fn setup(s:&Store,g:&str){
    goal(s,g,30);
    put(s,json!({"cmd":"class","id":"base","version":1,"operators":["execute","verify","delegate"],"capabilities":[]}));
    put(s,json!({"cmd":"context","id":"ctx","artifacts":[],"data":{}}));
    agent(s,g,"a","base:1"); agent(s,g,"b","base:1");
}
fn agent(s:&Store,g:&str,a:&str,c:&str){
    put(s,json!({"cmd":"agent","id":a,"goal":g,"class":c,"context":"ctx","parent":null}));
    put(s,json!({"cmd":"activate","agent":a}));
}
fn task(s:&Store,g:&str,id:&str,depends:Vec<&str>,input:Value,expected:Value,cap:Option<&str>){
    put(s,json!({"cmd":"task","id":id,"goal":g,"objective":"compute","depends":depends,"input":input,"expected":digest(&expected),"capability":cap,"max_attempts":3}));
}
fn claim(s:&Store,t:&str,a:&str)->u64{put(s,json!({"cmd":"claim","task":t,"agent":a,"ttl":5}))["generation"].as_u64().unwrap()}
fn submit(s:&Store,t:&str,a:&str,g:u64,v:Value)->Value{
    put(s,json!({"cmd":"submit","task":t,"agent":a,"generation":g,"value":v}));
    put(s,json!({"cmd":"verify","task":t}))
}
#[test]
fn restart_fences_old_worker_and_keeps_checkpoint(){
    let tmp=Temp::new();let s=tmp.store();setup(&s,"g");
    task(&s,"g","t",vec![],json!(3),json!(6),None);
    task(&s,"g","u",vec!["t"],json!(4),json!(8),None);
    put(&s,json!({"cmd":"seal","goal":"g"}));
    assert_eq!(s.inspect().unwrap().ready(),vec!["t"]);
    let n=claim(&s,"t","a");
    put(&s,json!({"cmd":"checkpoint","task":"t","agent":"a","generation":n,"data":{"step":1}}));
    drop(s);
    let s=tmp.store();put(&s,json!({"cmd":"tick","now":5}));
    assert_eq!(s.inspect().unwrap().tasks["t"].checkpoint,Some(json!({"step":1})));
    let n2=claim(&s,"t","b");assert!(n2>n);
    assert!(s.apply(cmd(json!({"cmd":"submit","task":"t","agent":"a","generation":n,"value":6}))).is_err());
    assert_eq!(submit(&s,"t","b",n2,json!(6))["accepted"],true);
    assert_eq!(s.inspect().unwrap().ready(),vec!["u"]);
    let n=claim(&s,"u","b");assert_eq!(submit(&s,"u","b",n,json!(9))["accepted"],false);
    let n=claim(&s,"u","a");assert_eq!(submit(&s,"u","a",n,json!(8))["accepted"],true);
    let st=s.inspect().unwrap();assert_eq!(st.goals["g"].status,GoalStatus::Complete);assert_eq!(st.artifacts.len(),2);assert_eq!(st.evidence.len(),3);
    assert!(s.apply(cmd(json!({"cmd":"verify","task":"u"}))).is_err());
}
#[test]
fn promote_then_reuse_and_disable(){
    let tmp=Temp::new();let s=tmp.store();setup(&s,"g");
    for (id,input,out) in [("e1",2,4),("e2",5,10)] {
        task(&s,"g",id,vec![],json!(input),json!(out),None);
        let n=claim(&s,id,"a");assert_eq!(submit(&s,id,"a",n,json!(out))["accepted"],true);
    }
    let program=json!({"op":"multiply","left":{"op":"input"},"right":{"op":"const","value":2}});
    assert!(s.apply(cmd(json!({"cmd":"promote","id":"bad","version":1,"program":{"op":"const","value":4},"examples":["e1","e2"]}))).is_err());
    put(&s,json!({"cmd":"promote","id":"double","version":1,"program":program,"examples":["e1","e2"]}));
    put(&s,json!({"cmd":"class","id":"computed","version":1,"operators":["execute"],"capabilities":["double:1"]}));
    agent(&s,"g","c","computed:1");
    task(&s,"g","holdout",vec!["e1","e2"],json!(13),json!(26),Some("double:1"));
    assert!(s.apply(cmd(json!({"cmd":"claim","task":"holdout","agent":"a","ttl":5}))).is_err());
    let n=claim(&s,"holdout","c");put(&s,json!({"cmd":"execute","task":"holdout","agent":"c","generation":n}));
    assert_eq!(put(&s,json!({"cmd":"verify","task":"holdout"}))["accepted"],true);
    put(&s,json!({"cmd":"disable","capability":"double:1"}));
    assert!(s.apply(cmd(json!({"cmd":"task","id":"x","goal":"g","objective":"disabled","depends":[],"input":3,"expected":digest(&json!(6)),"capability":"double:1","max_attempts":2}))).is_err());
}
#[test]
fn incarnation_mailbox_request_replay_cancel(){
    let tmp=Temp::new();let s=tmp.store();setup(&s,"g");
    let message=json!({"cmd":"send","id":"m","from":"a","to":"b","body":{"notice":"ready"}});
    let receipt=s.apply_request(Some("request-1".into()),cmd(message.clone())).unwrap();
    assert_eq!(s.apply_request(Some("request-1".into()),cmd(message)).unwrap(),receipt);
    assert!(s.apply_request(Some("request-1".into()),cmd(json!({"cmd":"tick","now":1}))).is_err());
    assert!(s.apply(cmd(json!({"cmd":"ack","id":"m","agent":"a"}))).is_err());
    put(&s,json!({"cmd":"ack","id":"m","agent":"b"}));
    task(&s,"g","t",vec![],json!(1),json!(2),None);let n=claim(&s,"t","a");
    put(&s,json!({"cmd":"retire","agent":"a"}));
    assert_eq!(put(&s,json!({"cmd":"activate","agent":"a"}))["incarnation"],2);
    let n2=claim(&s,"t","a");assert!(n2>n);
    put(&s,json!({"cmd":"renew","task":"t","agent":"a","generation":n2,"ttl":10}));
    put(&s,json!({"cmd":"tick","now":5}));
    assert_eq!(s.inspect().unwrap().tasks["t"].status,TaskStatus::Running);
    put(&s,json!({"cmd":"cancel","goal":"g"}));
    assert!(s.apply(cmd(json!({"cmd":"submit","task":"t","agent":"a","generation":n2,"value":2}))).is_err());
    let st=s.inspect().unwrap();assert!(st.messages["m"].acknowledged);assert_eq!(st.goals["g"].status,GoalStatus::Cancelled);
    assert_eq!(st.tasks["t"].status,TaskStatus::Cancelled);
}
#[test]
fn corruption_fails_closed_and_uncommitted_temp_is_ignored(){
    let tmp=Temp::new();let s=tmp.store();goal(&s,"g",3);
    fs::write(tmp.0.join(".tmp-crash"),b"partial").unwrap();
    assert_eq!(s.inspect().unwrap().goals.len(),1);
    let p=tmp.0.join("00000000000000000001.rev.json");
    let mut v:Value=serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
    v["record"]["state"]["goals"]["g"]["objective"]=json!("tampered");
    fs::write(p,serde_json::to_vec(&v).unwrap()).unwrap();
    assert!(s.inspect().is_err());assert!(s.apply(cmd(json!({"cmd":"tick","now":1}))).is_err());
}
#[test]
fn actual_separate_processes_recover(){
    let tmp=Temp::new();let exe=env!("CARGO_BIN_EXE_persistent-intelligence-runtime");
    fn apply(exe:&str,root:&PathBuf,id:&str,v:Value)->Value{
        let mut p=Process::new(exe).arg(root).args(["apply",id]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        p.stdin.take().unwrap().write_all(v.to_string().as_bytes()).unwrap();
        let o=p.wait_with_output().unwrap();assert!(o.status.success(),"{}",String::from_utf8_lossy(&o.stderr));serde_json::from_slice(&o.stdout).unwrap()
    }
    let v=json!({"cmd":"goal","id":"g","objective":"survive processes","claim_budget":2});
    let a=apply(exe,&tmp.0,"req",v.clone());let b=apply(exe,&tmp.0,"req",v);
    assert_eq!(a,b);
    apply(exe,&tmp.0,"clock",json!({"cmd":"tick","now":7}));
    let o=Process::new(exe).arg(&tmp.0).arg("inspect").output().unwrap();
    assert!(o.status.success());let st:State=serde_json::from_slice(&o.stdout).unwrap();
    assert_eq!(st.clock,7);assert_eq!(st.goals.len(),1);
    assert_eq!(fs::read_dir(&tmp.0).unwrap().filter(|x|x.as_ref().unwrap().file_name().to_string_lossy().ends_with(".rev.json")).count(),2);
}
#[test]
fn budget_exhaustion_and_explicit_failure(){
    let tmp=Temp::new();let s=tmp.store();setup(&s,"g");
    task(&s,"g","t",vec![],json!(1),json!(2),None);
    for _ in 0..3 {let n=claim(&s,"t","a");put(&s,json!({"cmd":"fail","task":"t","agent":"a","generation":n,"reason":"simulated failure"}));}
    let st=s.inspect().unwrap();assert_eq!(st.tasks["t"].status,TaskStatus::Exhausted);assert!(st.ready().is_empty());assert_eq!(st.goals["g"].status,GoalStatus::Open);
}
