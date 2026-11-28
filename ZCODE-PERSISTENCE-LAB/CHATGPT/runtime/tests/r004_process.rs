//! Native subprocess qualification source. Never run as part of the coding-only phase.
use persistent_intelligence_runtime::{Expr,Operator,digest,kernel::{model::*,transition::{Action,Actor,Request},persistence::KernelStore,supervisor::RunConfig}};
use std::{collections::{BTreeMap,BTreeSet},fs,path::PathBuf,process::{Child,Command,Stdio},thread,time::{Duration,Instant,SystemTime,UNIX_EPOCH}};
use serde_json::json;
struct Root(PathBuf);
impl Root{fn new()->Self{let p=std::env::temp_dir().join(format!("vivim-r004-process-{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));fs::create_dir(&p).unwrap();Self(p)}}
impl Drop for Root{fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
struct Owned(Child);
impl Drop for Owned{fn drop(&mut self){if self.0.try_wait().ok().flatten().is_none(){let _=self.0.kill();let _=self.0.wait();}}}
#[cfg(windows)]
mod observation {
    use std::{ffi::c_void,mem::size_of,ptr};
    type Handle=*mut c_void;
    #[repr(C)]
    struct Entry{
        size:u32,usage:u32,pid:u32,heap:usize,module:u32,threads:u32,parent:u32,priority:i32,flags:u32,exe:[u16;260],
    }
    #[link(name="kernel32")]
    unsafe extern "system" {
        fn CreateToolhelp32Snapshot(flags:u32,pid:u32)->Handle;
        fn Process32FirstW(snapshot:Handle,entry:*mut Entry)->i32;
        fn Process32NextW(snapshot:Handle,entry:*mut Entry)->i32;
        fn OpenProcess(access:u32,inherit:i32,pid:u32)->Handle;
        fn WaitForSingleObject(handle:Handle,ms:u32)->u32;
        fn GetExitCodeProcess(handle:Handle,code:*mut u32)->i32;
        fn CloseHandle(handle:Handle)->i32;
        fn GetLastError()->u32;
    }
    struct OwnedHandle(Handle);
    impl Drop for OwnedHandle{fn drop(&mut self){unsafe{CloseHandle(self.0);}}}
    pub struct Worker{pub pid:u32,handle:OwnedHandle}
    impl Worker{
        pub fn alive(&self)->bool{match unsafe{WaitForSingleObject(self.handle.0,0)}{258=>true,0=>false,n=>panic!("unexpected process wait {n}")}}
        pub fn wait_exit(&self)->u32{
            assert_eq!(unsafe{WaitForSingleObject(self.handle.0,10_000)},0,"owned worker {} survived controller death",self.pid);
            let mut code=0;assert_ne!(unsafe{GetExitCodeProcess(self.handle.0,&mut code)},0);code
        }
    }
    pub fn children(parent:u32)->Vec<Worker>{
        let snapshot=unsafe{CreateToolhelp32Snapshot(2,0)};
        assert_ne!(snapshot as isize,-1,"process snapshot failed");
        let snapshot=OwnedHandle(snapshot);
        let mut entry:Entry=unsafe{std::mem::zeroed()};
        entry.size=size_of::<Entry>() as u32;
        let mut ok=unsafe{Process32FirstW(snapshot.0,&mut entry)};
        assert_ne!(ok,0,"process enumeration failed");
        let mut workers=vec![];
        while ok!=0{
            if entry.parent==parent{
                let handle=unsafe{OpenProcess(0x0010_0000|0x1000,0,entry.pid)};
                if handle!=ptr::null_mut(){
                    let worker=Worker{pid:entry.pid,handle:OwnedHandle(handle)};
                    if worker.alive(){workers.push(worker);}
                } else {
                    let error=unsafe{GetLastError()};
                    assert_eq!(error,87,"cannot establish child process identity for PID {}",entry.pid);
                }
            }
            ok=unsafe{Process32NextW(snapshot.0,&mut entry)};
        }
        assert_eq!(unsafe{GetLastError()},18,"process enumeration did not reach its end");
        workers
    }
}
fn host(store:&KernelStore,n:&mut u64,action:Action){*n+=1;store.apply(Request{id:format!("seed-{n}"),actor:Actor::Host,action}).unwrap();}
#[test]
#[ignore="explicit real process qualification; do not run until qualification is authorized"]
fn actual_controller_death_restarts_durable_goal(){
    let root=Root::new();let store_path=root.0.join("store");let workspace=root.0.join("workspace");
    fs::create_dir(&workspace).unwrap();let store=KernelStore::open(&store_path).unwrap();
    let mut n=0;
    host(&store,&mut n,Action::Goal{id:"goal".into(),parent:None,objective:"recover verified arithmetic work".into(),claim_budget:128,predicate:None});
    host(&store,&mut n,Action::Slots{count:4});
    let input=Shape::Record{fields:BTreeMap::from([("x".into(),Shape::Integer)])};
    host(&store,&mut n,Action::Operator{contract:OperatorContract{id:"first".into(),version:1,behavior:Operator::Execute,input:input.clone(),output:Shape::Integer,program:Expr::Field{source:Box::new(Expr::Input),key:"x".into()}}});
    host(&store,&mut n,Action::Operator{contract:OperatorContract{id:"increment".into(),version:1,behavior:Operator::Execute,input:Shape::Integer,output:Shape::Integer,program:Expr::Add{left:Box::new(Expr::Input),right:Box::new(Expr::Const{value:json!(1)})}}});
    let mut nodes=vec![Node{id:"first".into(),operator:"first@1".into(),input:Wire::Input}];
    for i in 0..127{let previous=nodes.last().unwrap().id.clone();nodes.push(Node{id:format!("n-{i}"),operator:"increment@1".into(),input:Wire::Node{id:previous}});}
    let result=Wire::Node{id:nodes.last().unwrap().id.clone()};
    host(&store,&mut n,Action::Class{class:Class{id:"long".into(),version:1,input,output:Shape::Integer,nodes,result,lineage:vec![]}});
    for i in 0..12{
        host(&store,&mut n,Action::Task{spec:TaskSpec{id:format!("task-{i}"),goal:"goal".into(),objective:"increment 127 times".into(),dependencies:BTreeSet::new(),inputs:BTreeMap::from([("x".into(),Port{shape:Shape::Integer,binding:Binding::Literal{value:json!(i)}})]),output:Shape::Integer,oracle:Oracle::Digest{expected:digest(&json!(i+127))},class:Some("long@1".into()),family:None,context:None,max_attempts:4,priority:0,uncertainty:0,information_gain:0,expected_cost:1}});
    }
    host(&store,&mut n,Action::Seal{goal:"goal".into()});
    let config=RunConfig{duration_ms:120_000,lease_ms:30_000,poll_ms:10,max_jobs:64,max_learning_plans:0,stop_when_idle:true};
    let file=root.0.join("config.json");fs::write(&file,serde_json::to_vec(&config).unwrap()).unwrap();
    let exe=env!("CARGO_BIN_EXE_persistent-intelligence-runtime");
    let launch=||Owned(Command::new(exe).arg("kernel-controller").arg(&store_path).arg(&workspace).arg(&file).stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap());
    let mut first=launch();let timer=Instant::now();
    let killed=loop{
        let state=store.inspect().unwrap();
        if let Some(t)=state.tasks.values().find(|t|t.phase==TaskPhase::Running&&t.lease.as_ref().is_some_and(|l|state.agents.get(&l.worker).and_then(|a|a.incarnations.last()).is_some_and(|i|i.process_ref.is_some()))){
            break (t.spec.id.clone(),t.lease.clone().unwrap());
        }
        assert!(first.0.try_wait().unwrap().is_none(),"controller finished before crash injection");
        assert!(timer.elapsed()<Duration::from_secs(45),"no observable live worker");
        thread::sleep(Duration::from_millis(5));
    };
    #[cfg(windows)]
    let observed={
        let pid=store.inspect().unwrap().agents[&killed.1.worker].incarnations.iter()
            .find(|i|i.number==killed.1.incarnation).unwrap().process_ref.as_ref().unwrap()
            .strip_prefix("native-pid-").unwrap().parse::<u32>().unwrap();
        let workers=observation::children(first.0.id());
        assert!(workers.iter().any(|w|w.pid==pid&&w.alive()),"durable lease alone does not establish an active OS worker");
        workers
    };
    first.0.kill().unwrap();first.0.wait().unwrap();
    #[cfg(windows)]
    {
        let exits:Vec<_>=observed.iter().map(|w|(w.pid,w.wait_exit())).collect();
        assert!(exits.iter().any(|(_,code)|*code==75),"no observed worker exited via the parent-death watchdog: {exits:?}");
        assert!(observation::children(first.0.id()).is_empty(),"controller left a live child");
        eprintln!("controller={} independently observed worker exits={exits:?}",first.0.id());
    }
    let mut second=launch();let timer=Instant::now();
    loop{
        if let Some(status)=second.0.try_wait().unwrap(){assert!(status.success(),"replacement controller failed");break;}
        assert!(timer.elapsed()<Duration::from_secs(125),"replacement timed out");
        thread::sleep(Duration::from_millis(20));
    }
    let state=store.inspect().unwrap();
    assert_eq!(state.goals["goal"].phase,GoalPhase::Complete);
    assert_eq!(state.artifacts.len(),12);assert_eq!(state.running(),0);
    assert!(state.tasks[&killed.0].generation>killed.1.generation,"crashed attempt was not replaced");
    assert!(state.agents[&killed.1.worker].incarnations.iter().find(|i|i.number==killed.1.incarnation).unwrap().ended_ms.is_some());
    for i in 0..12{assert_eq!(*state.artifact_value(&format!("task-{i}")).unwrap(),json!(i+127));}
}
