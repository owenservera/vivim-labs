//! Learn reusable pure DAG procedures from independently verified goal traces.
//! Learning never creates external-effect authority or weakens a future oracle.
use super::{model::*,transition::{Actor,Action}};
use crate::{digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::collections::{BTreeMap,BTreeSet};
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Policy{
    pub version:u64,pub family:Id,pub minimum_goals:usize,pub max_goals:usize,pub max_steps:usize,pub max_recipes:usize,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(tag="kind",rename_all="snake_case")]
pub enum Source{Parameter{key:Id},Step{index:usize}}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Input{pub shape:Shape,pub source:Source}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Step{
    pub class:Id,pub context:Id,pub output:Shape,pub inputs:BTreeMap<Id,Input>,pub dependencies:BTreeSet<usize>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Example{
    pub goal:Id,pub tasks:Vec<Id>,pub evidence:Vec<Id>,pub input_digest:Digest,pub repaired_steps:BTreeSet<usize>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Recipe{
    pub id:Id,pub family:Id,pub steps:Vec<Step>,pub examples:Vec<Example>,pub enabled:bool,pub sequence:u64,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Instantiate{
    pub recipe:Id,pub goal:Id,pub objective:String,pub parameters:BTreeMap<Id,Value>,pub oracles:Vec<Oracle>,pub claim_budget:u64,
}
impl World{
    pub fn configure_recipe_policy(&mut self,p:Policy)->Result<()>{
        id(&p.family)?;
        ensure(p.version>self.recipe_policies.get(&p.family).map_or(0,|x|x.version)&&(2..=32).contains(&p.minimum_goals)&&p.max_goals>=p.minimum_goals&&p.max_goals<=128&&(1..=32).contains(&p.max_steps)&&(1..=128).contains(&p.max_recipes),"invalid recipe policy")?;
        self.recipe_policies.insert(p.family.clone(),p);Ok(())
    }
    fn trace_recipe(&self,goal:&str,p:&Policy)->Result<(Vec<Step>,Example)>{
        let g=self.goals.get(goal).ok_or("goal missing")?;
        ensure(g.phase==GoalPhase::Complete&&!self.goals.values().any(|g|g.parent.as_deref()==Some(goal)),"recipe requires complete leaf goal")?;
        let tasks:BTreeMap<_,_>=self.tasks.values().filter(|t|t.spec.goal==goal).map(|t|(t.spec.id.clone(),t)).collect();
        ensure(!tasks.is_empty()&&tasks.len()<=p.max_steps,"recipe step budget")?;
        ensure(tasks.values().all(|t|t.phase==TaskPhase::Verified&&t.spec.family.as_ref()==Some(&p.family)&&!self.effects.values().any(|e|e.task==t.spec.id)),"recipe requires pure verified family tasks")?;
        let mut order=Vec::<Id>::new();let mut indices=BTreeMap::new();
        while order.len()<tasks.len(){
            let next=tasks.iter().find(|(k,t)|!indices.contains_key(*k)&&t.spec.dependencies.iter().all(|d|indices.contains_key(d))).map(|(k,_)|k.clone()).ok_or("recipe dependency cycle")?;
            indices.insert(next.clone(),order.len());order.push(next);
        }
        let mut steps=vec![];let mut evidence=vec![];let mut parameters=BTreeMap::new();let mut repaired_steps=BTreeSet::new();
        for (i,key) in order.iter().enumerate(){
            let t=tasks[key];let class=t.assigned_class.clone().ok_or("recipe class missing")?;let context=t.assigned_context.clone().ok_or("recipe context missing")?;
            ensure(self.class_available(&class)&&self.contexts.contains_key(&context),"recipe class/context unavailable")?;
            let input=t.input.as_ref().ok_or("recipe input missing")?;
            let artifact=self.artifacts.get(t.artifact.as_deref().ok_or("recipe artifact missing")?).ok_or("recipe artifact unavailable")?;
            let e=self.evidence.get(&artifact.evidence).ok_or("recipe evidence missing")?;
            let output=self.payload(&artifact.payload)?;
            ensure(e.accepted&&e.task==*key&&e.generation==t.generation&&e.input==digest(input)&&e.candidate==artifact.payload&&t.spec.oracle.evaluate(input,output)?,"recipe evidence mismatch")?;
            ensure(self.run_class(&class,input)?==*output,"recipe native replay differs from evidence")?;
            let mut inputs=BTreeMap::new();
            for (port,definition) in &t.spec.inputs{
                let source=match &definition.binding{
                    Binding::Predecessor{task}=>Source::Step{index:*indices.get(task).ok_or("recipe predecessor missing")?},
                    _=>{
                        let value=input.get(port).ok_or("recipe parameter missing")?;
                        // All non-predecessor values are supplied afresh on reuse.
                        // No old context content or secret is copied into the recipe.
                        let key=format!("p-{i}-{port}");id(&key)?;
                        parameters.insert(key.clone(),digest(value));Source::Parameter{key}
                    }
                };
                inputs.insert(port.clone(),Input{shape:definition.shape.clone(),source});
            }
            if t.attempts>1{repaired_steps.insert(i);}
            evidence.push(e.id.clone());
            steps.push(Step{class,context,output:t.spec.output.clone(),inputs,dependencies:t.spec.dependencies.iter().map(|x|indices[x]).collect()});
        }
        Ok((steps,Example{goal:goal.into(),tasks:order,evidence,input_digest:digest(&json!(parameters)),repaired_steps}))
    }
    pub fn discover_recipes(&mut self,family:&str)->Result<Value>{
        let p=self.recipe_policies.get(family).ok_or("recipe policy missing")?.clone();
        let goals:Vec<_>=self.goals.values().filter(|g|g.phase==GoalPhase::Complete).take(p.max_goals).map(|g|g.id.clone()).collect();
        let mut groups:BTreeMap<Digest,(Vec<Step>,Vec<Example>)>=BTreeMap::new();
        let mut rejected=0u64;
        for g in &goals{
            match self.trace_recipe(g,&p){
                Ok((steps,e))=>{
                    let hash=digest(&json!([family,&steps]));let group=groups.entry(hash).or_insert((steps,vec![]));
                    if !group.1.iter().any(|x|x.input_digest==e.input_digest){group.1.push(e);}
                },
                Err(_)=>rejected+=1,
            }
        }
        let mut added=vec![];
        let existing=self.recipes.values().filter(|r|r.family==family).count();
        for (hash,(steps,examples)) in groups{
            if examples.len()<p.minimum_goals||existing+added.len()>=p.max_recipes{continue;}
            let key=format!("recipe-{hash}");if self.recipes.contains_key(&key){continue;}
            self.recipes.insert(key.clone(),Recipe{id:key.clone(),family:family.into(),steps,examples,enabled:true,sequence:self.sequence.saturating_add(1)});
            added.push(key);
        }
        self.recipe_scans.insert(family.into(),self.metrics.accepted);
        Ok(json!({"family":family,"scanned_goals":goals.len(),"rejected_traces":rejected,"published":added}))
    }
    pub fn ready_recipe_scans(&self)->Vec<Id>{
        self.recipe_policies.keys().filter(|k|self.metrics.accepted>self.recipe_scans.get(*k).copied().unwrap_or(0)).cloned().collect()
    }
    pub fn instantiate_recipe(&mut self,r:Instantiate)->Result<Value>{
        let recipe=self.recipes.get(&r.recipe).ok_or("recipe missing")?.clone();
        id(&r.goal)?;ensure(r.goal.len()<=80&&recipe.enabled&&r.oracles.len()==recipe.steps.len()&&r.claim_budget>=recipe.steps.len() as u64&&r.claim_budget<=1024,"invalid recipe instantiation")?;
        let actor=Actor::Host;
        let expected:BTreeSet<_>=recipe.steps.iter().flat_map(|s|s.inputs.values()).filter_map(|i|match &i.source{Source::Parameter{key}=>Some(key.clone()),_=>None}).collect();
        ensure(expected==r.parameters.keys().cloned().collect(),"recipe parameter set mismatch")?;
        let keys:Vec<_>=(0..recipe.steps.len()).map(|i|format!("{}-step-{i}",r.goal)).collect();
        self.apply_adapter_action(&actor,Action::Goal{id:r.goal.clone(),parent:None,objective:r.objective,claim_budget:r.claim_budget,predicate:None})?;
        for (i,s) in recipe.steps.iter().enumerate(){
            ensure(self.class_available(&s.class),"recipe class disabled")?;
            let mut inputs=BTreeMap::new();
            for (name,input) in &s.inputs{
                let binding=match &input.source{
                    Source::Parameter{key}=>Binding::Literal{value:r.parameters[key].clone()},
                    Source::Step{index}=>{ensure(*index<i,"recipe predecessor order")?;Binding::Predecessor{task:keys[*index].clone()}}
                };
                inputs.insert(name.clone(),Port{shape:input.shape.clone(),binding});
            }
            self.apply_adapter_action(&actor,Action::Task{spec:TaskSpec{id:keys[i].clone(),goal:r.goal.clone(),objective:format!("verified recipe {} step {i}",recipe.id),dependencies:s.dependencies.iter().map(|x|keys[*x].clone()).collect(),inputs,output:s.output.clone(),oracle:r.oracles[i].clone(),class:Some(s.class.clone()),family:Some(recipe.family.clone()),context:Some(s.context.clone()),max_attempts:2,priority:0,uncertainty:0,information_gain:0,expected_cost:1}})?;
        }
        self.apply_adapter_action(&actor,Action::Seal{goal:r.goal.clone()})?;
        Ok(json!({"goal":r.goal,"recipe":recipe.id,"tasks":keys,"execution":"not_started"}))
    }
}
