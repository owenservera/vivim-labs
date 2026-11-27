//! Architecture-neutral R003-V1 package intake. No builds or entrant execution.
//! Validate extracted, evaluator-controlled directories before review/isolation.
//! Structural compatibility is not evidence of mathematical qualification.
use serde_json::Value;
use sha2::{Digest,Sha256};
use std::{collections::BTreeSet,fs,path::{Path,PathBuf}};
type Result<T>=std::result::Result<T,String>;
fn err(s:impl Into<String>)->String{s.into()}
fn digest(bytes:&[u8])->String{Sha256::digest(bytes).iter().map(|b|format!("{b:02x}")).collect()}
fn safe_rel(s:&str)->bool{
    !s.is_empty()&&s.split('/').all(|p|!p.is_empty()&&p!="."&&p!=".."&&
        !p.ends_with('.')&&p.bytes().all(|b|b.is_ascii_alphanumeric()||matches!(b,b'_'|b'-'|b'.'))&&
        !matches!(p.split('.').next().unwrap_or("").to_ascii_uppercase().as_str(),"CON"|"PRN"|"AUX"|"NUL"|"COM1"|"COM2"|"COM3"|"COM4"|"COM5"|"COM6"|"COM7"|"COM8"|"COM9"|"LPT1"|"LPT2"|"LPT3"|"LPT4"|"LPT5"|"LPT6"|"LPT7"|"LPT8"|"LPT9"))
}
fn safe_meta(p:&Path)->Result<fs::Metadata>{
    let m=fs::symlink_metadata(p).map_err(|e|format!("{}: {e}",p.display()))?;
    if m.file_type().is_symlink(){return Err(err("symlink not permitted"));}
    #[cfg(windows)]{
        use std::os::windows::fs::MetadataExt;
        if m.file_attributes()&0x400!=0{return Err(err("reparse point not permitted"));}
    }
    if !m.is_dir()&&!m.is_file(){return Err(err("special file not permitted"));}
    Ok(m)
}
fn locate(root:&Path,rel:&str)->Result<PathBuf>{
    if !safe_rel(rel){return Err(format!("unsafe relative path: {rel}"));}
    let mut p=root.to_path_buf();safe_meta(&p)?;
    for part in rel.split('/'){p.push(part);safe_meta(&p)?;}
    Ok(p)
}
fn file(root:&Path,rel:&str)->Result<PathBuf>{let p=locate(root,rel)?;if !safe_meta(&p)?.is_file(){return Err(format!("not a file: {rel}"));}Ok(p)}
fn directory(root:&Path,rel:&str)->Result<PathBuf>{let p=locate(root,rel)?;if !safe_meta(&p)?.is_dir(){return Err(format!("not a directory: {rel}"));}Ok(p)}
fn json(root:&Path,rel:&str)->Result<Value>{
    let p=file(root,rel)?;if safe_meta(&p)?.len()>8*1024*1024{return Err(err("JSON exceeds 8 MiB intake limit"));}
    serde_json::from_slice(&fs::read(&p).map_err(|e|e.to_string())?).map_err(|e|format!("{rel}: {e}"))
}
fn field<'a>(v:&'a Value,k:&str)->Result<&'a str>{v[k].as_str().filter(|s|!s.trim().is_empty()).ok_or_else(||format!("missing/empty string {k}"))}
fn exact(v:&Value,k:&str,want:&str)->Result<()>{if field(v,k)?==want{Ok(())}else{Err(format!("{k}: expected {want}"))}}
fn team_id(s:&str)->bool{(3..=32).contains(&s.len())&&s.as_bytes()[0].is_ascii_lowercase()&&s.bytes().all(|b|b.is_ascii_lowercase()||b.is_ascii_digit()||b==b'-')}
fn is_digest(s:&str)->bool{s.len()==64&&s.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b))}
fn check_tree(root:&Path,depth:usize,count:&mut usize)->Result<()>{
    if depth>64{return Err(err("package directory depth exceeds 64"));}
    let mut names=BTreeSet::new();
    for entry in fs::read_dir(root).map_err(|e|e.to_string())?{
        let e=entry.map_err(|e|e.to_string())?;*count+=1;
        if *count>100_000{return Err(err("package entry limit exceeded"));}
        let name=e.file_name().into_string().map_err(|_|err("non-UTF8 filename"))?;
        if !safe_rel(&name)||name.contains('/')||!names.insert(name.to_ascii_lowercase()){return Err(format!("unsafe or colliding filename: {name}"));}
        let m=safe_meta(&e.path())?;
        if m.is_dir(){check_tree(&e.path(),depth+1,count)?;}
        else if [".zip",".7z",".rar",".tar",".tgz",".gz"].iter().any(|s|name.to_ascii_lowercase().ends_with(s)){return Err(format!("nested archive: {name}"));}
    }Ok(())
}
fn evidence_hash(p:&Path)->Result<String>{
    use std::io::Read;
    let mut f=fs::File::open(p).map_err(|e|e.to_string())?;let mut h=Sha256::new();let mut b=[0u8;65536];
    loop{let n=f.read(&mut b).map_err(|e|e.to_string())?;if n==0{break;}h.update(&b[..n]);}
    Ok(h.finalize().iter().map(|b|format!("{b:02x}")).collect())
}
pub fn validate(root:&Path)->Result<Value>{
    if !safe_meta(root)?.is_dir(){return Err(err("package root is not a directory"));}
    let mut count=0;check_tree(root,0,&mut count)?;
    let t=json(root,"team.json")?;exact(&t,"schema","vivim.math.team/1")?;exact(&t,"protocol","R003-V1")?;
    let id=field(&t,"team_id")?;if !team_id(id){return Err(err("invalid team_id"));}
    file(root,"README.md")?;
    let entries=directory(root,"entrants")?;
    let expected:Vec<_>=(1..=32).map(|n|format!("E{n:02}")).collect();
    let array=t["entrants"].as_array().ok_or("missing entrants array")?;
    if array.len()!=32||array.iter().zip(&expected).any(|(v,e)|v.as_str()!=Some(e.as_str())){return Err(err("entrants must be E01 through E32 in order"));}
    let actual:BTreeSet<_>=fs::read_dir(&entries).map_err(|e|e.to_string())?.map(|e|e.map(|v|v.file_name().to_string_lossy().into_owned())).collect::<std::io::Result<_>>().map_err(|e|e.to_string())?;
    if actual!=expected.iter().cloned().collect(){return Err(err("entrant directory set mismatch"));}
    let mut names=BTreeSet::new();let mut report=Vec::new();
    for entrant in expected{
        let base=directory(&entries,&entrant)?;let m=json(&base,"manifest.json")?;
        for (k,v) in [("schema","vivim.math.entrant/1"),("team_id",id),("entrant_id",entrant.as_str()),("seed","native/seed.S"),("seed_symbol","vivim_seed_v1"),("abi","win64-i64-v1"),("target","x86_64-pc-windows-msvc"),("runner_manifest","runner/Cargo.toml"),("runner_bin","vivim-entrant"),("evidence_index","evidence/index.json")]{exact(&m,k,v)?;}
        let name=field(&m,"name")?;field(&m,"architecture_summary")?;
        if !names.insert(name.trim().to_lowercase()){return Err(format!("duplicate entrant name: {name}"));}
        let limits=m["limitations"].as_array().ok_or("limitations must be array")?;
        if limits.iter().any(|v|v.as_str().is_none_or(|s|s.trim().is_empty())){return Err(err("invalid limitations entry"));}
        for rel in ["README.md","native/seed.S","runner/Cargo.toml","runner/Cargo.lock","evidence/index.json"]{file(&base,rel)?;}
        for rel in ["runner/src","obligations","evidence","artifacts"]{directory(&base,rel)?;}
        let seed_path=file(&base,"native/seed.S")?;
        if safe_meta(&seed_path)?.len()==0||safe_meta(&seed_path)?.len()>8*1024*1024{return Err(err("seed source empty or exceeds intake limit"));}
        let seed=fs::read_to_string(&seed_path).map_err(|e|e.to_string())?;
        if !seed.lines().any(|l|l.trim()=="vivim_seed_v1:"){return Err(format!("{entrant}: mandatory seed label absent"));}
        let obligations=m["obligations"].as_object().ok_or("missing obligations")?;
        if obligations.len()!=16{return Err(err("expected 16 obligations"));}
        let mut required=BTreeSet::new();
        for n in 1..=16{
            let key=format!("D{n:02}");file(&base,&format!("obligations/{key}.md"))?;
            let status=obligations.get(&key).and_then(Value::as_str).ok_or("missing obligation status")?;
            if !matches!(status,"OPEN"|"IMPLEMENTED"|"DEV_PASS"|"DEV_FAIL"){return Err(format!("invalid self-assessed status: {status}"));}
            if status!="OPEN"{required.insert(key);}
        }
        let idx=json(&base,"evidence/index.json")?;exact(&idx,"schema","vivim.math.evidence-index/1")?;
        let evidence=idx["entries"].as_array().ok_or("missing evidence entries")?;
        for e in evidence{
            let obligation=field(e,"obligation")?;if !obligations.contains_key(obligation){return Err(err("unknown evidence obligation"));}
            let path=field(e,"path")?;let expected_hash=field(e,"sha256")?;let kind=field(e,"kind")?;field(e,"description")?;
            if !is_digest(expected_hash)||!matches!(kind,"source"|"test"|"artifact"|"measurement"|"failure"|"report"){return Err(err("invalid evidence digest or kind"));}
            if evidence_hash(&file(&base,path)?)?!=expected_hash{return Err(format!("{entrant}: evidence digest mismatch: {path}"));}
            required.remove(obligation);
        }
        if !required.is_empty(){return Err(format!("{entrant}: non-OPEN obligations lack evidence: {required:?}"));}
        report.push(serde_json::json!({"entrant_key":format!("{id}/{entrant}"),"entrant_id":entrant,"name":name,"seed_sha256":digest(seed.as_bytes()),"manifest_sha256":evidence_hash(&file(&base,"manifest.json")?)?,"status":"STRUCTURAL_PASS_UNQUALIFIED"}));
    }
    Ok(serde_json::json!({"schema":"vivim.math.intake-report/1","team_id":id,"count":report.len(),"qualified":0,"status":"STRUCTURAL_PASS_UNQUALIFIED","entrants":report}))
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn rejects_paths_and_identity(){for bad in ["../x","C:/x","a\\b","a//b","a/./b","NUL.txt","a:stream","a/..","x."]{assert!(!safe_rel(bad),"{bad}");}assert!(safe_rel("native/seed.S"));assert!(team_id("example-team"));assert!(!team_id("Evil"));assert!(!team_id("a"));assert!(is_digest(&"a".repeat(64)));}
    // Structural fixture ONLY; no candidate source is executed and no
    // mathematical or constitutional qualification is asserted.
    struct Temp(PathBuf);
    impl Drop for Temp{fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
    fn fixture()->Temp{
        let n=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let root=std::env::temp_dir().join(format!("vivim-intake-fixture-{}-{n}",std::process::id()));
        fs::create_dir(&root).unwrap();let t=Temp(root);
        let ids:Vec<_>=(1..=32).map(|n|format!("E{n:02}")).collect();
        fs::write(t.0.join("team.json"),serde_json::to_vec(&serde_json::json!({"schema":"vivim.math.team/1","team_id":"test-team","protocol":"R003-V1","entrants":ids})).unwrap()).unwrap();
        fs::write(t.0.join("README.md"),"structural fixture").unwrap();
        for id in ids{
            let base=t.0.join("entrants").join(&id);
            for d in ["native","runner/src","obligations","evidence","artifacts"]{fs::create_dir_all(base.join(d)).unwrap();}
            for f in ["README.md","runner/Cargo.toml","runner/Cargo.lock"]{fs::write(base.join(f),"fixture only").unwrap();}
            fs::write(base.join("native/seed.S"),".globl vivim_seed_v1\nvivim_seed_v1:\n").unwrap();
            let mut obligations=serde_json::Map::new();
            for n in 1..=16{let k=format!("D{n:02}");fs::write(base.join(format!("obligations/{k}.md")),"OPEN").unwrap();obligations.insert(k,Value::String("OPEN".into()));}
            let m=serde_json::json!({"schema":"vivim.math.entrant/1","team_id":"test-team","entrant_id":id,"name":format!("Fixture {id}"),"architecture_summary":"structural fixture, not architecture","seed":"native/seed.S","seed_symbol":"vivim_seed_v1","abi":"win64-i64-v1","target":"x86_64-pc-windows-msvc","runner_manifest":"runner/Cargo.toml","runner_bin":"vivim-entrant","obligations":obligations,"evidence_index":"evidence/index.json","limitations":["No executable implementation"]});
            fs::write(base.join("manifest.json"),serde_json::to_vec(&m).unwrap()).unwrap();
            fs::write(base.join("evidence/index.json"),r#"{"schema":"vivim.math.evidence-index/1","entries":[]}"#).unwrap();
        }t
    }
    #[test]fn accepts_only_structure_then_rejects_bad_evidence(){
        let t=fixture();let r=validate(&t.0).unwrap();assert_eq!(r["count"],32);assert_eq!(r["qualified"],0);
        let base=t.0.join("entrants/E01");
        let mut m=json(&base,"manifest.json").unwrap();m["obligations"]["D01"]=Value::String("IMPLEMENTED".into());
        fs::write(base.join("manifest.json"),serde_json::to_vec(&m).unwrap()).unwrap();
        assert!(validate(&t.0).unwrap_err().contains("lack evidence"));
        let path="native/seed.S";let hash=evidence_hash(&base.join(path)).unwrap();
        let idx=serde_json::json!({"schema":"vivim.math.evidence-index/1","entries":[{"obligation":"D01","path":path,"sha256":hash,"kind":"source","description":"fixture only"}]});
        fs::write(base.join("evidence/index.json"),serde_json::to_vec(&idx).unwrap()).unwrap();
        assert_eq!(validate(&t.0).unwrap()["qualified"],0);
        fs::write(base.join(path),".globl vivim_seed_v1\nvivim_seed_v1:\n# changed\n").unwrap();
        assert!(validate(&t.0).unwrap_err().contains("digest mismatch"));
    }

}
