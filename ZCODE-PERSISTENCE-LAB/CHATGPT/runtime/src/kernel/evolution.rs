//! Bounded deterministic discovery. No model, arbitrary code or external effects.
use super::model::*;
use crate::{Expr,Operator,Result,digest};
use serde::{Deserialize,Serialize};
use serde_json::{Value,json};
use std::collections::{BTreeMap,BTreeSet};

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Discovery {
    pub family:Id,pub training:Vec<Id>,pub holdouts:Vec<Id>,
    pub candidates_evaluated:u64,pub program:Expr,pub input:Shape,pub output:Shape,
}
impl World {
    fn example(&self,key:&str,family:&str,input_shape:&Shape,output_shape:&Shape)->Result<(Value,Value)>{
        let t=self.tasks.get(key).ok_or("example task missing")?;
        ensure(t.phase==TaskPhase::Verified&&t.spec.family.as_deref()==Some(family),"example not verified in family")?;
        let a=self.artifacts.get(t.artifact.as_deref().ok_or("artifact missing")?).ok_or("artifact missing")?;
        let e=self.evidence.get(&a.evidence).ok_or("evidence missing")?;
        ensure(e.accepted&&e.candidate==a.payload,"example evidence invalid")?;
        let input=self.payload(&e.input)?.clone();let output=self.payload(&a.payload)?.clone();
        ensure(input_shape.accepts(&input)&&output_shape.accepts(&output),"example shape mismatch")?;
        Ok((input,output))
    }
    pub fn examples(&self,family:&str,training:&[Id],holdouts:&[Id],input:&Shape,output:&Shape)->Result<(Vec<(Value,Value)>,Vec<(Value,Value)>)>{
        ensure((2..=128).contains(&training.len())&&(2..=128).contains(&holdouts.len()),"need at least two training and two holdout examples")?;
        let mut seen=BTreeSet::new();
        let mut read=|ids:&[Id]|->Result<Vec<(Value,Value)>>{
            let mut out=Vec::new();
            for key in ids {let pair=self.example(key,family,input,output)?;
                ensure(seen.insert(digest(&pair.0)),"training/holdout inputs must be disjoint and distinct")?;out.push(pair);}
            Ok(out)
        };
        let a=read(training)?;let b=read(holdouts)?;Ok((a,b))
    }
    pub fn admit_capability(&mut self,c:Capability)->Result<()>{
        id(&c.family)?;let key=version_key(&c.id,c.version)?;
        ensure(c.enabled&&!self.capabilities.contains_key(&key),"invalid capability version")?;
        let class=self.classes.get(&c.class).ok_or("class missing")?;
        let (train,holdout)=self.examples(&c.family,&c.training,&c.holdouts,&class.input,&class.output)?;
        for (input,output) in train.iter().chain(holdout.iter()){
            ensure(self.run_class(&c.class,input)?==*output,"regression or holdout failed")?;
        }
        // New revisions must also preserve already admitted examples for the same family and signature.
        for prior in self.capabilities.values().filter(|p|p.family==c.family){
            let old=self.classes.get(&prior.class).ok_or("prior class missing")?;
            if old.input==class.input&&old.output==class.output{
                for example in prior.training.iter().chain(prior.holdouts.iter()){
                    let (x,y)=self.example(example,&c.family,&class.input,&class.output)?;
                    ensure(self.run_class(&c.class,&x)?==y,"prior version regression")?;
                }
            }
        }
        self.capabilities.insert(key,c);self.metrics.learned=self.metrics.learned.saturating_add(1);Ok(())
    }
    /// Discover one expression from verified examples, then validate a disjoint holdout
    /// only once. Failed holdouts do not drive a further search.
    pub fn discover(&self,family:&str,training:Vec<Id>,holdouts:Vec<Id>,input:Shape,output:Shape,limit:u64)->Result<Discovery>{
        ensure((1..=100_000).contains(&limit),"invalid discovery budget")?;
        let (train,holdout)=self.examples(family,&training,&holdouts,&input,&output)?;
        let mut terms=vec![Expr::Input];
        if let Shape::Record{fields}=&input{
            for key in fields.keys(){terms.push(Expr::Field{source:Box::new(Expr::Input),key:key.clone()});}
        }
        let mut n=0u64;
        let mut candidate=|program:Expr|->Result<Option<Expr>>{
            if n>=limit{return Ok(None)}n+=1;
            if train.iter().all(|(x,y)|program.eval(x).is_ok_and(|v|v==*y)){Ok(Some(program))}else{Ok(None)}
        };
        let mut found=None;
        for t in &terms{if let Some(p)=candidate(t.clone())?{found=Some(p);break;}}
        if found.is_none(){found=candidate(Expr::Const{value:train[0].1.clone()})?;}
        // Infer exact integer affine coefficients from two distinct observed x values.
        if found.is_none(){
            'outer:for t in &terms{
                let Ok(a)=t.eval(&train[0].0) else{continue};
                let (Some(x0),Some(y0))=(a.as_i64(),train[0].1.as_i64()) else{continue};
                for (x,y) in train.iter().skip(1){
                    let Ok(v)=t.eval(x) else{continue};
                    let (Some(x1),Some(y1))=(v.as_i64(),y.as_i64()) else{continue};
                    let dx=i128::from(x1)-i128::from(x0);let dy=i128::from(y1)-i128::from(y0);
                    if dx==0||dy%dx!=0{continue}
                    let slope=dy/dx;let Some(offset)=slope.checked_mul(i128::from(x0)).and_then(|v|i128::from(y0).checked_sub(v)) else{continue};
                    let (Ok(m),Ok(b))=(i64::try_from(slope),i64::try_from(offset)) else{continue};
                    let p=Expr::Add{left:Box::new(Expr::Multiply{left:Box::new(t.clone()),right:Box::new(Expr::Const{value:json!(m)})}),right:Box::new(Expr::Const{value:json!(b)})};
                    if let Some(p)=candidate(p)?{found=Some(p);break 'outer;}
                }
            }
        }
        if found.is_none(){
            'outer:for a in &terms{for b in &terms{
                for p in [Expr::Add{left:Box::new(a.clone()),right:Box::new(b.clone())},Expr::Multiply{left:Box::new(a.clone()),right:Box::new(b.clone())},Expr::Equal{left:Box::new(a.clone()),right:Box::new(b.clone())},Expr::Concat{items:vec![a.clone(),b.clone()]}]{
                    if let Some(p)=candidate(p)?{found=Some(p);break 'outer;}
                }
            }}
        }
        let program=found.ok_or("no program in bounded discovery grammar")?;
        // Release the closure's mutable borrow of the counter before returning it.
        drop(candidate);
        ensure(holdout.iter().all(|(x,y)|program.eval(x).is_ok_and(|v|v==*y)),"candidate failed independent holdout")?;
        Ok(Discovery{family:family.into(),training,holdouts,candidates_evaluated:n,program,input,output})
    }
    /// Atomically generate a verified class and capability from a bounded discovery.
    /// A failed validation changes nothing.
    pub fn assimilate(&mut self,id_:&str,version:u64,d:Discovery)->Result<Id>{
        let mut next=self.clone();let k=version_key(id_,version)?;
        ensure(!next.operators.contains_key(&k)&&!next.classes.contains_key(&k)&&!next.capabilities.contains_key(&k),"version exists")?;
        let parent=next.capabilities.values().filter(|c|c.id==id_&&c.version<version).max_by_key(|c|c.version).map(|c|c.class.clone());
        next.operators.insert(k.clone(),OperatorContract{id:id_.into(),version,behavior:Operator::Learn,input:d.input.clone(),output:d.output.clone(),program:d.program});
        let class=Class{id:id_.into(),version,input:d.input,output:d.output,nodes:vec![Node{id:"learned".into(),operator:k.clone(),input:Wire::Input}],result:Wire::Node{id:"learned".into()},lineage:parent.into_iter().collect()};
        next.validate_class(&class)?;next.classes.insert(k.clone(),class);
        next.admit_capability(Capability{id:id_.into(),version,family:d.family,class:k.clone(),training:d.training,holdouts:d.holdouts,enabled:true})?;
        *self=next;Ok(k)
    }
}
