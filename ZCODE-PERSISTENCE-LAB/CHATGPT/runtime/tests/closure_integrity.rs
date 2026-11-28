//! Source-only regression fixtures for closure review. No provider calls.
use persistent_intelligence_runtime::{digest,kernel::{model::*,index::Index,persistence::Receipt,transition::{Actor,Action},routing::{Router,Config,Demand,Usage,Outcome,Status}}};
use std::{collections::{BTreeMap,BTreeSet},fs,path::PathBuf,time::{SystemTime,UNIX_EPOCH}};
use serde_json::json;
struct Temp(PathBuf);
impl Temp{fn new()->Self{let p=std::env::temp_dir().join(format!("vivim-index-{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));fs::create_dir(&p).unwrap();Self(p)}}
impl Drop for Temp{fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
#[test]
fn adaptive_index_splits_collision_and_reopens(){
    let temp=Temp::new();let index=Index::open(&temp.0).unwrap();let mut world=World::default();
    let mut buckets:BTreeMap<String,Vec<String>>=BTreeMap::new();
    let keys=loop{
        let n:usize=buckets.values().map(|x|x.len()).sum();let key=format!("entity-{n}");
        let hash=digest(&json!(key));let group=buckets.entry(hash[..2].into()).or_default();group.push(key);
        if group.len()==3{break group.clone();}
    };
    for k in &keys{world.blobs.insert(k.clone(),json!("x".repeat(400_000)));}
    world.sequence=1;
    let receipts=BTreeMap::<Id,Receipt>::new();
    let first=index.write(1,&world,&receipts,None).unwrap();
    let root=index.manifest(&first).unwrap();assert_eq!(root.schema,3);
    assert!(root.directories.contains_key("blobs"));assert!(root.tables.is_empty());
    for k in &keys{assert_eq!(index.lookup(&first,1,"blobs",k).unwrap(),world.blobs.get(k).cloned());}
    let (restored,_)=index.load(&first,1).unwrap();assert_eq!(restored.blobs,world.blobs);
    world.sequence=2;world.blobs.insert(keys[0].clone(),json!(42));
    let second=index.write(2,&world,&receipts,Some(&first)).unwrap();
    index.collect(&BTreeSet::from([first.clone(),second.clone()])).unwrap();
    assert_eq!(index.lookup(&second,2,"blobs",&keys[0]).unwrap(),Some(json!(42)));
    assert_ne!(index.lookup(&first,1,"blobs",&keys[0]).unwrap(),Some(json!(42)));
}
#[test]
fn terminal_parent_fences_open_descendants(){
    let mut w=World::default();
    let host=Actor::Host;
    for (key,parent) in [("parent",None),("child",Some("parent")),("grandchild",Some("child"))]{
        w.transition(&host,Action::Goal{id:key.into(),parent:parent.map(str::to_owned),objective:"hierarchy fixture".into(),claim_budget:1,predicate:None}).unwrap();
    }
    w.transition(&host,Action::Cancel{goal:"parent".into()}).unwrap();
    for key in ["parent","child","grandchild"]{assert_eq!(w.goals[key].phase,GoalPhase::Cancelled);}
}
#[test]
fn cancelled_output_preserves_charge_and_no_success_credit(){
    let mut c:Config=serde_json::from_str(include_str!("../routing.example.json")).unwrap();c.profiles.get_mut("a-standard-low").unwrap().enabled=true;
    let mut r=Router::default();r.configure(c).unwrap();
    let d:Demand=serde_json::from_value(json!({"id":"d","goal":"goal","task":"task","family":"code","capabilities":["code"],"input_tokens":10,"output_tokens":10,"reasoning_tokens":10,"priority":100,"uncertainty_bps":0,"information_gain_bps":0,"minimum_success_bps":0,"deadline_ms":null,"allowed_profiles":["a-standard-low"],"allowed_providers":[],"allow_frontier":false,"independent_review":false,"prefer_native":false,"extra_pools":[]})).unwrap();
    r.queue(d).unwrap();assert_eq!(r.reserve("hold".into(),"d").unwrap()["reserved"],true);
    r.dispatch("hold",1,"worker".into(),1).unwrap();
    r.settle("hold",Usage{requests:1,input:1,output:1,reasoning:0,credits:1},Outcome::Completed,"receipt".into(),1).unwrap();
    let charged=r.reservations["hold"].actual.clone();r.discard_unverified(&BTreeSet::new());r.cancel_goal("goal");
    assert_eq!(r.reservations["hold"].status,Status::Settled);assert_eq!(r.reservations["hold"].actual,charged);
    assert!(r.reservations["hold"].not_evaluated_reason.is_some());
    assert!(r.record_verified("hold",true,"late-evidence".into()).is_err());
    assert_eq!(r.reservations["hold"].verified,None);
}

#[test]
fn native_attempt_accounting_is_atomic_and_host_owned(){
    use persistent_intelligence_runtime::{Expr,Operator,kernel::resource::{Receipt as ResourceReceipt,Counters}};
    let mut w=World::default();
    let host=Actor::Host;
    let mut run=|action:Action|w.transition(&host,action).unwrap();
    run(Action::Goal{id:"accounting".into(),parent:None,objective:"atomic measured attempt".into(),claim_budget:2,predicate:None});
    run(Action::Slots{count:1});
    run(Action::Context{context:Context{id:"context".into(),version:1,entries:BTreeMap::new(),artifacts:BTreeMap::new(),byte_budget:MAX_VALUE_BYTES}});
    let shape=Shape::Record{fields:BTreeMap::from([("x".into(),Shape::Integer)])};
    run(Action::Operator{contract:OperatorContract{id:"identity".into(),version:1,behavior:Operator::Execute,input:shape.clone(),output:Shape::Integer,program:Expr::Field{source:Box::new(Expr::Input),key:"x".into()}}});
    run(Action::Class{class:Class{id:"class".into(),version:1,input:shape,output:Shape::Integer,nodes:vec![Node{id:"n".into(),operator:"identity@1".into(),input:Wire::Input}],result:Wire::Node{id:"n".into()},lineage:vec![]}});
    run(Action::Agent{id:"worker".into(),goal:"accounting".into(),parent:None,class:"class@1".into(),context:"context@1".into()});
    run(Action::Task{spec:TaskSpec{id:"t".into(),goal:"accounting".into(),objective:"identity".into(),dependencies:BTreeSet::new(),inputs:BTreeMap::from([("x".into(),Port{shape:Shape::Integer,binding:Binding::Literal{value:json!(42)}})]),output:Shape::Integer,oracle:Oracle::Digest{expected:digest(&json!(42))},class:Some("class@1".into()),family:None,context:Some("context@1".into()),max_attempts:1,priority:0,uncertainty:0,information_gain:0,expected_cost:1}});
    run(Action::Activate{agent:"worker".into(),executor:"native-pure-process".into()});
    run(Action::Claim{task:"t".into(),agent:"worker".into(),ttl_ms:1000});
    run(Action::BindExecutor{agent:"worker".into(),incarnation:1,process_ref:"native-pid-123".into()});
    drop(run);
    let r=ResourceReceipt{task:"t".into(),generation:1,process_ref:"native-pid-123".into(),elapsed_ms:17,counters:Counters::default()};
    let worker=Actor::Worker{id:"worker".into(),incarnation:1};
    assert!(w.transition(&worker,Action::RecordAttemptAccounting{receipt:r.clone()}).is_err());
    let mut wrong=r.clone();wrong.process_ref="native-pid-999".into();
    assert!(w.transition(&host,Action::RecordAttemptAccounting{receipt:wrong}).is_err());
    assert!(w.run_costs.is_empty());assert!(w.resource_receipts.is_empty());
    assert_eq!(w.metrics.worker_elapsed_ms,0);
    w.transition(&host,Action::RecordAttemptAccounting{receipt:r.clone()}).unwrap();
    assert_eq!(w.run_costs["t-1"].elapsed_ms,17);
    assert_eq!(w.resource_receipts["t-1"].elapsed_ms,17);
    assert_eq!(w.attempt_history["t-1"].measured_ms,Some(17));
    assert_eq!(w.metrics.worker_elapsed_ms,17);
    assert!(w.transition(&host,Action::RecordAttemptAccounting{receipt:r}).is_err());
    assert_eq!(w.metrics.worker_elapsed_ms,17);
}
#[test]
fn initialization_batch_is_atomic_and_scan_is_revision_fenced(){
    use persistent_intelligence_runtime::kernel::{persistence::KernelStore,transition::Request,index::ScanCursor};
    let temp=Temp::new();let store=KernelStore::open(&temp.0).unwrap();
    let goal=|key:&str|Action::Goal{id:key.into(),parent:None,objective:"batch fixture".into(),claim_budget:2,predicate:None};
    let apply=|key:&str,action:Action|store.apply(Request{id:key.into(),actor:Actor::Host,action});
    assert!(apply("bad",Action::InitializeBatch{actions:vec![goal("a"),goal("a")]}).is_err());
    assert_eq!(store.anchor().unwrap().revision,0);assert!(store.inspect().unwrap().goals.is_empty());
    assert!(apply("forbidden",Action::InitializeBatch{actions:vec![goal("a"),Action::Tick{now_ms:1}]}).is_err());
    let action=Action::InitializeBatch{actions:(0..7).map(|i|goal(&format!("g-{i}"))).collect()};
    let receipt=apply("good",action.clone()).unwrap();assert_eq!(receipt["initialized"],7);
    assert_eq!(receipt,apply("good",action).unwrap());assert_eq!(store.anchor().unwrap().revision,1);
    let mut cursor=None;let mut seen=BTreeSet::new();
    loop{
        let v=store.scan("goals",cursor,2,2,64*1024).unwrap();
        for row in v["page"]["entries"].as_array().unwrap(){assert!(seen.insert(row[0].as_str().unwrap().to_owned()));}
        cursor=serde_json::from_value::<Option<ScanCursor>>(v["page"]["next"].clone()).unwrap();
        if cursor.is_none(){break;}
    }
    assert_eq!(seen.len(),7);
    let v=store.scan("goals",None,1,1,64*1024).unwrap();
    let cursor:Option<ScanCursor>=serde_json::from_value(v["page"]["next"].clone()).unwrap();assert!(cursor.is_some());
    apply("new",goal("new")).unwrap();
    assert!(store.scan("goals",cursor,2,2,64*1024).is_err());
}

#[test]
fn atomic_host_measurement_replays_without_second_revision(){
    use persistent_intelligence_runtime::kernel::{persistence::KernelStore,transition::Request};
    let temp=Temp::new();let store=KernelStore::open(&temp.0).unwrap();
    let r=Request{id:"metered".into(),actor:Actor::Host,action:Action::GenerateDemands{limit:1}};
    let response=store.apply_metered(r.clone(),"routing").unwrap();
    assert_eq!(store.anchor().unwrap().revision,1);
    let w=store.inspect().unwrap();
    let m=&w.host_costs["metered"];assert_eq!(m.operation,"transition-routing");assert_eq!(m.result_digest.as_ref(),Some(&digest(&response)));
    let reopened=KernelStore::open(&temp.0).unwrap();
    assert_eq!(reopened.apply_metered(r.clone(),"routing").unwrap(),response);
    assert_eq!(reopened.anchor().unwrap().revision,1);
    assert!(reopened.apply_metered(r,"learning").is_err());
    let bad=Request{id:"denied".into(),actor:Actor::Host,action:Action::Tick{now_ms:2}};
    assert!(reopened.apply_metered(bad,"routing").is_err());
    assert_eq!(reopened.anchor().unwrap().revision,1);
}

#[test]
fn retired_journal_and_pinned_state_survive_checkpoint_collection(){
    use persistent_intelligence_runtime::kernel::{persistence::KernelStore,transition::Request};
    let temp=Temp::new();let store=KernelStore::open(&temp.0).unwrap();
    let apply=|key:&str,action:Action|store.apply(Request{id:key.into(),actor:Actor::Host,action});
    apply("initial",Action::Goal{id:"initial".into(),parent:None,objective:"retain evidence".into(),claim_budget:2,predicate:None}).unwrap();
    let pin=store.pin_history("initial-state").unwrap();
    assert_eq!(pin.revision,1);
    for n in 2..=66{apply(&format!("tick-{n}"),Action::Tick{now_ms:n}).unwrap();}
    store.collect_garbage().unwrap();
    let old=store.historical_record(1).unwrap();assert_eq!(old["available"],true);assert_eq!(old["source"],"archive");
    assert_eq!(old["anchor"]["digest"],pin.anchor.digest);
    assert_eq!(store.historical_lookup("initial-state","goals","initial").unwrap().unwrap()["id"],"initial");
    assert!(store.historical_lookup("initial-state","receipts","tick-2").unwrap().is_none());
    let receipt=store.receipt_status("initial").unwrap();assert_eq!(receipt["found"],true);assert_eq!(receipt["revision"],1);
    assert_eq!(receipt["send_authority"],"none");assert!(receipt.get("response").is_none());
    assert_eq!(store.history_pins().unwrap().len(),1);
    let reopened=KernelStore::open(&temp.0).unwrap();assert_eq!(reopened.historical_record(1).unwrap(),old);
    assert_eq!(reopened.anchor().unwrap().revision,66);
}
