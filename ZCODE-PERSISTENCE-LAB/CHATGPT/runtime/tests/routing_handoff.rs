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
    let first=host("claim",Action::ClaimHandoff{handoff:handoff.clone()});assert_eq!(first["send_authority"],"single_attempt_only");
    let reopened=KernelStore::open(&temp.0).unwrap();
    let replay=reopened.apply(Request{id:"claim".into(),actor:Actor::Host,action:Action::ClaimHandoff{handoff:handoff.clone()}}).unwrap();
    assert_eq!(replay["send_authority"],"none_reconcile_only");
    assert!(reopened.apply(Request{id:"different-claim".into(),actor:Actor::Host,action:Action::ClaimHandoff{handoff:handoff.clone()}}).is_err());
    // Synthetic receipt, never a real provider call.
    let receipt=AdapterReceipt{handoff:handoff.clone(),request_id:"synthetic-receipt".into(),provider:"provider-a".into(),account:"account-a".into(),model:"example-standard".into(),thinking:"low".into(),usage:Usage{requests:1,input:10,output:1,reasoning:0,credits:1},outcome:Outcome::Completed,provider_receipt:"fixture-only".into(),latency_ms:10,output:Some(json!(42))};
    assert_eq!(host("finish",Action::FinishHandoff{receipt})["submitted"],true);
    host("verify",Action::Verify{task:"task".into()});
    let state=reopened.inspect().unwrap();
    assert_eq!(state.goals["goal"].phase,GoalPhase::Complete);
    assert_eq!(state.router.reservations[&handoff].verified,Some(true));
    assert_eq!(state.router.performance["a-standard-low"]["code"].passed,1);
}
