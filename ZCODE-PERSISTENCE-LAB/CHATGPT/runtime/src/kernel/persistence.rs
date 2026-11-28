//! Bounded revision journal with a durable head, incremental replay and compaction.
//! Host-local storage: one cooperating writer at a time; no provider or database service.
use super::{model::*,transition::{Action,Actor,Request},evolution::Discovery};
use crate::{digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::{collections::{BTreeMap,BTreeSet},fs::{self,File,OpenOptions},io::Write,path::{Path,PathBuf},time::{SystemTime,UNIX_EPOCH,Instant},sync::Mutex};

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
struct Record {schema:u32,revision:u64,previous:String,command:Transaction,response:Value,#[serde(default,skip_serializing_if="Option::is_none")] index:Option<String>,#[serde(default,skip_serializing_if="Option::is_none")] host_measurement:Option<super::resource::HostCost>}
impl Record{
    fn verify_measurement(&self,w:&World)->Result<()>{
        if let Some(m)=&self.host_measurement{
            let Transaction::Command{request}=&self.command else{return Err("measurement requires host command".into());};
            let operation=super::resource::measured_operation(&request.action).ok_or("unsupported measured action")?;
            ensure(request.actor==Actor::Host&&m.request==request.id&&m.operation==format!("transition-{operation}")
                &&m.result_digest.as_ref()==Some(&digest(&self.response)),"measurement receipt identity mismatch")?;
            let stored=w.host_costs.get(&m.request).ok_or("indexed host measurement missing")?;
            ensure(encode(stored)?==encode(m)?,"indexed host measurement mismatch")?;
        }
        Ok(())
    }
}
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
    ImportLegacy{id:Id,state:crate::State},
}
impl Transaction{
    fn id(&self)->&str{match self{Self::Command{request}=>&request.id,Self::Assimilate{id,..}|Self::ImportLegacy{id,..}=>id}}
    fn run(&self,w:&mut World)->Result<Value>{
        match self{
            Self::Command{request}=>w.transition_owned(&request.actor,request.action.clone()),
            Self::ImportLegacy{state,..}=>{
                ensure(w.sequence==0,"migration requires a fresh kernel store")?;
                let (imported,report)=super::migration::import(state)?;*w=imported;
                serde_json::to_value(report).map_err(error)
            }
            Self::Assimilate{name,version,discovery,..}=>{
                let key=w.assimilate(name,*version,discovery.clone())?;
                w.sequence=w.sequence.checked_add(1).ok_or("sequence overflow")?;
                Ok(json!({"capability":key}))
            }
        }
    }
}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Head {schema:u32,revision:u64,record:String,snapshot:u64,snapshot_digest:Option<String>,#[serde(default,skip_serializing_if="Option::is_none")] index:Option<String>}
/// The head is append-only; a transaction is committed only when its marker exists.
/// A missing final marker cannot be distinguished from a crash before admission without
/// an external anchor. An independent anchor can be exported by the caller.
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Anchor {pub revision:u64,pub digest:String}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct ArchivedRevision{
    head:Envelope<Head>,record:Envelope<Record>,
    #[serde(default,skip_serializing_if="Option::is_none")] snapshot:Option<Envelope<Snapshot>>,
}
impl ArchivedRevision{
    fn check(&self,revision:u64)->Result<()>{
        self.head.check()?;self.record.check()?;
        let h=&self.head.body;let r=&self.record.body;
        ensure(h.schema==1&&r.schema==1&&h.revision==revision&&r.revision==revision
            &&h.record==self.record.checksum&&h.index==r.index,"archived revision mismatch")?;
        if let Some(s)=&self.snapshot{
            s.check()?;ensure(s.body.revision==revision&&s.body.previous==h.record
                &&h.snapshot==revision&&h.snapshot_digest.as_ref()==Some(&s.checksum),"archived snapshot mismatch")?;
        }
        Ok(())
    }
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct HistoricalPin{pub name:Id,pub revision:u64,pub anchor:Anchor,pub index:Digest}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetentionPolicy{
    pub entry_budget_per_area:usize,
    pub max_archive_bytes:Option<u64>,
    pub max_index_bytes:Option<u64>,
    pub max_pin_files:Option<u64>,
}
#[derive(Default,Serialize)]
struct Inventory{
    files:u64,bytes:u64,visited:usize,complete:bool,
}
impl Inventory{
    fn scan(root:&Path,budget:usize)->Result<Self>{
        if !root.exists(){return Ok(Self{complete:true,..Self::default()});}
        ensure(fs::symlink_metadata(root).map_err(error)?.file_type().is_dir(),"retention area not regular directory")?;
        // Stack of lazy directory iterators avoids collecting all paths into RAM.
        let mut stack=vec![fs::read_dir(root).map_err(error)?];
        let mut out=Self::default();
        while let Some(iter)=stack.last_mut(){
            let Some(item)=iter.next() else{stack.pop();continue;};
            if out.visited>=budget{return Ok(out);}
            out.visited+=1;
            let item=item.map_err(error)?;let ty=item.file_type().map_err(error)?;
            if ty.is_dir(){
                ensure(stack.len()<16,"retention directory depth exceeded")?;
                stack.push(fs::read_dir(item.path()).map_err(error)?);
            }else if ty.is_file(){
                out.files=out.files.checked_add(1).ok_or("inventory file count overflow")?;
                out.bytes=out.bytes.checked_add(item.metadata().map_err(error)?.len()).ok_or("inventory byte count overflow")?;
            }else{return Err("nonregular entry in retention area; no traversal".into());}
        }
        out.complete=true;Ok(out)
    }
    fn status(&self,value:u64,limit:Option<u64>)->&'static str{
        match limit{None=>"not_configured",Some(n) if value>n=>"over_limit",Some(_) if self.complete=>"within_limit",_=>"unknown_incomplete_inventory"}
    }
}
#[derive(Clone)]
struct Loaded {world:World,receipts:BTreeMap<Id,Receipt>,head:Head,lazy_receipts:bool}
/// Explicit acceleration for a trusted cooperating local host. Does not
/// claim that unvisited files have been reverified since the last full read.
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HotPolicy{pub max_cached_accesses:u64}
impl HotPolicy{
    pub fn validate(&self)->Result<()>{
        ensure((1..=4096).contains(&self.max_cached_accesses),"invalid resident-state access budget")
    }
}
#[derive(Default)]
struct Resident{
    state:Option<Loaded>,accesses:u64,full_loads:u64,hits:u64,invalidations:u64,
}

impl Loaded{
    fn receipt(&self,index:&super::index::Index,key:&str)->Result<Option<Receipt>>{
        if self.lazy_receipts{
            let root=self.head.index.as_ref().ok_or("lazy receipt root missing")?;
            index.lookup(root,self.head.revision,"receipts",key)?
                .map(|v|serde_json::from_value(v).map_err(error)).transpose()
        }else{Ok(self.receipts.get(key).cloned())}
    }
}
pub struct KernelStore {root:PathBuf,hot:Option<HotPolicy>,resident:Mutex<Resident>}
const SNAP_INTERVAL:u64=64;
impl KernelStore{
    pub fn open(root:impl AsRef<Path>)->Result<Self>{
        fs::create_dir_all(root.as_ref()).map_err(error)?;
        Ok(Self{root:root.as_ref().to_path_buf(),hot:None,resident:Mutex::new(Resident::default())})
    }
    pub fn open_hot(root:impl AsRef<Path>,policy:HotPolicy)->Result<Self>{
        policy.validate()?;let mut store=Self::open(root)?;store.hot=Some(policy);Ok(store)
    }
    pub fn resident_report(&self)->Result<Value>{
        let r=self.resident.lock().map_err(|_|"resident cache poisoned")?;
        Ok(json!({"enabled":self.hot.is_some(),"policy":self.hot,"cached_revision":r.state.as_ref().map(|s|s.head.revision),
            "cached_accesses_since_full_load":r.accesses,"full_loads":r.full_loads,"hits":r.hits,"invalidations":r.invalidations,
            "integrity_scope":"head_tail_and_latest_receipt_per_hit_full_world_at_reload"}))
    }
    pub fn root(&self)->&Path{&self.root}
    pub(crate) fn controller_lock(&self)->Result<File>{
        let f=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(self.root.join("controller.lock")).map_err(error)?;
        f.try_lock().map_err(|e|format!("controller already active or lock failed: {e:?}"))?;Ok(f)
    }
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
    fn latest_head(&self)->Result<Option<Head>>{
        let Some(rev)=self.heads()?.last().copied() else{return Ok(None);};
        let head:Envelope<Head>=read(&self.path("head",rev))?;head.check()?;
        ensure(head.body.schema==1&&head.body.revision==rev&&head.body.snapshot<=rev,"invalid head")?;
        Ok(Some(head.body))
    }
    fn load(&self)->Result<Loaded>{self.load_resident(false)}
    fn load_for_commit(&self)->Result<Loaded>{self.load_resident(true)}
    fn load_resident(&self,take:bool)->Result<Loaded>{
        let Some(policy)=&self.hot else{return self.load_uncached();};
        // All callers already hold the cooperating filesystem writer lock.
        // Take ownership for mutations: a rejected transition cannot leave a
        // partially mutated resident snapshot, even if the error is retried.
        let mut r=self.resident.lock().map_err(|_|"resident cache poisoned")?;
        let cached=r.state.take();
        if let Some(s)=cached{
            let current=self.latest_head()?;
            let same=match &current{Some(h)=>checksum(h)?==checksum(&s.head)?,None=>false};
            if same&&r.accesses<policy.max_cached_accesses{
                let h=current.as_ref().unwrap();let root=h.index.as_ref().ok_or("resident index missing")?;
                let index=super::index::Index::open(&self.root)?;
                let manifest=index.manifest(root)?;
                ensure(manifest.schema==4&&manifest.revision==h.revision&&s.world.sequence==h.revision,"resident revision mismatch")?;
                let last:Envelope<Record>=read(&self.path("record",h.revision))?;last.check()?;
                ensure(last.body.revision==h.revision&&last.body.index==h.index&&last.checksum==h.record,"resident tail mismatch")?;
                let receipt=s.receipt(&index,last.body.command.id())?.ok_or("resident tail receipt missing")?;
                ensure(receipt.revision==h.revision&&receipt.request_digest==checksum(&last.body.command)?&&receipt.response==last.body.response,"resident receipt mismatch")?;
                last.body.verify_measurement(&s.world)?;
                r.accesses+=1;r.hits=r.hits.saturating_add(1);
                if take{return Ok(s);}
                let out=s.clone();r.state=Some(s);return Ok(out);
            }
            r.invalidations=r.invalidations.saturating_add(1);
        }
        let s=self.load_uncached()?;
        r.full_loads=r.full_loads.saturating_add(1);r.accesses=0;
        if !take&&s.lazy_receipts{r.state=Some(s.clone());}
        Ok(s)
    }
    fn load_uncached(&self)->Result<Loaded>{
        let heads=self.heads()?;
        let Some(&latest)=heads.last() else{
            return Ok(Loaded{world:World::default(),receipts:BTreeMap::new(),head:Head{schema:1,revision:0,record:"genesis".into(),snapshot:0,snapshot_digest:None,index:None},lazy_receipts:false})
        };
        let top:Envelope<Head>=read(&self.path("head",latest))?;top.check()?;
        ensure(top.body.schema==1&&top.body.revision==latest&&top.body.snapshot<=latest,"invalid head")?;
        if let Some(root)=&top.body.index{
            let index=super::index::Index::open(&self.root)?;
            let lazy_receipts=index.manifest(root)?.schema==4;
            let (world,receipts)=if lazy_receipts{(index.load_world(root,latest)?,BTreeMap::new())}else{index.load(root,latest)?};
            let lookup_receipt=|key:&str|->Result<Receipt>{
                if lazy_receipts{
                    let v=index.lookup(root,latest,"receipts",key)?.ok_or("indexed receipt missing")?;
                    serde_json::from_value(v).map_err(error)
                }else{receipts.get(key).cloned().ok_or("indexed receipt missing".into())}
            };
            // Verify the bounded chain without re-running potentially expensive
            // learning/computation commands. Each retained marker names its index.
            let start=top.body.snapshot;
            let mut previous=if start==0{"genesis".to_string()}else{
                let marker:Envelope<Head>=read(&self.path("head",start))?;marker.check()?;
                ensure(marker.body.revision==start&&marker.body.index==marker.body.snapshot_digest&&marker.body.snapshot==start&&marker.body.index==top.body.snapshot_digest,"checkpoint head mismatch")?;
                let rec:Envelope<Record>=read(&self.path("record",start))?;rec.check()?;
                ensure(rec.body.revision==start&&rec.body.index==marker.body.index&&rec.checksum==marker.body.record,"checkpoint record mismatch")?;
                let receipt=lookup_receipt(rec.body.command.id())?;
                ensure(receipt.revision==start&&receipt.request_digest==checksum(&rec.body.command)?&&receipt.response==rec.body.response,"checkpoint receipt mismatch")?;
                rec.body.verify_measurement(&world)?;
                marker.body.record
            };
            for rev in (start+1)..=latest{
                let marker:Envelope<Head>=read(&self.path("head",rev))?;marker.check()?;
                let rec:Envelope<Record>=read(&self.path("record",rev))?;rec.check()?;
                ensure(marker.body.revision==rev&&marker.body.record==rec.checksum&&rec.body.revision==rev&&rec.body.previous==previous&&rec.body.index==marker.body.index,"indexed journal chain mismatch")?;
                let receipt=lookup_receipt(rec.body.command.id())?;
                ensure(receipt.revision==rev&&receipt.request_digest==checksum(&rec.body.command)?&&receipt.response==rec.body.response,"indexed receipt mismatch")?;
                rec.body.verify_measurement(&world)?;
                previous=rec.checksum;
            }
            ensure(previous==top.body.record&&world.sequence==latest,"indexed tail mismatch")?;
            let last:Envelope<Record>=read(&self.path("record",latest))?;last.check()?;
            ensure(last.body.index==top.body.index&&last.checksum==top.body.record,"latest index record mismatch")?;
            return Ok(Loaded{world,receipts,head:top.body,lazy_receipts});
        }
        let mut world=World::default();let mut receipts=BTreeMap::new();let mut previous="genesis".to_string();
        let start=top.body.snapshot;
        if start>0{
            let snap:Envelope<Snapshot>=read(&self.path("snapshot",start))?;snap.check()?;
            ensure(snap.body.schema==1&&snap.body.revision==start,"snapshot revision mismatch")?;
            let marker:Envelope<Head>=read(&self.path("head",start))?;marker.check()?;
            ensure(marker.body.revision==start&&marker.body.record==snap.body.previous&&marker.body.snapshot_digest.as_ref()==Some(&snap.checksum)&&top.body.snapshot_digest.as_ref()==Some(&snap.checksum),"snapshot anchor mismatch")?;
            previous=snap.body.previous;world=snap.body.world;receipts=snap.body.receipts;
        }
        for rev in (start+1)..=latest{
            let marker:Envelope<Head>=read(&self.path("head",rev))?;marker.check()?;
            let rec:Envelope<Record>=read(&self.path("record",rev))?;rec.check()?;
            ensure(marker.body.schema==1&&marker.body.revision==rev&&marker.body.record==rec.checksum,"head record mismatch")?;
            ensure(rec.body.schema==1&&rec.body.revision==rev&&rec.body.previous==previous,"record chain mismatch")?;
            let result=rec.body.command.run(&mut world)?;
            ensure(result==rec.body.response,"replay response mismatch")?;
            if let Some(m)=&rec.body.host_measurement{world.record_host_cost(m.clone())?;rec.body.verify_measurement(&world)?;}
            let req=rec.body.command.id().to_string();
            ensure(receipts.insert(req,Receipt{request_digest:checksum(&rec.body.command)?,response:result,revision:rev}).is_none(),"duplicate request")?;
            previous=rec.checksum;
        }
        ensure(previous==top.body.record,"tail digest mismatch")?;
        Ok(Loaded{world,receipts,head:top.body,lazy_receipts:false})
    }
    pub fn lookup(&self,table:&str,key:&str)->Result<Option<Value>>{
        let _guard=self.lock()?;
        let Some(head)=self.latest_head()? else{return Ok(None);};
        if let Some(root)=head.index.as_ref(){
            let last:Envelope<Record>=read(&self.path("record",head.revision))?;last.check()?;
            ensure(last.body.revision==head.revision&&last.body.index==head.index&&last.checksum==head.record,"lookup index record mismatch")?;
            return super::index::Index::open(&self.root)?.lookup(root,head.revision,table,key);
        }
        let loaded=self.load()?;
        if table=="receipts"{return loaded.receipts.get(key).map(|r|serde_json::to_value(r).map_err(error)).transpose();}
        let v=serde_json::to_value(loaded.world).map_err(error)?;let mut node=&v;for part in table.split('.'){let Some(next)=node.get(part) else{return Ok(None);};node=next;}Ok(node.get(key).cloned())
    }
    pub fn scan(&self,table:&str,cursor:Option<super::index::ScanCursor>,limit:usize,max_pages:usize,max_bytes:usize)->Result<Value>{
        let _guard=self.lock()?;
        let head=self.latest_head()?.ok_or("scan requires committed indexed state")?;
        let root=head.index.as_ref().ok_or("legacy store requires migration before paged scan")?;
        let last:Envelope<Record>=read(&self.path("record",head.revision))?;last.check()?;
        ensure(last.body.revision==head.revision&&last.body.index==head.index&&last.checksum==head.record,"scan index record mismatch")?;
        let page=super::index::Index::open(&self.root)?.scan(root,head.revision,table,cursor,limit,max_pages,max_bytes)?;
        Ok(json!({"anchor":Anchor{revision:head.revision,digest:checksum(&head)?},"page":page,"integrity":"visited_pages_and_current_head_only"}))
    }
    pub fn inspect(&self)->Result<World>{let _guard=self.lock()?;Ok(self.load()?.world)}
    /// A consistent observation, rather than two separately acquired snapshots.
    pub fn inspect_with_anchor(&self)->Result<(World,Anchor)>{
        let _guard=self.lock()?;let s=self.load_uncached()?;
        let anchor=Anchor{revision:s.head.revision,digest:checksum(&s.head)?};
        Ok((s.world,anchor))
    }
    pub fn anchor(&self)->Result<Anchor>{
        let _guard=self.lock()?;let s=self.load_uncached()?;Ok(Anchor{revision:s.head.revision,digest:checksum(&s.head)?})
    }
    pub fn verify_anchor(&self,anchor:&Anchor)->Result<()>{
        let current=self.anchor()?;
        ensure(current.revision==anchor.revision&&current.digest==anchor.digest,"external anchor mismatch")
    }
    pub fn apply(&self,request:Request)->Result<Value>{
        ensure(!matches!(&request.action,Action::AttestCandidate{..}|Action::AttestTracePattern{..})&&!matches!(&request.actor,Actor::Adapter{..}),"custody and adapter actions require signed ingress")?;
        self.commit(Transaction::Command{request})
    }
    /// Meter only a whitelisted successful deterministic transition, before
    /// serializing its state. The same head commits both action and receipt.
    /// Lock/load/write latency and failed/uncertain actions are not measured here.
    pub fn apply_metered(&self,request:Request,operation:&str)->Result<Value>{
        ensure(request.actor==Actor::Host&&super::resource::measured_operation(&request.action)==Some(operation),"unsupported metered host operation")?;
        let _guard=self.lock()?;let loaded=self.load_for_commit()?;
        self.commit_loaded_metered(Transaction::Command{request},loaded,Some(operation))
    }
    pub fn assimilate(&self,request_id:Id,name:Id,version:u64,discovery:Discovery)->Result<Value>{
        self.commit(Transaction::Assimilate{id:request_id,name,version,discovery})
    }
    pub fn import_r003(&self,request_id:Id,source:&Path)->Result<Value>{
        ensure(source.is_dir(),"legacy source store missing")?;
        ensure(source.canonicalize().map_err(error)?!=self.root.canonicalize().map_err(error)?,"source and destination must differ")?;
        let old=crate::Store::open(source)?;let state=old.inspect()?;
        self.commit(Transaction::ImportLegacy{id:request_id,state})
    }
    pub fn apply_signed(&self,authority:&super::authority::Authority,signed:super::authority::SignedRequest,now_ms:u64)->Result<Value>{
        authority.authenticate(&signed,now_ms)?;
        let _guard=self.lock()?;let loaded=self.load_for_commit()?;
        let command=Transaction::Command{request:signed.request.clone()};
        if loaded.receipt(&super::index::Index::open(&self.root)?,command.id())?.is_some(){authority.authorize_replay(&signed,&loaded.world,now_ms)?;}else{authority.authorize(&signed,&loaded.world,now_ms)?;}
        self.commit_loaded(command,loaded)
    }
    /// Explicit trusted-host action. Unknown effects can only be observed, not
    /// executed. A publication and its receipt can be separated by a crash;
    /// the next observation proves the existing immutable object.
    pub fn local_effect(&self,effect:&str,publish:bool)->Result<Value>{
        id(effect)?;
        let _guard=self.lock()?;let loaded=self.load_for_commit()?;
        let e=loaded.world.effects.get(effect).ok_or("effect missing")?.clone();
        if e.phase==EffectPhase::Committed{return Ok(json!({"effect":effect,"phase":"committed","receipt":e.receipt}));}
        ensure(matches!(e.phase,EffectPhase::Prepared|EffectPhase::Unknown),"effect is not reconcilable")?;
        if publish{
            let t=loaded.world.tasks.get(&e.task).ok_or("effect task missing")?;
            ensure(e.phase==EffectPhase::Prepared&&t.phase==TaskPhase::Running&&t.generation==e.generation&&t.lease.as_ref().is_some_and(|l|loaded.world.clock_ms<l.deadline_ms)&&loaded.world.goals.get(&t.spec.goal).is_some_and(|g|g.phase==GoalPhase::Open),"publication no longer authorized")?;
        }
        let payload=loaded.world.payload(&e.payload)?;
        if publish{
            let proposed=super::local_effect::receipt(&e,payload,EffectPhase::Committed)?;
            let input=json!({"effect":effect,"task":e.task,"generation":e.generation,"payload":payload,"outcome":EffectPhase::Committed});
            ensure(e.oracle.evaluate(&input,&proposed)?,"publication oracle rejects intended receipt")?;
        }
        let (outcome,receipt)=super::local_effect::observe(&self.root,&e,payload,publish)?;
        let request= Request{id:format!("local-{}",digest(&json!([effect,e.generation,&receipt]))),actor:Actor::Host,action:Action::ResolveEffect{effect:effect.into(),outcome,receipt}};
        self.commit_loaded(Transaction::Command{request},loaded)
    }
    fn commit(&self,command:Transaction)->Result<Value>{
        let _guard=self.lock()?;let loaded=self.load_for_commit()?;self.commit_loaded(command,loaded)
    }
    fn commit_loaded(&self,command:Transaction,loaded:Loaded)->Result<Value>{
        self.commit_loaded_metered(command,loaded,None)
    }
    fn commit_loaded_metered(&self,command:Transaction,mut loaded:Loaded,operation:Option<&str>)->Result<Value>{
        id(command.id())?;
        let request_digest=checksum(&command)?;
        let index=super::index::Index::open(&self.root)?;
        if let Some(receipt)=loaded.receipt(&index,command.id())?{
            ensure(receipt.request_digest==request_digest,"request ID conflict")?;
            if let Some(operation)=operation{
                ensure(loaded.world.host_costs.get(command.id()).is_some_and(|m|m.operation==format!("transition-{operation}")&&m.result_digest.as_ref()==Some(&digest(&receipt.response))),"existing receipt has no matching atomic measurement")?;
            }
            if let Transaction::Command{request}= &command{
                if let Action::ClaimHandoff{handoff}|Action::AuthorizeMailboxTake{handoff,..}=&request.action{return Ok(json!({"handoff_id":handoff,"replayed":true,"send_authority":"none_reconcile_only"}));}
            }
            let replay_response=receipt.response.clone();
            // A replay consumes no new durable revision. Keep the old
            // resident state only if it was fully eligible for reuse.
            if self.hot.is_some()&&loaded.lazy_receipts{
                self.resident.lock().map_err(|_|"resident cache poisoned")?.state=Some(loaded);
            }
            return Ok(replay_response)
        }
        let start=Instant::now();
        let response=command.run(&mut loaded.world)?;
        let host_measurement=if let Some(operation)=operation{
            let m=super::resource::HostCost{request:command.id().into(),operation:format!("transition-{operation}"),
                elapsed_ms:u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX),result_digest:Some(digest(&response))};
            loaded.world.record_host_cost(m.clone())?;Some(m)
        }else{None};
        let rev=loaded.head.revision.checked_add(1).ok_or("revision overflow")?;
        // Record/snapshot without head are orphan preparations from a prior interrupted commit.
        for kind in ["record","snapshot"]{let p=self.path(kind,rev);if p.exists(){fs::remove_file(p).map_err(error)?;}}
        let request_id=command.id().to_string();
        let receipt=Receipt{request_digest,response:response.clone(),revision:rev};
        let index_root=index.write_tracked(rev,&loaded.world,&loaded.receipts,&request_id,&receipt,loaded.head.index.as_deref())?;
        let rec=Envelope::new(Record{schema:1,revision:rev,previous:loaded.head.record.clone(),command,response:response.clone(),index:Some(index_root.clone()),host_measurement})?;
        write_new(&self.root,&Self::name("record",rev),&rec)?;
        #[cfg(test)] super::qualification::pause_at(&self.root,"before_head")?;
        let (snapshot,snapshot_digest)=if rev%SNAP_INTERVAL==0||loaded.head.index.is_none(){
            (rev,Some(index_root.clone()))
        }else{(loaded.head.snapshot,loaded.head.snapshot_digest)};
        let head=Envelope::new(Head{schema:1,revision:rev,record:rec.checksum,snapshot,snapshot_digest,index:Some(index_root)})?;
        write_new(&self.root,&Self::name("head",rev),&head)?;
        #[cfg(test)] super::qualification::pause_at(&self.root,"after_head")?;
        // Retirement is best effort after the commit. Never prune an unarchived
        // committed record. Explicit maintenance reports any failure.
        if rev==snapshot&&self.prune(snapshot).is_ok(){let _=self.collect_index();}
        if self.hot.is_some(){
            loaded.world.mark_history_clean();loaded.head=head.body;loaded.lazy_receipts=true;loaded.receipts.clear();
            let mut r=self.resident.lock().map_err(|_|"resident cache poisoned after commit")?;
            r.state=Some(loaded);
        }
        Ok(response)
    }
    fn archive_root(&self)->PathBuf{self.root.join("journal-archive")}
    fn pin_root(&self)->PathBuf{self.root.join("historical-pins")}
    fn archive_path(&self,rev:u64)->PathBuf{
        self.archive_root().join(format!("{:016x}",rev/1024)).join(Self::name("archive",rev))
    }
    fn read_archive(&self,rev:u64)->Result<Option<ArchivedRevision>>{
        let p=self.archive_path(rev);
        if !p.exists(){return Ok(None);}
        let entry:Envelope<ArchivedRevision>=read(&p)?;entry.check()?;entry.body.check(rev)?;
        Ok(Some(entry.body))
    }
    fn archive_revision(&self,rev:u64)->Result<()>{
        let path=self.archive_path(rev);
        // A previous retirement may have removed one of the active files.
        if let Some(old)=self.read_archive(rev)?{
            if self.path("head",rev).exists(){
                let h:Envelope<Head>=read(&self.path("head",rev))?;h.check()?;
                ensure(h.checksum==old.head.checksum,"archive conflicts with active head")?;
            }
            if self.path("record",rev).exists(){
                let r:Envelope<Record>=read(&self.path("record",rev))?;r.check()?;
                ensure(r.checksum==old.record.checksum,"archive conflicts with active record")?;
            }
            if self.path("snapshot",rev).exists(){
                let s:Envelope<Snapshot>=read(&self.path("snapshot",rev))?;s.check()?;
                ensure(old.snapshot.as_ref().is_some_and(|x|x.checksum==s.checksum),"archive conflicts with active snapshot")?;
            }
            return Ok(());
        }
        let head:Envelope<Head>=read(&self.path("head",rev))?;
        let record:Envelope<Record>=read(&self.path("record",rev))?;
        let snapshot=if self.path("snapshot",rev).exists(){Some(read(&self.path("snapshot",rev))?)}else{None};
        let body=ArchivedRevision{head,record,snapshot};body.check(rev)?;
        let dir=path.parent().ok_or("archive directory missing")?;
        fs::create_dir_all(dir).map_err(error)?;
        sync_directory(&self.root)?;sync_directory(&self.archive_root())?;
        let entry=Envelope::new(body)?;
        write_new(dir,&Self::name("archive",rev),&entry)?;
        let verified=self.read_archive(rev)?.ok_or("archive disappeared")?;
        ensure(verified.head.checksum==entry.body.head.checksum&&verified.record.checksum==entry.body.record.checksum,"archive verification failed")
    }
    fn retained_pins(&self)->Result<Vec<HistoricalPin>>{
        let dir=self.pin_root();if !dir.exists(){return Ok(vec![]);}
        let mut pins=vec![];
        for entry in fs::read_dir(dir).map_err(error)?{
            let entry=entry.map_err(error)?;
            let name=entry.file_name().to_string_lossy().into_owned();
            if name.starts_with(".tmp-"){continue;}
            let key=name.strip_suffix(".json").ok_or("invalid historical pin filename")?;id(key)?;
            ensure(entry.file_type().map_err(error)?.is_file(),"invalid historical pin file")?;
            let p:Envelope<HistoricalPin>=read(&entry.path())?;p.check()?;
            ensure(p.body.name==key&&p.body.revision==p.body.anchor.revision&&hash_id(&p.body.index)&&hash_id(&p.body.anchor.digest),"historical pin identity mismatch")?;
            pins.push(p.body);
        }
        pins.sort_by(|a,b|a.name.cmp(&b.name));Ok(pins)
    }
    /// Pins retain actual indexed state, not just the retired command.
    /// Caller must export the returned anchor independently for tamper evidence.
    pub fn pin_history(&self,name:&str)->Result<HistoricalPin>{
        id(name)?;let _guard=self.lock()?;
        let s=self.load_uncached()?;
        let index=s.head.index.clone().ok_or("historical pin requires indexed state")?;
        let p=HistoricalPin{name:name.into(),revision:s.head.revision,anchor:Anchor{revision:s.head.revision,digest:checksum(&s.head)?},index};
        let dir=self.pin_root();fs::create_dir_all(&dir).map_err(error)?;sync_directory(&self.root)?;
        write_new(&dir,&format!("{name}.json"),&Envelope::new(p.clone())?)?;Ok(p)
    }
    pub fn history_pins(&self)->Result<Vec<HistoricalPin>>{let _guard=self.lock()?;self.retained_pins()}
    pub fn historical_lookup(&self,name:&str,table:&str,key:&str)->Result<Option<Value>>{
        id(name)?;let _guard=self.lock()?;
        let p=self.retained_pins()?.into_iter().find(|p|p.name==name).ok_or("historical pin missing")?;
        // Match the head to the independently exportable anchor. A pin without
        // its original head or archived head is not a valid historical view.
        let h=if self.path("head",p.revision).exists(){
            let h:Envelope<Head>=read(&self.path("head",p.revision))?;h.check()?;h.body
        }else{self.read_archive(p.revision)?.ok_or("historical head missing")?.head.body};
        ensure(checksum(&h)?==p.anchor.digest&&h.index.as_ref()==Some(&p.index),"historical pin head mismatch")?;
        super::index::Index::open(&self.root)?.lookup(&p.index,p.revision,table,key)
    }
    pub fn historical_record(&self,rev:u64)->Result<Value>{
        ensure(rev>0,"invalid historical revision")?;
        let _guard=self.lock()?;
        let latest=self.latest_head()?.ok_or("store has no committed revision")?;
        ensure(rev<=latest.revision,"revision not committed")?;
        let (h,r,source)=if self.path("head",rev).exists()&&self.path("record",rev).exists(){
            let h:Envelope<Head>=read(&self.path("head",rev))?;
            let r:Envelope<Record>=read(&self.path("record",rev))?;
            let e=ArchivedRevision{head:h,record:r,snapshot:None};e.check(rev)?;(e.head,e.record,"active")
        }else if let Some(e)=self.read_archive(rev)?{(e.head,e.record,"archive")}
        else{return Ok(json!({"revision":rev,"available":false,"reason":"historical_record_unavailable_in_existing_store"}));};
        Ok(json!({"revision":rev,"available":true,"source":source,"anchor":Anchor{revision:rev,digest:h.checksum.clone()},
            "head":h,"record":r,"send_authority":"none_historical_record_only","integrity":"individual_record_and_head_not_full_chain"}))
    }
    /// Verify a single historical revision, comparing all surviving active
    /// components to its archived counterpart. Missing older records are
    /// distinguishable from malformed files but cannot be presumed legitimate.
    fn audit_record(&self,rev:u64)->Result<Option<ArchivedRevision>>{
        let archived=self.read_archive(rev)?;
        let has_head=self.path("head",rev).exists();let has_record=self.path("record",rev).exists();
        if let Some(a)=archived{
            if has_head{
                let h:Envelope<Head>=read(&self.path("head",rev))?;h.check()?;
                ensure(h.checksum==a.head.checksum,"active/archive head disagreement")?;
            }
            if has_record{
                let r:Envelope<Record>=read(&self.path("record",rev))?;r.check()?;
                ensure(r.checksum==a.record.checksum,"active/archive command disagreement")?;
            }
            return Ok(Some(a));
        }
        ensure(has_head==has_record,"partial active revision lacks archive")?;
        if !has_head{return Ok(None);}
        let h=read(&self.path("head",rev))?;let r=read(&self.path("record",rev))?;
        let snapshot=if self.path("snapshot",rev).exists(){Some(read(&self.path("snapshot",rev))?)}else{None};
        let a=ArchivedRevision{head:h,record:r,snapshot};a.check(rev)?;Ok(Some(a))
    }
    /// Audits bounded windows; never silently bridges a missing revision.
    /// A previous page's anchor may be supplied to fence a multi-page audit.
    pub fn audit_history(&self,start:u64,limit:usize,expected:Option<&Anchor>)->Result<Value>{
        ensure(start>0&&(1..=256).contains(&limit),"invalid history audit window")?;
        let _guard=self.lock()?;
        let h=self.latest_head()?.ok_or("store has no committed history")?;
        let anchor=Anchor{revision:h.revision,digest:checksum(&h)?};
        if let Some(e)=expected{ensure(e.revision==anchor.revision&&e.digest==anchor.digest,"history audit head changed")?;}
        ensure(start<=h.revision,"history audit start exceeds current head")?;
        let end=start.saturating_add(limit as u64-1).min(h.revision);
        let index=super::index::Index::open(&self.root)?;
        let root=h.index.as_ref();
        let mut previous=if start==1{Some("genesis".to_string())}
            else{self.audit_record(start-1)?.map(|r|r.record.checksum)};
        let mut records=vec![];let mut available=0usize;let mut missing=0usize;
        let mut link_verified=0usize;let mut unknown_links=0usize;let mut receipts_verified=0usize;
        for rev in start..=end{
            let Some(a)=self.audit_record(rev)? else{
                ensure(rev<h.snapshot,"missing retained committed revision")?;
                previous=None;missing+=1;
                records.push(json!({"revision":rev,"status":"missing_history","link":"unknown","receipt":"not_compared"}));continue;
            };
            let link=if let Some(p)=&previous{
                ensure(a.record.body.previous==*p,"historical record chain mismatch")?;
                link_verified+=1;"verified"
            }else{unknown_links+=1;"unknown_predecessor"};
            let receipt=if let Some(root)=root{
                let v=index.lookup(root,h.revision,"receipts",a.record.body.command.id())?.ok_or("historical receipt missing from current index")?;
                let r:Receipt=serde_json::from_value(v).map_err(error)?;
                ensure(r.revision==rev&&r.request_digest==checksum(&a.record.body.command)?&&r.response==a.record.body.response,"historical receipt mismatch")?;
                receipts_verified+=1;"verified_current_index"
            }else{"legacy_not_compared"};
            if rev==h.revision{ensure(a.head.checksum==anchor.digest,"history tail anchor mismatch")?;}
            available+=1;previous=Some(a.record.checksum.clone());
            records.push(json!({"revision":rev,"status":"available","link":link,"receipt":receipt,"record_digest":a.record.checksum,"head_digest":a.head.checksum}));
        }
        Ok(json!({"anchor":anchor,"start":start,"end":end,"records":records,"available":available,"missing":missing,
            "links_verified":link_verified,"links_unknown":unknown_links,"receipts_verified":receipts_verified,
            "window_complete":missing==0&&unknown_links==0&&receipts_verified==available,
            "next_start":if end<h.revision{Some(end+1)}else{None},
            "scope":"bounded_window_not_global_or_external_custody_proof","external_anchor_supplied":expected.is_some()}))
    }
    /// Non-destructive capacity feedback. Truncated inventories report lower
    /// bounds; over-limit is actionable, within-limit requires a complete scan.
    pub fn retention_report(&self,policy:RetentionPolicy)->Result<Value>{
        ensure((1..=1_000_000).contains(&policy.entry_budget_per_area),"invalid retention inventory budget")?;
        let _guard=self.lock()?;
        let archive=Inventory::scan(&self.archive_root(),policy.entry_budget_per_area)?;
        let index=Inventory::scan(&self.root.join("entity-index").join("objects"),policy.entry_budget_per_area)?;
        let pins=Inventory::scan(&self.pin_root(),policy.entry_budget_per_area)?;
        Ok(json!({"policy":policy,"archive":{"inventory":archive,"status":archive.status(archive.bytes,policy.max_archive_bytes)},
            "index":{"inventory":index,"status":index.status(index.bytes,policy.max_index_bytes)},
            "pins":{"inventory":pins,"status":pins.status(pins.files,policy.max_pin_files)},
            "scope":"local_regular_file_bytes_not_allocated_disk_bytes","deletion_authorized":false,
            "recommendation":"review_capacity_or_export_evidence_without_destroying_pins"}))
    }
    /// Point receipt inspection is deliberately not an execution/replay path.
    /// One-shot response bodies are never returned through this interface.
    pub fn receipt_status(&self,key:&str)->Result<Value>{
        id(key)?;let _guard=self.lock()?;
        let Some(head)=self.latest_head()? else{return Ok(json!({"request":key,"found":false}));};
        let receipt=if let Some(root)=&head.index{
            let last:Envelope<Record>=read(&self.path("record",head.revision))?;last.check()?;
            ensure(last.body.revision==head.revision&&last.body.index==head.index&&last.checksum==head.record,"receipt status head mismatch")?;
            super::index::Index::open(&self.root)?.lookup(root,head.revision,"receipts",key)?
                .map(|v|serde_json::from_value::<Receipt>(v).map_err(error)).transpose()?
        }else{self.load()?.receipts.get(key).cloned()};
        if let Some(r)=receipt{
            ensure(r.revision<=head.revision&&hash_id(&r.request_digest),"invalid receipt identity")?;
            Ok(json!({"request":key,"found":true,"revision":r.revision,"request_digest":r.request_digest,"response_digest":digest(&r.response),
                "send_authority":"none","scope":"point_receipt_observation_only"}))
        }else{Ok(json!({"request":key,"found":false,"observed_revision":head.revision,"send_authority":"none"}))}
    }
    fn collect_index(&self)->Result<usize>{
        let mut roots=BTreeSet::new();
        for rev in self.heads()?{let h:Envelope<Head>=read(&self.path("head",rev))?;h.check()?;if let Some(index)=h.body.index{roots.insert(index);}}
        for p in self.retained_pins()?{roots.insert(p.index);}
        super::index::Index::open(&self.root)?.collect(&roots)
    }
    /// Explicit complete current-index audit. Normal transactions only read
    /// historical receipt pages required for the retained journal and replay.
    pub fn audit_index(&self)->Result<Value>{
        let _guard=self.lock()?;let loaded=self.load_uncached()?;
        let root=loaded.head.index.as_ref().ok_or("audit requires indexed state")?;
        let index=super::index::Index::open(&self.root)?;
        let (world,receipts)=index.load(root,loaded.head.revision)?;
        ensure(encode(&world)?==encode(&loaded.world)?,"audit world mismatch")?;
        ensure(receipts.len() as u64==loaded.head.revision,"receipt count differs from committed revision")?;
        let mut seen=BTreeSet::new();
        for (id,r) in &receipts{
            super::model::id(id)?;
            ensure(r.revision>0&&r.revision<=loaded.head.revision&&hash_id(&r.request_digest)&&seen.insert(r.revision),"invalid receipt coverage")?;
        }
        Ok(json!({"revision":loaded.head.revision,"schema":index.manifest(root)?.schema,"receipts":receipts.len(),
            "root":root,"scope":"current_index_objects_and_receipt_coverage_not_full_archived_chain"}))
    }
    pub fn collect_garbage(&self)->Result<usize>{
        let _guard=self.lock()?;let loaded=self.load_uncached()?;
        if loaded.head.snapshot>0{self.prune(loaded.head.snapshot)?;}
        self.collect_index()
    }
    fn prune(&self,snapshot:u64)->Result<()>{
        // Group by revision, including remnants from a partially completed
        // retirement. Never infer that an uncommitted preparation was committed.
        let mut revisions=BTreeSet::new();
        for e in fs::read_dir(&self.root).map_err(error)?{
            let e=e.map_err(error)?;let name=e.file_name().to_string_lossy().into_owned();
            let Some((n,kind))=name.split_once('.') else{continue};
            let Ok(rev)=n.parse::<u64>() else{continue};
            if rev<snapshot&&matches!(kind,"head.json"|"record.json"|"snapshot.json"){revisions.insert(rev);}
        }
        for rev in revisions{
            // Preserve a verifiable head/record pair before removing any part.
            // Orphan remnants without archive/head are retained, not deleted.
            if !self.path("head",rev).exists()&&self.read_archive(rev)?.is_none(){continue;}
            self.archive_revision(rev)?;
            for kind in ["record","snapshot","head"]{
                let p=self.path(kind,rev);if p.exists(){fs::remove_file(p).map_err(error)?;}
            }
        }
        sync_directory(&self.root)
    }
    pub fn project_host_agent(&self,agent:&str)->Result<PathBuf>{
        let _guard=self.lock()?;let w=self.load()?.world;
        super::projection::project(&w,agent,&self.root.join("projections"))
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
