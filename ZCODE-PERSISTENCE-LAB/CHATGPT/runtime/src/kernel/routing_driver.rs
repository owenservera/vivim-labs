//! Trusted adapter driver. Synthetic and external mailbox transports are local.
//! External executors remain separately provisioned; no implicit network calls.
use super::{model::{Id,ensure,id,bounded,MAX_VALUE_BYTES},persistence::KernelStore,transition::{Actor,Action,Request},routing::{Usage,Outcome},routing_adapter::{Envelope,AdapterReceipt,HandoffPhase,Transport}};
use crate::{Expr,digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::{fs::{self,OpenOptions,File},io::{Read,Seek,SeekFrom,Write},path::{Path,PathBuf},time::{SystemTime,UNIX_EPOCH}};
fn error(e:impl std::fmt::Display)->String{e.to_string()}
fn apply(store:&KernelStore,key:Id,action:Action)->Result<Value>{store.apply(Request{id:key,actor:Actor::Host,action})}
fn key(kind:&str,id:&str)->Id{format!("{kind}-{}",digest(&json!(id)))}
pub(crate) fn advance_clock(store:&KernelStore)->Result<()>{
    let state=store.inspect()?;
    let now=u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH).map_err(error)?.as_millis()).map_err(error)?.max(state.clock_ms);
    if now>state.clock_ms{
        if let Err(reason)=apply(store,key("adapter-tick",&now.to_string()),Action::Tick{now_ms:now}){
            if store.inspect()?.clock_ms<now{return Err(reason);}
        }
    }Ok(())
}
pub trait Backend {
    fn supports(&self,envelope:&Envelope)->bool;
    fn transport(&self)->Transport{Transport::Direct}
    fn finish_action(&self,receipt:AdapterReceipt)->Result<Action>{Ok(Action::FinishHandoff{receipt})}
    /// Called only after a new committed claim. Err means uncertain outcome.
    fn send(&mut self,envelope:&Envelope,input:&Value)->Result<AdapterReceipt>;
    /// Asynchronous transports may durably publish work and return None.
    fn begin(&mut self,envelope:&Envelope,input:&Value)->Result<Option<AdapterReceipt>>{self.send(envelope,input).map(Some)}
    /// Observation only. None means unknown, not permission to resend.
    fn reconcile(&mut self,envelope:&Envelope)->Result<Option<AdapterReceipt>>;
}
pub fn step<B:Backend>(store:&KernelStore,adapter:&str,backend:&mut B,limit:usize)->Result<Value>{
    id(adapter)?;ensure((1..=128).contains(&limit),"invalid driver operation budget")?;
    // Serialize all cooperating drivers for this adapter. Store transaction
    // fencing still protects claims independently of this lock.
    let mut lock=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(store.root().join(format!("adapter-{adapter}.lock"))).map_err(error)?;
    lock.try_lock().map_err(|e|format!("adapter driver already owned: {e:?}"))?;
    let state=store.inspect()?;
    let transport=backend.transport();transport.validate()?;
    let mut queue:Vec<_>=state.router.adapters.handoffs.values().filter(|e|e.adapter==adapter&&matches!(e.phase,HandoffPhase::Prepared|HandoffPhase::Claimed|HandoffPhase::Unknown)).cloned().collect();
    // Unsupported entries must not consume the operation budget. Rotate the
    // stable order after the last visited item, including unknown outcomes.
    // Cursor is advisory only; losing it never grants new send authority.
    queue.retain(|e|backend.supports(e)&&state.router.adapters.transport_claims.get(&e.id).is_none_or(|t|t==&transport));
    queue.sort_by_key(|e|e.id.clone());
    let mut cursor=String::new();
    if lock.metadata().map_err(error)?.len()<=100{
        lock.seek(SeekFrom::Start(0)).map_err(error)?;
        if (&mut lock).take(100).read_to_string(&mut cursor).is_err(){cursor.clear();}
    }
    if !queue.is_empty(){
        let offset=queue.partition_point(|e|e.id.as_str()<=cursor.as_str())%queue.len();
        queue.rotate_left(offset);
    }
    let mut sent=0;let mut published=0;let mut reconciled=0;let mut unknown=0;let mut errors=vec![];let mut visited=0usize;
    for e in queue.into_iter().take(limit){
        lock.seek(SeekFrom::Start(0)).map_err(error)?;lock.set_len(0).map_err(error)?;
        lock.write_all(e.id.as_bytes()).map_err(error)?;lock.sync_data().map_err(error)?;
        visited+=1;
        advance_clock(store)?;
        let state=store.inspect()?;
        let Some(e)=state.router.adapters.handoffs.get(&e.id).cloned() else{continue;};
        if !backend.supports(&e)||state.router.adapters.transport_claims.get(&e.id).is_some_and(|t|t!=&transport){continue;}
        let result=match e.phase{
            HandoffPhase::Prepared=>{
                match apply(store,key("claim",&e.id),Action::ClaimTransportHandoff{handoff:e.id.clone(),transport:transport.clone()}){
                    Ok(v) if (transport==Transport::Direct&&v["send_authority"]=="single_attempt_only")
                        ||(matches!(&transport,Transport::Mailbox{..})&&v["send_authority"]=="publish_only")=>{
                        let claimed:Envelope=serde_json::from_value(v["handoff"].clone()).map_err(error)?;
                        if matches!(&transport,Transport::Mailbox{..}){published+=1;}else{sent+=1;}
                        backend.begin(&claimed,&v["input"])
                    }
                    Ok(_)=>backend.reconcile(&e),
                    Err(reason)=>{errors.push(json!({"handoff":e.id,"stage":"claim","reason":reason}));continue;}
                }
            }
            HandoffPhase::Claimed|HandoffPhase::Unknown=>backend.reconcile(&e),
            _=>continue,
        };
        // A slow backend cannot use an old task clock to admit a late result.
        // If the clock update fails, the receipt remains reconcilable.
        advance_clock(store)?;
        match result{
            Ok(Some(receipt))=>{
                if receipt.handoff!=e.id{unknown+=1;let _=apply(store,key("unknown",&e.id),Action::InferenceUnknown{reservation:e.id.clone()});errors.push(json!({"handoff":e.id,"stage":"receipt","reason":"wrong handoff"}));continue;}
                let request=key("finish",&e.id);
                match backend.finish_action(receipt).and_then(|action|apply(store,request,action)){
                    Ok(_)=>reconciled+=1,
                    Err(reason)=>{unknown+=1;let _=apply(store,key("unknown",&e.id),Action::InferenceUnknown{reservation:e.id.clone()});errors.push(json!({"handoff":e.id,"stage":"finish","reason":reason}));}
                }
            }
            Ok(None)=>{unknown+=1;let _=apply(store,key("unknown",&e.id),Action::InferenceUnknown{reservation:e.id.clone()});}
            Err(reason)=>{unknown+=1;let _=apply(store,key("unknown",&e.id),Action::InferenceUnknown{reservation:e.id.clone()});errors.push(json!({"handoff":e.id,"stage":"backend","reason":reason}));}
        }
    }
    Ok(json!({"adapter":adapter,"send_attempts":sent,"publication_intents":published,"settled":reconciled,"visited":visited,"unknown":unknown,"errors":errors}))
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
    fn supports(&self,e:&Envelope)->bool{self.check(e).is_ok()}
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

/// Explicit bounded offline orchestration. Only synthetic backend calls are
/// permitted by this entrypoint. No background service or live provider calls.
pub fn synthetic_cycle(store:&KernelStore,backend:&mut Synthetic,max_rounds:usize,max_operations:usize)->Result<Value>{
    ensure((1..=128).contains(&max_rounds)&&(1..=128).contains(&max_operations),"invalid synthetic cycle budget")?;
    let _controller=store.controller_lock()?;
    let mut records=vec![];
    for round in 0..max_rounds{
        advance_clock(store)?;
        let s=store.inspect()?;
        // Refuse mixed live/synthetic stores: the offline loop must not
        // prepare jobs for real providers or contaminate their performance data.
        let c=s.router.config.as_ref().ok_or("router not configured")?;
        ensure(c.profiles.values().filter(|p|p.enabled).all(|p|p.provider==backend.config.provider&&p.adapter==backend.config.adapter&&p.account==backend.config.account&&p.model==backend.config.model&&p.thinking==backend.config.thinking),"synthetic cycle requires exclusively matching synthetic profiles")?;
        let prepared=if s.routing_cycle_needed(){apply(store,key("synthetic-cycle",&format!("{}-{round}",s.sequence)),Action::RoutingCycle{limit:max_operations})?}else{json!({"prepared":[]})};
        let adapter=backend.config.adapter.clone();
        let report=step(store,&adapter,backend,max_operations)?;
        let s=store.inspect()?;
        let mut verified=0usize;
        let submitted:Vec<_>=s.router.adapters.handoffs.values().filter(|e|e.adapter==adapter&&e.phase==HandoffPhase::Completed&&s.tasks.get(&e.task).is_some_and(|t|t.phase==super::model::TaskPhase::Submitted&&t.generation==e.generation)).map(|e|e.task.clone()).take(max_operations).collect();
        for task in submitted{let v=apply(store,key("synthetic-verify",&format!("{}-{}",s.sequence,task)),Action::Verify{task})?;if v["accepted"]==true{verified+=1;}}
        let progress=prepared["prepared"].as_array().is_some_and(|a|!a.is_empty())||report["settled"].as_u64().unwrap_or(0)>0||verified>0;
        records.push(json!({"round":round,"prepared":prepared["prepared"].as_array().map_or(0,|a|a.len()),"driver":report,"verified":verified}));
        if !progress{break;}
    }
    Ok(json!({"rounds":records,"provider_calls":0,"mode":"synthetic_only"}))
}
