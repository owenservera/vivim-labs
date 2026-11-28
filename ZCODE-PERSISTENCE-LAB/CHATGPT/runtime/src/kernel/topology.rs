//! Derived relationship index. Never authorizes a transition: the live
//! World remains authoritative. Build from the admitted graph after structural
//! changes, then reuse only while that graph's identities/edges stay fixed.
use super::model::*;
use crate::Result;
use serde_json::{json,Value};
use std::collections::{BTreeMap,BTreeSet,VecDeque};
#[derive(Default,Debug)]
pub(crate) struct Topology{
    tasks:BTreeMap<Id,Vec<Id>>,children:BTreeMap<Id,Vec<Id>>,
    agents:BTreeMap<Id,Vec<Id>>,leases:BTreeMap<Id,Vec<Id>>,
    successors:BTreeMap<Id,Vec<Id>>,
}
impl Topology{
    pub(crate) fn build(w:&World)->Self{
        let mut t=Self::default();
        for (key,g) in &w.goals{if let Some(p)=&g.parent{t.children.entry(p.clone()).or_default().push(key.clone());}}
        for (key,task) in &w.tasks{
            t.tasks.entry(task.spec.goal.clone()).or_default().push(key.clone());
            if let Some(l)=&task.lease{t.leases.entry(l.worker.clone()).or_default().push(key.clone());}
            for p in &task.spec.dependencies{t.successors.entry(p.clone()).or_default().push(key.clone());}
        }
        for (key,a) in &w.agents{t.agents.entry(a.goal.clone()).or_default().push(key.clone());}
        t
    }
    fn get<'a>(m:&'a BTreeMap<Id,Vec<Id>>,key:&str)->&'a [Id]{m.get(key).map(Vec::as_slice).unwrap_or(&[])}
    pub(crate) fn tasks(&self,key:&str)->&[Id]{Self::get(&self.tasks,key)}
    pub(crate) fn children(&self,key:&str)->&[Id]{Self::get(&self.children,key)}
    pub(crate) fn agents(&self,key:&str)->&[Id]{Self::get(&self.agents,key)}
    pub(crate) fn leases(&self,key:&str)->&[Id]{Self::get(&self.leases,key)}
    pub(crate) fn successors(&self,key:&str)->&[Id]{Self::get(&self.successors,key)}
    pub(crate) fn terminal_dependents(&self,w:&World)->BTreeSet<Id>{
        let mut out=BTreeSet::new();
        for (key,t) in &w.tasks{
            if matches!(t.phase,TaskPhase::Blocked|TaskPhase::Exhausted|TaskPhase::Cancelled){
                for child in self.successors(key){
                    if w.tasks.get(child).is_some_and(|t|t.phase==TaskPhase::Pending){out.insert(child.clone());}
                }
            }
        }
        out
    }
}
impl World{
    /// Bounded, deterministic forward invalidation preview. Not an effect or
    /// an instruction to cancel/recompute verified work.
    pub fn dependency_impact(&self,roots:Vec<Id>,limit:usize)->Result<Value>{
        ensure((1..=1024).contains(&limit)&&!roots.is_empty()&&roots.len()<=limit,"invalid dependency impact budget")?;
        let topology=Topology::build(self);let mut seen=BTreeSet::new();let mut queue=VecDeque::new();
        for k in roots{ensure(self.tasks.contains_key(&k),"impact root task missing")?;if seen.insert(k.clone()){queue.push_back((k,0usize));}}
        let mut items=vec![];let mut goals=BTreeSet::new();let mut truncated=false;
        while let Some((k,depth))=queue.pop_front(){
            let t=&self.tasks[&k];goals.insert(t.spec.goal.clone());
            items.push(json!({"task":k,"depth":depth,"phase":t.phase,"version":t.version,"generation":t.generation}));
            for next in topology.successors(&k){
                if seen.contains(next){continue;}
                if seen.len()>=limit{truncated=true;continue;}
                seen.insert(next.clone());queue.push_back((next.clone(),depth+1));
            }
        }
        Ok(json!({"revision":self.sequence,"affected_tasks":items,"affected_goals":goals,
            "truncated":truncated,"authority":"read_only_invalidation_preview"}))
    }
}
