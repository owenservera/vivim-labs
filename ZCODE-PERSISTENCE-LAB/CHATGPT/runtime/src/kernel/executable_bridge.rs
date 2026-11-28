//! Trusted local executable bridge. A provisioned adapter implements the
//! request/receipt file protocol. This module is not a provider SDK and does
//! not assume a particular CLI implements that protocol.
use super::{adapter_mailbox::Mailbox,model::{ensure,id,hash_id,MAX_VALUE_BYTES},persistence::KernelStore,
    routing_adapter::{AdapterReceipt,HandoffPhase},routing_driver::{Backend,advance_clock,step},transition::{Actor,Request}};
use crate::{digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs::{self,File,OpenOptions},io::{Read,Write,Seek,SeekFrom},path::{Path,PathBuf},
    process::{Child,Command,Stdio},time::{Duration,Instant,SystemTime,UNIX_EPOCH},thread};
fn err(e:impl std::fmt::Display)->String{e.to_string()}
fn now_ms()->Result<u64>{u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH).map_err(err)?.as_millis()).map_err(err)}
fn hash_file(p:&Path)->Result<String>{
    let m=fs::symlink_metadata(p).map_err(err)?;
    ensure(m.file_type().is_file()&&m.len()<=512*1024*1024,"invalid or oversized adapter executable")?;
    let mut file=File::open(p).map_err(err)?;let mut hash=Sha256::new();let mut b=[0u8;65536];
    loop{let n=file.read(&mut b).map_err(err)?;if n==0{break;}hash.update(&b[..n]);}
    Ok(format!("{:x}",hash.finalize()))
}
fn read<T:for<'de>Deserialize<'de>>(p:&Path,limit:usize)->Result<T>{
    let meta=fs::symlink_metadata(p).map_err(err)?;
    ensure(meta.file_type().is_file()&&meta.len()<=limit as u64,"invalid bridge record size or type")?;
    let mut bytes=vec![];File::open(p).map_err(err)?.take(limit as u64+1).read_to_end(&mut bytes).map_err(err)?;
    ensure(bytes.len()<=limit,"bridge record exceeded read budget")?;
    serde_json::from_slice(&bytes).map_err(err)
}
fn write<T:Serialize>(root:&Path,name:&str,v:&T)->Result<()>{
    let p=root.join(name);ensure(!p.exists(),"bridge record already exists")?;
    let tmp=root.join(format!(".tmp-{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).map_err(err)?.as_nanos()));
    let mut file=OpenOptions::new().create_new(true).write(true).open(&tmp).map_err(err)?;
    file.write_all(&serde_json::to_vec(v).map_err(err)?).map_err(err)?;file.sync_all().map_err(err)?;drop(file);
    fs::rename(&tmp,&p).map_err(err)?;
    #[cfg(unix)] File::open(root).and_then(|f|f.sync_all()).map_err(err)?;
    Ok(())
}
fn pin<T:Serialize>(v:&T)->Result<String>{Ok(digest(&serde_json::to_value(v).map_err(err)?))}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config{
    pub executable:PathBuf,pub executable_sha256:String,
    pub args:Vec<String>,pub inherit_environment:Vec<String>,
    pub max_runtime_ms:u64,pub max_receipt_bytes:usize,pub max_jobs:usize,
}
impl Config{
    fn validate(&self)->Result<()>{
        ensure(self.executable.is_absolute()&&hash_id(&self.executable_sha256),"explicit absolute pinned executable required")?;
        ensure(self.args.len()<=64&&self.args.iter().all(|a|a.len()<=4096&&!a.contains('\0')),"invalid adapter argv")?;
        ensure(self.inherit_environment.len()<=64&&self.inherit_environment.iter().all(|k|!k.is_empty()&&k.len()<=128&&k.bytes().all(|b|b.is_ascii_alphanumeric()||b==b'_')),"invalid environment allowlist")?;
        ensure((100..=1_800_000).contains(&self.max_runtime_ms)&&(1024..=2*MAX_VALUE_BYTES+65536).contains(&self.max_receipt_bytes)&&(1..=65536).contains(&self.max_jobs),"invalid bridge budgets")
    }
    fn verify_executable(&self)->Result<()>{
        self.validate()?;
        ensure(self.executable.canonicalize().map_err(err)?==self.executable,"executable must be canonical")?;
        ensure(hash_file(&self.executable)?==self.executable_sha256,"adapter executable pin changed")
    }
}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Manifest{schema:u32,mailbox:String,mailbox_root:PathBuf,config:Config}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Entry<T>{digest:String,body:T}
impl<T:Serialize> Entry<T>{
    fn new(body:T)->Result<Self>{Ok(Self{digest:pin(&body)?,body})}
    fn checked(self)->Result<T>{ensure(pin(&self.body)?==self.digest,"bridge record integrity mismatch")?;Ok(self.body)}
}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct Intent{manifest:String,handoff:String,created_ms:u64}
struct Owned(Child);
impl Drop for Owned{fn drop(&mut self){if self.0.try_wait().ok().flatten().is_none(){let _=self.0.kill();let _=self.0.wait();}}}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CyclePolicy{pub max_publications:usize,pub max_visits:usize,pub max_launch_intents:usize,pub max_cycle_ms:u64}
impl CyclePolicy{
    fn validate(&self)->Result<()>{
        ensure((1..=128).contains(&self.max_publications)&&(1..=128).contains(&self.max_visits)
            &&self.max_launch_intents<=self.max_visits&&(100..=1_800_000).contains(&self.max_cycle_ms),"invalid bridge cycle budget")
    }
}
pub struct Bridge{root:PathBuf,manifest:Manifest,identity:String}
impl Bridge{
    pub fn create(root:&Path,mailbox:&Mailbox,config:Config)->Result<Self>{
        config.verify_executable()?;
        let parent=root.parent().ok_or("bridge parent missing")?.canonicalize().map_err(err)?;
        let root=parent.join(root.file_name().ok_or("bridge name missing")?);
        ensure(!root.starts_with(mailbox.root())&&!mailbox.root().starts_with(&root)
            &&!root.starts_with(mailbox.store_root())&&!mailbox.store_root().starts_with(&root),"bridge, mailbox and kernel store must be separate")?;
        fs::create_dir(&root).map_err(err)?;let root=root.canonicalize().map_err(err)?;
        fs::create_dir(root.join("jobs")).map_err(err)?;
        let manifest=Manifest{schema:1,mailbox:mailbox.identity().into(),mailbox_root:mailbox.root().into(),config};
        let identity=pin(&manifest)?;write(&root,"manifest.json",&Entry::new(manifest.clone())?)?;
        Ok(Self{root,manifest,identity})
    }
    pub fn open(root:&Path,mailbox:&Mailbox)->Result<Self>{
        let root=root.canonicalize().map_err(err)?;
        let manifest:Manifest=read::<Entry<Manifest>>(&root.join("manifest.json"),512*1024)?.checked()?;
        manifest.config.validate()?;
        ensure(manifest.schema==1&&manifest.mailbox==mailbox.identity()&&manifest.mailbox_root==mailbox.root(),"bridge/mailbox identity mismatch")?;
        ensure(!root.starts_with(mailbox.root())&&!mailbox.root().starts_with(&root)
            &&!root.starts_with(mailbox.store_root())&&!mailbox.store_root().starts_with(&root),"bridge directory overlaps protected state")?;
        ensure(fs::symlink_metadata(root.join("jobs")).map_err(err)?.file_type().is_dir(),"invalid jobs directory")?;
        let identity=pin(&manifest)?;Ok(Self{root,manifest,identity})
    }
    fn lock(&self)->Result<File>{
        let f=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(self.root.join("bridge.lock")).map_err(err)?;
        f.try_lock().map_err(|e|format!("bridge already owned: {e:?}"))?;Ok(f)
    }
    fn job(&self,key:&str)->Result<PathBuf>{id(key)?;Ok(self.root.join("jobs").join(key))}
    fn keys(&self)->Result<Vec<String>>{
        let mut keys=vec![];
        for item in fs::read_dir(self.root.join("jobs")).map_err(err)?{
            let item=item.map_err(err)?;let key=item.file_name().to_string_lossy().into_owned();id(&key)?;
            ensure(item.file_type().map_err(err)?.is_dir(),"nonregular bridge job")?;
            keys.push(key);ensure(keys.len()<=self.manifest.config.max_jobs,"bridge job budget exceeded")?;
        }keys.sort();Ok(keys)
    }
    fn intent(&self,key:&str)->Result<Intent>{
        let dir=self.job(key)?;
        ensure(fs::symlink_metadata(&dir).map_err(err)?.file_type().is_dir(),"invalid job directory")?;
        let v:Intent=read::<Entry<Intent>>(&dir.join("intent.json"),65536)?.checked()?;
        ensure(v.manifest==self.identity&&v.handoff==key,"bridge job identity mismatch")?;Ok(v)
    }
    pub fn inspect(&self)->Result<Value>{
        let _lock=self.lock()?;let mut jobs=vec![];
        for key in self.keys()?{
            let dir=self.job(&key)?;
            let intent_valid=if dir.join("intent.json").exists(){self.intent(&key)?;true}else{false};
            jobs.push(json!({"handoff":key,"intent_present":intent_valid,"request_present":dir.join("request.json").is_file(),
                "receipt_present":dir.join("receipt.json").is_file(),"exit_observation_present":dir.join("exit.json").is_file()}));
        }
        Ok(json!({"manifest_digest":self.identity,"adapter_executable_sha256":self.manifest.config.executable_sha256,
            "jobs":jobs,"authority":"inspection_only"}))
    }
    fn reconcile_locked(&self,store:&KernelStore,mailbox:&Mailbox,key:&str)->Result<Value>{
        let dir=self.job(key)?;
        if !dir.exists(){return Ok(json!({"handoff":key,"state":"no_job","send_authority":"none"}));}
        if !dir.join("intent.json").exists(){return Ok(json!({"handoff":key,"state":"incomplete_intent","send_authority":"none"}));}
        self.intent(key)?;
        if !dir.join("receipt.json").exists(){return Ok(json!({"handoff":key,"state":"outcome_unknown","send_authority":"none_reconcile_only"}));}
        let r:AdapterReceipt=read(&dir.join("receipt.json"),self.manifest.config.max_receipt_bytes)?;
        ensure(r.handoff==key,"adapter returned wrong handoff")?;
        // Submission validates the recorded packet and one-shot take. A
        // recovered receipt may arrive after the original lease expired.
        mailbox.submit(r.clone())?;
        advance_clock(store)?;
        let action=mailbox.finish_action(r)?;
        let settled=store.apply(Request{id:format!("finish-{}",digest(&json!(key))),actor:Actor::Host,action})?;
        Ok(json!({"handoff":key,"state":"settled","settlement":settled,"send_authority":"none"}))
    }
    pub fn reconcile(&self,store:&KernelStore,mailbox:&Mailbox,key:&str)->Result<Value>{
        let _lock=self.lock()?;self.reconcile_locked(store,mailbox,key)
    }
    /// Explicit host operation. Fair rotation is only a visit preference;
    /// losing or corrupting the cursor cannot renew send authorization.
    pub fn cycle(&self,store:&KernelStore,mailbox:&mut Mailbox,policy:CyclePolicy)->Result<Value>{
        policy.validate()?;
        let start=Instant::now();
        let mut cursor=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(self.root.join("cycle.lock")).map_err(err)?;
        cursor.try_lock().map_err(|e|format!("bridge cycle already owned: {e:?}"))?;
        let adapter=mailbox.adapter().to_owned();
        let publication=step(store,&adapter,mailbox,policy.max_publications)?;
        let inspection=mailbox.inspect()?;
        let w=store.inspect()?;
        let mut entries:Vec<_>=inspection["entries"].as_array().ok_or("mailbox entry list missing")?.iter()
            .filter_map(|e|Some((e["handoff"].as_str()?.to_owned(),e["state"].as_str()?.to_owned())))
            .filter(|(key,_)|w.router.adapters.handoffs.get(key).is_some_and(|e|matches!(e.phase,HandoffPhase::Claimed|HandoffPhase::Unknown)))
            .collect();
        entries.sort_by(|a,b|a.0.cmp(&b.0));drop(w);
        let mut last=String::new();
        if cursor.metadata().map_err(err)?.len()<=100{
            cursor.seek(SeekFrom::Start(0)).map_err(err)?;
            if (&mut cursor).take(100).read_to_string(&mut last).is_err(){last.clear();}
        }
        if !entries.is_empty(){let n=entries.partition_point(|(k,_)|k.as_str()<=last.as_str())%entries.len();entries.rotate_left(n);}
        let mut visits=0usize;let mut launch_intents=0usize;let mut results=vec![];
        for (key,state) in entries.into_iter().take(policy.max_visits){
            let elapsed=u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
            if elapsed>=policy.max_cycle_ms{break;}
            visits+=1;
            cursor.seek(SeekFrom::Start(0)).map_err(err)?;cursor.set_len(0).map_err(err)?;cursor.write_all(key.as_bytes()).map_err(err)?;cursor.sync_data().map_err(err)?;
            let result=if self.job(&key)?.exists(){
                self.reconcile(store,mailbox,&key)
            }else if state=="awaiting_take"&&launch_intents<policy.max_launch_intents{
                // Count intentions conservatively, even when a racing process
                // already took the packet or authorization expired.
                launch_intents+=1;
                self.run_bounded(store,mailbox,&key,policy.max_cycle_ms.saturating_sub(elapsed))
            }else{continue;};
            match result{Ok(v)=>results.push(v),Err(reason)=>results.push(json!({"handoff":key,"state":"unresolved","reason":reason,"send_authority":"none"}))}
        }
        Ok(json!({"publication_and_reconciliation":publication,"visited":visits,"launch_intents":launch_intents,
            "results":results,"elapsed_ms":u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX),
            "budget":policy,"scope":"explicit_host_cycle_existing_prepared_handoffs_only"}))
    }
    pub fn run_once(&self,store:&KernelStore,mailbox:&Mailbox,key:&str)->Result<Value>{
        self.run_bounded(store,mailbox,key,self.manifest.config.max_runtime_ms)
    }
    fn run_bounded(&self,store:&KernelStore,mailbox:&Mailbox,key:&str,max_runtime_ms:u64)->Result<Value>{
        let _lock=self.lock()?;let dir=self.job(key)?;
        if dir.exists(){return self.reconcile_locked(store,mailbox,key);}
        self.manifest.config.verify_executable()?;
        ensure(self.keys()?.len()<self.manifest.config.max_jobs,"bridge job budget reached")?;
        fs::create_dir(&dir).map_err(err)?;
        write(&dir,"intent.json",&Entry::new(Intent{manifest:self.identity.clone(),handoff:key.into(),created_ms:now_ms()?})?)?;
        // Once an intent exists, even a crash before take is reconciliation-only.
        let take=mailbox.take(store,key)?;
        if take["send_authority"]!="single_attempt_only"{
            return Ok(json!({"handoff":key,"state":"authorization_unavailable","send_authority":"none"}));
        }
        let deadline=take["expires_ms"].as_u64().ok_or("missing mailbox deadline")?;
        let request=json!({"schema":1,"bridge_manifest":self.identity,"handoff":take["handoff"],"input":take["input"],"expires_ms":deadline});
        write(&dir,"request.json",&request)?;
        let budget=self.manifest.config.max_runtime_ms.min(max_runtime_ms).min(deadline.saturating_sub(now_ms()?));
        if budget==0{return Ok(json!({"handoff":key,"state":"expired_before_launch","send_authority":"none"}));}
        let c=&self.manifest.config;
        let mut command=Command::new(&c.executable);
        command.args(&c.args).arg(dir.join("request.json")).arg(dir.join("receipt.json")).current_dir(&dir)
            .env_clear().stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
        for key in &c.inherit_environment{if let Some(value)=std::env::var_os(key){command.env(key,value);}}
        let started=Instant::now();
        // No shell interpolation. The trusted executable owns provider access.
        // No auto-respawn, including spawn failure and timeout.
        let mut child=match command.spawn(){
            Ok(child)=>Owned(child),
            Err(e)=>{
                write(&dir,"exit.json",&json!({"spawned":false,"error":e.to_string()}))?;
                return Ok(json!({"handoff":key,"state":"spawn_failed_outcome_not_assumed","send_authority":"none"}));
            }
        };
        let (status,timed_out)=loop{
            if let Some(s)=child.0.try_wait().map_err(err)?{break (s,false);}
            if started.elapsed()>=Duration::from_millis(budget){
                child.0.kill().map_err(err)?;break (child.0.wait().map_err(err)?,true);
            }
            thread::sleep(Duration::from_millis(10));
        };
        write(&dir,"exit.json",&json!({"spawned":true,"exit_code":status.code(),"success":status.success(),"timed_out":timed_out,
            "direct_child_elapsed_ms":u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX)}))?;
        self.reconcile_locked(store,mailbox,key)
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn rejects_unpinned_or_unbounded_configuration(){
        let mut c=Config{executable:PathBuf::from("relative"),executable_sha256:"0".repeat(64),args:vec![],
            inherit_environment:vec![],max_runtime_ms:1000,max_receipt_bytes:4096,max_jobs:1};
        assert!(c.validate().is_err());
        c.executable=std::env::current_exe().unwrap();c.max_runtime_ms=0;
        assert!(c.validate().is_err());
    }
}
