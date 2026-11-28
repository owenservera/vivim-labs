//! Trusted adapter driver. The only built-in backend is synthetic and local.
//! The host must supply a bounded real backend later; no credentials or network.
use super::{model::{Id,ensure,id,bounded,MAX_VALUE_BYTES},persistence::KernelStore,transition::{Actor,Action,Request},routing::{Usage,Outcome},routing_adapter::{Envelope,AdapterReceipt,HandoffPhase}};
use crate::{Expr,digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::{fs::{self,OpenOptions,File},io::Write,path::{Path,PathBuf},time::{SystemTime,UNIX_EPOCH}};
fn error(e:impl std::fmt::Display)->String{e.to_string()}
fn apply(store:&KernelStore,key:Id,action:Action)->Result<Value>{store.apply(Request{id:key,actor:Actor::Host,action})}
fn key(kind:&str,id:&str)->Id{format!("{kind}-{}",digest(&json!(id)))}
pub trait Backend {
    /// Called only after a new committed claim. Err means uncertain outcome.
    fn send(&mut self,envelope:&Envelope,input:&Value)->Result<AdapterReceipt>;
    /// Observation only. None means unknown, not permission to resend.
    fn reconcile(&mut self,envelope:&Envelope)->Result<Option<AdapterReceipt>>;
}
pub fn step<B:Backend>(store:&KernelStore,adapter:&str,backend:&mut B,limit:usize)->Result<Value>{
    id(adapter)?;ensure((1..=128).contains(&limit),"invalid driver operation budget")?;
    // Serialize all cooperating drivers for this adapter. Store transaction
    // fencing still protects claims independently of this lock.
    let lock=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(store.root().join(format!("adapter-{adapter}.lock"))).map_err(error)?;
    lock.try_lock().map_err(|e|format!("adapter driver already owned: {e:?}"))?;
    let state=store.inspect()?;
    let mut queue:Vec<_>=state.router.adapters.handoffs.values().filter(|e|e.adapter==adapter&&matches!(e.phase,HandoffPhase::Prepared|HandoffPhase::Claimed|HandoffPhase::Unknown)).cloned().collect();
    // Reconcile old outcomes first, even if the binding is now disabled.
    queue.sort_by_key(|e|(e.phase==HandoffPhase::Prepared,e.deadline_ms,e.id.clone()));
    let mut sent=0;let mut reconciled=0;let mut unknown=0;let mut errors=vec![];
    for e in queue.into_iter().take(limit){
        let current=store.inspect()?;
        let wall=u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH).map_err(error)?.as_millis()).map_err(error)?;
        let now=wall.max(current.clock_ms);
        if now>current.clock_ms{apply(store,key("adapter-tick",&format!("{adapter}-{now}")),Action::Tick{now_ms:now})?;}
        let state=store.inspect()?;
        let Some(e)=state.router.adapters.handoffs.get(&e.id).cloned() else{continue;};
        let result=match e.phase{
            HandoffPhase::Prepared=>{
                match apply(store,key("claim",&e.id),Action::ClaimHandoff{handoff:e.id.clone()}){
                    Ok(v) if v["send_authority"]=="single_attempt_only"=>{
                        sent+=1;backend.send(&e,&v["input"]).map(Some)
                    }
                    Ok(_)=>backend.reconcile(&e),
                    Err(reason)=>{errors.push(json!({"handoff":e.id,"stage":"claim","reason":reason}));continue;}
                }
            }
            HandoffPhase::Claimed|HandoffPhase::Unknown=>backend.reconcile(&e),
            _=>continue,
        };
        match result{
            Ok(Some(receipt))=>{
                if receipt.handoff!=e.id{unknown+=1;errors.push(json!({"handoff":e.id,"stage":"receipt","reason":"wrong handoff"}));continue;}
                let request=key("finish",&e.id);
                match apply(store,request,Action::FinishHandoff{receipt}){
                    Ok(_)=>reconciled+=1,
                    Err(reason)=>{unknown+=1;errors.push(json!({"handoff":e.id,"stage":"finish","reason":reason}));}
                }
            }
            Ok(None)=>{unknown+=1;let _=apply(store,key("unknown",&e.id),Action::InferenceUnknown{reservation:e.id.clone()});}
            Err(reason)=>{unknown+=1;let _=apply(store,key("unknown",&e.id),Action::InferenceUnknown{reservation:e.id.clone()});errors.push(json!({"handoff":e.id,"stage":"backend","reason":reason}));}
        }
    }
    Ok(json!({"adapter":adapter,"send_attempts":sent,"settled":reconciled,"unknown":unknown,"errors":errors}))
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct SyntheticConfig{
    pub adapter:Id,pub provider:Id,pub account:Id,pub model:String,pub thinking:String,
    pub program:Expr,pub usage:Usage,pub latency_ms:u64,
}
pub struct Synthetic{config:SyntheticConfig,root:PathBuf}
impl Synthetic{
    pub fn open(root:&Path,config:SyntheticConfig)->Result<Self>{
        id(&config.adapter)?;id(&config.provider)?;id(&config.account)?;
        ensure(config.provider.starts_with("synthetic-")&&config.adapter.starts_with("synthetic-"),"synthetic backend requires synthetic-only profile identity")?;
        ensure(config.usage.requests==1,"synthetic request accounting must be one")?;
        fs::create_dir_all(root).map_err(error)?;
        Ok(Self{config,root:root.canonicalize().map_err(error)?})
    }
    fn check(&self,e:&Envelope)->Result<()>{
        ensure(e.adapter==self.config.adapter&&e.provider==self.config.provider&&e.account==self.config.account&&e.model==self.config.model&&e.thinking==self.config.thinking,"synthetic scope mismatch")
    }
    fn path(&self,e:&Envelope)->PathBuf{self.root.join(format!("{}.json",digest(&json!([e.id,e.input_digest,e.generation,e.profile]))))}
    fn existing(&self,e:&Envelope)->Result<Option<AdapterReceipt>>{
        let p=self.path(e);if !p.exists(){return Ok(None);}
        ensure(fs::metadata(&p).map_err(error)?.len()<=2*MAX_VALUE_BYTES as u64,"synthetic receipt too large")?;
        let v:Value=serde_json::from_slice(&fs::read(p).map_err(error)?).map_err(error)?;
        let expected=digest(&v["receipt"]);
        ensure(v["digest"].as_str()==Some(expected.as_str()),"synthetic receipt integrity mismatch")?;
        let r:AdapterReceipt=serde_json::from_value(v["receipt"].clone()).map_err(error)?;
        ensure(r.handoff==e.id&&r.provider==e.provider&&r.account==e.account&&r.model==e.model&&r.thinking==e.thinking,"synthetic receipt identity mismatch")?;
        Ok(Some(r))
    }
}
impl Backend for Synthetic{
    fn send(&mut self,e:&Envelope,input:&Value)->Result<AdapterReceipt>{
        self.check(e)?;ensure(digest(input)==e.input_digest,"input digest mismatch")?;
        if let Some(r)=self.existing(e)?{return Ok(r);}
        let u=&self.config.usage;let cap=&e.ceiling;
        ensure(u.requests<=cap.requests&&u.input<=cap.input&&u.output<=cap.output&&u.reasoning<=cap.reasoning&&u.credits<=cap.credits,"synthetic usage exceeds ceiling")?;
        let output=self.config.program.eval(input).ok().filter(|v|bounded(v).is_ok());
        let outcome=if output.is_some(){Outcome::Completed}else{Outcome::Rejected};
        let r=AdapterReceipt{handoff:e.id.clone(),request_id:key("synthetic",&e.id),provider:e.provider.clone(),account:e.account.clone(),model:e.model.clone(),thinking:e.thinking.clone(),usage:u.clone(),outcome,provider_receipt:format!("synthetic-only:{}",e.id),latency_ms:self.config.latency_ms,output};
        let v=serde_json::to_value(&r).map_err(error)?;let body=json!({"digest":digest(&v),"receipt":v});
        let p=self.path(e);let tmp=p.with_extension("tmp");
        if tmp.exists(){fs::remove_file(&tmp).map_err(error)?;}
        let mut f=OpenOptions::new().create_new(true).write(true).open(&tmp).map_err(error)?;
        f.write_all(&serde_json::to_vec(&body).map_err(error)?).map_err(error)?;f.sync_all().map_err(error)?;drop(f);
        ensure(!p.exists(),"synthetic receipt collision")?;fs::rename(tmp,p).map_err(error)?;
        #[cfg(unix)] File::open(&self.root).and_then(|f|f.sync_all()).map_err(error)?;
        Ok(r)
    }
    fn reconcile(&mut self,e:&Envelope)->Result<Option<AdapterReceipt>>{
        self.check(e)?;self.existing(e)
    }
}
