//! Bounded revision journal with a durable head, incremental replay and compaction.
//! Host-local storage: one cooperating writer at a time; no provider or database service.
use super::{model::*,transition::{Action,Actor,Request},evolution::Discovery};
use crate::{digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::{collections::BTreeMap,fs::{self,File,OpenOptions},io::Write,path::{Path,PathBuf},time::{SystemTime,UNIX_EPOCH}};

fn error(e:impl std::fmt::Display)->String{e.to_string()}
fn encode<T:Serialize>(x:&T)->Result<Vec<u8>>{serde_json::to_vec(x).map_err(error)}
fn checksum<T:Serialize>(x:&T)->Result<String>{Ok(crate::hash(&encode(x)?))}
fn read<T:for<'de> Deserialize<'de>>(path:&Path)->Result<T>{serde_json::from_slice(&fs::read(path).map_err(error)?).map_err(error)}
fn sync_directory(path:&Path)->Result<()>{
    #[cfg(unix)] File::open(path).and_then(|f|f.sync_all()).map_err(error)?;
    #[cfg(not(unix))] let _=path;
    Ok(())
}
fn write_new<T:Serialize>(root:&Path,name:&str,x:&T)->Result<()>{
    let stamp=SystemTime::now().duration_since(UNIX_EPOCH).map_err(error)?.as_nanos();
    let tmp=root.join(format!(".tmp-{}-{stamp}",std::process::id()));
    let target=root.join(name);ensure(!target.exists(),"immutable destination already exists")?;
    let mut f=OpenOptions::new().create_new(true).write(true).open(&tmp).map_err(error)?;
    f.write_all(&encode(x)?).map_err(error)?;f.sync_all().map_err(error)?;drop(f);
    fs::rename(&tmp,&target).map_err(error)?;sync_directory(root)
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Receipt {pub request_digest:String,pub response:Value,pub revision:u64}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Snapshot {schema:u32,revision:u64,previous:String,world:World,receipts:BTreeMap<Id,Receipt>}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Record {schema:u32,revision:u64,previous:String,command:Transaction,response:Value}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Envelope<T>{checksum:String,body:T}
impl<T:Serialize> Envelope<T>{
    fn new(body:T)->Result<Self>{Ok(Self{checksum:checksum(&body)?,body})}
    fn check(&self)->Result<()>{ensure(checksum(&self.body)?==self.checksum,"journal checksum mismatch")}
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(tag="kind",rename_all="snake_case")]
pub enum Transaction {
    Command{request:Request},
    Assimilate{id:Id,name:Id,version:u64,discovery:Discovery},
}
impl Transaction{
    fn id(&self)->&str{match self{Self::Command{request}=>&request.id,Self::Assimilate{id,..}=>id}}
    fn run(&self,w:&mut World)->Result<Value>{
        match self{
            Self::Command{request}=>w.transition(&request.actor,request.action.clone()),
            Self::Assimilate{name,version,discovery,..}=>{
                let key=w.assimilate(name,*version,discovery.clone())?;
                w.sequence=w.sequence.checked_add(1).ok_or("sequence overflow")?;
                Ok(json!({"capability":key}))
            }
        }
    }
}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Head {schema:u32,revision:u64,record:String,snapshot:u64}
/// The head is append-only; a transaction is committed only when its marker exists.
/// A missing final marker cannot be distinguished from a crash before admission without
/// an external anchor. An independent anchor can be exported by the caller.
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Anchor {pub revision:u64,pub digest:String}
struct Loaded {world:World,receipts:BTreeMap<Id,Receipt>,head:Head}
pub struct KernelStore {root:PathBuf}
const SNAP_INTERVAL:u64=64;
impl KernelStore{
    pub fn open(root:impl AsRef<Path>)->Result<Self>{
        fs::create_dir_all(root.as_ref()).map_err(error)?;
        Ok(Self{root:root.as_ref().to_path_buf()})
    }
    pub fn root(&self)->&Path{&self.root}
    fn lock(&self)->Result<File>{
        let f=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(self.root.join("kernel.lock")).map_err(error)?;
        f.lock().map_err(error)?;Ok(f)
    }
    fn name(kind:&str,revision:u64)->String{format!("{revision:020}.{kind}.json")}
    fn path(&self,kind:&str,revision:u64)->PathBuf{self.root.join(Self::name(kind,revision))}
    fn heads(&self)->Result<Vec<u64>>{
        let mut v=vec![];
        for e in fs::read_dir(&self.root).map_err(error)?{
            let name=e.map_err(error)?.file_name().to_string_lossy().into_owned();
            if let Some(n)=name.strip_suffix(".head.json"){v.push(n.parse::<u64>().map_err(error)?);}
        }
        v.sort_unstable();Ok(v)
    }
    fn load(&self)->Result<Loaded>{
        let heads=self.heads()?;
        let Some(&latest)=heads.last() else{
            return Ok(Loaded{world:World::default(),receipts:BTreeMap::new(),head:Head{schema:1,revision:0,record:"genesis".into(),snapshot:0}})
        };
        let top:Envelope<Head>=read(&self.path("head",latest))?;top.check()?;
        ensure(top.body.schema==1&&top.body.revision==latest&&top.body.snapshot<=latest,"invalid head")?;
        let mut world=World::default();let mut receipts=BTreeMap::new();let mut previous="genesis".to_string();
        let start=top.body.snapshot;
        if start>0{
            let snap:Envelope<Snapshot>=read(&self.path("snapshot",start))?;snap.check()?;
            ensure(snap.body.schema==1&&snap.body.revision==start,"snapshot revision mismatch")?;
            let marker:Envelope<Head>=read(&self.path("head",start))?;marker.check()?;
            ensure(marker.body.revision==start&&marker.body.record==snap.body.previous,"snapshot anchor mismatch")?;
            previous=snap.body.previous;world=snap.body.world;receipts=snap.body.receipts;
        }
        for rev in (start+1)..=latest{
            let marker:Envelope<Head>=read(&self.path("head",rev))?;marker.check()?;
            let rec:Envelope<Record>=read(&self.path("record",rev))?;rec.check()?;
            ensure(marker.body.schema==1&&marker.body.revision==rev&&marker.body.record==rec.checksum,"head record mismatch")?;
            ensure(rec.body.schema==1&&rec.body.revision==rev&&rec.body.previous==previous,"record chain mismatch")?;
            let result=rec.body.command.run(&mut world)?;
            ensure(result==rec.body.response,"replay response mismatch")?;
            let req=rec.body.command.id().to_string();
            ensure(receipts.insert(req,Receipt{request_digest:checksum(&rec.body.command)?,response:result,revision:rev}).is_none(),"duplicate request")?;
            previous=rec.checksum;
        }
        ensure(previous==top.body.record,"tail digest mismatch")?;
        Ok(Loaded{world,receipts,head:top.body})
    }
    pub fn inspect(&self)->Result<World>{let _guard=self.lock()?;Ok(self.load()?.world)}
    pub fn anchor(&self)->Result<Anchor>{
        let _guard=self.lock()?;let s=self.load()?;Ok(Anchor{revision:s.head.revision,digest:s.head.record})
    }
    pub fn verify_anchor(&self,anchor:&Anchor)->Result<()>{
        let current=self.anchor()?;
        ensure(current.revision==anchor.revision&&current.digest==anchor.digest,"external anchor mismatch")
    }
    pub fn apply(&self,request:Request)->Result<Value>{self.commit(Transaction::Command{request})}
    pub fn assimilate(&self,request_id:Id,name:Id,version:u64,discovery:Discovery)->Result<Value>{
        self.commit(Transaction::Assimilate{id:request_id,name,version,discovery})
    }
    fn commit(&self,command:Transaction)->Result<Value>{
        id(command.id())?;
        let _guard=self.lock()?;let mut loaded=self.load()?;
        let request_digest=checksum(&command)?;
        if let Some(receipt)=loaded.receipts.get(command.id()){
            ensure(receipt.request_digest==request_digest,"request ID conflict")?;return Ok(receipt.response.clone())
        }
        let response=command.run(&mut loaded.world)?;
        let rev=loaded.head.revision.checked_add(1).ok_or("revision overflow")?;
        // Record/snapshot without head are orphan preparations from a prior interrupted commit.
        for kind in ["record","snapshot"]{let p=self.path(kind,rev);if p.exists(){fs::remove_file(p).map_err(error)?;}}
        let request_id=command.id().to_string();
        let rec=Envelope::new(Record{schema:1,revision:rev,previous:loaded.head.record.clone(),command,response:response.clone()})?;
        write_new(&self.root,&Self::name("record",rev),&rec)?;
        loaded.receipts.insert(request_id,Receipt{request_digest,response:response.clone(),revision:rev});
        let snapshot=if rev%SNAP_INTERVAL==0{
            let snap=Envelope::new(Snapshot{schema:1,revision:rev,previous:rec.checksum.clone(),world:loaded.world,receipts:loaded.receipts})?;
            write_new(&self.root,&Self::name("snapshot",rev),&snap)?;rev
        }else{loaded.head.snapshot};
        let head=Envelope::new(Head{schema:1,revision:rev,record:rec.checksum,snapshot})?;
        write_new(&self.root,&Self::name("head",rev),&head)?;
        // Keep the current checkpoint and its committed tail; pruning is retryable and
        // cannot invalidate the just-committed transaction.
        if snapshot>0{let _=self.prune(snapshot);}
        Ok(response)
    }
    fn prune(&self,snapshot:u64)->Result<()>{
        for e in fs::read_dir(&self.root).map_err(error)?{
            let e=e.map_err(error)?;let name=e.file_name().to_string_lossy().into_owned();
            let Some((n,kind))=name.split_once('.') else{continue};
            let Ok(rev)=n.parse::<u64>() else{continue};
            if rev<snapshot&&matches!(kind,"head.json"|"record.json"|"snapshot.json"){fs::remove_file(e.path()).map_err(error)?;}
            if rev==snapshot&&kind=="record.json"{fs::remove_file(e.path()).map_err(error)?;}
        }
        sync_directory(&self.root)
    }
    pub fn project_agent(&self,agent:&str,root:&Path)->Result<PathBuf>{
        id(agent)?;let world=self.inspect()?;let a=world.agents.get(agent).ok_or("agent missing")?;
        let path=root.join(agent);fs::create_dir_all(path.join("workspace")).map_err(error)?;
        fs::create_dir_all(path.join("incarnations")).map_err(error)?;
        let identity=json!({"id":a.id,"goal":a.goal,"parent":a.parent,"class":a.class,"context":a.context});
        let snapshot=Envelope::new(json!({"identity":identity,"phase":a.phase,"agent":a,"mailbox":world.messages.values().filter(|m|m.recipient==agent).collect::<Vec<_>>()}))?;
        let name=format!("{:020}.projection.json",world.sequence);
        if !path.join(&name).exists(){write_new(&path,&name,&snapshot)?;}
        Ok(path)
    }
}
