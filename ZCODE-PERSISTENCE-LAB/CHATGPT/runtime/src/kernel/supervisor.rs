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
fn metered_host(store:&KernelStore,operation:&str,action:Action)->Result<Value>{
    let request=request_id()?;let start=Instant::now();
    let result=store.apply(Request{id:request.clone(),actor:Actor::Host,action});
    let receipt=super::resource::HostCost{request,operation:operation.into(),elapsed_ms:u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX),result_digest:result.as_ref().ok().map(crate::digest)};
    host(store,Action::RecordHostCost{receipt})?;result
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Work {pub class:Class,pub operators:BTreeMap<Id,OperatorContract>,pub input:Value,#[serde(default)] pub resume:Option<Value>}
impl Work{
    pub fn from_world(w:&World,class:&str,input:Value)->Result<Self>{
        let c=w.classes.get(class).ok_or("class missing")?.clone();
        let mut operators=BTreeMap::new();
        for n in &c.nodes{operators.insert(n.operator.clone(),w.operators.get(&n.operator).ok_or("operator missing")?.clone());}
        Ok(Self{class:c,operators,input,resume:None})
    }
    pub fn execute(self)->Result<Value>{self.execute_with(|_|Ok(()))}
    pub fn execute_with<F:FnMut(Value)->Result<()>>(self,emit:F)->Result<Value>{
        // Parent validates the actual class lineage. Children need only the executable graph.
        let mut w=World::default();w.operators=self.operators;
        let mut c=self.class;c.lineage.clear();let key=version_key(&c.id,c.version)?;
        w.validate_class(&c)?;w.classes.insert(key.clone(),c);w.run_class_resumable(&key,&self.input,self.resume.as_ref(),emit)
    }
}
pub fn watch_parent(){
    // A controller owns the write end of stdin. EOF means that it exited or
    // explicitly closed the pipe. This child executes only pure computations.
    thread::spawn(||{use std::io::Read;let mut byte=[0u8;1];loop{match std::io::stdin().read(&mut byte){Ok(0)|Err(_)=>std::process::exit(75),Ok(_)=>{}}}});
}
pub fn compute_file(input:&Path,output:&Path)->Result<()>{
    let bytes=fs::read(input).map_err(error)?;
    ensure(bytes.len()<=16*MAX_VALUE_BYTES,"work package exceeds budget")?;
    let work:Work=serde_json::from_slice(&bytes).map_err(error)?;
    let progress=output.parent().ok_or("result parent missing")?.join("progress");
    fs::create_dir_all(&progress).map_err(error)?;
    let value=work.execute_with(|checkpoint|{
        let n=checkpoint.get("next_node").and_then(Value::as_u64).ok_or("checkpoint cursor missing")?;
        let dest=progress.join(format!("{n:020}.json"));let tmp=progress.join(format!("{n:020}.tmp"));
        if dest.exists(){return Ok(());}
        use std::io::Write;
        let mut f=fs::OpenOptions::new().create_new(true).write(true).open(&tmp).map_err(error)?;
        f.write_all(&serde_json::to_vec(&checkpoint).map_err(error)?).map_err(error)?;f.sync_all().map_err(error)?;drop(f);
        fs::rename(tmp,dest).map_err(error)
    })?;bounded(&value)?;
    let bytes=serde_json::to_vec(&value).map_err(error)?;
    use std::io::Write;
    let tmp=output.with_extension("tmp");
    let mut f=fs::OpenOptions::new().write(true).create_new(true).open(&tmp).map_err(error)?;
    f.write_all(&bytes).map_err(error)?;f.sync_all().map_err(error)?;drop(f);
    ensure(!output.exists(),"result already exists")?;fs::rename(tmp,output).map_err(error)
}
struct Job {child:Child,task:Id,worker:Id,incarnation:u64,generation:u64,output:PathBuf,started:Instant,last_checkpoint:u64}
impl Job{
    fn resources(&self,store:&KernelStore)->Result<()> {
        let receipt=super::resource::Receipt{task:self.task.clone(),generation:self.generation,process_ref:format!("native-pid-{}",self.child.id()),elapsed_ms:self.started.elapsed().as_millis().min(3_600_000) as u64,counters:super::resource::observe(&self.child,&self.output)};
        host(store,Action::RecordAttemptAccounting{receipt})?;Ok(())
    }
    fn actor(&self)->Actor{Actor::Worker{id:self.worker.clone(),incarnation:self.incarnation}}
    fn checkpoint(&mut self,store:&KernelStore)->Result<()>{
        let Some(parent)=self.output.parent() else{return Ok(())};let path=parent.join("progress");
        if !path.is_dir(){return Ok(());}
        let mut files=vec![];
        for e in fs::read_dir(&path).map_err(error)?{let e=e.map_err(error)?;let name=e.file_name().to_string_lossy().into_owned();if let Some(n)=name.strip_suffix(".json").and_then(|x|x.parse::<u64>().ok()){if n>self.last_checkpoint{files.push((n,e.path()));}}}
        files.sort_by_key(|x|std::cmp::Reverse(x.0));
        for (n,p) in files{if fs::metadata(&p).map_err(error)?.len()>MAX_VALUE_BYTES as u64{continue;}let bytes=fs::read(p).map_err(error)?;let Ok(data)=serde_json::from_slice::<Value>(&bytes) else{continue;};if data.get("next_node").and_then(Value::as_u64)!=Some(n){continue;}
            apply(store,self.actor(),Action::Checkpoint{task:self.task.clone(),generation:self.generation,data})?;self.last_checkpoint=n;break;
        }Ok(())
    }
}
impl Drop for Job{
    fn drop(&mut self){if self.child.try_wait().ok().flatten().is_none(){let _=self.child.kill();let _=self.child.wait();}}
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct RunConfig {pub duration_ms:u64,pub lease_ms:u64,pub poll_ms:u64,pub max_jobs:u64,#[serde(default="default_learning_budget")] pub max_learning_plans:u64,#[serde(default="default_idle_stop")] pub stop_when_idle:bool}
fn default_learning_budget()->u64{16}
fn default_idle_stop()->bool{true}
impl RunConfig{
    pub fn validate(&self)->Result<()>{
        ensure(self.duration_ms>0&&self.lease_ms>=100&&self.lease_ms<=3_600_000&&self.poll_ms>=10&&self.poll_ms<=self.lease_ms/2&&self.max_jobs>0&&self.max_learning_plans<=1024,"invalid supervisor limits")
    }
}
fn suitable(w:&World,task:&Task,class:&str)->Option<Id>{
    w.agents.values().filter(|a|a.goal==task.spec.goal&&a.class==class&&matches!(a.phase,AgentPhase::Dormant|AgentPhase::Recovering)&&a.incarnations.last().is_none_or(|i|i.ended_ms.is_some()||(i.executor=="native-pure-process"&&i.process_ref.is_none())))
        .filter(|a|task.spec.context.as_ref().is_none_or(|c|c==&a.context)&&w.contexts.get(&a.context).is_some_and(|c|w.assemble(&task.spec,c).is_ok()))
        .min_by_key(|a|(a.incarnations.len(),a.id.clone())).map(|a|a.id.clone())
}
fn genesis(store:&KernelStore,w:&World,task:&Task,class:&str)->Result<Id>{
    let context=task.spec.context.clone();
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
    let _controller=store.controller_lock()?;
    fs::create_dir_all(workspace).map_err(error)?;
    let workspace=workspace.canonicalize().map_err(error)?;
    let exe=std::env::current_exe().map_err(error)?;
    let started=Instant::now();let mut jobs:Vec<Job>=vec![];let mut launched=0u64;let mut completed=0u64;let mut failures=0u64;let mut learning_runs=0u64;let mut knowledge_runs=0u64;
    let before=store.inspect()?;
    let base_clock=before.clock_ms.max(wall_ms()?);
    // Exclusive controller lock proves no previous cooperating supervisor owns
    // the store. Pure children are parent-pipe bound; stale results are fenced.
    host(store,Action::Tick{now_ms:base_clock})?;
    if before.agents.values().any(|a|a.incarnations.last().is_some_and(|i|i.ended_ms.is_none()&&i.executor=="native-pure-process")){
        host(store,Action::RecoverNative{reason:"native controller replacement".into()})?;
    }
    let mut stop_reason="duration_limit";
    while started.elapsed()<Duration::from_millis(config.duration_ms){
        if workspace.join("STOP").exists(){stop_reason="operator_stop";break;}
        let state=store.inspect()?;
        let now=base_clock.saturating_add(u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX)).max(state.clock_ms);
        if now>state.clock_ms&&(state.running()>0||!state.ready().is_empty()||!state.ready_learning().is_empty()||state.router.trial_active()){
            host(store,Action::Tick{now_ms:now})?;
        }
        let mut i=0;
        while i<jobs.len(){
            let status=jobs[i].child.try_wait().map_err(error)?;
            let w=store.inspect()?;
            let valid=w.tasks.get(&jobs[i].task).is_some_and(|t|t.phase==TaskPhase::Running&&t.lease.as_ref().is_some_and(|l|l.worker==jobs[i].worker&&l.generation==jobs[i].generation&&l.incarnation==jobs[i].incarnation));
            if valid{jobs[i].checkpoint(store)?;}
            if !valid||jobs[i].started.elapsed()>=Duration::from_millis(config.lease_ms){
                let mut job=jobs.swap_remove(i);
                if job.child.try_wait().map_err(error)?.is_none(){let _=job.child.kill();let _=job.child.wait();}
                job.resources(store)?;
                // Elapsed cost is already committed atomically with the resource receipt.
                if valid{let _=apply(store,job.actor(),Action::Fail{task:job.task.clone(),generation:job.generation,reason:"native worker timeout".into()});}
                failures+=1;drop(job);continue;
            }
            let Some(status)=status else{i+=1;continue;};
            let job=jobs.swap_remove(i);
            job.resources(store)?;
            // Elapsed cost is already committed atomically with the resource receipt.
            let output=if status.success(){
                fs::read(&job.output).map_err(error).and_then(|bytes|{
                    ensure(bytes.len()<=MAX_VALUE_BYTES,"result too large")?;
                    serde_json::from_slice::<Value>(&bytes).map_err(error)
                })
            }else{Err(format!("native worker exit: {status}"))};
            match output{
                Ok(value)=>{
                    match apply(store,job.actor(),Action::Submit{task:job.task.clone(),generation:job.generation,output:value}){
                        Ok(_)=>{let response=metered_host(store,"verify",Action::Verify{task:job.task.clone()})?;if response["accepted"]==true{completed+=1;}else{failures+=1;}}
                        Err(e)=>{let _=apply(store,job.actor(),Action::Fail{task:job.task.clone(),generation:job.generation,reason:e});failures+=1;}
                    }
                }
                Err(e)=>{let _=apply(store,job.actor(),Action::Fail{task:job.task.clone(),generation:job.generation,reason:e});failures+=1;}
            }
            store.project_host_agent(&job.worker)?;
        }
        // Recover durable submissions after controller replacement.
        let w=store.inspect()?;
        for t in w.tasks.values().filter(|t|t.phase==TaskPhase::Submitted){
            if metered_host(store,"verify",Action::Verify{task:t.spec.id.clone()})?["accepted"]==true{completed+=1;}else{failures+=1;}
        }
        let mut progress=false;
        if learning_runs.saturating_add(knowledge_runs)<config.max_learning_plans{
            if let Some(action)=store.inspect()?.router.trial_auto_action(){
                metered_host(store,"trial",action)?;learning_runs+=1;progress=true;
            }
        }
        while learning_runs.saturating_add(knowledge_runs)<config.max_learning_plans{
            let w=store.inspect()?;
            let Some(plan)=w.ready_learning().into_iter().next() else{break;};
            metered_host(store,"learning",Action::LearnPlan{plan})?;
            learning_runs+=1;progress=true;
        }
        while learning_runs.saturating_add(knowledge_runs)<config.max_learning_plans{
            let w=store.inspect()?;
            let Some((rule,payload))=w.ready_knowledge().into_iter().next() else{break;};
            metered_host(store,"knowledge",Action::AssimilateRecurring{rule,payload})?;
            knowledge_runs+=1;progress=true;
        }
        while learning_runs.saturating_add(knowledge_runs)<config.max_learning_plans{
            let w=store.inspect()?;let Some(family)=w.ready_recipe_scans().into_iter().next() else{break;};
            metered_host(store,"recipe",Action::DiscoverRecipes{family})?;knowledge_runs+=1;progress=true;
        }
        while learning_runs.saturating_add(knowledge_runs)<config.max_learning_plans{
            let w=store.inspect()?;
            let Some((task,repair))=w.ready().into_iter().find_map(|task|w.learned_repair_for(&task).map(|r|(task,r))) else{break;};
            metered_host(store,"repair",Action::ApplyLearnedRepair{task,repair})?;knowledge_runs+=1;progress=true;
        }
        if launched<config.max_jobs&&store.inspect()?.routing_cycle_needed(){
            let routing=metered_host(store,"routing",Action::RoutingCycle{limit:(config.max_jobs-launched).min(4) as usize})?;
            let count=routing["prepared"].as_array().map_or(0,|a|a.len()) as u64;
            launched=launched.saturating_add(count);progress|=count>0;
        }
        while launched<config.max_jobs{
            let w=store.inspect()?;
            if w.running()>=w.slots{break;}
            let mut choice=None;
            for key in w.ready(){
                if !w.native_allowed(&key){continue;}
                let t=&w.tasks[&key];
                if let Some(class)=w.match_class(&t.spec){
                    let empty=Context{id:"native-empty".into(),version:1,entries:BTreeMap::new(),artifacts:BTreeMap::new(),byte_budget:MAX_VALUE_BYTES};
                    if suitable(&w,t,&class).is_some()||match &t.spec.context{Some(k)=>w.contexts.get(k).is_some_and(|c|w.assemble(&t.spec,c).is_ok()),None=>w.assemble(&t.spec,&empty).is_ok()}{
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
                let mut work=Work::from_world(&snapshot,&class,claim["input"].clone())?;
                work.resume=claim.get("checkpoint").filter(|v|!v.is_null()).cloned();
                let last_checkpoint=work.resume.as_ref().and_then(|v|v.get("next_node")).and_then(Value::as_u64).unwrap_or(0);
                store.project_host_agent(&worker)?;
                let path=store.project_agent(&worker,&workspace)?;
                let attempt=path.join("incarnations").join(format!("{}-{}-{}",task,lease.incarnation,lease.generation));
                fs::create_dir(&attempt).map_err(error)?;
                let input=attempt.join("work.json");let output=attempt.join("output.json");
                fs::write(&input,serde_json::to_vec(&work).map_err(error)?).map_err(error)?;
                let child=Command::new(&exe).arg("kernel-compute").arg(&input).arg(&output).arg("--watch-parent").current_dir(&attempt).stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::null()).spawn().map_err(error)?;
                let job=Job{child,task:task.clone(),worker:worker.clone(),incarnation:lease.incarnation,generation:lease.generation,output,started:Instant::now(),last_checkpoint};
                host(store,Action::BindExecutor{agent:worker.clone(),incarnation:lease.incarnation,process_ref:format!("native-pid-{}",job.child.id())})?;
                Ok(job)
            })();
            launched+=1;progress=true;
            match launch{Ok(job)=>jobs.push(job),Err(reason)=>{apply(store,actor,Action::Fail{task,generation:lease.generation,reason})?;failures+=1;}}
        }
        if jobs.is_empty()&&!progress{
            let w=store.inspect()?;
            if w.running()==0&&(!w.router.trial_active()||launched>=config.max_jobs)&&(config.stop_when_idle||launched>=config.max_jobs){stop_reason=if launched>=config.max_jobs{"job_limit"}else{"idle"};break;}
        }
        thread::sleep(Duration::from_millis(config.poll_ms));
    }
    // Cancellation of this invocation is not cancellation of durable goals.
    // Fence its jobs and release their leases before terminating owned child processes.
    for mut job in jobs{
        if job.child.try_wait().map_err(error)?.is_none(){let _=job.child.kill();let _=job.child.wait();}
        job.resources(store)?;
        // Elapsed cost is already committed atomically with the resource receipt.
        let _=apply(store,job.actor(),Action::Fail{task:job.task.clone(),generation:job.generation,reason:"supervisor stopped".into()});
        drop(job);
    }
    let after=store.inspect()?;
    Ok(json!({"reason":stop_reason,"launched":launched,"completed":completed,"failures":failures,"learning_runs":learning_runs,"knowledge_runs":knowledge_runs,"elapsed_ms":started.elapsed().as_millis() as u64,"metrics_before":before.metrics,"metrics_after":after.metrics}))
}
