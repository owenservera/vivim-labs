//! Source-only cross-goal assimilation fixture. No provider calls.
use persistent_intelligence_runtime::{digest,Expr,Operator,kernel::{assimilation::{Policy,Cohort,Instantiate},model::*,transition::{Action,Actor},index::Index,persistence::Receipt}};
use serde_json::{json,Value};
use std::{collections::{BTreeMap,BTreeSet},fs,path::PathBuf,time::{SystemTime,UNIX_EPOCH}};
struct Temp(PathBuf);
impl Temp{fn new()->Self{let p=std::env::temp_dir().join(format!("vivim-assimilation-{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));fs::create_dir(&p).unwrap();Self(p)}}
impl Drop for Temp{fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
fn host(w:&mut World,a:Action)->Value{w.transition(&Actor::Host,a).unwrap()}
fn goal(w:&mut World,key:&str){host(w,Action::Goal{id:key.into(),parent:None,objective:"typed verified trace".into(),claim_budget:16,predicate:None});}
fn seed(w:&mut World,n:i64)->(Id,Id,Id){
    let g=format!("g-{n}");let class=format!("class-{n}");let ctx=format!("ctx-{n}");let op=format!("op-{n}");let node=format!("node-{n}");
    goal(w,&g);
    let shape=Shape::Record{fields:BTreeMap::from([("x".into(),Shape::Integer)])};
    host(w,Action::Context{context:Context{id:ctx.clone(),version:1,entries:BTreeMap::from([("value".into(),json!(n))]),artifacts:BTreeMap::new(),byte_budget:1024}});
    host(w,Action::Operator{contract:OperatorContract{id:op.clone(),version:1,behavior:Operator::Execute,input:shape.clone(),output:Shape::Integer,program:Expr::Add{left:Box::new(Expr::Field{source:Box::new(Expr::Input),key:"x".into()}),right:Box::new(Expr::Const{value:json!(1)})}}});
    let class_key=format!("{class}@1");let context_key=format!("{ctx}@1");
    host(w,Action::Class{class:Class{id:class,version:1,input:shape,output:Shape::Integer,nodes:vec![Node{id:node.clone(),operator:format!("{op}@1"),input:Wire::Input}],result:Wire::Node{id:node},lineage:vec![]}});
    for step in 0..2{
        let task=format!("{g}-t-{step}");let first=format!("{g}-t-0");
        host(w,Action::Task{spec:TaskSpec{id:task,goal:g.clone(),objective:"increment".into(),dependencies:if step==0{BTreeSet::new()}else{BTreeSet::from([first.clone()])},
            inputs:BTreeMap::from([("x".into(),Port{shape:Shape::Integer,binding:if step==0{Binding::Context{key:"value".into()}}else{Binding::Predecessor{task:first}}})]),output:Shape::Integer,
            oracle:Oracle::Digest{expected:digest(&json!(n+step+1))},class:Some(class_key.clone()),family:Some("increment".into()),context:Some(context_key.clone()),
            max_attempts:2,priority:0,uncertainty:0,information_gain:0,expected_cost:1}});
    }
    host(w,Action::Seal{goal:g.clone()});
    (g,class_key,context_key)
}
fn finish(w:&mut World,g:&str,class:&str,ctx:&str,tasks:&[Id],n:i64){
    for (i,t) in tasks.iter().enumerate(){
        let a=format!("agent-{g}-{i}");
        host(w,Action::Agent{id:a.clone(),goal:g.into(),parent:None,class:class.into(),context:ctx.into()});
        host(w,Action::Activate{agent:a.clone(),executor:"fixture".into()});
        host(w,Action::Claim{task:t.clone(),agent:a.clone(),ttl_ms:1000});
        w.transition(&Actor::Worker{id:a,incarnation:1},Action::Submit{task:t.clone(),generation:1,output:json!(n+i as i64+1)}).unwrap();
        host(w,Action::Verify{task:t.clone()});
    }
    assert_eq!(w.goals[g].phase,GoalPhase::Complete);
}
#[test]
fn cross_goal_freeze_review_reuse_and_disable(){
    let mut w=World::default();
    let seeds:Vec<_>=(1..=4).map(|n|seed(&mut w,n)).collect();
    assert_eq!(w.structural_class_pin(&seeds[0].1).unwrap(),w.structural_class_pin(&seeds[1].1).unwrap());
    goal(&mut w,"review");
    host(&mut w,Action::ConfigureAssimilation{policy:Policy{family:"increment".into(),version:1,enabled:true,min_training_goals:2,min_holdout_goals:2,max_steps:4,max_cohorts:4,auto_promote:true}});
    let cohort=Cohort{id:"cohort".into(),family:"increment".into(),policy_version:1,review_goal:"review".into(),verifier:"verifier".into(),
        training_goals:vec![seeds[0].0.clone(),seeds[1].0.clone()],holdout_goals:vec![seeds[2].0.clone(),seeds[3].0.clone()]};
    host(&mut w,Action::RegisterTraceCohort{cohort:cohort.clone()});
    let mut overlap=cohort.clone();overlap.id="overlap".into();
    assert!(w.transition(&Actor::Host,Action::RegisterTraceCohort{cohort:overlap}).is_err());
    for (i,(g,class,ctx)) in seeds.iter().take(2).enumerate(){finish(&mut w,g,class,ctx,&[format!("{g}-t-0"),format!("{g}-t-1")],i as i64+1);}
    let frozen=host(&mut w,Action::FreezeTracePattern{cohort:"cohort".into()});
    let candidate=frozen["id"].as_str().unwrap().to_owned();assert!(frozen["rejection"].is_null());
    assert!(w.evaluate_trace_pattern(&candidate).is_err());
    assert!(w.transition(&Actor::Host,Action::PromoteTracePattern{candidate:candidate.clone()}).is_err());
    for (i,(g,class,ctx)) in seeds.iter().enumerate().skip(2){finish(&mut w,g,class,ctx,&[format!("{g}-t-0"),format!("{g}-t-1")],i as i64+1);}
    let verdict=w.evaluate_trace_pattern(&candidate).unwrap();assert!(verdict.passed);
    assert!(w.transition(&Actor::Verifier{id:"wrong".into()},Action::AttestTracePattern{verdict:verdict.clone()}).is_err());
    let mut false_verdict=verdict.clone();false_verdict.passed=false;
    assert!(w.transition(&Actor::Verifier{id:"verifier".into()},Action::AttestTracePattern{verdict:false_verdict}).is_err());
    w.transition(&Actor::Verifier{id:"verifier".into()},Action::AttestTracePattern{verdict}).unwrap();
    assert_eq!(w.ready_trace_promotions(),vec![candidate.clone()]);
    host(&mut w,Action::PromoteTracePattern{candidate:candidate.clone()});
    host(&mut w,Action::Context{context:Context{id:"new-context".into(),version:1,entries:BTreeMap::from([("value".into(),json!(99))]),artifacts:BTreeMap::new(),byte_budget:1024}});
    let request=Instantiate{candidate:candidate.clone(),goal:"reuse".into(),objective:"new context, same behavior".into(),parameters:BTreeMap::new(),
        classes:vec![seeds[0].1.clone(),seeds[1].1.clone()],contexts:vec!["new-context@1".into(),"new-context@1".into()],
        oracles:vec![Oracle::Digest{expected:digest(&json!(100))},Oracle::Digest{expected:digest(&json!(101))}],claim_budget:8};
    let mut invalid=request.clone();invalid.contexts[0]="missing@1".into();
    assert!(w.transition(&Actor::Host,Action::InstantiateTracePattern{request:invalid}).is_err());assert!(!w.goals.contains_key("reuse"));
    host(&mut w,Action::InstantiateTracePattern{request:request.clone()});
    // Execute each new step under its explicit class/context, not a prior agent.
    for i in 0..2{
        let task=format!("reuse-step-{i}");let agent=format!("reuse-agent-{i}");
        host(&mut w,Action::Agent{id:agent.clone(),goal:"reuse".into(),parent:None,class:request.classes[i].clone(),context:request.contexts[i].clone()});
        host(&mut w,Action::Activate{agent:agent.clone(),executor:"fixture".into()});
        let claim=host(&mut w,Action::Claim{task:task.clone(),agent:agent.clone(),ttl_ms:1000});
        let output=w.run_class(&request.classes[i],&claim["input"]).unwrap();assert_eq!(output,json!(100+i));
        w.transition(&Actor::Worker{id:agent,incarnation:1},Action::Submit{task:task.clone(),generation:1,output}).unwrap();
        host(&mut w,Action::Verify{task});
    }
    assert_eq!(w.goals["reuse"].phase,GoalPhase::Complete);
    let report=w.assimilation_report().unwrap();assert_eq!(report["candidate_yield"][0]["completed_goals"],1);
    let temp=Temp::new();let index=Index::open(&temp.0).unwrap();
    let root=index.write(w.sequence,&w,&BTreeMap::<Id,Receipt>::new(),None).unwrap();
    let (mut recovered,_)=index.load(&root,w.sequence).unwrap();
    assert!(recovered.assimilation.promotions[&candidate].enabled);
    host(&mut recovered,Action::DisableTracePattern{candidate:candidate.clone(),reason:"owner withdrawal".into()});
    let mut next=request;next.goal="new-reuse".into();
    assert!(recovered.transition(&Actor::Host,Action::InstantiateTracePattern{request:next}).is_err());
    assert_eq!(recovered.goals["reuse"].phase,GoalPhase::Complete);
}
