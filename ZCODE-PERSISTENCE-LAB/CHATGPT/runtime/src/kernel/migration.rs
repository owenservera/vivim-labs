//! Explicit, lossless archival import of the R003 state and conservative
//! reconstruction of its verified goals, tasks and artifacts. Old execution
//! leases are never carried into the new scheduler.
use super::model::*;
use crate::{digest,Expr,Operator,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::collections::{BTreeMap,BTreeSet};

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct LegacyArchive {
    pub source_digest:Digest,pub state:Value,pub report:MigrationReport,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct MigrationReport {
    pub goals:BTreeMap<Id,Id>,pub tasks:BTreeMap<Id,Id>,pub executable_classes:BTreeMap<Id,Id>,
    pub verified_artifacts:usize,pub fenced_attempts:usize,
    pub archived_agents:usize,pub archived_contexts:usize,pub archived_messages:usize,
    pub warnings:Vec<String>,
}
fn mapped(kind:&str,old:&str)->Id{format!("legacy-{kind}-{}",digest(&json!(old)))}
pub fn import(old:&crate::State)->Result<(World,MigrationReport)>{
    let mut w=World::default();
    let mut report=MigrationReport{
        goals:old.goals.keys().map(|k|(k.clone(),mapped("goal",k))).collect(),
        tasks:old.tasks.keys().map(|k|(k.clone(),mapped("task",k))).collect(),
        executable_classes:BTreeMap::new(),verified_artifacts:0,fenced_attempts:0,
        archived_agents:old.agents.len(),archived_contexts:old.contexts.len(),archived_messages:old.messages.len(),
        warnings:vec![
            "R003 logical clock and every original record are preserved in the archive, not treated as current wall time.".into(),
            "Old agents, context, mailboxes, checkpoints, rejected proposals and non-executable class labels are archived; no stale executor or unverified proposal is resumed.".into(),
            "Old active programs are exposed as explicit classes, not automatically admitted as R004 learned capabilities; independent holdouts are still required.".into(),
            "Pending tasks without executable classes require explicit host binding before autonomous native execution.".into(),
        ],
    };
    ensure(report.goals.values().collect::<BTreeSet<_>>().len()==old.goals.len(),"goal mapping collision")?;
    ensure(report.tasks.values().collect::<BTreeSet<_>>().len()==old.tasks.len(),"task mapping collision")?;
    let input_shape=Shape::Record{fields:BTreeMap::from([("legacy_input".into(),Shape::Any)])};
    for (key,c) in &old.capabilities{
        if !c.active{continue;}
        let name=mapped("program",key);let class_key=version_key(&name,1)?;
        w.operators.insert(class_key.clone(),OperatorContract{id:name.clone(),version:1,behavior:Operator::Execute,input:Shape::Any,output:Shape::Any,program:c.program.clone()});
        let class=Class{id:name,version:1,input:input_shape.clone(),output:Shape::Any,
            nodes:vec![Node{id:"execute".into(),operator:class_key.clone(),input:Wire::Field{source:Box::new(Wire::Input),key:"legacy_input".into()}}],
            result:Wire::Node{id:"execute".into()},lineage:vec![]};
        w.validate_class(&class)?;
        w.classes.insert(class_key.clone(),class);report.executable_classes.insert(key.clone(),class_key);
    }
    for (key,g) in &old.goals{
        let id=report.goals[key].clone();
        ensure(g.claim_budget>0&&g.claims<=g.claim_budget,"invalid legacy goal budget")?;
        let phase=match g.status{crate::GoalStatus::Open=>GoalPhase::Open,crate::GoalStatus::Complete=>GoalPhase::Complete,crate::GoalStatus::Cancelled=>GoalPhase::Cancelled};
        if phase==GoalPhase::Complete{
            ensure(g.sealed&&old.tasks.values().any(|t|t.goal==*key)&&old.tasks.values().filter(|t|t.goal==*key).all(|t|t.status==crate::TaskStatus::Committed),"invalid completed legacy goal")?;
        }
        w.goals.insert(id.clone(),Goal{id,parent:None,objective:g.objective.clone(),version:1,predicate:None,sealed:g.sealed,phase,claim_budget:g.claim_budget,claims:g.claims});
    }
    for (key,t) in &old.tasks{
        let id=report.tasks[key].clone();let goal=report.goals.get(&t.goal).ok_or("legacy goal missing")?.clone();
        let dependencies=t.depends.iter().map(|d|report.tasks.get(d).cloned().ok_or("legacy dependency missing")).collect::<std::result::Result<BTreeSet<_>,_>>()?;
        ensure(dependencies.len()==t.depends.len()&&t.depends.iter().all(|d|old.tasks.get(d).is_some_and(|x|x.goal==t.goal)),"legacy dependencies invalid")?;
        let max_attempts=u32::try_from(t.max_attempts).map_err(|_|"legacy attempt limit exceeds R004")?;
        let attempts=u32::try_from(t.attempts).map_err(|_|"legacy attempt count exceeds R004")?;
        ensure(max_attempts>0&&attempts<=max_attempts&&hash_id(&t.expected),"invalid legacy task")?;
        let phase=match t.status{
            crate::TaskStatus::Committed=>TaskPhase::Verified,
            crate::TaskStatus::Cancelled=>TaskPhase::Cancelled,
            crate::TaskStatus::Exhausted=>TaskPhase::Exhausted,
            crate::TaskStatus::Running|crate::TaskStatus::Submitted|crate::TaskStatus::Pending=>{
                if t.status!=crate::TaskStatus::Pending{report.fenced_attempts+=1;}
                if attempts>=max_attempts{TaskPhase::Exhausted}else{TaskPhase::Pending}
            }
        };
        let class=t.capability.as_ref().and_then(|c|report.executable_classes.get(c)).cloned();
        let spec=TaskSpec{id:id.clone(),goal,objective:t.objective.clone(),dependencies,
            inputs:BTreeMap::from([("legacy_input".into(),Port{shape:Shape::Any,binding:Binding::Literal{value:t.input.clone()}})]),
            output:Shape::Any,oracle:Oracle::Digest{expected:t.expected.clone()},class:class.clone(),family:None,context:None,
            max_attempts,priority:0,uncertainty:0,information_gain:0,expected_cost:1};
        let input=json!({"legacy_input":t.input});bounded(&input)?;
        w.tasks.insert(id.clone(),Task{spec,version:1,phase:phase.clone(),attempts,generation:t.generation,lease:None,input:Some(input),proposal:None,
            artifact:if phase==TaskPhase::Verified{Some(id)}else{None},reason:if phase==TaskPhase::Pending{Some("legacy lease fenced".into())}else{None},assigned_class:class,assigned_context:None});
    }
    // Check the dependency graph without relying on record order in the old map.
    for t in w.tasks.values(){
        let mut todo:Vec<_>=t.spec.dependencies.iter().cloned().collect();let mut seen=BTreeSet::new();
        while let Some(k)=todo.pop(){ensure(k!=t.spec.id,"legacy dependency cycle")?;
            if seen.insert(k.clone()){todo.extend(w.tasks.get(&k).ok_or("legacy dependency missing")?.spec.dependencies.iter().cloned());}
        }
    }
    for (key,t) in &old.tasks{
        if t.status!=crate::TaskStatus::Committed{continue;}
        let task=report.tasks[key].clone();
        let artifact=t.artifact.as_ref().and_then(|a|old.artifacts.get(a)).ok_or("legacy artifact missing")?;
        ensure(artifact.task==*key&&artifact.generation==t.generation&&digest(&artifact.value)==artifact.digest&&artifact.digest==t.expected,"legacy artifact integrity mismatch")?;
        ensure(old.evidence.iter().any(|e|e.task==*key&&e.generation==t.generation&&e.accepted&&e.actual==artifact.digest&&e.expected==t.expected),"legacy evidence missing")?;
        ensure(t.depends.iter().all(|d|old.tasks[d].status==crate::TaskStatus::Committed),"legacy artifact depends on unverified task")?;
        let payload=w.put_payload(artifact.value.clone())?;
        let input=w.put_payload(json!({"legacy_input":t.input}))?;
        let evidence=format!("{task}-{}",t.generation);
        let parents:Vec<_>=t.depends.iter().map(|d|report.tasks[d].clone()).collect();
        let parent_evidence=parents.iter().zip(t.depends.iter()).map(|(p,d)|format!("{p}-{}",old.tasks[d].generation)).collect();
        w.evidence.insert(evidence.clone(),Evidence{id:evidence.clone(),task:task.clone(),generation:t.generation,oracle:Oracle::Digest{expected:t.expected.clone()},input,candidate:payload.clone(),accepted:true,verifier:"legacy-r003".into(),parents:parent_evidence,clock_ms:0,class:w.tasks[&task].assigned_class.clone(),context:None});
        w.artifacts.insert(task.clone(),Artifact{id:task.clone(),version:1,payload,task,generation:t.generation,inputs:parents,evidence});
        report.verified_artifacts+=1;
    }
    // Old active processes are not authority in the new store; archived identities
    // remain recoverable as historical records and new workers are generated as needed.
    let state=serde_json::to_value(old).map_err(|e|e.to_string())?;
    w.legacy_import=Some(LegacyArchive{source_digest:digest(&state),state,report:report.clone()});
    w.sequence=1;
    Ok((w,report))
}
