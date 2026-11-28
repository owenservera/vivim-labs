//! Qualification fixtures only. These tests have not been run at authoring time.
use super::{model::*,persistence::KernelStore,transition::{Actor,Action,Request}};
use crate::Result;
use std::{fs,path::{Path,PathBuf},process::{Child,Command,Stdio},thread,time::{Duration,Instant,SystemTime,UNIX_EPOCH}};
use serde_json::json;

pub(crate) fn pause_at(root:&Path,stage:&str)->Result<()>{
    if std::env::var("VIVIM_TEST_PAUSE_STAGE").ok().as_deref()!=Some(stage){return Ok(());}
    let expected=std::env::var_os("VIVIM_TEST_STORE").ok_or("missing test root")?;
    if root!=Path::new(&expected){return Ok(());}
    fs::write(root.join("test-pause-ready"),stage).map_err(|e|e.to_string())?;
    // Parent kills this owned test process. Finite watchdog avoids an orphan.
    thread::sleep(Duration::from_secs(30));Err("qualification pause timed out".into())
}
struct Temp(PathBuf);
impl Temp{fn new()->Self{
    let p=std::env::temp_dir().join(format!("vivim-qualification-{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
    fs::create_dir(&p).unwrap();Self(p)
}}
impl Drop for Temp{fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
struct Owned(Child);
impl Drop for Owned{fn drop(&mut self){if self.0.try_wait().ok().flatten().is_none(){let _=self.0.kill();let _=self.0.wait();}}}
fn goal(key:&str)->Request{Request{id:format!("req-{key}"),actor:Actor::Host,action:Action::Goal{id:key.into(),parent:None,objective:"qualification".into(),claim_budget:100,predicate:None}}}
#[test]
#[ignore="subprocess fixture; launched only by crash/concurrency tests"]
fn writer_child(){
    let Some(root)=std::env::var_os("VIVIM_TEST_STORE") else{return;};
    let key=std::env::var("VIVIM_TEST_GOAL").unwrap();
    KernelStore::open(PathBuf::from(root)).unwrap().apply(goal(&key)).unwrap();
}
fn child(root:&Path,key:&str,pause:Option<&str>)->Owned{
    let mut cmd=Command::new(std::env::current_exe().unwrap());
    cmd.args(["--ignored","--exact","kernel::qualification::writer_child","--nocapture"])
        .env("VIVIM_TEST_STORE",root).env("VIVIM_TEST_GOAL",key).env_remove("VIVIM_TEST_PAUSE_STAGE").stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    if let Some(s)=pause{cmd.env("VIVIM_TEST_PAUSE_STAGE",s);}
    Owned(cmd.spawn().unwrap())
}
fn wait_pause(root:&Path,c:&mut Owned){
    let start=Instant::now();
    while !root.join("test-pause-ready").exists(){
        assert!(start.elapsed()<Duration::from_secs(20),"writer failed to reach crash boundary");
        assert!(c.0.try_wait().unwrap().is_none(),"writer exited before crash boundary");
        thread::sleep(Duration::from_millis(10));
    }
}
#[test]
fn crash_before_and_after_commit_marker(){
    for (stage,expected) in [("before_head",false),("after_head",true)]{
        let temp=Temp::new();let root=&temp.0;
        let store=KernelStore::open(root).unwrap();store.apply(goal("baseline")).unwrap();
        let mut c=child(root,"candidate",Some(stage));wait_pause(root,&mut c);
        c.0.kill().unwrap();c.0.wait().unwrap();
        let fresh=KernelStore::open(root).unwrap();
        assert_eq!(fresh.inspect().unwrap().goals.contains_key("candidate"),expected);
        assert!(fresh.inspect().unwrap().goals.contains_key("baseline"));
        fresh.apply(goal("candidate")).unwrap();
        assert_eq!(fresh.inspect().unwrap().goals.len(),2);
        assert_eq!(fresh.anchor().unwrap().revision,2);
    }
}
#[test]
fn independent_concurrent_writers_preserve_all_commits(){
    let temp=Temp::new();
    let mut children:Vec<_>=(0..8).map(|n|child(&temp.0,&format!("writer-{n}"),None)).collect();
    for c in &mut children{assert!(c.0.wait().unwrap().success());}
    let s=KernelStore::open(&temp.0).unwrap();
    assert_eq!(s.inspect().unwrap().goals.len(),8);
    assert_eq!(s.anchor().unwrap().revision,8);
    assert!(s.lookup("goals","writer-3").unwrap().is_some());
}
#[test]
fn independent_anchor_detects_deleted_tail(){
    let temp=Temp::new();let s=KernelStore::open(&temp.0).unwrap();
    s.apply(goal("first")).unwrap();s.apply(goal("second")).unwrap();
    let anchor=s.anchor().unwrap();
    fs::remove_file(temp.0.join("00000000000000000002.head.json")).unwrap();
    assert!(s.verify_anchor(&anchor).is_err());
}
#[test]
fn sixty_logical_agents_four_lease_slots(){
    use std::collections::{BTreeMap,BTreeSet};
    use crate::{Expr,Operator,digest};
    let mut w=World::default();
    let host=Actor::Host;
    let mut run=|a:Action|w.transition(&host,a).unwrap();
    run(goal("population").action);
    run(Action::Slots{count:4});
    run(Action::Context{context:Context{id:"context".into(),version:1,entries:BTreeMap::new(),artifacts:BTreeMap::new(),byte_budget:MAX_VALUE_BYTES}});
    let shape=Shape::Record{fields:BTreeMap::from([("x".into(),Shape::Integer)])};
    run(Action::Operator{contract:OperatorContract{id:"identity".into(),version:1,behavior:Operator::Execute,input:shape.clone(),output:Shape::Integer,program:Expr::Field{source:Box::new(Expr::Input),key:"x".into()}}});
    run(Action::Class{class:Class{id:"class".into(),version:1,input:shape,output:Shape::Integer,nodes:vec![Node{id:"node".into(),operator:"identity@1".into(),input:Wire::Input}],result:Wire::Node{id:"node".into()},lineage:vec![]}});
    for n in 0..60{
        run(Action::Agent{id:format!("a-{n}"),goal:"population".into(),parent:None,class:"class@1".into(),context:"context@1".into()});
        run(Action::Task{spec:TaskSpec{id:format!("t-{n}"),goal:"population".into(),objective:"identity".into(),dependencies:BTreeSet::new(),inputs:BTreeMap::from([("x".into(),Port{shape:Shape::Integer,binding:Binding::Literal{value:json!(n)}})]),output:Shape::Integer,oracle:Oracle::Digest{expected:digest(&json!(n))},class:Some("class@1".into()),family:None,context:Some("context@1".into()),max_attempts:2,priority:0,uncertainty:0,information_gain:0,expected_cost:1}});
    }
    run(Action::Seal{goal:"population".into()});
    for n in 0..5{run(Action::Activate{agent:format!("a-{n}"),executor:"qualification".into()});}
    for n in 0..4{run(Action::Claim{task:format!("t-{n}"),agent:format!("a-{n}"),ttl_ms:1000});}
    drop(run);
    assert_eq!(w.running(),4);assert_eq!(w.agents.len(),60);
    assert!(w.transition(&host,Action::Claim{task:"t-4".into(),agent:"a-4".into(),ttl_ms:1000}).is_err());
    for n in 0..4{
        let actor=Actor::Worker{id:format!("a-{n}"),incarnation:1};
        w.transition(&actor,Action::Submit{task:format!("t-{n}"),generation:1,output:json!(n)}).unwrap();
        w.transition(&host,Action::Verify{task:format!("t-{n}")}).unwrap();
    }
    assert_eq!(w.metrics.accepted,4);assert_eq!(w.running(),0);
    assert_eq!(w.audit_population()["logical_agents"],60);
}
