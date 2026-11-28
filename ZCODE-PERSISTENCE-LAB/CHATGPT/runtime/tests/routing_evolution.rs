//! Source-only policy evolution guard fixtures. Synthetic accounting only.
use persistent_intelligence_runtime::kernel::{routing::{Config,Router,Demand,Usage,Outcome,Status},routing_optimizer::BatchPolicy,routing_evolution::{TrialSpec,Phase}};
use serde_json::json;
use std::collections::{BTreeMap,BTreeSet};
fn setup()->Router{
    let mut c:Config=serde_json::from_str(include_str!("../routing.example.json")).unwrap();
    for p in c.profiles.values_mut(){p.enabled=p.id=="a-standard-low";}
    let mut r=Router::default();r.configure(c).unwrap();
    r.set_batch_policy(BatchPolicy{version:1,..BatchPolicy::default()}).unwrap();
    for k in ["b1","b2","c1","c2","outside"]{
        let d:Demand=serde_json::from_value(json!({"id":k,"goal":format!("goal-{k}"),"task":format!("task-{k}"),"family":"code","capabilities":["code"],"input_tokens":100,"output_tokens":100,"reasoning_tokens":100,"priority":100,"uncertainty_bps":0,"information_gain_bps":0,"minimum_success_bps":0,"deadline_ms":null,"allowed_profiles":["a-standard-low"],"allowed_providers":[],"allow_frontier":false,"independent_review":false,"prefer_native":false,"extra_pools":[]})).unwrap();
        r.queue(d).unwrap();
    }
    r
}
fn spec(r:&Router)->TrialSpec{
    let caps:BTreeMap<_,_>=r.config.as_ref().unwrap().pools.iter().map(|(k,p)|(k.clone(),if k=="global-daily-money"{1000}else{p.limit.min(4)})).collect();
    TrialSpec{id:"trial".into(),candidate:BatchPolicy{version:2,beam_width:8,..BatchPolicy::default()},baseline_demands:BTreeSet::from(["b1".into(),"b2".into()]),candidate_demands:BTreeSet::from(["c1".into(),"c2".into()]),duration_ms:1000,caps,objective_pool:"global-daily-money".into(),min_samples:2,min_success_bps:10_000,min_savings_bps:1000,auto_promote:true,auto_advance:true}
}
fn settle(r:&mut Router,k:&str,output:u64,accepted:bool){
    let key=format!("hold-{k}");
    assert_eq!(r.reserve(key.clone(),k).unwrap()["reserved"],true);
    r.dispatch(&key,1,format!("worker-{k}"),1).unwrap();
    r.settle(&key,Usage{requests:1,input:10,output,reasoning:0,credits:1},Outcome::Completed,format!("fixture-{k}"),1).unwrap();
    r.record_verified(&key,accepted,format!("evidence-{k}")).unwrap();
}
#[test]
fn matched_observations_can_promote_and_rollback_under_same_caps(){
    let mut r=setup();let s=spec(&r);r.begin_trial(s).unwrap();
    assert!(r.recommend("outside").unwrap().chosen.is_none());
    assert!(r.recommend("c1").unwrap().chosen.is_none());
    settle(&mut r,"b1",100,true);settle(&mut r,"b2",100,true);
    assert!(r.close_trial_arm().is_err());
    r.tick(1000).unwrap();r.close_trial_arm().unwrap();
    assert_eq!(r.evolution.trials["trial"].phase,Phase::Between);
    r.start_candidate_arm().unwrap();
    settle(&mut r,"c1",10,true);settle(&mut r,"c2",10,true);
    r.tick(2000).unwrap();let result=r.close_trial_arm().unwrap();
    assert_eq!(result["promoted"],true);assert_eq!(r.batch_policy.version,2);
    assert_eq!(r.evolution.trials["trial"].candidate_report.as_ref().unwrap().successes,2);
    r.rollback_trial("trial").unwrap();assert_eq!(r.batch_policy.version,3);
    assert_eq!(r.batch_policy.beam_width,BatchPolicy::default().beam_width);
}
#[test]
fn unknown_charge_blocks_closure_and_abort_does_not_release_it(){
    let mut r=setup();let s=spec(&r);r.begin_trial(s).unwrap();
    r.reserve("hold".into(),"b1").unwrap();r.dispatch("hold",1,"worker".into(),1).unwrap();
    r.unknown("hold").unwrap();r.tick(1000).unwrap();
    assert!(r.close_trial_arm().is_err());
    assert!(r.trial_auto_action().is_none());
    r.abort_trial("operator stop".into()).unwrap();
    assert_eq!(r.reservations["hold"].status,Status::Unknown);
    assert_eq!(r.batch_policy.version,1);assert_eq!(r.evolution.trials["trial"].phase,Phase::Aborted);
}
#[test]
fn missing_work_counts_against_challenger_and_blocks_promotion(){
    let mut r=setup();let s=spec(&r);r.begin_trial(s).unwrap();
    settle(&mut r,"b1",100,true);settle(&mut r,"b2",100,true);
    r.tick(1000).unwrap();r.close_trial_arm().unwrap();r.start_candidate_arm().unwrap();
    settle(&mut r,"c1",1,true);r.tick(2000).unwrap();
    let v=r.close_trial_arm().unwrap();assert_eq!(v["promoted"],false);
    let c=r.evolution.trials["trial"].candidate_report.as_ref().unwrap();
    assert_eq!(c.enrolled,2);assert_eq!(c.success_bps,5000);assert_eq!(r.batch_policy.version,1);
}
