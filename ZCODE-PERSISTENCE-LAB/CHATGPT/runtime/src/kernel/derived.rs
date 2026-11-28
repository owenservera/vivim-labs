//! Non-authoritative resident projections. Exact mutation ancestry or full
//! rebuild; never trust a revision number shared by diverging World clones.
use super::{model::*,shared_map::Stamp,topology::Topology};
use serde_json::{json,Value};
use std::{collections::BTreeMap,sync::{Arc,Mutex}};
type Cost=(u128,u128);
type Outcome=(u64,u64);
#[derive(Clone,Debug)]
struct Stamps{goals:Stamp,tasks:Stamp,agents:Stamp,costs:Stamp,attempts:Stamp}
impl Stamps{
    fn of(w:&World)->Self{Self{goals:w.goals.stamp(),tasks:w.tasks.stamp(),agents:w.agents.stamp(),costs:w.run_costs.stamp(),attempts:w.attempt_history.stamp()}}
    fn same(&self,b:&Self)->bool{self.goals.same(&b.goals)&&self.tasks.same(&b.tasks)&&self.agents.same(&b.agents)&&self.costs.same(&b.costs)&&self.attempts.same(&b.attempts)}
}
#[derive(Clone,Debug)]
pub(crate) struct Derived{
    stamps:Stamps,pub(crate) topology:Topology,
    pub(crate) costs:BTreeMap<Id,Cost>,
    cost_rows:BTreeMap<Id,(Id,u64)>,
    attempt_rows:BTreeMap<Id,(Id,bool)>,
    task_outcomes:BTreeMap<Id,Outcome>,
    pub(crate) outcomes:BTreeMap<Id,Outcome>,
}
impl Derived{
    pub(crate) fn build(w:&World)->Self{
        let mut d=Self{stamps:Stamps::of(w),topology:Topology::build(w),costs:BTreeMap::new(),cost_rows:BTreeMap::new(),
            attempt_rows:BTreeMap::new(),task_outcomes:BTreeMap::new(),outcomes:BTreeMap::new()};
        for key in w.run_costs.keys(){d.cost(w,key);}
        for key in w.attempt_history.keys(){d.attempt(w,key);}
        d.family_outcomes(w);d
    }
    fn cost(&mut self,w:&World,key:&str){
        let next=w.run_costs.get(key).and_then(|r|r.family.as_ref().map(|f|(f.clone(),r.elapsed_ms)));
        if self.cost_rows.get(key)==next.as_ref(){return;}
        if let Some((f,n))=self.cost_rows.remove(key){
            let total=self.costs.get_mut(&f).expect("derived cost contribution");
            total.0-=u128::from(n);total.1-=1;
            if total.1==0{self.costs.remove(&f);}
        }
        if let Some((f,n))=next{
            let total=self.costs.entry(f.clone()).or_default();total.0+=u128::from(n);total.1+=1;
            self.cost_rows.insert(key.into(),(f,n));
        }
    }
    fn attempt(&mut self,w:&World,key:&str){
        let next=w.attempt_history.get(key).filter(|a|a.ended_ms.is_some()).map(|a|(a.task.clone(),a.outcome.as_deref()==Some("verified")));
        if self.attempt_rows.get(key)==next.as_ref(){return;}
        if let Some((task,ok))=self.attempt_rows.remove(key){
            let count=self.task_outcomes.get_mut(&task).expect("derived attempt contribution");
            count.0-=1;count.1-=u64::from(ok);
            if count.0==0{self.task_outcomes.remove(&task);}
        }
        if let Some((task,ok))=next{
            let count=self.task_outcomes.entry(task.clone()).or_default();count.0+=1;count.1+=u64::from(ok);
            self.attempt_rows.insert(key.into(),(task,ok));
        }
    }
    fn family_outcomes(&mut self,w:&World){
        // Family belongs to the current task, not the historical attempt.
        // This preserves the original scheduler's reclassification semantics.
        self.outcomes.clear();
        for (task,(n,ok)) in &self.task_outcomes{
            if let Some(f)=w.tasks.get(task).and_then(|t|t.spec.family.as_ref()){
                let e=self.outcomes.entry(f.clone()).or_default();
                e.0=e.0.saturating_add(*n);e.1=e.1.saturating_add(*ok);
            }
        }
    }
    fn refresh(&mut self,w:&World,now:Stamps)->bool{
        let Some(goals)=w.goals.changes_since(&self.stamps.goals) else{return false;};
        let Some(tasks)=w.tasks.changes_since(&self.stamps.tasks) else{return false;};
        let Some(agents)=w.agents.changes_since(&self.stamps.agents) else{return false;};
        let Some(costs)=w.run_costs.changes_since(&self.stamps.costs) else{return false;};
        let Some(attempts)=w.attempt_history.changes_since(&self.stamps.attempts) else{return false;};
        for k in goals{self.topology.goal(w,&k);}
        for k in &tasks{self.topology.task(w,k);}
        for k in agents{self.topology.agent(w,&k);}
        for k in costs{self.cost(w,&k);}
        let changed=!attempts.is_empty()||!tasks.is_empty();
        for k in attempts{self.attempt(w,&k);}
        if changed{self.family_outcomes(w);}
        self.stamps=now;true
    }
    pub(crate) fn projection(&self)->Value{
        // Decimal strings preserve exact wide integer sums in JSON.
        let costs:BTreeMap<_,_>=self.costs.iter().map(|(f,(sum,n))|(f.clone(),[sum.to_string(),n.to_string()])).collect();
        json!({"schema":1,"topology":self.topology,"costs":costs,"family_outcomes":self.outcomes})
    }
}
#[derive(Default,Debug)]
pub(crate) struct Cache{inner:Mutex<Option<Arc<Derived>>>}
impl Clone for Cache{
    fn clone(&self)->Self{Self{inner:Mutex::new(self.inner.lock().unwrap_or_else(|e|e.into_inner()).clone())}}
}
impl World{
    pub(crate) fn derived_snapshot(&self)->Arc<Derived>{
        let now=Stamps::of(self);
        let mut slot=self.derived.inner.lock().unwrap_or_else(|e|e.into_inner());
        if let Some(d)=slot.as_mut(){
            if d.stamps.same(&now){return d.clone();}
            if Arc::make_mut(d).refresh(self,now){return d.clone();}
        }
        let fresh=Arc::new(Derived::build(self));*slot=Some(fresh.clone());fresh
    }
    pub fn derived_audit(&self)->Value{
        let cached=self.derived_snapshot().projection();let rebuilt=Derived::build(self).projection();
        json!({"revision":self.sequence,"equivalent":cached==rebuilt,"projection_digest":crate::digest(&cached),
            "rebuilt_digest":crate::digest(&rebuilt),"authority":"derived_projection_only"})
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn lineage_updates_costs_and_falls_back_on_bulk_mutation(){
        let mut w=World::default();
        let make=|family:&str,n:u64|RunCost{task:"t".into(),generation:1,elapsed_ms:n,family:Some(family.into()),class:None};
        w.run_costs.insert("r".into(),make("a",7));
        let one=w.derived_snapshot();assert_eq!(one.costs["a"],(7,1));
        w.run_costs.insert("r".into(),make("b",11));
        let two=w.derived_snapshot();assert!(!two.costs.contains_key("a"));assert_eq!(two.costs["b"],(11,1));assert_eq!(one.costs["a"],(7,1));
        let mut other=w.clone();other.run_costs.insert("r2".into(),make("b",3));
        assert_eq!(other.derived_snapshot().costs["b"],(14,2));
        assert_eq!(w.derived_snapshot().costs["b"],(11,1));
        w.run_costs.retain(|_,_|false);assert!(w.derived_snapshot().costs.is_empty());
        assert_eq!(w.derived_audit()["equivalent"],true);
    }
}
