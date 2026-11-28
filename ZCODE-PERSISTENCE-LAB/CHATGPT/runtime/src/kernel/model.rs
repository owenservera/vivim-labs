//! Versioned domain contracts for the provider-independent runtime.
use crate::{Expr, Operator, digest, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub type Id = String;
pub type Digest = String;
pub const MAX_VALUE_BYTES: usize = 1_048_576;
pub const MAX_GRAPH_NODES: usize = 256;
pub fn ensure(condition: bool, message: &str) -> Result<()> {
    if condition { Ok(()) } else { Err(message.into()) }
}
pub fn id(s: &str) -> Result<()> {
    ensure(!s.is_empty() && s.len() <= 100 && s.bytes().all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b)), "invalid identifier")
}
pub fn bounded(v: &Value) -> Result<()> {
    ensure(serde_json::to_vec(v).map_err(|e| e.to_string())?.len() <= MAX_VALUE_BYTES, "value budget exceeded")
}
pub fn hash_id(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag="kind",rename_all="snake_case")]
pub enum Shape {
    Any, Null, Bool, Integer, String, Array { item: Box<Shape> },
    Record { fields: BTreeMap<String, Shape> }
}
impl Shape {
    pub fn accepts(&self, v: &Value) -> bool {
        match self {
            Self::Any => true, Self::Null => v.is_null(), Self::Bool => v.is_boolean(),
            Self::Integer => v.as_i64().is_some(), Self::String => v.is_string(),
            Self::Array{item} => v.as_array().is_some_and(|a| a.iter().all(|x|item.accepts(x))),
            Self::Record{fields} => v.as_object().is_some_and(|o| o.len()==fields.len() &&
                fields.iter().all(|(k,s)|o.get(k).is_some_and(|x|s.accepts(x))))
        }
    }
    /// Conservative compatibility; Any is not a proof that a concrete target accepts a source.
    pub fn admits(&self, source:&Shape)->bool {
        if self==&Self::Any || self==source { return true; }
        match (self,source) {
            (Self::Array{item:a},Self::Array{item:b})=>a.admits(b),
            (Self::Record{fields:a},Self::Record{fields:b})=>a.len()==b.len()&&a.iter().all(|(k,s)|b.get(k).is_some_and(|x|s.admits(x))),
            _=>false
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="source",rename_all="snake_case")]
pub enum Binding { Literal{value:Value}, Predecessor{task:Id}, Context{key:String} }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Port { pub shape:Shape, pub binding:Binding }
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="kind",rename_all="snake_case")]
pub enum Oracle {
    Digest { expected:Digest },
    Predicate { version:u64, expression:Expr },
    All { children:Vec<Oracle> },
}
impl Oracle {
    pub fn validate(&self,depth:usize)->Result<()>{
        ensure(depth<16,"oracle depth exceeded")?;
        match self {
            Self::Digest{expected}=>ensure(hash_id(expected),"invalid expected digest"),
            Self::Predicate{version,..}=>ensure(*version>0,"oracle version required"),
            Self::All{children}=>{
                ensure(!children.is_empty()&&children.len()<=32,"invalid oracle composition")?;
                for c in children {c.validate(depth+1)?;} Ok(())
            }
        }
    }
    pub fn evaluate(&self,input:&Value,output:&Value)->Result<bool>{
        match self {
            Self::Digest{expected}=>Ok(digest(output)==*expected),
            Self::Predicate{expression,..}=>expression.eval(&serde_json::json!({"input":input,"output":output}))?.as_bool().ok_or("oracle must produce boolean".into()),
            Self::All{children}=>{for c in children {if !c.evaluate(input,output)? {return Ok(false);}} Ok(true)}
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all="snake_case")]
pub enum GoalPhase { Open, Complete, Exhausted, Blocked, Cancelled }
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all="snake_case")]
pub enum TaskPhase { Pending, Running, Submitted, Verified, Exhausted, Blocked, Cancelled }
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all="snake_case")]
pub enum AgentPhase { Dormant, WaitingDependency, WaitingMessage, Running, Recovering, Archived }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Goal {
    pub id:Id, pub parent:Option<Id>, pub objective:String, pub version:u64,
    pub predicate:Option<Oracle>, pub sealed:bool, pub phase:GoalPhase,
    pub claim_budget:u64, pub claims:u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskSpec {
    pub id:Id, pub goal:Id, pub objective:String, pub dependencies:BTreeSet<Id>,
    pub inputs:BTreeMap<String,Port>, pub output:Shape, pub oracle:Oracle,
    pub class:Option<Id>, pub family:Option<Id>, pub max_attempts:u32,
    pub priority:i64, pub uncertainty:u32, pub expected_cost:u32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lease { pub worker:Id, pub incarnation:u64, pub generation:u64, pub deadline_ms:u64 }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub spec:TaskSpec, pub version:u64, pub phase:TaskPhase, pub attempts:u32,
    pub generation:u64, pub lease:Option<Lease>, pub input:Option<Value>,
    pub proposal:Option<Digest>, pub artifact:Option<Id>, pub reason:Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    pub id:Id, pub version:u64, pub entries:BTreeMap<String,Value>,
    pub artifacts:BTreeMap<String,Id>, pub byte_budget:usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Incarnation {
    pub number:u64, pub executor:String, pub started_ms:u64,
    pub ended_ms:Option<u64>, pub reason:Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id:Id, pub goal:Id, pub parent:Option<Id>, pub class:Id, pub context:Id,
    pub phase:AgentPhase, pub incarnations:Vec<Incarnation>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artifact {
    pub id:Id, pub version:u64, pub payload:Digest, pub task:Id,
    pub generation:u64, pub inputs:Vec<Id>, pub evidence:Id,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Evidence {
    pub id:Id, pub task:Id, pub generation:u64, pub oracle:Oracle,
    pub input:Digest, pub candidate:Digest, pub accepted:bool,
    pub verifier:Id, pub parents:Vec<Id>, pub clock_ms:u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Checkpoint {
    pub version:u64, pub generation:u64, pub worker:Id, pub incarnation:u64,
    pub payload:Digest, pub previous:Option<Digest>, pub input:Digest,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id:Id, pub sender:Id, pub recipient:Id, pub body:Digest,
    pub sequence:u64, pub acknowledged:bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperatorContract {
    pub id:Id, pub version:u64, pub behavior:Operator,
    pub input:Shape, pub output:Shape, pub program:Expr,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="source",rename_all="snake_case")]
pub enum Wire { Input, Node { id:Id } }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node { pub id:Id, pub operator:Id, pub input:Wire }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Class {
    pub id:Id, pub version:u64, pub input:Shape, pub output:Shape,
    pub nodes:Vec<Node>, pub result:Wire, pub lineage:Vec<Id>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Capability {
    pub id:Id, pub version:u64, pub family:Id, pub class:Id,
    pub training:Vec<Id>, pub holdouts:Vec<Id>, pub enabled:bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Metrics {
    pub claims:u64, pub accepted:u64, pub rejected:u64, pub failures:u64,
    pub deterministic_runs:u64, pub reused:u64, pub learned:u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct World {
    pub clock_ms:u64, pub slots:usize, pub sequence:u64,
    pub goals:BTreeMap<Id,Goal>, pub tasks:BTreeMap<Id,Task>,
    pub agents:BTreeMap<Id,Agent>, pub contexts:BTreeMap<Id,Context>,
    pub operators:BTreeMap<Id,OperatorContract>, pub classes:BTreeMap<Id,Class>,
    pub artifacts:BTreeMap<Id,Artifact>, pub evidence:BTreeMap<Id,Evidence>,
    pub blobs:BTreeMap<Digest,Value>, pub checkpoints:BTreeMap<Id,Vec<Checkpoint>>,
    pub messages:BTreeMap<Id,Message>, pub capabilities:BTreeMap<Id,Capability>,
    pub metrics:Metrics,
}
impl Default for World {
    fn default()->Self {
        Self{clock_ms:0,slots:4,sequence:0,goals:BTreeMap::new(),tasks:BTreeMap::new(),agents:BTreeMap::new(),contexts:BTreeMap::new(),operators:BTreeMap::new(),classes:BTreeMap::new(),artifacts:BTreeMap::new(),evidence:BTreeMap::new(),blobs:BTreeMap::new(),checkpoints:BTreeMap::new(),messages:BTreeMap::new(),capabilities:BTreeMap::new(),metrics:Metrics::default()}
    }
}
pub fn version_key(name:&str,version:u64)->Result<Id>{
    id(name)?;ensure(version>0,"version must be positive")?;Ok(format!("{name}@{version}"))
}
