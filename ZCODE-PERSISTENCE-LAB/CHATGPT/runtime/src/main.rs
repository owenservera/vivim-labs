use persistent_intelligence_runtime::{Command,Store,digest};
use serde_json::Value;
use std::{env,fs,io::{self,Read}};
fn run()->Result<(),String>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.first().is_some_and(|x|x=="hash"){
        let mut s=String::new();io::stdin().read_to_string(&mut s).map_err(|e|e.to_string())?;
        let v:Value=serde_json::from_str(&s).map_err(|e|e.to_string())?;
        println!("{}",digest(&v)); return Ok(());
    }
    let root=args.first().ok_or("usage: runtime <store> inspect|ready|drain <goal> <max-steps>|apply [request-id] | apply-file <file> [request-id]; or hash < JSON")?;
    let action=args.get(1).ok_or("action required")?;
    let store=Store::open(root)?;
    let output=match action.as_str(){
        "inspect"=>serde_json::to_value(store.inspect()?).map_err(|e|e.to_string())?,
        "ready"=>serde_json::to_value(store.inspect()?.ready()).map_err(|e|e.to_string())?,
        "drain"=>{
            let goal=args.get(2).ok_or("goal required")?.clone();
            let max:usize=args.get(3).ok_or("maximum steps required")?.parse().map_err(|_|"invalid step limit")?;
            if max==0 || max>10_000{return Err("step limit must be 1..10000".into());}
            let mut results=Vec::new();
            for _ in 0..max{
                let result=store.apply(Command::Step{goal:goal.clone(),ttl:1})?;
                let idle=result["action"]=="idle";results.push(result);if idle{break;}
            }
            Value::Array(results)
        },
        "apply"|"apply-file"=>{
            let (data,id)=if action=="apply"{
                let mut s=String::new();io::stdin().read_to_string(&mut s).map_err(|e|e.to_string())?;(s,args.get(2).cloned())
            } else {
                (fs::read_to_string(args.get(2).ok_or("command file required")?).map_err(|e|e.to_string())?,args.get(3).cloned())
            };
            let c:Command=serde_json::from_str(&data).map_err(|e|e.to_string())?;
            store.apply_request(id,c)?
        }
        _=>return Err("unknown action".into())
    };
    println!("{}",serde_json::to_string_pretty(&output).map_err(|e|e.to_string())?);Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1);}}
