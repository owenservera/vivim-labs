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
        "lookup"=>store.lookup(required(args,3,"table")?,required(args,4,"entity ID")?)?.unwrap_or(Value::Null),
        "scan"=>{
            let table=required(args,3,"table")?;
            let limit=required(args,4,"entry limit")?.parse::<usize>().map_err(|e|e.to_string())?;
            let pages=required(args,5,"page limit")?.parse::<usize>().map_err(|e|e.to_string())?;
            let bytes=required(args,6,"byte budget")?.parse::<usize>().map_err(|e|e.to_string())?;
            let cursor=args.get(7).map(|p|read_json(p)).transpose()?;
            store.scan(table,cursor,limit,pages,bytes)?
        },
        "collect"=>serde_json::json!({"objects_removed":store.collect_garbage()?}),
        "population"=>store.inspect()?.audit_population(),
        "resources"=>store.inspect()?.resource_report(),
        "provider-accounting"=>store.inspect()?.provider_accounting(),
        "endurance-evaluate"=>{
            use persistent_intelligence_runtime::kernel::endurance::{Policy,Observation};
            let policy:Policy=read_json(required(args,3,"endurance policy file")?)?;
            let observed:Observation=read_json(required(args,4,"observation file")?)?;
            serde_json::to_value(policy.evaluate(&observed)?).map_err(|e|e.to_string())?
        },
        "recipes"=>serde_json::to_value(store.inspect()?.recipes).map_err(|e|e.to_string())?,
        "assimilation"=>store.inspect()?.assimilation_report()?,
        "reuse-economics"=>store.inspect()?.reuse_comparison_report(required(args,3,"comparison ID")?)?,
        "assimilation-review"=>serde_json::to_value(store.inspect()?.evaluate_trace_pattern(required(args,3,"frozen candidate")?)?).map_err(|e|e.to_string())?,
        "assimilation-plan"=>store.inspect()?.trace_reuse_plan(required(args,3,"family")?)?,
        "repairs"=>serde_json::to_value(store.inspect()?.recipe_repairs).map_err(|e|e.to_string())?,
        "routing-policy-candidates"=>store.inspect()?.router.policy_candidates()?,
        "routing-trials"=>serde_json::to_value(store.inspect()?.router.evolution).map_err(|e|e.to_string())?,
        "routing-joint-plan"=>{
            let limit=required(args,3,"plan limit")?.parse::<usize>().map_err(|e|e.to_string())?;
            serde_json::to_value(store.inspect()?.joint_routing_plan(limit)?).map_err(|e|e.to_string())?
        },
        "routing-shadow-compare"=>{
            let limit=required(args,3,"plan limit")?.parse::<usize>().map_err(|e|e.to_string())?;
            let policies=read_json(required(args,4,"shadow policies file")?)?;
            store.inspect()?.compare_routing_policies(limit,policies)?
        },
        "routing-synthetic-cycle"=>{
            use persistent_intelligence_runtime::kernel::routing_driver::{Synthetic,SyntheticConfig,synthetic_cycle};
            let config:SyntheticConfig=read_json(required(args,3,"synthetic config")?)?;
            let ledger=Path::new(required(args,4,"synthetic ledger")?);
            let rounds=required(args,5,"rounds")?.parse::<usize>().map_err(|e|e.to_string())?;
            let limit=required(args,6,"operations per round")?.parse::<usize>().map_err(|e|e.to_string())?;
            let mut backend=Synthetic::open(ledger,config)?;
            synthetic_cycle(&store,&mut backend,rounds,limit)?
        },
        "routing-synthetic-step"=>{
            use persistent_intelligence_runtime::kernel::routing_driver::{Synthetic,SyntheticConfig,step};
            let config:SyntheticConfig=read_json(required(args,3,"synthetic config")?)?;
            let adapter=config.adapter.clone();
            let ledger=Path::new(required(args,4,"synthetic ledger")?);
            let limit=required(args,5,"operation limit")?.parse::<usize>().map_err(|e|e.to_string())?;
            let mut backend=Synthetic::open(ledger,config)?;
            step(&store,&adapter,&mut backend,limit)?
        },
        "routing-mailbox"=>{
            use persistent_intelligence_runtime::kernel::{adapter_mailbox::{Mailbox,Config},routing_adapter::AdapterReceipt,routing_driver::step};
            let op=required(args,3,"mailbox operation")?;
            let root=Path::new(required(args,4,"mailbox root")?);
            if op=="create"{
                let config:Config=read_json(required(args,5,"mailbox config")?)?;
                Mailbox::create(root,&store,config)?.inspect()
            }else{
                let mut mailbox=Mailbox::open(root,&store)?;
                match op{
                    "inspect"=>mailbox.inspect(),
                    "step"=>{let adapter=mailbox.adapter().to_owned();let limit=required(args,5,"operation limit")?.parse::<usize>().map_err(|e|e.to_string())?;step(&store,&adapter,&mut mailbox,limit)},
                    "take"=>mailbox.take(&store,required(args,5,"handoff ID")?),
                    "submit"=>{let receipt:AdapterReceipt=read_json(required(args,5,"receipt file")?)?;mailbox.submit(receipt)},
                    _=>Err("unknown mailbox operation".into()),
                }
            }?
        },
        "routing-adapter-queue"=>store.inspect()?.adapter_queue(required(args,3,"adapter ID")?)?,
        "routing-refresh-plan"=>store.inspect()?.router.quota_refresh_plan()?,
        "routing-cycle"=>{
            let limit=required(args,3,"reservation limit")?.parse::<usize>().map_err(|e|e.to_string())?;
            store.apply(Request{id:required(args,4,"request ID")?.into(),actor:Actor::Host,action:Action::RoutingCycle{limit}})?
        },
        "routing-budget"=>store.inspect()?.router.budget_report()?,
        "routing-plan"=>store.inspect()?.inference_plan()?,
        "routing-recommend"=>serde_json::to_value(store.inspect()?.router.recommend(required(args,3,"demand ID")?)?).map_err(|e|e.to_string())?,
        "routing-configure"=>{
            let config=read_json(required(args,3,"routing config file")?)?;
            store.apply(Request{id:required(args,4,"request ID")?.into(),actor:Actor::Host,action:Action::ConfigureRouter{config}})?
        },
        "routing-allocate"=>{
            let limit=required(args,3,"reservation limit")?.parse::<usize>().map_err(|e|e.to_string())?;
            store.apply(Request{id:required(args,4,"request ID")?.into(),actor:Actor::Host,action:Action::AllocateInference{limit}})?
        },
        "project-host"=>serde_json::json!({"path":store.project_host_agent(required(args,3,"agent ID")?)?}),
        "ingress"=>{
            let authority=persistent_intelligence_runtime::kernel::authority::Authority::load(Path::new(required(args,3,"trusted host authority file")?))?;
            let limit=required(args,4,"request limit")?.parse::<u64>().map_err(|e|e.to_string())?;
            persistent_intelligence_runtime::kernel::ingress::serve(&store,&authority,io::stdin().lock(),io::stdout().lock(),limit)?;return Ok(());
        }
        "local-effect"=>{
            let effect=required(args,3,"effect ID")?;
            let publish=match required(args,4,"publish or reconcile")?{"publish"=>true,"reconcile"=>false,_=>return Err("invalid effect mode".into())};
            store.local_effect(effect,publish)?
        }
        "private-evaluate"=>{
            let private:persistent_intelligence_runtime::kernel::custody::PrivateCohort=read_json(required(args,4,"private cohort file")?)?;
            serde_json::to_value(private.evaluate(&store.inspect()?,required(args,3,"candidate ID")?)?).map_err(|e|e.to_string())?
        }
        "apply-signed"=>{
            let authority=persistent_intelligence_runtime::kernel::authority::Authority::load(Path::new(required(args,3,"host authority file")?))?;
            let signed=read_json(required(args,4,"signed request file or -")?)?;
            let now=u64::try_from(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e|e.to_string())?.as_millis()).map_err(|e|e.to_string())?;
            store.apply_signed(&authority,signed,now)?
        },
        "import-r003"=>store.import_r003(required(args,4,"migration request ID")?.to_owned(),Path::new(required(args,3,"legacy store")?))?,
        "anchor"=>serde_json::to_value(store.anchor()?).map_err(|e|e.to_string())?,
        "verify-anchor"=>{let anchor:Anchor=read_json(required(args,3,"anchor file")?)?;store.verify_anchor(&anchor)?;serde_json::json!({"verified":true})}
        "apply"=>{let request:Request=read_json(required(args,3,"request JSON file or -")?)?;store.apply(request)?}
        "genesis"=>{
            let manifest=read_json(required(args,3,"genesis manifest")?)?;
            store.apply(Request{id:required(args,4,"request ID")?.into(),actor:Actor::Host,action:Action::Genesis{manifest}})?
        }
        "routing-waits"=>serde_json::to_value(store.inspect()?.router.waits).map_err(|e|e.to_string())?,
        "routing-demand-plan"=>store.inspect()?.demand_generation_plan(128)?,
        "routing-demand-configure"=>{
            let policy=read_json(required(args,3,"demand policy file")?)?;
            store.apply(Request{id:required(args,4,"request ID")?.into(),actor:Actor::Host,action:Action::ConfigureDemandPolicy{policy}})?
        },
        "routing-demand-generate"=>{
            let limit=required(args,3,"generation limit")?.parse::<usize>().map_err(|e|e.to_string())?;
            store.apply(Request{id:required(args,4,"request ID")?.into(),actor:Actor::Host,action:Action::GenerateDemands{limit}})?
        },
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
        "serve"=>{
            let workspace=Path::new(required(args,3,"workspace")?);
            let ledger=Path::new(required(args,4,"service ledger")?);
            let config=read_json(required(args,5,"service configuration")?)?;
            persistent_intelligence_runtime::kernel::service::serve(store.root(),workspace,ledger,config)?
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
        _=>return Err("unknown kernel action; inspect|ready|import-r003|anchor|verify-anchor|apply|bootstrap|discover|assimilate|run|project".into()),
    };
    println!("{}",serde_json::to_string_pretty(&output).map_err(|e|e.to_string())?);
    Ok(())
}
fn run()->Result<(),String>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.first().is_some_and(|x|x=="qualify-native"){
        let plan=read_json(required(&args,2,"qualification plan file")?)?;
        let root=std::path::Path::new(required(&args,1,"fresh qualification root")?);
        let report=persistent_intelligence_runtime::kernel::native_qualification::run(root,plan)?;
        println!("{}",serde_json::to_string_pretty(&report).map_err(|e|e.to_string())?);
        return Ok(());
    }
    if args.first().is_some_and(|x|x=="comparative-vault"){
        use persistent_intelligence_runtime::kernel::{comparative::Spec,experiment_vault::{Vault,Limits,WriteRequest}};
        let action=required(&args,1,"vault action")?;
        let root=std::path::Path::new(required(&args,2,"isolated vault root")?);
        let spec:Spec=read_json(required(&args,3,"comparison spec")?)?;
        let run_id=required(&args,4,"planned run ID")?;
        let report=if action=="create"{
            let limits:Limits=read_json(required(&args,5,"vault limits")?)?;
            Vault::create(root,&spec,run_id,limits)?.recover()?
        }else{
            let vault=Vault::open(root,&spec,run_id)?;
            match action{
                "recover"=>vault.recover()?,
                "write"=>{let request:WriteRequest=read_json(required(&args,5,"vault write request")?)?;vault.write(request)?},
                "advance"=>vault.advance(required(&args,5,"expected epoch")?.parse::<u64>().map_err(|e|e.to_string())?)?,
                _=>return Err("vault action must be create, recover, write or advance".into()),
            }
        };
        println!("{}",serde_json::to_string_pretty(&report).map_err(|e|e.to_string())?);
        return Ok(());
    }
    if args.first().is_some_and(|x|x=="comparative-plan"){
        let spec:persistent_intelligence_runtime::kernel::comparative::Spec=read_json(required(&args,1,"comparison specification")?)?;
        println!("{}",serde_json::to_string_pretty(&spec.plan()?).map_err(|e|e.to_string())?);
        return Ok(());
    }
    if args.first().is_some_and(|x|x=="kernel-controller"){
        use persistent_intelligence_runtime::kernel::{persistence::KernelStore,supervisor::{watch_parent,supervise,RunConfig}};
        use std::path::Path;
        let root=required(&args,1,"kernel store")?;let workspace=required(&args,2,"workspace")?;
        let config:RunConfig=read_json(required(&args,3,"controller configuration")?)?;
        watch_parent();let store=KernelStore::open(root)?;
        println!("{}",serde_json::to_string_pretty(&supervise(&store,Path::new(workspace),config)?).map_err(|e|e.to_string())?);return Ok(());
    }
    if args.first().is_some_and(|x|x=="kernel-compute"){
        if args.get(3).is_some_and(|x|x=="--watch-parent"){persistent_intelligence_runtime::kernel::supervisor::watch_parent();}
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
