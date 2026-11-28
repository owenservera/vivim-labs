//! Executable typed composition and context binding. All programs are pure.
use super::model::*;
use crate::{digest, Result};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};

impl World {
    pub fn payload(&self, key:&str)->Result<&Value>{
        let value=self.blobs.get(key).ok_or("payload missing")?;
        ensure(digest(value)==key,"payload integrity mismatch")?;
        Ok(value)
    }
    pub fn put_payload(&mut self,value:Value)->Result<Digest>{
        bounded(&value)?;let key=digest(&value);
        if let Some(old)=self.blobs.get(&key){ensure(old==&value,"digest collision")?;}
        else {self.blobs.insert(key.clone(),value);}
        Ok(key)
    }
    pub fn artifact_value(&self,key:&str)->Result<&Value>{
        self.payload(&self.artifacts.get(key).ok_or("artifact missing")?.payload)
    }
    pub fn validate_class(&self,class:&Class)->Result<()>{
        ensure(!class.nodes.is_empty()&&class.nodes.len()<=MAX_GRAPH_NODES,"invalid class graph size")?;
        let mut outputs:BTreeMap<Id,Shape>=BTreeMap::new();
        for node in &class.nodes{
            id(&node.id)?;ensure(!outputs.contains_key(&node.id),"duplicate node")?;
            let op=self.operators.get(&node.operator).ok_or("operator missing")?;
            let source=match &node.input{Wire::Input=>&class.input,Wire::Node{id}=>outputs.get(id).ok_or("forward reference or cycle")?};
            ensure(op.input.admits(source),"incompatible operator input")?;
            outputs.insert(node.id.clone(),op.output.clone());
        }
        let result=match &class.result{Wire::Input=>&class.input,Wire::Node{id}=>outputs.get(id).ok_or("result node missing")?};
        ensure(class.output.admits(result),"incompatible class output")?;
        ensure(class.lineage.iter().all(|k|self.classes.contains_key(k)),"class ancestor missing")
    }
    pub fn run_class(&self,key:&str,input:&Value)->Result<Value>{
        bounded(input)?;
        let class=self.classes.get(key).ok_or("class missing")?;
        self.validate_class(class)?;
        ensure(class.input.accepts(input),"class input shape mismatch")?;
        let mut values:BTreeMap<Id,Value>=BTreeMap::new();
        let mut total_bytes=0usize;
        for n in &class.nodes{
            let op=&self.operators[&n.operator];
            let x=match &n.input{Wire::Input=>input,Wire::Node{id}=>values.get(id).ok_or("node unavailable")?};
            ensure(op.input.accepts(x),"operator input shape mismatch")?;
            let y=op.program.eval(x)?;
            bounded(&y)?;ensure(op.output.accepts(&y),"operator output shape mismatch")?;
            total_bytes=total_bytes.checked_add(serde_json::to_vec(&y).map_err(|e|e.to_string())?.len()).ok_or("class memory overflow")?;
            ensure(total_bytes<=8*MAX_VALUE_BYTES,"class intermediate budget exceeded")?;
            values.insert(n.id.clone(),y);
        }
        let result=match &class.result{Wire::Input=>input.clone(),Wire::Node{id}=>values.remove(id).ok_or("result unavailable")?};
        ensure(class.output.accepts(&result),"class output shape mismatch")?;
        Ok(result)
    }
    pub fn assemble(&self,task:&TaskSpec,context:&Context)->Result<Value>{
        let mut record=Map::new();
        let mut context_bytes=0usize;
        for (name,port) in &task.inputs{
            let v=match &port.binding{
                Binding::Literal{value}=>value,
                Binding::Predecessor{task:id}=>{
                    ensure(task.dependencies.contains(id),"undeclared predecessor")?;
                    let predecessor=self.tasks.get(id).ok_or("predecessor missing")?;
                    ensure(predecessor.phase==TaskPhase::Verified,"predecessor unverified")?;
                    self.artifact_value(predecessor.artifact.as_deref().ok_or("predecessor artifact missing")?)?
                }
                Binding::Context{key}=>{
                    let v=if let Some(v)=context.entries.get(key){v}
                    else{self.artifact_value(context.artifacts.get(key).ok_or("context key missing")?)?};
                    context_bytes=context_bytes.checked_add(serde_json::to_vec(v).map_err(|e|e.to_string())?.len()).ok_or("context budget overflow")?;
                    ensure(context_bytes<=context.byte_budget,"context budget exceeded")?;
                    v
                }
            };
            bounded(v)?;ensure(port.shape.accepts(v),"input port shape mismatch")?;
            record.insert(name.clone(),v.clone());
        }
        let value=Value::Object(record);bounded(&value)?;Ok(value)
    }
    pub fn input_shape(spec:&TaskSpec)->Shape{
        Shape::Record{fields:spec.inputs.iter().map(|(name,p)|(name.clone(),p.shape.clone())).collect()}
    }
    pub fn validate_task(&self,spec:&TaskSpec,replace:bool)->Result<()>{
        id(&spec.id)?;id(&spec.goal)?;
        ensure(!spec.objective.trim().is_empty()&&spec.max_attempts>0&&spec.expected_cost>0,"invalid task contract")?;
        ensure(spec.inputs.len()<=MAX_GRAPH_NODES&&spec.dependencies.len()<=MAX_GRAPH_NODES,"task graph limit")?;
        spec.oracle.validate(0)?;
        let g=self.goals.get(&spec.goal).ok_or("goal missing")?;
        ensure(g.phase==GoalPhase::Open,"goal not open")?;
        ensure(!g.sealed||replace,"goal sealed")?;
        for dependency in &spec.dependencies{
            let t=self.tasks.get(dependency).ok_or("dependency missing")?;
            ensure(t.spec.goal==spec.goal,"cross-goal dependency")?;
            let mut pending=vec![dependency.as_str()];let mut seen=BTreeSet::new();
            while let Some(next)=pending.pop(){
                ensure(next!=spec.id,"dependency cycle")?;
                if seen.insert(next){
                    let existing=self.tasks.get(next).ok_or("broken dependency")?;
                    pending.extend(existing.spec.dependencies.iter().map(String::as_str));
                }
            }
        }
        for (name,port) in &spec.inputs{
            id(name)?;
            match &port.binding{
                Binding::Literal{value}=>{bounded(value)?;ensure(port.shape.accepts(value),"literal shape mismatch")?;}
                Binding::Predecessor{task}=>{
                    ensure(spec.dependencies.contains(task),"input not in dependency set")?;
                    ensure(port.shape.admits(&self.tasks.get(task).ok_or("dependency missing")?.spec.output),"predecessor shape mismatch")?;
                }
                Binding::Context{key}=>id(key)?,
            }
        }
        if let Some(key)=&spec.class{
            let class=self.classes.get(key).ok_or("class missing")?;
            ensure(class.input.admits(&Self::input_shape(spec))&&spec.output.admits(&class.output),"task/class signature mismatch")?;
        }
        if let Some(family)=&spec.family{id(family)?;}
        if let Some(context)=&spec.context{ensure(self.contexts.contains_key(context),"context version missing")?;}
        else {ensure(!spec.inputs.values().any(|p|matches!(&p.binding,Binding::Context{..})),"context input requires explicit context version")?;}
        Ok(())
    }
    pub fn match_class(&self,spec:&TaskSpec)->Option<Id>{
        if let Some(c)=&spec.class{return Some(c.clone());}
        let family=spec.family.as_ref()?;
        self.capabilities.values().filter(|c|c.enabled&&&c.family==family)
            .filter(|c|self.classes.get(&c.class).is_some_and(|k|k.input.admits(&Self::input_shape(spec))&&spec.output.admits(&k.output)))
            .max_by(|a,b|(a.version,&a.id).cmp(&(b.version,&b.id))).map(|c|c.class.clone())
    }
}
