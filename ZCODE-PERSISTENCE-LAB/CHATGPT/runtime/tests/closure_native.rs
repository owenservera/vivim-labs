//! Authored native closure qualification; do not run until authorized.
use persistent_intelligence_runtime::{Expr,Operator,digest,kernel::{model::*,transition::{Actor,Action,Request},persistence::KernelStore,supervisor::{RunConfig,supervise},genesis::Manifest}};
use std::{collections::{BTreeMap,BTreeSet},fs,path::PathBuf,time::{SystemTime,UNIX_EPOCH},process::{Command,Stdio,Child},thread,time::{Duration,Instant}};
use serde_json::json;
struct Root(PathBuf);
impl Root{fn new()->Self{let p=std::env::temp_dir().join(format!("vivim-closure-{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));fs::create_dir(&p).unwrap();Self(p)}}
impl Drop for Root{fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
struct Owned(Child);
impl Drop for Owned{fn drop(&mut self){if self.0.try_wait().ok().flatten().is_none(){let _=self.0.kill();let _=self.0.wait();}}}
fn apply(s:&KernelStore,n:&mut u64,a:Action){*n+=1;s.apply(Request{id:format!("seed-{n}"),actor:Actor::Host,action:a}).unwrap();}
#[test]
#[ignore="requires explicit native execution authorization"]
fn genesis_manifest_executes_and_reopens(){
    let root=Root::new();let store=KernelStore::open(root.0.join("store")).unwrap();
    let m=Manifest{id:"genesis".into(),objective:"native qualification".into(),tracker_digest:digest(&json!("fixture")),input:42,claim_budget:8};
    store.apply(Request{id:"initialize".into(),actor:Actor::Host,action:Action::Genesis{manifest:m}}).unwrap();
    let before=store.inspect().unwrap();assert_eq!(before.slots,1);assert_eq!(before.agents.len(),1);assert_eq!(before.tasks.len(),2);
    supervise(&store,&root.0.join("workspace"),RunConfig{duration_ms:120_000,lease_ms:30_000,poll_ms:10,max_jobs:8,max_learning_plans:0,stop_when_idle:true}).unwrap();
    let reopened=KernelStore::open(root.0.join("store")).unwrap();let w=reopened.inspect().unwrap();
    assert_eq!(w.goals["genesis"].phase,GoalPhase::Complete);
    assert_eq!(*w.artifact_value("genesis-first").unwrap(),json!(43));
    assert_eq!(*w.artifact_value("genesis-second").unwrap(),json!(44));
    assert!(w.router.reservations.is_empty());assert_eq!(w.running(),0);
}
#[test]
#[ignore="requires real subprocess population qualification"]
fn sixty_agents_complete_independent_native_work_with_four_slots(){
    let root=Root::new();let store_path=root.0.join("store");let workspace=root.0.join("workspace");let store=KernelStore::open(&store_path).unwrap();
    fs::create_dir(&workspace).unwrap();let mut n=0;
    apply(&store,&mut n,Action::Goal{id:"population".into(),parent:None,objective:"60 verified independent tasks".into(),claim_budget:120,predicate:None});
    apply(&store,&mut n,Action::Slots{count:4});
    apply(&store,&mut n,Action::Context{context:Context{id:"ctx".into(),version:1,entries:BTreeMap::new(),artifacts:BTreeMap::new(),byte_budget:MAX_VALUE_BYTES}});
    let input=Shape::Record{fields:BTreeMap::from([("x".into(),Shape::Integer)])};
    apply(&store,&mut n,Action::Operator{contract:OperatorContract{id:"identity".into(),version:1,behavior:Operator::Execute,input:input.clone(),output:Shape::Integer,program:Expr::Field{source:Box::new(Expr::Input),key:"x".into()}}});
    apply(&store,&mut n,Action::Class{class:Class{id:"class".into(),version:1,input,output:Shape::Integer,nodes:vec![Node{id:"identity".into(),operator:"identity@1".into(),input:Wire::Input}],result:Wire::Node{id:"identity".into()},lineage:vec![]}});
    for i in 0..60{
        apply(&store,&mut n,Action::Agent{id:format!("agent-{i}"),goal:"population".into(),parent:None,class:"class@1".into(),context:"ctx@1".into()});
        apply(&store,&mut n,Action::Task{spec:TaskSpec{id:format!("task-{i}"),goal:"population".into(),objective:"identity".into(),dependencies:BTreeSet::new(),inputs:BTreeMap::from([("x".into(),Port{shape:Shape::Integer,binding:Binding::Literal{value:json!(i)}})]),output:Shape::Integer,oracle:Oracle::Digest{expected:digest(&json!(i))},class:Some("class@1".into()),family:None,context:Some("ctx@1".into()),max_attempts:2,priority:0,uncertainty:0,information_gain:0,expected_cost:1}});
    }
    apply(&store,&mut n,Action::Seal{goal:"population".into()});
    let cfg=RunConfig{duration_ms:300_000,lease_ms:60_000,poll_ms:20,max_jobs:120,max_learning_plans:0,stop_when_idle:true};
    let config=root.0.join("config.json");fs::write(&config,serde_json::to_vec(&cfg).unwrap()).unwrap();
    let mut child=Owned(Command::new(env!("CARGO_BIN_EXE_persistent-intelligence-runtime")).arg("kernel-controller").arg(&store_path).arg(&workspace).arg(config).stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap());
    let start=Instant::now();let mut peak=0;
    loop{
        let w=store.inspect().unwrap();peak=peak.max(w.running());assert!(w.running()<=4);assert_eq!(w.agents.len(),60);
        if let Some(status)=child.0.try_wait().unwrap(){assert!(status.success());break;}
        assert!(start.elapsed()<Duration::from_secs(310));thread::sleep(Duration::from_millis(10));
    }
    let w=store.inspect().unwrap();assert_eq!(w.goals["population"].phase,GoalPhase::Complete);assert_eq!(w.artifacts.len(),60);assert!(peak>0&&peak<=4);
    let agents:BTreeSet<_>=w.attempt_history.values().map(|a|a.agent.clone()).collect();assert_eq!(agents.len(),60);
    assert!(w.agents.values().all(|a|a.incarnations.iter().all(|i|i.executor=="native-pure-process")));
    assert!(w.router.reservations.is_empty());
    for i in 0..60{assert_eq!(*w.artifact_value(&format!("task-{i}")).unwrap(),json!(i));}
}
