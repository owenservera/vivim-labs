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
    /// Public compatibility wrapper; the host retains a measured receipt even
    /// when synthesis or independent evaluation fails.
    pub fn discover(&self,family:&str,training:Vec<Id>,holdouts:Vec<Id>,input:Shape,output:Shape,limit:u64)->Result<Discovery>{
        self.discover_measured(family,training,holdouts,input,output,limit).0
    }
    pub fn discover_measured(&self,family:&str,training:Vec<Id>,holdouts:Vec<Id>,input:Shape,output:Shape,limit:u64)->(Result<Discovery>,SearchReceipt){
        let mut receipt=SearchReceipt::default();
        let result=(||->Result<Discovery>{
            ensure((1..=100_000).contains(&limit),"invalid discovery budget")?;
            let (train,holdout)=self.examples(family,&training,&holdouts,&input,&output)?;
            // The synthesis engine receives training only. Holdouts enter the
            // evaluator once, after the candidate has been irrevocably selected.
            let program=synthesize(&input,&output,&train,limit,&mut receipt)?;
            for (x,y) in &holdout{
                receipt.holdout_evaluations+=1;
                ensure(program.eval(x).is_ok_and(|v|v==*y),"candidate failed independent holdout")?;
            }
            Ok(Discovery{family:family.into(),training,holdouts,candidates_evaluated:receipt.candidates_evaluated,program,input,output})
        })();
        (result,receipt)
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
        let c=next.capabilities[&k].clone();
        let artifacts:Vec<_>=c.training.iter().chain(c.holdouts.iter()).map(|t|next.tasks[t].artifact.clone().ok_or("artifact missing")).collect::<std::result::Result<_,_>>()?;
        let evidence=artifacts.iter().map(|a|next.artifacts[a].evidence.clone()).collect();
        let supersedes=next.knowledge.values().filter(|x|x.id==id_&&x.version<version&&x.kind==KnowledgeKind::ComputationalOperator).max_by_key(|x|x.version).map(|x|format!("{}@{}",x.id,x.version));
        next.publish_knowledge(Knowledge{id:id_.into(),version,kind:KnowledgeKind::ComputationalOperator,family:Some(c.family),artifacts,class:Some(k.clone()),evidence,supersedes,enabled:true})?;
        *self=next;Ok(k)
    }
}


