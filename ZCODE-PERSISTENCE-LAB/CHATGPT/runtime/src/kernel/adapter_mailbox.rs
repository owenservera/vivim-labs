//! Durable host-to-external-adapter transport. No provider calls occur here.
//! The caller must be a trusted local host bridge, not an untrusted worker.
//! A claim is single-attempt authority, never permission to retry on uncertainty.
use super::{model::{ensure,id,hash_id,Id,MAX_VALUE_BYTES},persistence::KernelStore,routing_adapter::{AdapterReceipt,Envelope,HandoffPhase,Transport},routing_driver::{Backend,advance_clock},routing::{Outcome,Status},transition::{Actor,Action,Request}};
use crate::{digest,Result};
use serde::{Deserialize,Serialize};
use serde_json::{json,Value};
use std::{fs::{self,File,OpenOptions},io::Write,path::{Path,PathBuf},time::{SystemTime,UNIX_EPOCH}};

fn err(e:impl std::fmt::Display)->String{e.to_string()}
fn val<T:Serialize>(v:&T)->Result<Value>{serde_json::to_value(v).map_err(err)}
fn pin<T:Serialize>(v:&T)->Result<String>{Ok(digest(&val(v)?))}
fn read<T:for<'de>Deserialize<'de>>(p:&Path,limit:u64)->Result<T>{
    let m=fs::symlink_metadata(p).map_err(err)?;
    ensure(m.file_type().is_file()&&m.len()<=limit,"invalid or oversized mailbox file")?;
    serde_json::from_slice(&fs::read(p).map_err(err)?).map_err(err)
}
fn write<T:Serialize>(root:&Path,name:&str,v:&T)->Result<()>{
    let p=root.join(name);ensure(!p.exists(),"mailbox record already exists")?;
    let n=SystemTime::now().duration_since(UNIX_EPOCH).map_err(err)?.as_nanos();
    let tmp=root.join(format!(".tmp-{}-{n}",std::process::id()));
    let mut f=OpenOptions::new().create_new(true).write(true).open(&tmp).map_err(err)?;
    f.write_all(&serde_json::to_vec(v).map_err(err)?).map_err(err)?;f.sync_all().map_err(err)?;drop(f);
    fs::rename(tmp,p).map_err(err)?;
    #[cfg(unix)] File::open(root).and_then(|f|f.sync_all()).map_err(err)?;
    Ok(())
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config{
    pub adapter:Id,pub provider:Id,pub account:Id,
    pub max_records:usize,pub max_payload_bytes:usize,
}
impl Config{
    fn validate(&self)->Result<()>{
        id(&self.adapter)?;id(&self.provider)?;id(&self.account)?;
        ensure((1..=65536).contains(&self.max_records)&&(1..=MAX_VALUE_BYTES).contains(&self.max_payload_bytes),"invalid mailbox budgets")
    }
}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Manifest{schema:u32,store:PathBuf,config:Config}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Packet{manifest:String,envelope:Envelope,input:Value}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Entry<T>{digest:String,body:T}
impl<T:Serialize> Entry<T>{
    fn new(body:T)->Result<Self>{Ok(Self{digest:pin(&body)?,body})}
    fn checked(self)->Result<T>{ensure(pin(&self.body)?==self.digest,"mailbox record digest mismatch")?;Ok(self.body)}
}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Taken{manifest:String,packet:String,handoff:Id,at_ms:u64}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Response{manifest:String,packet:String,receipt:AdapterReceipt}
pub struct Mailbox{root:PathBuf,manifest:Manifest,manifest_pin:String}
impl Mailbox{
    pub fn create(root:&Path,store:&KernelStore,config:Config)->Result<Self>{
        config.validate()?;
        let s=store.inspect()?;
        ensure(s.router.adapters.bindings.contains_key(&config.adapter),"mailbox adapter not registered")?;
        let profiles=&s.router.config.as_ref().ok_or("router not configured")?.profiles;
        ensure(profiles.values().any(|p|p.adapter==config.adapter&&p.provider==config.provider&&p.account==config.account),"mailbox profile scope missing")?;
        fs::create_dir(root).map_err(err)?;
        let root=root.canonicalize().map_err(err)?;let store=store.root().canonicalize().map_err(err)?;
        ensure(!root.starts_with(&store)&&!store.starts_with(&root),"mailbox and kernel store must be separate")?;
        for name in ["outbox","taken","receipts"]{fs::create_dir(root.join(name)).map_err(err)?;}
        let manifest=Manifest{schema:1,store,config};
        let manifest_pin=pin(&manifest)?;
        write(&root,"manifest.json",&Entry::new(manifest.clone())?)?;
        Ok(Self{root,manifest,manifest_pin})
    }
    pub fn open(root:&Path,store:&KernelStore)->Result<Self>{
        let root=root.canonicalize().map_err(err)?;
        let entry:Entry<Manifest>=read(&root.join("manifest.json"),64*1024)?;
        let manifest=entry.checked()?;
        manifest.config.validate()?;
        ensure(manifest.schema==1&&manifest.store==store.root().canonicalize().map_err(err)?,"mailbox belongs to a different store")?;
        for name in ["outbox","taken","receipts"]{
            ensure(fs::symlink_metadata(root.join(name)).map_err(err)?.file_type().is_dir(),"invalid mailbox directory")?;
        }
        let manifest_pin=pin(&manifest)?;
        Ok(Self{root,manifest,manifest_pin})
    }
    pub fn adapter(&self)->&str{&self.manifest.config.adapter}
    fn lock(&self)->Result<File>{
        let f=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(self.root.join("mailbox.lock")).map_err(err)?;
        f.lock().map_err(err)?;Ok(f)
    }
    fn name(key:&str)->Result<String>{id(key)?;Ok(format!("{key}.json"))}
    fn packet(&self,key:&str)->Result<Packet>{
        let entry:Entry<Packet>=read(&self.root.join("outbox").join(Self::name(key)?),2*MAX_VALUE_BYTES as u64+65536)?;
        let p=entry.checked()?;
        ensure(p.manifest==self.manifest_pin&&p.envelope.id==key&&self.supports(&p.envelope)
            &&digest(&p.input)==p.envelope.input_digest,"mailbox packet identity mismatch")?;
        ensure(serde_json::to_vec(&p.input).map_err(err)?.len()<=self.manifest.config.max_payload_bytes,"mailbox input exceeds budget")?;
        Ok(p)
    }
    fn taken(&self,key:&str,p:&Packet)->Result<Option<Taken>>{
        let path=self.root.join("taken").join(Self::name(key)?);
        if !path.exists(){return Ok(None);}
        let t:Taken=read::<Entry<Taken>>(&path,65536)?.checked()?;
        ensure(t.manifest==self.manifest_pin&&t.handoff==key&&t.packet==pin(p)?,"mailbox take identity mismatch")?;
        Ok(Some(t))
    }
    fn response(&self,key:&str,p:&Packet)->Result<Option<AdapterReceipt>>{
        let path=self.root.join("receipts").join(Self::name(key)?);
        if !path.exists(){return Ok(None);}
        let r:Response=read::<Entry<Response>>(&path,2*MAX_VALUE_BYTES as u64+65536)?.checked()?;
        ensure(r.manifest==self.manifest_pin&&r.packet==pin(p)?&&self.taken(key,p)?.is_some(),"mailbox receipt lacks bound claim")?;
        self.validate_receipt(p,&r.receipt)?;
        Ok(Some(r.receipt))
    }
    fn validate_receipt(&self,p:&Packet,r:&AdapterReceipt)->Result<()>{
        id(&r.request_id)?;
        ensure(r.handoff==p.envelope.id&&r.provider==p.envelope.provider&&r.account==p.envelope.account
            &&r.model==p.envelope.model&&r.thinking==p.envelope.thinking,"receipt identity mismatch; retain observed charge evidence for host reconciliation")?;
        ensure(!r.provider_receipt.trim().is_empty()&&r.provider_receipt.len()<=4096,"actual provider receipt reference required")?;
        ensure((r.outcome==Outcome::Completed)==r.output.is_some(),"receipt outcome/output mismatch")?;
        if let Some(v)=&r.output{ensure(serde_json::to_vec(v).map_err(err)?.len()<=self.manifest.config.max_payload_bytes,"output exceeds mailbox budget")?;}
        // Do not suppress actual usage because it exceeds a reservation ceiling.
        // Kernel settlement detects and charges overruns rather than hiding cost.
        Ok(())
    }
    fn keys(&self)->Result<Vec<Id>>{
        let mut keys=vec![];
        for item in fs::read_dir(self.root.join("outbox")).map_err(err)?{
            let item=item.map_err(err)?;let name=item.file_name().to_string_lossy().into_owned();
            if name.starts_with(".tmp-"){continue;}
            let key=name.strip_suffix(".json").ok_or("unexpected mailbox entry")?;
            id(key)?;ensure(item.file_type().map_err(err)?.is_file(),"mailbox outbox is not a regular file")?;
            keys.push(key.to_owned());ensure(keys.len()<=self.manifest.config.max_records,"mailbox record budget exceeded")?;
        }
        keys.sort();Ok(keys)
    }
    fn publish(&self,e:&Envelope,input:&Value)->Result<()>{
        let _lock=self.lock()?;
        ensure(self.supports(e)&&e.phase==HandoffPhase::Claimed&&e.claimed_ms.is_some(),"mailbox requires committed claim")?;
        ensure(hash_id(&e.input_digest)&&digest(input)==e.input_digest,"mailbox input digest mismatch")?;
        ensure(serde_json::to_vec(input).map_err(err)?.len()<=self.manifest.config.max_payload_bytes,"mailbox input exceeds budget")?;
        ensure(self.keys()?.len()<self.manifest.config.max_records,"mailbox full; operator retention review required")?;
        let p=Packet{manifest:self.manifest_pin.clone(),envelope:e.clone(),input:input.clone()};
        // A repeated publish is a programming error. Reconciliation only reads.
        write(&self.root.join("outbox"),&Self::name(&e.id)?,&Entry::new(p)?)
    }
    pub fn inspect(&self)->Result<Value>{
        let _lock=self.lock()?;let mut entries=vec![];
        for key in self.keys()?{
            let p=self.packet(&key)?;
            let taken=self.taken(&key,&p)?.is_some();let received=self.response(&key,&p)?.is_some();
            entries.push(json!({"handoff":key,"profile":p.envelope.profile,"deadline_ms":p.envelope.deadline_ms,
                "state":if received{"receipt_available"}else if taken{"taken_uncertain"}else{"awaiting_take"}}));
        }
        Ok(json!({"adapter":self.adapter(),"manifest_digest":self.manifest_pin,"entries":entries,"authority":"inspection_only"}))
    }
    /// Trusted bridge calls once immediately before transmission. Replays never
    /// return input or send authority, even if the bridge crashed before send.
    pub fn take(&self,store:&KernelStore,key:&str)->Result<Value>{
        ensure(self.manifest.store==store.root().canonicalize().map_err(err)?,"mailbox store mismatch")?;
        let _lock=self.lock()?;let p=self.packet(key)?;
        if self.taken(key,&p)?.is_some(){return Ok(json!({"handoff":key,"send_authority":"none_reconcile_only"}));}
        advance_clock(store)?;
        let w=store.inspect()?;
        let e=w.router.adapters.handoffs.get(key).ok_or("kernel handoff missing")?;
        let r=w.router.reservations.get(key).ok_or("kernel reservation missing")?;
        let a=&p.envelope;
        // Match stable fields, not the mutable claimed/unknown status.
        ensure(e.id==a.id&&e.adapter==a.adapter&&e.binding_version==a.binding_version&&e.profile==a.profile
            &&e.provider==a.provider&&e.account==a.account&&e.model==a.model&&e.thinking==a.thinking
            &&e.task==a.task&&e.generation==a.generation&&e.worker==a.worker&&e.incarnation==a.incarnation
            &&e.input_digest==a.input_digest&&e.deadline_ms==a.deadline_ms&&pin(&e.ceiling)?==pin(&a.ceiling)?,"kernel handoff changed")?;
        ensure(matches!(e.phase,HandoffPhase::Claimed|HandoffPhase::Unknown)&&matches!(r.status,Status::Dispatched|Status::Unknown)
            &&w.clock_ms<e.deadline_ms,"handoff no longer available for transmission")?;
        ensure(w.router.adapters.bindings.get(&e.adapter).is_some_and(|b|b.enabled&&b.version==e.binding_version),"adapter disabled or replaced")?;
        let t=w.tasks.get(&e.task).ok_or("mailbox task missing")?;
        ensure(t.phase==super::model::TaskPhase::Running&&t.lease.as_ref().is_some_and(|l|l.generation==e.generation&&l.incarnation==e.incarnation&&l.worker==e.worker)
            &&w.goals.get(&t.spec.goal).is_some_and(|g|g.phase==super::model::GoalPhase::Open),"task authorization expired")?;
        let packet_digest=pin(&p)?;
        // The kernel, not a mailbox directory, is the authoritative one-shot
        // fence. A crash between this commit and the local marker consumes
        // authorization and cannot silently grant a second transmission.
        let authorization=store.apply(Request{id:format!("mailbox-take-{}",digest(&json!(key))),actor:Actor::Host,
            action:Action::AuthorizeMailboxTake{handoff:key.into(),manifest_digest:self.manifest_pin.clone(),packet_digest:packet_digest.clone(),
                binding_version:a.binding_version,input_digest:a.input_digest.clone()}})?;
        if authorization["send_authority"]!="single_attempt_only"{
            return Ok(json!({"handoff":key,"send_authority":"none_reconcile_only","reason":"kernel_authorization_already_consumed"}));
        }
        let at_ms=authorization["take"]["authorized_ms"].as_u64().ok_or("missing committed mailbox timestamp")?;
        let expires_ms=authorization["take"]["deadline_ms"].as_u64().ok_or("missing committed mailbox deadline")?;
        let taken=Taken{manifest:self.manifest_pin.clone(),packet:packet_digest,handoff:key.into(),at_ms};
        write(&self.root.join("taken"),&Self::name(key)?,&Entry::new(taken)?)?;
        Ok(json!({"handoff":p.envelope,"input":p.input,"send_authority":"single_attempt_only","expires_ms":expires_ms}))
    }
    pub fn submit(&self,r:AdapterReceipt)->Result<Value>{
        let _lock=self.lock()?;let p=self.packet(&r.handoff)?;
        ensure(self.taken(&r.handoff,&p)?.is_some(),"receipt without committed mailbox take")?;
        self.validate_receipt(&p,&r)?;
        if let Some(old)=self.response(&r.handoff,&p)?{
            ensure(pin(&old)?==pin(&r)?,"conflicting mailbox receipt")?;
            return Ok(json!({"handoff":r.handoff,"stored":true,"replayed":true}));
        }
        let key=r.handoff.clone();let response=Response{manifest:self.manifest_pin.clone(),packet:pin(&p)?,receipt:r};
        write(&self.root.join("receipts"),&Self::name(&key)?,&Entry::new(response)?)?;
        Ok(json!({"handoff":key,"stored":true,"replayed":false,"settlement":"pending_kernel_reconciliation"}))
    }
}
impl Backend for Mailbox{
    fn transport(&self)->Transport{Transport::Mailbox{manifest_digest:self.manifest_pin.clone()}}
    fn finish_action(&self,r:AdapterReceipt)->Result<Action>{
        let _lock=self.lock()?;let p=self.packet(&r.handoff)?;
        let stored=self.response(&r.handoff,&p)?.ok_or("mailbox receipt missing")?;
        ensure(pin(&stored)?==pin(&r)?,"mailbox receipt differs from stored response")?;
        Ok(Action::FinishMailboxHandoff{receipt:r,manifest_digest:self.manifest_pin.clone(),packet_digest:pin(&p)?})
    }
    fn supports(&self,e:&Envelope)->bool{
        e.adapter==self.manifest.config.adapter&&e.provider==self.manifest.config.provider&&e.account==self.manifest.config.account
    }
    fn send(&mut self,_:&Envelope,_:&Value)->Result<AdapterReceipt>{Err("mailbox is asynchronous; use begin".into())}
    fn begin(&mut self,e:&Envelope,input:&Value)->Result<Option<AdapterReceipt>>{self.publish(e,input)?;Ok(None)}
    fn reconcile(&mut self,e:&Envelope)->Result<Option<AdapterReceipt>>{
        let _lock=self.lock()?;
        if !self.root.join("outbox").join(Self::name(&e.id)?).exists(){return Ok(None);}
        let p=self.packet(&e.id)?;
        ensure(p.envelope.input_digest==e.input_digest&&p.envelope.generation==e.generation&&p.envelope.profile==e.profile,"reconciliation identity mismatch")?;
        self.response(&e.id,&p)
    }
}
