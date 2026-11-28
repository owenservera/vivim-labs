//! R004 provider-neutral computational substrate. The R003 interface remains
//! intact for reproducibility; the new kernel has its own durable store.
pub mod model;
pub mod composition;
pub mod transition;
pub mod evolution;
pub mod persistence;
pub mod supervisor;
pub mod behavior;

use crate::{Expr,Operator,Result};
use model::{OperatorContract,Shape};
use std::collections::BTreeMap;
fn record(fields: &[(&str,Shape)]) -> Shape { Shape::Record{fields:fields.iter().map(|(k,v)|(k.to_string(),v.clone())).collect::<BTreeMap<_,_>>()} }
fn array(item:Shape)->Shape{Shape::Array{item:Box::new(item)}}
fn nullable(s:Shape)->Shape{Shape::OneOf{choices:vec![s,Shape::Null]}}
fn signature(op:&Operator)->(Shape,Shape){
    use Shape::*;
    let binary=record(&[("op",String),("left",Any),("right",Any)]);
    let alternative=record(&[("id",String),("cost",Integer)]);
    let example=record(&[("input",Integer),("output",Integer)]);
    match op {
        Operator::Observe=>(record(&[("value",Any)]),record(&[("digest",String),("shape",String),("bytes",Integer)])),
        Operator::Discover=>(record(&[("value",Any),("candidates",array(Any))]),array(Integer)),
        Operator::Reason=>(record(&[("facts",array(String)),("rules",array(record(&[("requires",array(String)),("concludes",String)])))]),array(String)),
        Operator::Decompose=>(record(&[("batch_size",Integer),("items",array(Any))]),array(array(Any))),
        Operator::Plan=>(record(&[("tasks",array(record(&[("id",String),("dependencies",array(String))])))]),array(String)),
        Operator::Construct=>(record(&[("fields",array(record(&[("key",String),("value",Any)])))]),Map{value:Box::new(Any)}),
        Operator::Execute=>(binary,OneOf{choices:vec![Integer,Bool,String]}),
        Operator::Experiment=>(record(&[("cases",array(binary))]),array(OneOf{choices:vec![record(&[("ok",Bool),("value",Any)]),record(&[("ok",Bool),("reason",String)])]})),
        Operator::Challenge=>(record(&[("cases",array(record(&[("op",String),("left",Any),("right",Any),("expected",Any)])))]),array(OneOf{choices:vec![record(&[("index",Integer),("expected",Any),("actual",Any)]),record(&[("index",Integer),("reason",String)])]})),
        Operator::Verify=>(OneOf{choices:vec![record(&[("actual",Any),("expected",Any)]),record(&[("actual",Any),("expected_digest",String)])]},record(&[("accepted",Bool),("digest",String)])),
        Operator::Repair=>(record(&[("excluded",array(String)),("alternatives",array(alternative.clone()))]),nullable(alternative.clone())),
        Operator::Synthesize=>(record(&[("records",array(Map{value:Box::new(Any)}))]),Map{value:Box::new(Any)}),
        Operator::Optimize=>{let candidate=record(&[("id",String),("cost",Integer),("feasible",Bool)]);(record(&[("candidates",array(candidate.clone()))]),nullable(candidate))},
        Operator::Delegate=>(record(&[("slots",Integer),("jobs",array(alternative))]),array(record(&[("job",String),("slot",Integer)]))),
        Operator::Learn=>(record(&[("examples",array(example))]),record(&[("slope",Integer),("offset",Integer)])),
    }
}
pub fn native_operator_contracts()->Result<Vec<OperatorContract>>{
    let operators=[
        ("observe",Operator::Observe),("discover",Operator::Discover),("reason",Operator::Reason),
        ("decompose",Operator::Decompose),("plan",Operator::Plan),("construct",Operator::Construct),
        ("execute",Operator::Execute),("experiment",Operator::Experiment),("challenge",Operator::Challenge),
        ("verify",Operator::Verify),("repair",Operator::Repair),("synthesize",Operator::Synthesize),
        ("optimize",Operator::Optimize),("delegate",Operator::Delegate),("learn",Operator::Learn),
    ];
    Ok(operators.into_iter().map(|(name,behavior)|{let (input,output)=signature(&behavior);OperatorContract{
        id:format!("native-{name}"),version:1,behavior:behavior.clone(),input,output,
        program:Expr::Behavior{operator:behavior,source:Box::new(Expr::Input)}
    }}).collect())
}
