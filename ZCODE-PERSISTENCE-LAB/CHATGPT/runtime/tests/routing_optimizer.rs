//! Authored qualification cases; not executed during coding-only phase.
use persistent_intelligence_runtime::kernel::{routing::{Config,Router,Demand},routing_optimizer::BatchPolicy};
use serde_json::json;
use std::collections::BTreeSet;
fn demand(id:&str,priority:u16,profiles:&[&str])->Demand{
    serde_json::from_value(json!({"id":id,"goal":format!("goal-{id}"),"task":format!("task-{id}"),"family":"code","capabilities":["code"],"input_tokens":10,"output_tokens":10,"reasoning_tokens":10,"priority":priority,"uncertainty_bps":0,"information_gain_bps":0,"minimum_success_bps":0,"deadline_ms":null,"allowed_profiles":profiles,"allowed_providers":[],"allow_frontier":false,"independent_review":false,"prefer_native":true,"extra_pools":[]})).unwrap()
}
#[test]
fn joint_assignment_preserves_the_only_compatible_resource(){
    let mut c:Config=serde_json::from_str(include_str!("../routing.example.json")).unwrap();
    c.policy.price_weight=0;c.policy.latency_weight=0;c.policy.scarcity_weight=0;c.policy.quality_weight=100;
    for p in c.profiles.values_mut(){p.enabled=!p.frontier;}
    c.profiles.get_mut("a-standard-low").unwrap().prior_success_bps=9000;
    c.profiles.get_mut("b-standard-medium").unwrap().prior_success_bps=8000;
    c.pools.get_mut("account-a-daily").unwrap().limit=1;
    let mut r=Router::default();r.configure(c).unwrap();
    r.queue(demand("flexible",200,&["a-standard-low","b-standard-medium"])).unwrap();
    r.queue(demand("constrained",100,&["a-standard-low"])).unwrap();
    assert_eq!(r.recommend("flexible").unwrap().chosen.unwrap().profile,"a-standard-low");
    r.set_batch_policy(BatchPolicy{version:1,beam_width:8,demand_limit:8,profiles_per_demand:4,node_budget:128,completion_weight:100,priority_weight:100}).unwrap();
    let allowed=BTreeSet::from(["flexible".to_owned(),"constrained".to_owned()]);
    let plan=r.optimize_batch(&allowed,2).unwrap();
    assert_eq!(plan.assignments.len(),2);
    assert!(plan.assignments.iter().any(|a|a.demand=="flexible"&&a.profile=="b-standard-medium"));
    assert!(plan.assignments.iter().any(|a|a.demand=="constrained"&&a.profile=="a-standard-low"));
    assert_eq!(plan.reserved_by_pool["account-a-daily"],1);
    assert!(r.reservations.is_empty(),"shadow search must not mutate real budgets");
    let report=r.compare_batch_policies(&allowed,2,vec![BatchPolicy{version:2,beam_width:1,demand_limit:8,profiles_per_demand:4,node_budget:128,completion_weight:100,priority_weight:100}]).unwrap();
    assert_eq!(report["policy_changed"],false);
    assert_eq!(r.batch_policy.version,1);
}
