mod fixture;mod resources;mod hardware;mod scheduler;mod winml;
fn main(){let cwd=std::env::current_dir().unwrap();match winml::evaluate_owned_fixture(&cwd){Ok(e)=>println!("WINML_CPU_REFERENCE_OK {e:?}"),Err(e)=>{eprintln!("WINML_CPU_REFERENCE_UNAVAILABLE {e:?}");std::process::exit(2)}}}
