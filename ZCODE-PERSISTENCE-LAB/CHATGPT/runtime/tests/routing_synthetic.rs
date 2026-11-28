//! Authored offline backend qualification; never touches a real AI provider.
use persistent_intelligence_runtime::{Expr,digest,kernel::{routing::{Usage,Outcome},routing_adapter::{Envelope,HandoffPhase},routing_driver::{Backend,Synthetic,SyntheticConfig}}};
use serde_json::json;
use std::{fs,path::PathBuf,time::{SystemTime,UNIX_EPOCH}};
struct Temp(PathBuf);impl Drop for Temp{fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
#[test]
fn synthetic_receipt_survives_backend_replacement(){
    let root=Temp(std::env::temp_dir().join(format!("vivim-synthetic-{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())));
    let config=SyntheticConfig{adapter:"synthetic-adapter".into(),provider:"synthetic-provider".into(),account:"lab".into(),model:"fixture".into(),thinking:"low".into(),program:Expr::Input,usage:Usage{requests:1,input:1,output:1,reasoning:0,credits:0},latency_ms:1};
    let input=json!(42);
    let e=Envelope{id:"handoff".into(),reservation:"handoff".into(),adapter:config.adapter.clone(),binding_version:1,profile:"fixture".into(),provider:config.provider.clone(),account:config.account.clone(),model:config.model.clone(),thinking:config.thinking.clone(),task:"task".into(),generation:1,worker:"worker".into(),incarnation:1,input_digest:digest(&input),ceiling:config.usage.clone(),deadline_ms:1000,phase:HandoffPhase::Claimed,claimed_ms:Some(1),receipt:None};
    let mut first=Synthetic::open(&root.0,config.clone()).unwrap();
    assert!(first.reconcile(&e).unwrap().is_none());
    let receipt=first.send(&e,&input).unwrap();assert_eq!(receipt.outcome,Outcome::Completed);
    drop(first);
    let mut replacement=Synthetic::open(&root.0,config).unwrap();
    let recovered=replacement.reconcile(&e).unwrap().unwrap();
    assert_eq!(recovered.request_id,receipt.request_id);assert_eq!(recovered.output,Some(input));
    let mut foreign=e.clone();foreign.provider="real-provider".into();
    assert!(!replacement.supports(&foreign));assert!(replacement.reconcile(&foreign).is_err());
}
