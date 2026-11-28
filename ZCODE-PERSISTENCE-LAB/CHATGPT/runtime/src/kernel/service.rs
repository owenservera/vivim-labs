//! Bounded outer supervisor. Owns exactly one controller subprocess at a time.
//! Durable launch reservations are charged before spawn, including uncertain exits.
use super::{model::*,supervisor::RunConfig,persistence::KernelStore};
use crate::{digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::{fs::{self,File,OpenOptions},io::Write,path::{Path,PathBuf},process::{Child,Command,Stdio},thread,time::{Duration,Instant,SystemTime,UNIX_EPOCH}};
fn err(e:impl std::fmt::Display)->String{e.to_string()}
fn now()->Result<u64>{u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH).map_err(err)?.as_millis()).map_err(err)}
fn write(path:&Path,v:&Value)->Result<()>{
    let tmp=path.with_extension("tmp");
    if tmp.exists(){fs::remove_file(&tmp).map_err(err)?;}
    let mut f=OpenOptions::new().create_new(true).write(true).open(&tmp).map_err(err)?;
    f.write_all(&serde_json::to_vec(v).map_err(err)?).map_err(err)?;f.sync_all().map_err(err)?;drop(f);
    ensure(!path.exists(),"service receipt already exists")?;
    fs::rename(tmp,path).map_err(err)?;
    #[cfg(unix)] File::open(path.parent().ok_or("missing receipt parent")?).and_then(|f|f.sync_all()).map_err(err)?;
    Ok(())
}
fn read(path:&Path)->Result<Value>{serde_json::from_slice(&fs::read(path).map_err(err)?).map_err(err)}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct ServiceConfig{
    pub duration_ms:u64,pub max_starts:u32,pub max_reserved_jobs:u64,
    pub max_reserved_learning:u64,pub backoff_ms:u64,pub poll_ms:u64,
    pub controller:RunConfig,
}
impl ServiceConfig{
    pub fn validate(&self)->Result<()>{
        self.controller.validate()?;
        ensure((100..=259_200_000).contains(&self.duration_ms)&&(1..=1024).contains(&self.max_starts)&&self.max_reserved_jobs>0&&self.backoff_ms<=60_000&&(10..=1000).contains(&self.poll_ms),"invalid outer service budget")
    }
}
struct Owned(Child);
impl Drop for Owned{fn drop(&mut self){if self.0.try_wait().ok().flatten().is_none(){let _=self.0.kill();let _=self.0.wait();}}}
/// Controller process success is transport evidence, not goal acceptance.
fn goal_outcome<'a>(phases:impl Iterator<Item=&'a GoalPhase>)-> &'static str {
    let mut any=false;let mut open=false;let mut failed=false;
    for phase in phases {
        any=true;
        match phase {
            GoalPhase::Open=>open=true,
            GoalPhase::Complete=>{},
            GoalPhase::Exhausted|GoalPhase::Blocked|GoalPhase::Cancelled=>failed=true,
        }
    }
    if !any {"no_goals"} else if open {"goals_pending"} else if failed {"goals_terminal_unsuccessful"} else {"goals_complete"}
}
fn durable_outcome(store:&Path)->Result<&'static str>{
    let state=KernelStore::open(store)?.inspect()?;
    Ok(goal_outcome(state.goals.values().map(|g|&g.phase)))
}
pub fn serve(store:&Path,workspace:&Path,ledger:&Path,config:ServiceConfig)->Result<Value>{
    config.validate()?;
    fs::create_dir_all(store).map_err(err)?;fs::create_dir_all(workspace).map_err(err)?;fs::create_dir_all(ledger).map_err(err)?;
    let store=store.canonicalize().map_err(err)?;let workspace=workspace.canonicalize().map_err(err)?;let ledger=ledger.canonicalize().map_err(err)?;
    ensure(!ledger.starts_with(&workspace)&&!workspace.starts_with(&ledger)&&!workspace.starts_with(&store)&&!store.starts_with(&workspace),"service workspace must be separate from host state")?;
    let _lock=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(ledger.join("service.lock")).map_err(err)?;
    _lock.try_lock().map_err(|e|format!("service already owned: {e:?}"))?;
    let pin=digest(&serde_json::to_value(&config).map_err(err)?);
    let manifest=ledger.join("manifest.json");
    if !manifest.exists(){write(&manifest,&json!({"schema":1,"config":pin,"start_ms":now()?,"store":store,"workspace":workspace}))?;}
    let m=read(&manifest)?;
    ensure(m["config"]==pin&&m["store"]==json!(store)&&m["workspace"]==json!(workspace),"service identity mismatch")?;
    let epoch=m["start_ms"].as_u64().ok_or("missing service epoch")?;
    let wall=now()?;ensure(wall>=epoch,"clock reversed across service restart; operator review required")?;
    let start=Instant::now();
    let mut starts=0u32;let mut jobs=0u64;let mut learning=0u64;
    // Reservations are immutable and survive a crash before spawn or receipt.
    loop{
        let path=ledger.join(format!("start-{:04}.json",starts+1));
        if !path.exists(){break;}
        let v=read(&path)?;
        ensure(v["ordinal"].as_u64()==Some(u64::from(starts+1))&&v["config"]==pin,"service reservation mismatch")?;
        jobs=jobs.checked_add(v["jobs"].as_u64().ok_or("missing reservation jobs")?).ok_or("job budget overflow")?;
        learning=learning.checked_add(v["learning"].as_u64().ok_or("missing learning reservation")?).ok_or("learning budget overflow")?;
        starts+=1;
        if !ledger.join(format!("exit-{starts:04}.json")).exists(){
            write(&ledger.join(format!("exit-{starts:04}.json")),&json!({"ordinal":starts,"outcome":"unknown_after_service_restart","charged":true}))?;
        }else{let receipt=read(&ledger.join(format!("exit-{starts:04}.json")))?;ensure(receipt["ordinal"].as_u64()==Some(u64::from(starts)),"service exit ordinal mismatch")?;}
    }
    ensure(starts<=config.max_starts&&jobs<=config.max_reserved_jobs&&learning<=config.max_reserved_learning,"service budget exceeded")?;
    let elapsed=||wall.saturating_sub(epoch).saturating_add(start.elapsed().as_millis().min(u128::from(u64::MAX)) as u64);
    let stop=||ledger.join("STOP").exists()||workspace.join("STOP").exists();
    let exe=std::env::current_exe().map_err(err)?;
    let mut disposition=durable_outcome(&store)?;let mut reason=if disposition=="goals_pending"{"budget"}else{disposition};
    while disposition=="goals_pending"&&starts<config.max_starts&&jobs<config.max_reserved_jobs&&elapsed()<config.duration_ms&&!stop(){
        let mut run=config.controller.clone();
        run.duration_ms=run.duration_ms.min(config.duration_ms.saturating_sub(elapsed()));
        run.max_jobs=run.max_jobs.min(config.max_reserved_jobs-jobs);
        run.max_learning_plans=run.max_learning_plans.min(config.max_reserved_learning-learning);
        if run.duration_ms==0||run.max_jobs==0{break;}
        starts+=1;jobs+=run.max_jobs;learning+=run.max_learning_plans;
        let runfile=ledger.join(format!("config-{starts:04}.json"));
        write(&ledger.join(format!("start-{starts:04}.json")),&json!({"ordinal":starts,"config":pin,"jobs":run.max_jobs,"learning":run.max_learning_plans,"started_ms":now()?}))?;
        write(&runfile,&serde_json::to_value(&run).map_err(err)?)?;
        let output=OpenOptions::new().create_new(true).write(true).open(ledger.join(format!("stdout-{starts:04}.json"))).map_err(err)?;
        let errors=OpenOptions::new().create_new(true).write(true).open(ledger.join(format!("stderr-{starts:04}.log"))).map_err(err)?;
        let child=Command::new(&exe).arg("kernel-controller").arg(&store).arg(&workspace).arg(&runfile)
            .stdin(Stdio::piped()).stdout(Stdio::from(output)).stderr(Stdio::from(errors)).spawn();
        let mut owned=match child{Ok(c)=>Owned(c),Err(e)=>{
            write(&ledger.join(format!("exit-{starts:04}.json")),&json!({"ordinal":starts,"outcome":"spawn_failed","detail":e.to_string()}))?;
            thread::sleep(Duration::from_millis(config.backoff_ms.min(config.duration_ms.saturating_sub(elapsed()))));continue;
        }};
        let outcome=loop{
            if let Some(status)=owned.0.try_wait().map_err(err)?{break json!({"outcome":"exit","success":status.success(),"code":status.code()});}
            if stop()||elapsed()>=config.duration_ms{
                let _=owned.0.kill();let status=owned.0.wait().map_err(err)?;
                break json!({"outcome":"stopped","code":status.code()});
            }
            thread::sleep(Duration::from_millis(config.poll_ms));
        };
        disposition=durable_outcome(&store)?;
        write(&ledger.join(format!("exit-{starts:04}.json")),&json!({"ordinal":starts,"result":outcome,"goal_disposition":disposition,"elapsed_ms":elapsed()}))?;
        if stop(){reason="operator_stop";break;}
        if disposition!="goals_pending"{reason=disposition;break;}
        thread::sleep(Duration::from_millis(config.backoff_ms.min(config.duration_ms.saturating_sub(elapsed()))));
    }
    if stop(){reason="operator_stop";}else if disposition=="goals_pending"&&elapsed()>=config.duration_ms{reason="duration";}
    Ok(json!({"reason":reason,"goal_disposition":disposition,"reserved_starts":starts,"reserved_jobs":jobs,"reserved_learning":learning,"elapsed_ms":elapsed(),"config_digest":pin}))
}

#[cfg(test)]
mod outcome_tests {
    use super::*;
    #[test]
    fn controller_exit_is_not_goal_acceptance(){
        assert_eq!(goal_outcome([].iter()),"no_goals");
        assert_eq!(goal_outcome([GoalPhase::Open].iter()),"goals_pending");
        assert_eq!(goal_outcome([GoalPhase::Complete,GoalPhase::Blocked].iter()),"goals_terminal_unsuccessful");
        assert_eq!(goal_outcome([GoalPhase::Cancelled,GoalPhase::Open].iter()),"goals_pending");
        assert_eq!(goal_outcome([GoalPhase::Complete,GoalPhase::Complete].iter()),"goals_complete");
        assert_eq!(goal_outcome([GoalPhase::Exhausted].iter()),"goals_terminal_unsuccessful");
    }
}