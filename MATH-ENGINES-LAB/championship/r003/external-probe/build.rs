//! Independently staged native source. Compile only in reviewed, isolated
//! evaluation infrastructure, never automatically on receipt of a team.
use std::{env,fs,path::PathBuf};
fn main(){
    println!("cargo:rerun-if-env-changed=VIVIM_EXTERNAL_SEED");
    let source=PathBuf::from(env::var_os("VIVIM_EXTERNAL_SEED").expect("absolute path to reviewed native/seed.S"));
    assert!(source.is_absolute(),"absolute seed path required");
    println!("cargo:rerun-if-changed={}",source.display());
    let data=fs::read(&source).expect("read native seed");
    assert!(data.len()<=8*1024*1024&&!data.is_empty(),"seed size");
    std::str::from_utf8(&data).expect("UTF-8 seed");
    fs::write(PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("seed.S"),data).unwrap();
}
