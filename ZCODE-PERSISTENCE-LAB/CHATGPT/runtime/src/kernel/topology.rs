//! Derived relationship index. Never authorizes a transition: the live
//! World remains authoritative. Build from the admitted graph after structural
//! changes, then reuse only while that graph's identities/edges stay fixed.
use super::model::*;
use crate::Result;
use serde_json::{json,Value};
use serde::{Serialize,Deserialize};
use std::collections::{BTreeMap,BTreeSet,VecDeque};
#[derive(Clone,Default,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub(crate) struct Topology{
    tasks:BTreeMap<Id,Vec<Id>>,children:BTreeMap<Id,Vec<Id>>,
    agents:BTreeMap<Id,Vec<Id>>,leases:BTreeMap<Id,Vec<Id>>,
    successors:BTreeMap<Id,Vec<Id>>,families:BTreeMap<Id,Vec<Id>>,
    goal_rows:BTreeMap<Id,Id>,task_rows:BTreeMap<Id,TaskRow>,agent_rows:BTreeMap<Id,Id>,
    pending:BTreeSet<Id>,active:BTreeSet<Id>,running:BTreeSet<Id>,open_goals:BTreeSet<Id>,waiting_agents:BTreeSet<Id>,
}
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
struct TaskRow{goal:Id,parents:BTreeSet<Id>,worker:Option<Id>,family:Option<Id>}
fn add(m:&mut BTreeMap<Id,Vec<Id>>,key:&str,id:&str){
    let v=m.entry(key.into()).or_default();
    if let Err(i)=v.binary_search_by(|x|x.as_str().cmp(id)){v.insert(i,id.into());}
}
fn remove(m:&mut BTreeMap<Id,Vec<Id>>,key:&str,id:&str){
    if let Some(v)=m.get_mut(key){
        if let Ok(i)=v.binary_search_by(|x|x.as_str().cmp(id)){v.remove(i);}
        if v.is_empty(){m.remove(key);}
    }
}
impl Topology{
    pub(crate) fn build(w:&World)->Self{
        let mut t=Self::default();
        for key in w.goals.keys(){t.goal(w,key);}
        for key in w.tasks.keys(){t.task(w,key);}
        for key in w.agents.keys(){t.agent(w,key);}
        t
    }
    pub(crate) fn goal(&mut self,w:&World,key:&str){
        if w.goals.get(key).is_some_and(|g|g.phase==GoalPhase::Open){self.open_goals.insert(key.into());}
        else{self.open_goals.remove(key);}
        let next=w.goals.get(key).and_then(|g|g.parent.clone());
        if self.goal_rows.get(key)==next.as_ref(){return;}
        if let Some(old)=self.goal_rows.remove(key){remove(&mut self.children,&old,key);}
        if let Some(p)=next{add(&mut self.children,&p,key);self.goal_rows.insert(key.into(),p);}
    }
    pub(crate) fn task(&mut self,w:&World,key:&str){
        let phase=w.tasks.get(key).map(|t|&t.phase);
        if matches!(phase,Some(TaskPhase::Pending)){self.pending.insert(key.into());}else{self.pending.remove(key);}
        if matches!(phase,Some(TaskPhase::Running)){self.running.insert(key.into());}else{self.running.remove(key);}
        if matches!(phase,Some(TaskPhase::Pending|TaskPhase::Running|TaskPhase::Submitted|TaskPhase::Reconciling)){self.active.insert(key.into());}else{self.active.remove(key);}
        let next=w.tasks.get(key).map(|t|TaskRow{goal:t.spec.goal.clone(),parents:t.spec.dependencies.clone(),worker:t.lease.as_ref().map(|l|l.worker.clone()),family:t.spec.family.clone()});
        if self.task_rows.get(key)==next.as_ref(){return;}
        if let Some(old)=self.task_rows.remove(key){
            remove(&mut self.tasks,&old.goal,key);
            if let Some(worker)=old.worker{remove(&mut self.leases,&worker,key);}
            for p in old.parents{remove(&mut self.successors,&p,key);}
            if let Some(f)=old.family{remove(&mut self.families,&f,key);}
        }
        if let Some(row)=next{
            add(&mut self.tasks,&row.goal,key);
            if let Some(worker)=&row.worker{add(&mut self.leases,worker,key);}
            for p in &row.parents{add(&mut self.successors,p,key);}
            if let Some(f)=&row.family{add(&mut self.families,f,key);}
            self.task_rows.insert(key.into(),row);
        }
    }
    pub(crate) fn agent(&mut self,w:&World,key:&str){
        if w.agents.get(key).is_some_and(|a|a.phase==AgentPhase::WaitingDependency){self.waiting_agents.insert(key.into());}else{self.waiting_agents.remove(key);}
        let next=w.agents.get(key).map(|a|a.goal.clone());
        if self.agent_rows.get(key)==next.as_ref(){return;}
        if let Some(old)=self.agent_rows.remove(key){remove(&mut self.agents,&old,key);}
        if let Some(g)=next{add(&mut self.agents,&g,key);self.agent_rows.insert(key.into(),g);}
    }
    fn get<'a>(m:&'a BTreeMap<Id,Vec<Id>>,key:&str)->&'a [Id]{m.get(key).map(Vec::as_slice).unwrap_or(&[])}
    pub(crate) fn tasks(&self,key:&str)->&[Id]{Self::get(&self.tasks,key)}
    pub(crate) fn children(&self,key:&str)->&[Id]{Self::get(&self.children,key)}
    pub(crate) fn agents(&self,key:&str)->&[Id]{Self::get(&self.agents,key)}
    pub(crate) fn leases(&self,key:&str)->&[Id]{Self::get(&self.leases,key)}
    pub(crate) fn successors(&self,key:&str)->&[Id]{Self::get(&self.successors,key)}
    pub(crate) fn family_tasks(&self,key:&str)->&[Id]{Self::get(&self.families,key)}
    pub(crate) fn task_family(&self,key:&str)->Option<&str>{self.task_rows.get(key).and_then(|r|r.family.as_deref())}
    /// Include both predecessors and successors, even when an intermediate
    /// task is terminal. Conservative weak components handle rewired edges,
    /// removed nodes and cross-goal edges without relying on an old DAG.
    pub(crate) fn component(&self,roots:&BTreeSet<Id>)->BTreeSet<Id>{
        let mut seen=roots.clone();let mut queue:VecDeque<_>=roots.iter().cloned().collect();
        while let Some(key)=queue.pop_front(){
            if let Some(row)=self.task_rows.get(&key){
                for p in &row.parents{if seen.insert(p.clone()){queue.push_back(p.clone());}}
            }
            for s in self.successors(&key){if seen.insert(s.clone()){queue.push_back(s.clone());}}
        }
        seen
    }
    pub(crate) fn pending(&self)->&BTreeSet<Id>{&self.pending}
    pub(crate) fn active(&self)->&BTreeSet<Id>{&self.active}
    pub(crate) fn running(&self)->&BTreeSet<Id>{&self.running}
    pub(crate) fn open_goals(&self)->&BTreeSet<Id>{&self.open_goals}
    pub(crate) fn waiting_agents(&self)->&BTreeSet<Id>{&self.waiting_agents}
    pub(crate) fn terminal_dependents(&self,w:&World)->BTreeSet<Id>{
        // Within settlement, active membership is a superset of any task
        // that could be pending after lease release. Check live phase/edges.
        self.active.iter().filter(|key|w.tasks.get(*key).is_some_and(|t|t.phase==TaskPhase::Pending
            &&t.spec.dependencies.iter().any(|p|w.tasks.get(p).is_some_and(|x|matches!(x.phase,TaskPhase::Blocked|TaskPhase::Exhausted|TaskPhase::Cancelled)))))
            .cloned().collect()
    }
}
impl World{
    /// Bounded, deterministic forward invalidation preview. Not an effect or
    /// an instruction to cancel/recompute verified work.
    pub fn dependency_impact(&self,roots:Vec<Id>,limit:usize)->Result<Value>{
        ensure((1..=1024).contains(&limit)&&!roots.is_empty()&&roots.len()<=limit,"invalid dependency impact budget")?;
        let topology=self.derived_snapshot();let mut seen=BTreeSet::new();let mut queue=VecDeque::new();
        for k in roots{ensure(self.tasks.contains_key(&k),"impact root task missing")?;if seen.insert(k.clone()){queue.push_back((k,0usize));}}
        let mut items=vec![];let mut goals=BTreeSet::new();let mut truncated=false;
        while let Some((k,depth))=queue.pop_front(){
            let t=&self.tasks[&k];goals.insert(t.spec.goal.clone());
            items.push(json!({"task":k,"depth":depth,"phase":t.phase,"version":t.version,"generation":t.generation}));
            for next in topology.topology.successors(&k){
                if seen.contains(next){continue;}
                if seen.len()>=limit{truncated=true;continue;}
                seen.insert(next.clone());queue.push_back((next.clone(),depth+1));
            }
        }
        Ok(json!({"revision":self.sequence,"affected_tasks":items,"affected_goals":goals,
            "truncated":truncated,"authority":"read_only_invalidation_preview"}))
    }
}
