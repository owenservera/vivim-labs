//! Native process supervisor. The child receives only a pure computational work
//! package; the parent retains the store and independent oracle.
use super::{model::*,transition::{Action,Actor,Request},persistence::KernelStore};
use crate::Result;
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::{collections::BTreeMap,fs,path::{Path,PathBuf},process::{Child,Command,Stdio},sync::atomic::{AtomicU64,Ordering},time::{Duration,Instant,SystemTime,UNIX_EPOCH},thread};

static NEXT:AtomicU64=AtomicU64::new(0);
fn error(e:impl std::fmt::Display)->String{e.to_string()}
fn wall_ms()->Result<u64>{u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH).map_err(error)?.as_millis()).map_err(error)}
fn request_id()->Result<Id>{Ok(format!("r-{}-{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).map_err(error)?.as_nanos(),NEXT.fetch_add(1,Ordering::Relaxed)))}
fn apply(store:&KernelStore,actor:Actor,action:Action)->Result<Value>{store.apply(Request{id:request_id()?,actor,action})}
fn host(store:&KernelStore,action:Action)->Result<Value>{apply(store,Actor::Host,action)}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Work {pub class:Class,pub operators:BTreeMap<Id,OperatorContract>,pub input:Value}
impl Work{
    pub fn from_world(w:&World,class:&str,input:Value)->Result<Self>{
        let c=w.classes.get(class).ok_or("class missing")?.clone();
        let mut operators=BTreeMap::new();
        for n in &c.nodes{operators.insert(n.operator.clone(),w.operators.get(&n.operator).ok_or("operator missing")?.clone());}
        Ok(Self{class:c,operators,input})
    }
    pub fn execute(self)->Result<Value>{
        // Parent validates the actual class lineage. Children need only the executable graph.
        let mut w=World::default();w.operators=self.operators;
        let mut c=self.class;c.lineage.clear();let key=version_key(&c.id,c.version)?;
        w.validate_class(&c)?;w.classes.insert(key.clone(),c);w.run_class(&key,&self.input)
    }
}
pub fn compute_file(input:&Path,output:&Path)->Result<()>{
    let bytes=fs::read(input).map_err(error)?;
    ensure(bytes.len()<=16*MAX_VALUE_BYTES,"work package exceeds budget")?;
    let work:Work=serde_json::from_slice(&bytes).map_err(error)?;
    let value=work.execute()?;bounded(&value)?;
    let bytes=serde_json::to_vec(&value).map_err(error)?;
    use std::io::Write;
    let tmp=output.with_extension("tmp");
    let mut f=fs::OpenOptions::new().write(true).create_new(true).open(&tmp).map_err(error)?;
    f.write_all(&bytes).map_err(error)?;f.sync_all().map_err(error)?;drop(f);
    ensure(!output.exists(),"result already exists")?;fs::rename(tmp,output).map_err(error)
}
struct Job {child:Child,task:Id,worker:Id,incarnation:u64,generation:u64,output:PathBuf,started:Instant}
impl Job{
    fn actor(&self)->Actor{Actor::Worker{id:self.worker.clone(),incarnation:self.incarnation}}
}
impl Drop for Job{
    fn drop(&mut self){if self.child.try_wait().ok().flatten().is_none(){let _=self.child.kill();let _=self.child.wait();}}
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct RunConfig {pub duration_ms:u64,pub lease_ms:u64,pub poll_ms:u64,pub max_jobs:u64}
impl RunConfig{
    pub fn validate(&self)->Result<()>{
        ensure((1..=259_200_000).contains(&self.duration_ms)&&self.lease_ms>=100&&self.lease_ms<=3_600_000&&self.poll_ms>=10&&self.poll_ms<=self.lease_ms/2&&self.max_jobs>0,"invalid supervisor limits")
    }
}
fn suitable(w:&World,task:&Task,class:&str)->Option<Id>{
    w.agents.values().filter(|a|a.goal==task.spec.goal&&a.class==class&&matches!(a.phase,AgentPhase::Dormant|AgentPhase::Recovering|AgentPhase::WaitingDependency))
        .filter(|a|w.contexts.get(&a.context).is_some_and(|c|w.assemble(&task.spec,c).is_ok()))
        .min_by_key(|a|(a.incarnations.len(),a.id.clone())).map(|a|a.id.clone())
}
fn genesis(store:&KernelStore,w:&World,task:&Task,class:&str)->Result<Id>{
    let context=w.contexts.iter().find(|(_,c)|w.assemble(&task.spec,c).is_ok()).map(|(k,_)|k.clone());
    let context=match context{Some(c)=>c,None=>{
        let key="native-empty@1".to_string();
        let c=Context{id:"native-empty".into(),version:1,entries:BTreeMap::new(),artifacts:BTreeMap::new(),byte_budget:MAX_VALUE_BYTES};
        w.assemble(&task.spec,&c)?;
        if !w.contexts.contains_key(&key){host(store,Action::Context{context:c})?;}key
    }};
    let worker=format!("native-{}",request_id()?);
    host(store,Action::Agent{id:worker.clone(),goal:task.spec.goal.clone(),parent:None,class:class.into(),context})?;
    Ok(worker)
}
/// A bounded, restartable host loop. Every child is owned by this invocation;
/// old workers have no store access and late results are fenced by lease generation.
pub fn supervise(store:&KernelStore,workspace:&Path,config:RunConfig)->Result<Value>{
    config.validate()?;
    fs::create_dir_all(workspace).map_err(error)?;
    let workspace=workspace.canonicalize().map_err(error)?;
    let exe=std::env::current_exe().map_err(error)?;
    let started=Instant::now();let mut jobs:Vec<Job>=vec![];let mut launched=0u64;let mut completed=0u64;let mut failures=0u64;
    let before=store.inspect()?;
    let base_clock=before.clock_ms.max(wall_ms()?);
    let mut stop_reason="duration_limit";
    while started.elapsed()<Duration::from_millis(config.duration_ms){
        if workspace.join("STOP").exists(){stop_reason="operator_stop";break;}
        let now=base_clock.saturating_add(u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX)).max(store.inspect()?.clock_ms);
        host(store,Action::Tick{now_ms:now})?;
        let mut i=0;
        while i<jobs.len(){
            let status=jobs[i].child.try_wait().map_err(error)?;
            let w=store.inspect()?;
            let valid=w.tasks.get(&jobs[i].task).is_some_and(|t|t.phase==TaskPhase::Running&&t.lease.as_ref().is_some_and(|l|l.worker==jobs[i].worker&&l.generation==jobs[i].generation&&l.incarnation==jobs[i].incarnation));
            if !valid||jobs[i].started.elapsed()>=Duration::from_millis(config.lease_ms){
                let job=jobs.swap_remove(i);
                if valid{let _=apply(store,job.actor(),Action::Fail{task:job.task.clone(),generation:job.generation,reason:"native worker timeout".into()});}
                failures+=1;drop(job);continue;
            }
            let Some(status)=status else{i+=1;continue;};
            let job=jobs.swap_remove(i);
            let output=if status.success(){
                fs::read(&job.output).map_err(error).and_then(|bytes|{
                    ensure(bytes.len()<=MAX_VALUE_BYTES,"result too large")?;
                    serde_json::from_slice::<Value>(&bytes).map_err(error)
                })
            }else{Err(format!("native worker exit: {status}"))};
            match output{
                Ok(value)=>{
                    match apply(store,job.actor(),Action::Submit{task:job.task.clone(),generation:job.generation,output:value}){
                        Ok(_)=>{let response=host(store,Action::Verify{task:job.task.clone()})?;if response["accepted"]==true{completed+=1;}else{failures+=1;}}
                        Err(e)=>{let _=apply(store,job.actor(),Action::Fail{task:job.task.clone(),generation:job.generation,reason:e});failures+=1;}
                    }
                }
                Err(e)=>{let _=apply(store,job.actor(),Action::Fail{task:job.task.clone(),generation:job.generation,reason:e});failures+=1;}
            }
        }
        // Recover durable submissions after controller replacement.
        let w=store.inspect()?;
        for t in w.tasks.values().filter(|t|t.phase==TaskPhase::Submitted){
            if host(store,Action::Verify{task:t.spec.id.clone()})?["accepted"]==true{completed+=1;}else{failures+=1;}
        }
        let mut progress=false;
        while launched<config.max_jobs{
            let w=store.inspect()?;
            if w.running()>=w.slots{break;}
            let mut choice=None;
            for key in w.ready(){
                let t=&w.tasks[&key];
                if let Some(class)=w.match_class(&t.spec){
                    if suitable(&w,t,&class).is_some()||w.contexts.values().any(|c|w.assemble(&t.spec,c).is_ok())||!t.spec.inputs.values().any(|p|matches!(&p.binding,Binding::Context{..})){
                        choice=Some((key,class));break;
                    }
                }
            }
            let Some((task,class))=choice else{break;};
            let worker=match suitable(&w,&w.tasks[&task],&class){Some(a)=>a,None=>genesis(store,&w,&w.tasks[&task],&class)?};
            let current=store.inspect()?;
            if current.current_incarnation(&worker).is_err(){host(store,Action::Activate{agent:worker.clone(),executor:"native-pure-process".into()})?;}
            let claim=host(store,Action::Claim{task:task.clone(),agent:worker.clone(),ttl_ms:config.lease_ms})?;
            let lease:Lease=serde_json::from_value(claim["lease"].clone()).map_err(error)?;
            let actor=Actor::Worker{id:worker.clone(),incarnation:lease.incarnation};
            let launch=(||->Result<Job>{
                let snapshot=store.inspect()?;
                let work=Work::from_world(&snapshot,&class,claim["input"].clone())?;
                let path=store.project_agent(&worker,&workspace)?;
                let attempt=path.join("incarnations").join(format!("{}-{}-{}",task,lease.incarnation,lease.generation));
                fs::create_dir(&attempt).map_err(error)?;
                let input=attempt.join("work.json");let output=attempt.join("output.json");
                fs::write(&input,serde_json::to_vec(&work).map_err(error)?).map_err(error)?;
                let child=Command::new(&exe).arg("kernel-compute").arg(&input).arg(&output).current_dir(&attempt).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn().map_err(error)?;
                Ok(Job{child,task:task.clone(),worker:worker.clone(),incarnation:lease.incarnation,generation:lease.generation,output,started:Instant::now()})
            })();
            launched+=1;progress=true;
            match launch{Ok(job)=>jobs.push(job),Err(reason)=>{apply(store,actor,Action::Fail{task,generation:lease.generation,reason})?;failures+=1;}}
        }
        if jobs.is_empty()&&!progress{
            let w=store.inspect()?;
            if w.running()==0 {stop_reason=if launched>=config.max_jobs{"job_limit"}else{"idle"};break;}
        }
        thread::sleep(Duration::from_millis(config.poll_ms));
    }
    // Cancellation of this invocation is not cancellation of durable goals.
    // Fence its jobs and release their leases before terminating owned child processes.
    for job in jobs{
        let _=apply(store,job.actor(),Action::Fail{task:job.task.clone(),generation:job.generation,reason:"supervisor stopped".into()});
        drop(job);
    }
    let after=store.inspect()?;
    Ok(json!({"reason":stop_reason,"launched":launched,"completed":completed,"failures":failures,"elapsed_ms":started.elapsed().as_millis() as u64,"metrics_before":before.metrics,"metrics_after":after.metrics}))
}
