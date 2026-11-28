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
    Record { fields: BTreeMap<String, Shape> }, Map { value:Box<Shape> }, OneOf { choices:Vec<Shape> }
}
impl Shape {
    pub fn accepts(&self, v: &Value) -> bool {
        match self {
            Self::Any => true, Self::Null => v.is_null(), Self::Bool => v.is_boolean(),
            Self::Integer => v.as_i64().is_some(), Self::String => v.is_string(),
            Self::Array{item} => v.as_array().is_some_and(|a| a.iter().all(|x|item.accepts(x))),
            Self::Map{value}=>v.as_object().is_some_and(|o|o.values().all(|x|value.accepts(x))),
            Self::OneOf{choices}=>choices.iter().any(|s|s.accepts(v)),
            Self::Record{fields} => v.as_object().is_some_and(|o| o.len()==fields.len() &&
                fields.iter().all(|(k,s)|o.get(k).is_some_and(|x|s.accepts(x))))
        }
    }
    /// Conservative compatibility; Any is not a proof that a concrete target accepts a source.
    pub fn admits(&self, source:&Shape)->bool {
        if self==&Self::Any || self==source { return true; }
        match (self,source) {
            (_,Self::OneOf{choices})=>choices.iter().all(|s|self.admits(s)),
            (Self::OneOf{choices},_)=>choices.iter().any(|s|s.admits(source)),
            (Self::Map{value:a},Self::Map{value:b})=>a.admits(b),
            (Self::Map{value:a},Self::Record{fields})=>fields.values().all(|s|a.admits(s)),
            (Self::Array{item:a},Self::Array{item:b})=>a.admits(b),
            (Self::Record{fields:a},Self::Record{fields:b})=>a.len()==b.len()&&a.iter().all(|(k,s)|b.get(k).is_some_and(|x|s.admits(x))),
            _=>false
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="source",rename_all="snake_case")]
pub enum Binding { Literal{value:Value}, Predecessor{task:Id}, Context{key:String}, Knowledge{version:Id}, SelectKnowledge{kind:KnowledgeKind} }
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
pub enum TaskPhase { Pending, Running, Submitted, Reconciling, Verified, Exhausted, Blocked, Cancelled }
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
    pub class:Option<Id>, pub family:Option<Id>, #[serde(default)] pub context:Option<Id>, pub max_attempts:u32,
    pub priority:i64, pub uncertainty:u32, #[serde(default)] pub information_gain:u32, pub expected_cost:u32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lease { pub worker:Id, pub incarnation:u64, pub generation:u64, pub deadline_ms:u64 }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub spec:TaskSpec, pub version:u64, pub phase:TaskPhase, pub attempts:u32,
    pub generation:u64, pub lease:Option<Lease>, pub input:Option<Value>,
    pub proposal:Option<Digest>, pub artifact:Option<Id>, pub reason:Option<String>,
    pub assigned_class:Option<Id>, pub assigned_context:Option<Id>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    pub id:Id, pub version:u64, pub entries:BTreeMap<String,Value>,
    pub artifacts:BTreeMap<String,Id>, pub byte_budget:usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Incarnation {
    pub number:u64, pub executor:String, pub started_ms:u64,
    pub ended_ms:Option<u64>, pub reason:Option<String>, pub process_ref:Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id:Id, pub goal:Id, pub parent:Option<Id>, pub class:Id, pub context:Id,
    pub phase:AgentPhase, pub incarnations:Vec<Incarnation>, pub wait_task:Option<Id>,
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
    pub verifier:Id, pub parents:Vec<Id>, pub clock_ms:u64, pub class:Option<Id>, pub context:Option<Id>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Checkpoint {
    pub version:u64, pub generation:u64, pub worker:Id, pub incarnation:u64,
    pub payload:Digest, pub previous:Option<Digest>, pub input:Digest,
    pub task_version:u64,pub class:Option<Id>,pub context:Option<Id>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id:Id, pub sender:Id, pub recipient:Id, pub body:Digest,
    pub sequence:u64, pub acknowledged:bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Delivery {
    pub message:Id,pub recipient:Id,pub attempts:u32,pub last_incarnation:Option<u64>,
    pub lease_until_ms:u64,pub acknowledged_ms:Option<u64>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rollback {
    pub capability:Id,pub reason:String,pub fallback:Option<Id>,pub sequence:u64,pub affected_tasks:Vec<Id>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttemptRecord {
    pub task:Id,pub generation:u64,pub agent:Id,pub incarnation:u64,pub class:Id,
    pub started_ms:u64,pub ended_ms:Option<u64>,pub outcome:Option<String>,
    pub measured_ms:Option<u64>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LearningCost {
    pub plan:Id,pub evaluated:u64,pub training_evaluations:u64,pub holdout_evaluations:u64,pub accepted:bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperatorContract {
    pub id:Id, pub version:u64, pub behavior:Operator,
    pub input:Shape, pub output:Shape, pub program:Expr,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="source",rename_all="snake_case")]
pub enum Wire { Input, Node { id:Id }, Field { source:Box<Wire>, key:String }, Record { fields:BTreeMap<String,Wire> }, Array { items:Vec<Wire> }, Literal { value:Value, shape:Shape } }
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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all="snake_case")]
pub enum EffectPhase { Proposed, Prepared, Unknown, Committed, Absent, Abandoned }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Effect {
    pub id:Id,pub task:Id,pub generation:u64,pub key:Id,pub payload:Digest,
    pub phase:EffectPhase,pub oracle:Oracle,pub receipt:Option<Digest>,
    pub sequence:u64,pub history:Vec<EffectEvent>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EffectEvent {
    pub phase:EffectPhase,pub sequence:u64,pub reason:String,pub receipt:Option<Digest>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LearningPlan {
    pub id:Id,pub version:u64,pub family:Id,pub input:Shape,pub output:Shape,
    pub training:Vec<Id>,pub holdouts:Vec<Id>,pub search_limit:u64,
    pub capability:Id,pub capability_version:u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LearningAttempt {
    pub plan:Id,pub accepted:bool,pub capability:Option<Id>,pub reason:Option<String>,
    pub evaluated:Option<u64>,pub holdout_inputs:Vec<Digest>,pub clock_ms:u64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all="snake_case")]
pub enum KnowledgeKind { Procedure, Repair, Class, Coordination, Context, Fact, Proof, ComputationalOperator }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Knowledge {
    pub id:Id,pub version:u64,pub kind:KnowledgeKind,pub family:Option<Id>,
    pub artifacts:Vec<Id>,pub class:Option<Id>,pub evidence:Vec<Id>,
    pub supersedes:Option<Id>,pub enabled:bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KnowledgeRule {
    pub id:Id,pub version:u64,pub family:Id,pub kind:KnowledgeKind,pub output:Shape,
    pub minimum_examples:usize,pub max_publications:usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KnowledgePublication {
    pub rule:Id,pub knowledge:Id,pub payload:Digest,pub examples:Vec<Id>,pub sequence:u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunCost {
    pub task:Id,pub generation:u64,pub elapsed_ms:u64,pub family:Option<Id>,pub class:Option<Id>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchedulingPolicy {
    pub version:u64,pub priority_weight:u32,pub critical_weight:u32,
    pub uncertainty_weight:u32,pub information_weight:u32,pub cost_weight:u32,
}
impl Default for SchedulingPolicy {
    fn default()->Self{Self{version:1,priority_weight:1000,critical_weight:100,uncertainty_weight:10,information_weight:20,cost_weight:1}}
}
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Metrics {
    pub claims:u64, pub accepted:u64, pub rejected:u64, pub failures:u64,
    pub deterministic_runs:u64, pub reused:u64, pub learned:u64,
    pub candidate_evaluations:u64, pub worker_elapsed_ms:u64, pub verification_rejections:u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct World {
    pub clock_ms:u64, pub slots:usize, pub sequence:u64,
    pub goals:super::shared_map::SharedMap<Goal>, pub tasks:super::shared_map::SharedMap<Task>,
    pub agents:super::shared_map::SharedMap<Agent>, pub contexts:BTreeMap<Id,Context>,
    pub operators:BTreeMap<Id,OperatorContract>, pub classes:BTreeMap<Id,Class>,
    pub artifacts:super::shared_map::SharedMap<Artifact>, pub evidence:super::shared_map::SharedMap<Evidence>,
    pub blobs:super::shared_map::SharedMap<Value>, pub checkpoints:BTreeMap<Id,Vec<Checkpoint>>,
    pub messages:BTreeMap<Id,Message>, pub capabilities:BTreeMap<Id,Capability>,
    pub metrics:Metrics,
    pub learning_plans:BTreeMap<Id,LearningPlan>,pub learning_attempts:BTreeMap<Id,LearningAttempt>,
    pub knowledge:BTreeMap<Id,Knowledge>,pub run_costs:super::shared_map::SharedMap<RunCost>,pub policy:SchedulingPolicy,
    pub goal_versions:BTreeMap<Id,Vec<Goal>>,pub task_versions:BTreeMap<Id,Vec<TaskSpec>>,
    pub effects:BTreeMap<Id,Effect>,
    #[serde(default)] pub delivery:BTreeMap<Id,Delivery>,
    #[serde(default)] pub rollback_history:BTreeMap<Id,Rollback>,
    #[serde(default)] pub attempt_history:super::shared_map::SharedMap<AttemptRecord>,
    #[serde(default)] pub learning_costs:BTreeMap<Id,LearningCost>,
    #[serde(default)] pub knowledge_rules:BTreeMap<Id,KnowledgeRule>,
    #[serde(default)] pub knowledge_publications:BTreeMap<Id,KnowledgePublication>,
    #[serde(default)] pub custody_families:BTreeSet<Id>,
    #[serde(default)] pub cohorts:BTreeMap<Id,super::custody::Cohort>,
    #[serde(default)] pub frozen_candidates:BTreeMap<Id,super::custody::FrozenCandidate>,
    #[serde(default)] pub custody_receipts:BTreeMap<Id,super::custody::CustodyReceipt>,
    #[serde(default,skip_serializing_if="super::routing::Router::is_empty")] pub router:super::routing::Router,
    #[serde(default,skip_serializing_if="BTreeMap::is_empty")] pub recipe_policies:BTreeMap<Id,super::recipes::Policy>,
    #[serde(default,skip_serializing_if="BTreeMap::is_empty")] pub recipes:BTreeMap<Id,super::recipes::Recipe>,
    #[serde(default,skip_serializing_if="BTreeMap::is_empty")] pub recipe_scans:BTreeMap<Id,u64>,
    #[serde(default,skip_serializing_if="super::shared_map::SharedMap::is_empty")] pub resource_receipts:super::shared_map::SharedMap<super::resource::Receipt>,
    #[serde(default,skip_serializing_if="BTreeMap::is_empty")] pub recipe_repairs:BTreeMap<Id,super::recipes::VerifiedRepair>,
    #[serde(default,skip_serializing_if="super::shared_map::SharedMap::is_empty")] pub host_costs:super::shared_map::SharedMap<super::resource::HostCost>,
    #[serde(default,skip_serializing_if="super::assimilation::Registry::is_empty")] pub assimilation:super::assimilation::Registry,
    #[serde(default,skip_serializing_if="super::reuse_economics::Registry::is_empty")] pub reuse_economics:super::reuse_economics::Registry,
    #[serde(skip)] #[doc(hidden)] pub derived:super::derived::Cache,
    pub legacy_import:Option<super::migration::LegacyArchive>,
}
impl World{
    /// Reset dirty keys only after a successfully committed index root.
    pub(crate) fn mark_history_clean(&mut self){
        self.goals.mark_clean();self.tasks.mark_clean();self.agents.mark_clean();
        self.blobs.mark_clean();self.artifacts.mark_clean();self.evidence.mark_clean();
        self.attempt_history.mark_clean();self.run_costs.mark_clean();
        self.resource_receipts.mark_clean();self.host_costs.mark_clean();
    }
}
impl Default for World {
    fn default()->Self {
        Self{clock_ms:0,slots:4,sequence:0,goals:super::shared_map::SharedMap::default(),tasks:super::shared_map::SharedMap::default(),agents:super::shared_map::SharedMap::default(),contexts:BTreeMap::new(),operators:BTreeMap::new(),classes:BTreeMap::new(),artifacts:super::shared_map::SharedMap::default(),evidence:super::shared_map::SharedMap::default(),blobs:super::shared_map::SharedMap::default(),checkpoints:BTreeMap::new(),messages:BTreeMap::new(),capabilities:BTreeMap::new(),metrics:Metrics::default(),learning_plans:BTreeMap::new(),learning_attempts:BTreeMap::new(),knowledge:BTreeMap::new(),run_costs:super::shared_map::SharedMap::default(),policy:SchedulingPolicy::default(),goal_versions:BTreeMap::new(),task_versions:BTreeMap::new(),effects:BTreeMap::new(),delivery:BTreeMap::new(),rollback_history:BTreeMap::new(),attempt_history:super::shared_map::SharedMap::default(),learning_costs:BTreeMap::new(),knowledge_rules:BTreeMap::new(),knowledge_publications:BTreeMap::new(),custody_families:BTreeSet::new(),cohorts:BTreeMap::new(),frozen_candidates:BTreeMap::new(),custody_receipts:BTreeMap::new(),router:super::routing::Router::default(),recipe_policies:BTreeMap::new(),recipes:BTreeMap::new(),recipe_scans:BTreeMap::new(),resource_receipts:super::shared_map::SharedMap::default(),recipe_repairs:BTreeMap::new(),host_costs:super::shared_map::SharedMap::default(),assimilation:super::assimilation::Registry::default(),reuse_economics:super::reuse_economics::Registry::default(),derived:super::derived::Cache::default(),legacy_import:None}
    }
}
pub fn version_key(name:&str,version:u64)->Result<Id>{
    id(name)?;ensure(version>0,"version must be positive")?;Ok(format!("{name}@{version}"))
}
