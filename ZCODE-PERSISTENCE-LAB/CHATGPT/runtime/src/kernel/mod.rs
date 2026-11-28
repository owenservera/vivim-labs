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
pub fn native_operator_contracts()->Result<Vec<OperatorContract>>{
    let operators=[
        ("observe",Operator::Observe),("discover",Operator::Discover),("reason",Operator::Reason),
        ("decompose",Operator::Decompose),("plan",Operator::Plan),("construct",Operator::Construct),
        ("execute",Operator::Execute),("experiment",Operator::Experiment),("challenge",Operator::Challenge),
        ("verify",Operator::Verify),("repair",Operator::Repair),("synthesize",Operator::Synthesize),
        ("optimize",Operator::Optimize),("delegate",Operator::Delegate),("learn",Operator::Learn),
    ];
    Ok(operators.into_iter().map(|(name,behavior)|OperatorContract{
        id:format!("native-{name}"),version:1,behavior:behavior.clone(),input:Shape::Any,
        output:Shape::Any,program:Expr::Behavior{operator:behavior,source:Box::new(Expr::Input)}
    }).collect())
}
