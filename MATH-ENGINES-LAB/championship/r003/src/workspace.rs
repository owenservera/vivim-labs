//! Materialize independent entrant workspaces from the canonical roster.
//! Creates a new explicitly selected directory only; never overwrites a state.
use serde_json::Value;
use std::{collections::BTreeSet,fs,path::Path,io::{self,Write}};
const ROSTER:&str=include_str!("../roster.json");
const DIRS:[&str;13]=["genesis","bootstrap","architecture","registry","generator","adversary","ledger","evidence","recovery","memory","measurements","promotion","receipts"];
pub fn roster()->Result<Vec<Value>,String>{
    let root:Value=serde_json::from_str(ROSTER).map_err(|e|e.to_string())?;
    let entries=root["entrants"].as_array().ok_or("missing entrants")?;
    if entries.len()!=64{return Err("roster must have 64 slots".into());}
    let mut names=BTreeSet::new();
    for (i,e) in entries.iter().enumerate(){
        let id=format!("P{:03}",i+1);
        if e["id"].as_str()!=Some(id.as_str())||e["conference"].as_u64()!=Some((i/16+1)as u64){return Err(format!("invalid slot {id}"));}
        if i<32{
            let name=e["name"].as_str().ok_or("missing name")?;
            if name.is_empty()||!names.insert(name){return Err(format!("duplicate or missing name {id}"));}
            for key in ["architecture","seed_direction","adversary"]{
                if e[key].as_str().is_none_or(str::is_empty){return Err(format!("{id}: missing {key}"));}
            }
            let obligations=e["deliverables"].as_object().ok_or("missing obligations")?;
            if obligations.len()!=16||(1..=16).any(|n|!obligations.contains_key(&format!("D{n:02}"))){return Err(format!("invalid obligations {id}"));}
            if e["status"]!="SCAFFOLDED"{return Err(format!("invalid development state {id}"));}
        }else if e["status"]!="RESERVED"||!e["name"].is_null(){return Err(format!("invalid reservation {id}"));}
        if !e["evolution_head"].is_null()||!e["genesis"].is_null(){return Err(format!("unqualified head {id}"));}
    }
    Ok(entries.clone())
}
fn write_new(path:&Path,data:&[u8])->io::Result<()>{
    let mut f=fs::OpenOptions::new().create_new(true).write(true).open(path)?;f.write_all(data)?;f.sync_all()
}
pub fn materialize(root:&Path)->Result<(),Box<dyn std::error::Error>>{
    let entries=roster().map_err(io::Error::other)?;
    fs::create_dir(root)?; // fail if exists; no overwrite/reuse
    write_new(&root.join("roster.json"),ROSTER.as_bytes())?;
    let candidates=root.join("candidates");fs::create_dir(&candidates)?;
    for e in &entries[..32]{
        let dir=candidates.join(e["id"].as_str().unwrap());fs::create_dir(&dir)?;
        write_new(&dir.join("manifest.json"),serde_json::to_string_pretty(e)?.as_bytes())?;
        for sub in DIRS{fs::create_dir(dir.join(sub))?;}
    }
    // Evaluator space is a layout only; directory segregation is not a
    // confidentiality boundary. Independent evaluator must secure holdouts.
    let evaluator=root.join("evaluator");fs::create_dir(&evaluator)?;
    for sub in ["development","sealed-holdout-placeholder","submissions","receipts","bracket","inheritance"]{
        fs::create_dir(evaluator.join(sub))?;
    }
    println!("WORKSPACE created=32 reserved=32 evaluator_layout=CREATED holdout=NOT_SEALED");
    Ok(())
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn roster_contract(){let r=roster().unwrap();assert_eq!(r.len(),64);assert_eq!(r.iter().filter(|e|e["status"]=="SCAFFOLDED").count(),32);}
}
