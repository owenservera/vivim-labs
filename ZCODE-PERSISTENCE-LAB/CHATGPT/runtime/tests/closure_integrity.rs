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
