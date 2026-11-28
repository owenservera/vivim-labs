//! Provider-neutral command admission and deterministic lifecycle transitions.
use super::model::*;
use crate::{digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::collections::BTreeSet;

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(tag="kind",rename_all="snake_case")]
pub enum Actor {Host,Worker{id:Id,incarnation:u64}}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(tag="action",rename_all="snake_case")]
pub enum Action {
    Goal{id:Id,parent:Option<Id>,objective:String,claim_budget:u64,predicate:Option<Oracle>},
    ReviseGoal{goal:Id,objective:String,predicate:Option<Oracle>},
    Operator{contract:OperatorContract},Class{class:Class},Context{context:Context},
    Agent{id:Id,goal:Id,parent:Option<Id>,class:Id,context:Id},
    Activate{agent:Id,executor:String},BindExecutor{agent:Id,incarnation:u64,process_ref:String},
    MigrateAgent{agent:Id,class:Id,context:Id},Retire{agent:Id,reason:String},
    Task{spec:TaskSpec},Refine{spec:TaskSpec},RebindTask{task:Id,class:Option<Id>,context:Option<Id>},RefineGraph{goal:Id,additions:Vec<TaskSpec>,revisions:Vec<TaskSpec>},Seal{goal:Id},Slots{count:usize},
    Claim{task:Id,agent:Id,ttl_ms:u64},Renew{task:Id,generation:u64,ttl_ms:u64},
    Checkpoint{task:Id,generation:u64,data:Value},
    Submit{task:Id,generation:u64,output:Value},Fail{task:Id,generation:u64,reason:String},
    Verify{task:Id},Tick{now_ms:u64},Send{id:Id,recipient:Id,body:Value},Ack{id:Id},
    WaitMessage{agent:Id},WaitDependency{agent:Id,task:Id},Cancel{goal:Id},
    AdmitCapability{capability:Capability},DisableCapability{capability:Id},
    ConfigureLearning{plan:LearningPlan},LearnPlan{plan:Id},PublishKnowledge{knowledge:Knowledge},DisableKnowledge{knowledge:Id},
    RecordCost{task:Id,generation:u64,elapsed_ms:u64},SetPolicy{policy:SchedulingPolicy},
    PrepareEffect{task:Id,generation:u64,key:Id,payload:Value,oracle:Oracle},
    EffectUnknown{effect:Id,reason:String},ResolveEffect{effect:Id,outcome:EffectPhase,receipt:Value},
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Request {pub id:Id,pub actor:Actor,pub action:Action}
impl Actor {
    fn host(&self)->Result<()>{ensure(self==&Self::Host,"host authority required")}
    fn worker(&self)->Result<(&str,u64)>{match self{Self::Worker{id,incarnation}=>Ok((id,*incarnation)),_=>Err("worker required".into())}}
}
impl World {
    fn open_goal(&self,key:&str)->Result<&Goal>{
        let g=self.goals.get(key).ok_or("goal missing")?;ensure(g.phase==GoalPhase::Open,"goal closed")?;Ok(g)
    }
    pub fn current_incarnation(&self,key:&str)->Result<u64>{
        let a=self.agents.get(key).ok_or("agent missing")?;
        let i=a.incarnations.last().ok_or("agent not activated")?;
        ensure(i.ended_ms.is_none()&&a.phase!=AgentPhase::Archived,"incarnation ended")?;Ok(i.number)
    }
    fn worker_identity<'a>(&self,actor:&'a Actor)->Result<(&'a str,u64)>{
        let (who,n)=actor.worker()?;ensure(self.current_incarnation(who)?==n,"stale incarnation")?;Ok((who,n))
    }
    fn check_lease(&self,actor:&Actor,task:&str,generation:u64)->Result<&Task>{
        let (who,n)=self.worker_identity(actor)?;
        let t=self.tasks.get(task).ok_or("task missing")?;self.open_goal(&t.spec.goal)?;
        let l=t.lease.as_ref().ok_or("lease missing")?;
        ensure(t.phase==TaskPhase::Running&&l.worker==who&&l.incarnation==n&&l.generation==generation&&self.clock_ms<l.deadline_ms,"stale or expired lease")?;Ok(t)
    }
    pub fn running(&self)->usize{self.tasks.values().filter(|t|t.phase==TaskPhase::Running).count()}
    pub fn ready(&self)->Vec<Id>{
        let mut v:Vec<_>=self.tasks.values().filter(|t|t.phase==TaskPhase::Pending&&t.attempts<t.spec.max_attempts
            &&self.goals.get(&t.spec.goal).is_some_and(|g|g.phase==GoalPhase::Open&&g.claims<g.claim_budget)
            &&!self.has_uncertain_effect(&t.spec.id)&&!self.has_abandoned_effect(&t.spec.id)
            &&t.spec.dependencies.iter().all(|d|self.tasks.get(d).is_some_and(|x|x.phase==TaskPhase::Verified))).collect();
        let scores=self.schedule_scores();
        v.sort_by_key(|t|(std::cmp::Reverse(scores.get(&t.spec.id).copied().unwrap_or(0)),t.spec.id.clone()));
        v.into_iter().map(|t|t.spec.id.clone()).collect()
    }
    fn release(&mut self,task:&str,reason:&str)->Result<()>{
        self.mark_prepared_unknown(task,reason);
        let uncertain=self.has_uncertain_effect(task);
        let t=self.tasks.get_mut(task).ok_or("task missing")?;
        let was_running=t.phase==TaskPhase::Running;let worker=t.lease.take().map(|l|l.worker);t.proposal=None;t.reason=Some(reason.into());
        t.phase=if uncertain{TaskPhase::Reconciling}else if t.attempts>=t.spec.max_attempts{TaskPhase::Exhausted}else{TaskPhase::Pending};
        if let Some(w)=worker.filter(|_|was_running){if let Some(a)=self.agents.get_mut(&w){
            if a.phase!=AgentPhase::Archived{a.phase=AgentPhase::Recovering;}
        }}
        self.metrics.failures=self.metrics.failures.saturating_add(1);Ok(())
    }
    fn end_agent(&mut self,key:&str,reason:&str,archive:bool)->Result<()>{
        let a=self.agents.get_mut(key).ok_or("agent missing")?;
        if let Some(i)=a.incarnations.last_mut(){if i.ended_ms.is_none(){i.ended_ms=Some(self.clock_ms);i.reason=Some(reason.into());}}
        a.phase=if archive{AgentPhase::Archived}else{AgentPhase::Recovering};
        let tasks:Vec<_>=self.tasks.values().filter(|t|t.phase==TaskPhase::Running&&t.lease.as_ref().is_some_and(|l|l.worker==key)).map(|t|t.spec.id.clone()).collect();
        for t in tasks{self.release(&t,reason)?;}Ok(())
    }
    fn settle(&mut self)->Result<()>{
        // Fixed point propagates terminal failures through task and goal dependencies.
        loop {
            let mut changed=false;
            let blocked:Vec<_>=self.tasks.values().filter(|t|t.phase==TaskPhase::Pending&&t.spec.dependencies.iter().any(|d|self.tasks.get(d).is_some_and(|p|matches!(p.phase,TaskPhase::Blocked|TaskPhase::Exhausted|TaskPhase::Cancelled)))).map(|t|t.spec.id.clone()).collect();
            for key in blocked{let t=self.tasks.get_mut(&key).unwrap();t.phase=TaskPhase::Blocked;t.reason=Some("predecessor terminal".into());changed=true;}
            let mut outcomes=Vec::new();
            for g in self.goals.values().filter(|g|g.phase==GoalPhase::Open){
                let tasks:Vec<_>=self.tasks.values().filter(|t|t.spec.goal==g.id).collect();
                let children:Vec<_>=self.goals.values().filter(|x|x.parent.as_ref()==Some(&g.id)).collect();
                let active=tasks.iter().any(|t|matches!(t.phase,TaskPhase::Running|TaskPhase::Submitted|TaskPhase::Reconciling));
                let failed=tasks.iter().any(|t|matches!(t.phase,TaskPhase::Exhausted|TaskPhase::Blocked|TaskPhase::Cancelled))||children.iter().any(|c|matches!(c.phase,GoalPhase::Exhausted|GoalPhase::Blocked|GoalPhase::Cancelled));
                if failed&&!active{outcomes.push((g.id.clone(),GoalPhase::Blocked));continue;}
                if g.claims>=g.claim_budget&&!active&&tasks.iter().any(|t|t.phase==TaskPhase::Pending){outcomes.push((g.id.clone(),GoalPhase::Exhausted));continue;}
                if g.sealed&&(!tasks.is_empty()||!children.is_empty())&&tasks.iter().all(|t|t.phase==TaskPhase::Verified)&&children.iter().all(|c|c.phase==GoalPhase::Complete){
                    let mut values=serde_json::Map::new();
                    for t in tasks{values.insert(t.spec.id.clone(),self.artifact_value(t.artifact.as_deref().ok_or("artifact missing")?)?.clone());}
                    let accepted=match &g.predicate{Some(o)=>o.evaluate(&Value::Null,&Value::Object(values)).unwrap_or(false),None=>true};
                    outcomes.push((g.id.clone(),if accepted{GoalPhase::Complete}else{GoalPhase::Blocked}));
                }
            }
            for (key,phase) in outcomes{
                self.goals.get_mut(&key).unwrap().phase=phase.clone();
                for t in self.tasks.values_mut().filter(|t|t.spec.goal==key&&t.phase==TaskPhase::Pending){
                    t.phase=if phase==GoalPhase::Exhausted{TaskPhase::Exhausted}else{TaskPhase::Blocked};
                    t.reason=Some("goal terminal".into());
                }
                let agents:Vec<_>=self.agents.values().filter(|a|a.goal==key&&a.phase!=AgentPhase::Archived).map(|a|a.id.clone()).collect();
                for a in agents{self.end_agent(&a,"goal terminal",true)?;}
                changed=true;
            }
            if !changed{break;}
        }
        let ready:BTreeSet<_>=self.ready().into_iter().collect();
        for a in self.agents.values_mut().filter(|a|a.phase==AgentPhase::WaitingDependency){
            if a.wait_task.as_ref().is_some_and(|t|ready.contains(t)||self.tasks.get(t).is_some_and(|x|matches!(x.phase,TaskPhase::Verified|TaskPhase::Blocked|TaskPhase::Exhausted|TaskPhase::Cancelled))){a.phase=AgentPhase::Dormant;a.wait_task=None;}
        }
        Ok(())
    }
    pub fn checkpoint_value(&self,task:&str)->Option<&Value>{
        let t=self.tasks.get(task)?;
        let input=digest(t.input.as_ref()?);
        let history=self.checkpoints.get(task)?;
        let mut previous=None;
        let mut valid=vec![];
        for (i,c) in history.iter().enumerate(){
            if c.version!=i as u64+1||c.previous.as_ref()!=previous{break;}
            previous=Some(&c.payload);
            if c.input==input&&c.task_version==t.version&&c.class==t.assigned_class&&c.context==t.assigned_context&&c.generation<=t.generation{
                if let Ok(v)=self.payload(&c.payload){valid.push(v);}
            }
        }
        valid.pop()
    }
    pub fn transition(&mut self,actor:&Actor,action:Action)->Result<Value>{
        // Public transitions are atomic even for callers not using the durable store.
        let mut next=self.clone();let output=next.apply_inner(actor,action)?;next.settle()?;
        next.sequence=next.sequence.checked_add(1).ok_or("sequence overflow")?;*self=next;Ok(output)
    }
    fn apply_inner(&mut self,actor:&Actor,action:Action)->Result<Value>{
        use Action::*;
        match action{
            Goal{id:key,parent,objective,claim_budget,predicate}=>{
                actor.host()?;id(&key)?;ensure(!self.goals.contains_key(&key)&&!objective.trim().is_empty()&&claim_budget>0,"invalid goal")?;
                if let Some(p)=&parent{ensure(!self.open_goal(p)?.sealed,"parent sealed")?;}
                if let Some(o)=&predicate{o.validate(0)?;}
                self.goals.insert(key.clone(),super::model::Goal{id:key,parent,objective,version:1,predicate,sealed:false,phase:GoalPhase::Open,claim_budget,claims:0});
            }
            ReviseGoal{goal,objective,predicate}=>{
                actor.host()?;let g=self.open_goal(&goal)?;ensure(!g.sealed&&!objective.trim().is_empty(),"sealed or empty goal")?;
                if let Some(o)=&predicate{o.validate(0)?;}
                let prior=self.goals[&goal].clone();self.goal_versions.entry(goal.clone()).or_default().push(prior);
                let g=self.goals.get_mut(&goal).unwrap();g.version=g.version.checked_add(1).ok_or("version overflow")?;g.objective=objective;g.predicate=predicate;
            }
            Operator{contract}=>{
                actor.host()?;let key=version_key(&contract.id,contract.version)?;
                ensure(!self.operators.contains_key(&key),"operator version exists")?;self.operators.insert(key,contract);
            }
            Class{class}=>{
                actor.host()?;let key=version_key(&class.id,class.version)?;
                ensure(!self.classes.contains_key(&key),"class version exists")?;self.validate_class(&class)?;self.classes.insert(key,class);
            }
            Context{context}=>{
                actor.host()?;let key=version_key(&context.id,context.version)?;
                ensure(!self.contexts.contains_key(&key)&&context.byte_budget>0&&context.byte_budget<=MAX_VALUE_BYTES,"invalid context")?;
                for (k,v) in &context.entries{id(k)?;bounded(v)?;ensure(!context.artifacts.contains_key(k),"ambiguous context key")?;}
                for (k,a) in &context.artifacts{id(k)?;self.artifact_value(a)?;}
                self.contexts.insert(key,context);
            }
            Agent{id:key,goal,parent,class,context}=>{
                actor.host()?;id(&key)?;self.open_goal(&goal)?;
                ensure(!self.agents.contains_key(&key)&&self.classes.contains_key(&class)&&self.contexts.contains_key(&context),"invalid agent")?;
                if let Some(p)=&parent{ensure(self.agents.get(p).is_some_and(|a|a.goal==goal),"parent missing or out of goal")?;}
                self.agents.insert(key.clone(),super::model::Agent{id:key,goal,parent,class,context,phase:AgentPhase::Dormant,incarnations:vec![],wait_task:None});
            }
            Activate{agent,executor}=>{
                actor.host()?;let a=self.agents.get(&agent).ok_or("agent missing")?;self.open_goal(&a.goal)?;
                ensure(a.phase!=AgentPhase::Archived&&a.incarnations.last().is_none_or(|i|i.ended_ms.is_some())&&!executor.trim().is_empty(),"cannot activate")?;
                let a=self.agents.get_mut(&agent).unwrap();let n=a.incarnations.last().map_or(Ok(1),|i|i.number.checked_add(1).ok_or("incarnation overflow"))?;
                a.incarnations.push(Incarnation{number:n,executor,started_ms:self.clock_ms,ended_ms:None,reason:None,process_ref:None});a.phase=AgentPhase::Dormant;
                return Ok(json!({"incarnation":n}));
            }
            BindExecutor{agent,incarnation,process_ref}=>{
                actor.host()?;ensure(self.current_incarnation(&agent)?==incarnation&&!process_ref.trim().is_empty(),"invalid executor binding")?;
                let i=self.agents.get_mut(&agent).unwrap().incarnations.last_mut().unwrap();
                ensure(i.process_ref.is_none(),"executor already bound")?;i.process_ref=Some(process_ref);
            }
            MigrateAgent{agent,class,context}=>{
                actor.host()?;let a=self.agents.get(&agent).ok_or("agent missing")?;self.open_goal(&a.goal)?;
                ensure(!matches!(a.phase,AgentPhase::Running|AgentPhase::Archived),"cannot migrate agent")?;
                let c=self.classes.get(&class).ok_or("class missing")?;
                ensure(class==a.class||c.lineage.contains(&a.class),"migration requires declared class lineage")?;
                ensure(self.contexts.contains_key(&context),"context missing")?;
                self.end_agent(&agent,"class/context migration",false)?;
                let a=self.agents.get_mut(&agent).unwrap();a.class=class;a.context=context;a.wait_task=None;
            }
            Retire{agent,reason}=>{actor.host()?;self.end_agent(&agent,&reason,true)?;}
            Task{spec}=>{
                actor.host()?;ensure(!self.tasks.contains_key(&spec.id),"task exists")?;self.validate_task(&spec,false)?;
                self.tasks.insert(spec.id.clone(),super::model::Task{spec,version:1,phase:TaskPhase::Pending,attempts:0,generation:0,lease:None,input:None,proposal:None,artifact:None,reason:None,assigned_class:None,assigned_context:None});
            }
            Refine{spec}=>{
                actor.host()?;let t=self.tasks.get(&spec.id).ok_or("task missing")?;
                ensure(t.phase==TaskPhase::Pending&&t.attempts==0&&t.spec.goal==spec.goal,"only unstarted task may be refined")?;
                ensure(t.spec.output==spec.output,"refinement changes downstream output contract")?;self.validate_task(&spec,true)?;
                let prior=self.tasks[&spec.id].spec.clone();self.task_versions.entry(spec.id.clone()).or_default().push(prior);
                let t=self.tasks.get_mut(&spec.id).unwrap();t.version=t.version.checked_add(1).ok_or("version overflow")?;t.spec=spec;
            }
            RebindTask{task,class,context}=>{
                actor.host()?;let t=self.tasks.get(&task).ok_or("task missing")?;
                ensure(t.phase==TaskPhase::Pending&&!self.has_uncertain_effect(&task),"only pending reconciled tasks may be rebound")?;
                let mut spec=t.spec.clone();spec.class=class;spec.context=context;
                self.validate_task(&spec,true)?;
                let prior=self.tasks[&task].spec.clone();self.task_versions.entry(task.clone()).or_default().push(prior);
                let t=self.tasks.get_mut(&task).unwrap();t.version=t.version.checked_add(1).ok_or("version overflow")?;
                t.spec=spec;t.input=None;t.assigned_class=None;t.assigned_context=None;
            }
            RefineGraph{goal,additions,revisions}=>{
                actor.host()?;self.open_goal(&goal)?;
                ensure(additions.len()+revisions.len()<=MAX_GRAPH_NODES,"graph refinement budget")?;
                for spec in additions {
                    ensure(spec.goal==goal&&!self.tasks.contains_key(&spec.id),"invalid graph addition")?;
                    self.validate_task(&spec,true)?;
                    self.tasks.insert(spec.id.clone(),super::model::Task{spec,version:1,phase:TaskPhase::Pending,attempts:0,generation:0,lease:None,input:None,proposal:None,artifact:None,reason:None,assigned_class:None,assigned_context:None});
                }
                for spec in revisions {
                    ensure(spec.goal==goal,"cross-goal refinement")?;
                    self.apply_inner(actor,Refine{spec})?;
                }
            }
            Seal{goal}=>{
                actor.host()?;self.open_goal(&goal)?;
                ensure(self.tasks.values().any(|t|t.spec.goal==goal)||self.goals.values().any(|g|g.parent.as_ref()==Some(&goal)),"empty goal")?;
                self.goals.get_mut(&goal).unwrap().sealed=true;
            }
            Slots{count}=>{actor.host()?;ensure((1..=1024).contains(&count)&&count>=self.running(),"invalid slot count")?;self.slots=count;}
            Claim{task,agent,ttl_ms}=>{
                actor.host()?;ensure(ttl_ms>0&&self.running()<self.slots&&self.ready().contains(&task),"no ready slot")?;
                let t=&self.tasks[&task];let a=self.agents.get(&agent).ok_or("agent missing")?;
                ensure(a.goal==t.spec.goal&&matches!(a.phase,AgentPhase::Dormant|AgentPhase::Recovering),"agent not eligible")?;
                let incarnation=self.current_incarnation(&agent)?;
                if let Some(class)=self.match_class(&t.spec){ensure(class==a.class,"class not eligible")?;}
                else{ensure(t.spec.class.is_none(),"explicit class disabled")?;}
                if let Some(context)=&t.spec.context{ensure(context==&a.context,"context not authorized for task")?;}
                let contract=self.classes.get(&a.class).ok_or("agent class missing")?;
                ensure(contract.input.admits(&Self::input_shape(&t.spec))&&t.spec.output.admits(&contract.output),"agent class contract mismatch")?;
                let assigned_class=a.class.clone();let assigned_context=a.context.clone();
                let c=self.contexts.get(&a.context).ok_or("context missing")?;
                let input=self.assemble(&t.spec,c)?;
                let deadline_ms=self.clock_ms.checked_add(ttl_ms).ok_or("deadline overflow")?;
                let t=self.tasks.get_mut(&task).unwrap();t.attempts+=1;t.generation=t.generation.checked_add(1).ok_or("generation overflow")?;
                let l=Lease{worker:agent.clone(),incarnation,generation:t.generation,deadline_ms};
                t.lease=Some(l.clone());t.input=Some(input.clone());t.phase=TaskPhase::Running;t.reason=None;t.assigned_class=Some(assigned_class);t.assigned_context=Some(assigned_context);
                self.goals.get_mut(&t.spec.goal).unwrap().claims+=1;self.agents.get_mut(&agent).unwrap().phase=AgentPhase::Running;
                self.metrics.claims=self.metrics.claims.saturating_add(1);
                return Ok(json!({"lease":l,"input":input,"checkpoint":self.checkpoint_value(&task)}));
            }
            Renew{task,generation,ttl_ms}=>{
                self.check_lease(actor,&task,generation)?;ensure(ttl_ms>0,"invalid ttl")?;
                self.tasks.get_mut(&task).unwrap().lease.as_mut().unwrap().deadline_ms=self.clock_ms.checked_add(ttl_ms).ok_or("deadline overflow")?;
            }
            Checkpoint{task,generation,data}=>{
                let t=self.check_lease(actor,&task,generation)?;let l=t.lease.clone().unwrap();let input=digest(t.input.as_ref().ok_or("input missing")?);
                let task_version=t.version;let class=t.assigned_class.clone();let context=t.assigned_context.clone();
                let payload=self.put_payload(data)?;let history=self.checkpoints.entry(task).or_default();
                let version=history.last().map_or(Ok(1),|c|c.version.checked_add(1).ok_or("checkpoint overflow"))?;
                let previous=history.last().map(|c|c.payload.clone());
                history.push(super::model::Checkpoint{version,generation,worker:l.worker,incarnation:l.incarnation,payload,previous,input,task_version,class,context});
            }
            Submit{task,generation,output}=>{
                let t=self.check_lease(actor,&task,generation)?;ensure(!self.has_uncertain_effect(&task)&&!self.has_abandoned_effect(&task),"effect reconciliation required or abandoned")?;ensure(t.spec.output.accepts(&output),"output shape mismatch")?;
                let payload=self.put_payload(output)?;let t=self.tasks.get_mut(&task).unwrap();t.proposal=Some(payload);t.phase=TaskPhase::Submitted;
                let worker=t.lease.as_ref().unwrap().worker.clone();
                let a=self.agents.get_mut(&worker).unwrap();
                if let Some(i)=a.incarnations.last_mut(){if i.executor=="native-pure-process"{i.ended_ms=Some(self.clock_ms);i.reason=Some("submitted".into());}}
                a.phase=AgentPhase::Dormant;
            }
            Fail{task,generation,reason}=>{let t=self.check_lease(actor,&task,generation)?;let worker=t.lease.as_ref().unwrap().worker.clone();ensure(!reason.trim().is_empty(),"reason required")?;self.end_agent(&worker,&reason,false)?;}
            Verify{task}=>{
                actor.host()?;let t=self.tasks.get(&task).ok_or("task missing")?.clone();
                ensure(!self.has_uncertain_effect(&task),"effect reconciliation required")?;
                ensure(t.phase==TaskPhase::Submitted,"no submission")?;self.open_goal(&t.spec.goal)?;
                let input=t.input.as_ref().ok_or("input missing")?;let candidate=t.proposal.clone().ok_or("proposal missing")?;
                let output=self.payload(&candidate)?;let oracle=t.spec.oracle.evaluate(input,output);
                let accepted=t.spec.output.accepts(output)&&oracle.as_ref().is_ok_and(|v|*v);
                let rejection_reason=oracle.err().unwrap_or_else(||"oracle rejected".into());
                let input_digest=self.put_payload(input.clone())?;
                let parents:Vec<_>=t.spec.dependencies.iter().map(|d|self.tasks[d].artifact.clone().ok_or("dependency artifact missing")).collect::<std::result::Result<_,_>>()?;
                let evidence_id=format!("{}-{}",task,t.generation);ensure(!self.evidence.contains_key(&evidence_id),"evidence already exists")?;
                let parent_evidence=parents.iter().map(|p|self.artifacts[p].evidence.clone()).collect();
                self.evidence.insert(evidence_id.clone(),super::model::Evidence{id:evidence_id.clone(),task:task.clone(),generation:t.generation,oracle:t.spec.oracle.clone(),input:input_digest,candidate:candidate.clone(),accepted,verifier:"host".into(),parents:parent_evidence,clock_ms:self.clock_ms,class:t.assigned_class.clone(),context:t.assigned_context.clone()});
                if accepted{
                    ensure(!self.artifacts.contains_key(&task),"duplicate admission")?;
                    self.artifacts.insert(task.clone(),Artifact{id:task.clone(),version:t.version,payload:candidate,task:task.clone(),generation:t.generation,inputs:parents,evidence:evidence_id});
                    let x=self.tasks.get_mut(&task).unwrap();x.artifact=Some(task.clone());x.phase=TaskPhase::Verified;x.lease=None;x.proposal=None;
                    self.metrics.accepted=self.metrics.accepted.saturating_add(1);
                    let native=self.run_costs.contains_key(&format!("{task}-{}",t.generation));
                    if native{self.metrics.deterministic_runs=self.metrics.deterministic_runs.saturating_add(1);}
                    if native&&t.spec.class.is_none()&&t.spec.family.is_some()&&t.assigned_class.is_some(){self.metrics.reused=self.metrics.reused.saturating_add(1);}
                }else{self.metrics.rejected=self.metrics.rejected.saturating_add(1);self.metrics.verification_rejections=self.metrics.verification_rejections.saturating_add(1);self.release(&task,&rejection_reason)?;}
                return Ok(json!({"accepted":accepted,"task":task}));
            }
            Tick{now_ms}=>{
                actor.host()?;ensure(now_ms>=self.clock_ms,"clock reversed")?;self.clock_ms=now_ms;
                let workers:BTreeSet<_>=self.tasks.values().filter(|t|t.phase==TaskPhase::Running).filter_map(|t|t.lease.as_ref()).filter(|l|l.deadline_ms<=now_ms).map(|l|l.worker.clone()).collect();
                for w in workers{self.end_agent(&w,"lease expired",false)?;}
            }
            Send{id:key,recipient,body}=>{
                let (sender,_)=self.worker_identity(actor)?;let s=self.agents.get(sender).unwrap();let r=self.agents.get(&recipient).ok_or("recipient missing")?;
                ensure(s.goal==r.goal&&r.phase!=AgentPhase::Archived,"recipient unavailable")?;id(&key)?;
                let payload=self.put_payload(body)?;
                if let Some(m)=self.messages.get(&key){ensure(m.sender==sender&&m.recipient==recipient&&m.body==payload,"message id conflict")?;}
                else{self.messages.insert(key.clone(),Message{id:key,sender:sender.into(),recipient:recipient.clone(),body:payload,sequence:self.sequence+1,acknowledged:false});}
                let r=self.agents.get_mut(&recipient).unwrap();if r.phase==AgentPhase::WaitingMessage{r.phase=AgentPhase::Dormant;}
            }
            Ack{id:key}=>{let (who,_)=self.worker_identity(actor)?;let m=self.messages.get_mut(&key).ok_or("message missing")?;ensure(m.recipient==who,"wrong recipient")?;m.acknowledged=true;}
            WaitMessage{agent}=>{
                let (who,_)=self.worker_identity(actor)?;ensure(who==agent,"wrong worker")?;
                let a=self.agents.get_mut(&agent).unwrap();ensure(a.phase!=AgentPhase::Running,"worker busy")?;
                a.phase=if self.messages.values().any(|m|m.recipient==agent&&!m.acknowledged){AgentPhase::Dormant}else{AgentPhase::WaitingMessage};
            }
            WaitDependency{agent,task}=>{
                let (who,_)=self.worker_identity(actor)?;ensure(who==agent,"wrong worker")?;
                let t=self.tasks.get(&task).ok_or("task missing")?;
                let a=self.agents.get_mut(&agent).unwrap();ensure(a.goal==t.spec.goal&&a.phase!=AgentPhase::Running,"worker/task mismatch")?;
                a.wait_task=Some(task);a.phase=AgentPhase::WaitingDependency;
            }
            Cancel{goal}=>{
                actor.host()?;self.open_goal(&goal)?;let mut todo=vec![goal];
                while let Some(g)=todo.pop(){
                    todo.extend(self.goals.values().filter(|x|x.parent.as_ref()==Some(&g)&&x.phase==GoalPhase::Open).map(|x|x.id.clone()));
                    self.goals.get_mut(&g).unwrap().phase=GoalPhase::Cancelled;
                    let affected:Vec<_>=self.tasks.values().filter(|t|t.spec.goal==g).map(|t|t.spec.id.clone()).collect();
                    for t in affected{self.mark_prepared_unknown(&t,"goal cancelled");}
                    for t in self.tasks.values_mut().filter(|t|t.spec.goal==g&&t.phase!=TaskPhase::Verified){t.phase=TaskPhase::Cancelled;t.lease=None;t.proposal=None;}
                    let agents:Vec<_>=self.agents.values().filter(|a|a.goal==g).map(|a|a.id.clone()).collect();
                    for a in agents{self.end_agent(&a,"cancelled",true)?;}
                }
            }
            AdmitCapability{capability}=>{actor.host()?;self.admit_capability(capability)?;}
            DisableCapability{capability}=>{
                actor.host()?;let c=self.capabilities.get_mut(&capability).ok_or("capability missing")?;c.enabled=false;
                let class=c.class.clone();if !self.capabilities.values().any(|x|x.enabled&&x.class==class){
                    for k in self.knowledge.values_mut().filter(|k|k.class.as_ref()==Some(&class)){k.enabled=false;}
                }
            }
            ConfigureLearning{plan}=>{actor.host()?;let k=self.configure_learning(plan)?;return Ok(json!({"plan":k}));}
            LearnPlan{plan}=>{actor.host()?;return self.run_learning(&plan);}
            PublishKnowledge{knowledge}=>{actor.host()?;let key=self.publish_knowledge(knowledge)?;return Ok(json!({"knowledge":key}));}
            DisableKnowledge{knowledge}=>{actor.host()?;self.knowledge.get_mut(&knowledge).ok_or("knowledge missing")?.enabled=false;}
            RecordCost{task,generation,elapsed_ms}=>{actor.host()?;self.record_cost(&task,generation,elapsed_ms)?;}
            PrepareEffect{task,generation,key,payload,oracle}=>{actor.host()?;return self.prepare_effect(&task,generation,key,payload,oracle);}
            EffectUnknown{effect,reason}=>{actor.host()?;self.effect_unknown(&effect,reason)?;}
            ResolveEffect{effect,outcome,receipt}=>{actor.host()?;return self.resolve_effect(&effect,outcome,receipt);}
            SetPolicy{policy}=>{
                actor.host()?;ensure(policy.version>self.policy.version,"policy version must increase")?;
                ensure([policy.priority_weight,policy.critical_weight,policy.uncertainty_weight,policy.information_weight,policy.cost_weight].iter().all(|w|*w<=100_000),"policy weights exceed budget")?;
                self.policy=policy;
            }
        }
        Ok(json!({"ok":true}))
    }
}
