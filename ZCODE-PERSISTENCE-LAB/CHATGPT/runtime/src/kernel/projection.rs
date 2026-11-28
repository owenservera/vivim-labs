//! Complete host-owned agent projection. Canonical entities stay in the index;
//! the filesystem tree is a replaceable, integrity-addressed read model.
use super::model::*;
use crate::{digest,Result};
use serde_json::{Value,json};
use std::{collections::{BTreeMap,BTreeSet},fs,path::{Path,PathBuf},io::Write,time::{SystemTime,UNIX_EPOCH}};
fn err(e:impl std::fmt::Display)->String{e.to_string()}
fn immutable(path:&Path,value:&Value)->Result<()>{
    if path.exists(){
        let v:Value=serde_json::from_slice(&fs::read(path).map_err(err)?).map_err(err)?;
        return ensure(v==*value,"immutable projection collision");
    }
    let parent=path.parent().ok_or("projection parent missing")?;
    fs::create_dir_all(parent).map_err(err)?;
    let stamp=SystemTime::now().duration_since(UNIX_EPOCH).map_err(err)?.as_nanos();
    let tmp=parent.join(format!(".tmp-{}-{stamp}",std::process::id()));
    let mut f=fs::OpenOptions::new().write(true).create_new(true).open(&tmp).map_err(err)?;
    f.write_all(&serde_json::to_vec_pretty(value).map_err(err)?).map_err(err)?;f.sync_all().map_err(err)?;drop(f);
    fs::rename(tmp,path).map_err(err)?;
    #[cfg(unix)] fs::File::open(parent).and_then(|f|f.sync_all()).map_err(err)?;
    Ok(())
}
pub fn project(world:&World,agent:&str,root:&Path)->Result<PathBuf>{
    id(agent)?;
    let a=world.agents.get(agent).ok_or("agent missing")?;
    let goal=world.goals.get(&a.goal).ok_or("goal missing")?;
    let class=world.classes.get(&a.class).ok_or("class missing")?;
    let context=world.contexts.get(&a.context).ok_or("context missing")?;
    let attempts:Vec<_>=world.attempt_history.values().filter(|x|x.agent==agent).collect();
    let task_ids:BTreeSet<_>=attempts.iter().map(|x|x.task.as_str()).collect();
    let tasks:Vec<_>=world.tasks.values().filter(|t|task_ids.contains(t.spec.id.as_str())).collect();
    let evidence:Vec<_>=world.evidence.values().filter(|e|attempts.iter().any(|a|a.task==e.task&&a.generation==e.generation)).collect();
    let artifacts:Vec<_>=world.artifacts.values().filter(|a|evidence.iter().any(|e|e.id==a.evidence)).collect();
    let artifact_ids:BTreeSet<_>=artifacts.iter().map(|a|a.id.as_str()).collect();
    let discoveries:Vec<_>=world.knowledge.values().filter(|k|k.artifacts.iter().any(|id|artifact_ids.contains(id.as_str()))).collect();
    let checkpoints:BTreeMap<_,_>=world.checkpoints.iter().filter(|(k,_)|task_ids.contains(k.as_str())).map(|(k,v)|(k,v.iter().filter(|c|c.worker==agent).collect::<Vec<_>>())).collect();
    let messages:Vec<_>=world.messages.values().filter(|m|m.recipient==agent||m.sender==agent).map(|m|json!({"record":m,"delivery":world.delivery.get(&m.id)})).collect();
    let capabilities:Vec<_>=world.capabilities.values().filter(|c|c.class==a.class).collect();
    let mut parts=BTreeMap::from([
        ("identity",json!({"id":a.id})),
        ("lineage",json!({"parent":a.parent,"class_lineage":class.lineage})),
        ("class",json!({"key":a.class,"definition":class})),
        ("objective",json!({"goal":goal,"tasks":tasks})),
        ("capabilities",json!(capabilities)),
        ("constraints",json!({"claim_budget":goal.claim_budget,"claims":goal.claims,"global_slots":world.slots,"context_bytes":context.byte_budget})),
        ("context",json!({"key":a.context,"definition":context})),
        ("state",json!({"phase":a.phase,"wait_task":a.wait_task,"attempts":attempts,"clock_ms":world.clock_ms})),
        ("mailbox",json!(messages)),
        ("workspace",json!({"owner":agent,"authority":"derived_projection_only"})),
        ("discoveries",json!(discoveries)),
        ("checkpoints",json!(checkpoints)),
        ("evidence",json!({"evidence":evidence,"artifacts":artifacts})),
        ("incarnations",json!(a.incarnations)),
    ]);
    let path=root.join(agent);let objects=path.join("objects");let manifests=path.join("manifests");
    fs::create_dir_all(&objects).map_err(err)?;fs::create_dir_all(&manifests).map_err(err)?;
    let mut refs=BTreeMap::new();
    for (name,value) in std::mem::take(&mut parts){
        let hash=digest(&value);
        immutable(&objects.join(format!("{hash}.json")),&value)?;
        refs.insert(name,hash);
    }
    let manifest=json!({"schema":1,"agent":agent,"sequence":world.sequence,"parts":refs});
    immutable(&manifests.join(format!("{:020}.json",world.sequence)),&manifest)?;
    Ok(path)
}
pub fn read_part(root:&Path,agent:&str,sequence:u64,part:&str)->Result<Value>{
    id(agent)?;let path=root.join(agent);
    let manifest:Value=serde_json::from_slice(&fs::read(path.join("manifests").join(format!("{sequence:020}.json"))).map_err(err)?).map_err(err)?;
    ensure(manifest["agent"]==agent&&manifest["sequence"].as_u64()==Some(sequence),"projection manifest mismatch")?;
    let hash=manifest["parts"].get(part).and_then(Value::as_str).ok_or("projection part missing")?;
    ensure(hash_id(hash),"invalid projection digest")?;
    let value:Value=serde_json::from_slice(&fs::read(path.join("objects").join(format!("{hash}.json"))).map_err(err)?).map_err(err)?;
    ensure(digest(&value)==hash,"projection part integrity mismatch")?;Ok(value)
}
