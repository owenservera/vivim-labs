//! @hq:context Optional local native Windows ML evaluation, CPU explicitly selected.
//! @hq:invariant No model download, provider fallback, private corpus or execution grant.
//! @hq:invariant Reference graph is fixed and digest-bound; actual runtime results are checked.
use crate::{fixture,resources,hardware,scheduler::{self,Snapshot,Budget,Reserved,Work}};
use windows::{core::{HSTRING,Interface},AI::MachineLearning::{LearningModel,LearningModelDevice,LearningModelDeviceKind,LearningModelSession,LearningModelBinding,TensorFloat},Win32::System::WinRT::{RoInitialize,RoUninitialize,RO_INIT_MULTITHREADED}};
use sha2::{Digest,Sha256};
use std::{path::Path,time::{Instant,Duration},thread,fs::{self,OpenOptions},io::Write};
#[derive(Debug)]pub enum Error{Resource(String),Deferred,Invalid,Runtime{stage:&'static str,code:i32},Mismatch,Io}
fn cv<T>(stage:&'static str,x:windows::core::Result<T>)->Result<T,Error>{x.map_err(|e|Error::Runtime{stage,code:e.code().0})}
struct Apartment;
impl Apartment{fn enter()->Result<Self,Error>{unsafe{cv("RoInitialize",RoInitialize(RO_INIT_MULTITHREADED))?;}Ok(Self)}}
impl Drop for Apartment{fn drop(&mut self){unsafe{RoUninitialize()}}}
#[derive(Debug)]pub struct Evidence{pub digest:String,pub load_ms:u128,pub eval_us:u128,pub reference_us:u128,pub runs:u32,pub scores:[f32;4],pub memory_available:u64,pub logical_cpus:u32,pub cpu_busy_bp:u16}
const MIB:u64=1024*1024;
pub fn evaluate_owned_fixture(dir:&Path)->Result<Evidence,Error>{
 if !dir.is_absolute()||!dir.is_dir(){return Err(Error::Invalid)}
 let top=hardware::topology().map_err(|e|Error::Resource(format!("{e:?}")))?;
 if top.processor_groups!=1{return Err(Error::Deferred)}
 let a=resources::cpu().map_err(|e|Error::Resource(format!("{e:?}")))?;thread::sleep(Duration::from_millis(150));
 let b=resources::cpu().map_err(|e|Error::Resource(format!("{e:?}")))?;
 let busy=resources::cpu_busy_basis_points(a,b).ok_or(Error::Deferred)?;
 let mem=resources::memory().map_err(|e|Error::Resource(format!("{e:?}")))?;
 let p=dir.to_str().ok_or(Error::Invalid)?;let drive=p.strip_prefix(r"\\?\").unwrap_or(p).chars().next().ok_or(Error::Invalid)?;
 let cap=resources::capacity(drive).map_err(|e|Error::Resource(format!("{e:?}")))?;
 let sample=Snapshot{revision:1,observed_ms:0,memory_available:mem.available_bytes,disk_available:cap.available_bytes,logical_cpus:top.active_logical_processors,cpu_busy_bp:Some(busy)};
 let budget=Budget{max_age_ms:1000,memory_reserve:512*MIB,disk_reserve:128*MIB,cpu_reserve:1,max_parallel:1,busy_ceiling_bp:10000};
 let plan=scheduler::plan(sample,budget,Reserved::default(),0,&[Work{id:1,memory:256*MIB,disk:MIB,cpus:1,priority:1,enqueued_ms:0}]).map_err(|e|Error::Resource(format!("{e:?}")))?;
 if plan.admitted!=vec![1]{return Err(Error::Deferred)}
 let file=dir.join("wni-r047-reference.onnx");let mut f=OpenOptions::new().create_new(true).write(true).open(&file).map_err(|_|Error::Io)?;
 struct Cleanup(std::path::PathBuf);impl Drop for Cleanup{fn drop(&mut self){let _=fs::remove_file(&self.0);}}
 let _cleanup=Cleanup(file.clone());f.write_all(fixture::MODEL).map_err(|_|Error::Io)?;drop(f);
 let bytes=fs::read(&file).map_err(|_|Error::Io)?;if bytes!=fixture::MODEL{return Err(Error::Mismatch)}
 let digest=format!("{:x}",Sha256::digest(&bytes));
 let _com=Apartment::enter()?;let start=Instant::now();
 let model=cv("LoadFromFilePath",LearningModel::LoadFromFilePath(&HSTRING::from(file.to_str().ok_or(Error::Invalid)?)))?;
 let device=cv("CpuDevice",LearningModelDevice::Create(LearningModelDeviceKind::Cpu))?;
 let session=cv("CreateSession",LearningModelSession::CreateFromModelOnDevice(&model,&device))?;
 let load_ms=start.elapsed().as_millis();
 let binding=cv("Binding",LearningModelBinding::CreateFromSession(&session))?;
 let input=cv("TensorFloat",TensorFloat::CreateFromShapeArrayAndDataArray(&[4,3],&fixture::FEATURES))?;
 cv("Bind",binding.Bind(&HSTRING::from("X"),&input))?;
 let base_start=Instant::now();let expected=std::hint::black_box(fixture::reference());let reference_us=base_start.elapsed().as_micros();
 let mut scores=[0.;4];let start=Instant::now();let runs=10;
 for i in 0..runs{
  let result=cv("Evaluate",session.Evaluate(&binding,&HSTRING::from(format!("wni-{i}"))))?;
  if !cv("Succeeded",result.Succeeded())?{return Err(Error::Runtime{stage:"EvaluateStatus",code:cv("ErrorStatus",result.ErrorStatus())?})}
  let outputs=cv("Outputs",result.Outputs())?;let y=cv("Lookup",outputs.Lookup(&HSTRING::from("Y")))?;
  let y:TensorFloat=cv("Cast",y.cast())?;let view=cv("Vector",y.GetAsVectorView())?;
  if cv("Size",view.Size())?!=4{return Err(Error::Mismatch)}
  for j in 0..4{scores[j]=cv("GetAt",view.GetAt(j as u32))?;if !scores[j].is_finite()||(scores[j]-expected[j]).abs()>1e-6{return Err(Error::Mismatch)}}
 }
 let eval_us=start.elapsed().as_micros();cv("CloseSession",session.Close())?;
 Ok(Evidence{digest,load_ms,eval_us,reference_us,runs,scores,memory_available:mem.available_bytes,logical_cpus:top.active_logical_processors,cpu_busy_bp:busy})
}
