//! Non-authoritative resident projections. Exact mutation ancestry or full
//! rebuild; never trust a revision number shared by diverging World clones.
use super::{model::*,shared_map::Stamp,topology::Topology};
use serde_json::{json,Value};
use std::{collections::{BTreeMap,BTreeSet},sync::{Arc,Mutex}};
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
    pub(crate) schedule_costs:BTreeMap<Id,u64>,pub(crate) critical_paths:BTreeMap<Id,u64>,
}
impl Derived{
    pub(crate) fn build(w:&World)->Self{
        let mut d=Self{stamps:Stamps::of(w),topology:Topology::build(w),costs:BTreeMap::new(),cost_rows:BTreeMap::new(),
            attempt_rows:BTreeMap::new(),task_outcomes:BTreeMap::new(),outcomes:BTreeMap::new(),schedule_costs:BTreeMap::new(),critical_paths:BTreeMap::new()};
        for key in w.run_costs.keys(){d.cost(w,key);}
        for key in w.attempt_history.keys(){d.attempt(w,key);}
        d.family_outcomes(w);d.recompute_paths(w,&w.tasks.keys().cloned().collect());d
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
    fn adjust_family(&mut self,f:&str,n:Outcome,add:bool){
        if n.0==0{return;}
        if add{
            let e=self.outcomes.entry(f.into()).or_default();e.0+=n.0;e.1+=n.1;
        }else{
            let e=self.outcomes.get_mut(f).expect("derived family contribution");
            e.0-=n.0;e.1-=n.1;
            if e.0==0{self.outcomes.remove(f);}
        }
    }
    fn recompute_paths(&mut self,w:&World,affected:&BTreeSet<Id>){
        for k in affected{self.schedule_costs.remove(k);self.critical_paths.remove(k);}
        let live=|t:&Task|matches!(t.phase,TaskPhase::Pending|TaskPhase::Running|TaskPhase::Submitted|TaskPhase::Reconciling);
        let costs:BTreeMap<_,u64>=affected.iter().filter_map(|k|w.tasks.get(k)).filter(|t|live(t)).map(|t|{
            let n=t.spec.family.as_ref().and_then(|f|self.costs.get(f)).map(|(sum,n)|u64::try_from((sum/n).max(1)).unwrap_or(u64::MAX)).unwrap_or(u64::from(t.spec.expected_cost));
            (t.spec.id.clone(),n)
        }).collect();
        let mut successors:BTreeMap<Id,Vec<Id>>=costs.keys().map(|k|(k.clone(),vec![])).collect();
        let mut degree:BTreeMap<Id,usize>=costs.keys().map(|k|(k.clone(),0)).collect();
        for key in costs.keys(){
            let Some(t)=w.tasks.get(key) else{continue;};
            for p in &t.spec.dependencies{
                if let Some(s)=successors.get_mut(p){s.push(key.clone());*degree.get_mut(key).unwrap()+=1;}
            }
        }
        let mut queue:BTreeSet<_>=degree.iter().filter(|(_,n)|**n==0).map(|(k,_)|k.clone()).collect();
        let mut order=vec![];
        while let Some(k)=queue.pop_first(){
            order.push(k.clone());
            for s in &successors[&k]{let n=degree.get_mut(s).unwrap();*n-=1;if *n==0{queue.insert(s.clone());}}
        }
        for k in order.into_iter().rev(){
            let tail=successors[&k].iter().filter_map(|s|self.critical_paths.get(s)).copied().max().unwrap_or(0);
            self.critical_paths.insert(k.clone(),costs[&k].saturating_add(tail));
        }
        self.schedule_costs.extend(costs);
    }
    fn refresh(&mut self,w:&World,now:Stamps)->bool{
        let Some(goals)=w.goals.changes_since(&self.stamps.goals) else{return false;};
        let Some(tasks)=w.tasks.changes_since(&self.stamps.tasks) else{return false;};
        let Some(agents)=w.agents.changes_since(&self.stamps.agents) else{return false;};
        let Some(costs)=w.run_costs.changes_since(&self.stamps.costs) else{return false;};
        let Some(attempts)=w.attempt_history.changes_since(&self.stamps.attempts) else{return false;};
        // A family cost can affect many independent components. Include its
        // old and new task memberships and both versions of rewired edges.
        let mut families=BTreeSet::new();
        for k in &costs{
            if let Some((f,_))=self.cost_rows.get(k){families.insert(f.clone());}
            if let Some(f)=w.run_costs.get(k).and_then(|r|r.family.as_ref()){families.insert(f.clone());}
        }
        let mut roots=tasks.clone();
        for f in &families{roots.extend(self.topology.family_tasks(f).iter().cloned());}
        let mut affected=self.topology.component(&roots);
        // Remove old family attribution for only the affected task/attempt
        // rows; add current attribution after applying their changes.
        let mut outcome_tasks=tasks.clone();
        for k in &attempts{
            if let Some((t,_))=self.attempt_rows.get(k){outcome_tasks.insert(t.clone());}
            if let Some(a)=w.attempt_history.get(k){outcome_tasks.insert(a.task.clone());}
        }
        for k in &outcome_tasks{
            let old=self.topology.task_family(k).map(str::to_owned);
            let n=self.task_outcomes.get(k).copied().unwrap_or_default();
            if let Some(f)=old{self.adjust_family(&f,n,false);}
        }
        for k in goals{self.topology.goal(w,&k);}
        for k in &tasks{self.topology.task(w,k);}
        for k in agents{self.topology.agent(w,&k);}
        for k in costs{self.cost(w,&k);}
        for k in attempts{self.attempt(w,&k);}
        for k in &outcome_tasks{
            let new=self.topology.task_family(k).map(str::to_owned);
            let n=self.task_outcomes.get(k).copied().unwrap_or_default();
            if let Some(f)=new{self.adjust_family(&f,n,true);}
        }
        for f in &families{roots.extend(self.topology.family_tasks(f).iter().cloned());}
        // Former components remain dirty even after disconnect/deletion.
        roots.extend(affected.iter().cloned());
        affected.extend(self.topology.component(&roots));
        if !affected.is_empty(){self.recompute_paths(w,&affected);}
        self.stamps=now;true
    }
    pub(crate) fn projection(&self)->Value{
        // Decimal strings preserve exact wide integer sums in JSON.
        let costs:BTreeMap<_,_>=self.costs.iter().map(|(f,(sum,n))|(f.clone(),[sum.to_string(),n.to_string()])).collect();
        json!({"schema":1,"topology":self.topology,"costs":costs,"family_outcomes":self.outcomes,"schedule_costs":self.schedule_costs,"critical_paths":self.critical_paths})
    }
}
#[derive(Default,Debug)]
pub struct Cache{inner:Mutex<Option<Arc<Derived>>>}
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

#[cfg(test)]
mod scheduler_equivalence{
    use super::*;
    use crate::kernel::transition::{Action,Actor};
    use serde_json::json;
    fn host(w:&mut World,a:Action){w.transition(&Actor::Host,a).unwrap();}
    fn task(w:&mut World,key:&str,goal:&str,parents:&[&str],family:&str){
        host(w,Action::Task{spec:TaskSpec{id:key.into(),goal:goal.into(),objective:"scheduler qualification".into(),
            dependencies:parents.iter().map(|p|p.to_string()).collect(),inputs:BTreeMap::new(),output:Shape::Integer,
            oracle:Oracle::Digest{expected:crate::digest(&json!(1))},class:None,family:Some(family.into()),context:None,
            max_attempts:2,priority:1,uncertainty:1,information_gain:1,expected_cost:10}});
    }
    // Reference reconstruction intentionally does not consult derived cache.
    fn reference(w:&World)->BTreeMap<Id,i128>{
        let mut observations:BTreeMap<Id,(u128,u128)>=BTreeMap::new();
        for r in w.run_costs.values(){if let Some(f)=&r.family{let e=observations.entry(f.clone()).or_default();e.0=e.0.saturating_add(u128::from(r.elapsed_ms));e.1=e.1.saturating_add(1);}}
        let live=|t:&Task|matches!(t.phase,TaskPhase::Pending|TaskPhase::Running|TaskPhase::Submitted|TaskPhase::Reconciling);
        let costs:BTreeMap<_,u64>=w.tasks.values().filter(|t|live(t)).map(|t|{
            let cost=t.spec.family.as_ref().and_then(|f|observations.get(f)).map(|(sum,n)|u64::try_from((sum/n).max(1)).unwrap_or(u64::MAX)).unwrap_or(u64::from(t.spec.expected_cost));
            (t.spec.id.clone(),cost)
        }).collect();
        let mut successors:BTreeMap<Id,Vec<Id>>=costs.keys().map(|k|(k.clone(),vec![])).collect();
        let mut degree:BTreeMap<Id,usize>=costs.keys().map(|k|(k.clone(),0)).collect();
        for t in w.tasks.values().filter(|t|live(t)){
            for p in &t.spec.dependencies{if let Some(s)=successors.get_mut(p){s.push(t.spec.id.clone());*degree.get_mut(&t.spec.id).unwrap()+=1;}}
        }
        let mut queue:BTreeSet<_>=degree.iter().filter(|(_,d)|**d==0).map(|(k,_)|k.clone()).collect();
        let mut order=vec![];
        while let Some(k)=queue.pop_first(){order.push(k.clone());for s in &successors[&k]{let n=degree.get_mut(s).unwrap();*n-=1;if *n==0{queue.insert(s.clone());}}}
        let mut path:BTreeMap<Id,u64>=BTreeMap::new();
        for k in order.into_iter().rev(){let tail=successors[&k].iter().filter_map(|s|path.get(s)).copied().max().unwrap_or(0);path.insert(k.clone(),costs[&k].saturating_add(tail));}
        let mut outcomes:BTreeMap<Id,(u64,u64)>=BTreeMap::new();
        for a in w.attempt_history.values().filter(|a|a.ended_ms.is_some()){
            if let Some(f)=w.tasks.get(&a.task).and_then(|t|t.spec.family.as_ref()){
                let e=outcomes.entry(f.clone()).or_default();e.0=e.0.saturating_add(1);
                if a.outcome.as_deref()==Some("verified"){e.1=e.1.saturating_add(1);}
            }
        }
        let p=&w.policy;
        w.tasks.values().filter(|t|live(t)).map(|t|{
            let (attempts,successes)=t.spec.family.as_ref().and_then(|f|outcomes.get(f)).copied().unwrap_or((0,0));
            let failure=1000u128*(u128::from(attempts.saturating_sub(successes))+1)/(u128::from(attempts)+2);
            let novelty=1000u64/(attempts.saturating_add(1));
            let score=i128::from(t.spec.priority)*i128::from(p.priority_weight)
                +i128::from(path.get(&t.spec.id).copied().unwrap_or(0))*i128::from(p.critical_weight)
                +(i128::from(t.spec.uncertainty)+failure as i128)*i128::from(p.uncertainty_weight)
                +(i128::from(t.spec.information_gain)+i128::from(novelty))*i128::from(p.information_weight)
                -i128::from(costs[&t.spec.id])*i128::from(p.cost_weight);
            (t.spec.id.clone(),score)
        }).collect()
    }
    #[test]
    fn graph_rewire_family_cost_outcome_and_branch_match_reference(){
        let mut w=World::default();
        for goal in ["g1","g2"]{host(&mut w,Action::Goal{id:goal.into(),parent:None,objective:"fixture".into(),claim_budget:32,predicate:None});}
        task(&mut w,"a","g1",&[],"f");task(&mut w,"b","g1",&["a"],"f");
        task(&mut w,"c","g1",&["b"],"h");task(&mut w,"x","g2",&[],"f");task(&mut w,"y","g2",&["x"],"h");
        let check=|w:&World|{assert_eq!(w.schedule_scores(),reference(w));assert_eq!(w.derived_audit()["equivalent"],true);};
        check(&w);
        // Direct fixture mutations deliberately exercise changes unavailable
        // in normal admission, including deletion and graph branch divergence.
        w.run_costs.insert("r".into(),RunCost{task:"a".into(),generation:1,elapsed_ms:71,family:Some("f".into()),class:None});check(&w);
        w.attempt_history.insert("a-1".into(),AttemptRecord{task:"a".into(),generation:1,agent:"agent".into(),incarnation:1,class:"class".into(),started_ms:0,ended_ms:Some(1),outcome:Some("verified".into()),measured_ms:None});check(&w);
        let mut branch=w.clone();
        w.tasks.get_mut("b").unwrap().spec.dependencies.clear();check(&w);
        w.tasks.get_mut("a").unwrap().spec.family=Some("h".into());check(&w);
        branch.tasks.get_mut("a").unwrap().phase=TaskPhase::Verified;check(&branch);check(&w);
        w.attempt_history.get_mut("a-1").unwrap().outcome=Some("failed".into());check(&w);
        w.run_costs.remove("r");check(&w);
        w.tasks.remove("c");check(&w);
        w.attempt_history.clear();check(&w);
        w.tasks.retain(|k,_|k!="b");check(&w);
    }
}
