//! Independently supervised native-only workload and controller interruption.
//! No real provider calls, arbitrary program paths or shared-store mutation.
//! The report is scoped to this executable/workload, not a full gate verdict.
use super::{endurance::{Policy,Observation,Status},model::*,persistence::{KernelStore,Anchor},supervisor::RunConfig,transition::{Actor,Action,Request}};
use crate::{Expr,Operator,Result,digest};
use serde::{Deserialize,Serialize};
use serde_json::{Value,json};
use sha2::{Digest as ShaDigest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs::{self,File,OpenOptions},io::{Read,Write},path::Path,process::{Child,Command,Stdio},thread,time::{Duration,Instant}};

fn err(e:impl std::fmt::Display)->String{e.to_string()}
fn millis(t:Instant)->u64{u64::try_from(t.elapsed().as_millis()).unwrap_or(u64::MAX)}
fn value<T:Serialize>(x:&T)->Result<Value>{serde_json::to_value(x).map_err(err)}
fn hash_file(p:&Path)->Result<String>{
    let mut f=File::open(p).map_err(err)?;let mut hash=Sha256::new();let mut buf=[0u8;65536];
    loop{let n=f.read(&mut buf).map_err(err)?;if n==0{break;}hash.update(&buf[..n]);}
    Ok(format!("{:x}",hash.finalize()))
}
fn write_new(root:&Path,name:&str,data:&Value)->Result<()>{
    let path=root.join(name);let bytes=serde_json::to_vec_pretty(data).map_err(err)?;
    let mut f=OpenOptions::new().create_new(true).write(true).open(path).map_err(err)?;
    f.write_all(&bytes).map_err(err)?;f.sync_all().map_err(err)?;
    #[cfg(unix)] File::open(root).and_then(|f|f.sync_all()).map_err(err)?;
    Ok(())
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plan{
    pub version:u64,
    pub source_pin:String,
    pub seed:u64,
    pub tasks:u32,
    pub steps:u32,
    pub slots:usize,
    /// Interrupt the owned controller at these minimum verified-work counts.
    pub controller_faults_after:Vec<u32>,
    pub max_duration_ms:u64,
    pub max_starts:u32,
    pub total_job_budget:u64,
    pub jobs_per_start:u64,
    pub lease_ms:u64,
    pub poll_ms:u64,
    pub policy:Policy,
}
impl Plan{
    pub fn validate(&self)->Result<()>{
        self.policy.validate()?;
        ensure(self.version>0&&self.source_pin.len()==40&&self.source_pin.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b)),"valid declared git source pin required")?;
        ensure((1..=256).contains(&self.tasks)&&(1..=128).contains(&self.steps)&&(1..=4).contains(&self.slots),"invalid workload dimensions")?;
        ensure(self.controller_faults_after.len()<self.max_starts as usize&&self.controller_faults_after.windows(2).all(|w|w[0]<w[1])
            &&self.controller_faults_after.iter().all(|n|*n<self.tasks),"invalid fault schedule")?;
        ensure(self.max_duration_ms>0&&self.max_starts>0&&self.max_starts<=1024&&self.jobs_per_start>0
            &&self.total_job_budget>=u64::from(self.tasks)+self.controller_faults_after.len() as u64,
            "invalid finite execution budget")?;
        ensure(self.policy.min_verified_tasks<=u64::from(self.tasks)&&self.policy.min_fault_recoveries<=self.controller_faults_after.len() as u64
            &&self.policy.min_elapsed_ms.is_none_or(|x|x<=self.max_duration_ms),"policy target exceeds plan")?;
        RunConfig{duration_ms:self.max_duration_ms,lease_ms:self.lease_ms,poll_ms:self.poll_ms,max_jobs:self.jobs_per_start,max_learning_plans:0,stop_when_idle:true}.validate()
    }
    fn input(&self,i:u32)->i64{(self.seed%1_000_000) as i64+i64::from(i)}
    fn task(i:u32)->Id{format!("task-{i:04}")}
    fn expected(&self)->BTreeMap<Id,Value>{
        (0..self.tasks).map(|i|(Self::task(i),json!(self.input(i)+i64::from(self.steps)))).collect()
    }
}
struct Owned(Child);
impl Drop for Owned{fn drop(&mut self){if self.0.try_wait().ok().flatten().is_none(){let _=self.0.kill();let _=self.0.wait();}}}
fn seed(store:&KernelStore,p:&Plan,expected:&BTreeMap<Id,Value>)->Result<()>{
    let mut actions=Vec::new();let mut apply=|action|{
        actions.push(action);Ok::<(),String>(())
    };
    let shape=Shape::Record{fields:BTreeMap::from([("x".into(),Shape::Integer)])};
    apply(Action::Goal{id:"qualification".into(),parent:None,objective:"independent native workload".into(),claim_budget:p.total_job_budget,predicate:Some(Oracle::Digest{expected:digest(&value(expected)?)})})?;
    apply(Action::Slots{count:p.slots})?;
    apply(Action::Context{context:Context{id:"qualification-context".into(),version:1,entries:BTreeMap::new(),artifacts:BTreeMap::new(),byte_budget:MAX_VALUE_BYTES}})?;
    apply(Action::Operator{contract:OperatorContract{id:"first".into(),version:1,behavior:Operator::Execute,input:shape.clone(),output:Shape::Integer,program:Expr::Add{left:Box::new(Expr::Field{source:Box::new(Expr::Input),key:"x".into()}),right:Box::new(Expr::Const{value:json!(1)})}}})?;
    apply(Action::Operator{contract:OperatorContract{id:"increment".into(),version:1,behavior:Operator::Execute,input:Shape::Integer,output:Shape::Integer,program:Expr::Add{left:Box::new(Expr::Input),right:Box::new(Expr::Const{value:json!(1)})}}})?;
    let mut nodes=vec![Node{id:"first".into(),operator:"first@1".into(),input:Wire::Input}];
    for i in 1..p.steps{
        let previous=nodes.last().unwrap().id.clone();
        nodes.push(Node{id:format!("step-{i}"),operator:"increment@1".into(),input:Wire::Node{id:previous}});
    }
    let result=Wire::Node{id:nodes.last().unwrap().id.clone()};
    apply(Action::Class{class:Class{id:"qualification-class".into(),version:1,input:shape,output:Shape::Integer,nodes,result,lineage:vec![]}})?;
    for i in 0..p.tasks{
        let task=Plan::task(i);
        apply(Action::Task{spec:TaskSpec{id:task.clone(),goal:"qualification".into(),objective:"bounded integer increments".into(),dependencies:BTreeSet::new(),inputs:BTreeMap::from([("x".into(),Port{shape:Shape::Integer,binding:Binding::Literal{value:json!(p.input(i))}})]),output:Shape::Integer,oracle:Oracle::Digest{expected:digest(&expected[&task])},class:Some("qualification-class@1".into()),family:None,context:Some("qualification-context@1".into()),max_attempts:p.max_starts.saturating_add(1),priority:0,uncertainty:0,information_gain:0,expected_cost:1}})?;
    }
    for i in 0..p.slots{
        apply(Action::Agent{id:format!("qualification-agent-{i}"),goal:"qualification".into(),parent:None,class:"qualification-class@1".into(),context:"qualification-context@1".into()})?;
    }
    apply(Action::Seal{goal:"qualification".into()})?;
    drop(apply);
    // One durable revision per bounded batch, not per declaration. Seed
    // remains isolated; a failed later batch leaves an explicitly incomplete run.
    for (n,batch) in actions.chunks(32).enumerate(){
        store.apply(Request{id:format!("qualify-seed-{n}"),actor:Actor::Host,action:Action::InitializeBatch{actions:batch.to_vec()}})?;
    }
    Ok(())
}
#[derive(Clone,Debug,Serialize)]
pub struct Fault{
    pub ordinal:usize,pub trigger_verified:u32,pub controller_pid:u32,pub task:Id,
    pub generation:u64,pub injected_ms:u64,pub before:Anchor,
    pub controller_exit:Option<i32>,pub controller_exit_observed:bool,
    pub recovered_ms:Option<u64>,pub recovered_generation:Option<u64>,pub after:Option<Anchor>,
}
#[derive(Default)]
struct OracleState{
    seen:BTreeMap<Id,Digest>,
    lost:BTreeSet<Id>,
    duplicate_tasks:BTreeSet<Id>,
    bad_tasks:BTreeSet<Id>,
}
impl OracleState{
    fn observe(&mut self,w:&World,expected:&BTreeMap<Id,Value>)->Result<BTreeSet<Id>>{
        ensure(w.goals.len()==1&&w.goals.contains_key("qualification")&&w.tasks.keys().eq(expected.keys()),"unexpected workload or goal mutation")?;
        ensure(w.router.config.is_none()&&w.router.reservations.is_empty()&&w.effects.is_empty(),"unexpected provider or external effect authority")?;
        let mut by_task=BTreeMap::<Id,Vec<&Artifact>>::new();
        for a in w.artifacts.values(){by_task.entry(a.task.clone()).or_default().push(a);}
        let mut valid=BTreeSet::new();
        for (task,artifacts) in by_task{
            if artifacts.len()!=1{self.duplicate_tasks.insert(task.clone());continue;}
            let a=artifacts[0];
            let Some(expected)=expected.get(&task)else{self.bad_tasks.insert(task);continue;};
            let t=&w.tasks[&task];
            let accepted=w.evidence.get(&a.evidence).is_some_and(|e|e.accepted&&e.task==task&&e.generation==a.generation&&e.candidate==a.payload);
            let oracle=matches!(&t.spec.oracle,Oracle::Digest{expected:pin} if pin==&digest(expected));
            if !accepted||!oracle||t.phase!=TaskPhase::Verified||t.generation!=a.generation||t.artifact.as_deref()!=Some(a.id.as_str())||
                w.artifact_value(&a.id).ok()!=Some(expected)||a.payload!=digest(expected){
                self.bad_tasks.insert(task);continue;
            }
            let identity=digest(&json!([a.id,a.payload,a.generation,a.evidence]));
            if let Some(old)=self.seen.get(&task){if old!=&identity{self.lost.insert(task.clone());}}
            self.seen.insert(task.clone(),identity);valid.insert(task);
        }
        for task in self.seen.keys(){if !valid.contains(task){self.lost.insert(task.clone());}}
        Ok(valid)
    }
}
/// Execute only upon an explicit CLI invocation with a fresh root. Failure or
/// interruption leaves immutable partial evidence, never an invented report.
pub fn run(root:&Path,plan:Plan)->Result<Value>{
    plan.validate()?;
    let exe=std::env::current_exe().map_err(err)?;
    let executable_sha256=hash_file(&exe)?;
    fs::create_dir(root).map_err(err)?;
    let root=root.canonicalize().map_err(err)?;
    let store_path=root.join("store");let workspace=root.join("workspace");
    fs::create_dir(&workspace).map_err(err)?;
    let store=KernelStore::open(&store_path)?;
    let expected=plan.expected();let plan_digest=digest(&value(&plan)?);
    write_new(&root,"manifest.json",&json!({"plan":plan,"plan_digest":plan_digest,"executable_sha256":executable_sha256,"expected":expected,"source_pin_is_declared":true}))?;
    seed(&store,&plan,&expected)?;
    let start=Instant::now();let mut starts=0u32;let mut reserved=0u64;let mut peak=0usize;
    let mut faults:Vec<Fault>=vec![];let mut oracle=OracleState::default();let mut controller:Option<Owned>=None;
    let mut final_reason="budget";let mut valid=BTreeSet::new();let mut clean_exits=0u32;let mut abnormal_exits=0u32;
    loop{
        let (w,anchor)=store.inspect_with_anchor()?;
        valid=oracle.observe(&w,&expected)?;peak=peak.max(w.running());
        ensure(w.running()<=plan.slots,"observed lease concurrency exceeded configured slots")?;
        if let Some(f)=faults.last_mut().filter(|f|f.recovered_ms.is_none()){
            if valid.contains(&f.task)&&w.tasks[&f.task].generation>f.generation&&f.controller_exit_observed{
                f.recovered_ms=Some(millis(start).saturating_sub(f.injected_ms));
                f.recovered_generation=Some(w.tasks[&f.task].generation);f.after=Some(anchor.clone());
                write_new(&root,&format!("fault-{:04}-recovered.json",f.ordinal),&value(f)?)?;
            }
        }
        if root.join("STOP").exists(){final_reason="operator_stop";break;}
        if w.goals["qualification"].phase!=GoalPhase::Open{final_reason="terminal_goal";break;}
        if millis(start)>=plan.max_duration_ms{final_reason="duration_limit";break;}
        if let Some(c)=controller.as_mut(){
            if let Some(status)=c.0.try_wait().map_err(err)?{
                if status.success(){clean_exits+=1;}else{abnormal_exits+=1;}
                write_new(&root,&format!("controller-{starts:04}-exit.json"),&json!({"pid":c.0.id(),"success":status.success(),"exit_code":status.code(),"anchor":anchor,"elapsed_ms":millis(start)}))?;
                controller=None;continue;
            }
            if faults.last().is_none_or(|f|f.recovered_ms.is_some()){
                if let Some(&trigger)=plan.controller_faults_after.get(faults.len()).filter(|n|valid.len()>=**n as usize){
                    if let Some(t)=w.tasks.values().find(|t|t.phase==TaskPhase::Running&&t.lease.as_ref().is_some_and(|l|w.agents.get(&l.worker).and_then(|a|a.incarnations.last()).is_some_and(|i|i.process_ref.is_some()))){
                        let mut f=Fault{ordinal:faults.len()+1,trigger_verified:trigger,controller_pid:c.0.id(),task:t.spec.id.clone(),generation:t.generation,injected_ms:millis(start),before:anchor.clone(),controller_exit:None,controller_exit_observed:false,recovered_ms:None,recovered_generation:None,after:None};
                        write_new(&root,&format!("fault-{:04}-intent.json",f.ordinal),&value(&f)?)?;
                        c.0.kill().map_err(err)?;let status=c.0.wait().map_err(err)?;
                        f.controller_exit=status.code();f.controller_exit_observed=true;
                        write_new(&root,&format!("fault-{:04}-exit.json",f.ordinal),&value(&f)?)?;
                        faults.push(f);controller=None;continue;
                    }
                }
            }
        }else{
            if starts>=plan.max_starts||reserved>=plan.total_job_budget{break;}
            let remaining=plan.max_duration_ms.saturating_sub(millis(start));
            if remaining==0{final_reason="duration_limit";break;}
            let max_jobs=plan.jobs_per_start.min(plan.total_job_budget-reserved);
            starts+=1;reserved+=max_jobs;
            let config=RunConfig{duration_ms:remaining,lease_ms:plan.lease_ms,poll_ms:plan.poll_ms,max_jobs,max_learning_plans:0,stop_when_idle:true};
            let name=format!("controller-{starts:04}-config.json");
            write_new(&root,&name,&value(&config)?)?;
            write_new(&root,&format!("controller-{starts:04}-reservation.json"),&json!({"jobs":max_jobs,"reserved_jobs":reserved,"anchor":anchor,"elapsed_ms":millis(start)}))?;
            let stdout=OpenOptions::new().create_new(true).write(true).open(root.join(format!("controller-{starts:04}-stdout.json"))).map_err(err)?;
            let stderr=OpenOptions::new().create_new(true).write(true).open(root.join(format!("controller-{starts:04}-stderr.log"))).map_err(err)?;
            controller=Some(Owned(Command::new(&exe).arg("kernel-controller").arg(&store_path).arg(&workspace).arg(root.join(name)).stdin(Stdio::piped()).stdout(Stdio::from(stdout)).stderr(Stdio::from(stderr)).spawn().map_err(err)?));
        }
        thread::sleep(Duration::from_millis(plan.poll_ms.min(plan.max_duration_ms.saturating_sub(millis(start)))));
    }
    if let Some(mut c)=controller.take(){
        // Explicitly stop only the process this runner owns.
        if c.0.try_wait().map_err(err)?.is_none(){c.0.kill().map_err(err)?;c.0.wait().map_err(err)?;}
    }
    let (w,anchor)=store.inspect_with_anchor()?;valid=oracle.observe(&w,&expected)?;
    let recovered=faults.iter().filter(|f|f.recovered_ms.is_some()).count() as u64;
    let observation=Observation{elapsed_ms:Some(millis(start)),verified_tasks:Some(valid.len() as u64),fault_recoveries:Some(recovered),slowest_recovery_ms:faults.iter().filter_map(|f|f.recovered_ms).max(),lost_artifacts:Some(oracle.lost.len() as u64),duplicate_effects:Some(oracle.duplicate_tasks.len() as u64),human_interventions:Some(u64::from(final_reason=="operator_stop"))};
    let evaluation=plan.policy.evaluate(&observation)?;
    let complete=w.goals["qualification"].phase==GoalPhase::Complete&&valid.len()==expected.len()&&w.running()==0
        &&oracle.bad_tasks.is_empty()&&oracle.lost.is_empty()&&oracle.duplicate_tasks.is_empty()&&recovered==plan.controller_faults_after.len() as u64;
    let report=json!({"schema":1,"plan_digest":plan_digest,"executable_sha256":executable_sha256,"final_anchor":anchor,"stop_reason":final_reason,
        "workload_complete":complete,"policy_met":evaluation.status==Status::Met,"observation":observation,"evaluation":evaluation,
        "faults":faults,"reserved_jobs":reserved,"controller_starts":starts,"clean_exits":clean_exits,"abnormal_exits":abnormal_exits,
        "sampled_peak_leases":peak,"bad_tasks":oracle.bad_tasks,"lost_tasks":oracle.lost,"duplicate_tasks":oracle.duplicate_tasks,
        "provider_calls":0,"external_effects":"not_exercised","worker_os_death":"not_independently_observed_here","acceptance_credit":false});
    write_new(&root,"report.json",&report)?;Ok(report)
}
