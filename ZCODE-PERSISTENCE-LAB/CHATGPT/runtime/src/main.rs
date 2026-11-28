use persistent_intelligence_runtime::{Command,Store,digest};
use serde_json::Value;
use std::{env,fs,io::{self,Read}};
fn read_json<T:serde::de::DeserializeOwned>(file:&str)->Result<T,String>{
    let text=if file=="-"{let mut s=String::new();io::stdin().read_to_string(&mut s).map_err(|e|e.to_string())?;s}else{fs::read_to_string(file).map_err(|e|e.to_string())?};
    serde_json::from_str(&text).map_err(|e|e.to_string())
}
fn required<'a>(args:&'a [String],n:usize,label:&str)->Result<&'a str,String>{
    args.get(n).map(String::as_str).ok_or_else(||format!("{label} required"))
}
fn kernel_cli(args:&[String])->Result<(),String>{
    use persistent_intelligence_runtime::kernel::{model::*,transition::{Actor,Action,Request},persistence::{KernelStore,Anchor},supervisor::{RunConfig,supervise}};
    use std::path::Path;
    let root=required(args,1,"kernel store")?;
    let action=required(args,2,"kernel action")?;
    let store=KernelStore::open(root)?;
    let output=match action{
        "inspect"=>serde_json::to_value(store.inspect()?).map_err(|e|e.to_string())?,
        "ready"=>serde_json::json!(store.inspect()?.ready()),
        "anchor"=>serde_json::to_value(store.anchor()?).map_err(|e|e.to_string())?,
        "verify-anchor"=>{let anchor:Anchor=read_json(required(args,3,"anchor file")?)?;store.verify_anchor(&anchor)?;serde_json::json!({"verified":true})}
        "apply"=>{let request:Request=read_json(required(args,3,"request JSON file or -")?)?;store.apply(request)?}
        "bootstrap"=>{
            let mut receipts=Vec::new();
            for contract in persistent_intelligence_runtime::kernel::native_operator_contracts()?{
                let id=format!("bootstrap-{}",contract.id);
                receipts.push(store.apply(Request{id,actor:Actor::Host,action:Action::Operator{contract}})?);
            }
            serde_json::json!({"registered":receipts.len(),"receipts":receipts})
        }
        "discover"=>{
            #[derive(serde::Deserialize)]
            struct Spec{family:Id,training:Vec<Id>,holdouts:Vec<Id>,input:Shape,output:Shape,limit:u64}
            let s:Spec=read_json(required(args,3,"discovery spec")?)?;
            let d=store.inspect()?.discover(&s.family,s.training,s.holdouts,s.input,s.output,s.limit)?;
            serde_json::to_value(d).map_err(|e|e.to_string())?
        }
        "assimilate"=>{
            let request_id=required(args,3,"request ID")?.to_owned();
            let name=required(args,4,"capability name")?.to_owned();
            let version=required(args,5,"capability version")?.parse::<u64>().map_err(|e|e.to_string())?;
            let discovery=read_json(required(args,6,"discovery file")?)?;
            store.assimilate(request_id,name,version,discovery)?
        }
        "run"=>{
            let workspace=Path::new(required(args,3,"workspace")?);
            let config:RunConfig=read_json(required(args,4,"run config")?)?;
            supervise(&store,workspace,config)?
        }
        "project"=>{
            let agent=required(args,3,"agent ID")?;
            let workspace=Path::new(required(args,4,"projection root")?);
            serde_json::json!({"path":store.project_agent(agent,workspace)?})
        }
        _=>return Err("unknown kernel action; inspect|ready|anchor|verify-anchor|apply|bootstrap|discover|assimilate|run|project".into()),
    };
    println!("{}",serde_json::to_string_pretty(&output).map_err(|e|e.to_string())?);
    Ok(())
}
fn run()->Result<(),String>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.first().is_some_and(|x|x=="kernel-compute"){
        return persistent_intelligence_runtime::kernel::supervisor::compute_file(
            std::path::Path::new(required(&args,1,"work file")?),
            std::path::Path::new(required(&args,2,"result file")?)
        );
    }
    if args.first().is_some_and(|x|x=="kernel"){return kernel_cli(&args);}
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