#[derive(Clone,Debug,Serialize,Deserialize,Default)]
pub struct SearchReceipt{
    pub candidates_evaluated:u64,pub training_evaluations:u64,pub holdout_evaluations:u64,
}
/// Observational equivalence pruning across the training cohort. The grammar
/// is finite: projections (including nested fields), constants, hash,
/// arithmetic, equality, concatenation, arrays and conditionals. A strict
/// candidate/retained-term budget bounds the enumeration.
fn synthesize(input:&Shape,output:&Shape,train:&[(Value,Value)],limit:u64,receipt:&mut SearchReceipt)->Result<Expr>{
    const MAX_TERMS:usize=128;
    const MAX_ROUNDS:usize=4;
    fn fields(shape:&Shape,source:&Expr,depth:usize,terms:&mut Vec<Expr>){
        if depth>=4||terms.len()>=MAX_TERMS{return;}
        if let Shape::Record{fields:items}=shape{
            for (key,shape) in items{
                if terms.len()>=MAX_TERMS{break;}
                let field=Expr::Field{source:Box::new(source.clone()),key:key.clone()};
                terms.push(field.clone());fields(shape,&field,depth+1,terms);
            }
        }
    }
    fn candidate(p:Expr,train:&[(Value,Value)],output:&Shape,limit:u64,receipt:&mut SearchReceipt,
        seen:&mut BTreeSet<String>,pool:&mut Vec<Expr>)->Result<Option<Expr>>{
        if receipt.candidates_evaluated>=limit{return Ok(None);}
        receipt.candidates_evaluated+=1;
        let mut values=Vec::with_capacity(train.len());let mut exact=true;
        for (x,y) in train{
            receipt.training_evaluations+=1;
            let Ok(v)=p.eval(x) else{return Ok(None);};
            exact &= output.accepts(&v)&&v==*y;values.push(v);
        }
        if exact{return Ok(Some(p));}
        // A fingerprint is only a search pruning heuristic, not proof that two
        // expressions are equivalent outside the training cohort.
        let fingerprint=digest(&Value::Array(values));
        if pool.len()<MAX_TERMS&&seen.insert(fingerprint){pool.push(p);}
        Ok(None)
    }
    let mut initial=vec![Expr::Input];fields(input,&Expr::Input,0,&mut initial);
    for n in [-2,-1,0,1,2,3,10]{initial.push(Expr::Const{value:json!(n)});}
    initial.push(Expr::Const{value:json!(false)});initial.push(Expr::Const{value:json!(true)});
    initial.push(Expr::Const{value:json!("")});initial.push(Expr::Const{value:Value::Null});
    initial.push(Expr::Const{value:train[0].1.clone()});
    let mut pool=Vec::new();let mut seen=BTreeSet::new();
    macro_rules! consider{($p:expr)=>{{
        if receipt.candidates_evaluated>=limit{return Err("discovery candidate budget exhausted".into());}
        if let Some(p)=candidate($p,train,output,limit,receipt,&mut seen,&mut pool)?{return Ok(p);}
    }}}
    for p in initial{consider!(p);}
    // Exact affine coefficient inference is prioritized before generic search.
    for term in pool.clone(){
        let Ok(v)=term.eval(&train[0].0) else{continue;};
        let (Some(x0),Some(y0))=(v.as_i64(),train[0].1.as_i64()) else{continue;};
        for (input,output) in train.iter().skip(1){
            let Ok(v)=term.eval(input) else{continue;};
            let (Some(x1),Some(y1))=(v.as_i64(),output.as_i64()) else{continue;};
            let dx=i128::from(x1)-i128::from(x0);let dy=i128::from(y1)-i128::from(y0);
            if dx==0||dy%dx!=0{continue;}
            let m=dy/dx;
            let Some(b)=m.checked_mul(i128::from(x0)).and_then(|v|i128::from(y0).checked_sub(v)) else{continue;};
            let (Ok(m),Ok(b))=(i64::try_from(m),i64::try_from(b)) else{continue;};
            consider!(Expr::Add{left:Box::new(Expr::Multiply{left:Box::new(term.clone()),right:Box::new(Expr::Const{value:json!(m)})}),right:Box::new(Expr::Const{value:json!(b)})});
        }
    }
    let mut previous=0;
    for _ in 0..MAX_ROUNDS{
        let terms=pool.clone();let len=terms.len();
        if len==previous{break;}
        for (i,a) in terms.iter().enumerate().skip(previous){
            consider!(Expr::Sha256{value:Box::new(a.clone())});
            consider!(Expr::Array{items:vec![a.clone()]});
            for (j,b) in terms.iter().enumerate(){
                if j>=previous&&j>i{continue;}
                consider!(Expr::Add{left:Box::new(a.clone()),right:Box::new(b.clone())});
                consider!(Expr::Multiply{left:Box::new(a.clone()),right:Box::new(b.clone())});
                consider!(Expr::Equal{left:Box::new(a.clone()),right:Box::new(b.clone())});
                consider!(Expr::Concat{items:vec![a.clone(),b.clone()]});
                consider!(Expr::Concat{items:vec![b.clone(),a.clone()]});
                consider!(Expr::Array{items:vec![a.clone(),b.clone()]});
                consider!(Expr::Array{items:vec![b.clone(),a.clone()]});
            }
        }
        // Small conditional grammar; avoid cubic enumeration across the full pool.
        let conditions:Vec<_>=terms.iter().filter(|p|train.iter().all(|(x,_)|p.eval(x).is_ok_and(|v|v.is_boolean()))).take(8).collect();
        for cond in conditions{for yes in terms.iter().take(12){for no in terms.iter().take(12){
            consider!(Expr::If{condition:Box::new(cond.clone()),yes:Box::new(yes.clone()),no:Box::new(no.clone())});
        }}}
        previous=len;
    }
    Err("no solution in bounded discovery grammar".into())
}
