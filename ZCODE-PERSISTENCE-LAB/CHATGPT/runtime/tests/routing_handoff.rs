//! End-to-end handoff contract fixture; authored only, not executed.
use persistent_intelligence_runtime::{Expr,Operator,digest,kernel::{model::*,transition::{Actor,Action,Request},persistence::KernelStore,routing::{Config,Demand,Usage,Outcome},routing_adapter::{AdapterBinding,AdapterReceipt}}};
use serde_json::{Value,json};
use std::{collections::{BTreeMap,BTreeSet},fs,path::PathBuf,time::{SystemTime,UNIX_EPOCH}};
struct Temp(PathBuf);
impl Drop for Temp{fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
#[test]
fn one_shot_handoff_and_evidence_feedback_survive_reopen(){
    let temp=Temp(std::env::temp_dir().join(format!("vivim-route-{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())));
    let store=KernelStore::open(&temp.0).unwrap();
    let host=|id:&str,action:Action|->Value{store.apply(Request{id:id.into(),actor:Actor::Host,action}).unwrap()};
    let mut config:Config=serde_json::from_str(include_str!("../routing.example.json")).unwrap();
    config.profiles.get_mut("a-standard-low").unwrap().enabled=true;
    host("config",Action::ConfigureRouter{config});
    host("goal",Action::Goal{id:"goal".into(),parent:None,objective:"verified output".into(),claim_budget:8,predicate:None});
    let input=Shape::Record{fields:BTreeMap::from([("x".into(),Shape::Integer)])};
    host("context",Action::Context{context:Context{id:"ctx".into(),version:1,entries:BTreeMap::new(),artifacts:BTreeMap::new(),byte_budget:MAX_VALUE_BYTES}});
    host("operator",Action::Operator{contract:OperatorContract{id:"contract".into(),version:1,behavior:Operator::Execute,input:input.clone(),output:Shape::Integer,program:Expr::Field{source:Box::new(Expr::Input),key:"x".into()}}});
    host("class",Action::Class{class:Class{id:"adapter-class".into(),version:1,input,output:Shape::Integer,nodes:vec![Node{id:"result".into(),operator:"contract@1".into(),input:Wire::Input}],result:Wire::Node{id:"result".into()},lineage:vec![]}});
    host("task",Action::Task{spec:TaskSpec{id:"task".into(),goal:"goal".into(),objective:"identity".into(),dependencies:BTreeSet::new(),inputs:BTreeMap::from([("x".into(),Port{shape:Shape::Integer,binding:Binding::Literal{value:json!(42)}})]),output:Shape::Integer,oracle:Oracle::Digest{expected:digest(&json!(42))},class:None,family:Some("code".into()),context:Some("ctx@1".into()),max_attempts:3,priority:0,uncertainty:0,information_gain:0,expected_cost:1}});
    host("seal",Action::Seal{goal:"goal".into()});
    host("bind",Action::BindRoutingAdapter{binding:AdapterBinding{id:"future-adapter".into(),adapter:"future-adapter".into(),version:1,enabled:true,class:"adapter-class@1".into(),context:"ctx@1".into(),lease_ms:60_000}});
    let demand:Demand=serde_json::from_value(json!({"id":"demand","goal":"goal","task":"task","family":"code","capabilities":["code"],"input_tokens":10,"output_tokens":10,"reasoning_tokens":10,"priority":100,"uncertainty_bps":2000,"information_gain_bps":1000,"minimum_success_bps":0,"deadline_ms":null,"allowed_profiles":["a-standard-low"],"allowed_providers":[],"allow_frontier":false,"independent_review":false,"prefer_native":false,"extra_pools":[]})).unwrap();
    host("demand",Action::QueueInference{demand});
    let result=host("cycle",Action::RoutingCycle{limit:1});
    let handoff=result["prepared"][0]["handoff"]["id"].as_str().unwrap().to_owned();
    let manifest=digest(&json!("manifest"));let packet=digest(&json!("packet"));
    let claim=Action::ClaimTransportHandoff{handoff:handoff.clone(),transport:persistent_intelligence_runtime::kernel::routing_adapter::Transport::Mailbox{manifest_digest:manifest.clone()}};
    let first=host("claim",claim.clone());assert_eq!(first["send_authority"],"publish_only");
    let reopened=KernelStore::open(&temp.0).unwrap();
    let replay=reopened.apply(Request{id:"claim".into(),actor:Actor::Host,action:claim}).unwrap();
    assert_eq!(replay["send_authority"],"none_reconcile_only");
    assert!(reopened.apply(Request{id:"different-claim".into(),actor:Actor::Host,action:Action::ClaimHandoff{handoff:handoff.clone()}}).is_err());
    // A second-stage mailbox authority is committed only once in the kernel.
    let mailbox_take=Action::AuthorizeMailboxTake{handoff:handoff.clone(),manifest_digest:digest(&json!("manifest")),
        packet_digest:digest(&json!("packet")),binding_version:1,input_digest:first["handoff"]["input_digest"].as_str().unwrap().to_owned()};
    let taken=host("mailbox-take",mailbox_take.clone());assert_eq!(taken["send_authority"],"single_attempt_only");
    let replay=reopened.apply(Request{id:"mailbox-take".into(),actor:Actor::Host,action:mailbox_take.clone()}).unwrap();
    assert_eq!(replay["send_authority"],"none_reconcile_only");
    assert!(reopened.apply(Request{id:"alternate-mailbox".into(),actor:Actor::Host,action:mailbox_take}).is_err());
    assert!(reopened.inspect().unwrap().router.adapters.mailbox_takes.contains_key(&handoff));
    // Synthetic receipt, never a real provider call.
    let receipt=AdapterReceipt{handoff:handoff.clone(),request_id:"synthetic-receipt".into(),provider:"provider-a".into(),account:"account-a".into(),model:"example-standard".into(),thinking:"low".into(),usage:Usage{requests:1,input:10,output:1,reasoning:0,credits:1},outcome:Outcome::Completed,provider_receipt:"fixture-only".into(),latency_ms:10,output:Some(json!(42))};
    assert!(reopened.apply(Request{id:"direct-finish".into(),actor:Actor::Host,action:Action::FinishHandoff{receipt:receipt.clone()}}).is_err());
    assert!(reopened.apply(Request{id:"wrong-packet".into(),actor:Actor::Host,action:Action::FinishMailboxHandoff{receipt:receipt.clone(),manifest_digest:manifest.clone(),packet_digest:digest(&json!("wrong"))}}).is_err());
    assert_eq!(host("finish",Action::FinishMailboxHandoff{receipt,manifest_digest:manifest,packet_digest:packet})["submitted"],true);
    host("verify",Action::Verify{task:"task".into()});
    let state=reopened.inspect().unwrap();
    assert_eq!(state.goals["goal"].phase,GoalPhase::Complete);
    assert_eq!(state.router.reservations[&handoff].verified,Some(true));
    assert_eq!(state.router.performance["a-standard-low"]["code"].passed,1);
}

#[test]
fn host_programmed_demand_generation_is_native_first_and_scope_bounded(){
    use persistent_intelligence_runtime::kernel::demand_policy::Policy;
    let temp=Temp(std::env::temp_dir().join(format!("vivim-demand-{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())));
    let store=KernelStore::open(&temp.0).unwrap();
    let host=|key:&str,action:Action|store.apply(Request{id:key.into(),actor:Actor::Host,action}).unwrap();
    let mut config:Config=serde_json::from_str(include_str!("../routing.example.json")).unwrap();
    config.profiles.get_mut("a-standard-low").unwrap().enabled=true;
    host("config",Action::ConfigureRouter{config});
    host("goal",Action::Goal{id:"goal".into(),parent:None,objective:"auto demand".into(),claim_budget:8,predicate:None});
    host("foreign",Action::Goal{id:"foreign".into(),parent:None,objective:"not authorized".into(),claim_budget:8,predicate:None});
    let shape=Shape::Record{fields:BTreeMap::from([("x".into(),Shape::Integer)])};
    host("context",Action::Context{context:Context{id:"ctx".into(),version:1,entries:BTreeMap::new(),artifacts:BTreeMap::new(),byte_budget:MAX_VALUE_BYTES}});
    host("operator",Action::Operator{contract:OperatorContract{id:"contract".into(),version:1,behavior:Operator::Execute,input:shape.clone(),output:Shape::Integer,program:Expr::Field{source:Box::new(Expr::Input),key:"x".into()}}});
    host("class",Action::Class{class:Class{id:"adapter-class".into(),version:1,input:shape,output:Shape::Integer,nodes:vec![Node{id:"result".into(),operator:"contract@1".into(),input:Wire::Input}],result:Wire::Node{id:"result".into()},lineage:vec![]}});
    host("bind",Action::BindRoutingAdapter{binding:AdapterBinding{id:"future-adapter".into(),adapter:"future-adapter".into(),version:1,enabled:true,class:"adapter-class@1".into(),context:"ctx@1".into(),lease_ms:60_000}});
    for (task,goal,class) in [("routed","goal",None),("native","goal",Some("adapter-class@1".to_owned())),("foreign-task","foreign",None)]{
        host(task,Action::Task{spec:TaskSpec{id:task.into(),goal:goal.into(),objective:"identity".into(),dependencies:BTreeSet::new(),inputs:BTreeMap::from([("x".into(),Port{shape:Shape::Integer,binding:Binding::Literal{value:json!(42)}})]),output:Shape::Integer,oracle:Oracle::Digest{expected:digest(&json!(42))},class,family:Some("code".into()),context:Some("ctx@1".into()),max_attempts:3,priority:50,uncertainty:20,information_gain:10,expected_cost:1}});
    }
    let p:Policy=serde_json::from_value(json!({"version":1,"enabled":true,"goals":["goal"],"max_new_per_cycle":1,
        "families":{"code":{"capabilities":["code"],"allowed_profiles":["a-standard-low"],"input_tokens":100,"output_tokens":20,"reasoning_tokens":10,"minimum_reasoning_tokens":0,"adaptive_thinking":false,"minimum_success_bps":0,"allow_frontier":false,"deadline_after_ms":null,"extra_pools":[],"max_input_bytes":1024}}})).unwrap();
    host("policy",Action::ConfigureDemandPolicy{policy:p});
    let w=store.inspect().unwrap();
    assert_eq!(w.demand_generation_plan(128).unwrap()["planned"].as_array().unwrap().len(),1);
    let generated=host("generate",Action::GenerateDemands{limit:128});
    assert_eq!(generated["created"].as_array().unwrap().len(),1);
    let w=KernelStore::open(&temp.0).unwrap().inspect().unwrap();
    assert_eq!(w.router.demands.values().next().unwrap().task,"routed");
    assert!(w.native_allowed("native"));assert!(!w.demand_generation_needed());
    assert!(host("again",Action::GenerateDemands{limit:128})["created"].as_array().unwrap().is_empty());
}
