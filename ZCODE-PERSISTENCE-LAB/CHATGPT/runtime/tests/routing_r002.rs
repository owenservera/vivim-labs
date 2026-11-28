//! Source-only qualification fixtures. Do not execute during coding-only phase.
use persistent_intelligence_runtime::kernel::{routing::{Config,Router,Demand,Usage,Outcome,Status,DAY_MS},routing_planning::{FrontierGuard,Forecast}};
use serde_json::json;
fn config()->Config{
    let mut c:Config=serde_json::from_str(include_str!("../routing.example.json")).unwrap();
    for p in c.profiles.values_mut(){p.enabled=true;}
    c.policy.price_weight=0;c.policy.latency_weight=0;c.policy.scarcity_weight=0;
    c.policy.quality_weight=100;c.policy.reservation_ttl_ms=300_000;
    c
}
fn demand(id:&str,priority:u16,frontier:bool)->Demand{
    serde_json::from_value(json!({
        "id":id,"goal":format!("goal-{id}"),"task":format!("task-{id}"),"family":"code",
        "capabilities":["code"],"input_tokens":100,"output_tokens":100,"reasoning_tokens":100,
        "priority":priority,"uncertainty_bps":5000,"information_gain_bps":5000,
        "minimum_success_bps":0,"deadline_ms":null,"allowed_profiles":[],"allowed_providers":[],
        "allow_frontier":frontier,"independent_review":false,"prefer_native":true,"extra_pools":[]
    })).unwrap()
}
#[test]
fn frontier_pacing_and_emergency_reserve(){
    let mut r=Router::default();let mut c=config();
    c.pools.get_mut("account-a-frontier-daily").unwrap().limit=4;
    let f=c.pools.get_mut("account-a-frontier-daily").unwrap().frontier.as_mut().unwrap();
    f.emergency_bps=2500;f.opening_bps=10_000;f.per_goal_bps=10_000;f.minimum_uplift_bps=0;
    c.profiles.get_mut("a-frontier-high").unwrap().max_parallel=10;
    r.configure(c).unwrap();
    for n in 0..3{
        let id=format!("ordinary-{n}");let mut d=demand(&id,100,true);d.allowed_profiles.insert("a-frontier-high".into());
        r.queue(d).unwrap();assert_eq!(r.reserve(format!("reserve-{n}"),&id).unwrap()["reserved"],true);
    }
    let mut d=demand("ordinary-fourth",100,true);d.allowed_profiles.insert("a-frontier-high".into());r.queue(d).unwrap();
    assert!(r.recommend("ordinary-fourth").unwrap().chosen.is_none());
    let mut d=demand("critical",1000,true);d.allowed_profiles.insert("a-frontier-high".into());r.queue(d).unwrap();
    assert_eq!(r.reserve("critical-hold".into(),"critical").unwrap()["reserved"],true);
    assert_eq!(r.available("account-a-frontier-daily").unwrap(),0);
}
#[test]
fn unknown_hold_survives_reset_and_prevents_duplicate_demand(){
    let mut r=Router::default();let mut c=config();
    c.pools.get_mut("account-a-daily").unwrap().limit=1;
    r.configure(c).unwrap();r.tick(DAY_MS-100).unwrap();
    let mut d=demand("one",100,false);d.allowed_profiles.insert("a-standard-low".into());r.queue(d).unwrap();
    r.reserve("hold".into(),"one").unwrap();r.dispatch("hold",1,"worker".into(),1).unwrap();
    r.unknown("hold").unwrap();r.tick(DAY_MS+100).unwrap();
    assert_eq!(r.available("account-a-daily").unwrap(),0);
    assert!(r.recommend("one").unwrap().chosen.is_none());
    r.settle("hold",Usage::default(),Outcome::NotSent,"reconciled-not-sent".into(),0).unwrap();
    assert_eq!(r.available("account-a-daily").unwrap(),1);
}
#[test]
fn stale_observation_and_unsent_expiry(){
    let mut r=Router::default();let mut c=config();
    let p=c.pools.get_mut("account-a-daily").unwrap();p.require_observation=true;p.observation_ttl_ms=100;
    r.configure(c).unwrap();r.tick(10).unwrap();
    assert_eq!(r.available("account-a-daily").unwrap(),0);
    r.observe(serde_json::from_value(json!({"pool":"account-a-daily","at_ms":10,"remaining":2,"window_start_ms":0})).unwrap()).unwrap();
    let mut d=demand("one",100,false);d.allowed_profiles.insert("a-standard-low".into());r.queue(d).unwrap();
    r.reserve("hold".into(),"one").unwrap();assert_eq!(r.available("account-a-daily").unwrap(),1);
    r.tick(111).unwrap();assert_eq!(r.available("account-a-daily").unwrap(),0);
    assert!(r.dispatch("hold",1,"worker".into(),1).is_err());
    r.tick(300_011).unwrap();assert_eq!(r.reservations["hold"].status,Status::Released);
}
#[test]
fn adaptive_thinking_and_forecast_guard(){
    let mut r=Router::default();let mut c=config();
    let p=c.pools.get_mut("account-a-frontier-daily").unwrap();p.frontier.as_mut().unwrap().opening_bps=10_000;
    p.frontier.as_mut().unwrap().emergency_bps=0;p.frontier.as_mut().unwrap().minimum_uplift_bps=0;
    r.configure(c).unwrap();r.tick(1).unwrap();
    r.set_frontier_guard(FrontierGuard{version:1,max_forecast_bps:10_000,opportunity_weight:1,protect_pending_critical:true}).unwrap();
    r.set_forecast(Forecast{id:"forecast".into(),pool:"account-a-frontier-daily".into(),version:1,window_start_ms:0,expected_critical_units:20,confidence_bps:10_000,expires_ms:DAY_MS}).unwrap();
    let mut d=demand("adaptive",100,true);d.adaptive_thinking=true;d.reasoning_tokens=8000;d.minimum_reasoning_tokens=1000;
    r.queue(d).unwrap();
    let choice=r.recommend("adaptive").unwrap().chosen.unwrap();
    assert_ne!(choice.profile,"a-frontier-high");
    r.reserve("adaptive-hold".into(),"adaptive").unwrap();
    assert!(r.reservations["adaptive-hold"].ceiling.reasoning<=4000);
}
