//! Experiment-specific host-owned persistence gateway. Each run gets a fresh
//! immutable manifest, isolated journal, allowed persistence categories and
//! epoch-fenced writes. Not an OS sandbox: adapters must not have direct access
//! to the host vault directory or another run's state.
use super::{comparative::{Arm,Authority,Spec},model::{ensure,id,hash_id,Id,Digest,MAX_VALUE_BYTES}};
use crate::{digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::{collections::BTreeMap,fs::{self,File,OpenOptions},io::Write,path::{Path,PathBuf},time::{SystemTime,UNIX_EPOCH}};

fn err(e:impl std::fmt::Display)->String{e.to_string()}
fn value<T:Serialize>(v:&T)->Result<Value>{serde_json::to_value(v).map_err(err)}
fn checksum<T:Serialize>(v:&T)->Result<Digest>{Ok(digest(&value(v)?))}
fn read<T:for<'de>Deserialize<'de>>(p:&Path,limit:u64)->Result<T>{
    ensure(fs::symlink_metadata(p).map_err(err)?.file_type().is_file(),"vault record is not a regular file")?;
    ensure(fs::metadata(p).map_err(err)?.len()<=limit,"vault record exceeds read budget")?;
    serde_json::from_slice(&fs::read(p).map_err(err)?).map_err(err)
}
fn write_new<T:Serialize>(root:&Path,name:&str,v:&T)->Result<()>{
    let dest=root.join(name);ensure(!dest.exists(),"immutable vault record already exists")?;
    let stamp=SystemTime::now().duration_since(UNIX_EPOCH).map_err(err)?.as_nanos();
    let tmp=root.join(format!(".tmp-{}-{stamp}",std::process::id()));
    let mut f=OpenOptions::new().create_new(true).write(true).open(&tmp).map_err(err)?;
    f.write_all(&serde_json::to_vec(v).map_err(err)?).map_err(err)?;f.sync_all().map_err(err)?;drop(f);
    fs::rename(&tmp,dest).map_err(err)?;
    #[cfg(unix)] File::open(root).and_then(|f|f.sync_all()).map_err(err)?;
    Ok(())
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits{pub max_events:u64,pub max_records:usize,pub max_payload_bytes:usize}
impl Limits{
    fn validate(&self)->Result<()>{
        ensure((1..=65_536).contains(&self.max_events)&&(1..=4096).contains(&self.max_records)
            &&(1..=MAX_VALUE_BYTES).contains(&self.max_payload_bytes),"invalid vault limits")
    }
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest{
    pub schema:u32,pub run_id:Id,pub spec_digest:Digest,pub arm:Arm,pub plan_entry_digest:Digest,pub limits:Limits,
}
impl Manifest{
    fn from_plan(spec:&Spec,run_id:&str,limits:Limits)->Result<Self>{
        limits.validate()?;id(run_id)?;
        let plan=spec.plan()?;
        let entry=plan["runs"].as_array().ok_or("comparison plan missing runs")?.iter().find(|x|x["run_id"]==run_id).ok_or("unknown comparison run")?;
        let arm=serde_json::from_value(entry["arm"].clone()).map_err(err)?;
        let spec_digest=plan["spec_digest"].as_str().ok_or("comparison digest missing")?.to_owned();
        Ok(Self{schema:1,run_id:run_id.into(),spec_digest,arm,plan_entry_digest:digest(entry),limits})
    }
}
/// Native session reference is an opaque handle only. Other payloads are typed
/// at the boundary; semantic smuggling inside arbitrary checkpoints remains an
/// adapter-level trust issue and cannot be inferred away by a field name.
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(tag="kind",rename_all="snake_case",deny_unknown_fields)]
pub enum Record{
    NativeSession{session_ref:String},
    AgentIdentity{agent:Id,parent:Option<Id>},
    AgentCheckpoint{agent:Id,checkpoint:Value},
    Goal{goal:Id,objective:String},
    TaskDag{goal:Id,graph:Value},
    Artifact{task:Id,content:Value},
    Evidence{task:Id,artifact_digest:Digest,accepted:bool},
    AgentRegistry{agents:Vec<Id>},
    Class{definition:Value},
    Operator{definition:Value},
    Capability{definition:Value},
    Repair{definition:Value},
}
impl Record{
    pub fn authority(&self)->Authority{
        match self{
            Self::NativeSession{..}=>Authority::NativeSession,Self::AgentIdentity{..}=>Authority::AgentIdentity,
            Self::AgentCheckpoint{..}=>Authority::AgentCheckpoint,Self::Goal{..}=>Authority::Goal,Self::TaskDag{..}=>Authority::TaskDag,
            Self::Artifact{..}=>Authority::Artifact,Self::Evidence{..}=>Authority::Evidence,Self::AgentRegistry{..}=>Authority::AgentRegistry,
            Self::Class{..}=>Authority::Class,Self::Operator{..}=>Authority::Operator,Self::Capability{..}=>Authority::Capability,Self::Repair{..}=>Authority::Repair,
        }
    }
    fn validate(&self,limits:&Limits)->Result<()>{
        let size=serde_json::to_vec(self).map_err(err)?.len();
        ensure(size<=limits.max_payload_bytes,"record payload exceeds declared limit")?;
        match self{
            Self::NativeSession{session_ref}=>ensure(!session_ref.trim().is_empty()&&session_ref.len()<=256&&!session_ref.contains('\r')&&!session_ref.contains('\n'),"invalid opaque session reference"),
            Self::AgentIdentity{agent,parent}=>{id(agent)?;if let Some(p)=parent{id(p)?;ensure(p!=agent,"self-parent agent")?;}Ok(())},
            Self::AgentCheckpoint{agent,..}=>id(agent),
            Self::Goal{goal,objective}=>{id(goal)?;ensure(!objective.trim().is_empty(),"empty goal objective")},
            Self::TaskDag{goal,..}=>id(goal),
            Self::Artifact{task,..}=>id(task),
            Self::Evidence{task,artifact_digest,..}=>{id(task)?;ensure(hash_id(artifact_digest),"invalid artifact digest")},
            Self::AgentRegistry{agents}=>{ensure(agents.len()<=limits.max_records,"agent registry exceeds limit")?;let mut seen=std::collections::BTreeSet::new();for a in agents{id(a)?;ensure(seen.insert(a),"duplicate agent identity")?;}Ok(())},
            Self::Class{definition}|Self::Operator{definition}|Self::Capability{definition}|Self::Repair{definition}=>ensure(definition.is_object(),"definition must be a record"),
        }
    }
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WriteRequest{pub request_id:Id,pub epoch:u64,pub key:Id,pub expected_version:u64,pub record:Record}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(tag="kind",rename_all="snake_case")]
enum Operation{Write{request:WriteRequest},Advance{expected_epoch:u64}}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Event{sequence:u64,previous:Digest,manifest:Digest,operation:Operation}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Envelope{digest:Digest,event:Event}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Stored{key:Id,version:u64,epoch:u64,record:Record}
struct State{
    sequence:u64,epoch:u64,tail:Digest,
    records:BTreeMap<(Authority,Id),Stored>,
    requests:BTreeMap<Id,(Digest,Value)>,
}
impl State{
    fn new(manifest_digest:&str)->Self{Self{sequence:0,epoch:1,tail:manifest_digest.into(),records:BTreeMap::new(),requests:BTreeMap::new()}}
    fn apply(&mut self,m:&Manifest,op:&Operation)->Result<Value>{
        match op{
            Operation::Advance{expected_epoch}=>{
                ensure(*expected_epoch==self.epoch,"stale vault epoch")?;
                self.epoch=self.epoch.checked_add(1).ok_or("vault epoch overflow")?;
                Ok(json!({"epoch":self.epoch}))
            }
            Operation::Write{request:r}=>{
                id(&r.request_id)?;id(&r.key)?;
                ensure(r.epoch==self.epoch,"stale vault writer")?;
                let authority=r.record.authority();
                ensure(m.arm.admits(&authority),"persistence authority denied for this experimental arm")?;
                r.record.validate(&m.limits)?;
                ensure(!self.requests.contains_key(&r.request_id),"duplicate journal request")?;
                let key=(authority.clone(),r.key.clone());
                let version=self.records.get(&key).map_or(0,|s|s.version);
                ensure(version==r.expected_version,"record version conflict")?;
                ensure(version>0||self.records.len()<m.limits.max_records,"vault record budget exhausted")?;
                let version=version.checked_add(1).ok_or("record version overflow")?;
                let record_digest=checksum(&r.record)?;
                self.records.insert(key,Stored{key:r.key.clone(),version,epoch:self.epoch,record:r.record.clone()});
                let receipt=json!({"run_id":m.run_id,"epoch":self.epoch,"key":r.key,"authority":authority,"version":version,"record_digest":record_digest});
                self.requests.insert(r.request_id.clone(),(checksum(r)?,receipt.clone()));
                Ok(receipt)
            }
        }
    }
}
pub struct Vault{root:PathBuf,manifest:Manifest,manifest_digest:Digest}
impl Vault{
    pub fn create(root:&Path,spec:&Spec,run_id:&str,limits:Limits)->Result<Self>{
        let m=Manifest::from_plan(spec,run_id,limits)?;
        fs::create_dir(root).map_err(err)?;
        let root=root.canonicalize().map_err(err)?;
        fs::create_dir(root.join("events")).map_err(err)?;
        write_new(&root,"manifest.json",&m)?;
        let manifest_digest=checksum(&m)?;
        Ok(Self{root,manifest:m,manifest_digest})
    }
    pub fn open(root:&Path,spec:&Spec,run_id:&str)->Result<Self>{
        let root=root.canonicalize().map_err(err)?;
        let m:Manifest=read(&root.join("manifest.json"),64*1024)?;
        let expected=Manifest::from_plan(spec,run_id,m.limits.clone())?;
        ensure(value(&m)?==value(&expected)?,"vault manifest does not match planned run")?;
        ensure(fs::symlink_metadata(root.join("events")).map_err(err)?.file_type().is_dir(),"invalid vault journal directory")?;
        let manifest_digest=checksum(&m)?;
        Ok(Self{root,manifest:m,manifest_digest})
    }
    fn lock(&self)->Result<File>{
        let f=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(self.root.join("vault.lock")).map_err(err)?;
        f.lock().map_err(err)?;Ok(f)
    }
    fn load(&self)->Result<State>{
        let mut sequences=vec![];
        for item in fs::read_dir(self.root.join("events")).map_err(err)?{
            let item=item.map_err(err)?;let name=item.file_name().to_string_lossy().into_owned();
            if name.starts_with(".tmp-"){continue;}
            let n=name.strip_suffix(".json").ok_or("unexpected vault journal entry")?;
            let seq=n.parse::<u64>().map_err(err)?;
            ensure(name==format!("{seq:020}.json"),"noncanonical vault event name")?;
            sequences.push(seq);
            ensure(sequences.len() as u64<=self.manifest.limits.max_events,"vault journal exceeds event budget")?;
        }
        sequences.sort_unstable();
        let mut state=State::new(&self.manifest_digest);
        for seq in sequences{
            ensure(seq==state.sequence+1,"vault journal gap")?;
            let envelope:Envelope=read(&self.root.join("events").join(format!("{seq:020}.json")),(self.manifest.limits.max_payload_bytes as u64).saturating_add(64*1024))?;
            let e=&envelope.event;
            ensure(e.sequence==seq&&e.previous==state.tail&&e.manifest==self.manifest_digest&&checksum(e)?==envelope.digest,"vault journal integrity mismatch")?;
            state.apply(&self.manifest,&e.operation)?;
            state.sequence=seq;state.tail=envelope.digest;
        }
        Ok(state)
    }
    fn commit(&self,mut state:State,op:Operation)->Result<Value>{
        if let Operation::Write{request:r}=&op{
            ensure(r.epoch==state.epoch,"stale vault writer")?;
            if let Some((pin,receipt))=state.requests.get(&r.request_id){
                ensure(pin==&checksum(r)?,"vault request ID conflict")?;return Ok(receipt.clone());
            }
        }
        ensure(state.sequence<self.manifest.limits.max_events,"vault event budget exhausted")?;
        let response=state.apply(&self.manifest,&op)?;
        let sequence=state.sequence+1;
        let event=Event{sequence,previous:state.tail,manifest:self.manifest_digest.clone(),operation:op};
        let envelope=Envelope{digest:checksum(&event)?,event};
        write_new(&self.root.join("events"),&format!("{sequence:020}.json"),&envelope)?;
        Ok(response)
    }
    pub fn write(&self,request:WriteRequest)->Result<Value>{
        let _lock=self.lock()?;self.commit(self.load()?,Operation::Write{request})
    }
    pub fn advance(&self,expected_epoch:u64)->Result<Value>{
        let _lock=self.lock()?;self.commit(self.load()?,Operation::Advance{expected_epoch})
    }
    pub fn recover(&self)->Result<Value>{
        let _lock=self.lock()?;let s=self.load()?;
        Ok(json!({"run_id":self.manifest.run_id,"arm":self.manifest.arm,"allowed_authorities":self.manifest.arm.authority(),
            "manifest_digest":self.manifest_digest,"epoch":s.epoch,"sequence":s.sequence,"tail":s.tail,
            "records":s.records.values().collect::<Vec<_>>(),"isolation":"host_gateway_only","acceptance_credit":false}))
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    use std::time::{SystemTime,UNIX_EPOCH};
    struct Temp(PathBuf);impl Drop for Temp{fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
    #[test]
    fn session_arm_cannot_persist_goals_and_stale_epoch_cannot_write(){
        let spec=Spec{id:"cohort".into(),version:1,seed:5,repeats:1,workload_digest:digest(&json!(42)),max_wall_ms:1000,max_requests:3,max_tokens:100,max_concurrent_children:1,faults:vec![]};
        let plan=spec.plan().unwrap();
        let runs=plan["runs"].as_array().unwrap();
        let run=runs.iter().find(|r|r["arm"]=="session").unwrap()["run_id"].as_str().unwrap();
        let foreign=runs.iter().find(|r|r["arm"]=="intelligence").unwrap()["run_id"].as_str().unwrap();
        let temp=Temp(std::env::temp_dir().join(format!("vivim-vault-{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())));
        let v=Vault::create(&temp.0,&spec,run,Limits{max_events:10,max_records:4,max_payload_bytes:1024}).unwrap();
        let denied=WriteRequest{request_id:"denied".into(),epoch:1,key:"goal".into(),expected_version:0,record:Record::Goal{goal:"g".into(),objective:"not permitted".into()}};
        assert!(v.write(denied).is_err());
        let r=WriteRequest{request_id:"one".into(),epoch:1,key:"session".into(),expected_version:0,record:Record::NativeSession{session_ref:"opaque-session".into()}};
        assert_eq!(v.write(r.clone()).unwrap(),v.write(r.clone()).unwrap());
        assert_eq!(v.advance(1).unwrap()["epoch"],2);assert!(v.write(r).is_err());
        assert!(Vault::open(&temp.0,&spec,foreign).is_err());
        let recovered=Vault::open(&temp.0,&spec,run).unwrap().recover().unwrap();
        assert_eq!(recovered["records"].as_array().unwrap().len(),1);
        assert_eq!(recovered["sequence"],2);assert_eq!(recovered["epoch"],2);
    }
}
