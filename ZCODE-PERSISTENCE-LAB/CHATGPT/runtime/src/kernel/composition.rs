//! Executable typed composition and context binding. All programs are pure.
use super::model::*;
use crate::{digest, Result};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};

fn wire_shape(wire:&Wire,input:&Shape,outputs:&BTreeMap<Id,Shape>,depth:usize)->Result<Shape>{
    ensure(depth<32,"wire depth exceeded")?;
    match wire{
        Wire::Input=>Ok(input.clone()),
        Wire::Node{id}=>outputs.get(id).cloned().ok_or("forward reference or cycle".into()),
        Wire::Field{source,key}=>{
            match wire_shape(source,input,outputs,depth+1)?{
                Shape::Record{fields}=>fields.get(key).cloned().ok_or("unknown field".into()),
                Shape::Map{value}=>Ok(*value),
                Shape::Any=>Ok(Shape::Any),
                _=>Err("field source not record".into())
            }
        }
        Wire::Record{fields}=>{
            ensure(fields.len()<=MAX_GRAPH_NODES,"wire field limit")?;
            Ok(Shape::Record{fields:fields.iter().map(|(k,w)|{id(k)?;Ok((k.clone(),wire_shape(w,input,outputs,depth+1)?))}).collect::<Result<_>>()?})
        }
        Wire::Array{items}=>{
            ensure(items.len()<=MAX_GRAPH_NODES,"wire array limit")?;
            let mut choices=vec![];
            for w in items{let shape=wire_shape(w,input,outputs,depth+1)?;if !choices.contains(&shape){choices.push(shape);}}
            let item=if choices.len()==1{choices.remove(0)}else if choices.is_empty(){Shape::Any}else{Shape::OneOf{choices}};
            Ok(Shape::Array{item:Box::new(item)})
        }
        Wire::Literal{value,shape}=>{bounded(value)?;ensure(shape.accepts(value),"literal wire shape mismatch")?;Ok(shape.clone())}
    }
}
fn resolve_wire(wire:&Wire,input:&Value,outputs:&BTreeMap<Id,Value>,depth:usize)->Result<Value>{
    ensure(depth<32,"wire depth exceeded")?;
    let value=match wire{
        Wire::Input=>input.clone(),
        Wire::Node{id}=>outputs.get(id).cloned().ok_or("node unavailable")?,
        Wire::Field{source,key}=>resolve_wire(source,input,outputs,depth+1)?.get(key).cloned().ok_or("field unavailable")?,
        Wire::Record{fields}=>Value::Object(fields.iter().map(|(k,w)|Ok((k.clone(),resolve_wire(w,input,outputs,depth+1)?))).collect::<Result<Map<_,_>>>()?),
        Wire::Array{items}=>Value::Array(items.iter().map(|w|resolve_wire(w,input,outputs,depth+1)).collect::<Result<Vec<_>>>()?),
        Wire::Literal{value,..}=>value.clone(),
    };
    bounded(&value)?;Ok(value)
}


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
            let source=wire_shape(&node.input,&class.input,&outputs,0)?;
            ensure(op.input.admits(&source),"incompatible operator input")?;
            outputs.insert(node.id.clone(),op.output.clone());
        }
        let result=wire_shape(&class.result,&class.input,&outputs,0)?;
        ensure(class.output.admits(&result),"incompatible class output")?;
        ensure(class.lineage.iter().all(|k|self.classes.contains_key(k)),"class ancestor missing")
    }
    pub fn run_class(&self,key:&str,input:&Value)->Result<Value>{
        self.run_class_resumable(key,input,None,|_|Ok(()))
    }
    /// Resume a pure graph from an integrity-checked prefix. Final output still
    /// requires the host oracle. Invalid checkpoints restart from node zero.
    pub fn run_class_resumable<F>(&self,key:&str,input:&Value,checkpoint:Option<&Value>,mut emit:F)->Result<Value>
    where F:FnMut(Value)->Result<()>{
        bounded(input)?;
        let class=self.classes.get(key).ok_or("class missing")?;
        self.validate_class(class)?;
        ensure(class.input.accepts(input),"class input shape mismatch")?;
        let contracts:Vec<_>=class.nodes.iter().map(|n|&self.operators[&n.operator]).collect();
        let graph=digest(&serde_json::json!({"class":class,"operators":contracts}));
        let input_digest=digest(input);
        let restored=checkpoint.and_then(|c|{
            if c.get("graph")?.as_str()?!=graph||c.get("input")?.as_str()?!=input_digest{return None;}
            let next=usize::try_from(c.get("next_node")?.as_u64()?).ok()?;
            if next>class.nodes.len(){return None;}
            let values:BTreeMap<Id,Value>=serde_json::from_value(c.get("outputs")?.clone()).ok()?;
            if values.len()!=next||!class.nodes[..next].iter().all(|n|values.get(&n.id).is_some_and(|v|self.operators[&n.operator].output.accepts(v))){return None;}
            let check=digest(&serde_json::to_value(&values).ok()?);
            if c.get("output_digest")?.as_str()?!=check{return None;}
            Some((next,values))
        });
        let (next,mut values)=restored.unwrap_or_default();
        let mut total_bytes=0usize;
        for y in values.values(){
            bounded(y)?;
            total_bytes=total_bytes.checked_add(serde_json::to_vec(y).map_err(|e|e.to_string())?.len()).ok_or("class memory overflow")?;
        }
        ensure(total_bytes<=8*MAX_VALUE_BYTES,"class intermediate budget exceeded")?;
        for (i,n) in class.nodes.iter().enumerate().skip(next){
            let op=&self.operators[&n.operator];
            let x=resolve_wire(&n.input,input,&values,0)?;
            ensure(op.input.accepts(&x),"operator input shape mismatch")?;
            let y=op.program.eval(&x)?;
            bounded(&y)?;ensure(op.output.accepts(&y),"operator output shape mismatch")?;
            total_bytes=total_bytes.checked_add(serde_json::to_vec(&y).map_err(|e|e.to_string())?.len()).ok_or("class memory overflow")?;
            ensure(total_bytes<=8*MAX_VALUE_BYTES,"class intermediate budget exceeded")?;
            values.insert(n.id.clone(),y);
            // Checkpoints are optional optimization state. A large prefix is not
            // persisted rather than exceeding the durable value ceiling.
            if total_bytes<MAX_VALUE_BYTES/2{
                let outputs=serde_json::to_value(&values).map_err(|e|e.to_string())?;
                let c=serde_json::json!({"graph":graph,"input":input_digest,"next_node":i+1,"output_digest":digest(&outputs),"outputs":outputs});
                if bounded(&c).is_ok(){emit(c)?;}
            }
        }
        let result=resolve_wire(&class.result,input,&values,0)?;
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
                Binding::Knowledge{version}=>self.knowledge_value(version,task.family.as_deref())?,
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
                Binding::Knowledge{version}=>{let v=self.knowledge_value(version,spec.family.as_deref())?;ensure(port.shape.accepts(v),"knowledge shape mismatch")?;}
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
        if let Some(c)=&spec.class{
            let learned=self.capabilities.values().any(|k|&k.class==c);
            return if !learned||self.capabilities.values().any(|k|&k.class==c&&k.enabled){Some(c.clone())}else{None};
        }
        let family=spec.family.as_ref()?;
        self.capabilities.values().filter(|c|c.enabled&&&c.family==family)
            .filter(|c|self.classes.get(&c.class).is_some_and(|k|k.input.admits(&Self::input_shape(spec))&&spec.output.admits(&k.output)))
            .max_by(|a,b|(a.version,&a.id).cmp(&(b.version,&b.id))).map(|c|c.class.clone())
    }
}
