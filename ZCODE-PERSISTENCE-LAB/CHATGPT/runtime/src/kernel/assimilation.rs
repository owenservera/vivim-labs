//! Generalize verified pure traces into versioned reusable coordination and
//! context contracts. Structural equivalence is not universal semantic proof.
//! No original context values, credentials, agent identity or effect authority
//! are copied. Admission is independent-verifier gated; new work keeps new oracles.
use super::{model::*,transition::{Actor,Action}};
use crate::{digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{json,Value};
use std::collections::{BTreeMap,BTreeSet};
fn val<T:Serialize>(v:&T)->Result<Value>{serde_json::to_value(v).map_err(|e|e.to_string())}
fn pin<T:Serialize>(v:&T)->Result<Digest>{Ok(digest(&val(v)?))}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy{
    pub family:Id,pub version:u64,pub enabled:bool,
    pub min_training_goals:usize,pub min_holdout_goals:usize,
    pub max_steps:usize,pub max_cohorts:usize,pub auto_promote:bool,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cohort{
    pub id:Id,pub family:Id,pub policy_version:u64,
    pub review_goal:Id,pub verifier:Id,
    pub training_goals:Vec<Id>,pub holdout_goals:Vec<Id>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(tag="kind",rename_all="snake_case")]
pub enum Source{Parameter{key:Id},Context{key:Id},Step{index:usize}}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct PortPattern{pub shape:Shape,pub source:Source}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct StepPattern{
    pub class_shape:Digest,pub input:Shape,pub output:Shape,
    pub inputs:BTreeMap<Id,PortPattern>,pub dependencies:BTreeSet<usize>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Pattern{pub family:Id,pub steps:Vec<StepPattern>}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct TracePin{
    pub goal:Id,pub tasks:Vec<Id>,pub evidence:Vec<Id>,pub input_digest:Digest,pub trace_digest:Digest,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Candidate{
    pub id:Id,pub cohort:Id,pub policy:Policy,pub pattern:Option<Pattern>,
    pub pattern_digest:Option<Digest>,pub training:Vec<TracePin>,
    pub rejection:Option<String>,pub sequence:u64,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Verdict{
    pub cohort:Id,pub candidate:Id,pub pattern_digest:Digest,
    pub holdout_goals:Vec<Id>,pub evaluations:usize,pub passed:bool,pub evaluation_digest:Digest,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Review{pub verifier:Id,pub verdict:Verdict,pub sequence:u64}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Promotion{pub candidate:Id,pub enabled:bool,pub sequence:u64,pub disabled_reason:Option<String>}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Reuse{
    pub candidate:Id,pub goal:Id,pub tasks:Vec<Id>,pub class_pins:Vec<Id>,
    pub context_pins:Vec<Id>,pub sequence:u64,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Instantiate{
    pub candidate:Id,pub goal:Id,pub objective:String,pub parameters:BTreeMap<Id,Value>,
    pub classes:Vec<Id>,pub contexts:Vec<Id>,pub oracles:Vec<Oracle>,pub claim_budget:u64,
}
#[derive(Clone,Debug,Serialize,Deserialize,Default)]
pub struct Registry{
    pub policies:BTreeMap<Id,Policy>,pub cohorts:BTreeMap<Id,Cohort>,
    pub candidates:BTreeMap<Id,Candidate>,pub verdicts:BTreeMap<Id,Review>,
    pub promotions:BTreeMap<Id,Promotion>,pub runs:BTreeMap<Id,Reuse>,
}
impl Registry{
    pub fn is_empty(&self)->bool{self.policies.is_empty()&&self.cohorts.is_empty()&&self.candidates.is_empty()&&self.verdicts.is_empty()&&self.promotions.is_empty()&&self.runs.is_empty()}
}
fn source_wire(w:&Wire,ids:&BTreeMap<Id,usize>)->Result<Value>{
    // Serialize the wire first so future wire variants fail closed rather than
    // silently losing semantics. Node identifiers are normalized by ordinal.
    let mut v=val(w)?;
    fn rewrite(v:&mut Value,ids:&BTreeMap<Id,usize>)->Result<()>{
        match v{
            Value::Object(m)=>{
                if m.get("kind").and_then(Value::as_str)==Some("node"){
                    let key=m.get("id").and_then(Value::as_str).ok_or("node wire identity missing")?;
                    let n=ids.get(key).ok_or("non-topological class wire")?;
                    m.insert("id".into(),json!(n));return Ok(());
                }
                // Only recurse into known wire containers, not literal values.
                for (k,x) in m.iter_mut(){if k=="fields"||k=="items"||k=="source"{rewrite(x,ids)?;}}
            }
            Value::Array(a)=>for x in a{rewrite(x,ids)?;},
            _=>{},
        }
        Ok(())
    }
    rewrite(&mut v,ids)?;Ok(v)
}
impl World{
    /// Identity-independent pure graph fingerprint. Class/contract names and
    /// lineage are excluded, but expression, behavior, shapes and wire topology
    /// remain. Different graph orderings may conservatively fail to match.
    pub fn structural_class_pin(&self,key:&str)->Result<Digest>{
        let c=self.classes.get(key).ok_or("class missing")?;
        ensure(self.class_available(key),"class not admitted or disabled")?;
        self.validate_class(c)?;
        let mut ids=BTreeMap::new();let mut nodes=vec![];
        for n in &c.nodes{
            let o=self.operators.get(&n.operator).ok_or("operator missing")?;
            nodes.push(json!({"behavior":o.behavior,"input":o.input,"output":o.output,"program":o.program,"wire":source_wire(&n.input,&ids)?}));
            ids.insert(n.id.clone(),nodes.len()-1);
        }
        Ok(digest(&json!({"input":c.input,"output":c.output,"nodes":nodes,"result":source_wire(&c.result,&ids)?})))
    }
    pub fn configure_assimilation(&mut self,p:Policy)->Result<()>{
        id(&p.family)?;
        ensure(p.version>self.assimilation.policies.get(&p.family).map_or(0,|x|x.version)
            &&(2..=32).contains(&p.min_training_goals)&&(2..=32).contains(&p.min_holdout_goals)
            &&(1..=32).contains(&p.max_steps)&&(1..=128).contains(&p.max_cohorts),"invalid assimilation policy")?;
        self.assimilation.policies.insert(p.family.clone(),p);Ok(())
    }
    pub fn register_trace_cohort(&mut self,c:Cohort)->Result<()>{
        id(&c.id)?;id(&c.verifier)?;id(&c.review_goal)?;
        let p=self.assimilation.policies.get(&c.family).ok_or("assimilation policy missing")?;
        ensure(p.enabled&&p.version==c.policy_version&&self.goals.contains_key(&c.review_goal)
            &&!self.assimilation.cohorts.contains_key(&c.id)
            &&self.assimilation.cohorts.values().filter(|old|old.family==c.family).count()<p.max_cohorts,"cohort admission denied")?;
        ensure((p.min_training_goals..=32).contains(&c.training_goals.len())&&(p.min_holdout_goals..=32).contains(&c.holdout_goals.len()),"cohort size outside policy")?;
        let all:BTreeSet<_>=c.training_goals.iter().chain(&c.holdout_goals).collect();
        ensure(all.len()==c.training_goals.len()+c.holdout_goals.len()&&all.iter().all(|g|self.goals.contains_key(*g)),"cohort goals must be distinct and registered")?;
        // Holdouts are single-use across both training and review in this layer.
        // Register before freezing; independent review goals may complete later.
        ensure(self.assimilation.cohorts.values().all(|old|{
            old.holdout_goals.iter().all(|g|!all.contains(g))
                &&c.holdout_goals.iter().all(|g|!old.training_goals.contains(g))
        }),"cohort holdout overlap")?;
        self.assimilation.cohorts.insert(c.id.clone(),c);Ok(())
    }
    fn trace_pattern(&self,goal:&str,p:&Policy)->Result<(Pattern,TracePin)>{
        let g=self.goals.get(goal).ok_or("trace goal missing")?;
        ensure(g.phase==GoalPhase::Complete&&!self.goals.values().any(|g|g.parent.as_deref()==Some(goal)),"trace requires completed leaf goal")?;
        let tasks:BTreeMap<_,_>=self.tasks.values().filter(|t|t.spec.goal==goal).map(|t|(t.spec.id.clone(),t)).collect();
        ensure(!tasks.is_empty()&&tasks.len()<=p.max_steps,"trace step budget")?;
        ensure(tasks.values().all(|t|t.phase==TaskPhase::Verified&&t.spec.family.as_ref()==Some(&p.family)
            &&!self.effects.values().any(|e|e.task==t.spec.id)),"trace must be pure verified family work")?;
        let mut order=Vec::<Id>::new();let mut indices=BTreeMap::<Id,usize>::new();
        while order.len()<tasks.len(){
            let key=tasks.iter().find(|(k,t)|!indices.contains_key(*k)&&t.spec.dependencies.iter().all(|d|indices.contains_key(d)))
                .map(|(k,_)|k.clone()).ok_or("trace dependency cycle")?;
            indices.insert(key.clone(),order.len());order.push(key);
        }
        let mut steps=vec![];let mut evidence=vec![];let mut inputs=vec![];
        for (i,key) in order.iter().enumerate(){
            let t=tasks[key];
            let class=t.assigned_class.as_deref().ok_or("trace class missing")?;
            let context=self.contexts.get(t.assigned_context.as_deref().ok_or("trace context missing")?).ok_or("context version missing")?;
            let a=self.artifacts.get(t.artifact.as_deref().ok_or("trace artifact missing")?).ok_or("artifact missing")?;
            let e=self.evidence.get(&a.evidence).ok_or("trace evidence missing")?;
            let input=t.input.as_ref().ok_or("trace input missing")?;
            let output=self.payload(&a.payload)?;
            ensure(e.accepted&&e.task==*key&&e.generation==t.generation&&a.generation==t.generation
                &&e.candidate==a.payload&&e.input==digest(input)&&e.class.as_deref()==Some(class)
                &&e.context==t.assigned_context&&self.assemble(&t.spec,context)?==*input
                &&e.oracle.evaluate(input,output)?&&t.spec.oracle.evaluate(input,output)?
                &&self.run_class(class,input)?==*output,"trace replay or evidence mismatch")?;
            let mut ports=BTreeMap::new();
            for (name,port) in &t.spec.inputs{
                let source=match &port.binding{
                    Binding::Predecessor{task}=>Source::Step{index:*indices.get(task).ok_or("trace predecessor missing")?},
                    Binding::Context{key}=>{id(key)?;Source::Context{key:key.clone()}},
                    Binding::Literal{..}|Binding::Knowledge{..}|Binding::SelectKnowledge{..}=>{
                        let key=format!("p-{i}-{}",digest(&json!(name)));Source::Parameter{key}
                    }
                };
                ports.insert(name.clone(),PortPattern{shape:port.shape.clone(),source});
            }
            steps.push(StepPattern{class_shape:self.structural_class_pin(class)?,input:Self::input_shape(&t.spec),
                output:t.spec.output.clone(),inputs:ports,dependencies:t.spec.dependencies.iter().map(|d|indices[d]).collect()});
            evidence.push(e.id.clone());inputs.push(digest(input));
        }
        let pattern=Pattern{family:p.family.clone(),steps};
        let trace_digest=pin(&json!([&order,&evidence,&inputs,&pattern]))?;
        Ok((pattern,TracePin{goal:goal.into(),tasks:order,evidence,input_digest:pin(&inputs)?,trace_digest}))
    }
    fn candidate_id(cohort:&str)->Id{format!("trace-{}",digest(&json!(cohort)))}
    pub fn ready_trace_freezes(&self)->Vec<Id>{
        self.assimilation.cohorts.values().filter(|c|!self.assimilation.candidates.contains_key(&Self::candidate_id(&c.id))
            &&self.assimilation.policies.get(&c.family).is_some_and(|p|p.enabled&&p.version==c.policy_version)
            &&c.training_goals.iter().all(|g|self.goals.get(g).is_some_and(|x|x.phase==GoalPhase::Complete)))
            .map(|c|c.id.clone()).collect()
    }
    pub fn freeze_trace_pattern(&mut self,cohort:&str)->Result<Value>{
        ensure(self.ready_trace_freezes().iter().any(|x|x==cohort),"trace cohort not ready")?;
        let c=self.assimilation.cohorts[cohort].clone();let p=self.assimilation.policies[&c.family].clone();
        let key=Self::candidate_id(cohort);let mut candidate=Candidate{id:key.clone(),cohort:cohort.into(),policy:p.clone(),pattern:None,pattern_digest:None,training:vec![],rejection:None,sequence:self.sequence.checked_add(1).ok_or("sequence overflow")?};
        // Record rejection as durable outcome, not a retry loop. Candidate sees
        // only explicit training goals; no holdout trace is evaluated here.
        let result=(||->Result<()>{
            let mut seen=BTreeSet::new();let mut pattern=None;let mut expected=None;
            for g in &c.training_goals{
                let (current,trace)=self.trace_pattern(g,&p)?;
                let h=pin(&current)?;
                ensure(seen.insert(trace.input_digest.clone()),"training goal input repeated")?;
                if let Some(e)=&expected{ensure(e==&h,"training traces have different structures")?;}else{expected=Some(h);pattern=Some(current);}
                candidate.training.push(trace);
            }
            candidate.pattern=pattern;candidate.pattern_digest=expected;Ok(())
        })();
        if let Err(reason)=result{candidate.rejection=Some(reason);}
        let response=val(&candidate)?;bounded(&response)?;
        self.assimilation.candidates.insert(key,candidate);Ok(response)
    }
    /// Deterministic review proposal for a separately authenticated verifier.
    /// Holdouts here are logically disjoint, not private from a trusted host.
    pub fn evaluate_trace_pattern(&self,key:&str)->Result<Verdict>{
        let f=self.assimilation.candidates.get(key).ok_or("trace candidate missing")?;
        let c=self.assimilation.cohorts.get(&f.cohort).ok_or("trace cohort missing")?;
        let pattern=f.pattern_digest.as_ref().ok_or("candidate rejected during training")?;
        ensure(!self.assimilation.verdicts.contains_key(key),"trace cohort already consumed")?;
        let mut seen:BTreeSet<_>=f.training.iter().map(|t|t.input_digest.clone()).collect();
        let mut evaluations=vec![];let mut passed=true;
        for g in &c.holdout_goals{
            ensure(self.goals.get(g).is_some_and(|g|g.phase==GoalPhase::Complete),"holdout goal incomplete")?;
            match self.trace_pattern(g,&f.policy){
                Ok((p,t))=>{
                    let matches=pin(&p)?==*pattern&&seen.insert(t.input_digest.clone());
                    passed &=matches;evaluations.push(json!({"goal":g,"trace":t,"passed":matches}));
                }
                Err(reason)=>{passed=false;evaluations.push(json!({"goal":g,"error":reason,"passed":false}));}
            }
        }
        Ok(Verdict{cohort:c.id.clone(),candidate:key.into(),pattern_digest:pattern.clone(),holdout_goals:c.holdout_goals.clone(),
            evaluations:evaluations.len(),passed,evaluation_digest:pin(&evaluations)?})
    }
    pub fn attest_trace_pattern(&mut self,who:&str,v:Verdict)->Result<Value>{
        let c=self.assimilation.cohorts.get(&v.cohort).ok_or("trace cohort missing")?;
        ensure(c.verifier==who,"trace verifier scope mismatch")?;
        let expected=self.evaluate_trace_pattern(&v.candidate)?;
        ensure(pin(&expected)?==pin(&v)?,"trace verdict disagrees with independently replayed evidence")?;
        let accepted=v.passed;let key=v.candidate.clone();
        self.assimilation.verdicts.insert(key.clone(),Review{verifier:who.into(),verdict:v,sequence:self.sequence.checked_add(1).ok_or("sequence overflow")?});
        Ok(json!({"candidate":key,"accepted":accepted}))
    }
    pub fn ready_trace_promotions(&self)->Vec<Id>{
        self.assimilation.verdicts.iter().filter(|(k,r)|r.verdict.passed&&!self.assimilation.promotions.contains_key(*k)
            &&self.assimilation.candidates.get(*k).is_some_and(|f|self.assimilation.policies.get(&f.policy.family).is_some_and(|p|p.enabled&&p.auto_promote&&p.version==f.policy.version)))
            .map(|(k,_)|k.clone()).collect()
    }
    pub fn promote_trace_pattern(&mut self,key:&str)->Result<Value>{
        let f=self.assimilation.candidates.get(key).ok_or("trace candidate missing")?;
        let r=self.assimilation.verdicts.get(key).ok_or("independent trace verdict missing")?;
        let p=self.assimilation.policies.get(&f.policy.family).ok_or("assimilation policy missing")?;
        ensure(p.enabled&&p.version==f.policy.version&&r.verdict.passed&&f.pattern.is_some()
            &&f.pattern_digest.as_ref()==Some(&r.verdict.pattern_digest)&&!self.assimilation.promotions.contains_key(key),"trace promotion denied")?;
        // Recheck frozen training provenance before admission. Old evidence is
        // retained even when source classes are subsequently disabled.
        for t in &f.training{let (_,now)=self.trace_pattern(&t.goal,&f.policy)?;ensure(now.trace_digest==t.trace_digest,"frozen trace changed")?;}
        self.assimilation.promotions.insert(key.into(),Promotion{candidate:key.into(),enabled:true,sequence:self.sequence.checked_add(1).ok_or("sequence overflow")?,disabled_reason:None});
        Ok(json!({"candidate":key,"promoted":true}))
    }
    pub fn disable_trace_pattern(&mut self,key:&str,reason:String)->Result<Value>{
        ensure(!reason.trim().is_empty()&&reason.len()<=4096,"rollback reason required")?;
        let p=self.assimilation.promotions.get_mut(key).ok_or("trace promotion missing")?;
        ensure(p.enabled,"trace already disabled")?;p.enabled=false;p.disabled_reason=Some(reason);
        Ok(json!({"candidate":key,"disabled":true,"existing_goals_unchanged":true}))
    }
    pub fn trace_reuse_plan(&self,family:&str)->Result<Value>{
        id(family)?;
        let mut plans=vec![];
        for (key,p) in &self.assimilation.promotions{
            if !p.enabled{continue;}
            let f=&self.assimilation.candidates[key];
            if f.policy.family!=family||!self.assimilation.policies.get(family).is_some_and(|p|p.enabled&&p.version==f.policy.version){continue;}
            let pattern=f.pattern.as_ref().ok_or("promoted pattern missing")?;
            let mut classes=vec![];
            for step in &pattern.steps{
                let compatible:Vec<_>=self.classes.keys().filter(|c|self.structural_class_pin(c).is_ok_and(|h|h==step.class_shape)).take(128).cloned().collect();
                classes.push(compatible);
            }
            plans.push(json!({"candidate":key,"pattern":pattern,"compatible_classes":classes,"fresh_contexts_required":true,"fresh_oracles_required":true}));
        }
        Ok(json!({"family":family,"plans":plans,"authority":"proposal_only"}))
    }
    pub fn instantiate_trace_pattern(&mut self,r:Instantiate)->Result<Value>{
        id(&r.goal)?;ensure(r.goal.len()<=80&&!self.assimilation.runs.contains_key(&r.goal),"invalid reuse goal")?;
        let f=self.assimilation.candidates.get(&r.candidate).ok_or("trace candidate missing")?.clone();
        let pattern=f.pattern.ok_or("pattern missing")?;
        ensure(self.assimilation.promotions.get(&r.candidate).is_some_and(|p|p.enabled)
            &&self.assimilation.policies.get(&pattern.family).is_some_and(|p|p.enabled&&p.version==f.policy.version),"trace pattern unavailable")?;
        let n=pattern.steps.len();
        ensure(r.classes.len()==n&&r.contexts.len()==n&&r.oracles.len()==n&&r.claim_budget>=n as u64&&r.claim_budget<=1024,"invalid reuse allocations")?;
        let required:BTreeSet<_>=pattern.steps.iter().flat_map(|s|s.inputs.values()).filter_map(|p|match &p.source{Source::Parameter{key}=>Some(key.clone()),_=>None}).collect();
        ensure(required==r.parameters.keys().cloned().collect(),"reuse parameter set mismatch")?;
        let keys:Vec<_>=(0..n).map(|i|format!("{}-step-{i}",r.goal)).collect();
        // All tasks are created atomically through the outer World transition.
        let actor=Actor::Host;
        self.apply_adapter_action(&actor,Action::Goal{id:r.goal.clone(),parent:None,objective:r.objective,claim_budget:r.claim_budget,predicate:None})?;
        for (i,s) in pattern.steps.iter().enumerate(){
            ensure(self.structural_class_pin(&r.classes[i])?==s.class_shape,"reuse class structural mismatch")?;
            let ctx=self.contexts.get(&r.contexts[i]).ok_or("fresh context missing")?;
            let mut inputs=BTreeMap::new();
            for (name,p) in &s.inputs{
                let binding=match &p.source{
                    Source::Parameter{key}=>{let v=&r.parameters[key];bounded(v)?;ensure(p.shape.accepts(v),"reuse parameter shape mismatch")?;Binding::Literal{value:v.clone()}},
                    Source::Context{key}=>{
                        let v=if let Some(v)=ctx.entries.get(key){v}else{self.artifact_value(ctx.artifacts.get(key).ok_or("fresh context key missing")?)?};
                        ensure(p.shape.accepts(v),"fresh context shape mismatch")?;Binding::Context{key:key.clone()}
                    }
                    Source::Step{index}=>{ensure(*index<i,"reuse dependency order invalid")?;Binding::Predecessor{task:keys[*index].clone()}}
                };
                inputs.insert(name.clone(),Port{shape:p.shape.clone(),binding});
            }
            self.apply_adapter_action(&actor,Action::Task{spec:TaskSpec{id:keys[i].clone(),goal:r.goal.clone(),objective:format!("qualified trace {} step {i}",r.candidate),
                dependencies:s.dependencies.iter().map(|j|keys[*j].clone()).collect(),inputs,output:s.output.clone(),oracle:r.oracles[i].clone(),
                class:Some(r.classes[i].clone()),family:Some(pattern.family.clone()),context:Some(r.contexts[i].clone()),max_attempts:2,priority:0,uncertainty:0,information_gain:0,expected_cost:1}})?;
        }
        self.apply_adapter_action(&actor,Action::Seal{goal:r.goal.clone()})?;
        self.assimilation.runs.insert(r.goal.clone(),Reuse{candidate:r.candidate.clone(),goal:r.goal.clone(),tasks:keys.clone(),class_pins:r.classes,context_pins:r.contexts,sequence:self.sequence.checked_add(1).ok_or("sequence overflow")?});
        Ok(json!({"goal":r.goal,"candidate":r.candidate,"tasks":keys,"verified":false}))
    }
    pub fn assimilation_report(&self)->Result<Value>{
        let runs:Vec<_>=self.assimilation.runs.values().map(|r|{
            let verified=r.tasks.iter().filter(|t|self.tasks.get(*t).is_some_and(|t|t.phase==TaskPhase::Verified)).count();
            json!({"reuse":r,"verified_tasks":verified,"goal_phase":self.goals.get(&r.goal).map(|g|&g.phase)})
        }).collect();
        Ok(json!({"registry":self.assimilation,"reuse_outcomes":runs,"acceptance_credit":false}))
    }
}
