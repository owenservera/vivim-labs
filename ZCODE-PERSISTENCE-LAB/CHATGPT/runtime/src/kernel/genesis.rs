//! Atomic deterministic bootstrap. No inference or external effect authority.
use super::{model::*,transition::{Actor,Action}};
use crate::{Expr,Operator,digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::collections::{BTreeMap,BTreeSet};
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Manifest{pub id:Id,pub objective:String,pub tracker_digest:Digest,pub input:i64,pub claim_budget:u64}
impl World{
    pub fn initialize_genesis(&mut self,m:Manifest)->Result<Value>{
        id(&m.id)?;ensure(m.id.len()<=60&&hash_id(&m.tracker_digest)&&!m.objective.trim().is_empty()&&(4..=100).contains(&m.claim_budget),"invalid genesis manifest")?;
        ensure(self.goals.is_empty()&&self.tasks.is_empty()&&self.agents.is_empty()&&self.router.config.is_none(),"genesis requires fresh non-routed runtime")?;
        let first=m.input.checked_add(1).ok_or("genesis input overflow")?;
        let second=first.checked_add(1).ok_or("genesis input overflow")?;
        let actor=Actor::Host;
        for contract in super::native_operator_contracts()?{
            let key=version_key(&contract.id,contract.version)?;
            if let Some(old)=self.operators.get(&key){
                ensure(serde_json::to_value(old).map_err(|e|e.to_string())?==serde_json::to_value(&contract).map_err(|e|e.to_string())?,"native operator conflict")?;
            }else{self.apply_adapter_action(&actor,Action::Operator{contract})?;}
        }
        self.apply_adapter_action(&actor,Action::Slots{count:1})?;
        let goal=m.id.clone();let context=format!("{goal}-context");let class=format!("{goal}-increment");let a=format!("{goal}-first");let b=format!("{goal}-second");
        let input=Shape::Record{fields:BTreeMap::from([("x".into(),Shape::Integer)])};
        self.apply_adapter_action(&actor,Action::Goal{id:goal.clone(),parent:None,objective:m.objective,claim_budget:m.claim_budget,predicate:Some(Oracle::Digest{expected:digest(&json!({(a.clone()):first,(b.clone()):second}))})})?;
        self.apply_adapter_action(&actor,Action::Context{context:Context{id:context.clone(),version:1,entries:BTreeMap::from([("tracker-digest".into(),json!(m.tracker_digest))]),artifacts:BTreeMap::new(),byte_budget:MAX_VALUE_BYTES}})?;
        self.apply_adapter_action(&actor,Action::Operator{contract:OperatorContract{id:class.clone(),version:1,behavior:Operator::Execute,input:input.clone(),output:Shape::Integer,program:Expr::Add{left:Box::new(Expr::Field{source:Box::new(Expr::Input),key:"x".into()}),right:Box::new(Expr::Const{value:json!(1)})}}})?;
        let ck=version_key(&class,1)?;let cx=version_key(&context,1)?;
        self.apply_adapter_action(&actor,Action::Class{class:Class{id:class,version:1,input,output:Shape::Integer,nodes:vec![Node{id:"increment".into(),operator:ck.clone(),input:Wire::Input}],result:Wire::Node{id:"increment".into()},lineage:vec![]}})?;
        for (key,dependency,binding,expected) in [(a.clone(),BTreeSet::new(),Binding::Literal{value:json!(m.input)},first),(b.clone(),BTreeSet::from([a.clone()]),Binding::Predecessor{task:a.clone()},second)]{
            self.apply_adapter_action(&actor,Action::Task{spec:TaskSpec{id:key,goal:goal.clone(),objective:"native bootstrap increment with independent digest oracle".into(),dependencies:dependency,inputs:BTreeMap::from([("x".into(),Port{shape:Shape::Integer,binding})]),output:Shape::Integer,oracle:Oracle::Digest{expected:digest(&json!(expected))},class:Some(ck.clone()),family:None,context:Some(cx.clone()),max_attempts:2,priority:100,uncertainty:0,information_gain:0,expected_cost:1}})?;
        }
        self.apply_adapter_action(&actor,Action::Agent{id:format!("{goal}-agent"),goal:goal.clone(),parent:None,class:ck,context:cx})?;
        self.apply_adapter_action(&actor,Action::Seal{goal:goal.clone()})?;
        Ok(json!({"goal":goal,"tasks":[a,b],"slots":1,"provider_calls":0,"execution":"not_started"}))
    }
}
