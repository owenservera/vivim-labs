//! One explicitly permitted external effect: publication of a JSON object in a
//! host-owned, content-addressed local directory. No arbitrary paths or commands.
//! Caller holds the canonical writer lock over observation/publication/admission.
use super::model::*;
use crate::{digest,Result};
use serde_json::{Value,json};
use std::{fs::{self,OpenOptions},io::Write,path::Path,time::{SystemTime,UNIX_EPOCH}};
fn err(e:impl std::fmt::Display)->String{e.to_string()}
pub fn observe(root:&Path,effect:&Effect,payload:&Value,allow_publish:bool)->Result<(EffectPhase,Value)>{
    ensure(hash_id(&effect.payload)&&digest(payload)==effect.payload,"effect payload mismatch")?;
    ensure(payload.get("adapter").and_then(Value::as_str)==Some("local-json-publication-v1"),"adapter not authorized by intent")?;
    let value=payload.get("value").ok_or("publication value missing")?;
    bounded(value)?;
    let dir=root.join("effect-publications");fs::create_dir_all(&dir).map_err(err)?;
    let object=digest(value);let path=dir.join(format!("{object}.json"));
    if !path.exists()&&allow_publish{
        let stamp=SystemTime::now().duration_since(UNIX_EPOCH).map_err(err)?.as_nanos();
        let tmp=dir.join(format!(".tmp-{}-{stamp}",std::process::id()));
        let mut f=OpenOptions::new().write(true).create_new(true).open(&tmp).map_err(err)?;
        f.write_all(&serde_json::to_vec(value).map_err(err)?).map_err(err)?;f.sync_all().map_err(err)?;drop(f);
        ensure(!path.exists(),"publication collision")?;
        fs::rename(tmp,&path).map_err(err)?;
        #[cfg(unix)] fs::File::open(&dir).and_then(|f|f.sync_all()).map_err(err)?;
    }
    let outcome=if path.exists(){
        ensure(fs::metadata(&path).map_err(err)?.len()<=MAX_VALUE_BYTES as u64,"publication exceeds read limit")?;
        let observed:Value=serde_json::from_slice(&fs::read(&path).map_err(err)?).map_err(err)?;
        ensure(digest(&observed)==object&&observed==*value,"publication corrupted or conflicting")?;
        EffectPhase::Committed
    }else{EffectPhase::Absent};
    Ok((outcome.clone(),json!({"effect":effect.id,"outcome":outcome,"adapter":"local-json-publication-v1","object":object})))
}
