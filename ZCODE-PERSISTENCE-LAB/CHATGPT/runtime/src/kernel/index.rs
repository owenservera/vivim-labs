//! Immutable, content-addressed sharded entity index. The committed head names
//! the root manifest; unchanged pages are shared between revisions. A direct
//! point lookup reads a manifest and one page, never replays computations.
use super::{model::*,persistence::Receipt};
use crate::{digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,Map};
use std::{collections::{BTreeMap,BTreeSet},fs,path::{Path,PathBuf},io::Write,time::{SystemTime,UNIX_EPOCH}};

fn err(e:impl std::fmt::Display)->String{e.to_string()}
const PAGE_LIMIT:usize=64*1024*1024;
const MAPS:&[&str]=&["goals","tasks","agents","contexts","operators","classes","artifacts","evidence","blobs","checkpoints","messages","capabilities","learning_plans","learning_attempts","knowledge","run_costs","goal_versions","task_versions","effects","receipts","delivery","rollback_history","attempt_history","learning_costs","knowledge_rules","knowledge_publications","cohorts","frozen_candidates","custody_receipts"];
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct IndexRoot{
    pub schema:u32,pub revision:u64,pub state_digest:String,
    pub scalar:String,pub tables:BTreeMap<String,BTreeMap<String,String>>,
}
pub struct Index{root:PathBuf}
impl Index{
    pub fn open(root:&Path)->Result<Self>{
        let root=root.join("entity-index");fs::create_dir_all(root.join("objects")).map_err(err)?;
        Ok(Self{root})
    }
    fn object(&self,hash:&str)->Result<PathBuf>{
        ensure(hash_id(hash),"invalid index object digest")?;
        Ok(self.root.join("objects").join(&hash[..2]).join(format!("{hash}.json")))
    }
    fn sync_dir(path:&Path)->Result<()>{
        #[cfg(unix)] fs::File::open(path).and_then(|f|f.sync_all()).map_err(err)?;
        #[cfg(not(unix))] let _=path;
        Ok(())
    }
    fn put(&self,v:&Value)->Result<String>{
        let bytes=serde_json::to_vec(v).map_err(err)?;
        ensure(bytes.len()<=PAGE_LIMIT,"index page budget exceeded")?;
        let hash=digest(v);let dest=self.object(&hash)?;
        if dest.exists(){let old=self.get(&hash)?;ensure(old==*v,"index object collision")?;return Ok(hash);}
        let parent=dest.parent().ok_or("missing object parent")?;
        fs::create_dir_all(parent).map_err(err)?;
        let stamp=SystemTime::now().duration_since(UNIX_EPOCH).map_err(err)?.as_nanos();
        let tmp=parent.join(format!(".tmp-{}-{stamp}",std::process::id()));
        let mut f=fs::OpenOptions::new().write(true).create_new(true).open(&tmp).map_err(err)?;
        f.write_all(&bytes).map_err(err)?;f.sync_all().map_err(err)?;drop(f);
        fs::rename(tmp,&dest).map_err(err)?;Self::sync_dir(parent)?;
        Ok(hash)
    }
    fn get(&self,hash:&str)->Result<Value>{
        let p=self.object(hash)?;
        ensure(fs::metadata(&p).map_err(err)?.len()<=PAGE_LIMIT as u64,"index object exceeds read budget")?;
        let v:Value=serde_json::from_slice(&fs::read(p).map_err(err)?).map_err(err)?;
        ensure(digest(&v)==hash,"index object digest mismatch")?;Ok(v)
    }
    fn bucket(key:&str)->String{digest(&Value::String(key.into()))[..2].to_owned()}
    pub fn manifest(&self,hash:&str)->Result<IndexRoot>{
        let root:IndexRoot=serde_json::from_value(self.get(hash)?).map_err(err)?;
        ensure(root.schema==1&&hash_id(&root.scalar)&&hash_id(&root.state_digest),"invalid index root")?;
        for (table,buckets) in &root.tables{
            ensure(MAPS.contains(&table.as_str()),"unknown index table")?;
            for (key,hash) in buckets{
                ensure(key.len()==2&&key.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b))&&hash_id(hash),"invalid bucket")?;
            }
        }Ok(root)
    }
    /// Source-of-truth root for a committed revision. Unchanged shard hashes are
    /// reused. Existing immutable objects are read/verified only when necessary
    /// to construct a changed page or resolve a point lookup.
    pub fn write(&self,revision:u64,world:&World,receipts:&BTreeMap<Id,Receipt>,previous:Option<&str>)->Result<String>{
        let mut state=serde_json::to_value(world).map_err(err)?;
        let object=state.as_object_mut().ok_or("world not a record")?;
        object.insert("receipts".into(),serde_json::to_value(receipts).map_err(err)?);
        let state_digest=digest(&state);
        let object=state.as_object_mut().unwrap();
        let old=previous.map(|x|self.manifest(x)).transpose()?;
        let mut tables=BTreeMap::new();
        for &name in MAPS{
            let Some(table)=object.remove(name) else{continue;};
            let table=table.as_object().ok_or("index table must be map")?;
            let mut pages:BTreeMap<String,Map<String,Value>>=BTreeMap::new();
            for (key,val) in table{pages.entry(Self::bucket(key)).or_default().insert(key.clone(),val.clone());}
            let mut refs=BTreeMap::new();
            for (bucket,page) in pages{
                let v=Value::Object(page);let hash=digest(&v);
                let unchanged=old.as_ref().and_then(|o|o.tables.get(name)).and_then(|t|t.get(&bucket)).is_some_and(|x|x==&hash);
                if !unchanged{self.put(&v)?;}else{ensure(self.object(&hash)?.is_file(),"referenced index page missing")?;}
                refs.insert(bucket,hash);
            }
            tables.insert(name.into(),refs);
        }
        let scalar=self.put(&state)?;
        let root=IndexRoot{schema:1,revision,state_digest,scalar,tables};
        self.put(&serde_json::to_value(root).map_err(err)?)
    }
    pub fn load(&self,hash:&str,revision:u64)->Result<(World,BTreeMap<Id,Receipt>)>{
        let root=self.manifest(hash)?;ensure(root.revision==revision,"index revision mismatch")?;
        let mut state=self.get(&root.scalar)?;
        let object=state.as_object_mut().ok_or("scalar state not object")?;
        for (name,refs) in root.tables{
            let mut table=Map::new();
            for (bucket,hash) in refs{
                let page=self.get(&hash)?;
                for (key,val) in page.as_object().ok_or("invalid index page")?{
                    ensure(Self::bucket(key)==bucket,"index key in wrong bucket")?;
                    ensure(table.insert(key.clone(),val.clone()).is_none(),"duplicate index key")?;
                }
            }
            ensure(object.insert(name,Value::Object(table)).is_none(),"duplicate scalar/table")?;
        }
        ensure(digest(&state)==root.state_digest,"index state digest mismatch")?;
        let receipts=state.as_object_mut().unwrap().remove("receipts").ok_or("receipt index missing")?;
        Ok((serde_json::from_value(state).map_err(err)?,serde_json::from_value(receipts).map_err(err)?))
    }
    pub fn lookup(&self,hash:&str,revision:u64,table:&str,key:&str)->Result<Option<Value>>{
        ensure(MAPS.contains(&table),"unknown entity table")?;
        let root=self.manifest(hash)?;ensure(root.revision==revision,"index revision mismatch")?;
        let Some(page)=root.tables.get(table).and_then(|t|t.get(&Self::bucket(key))) else{return Ok(None);};
        let v=self.get(page)?;
        Ok(v.as_object().ok_or("invalid index page")?.get(key).cloned())
    }
    /// Caller holds writer lock and supplies every retained committed root.
    /// A crash can leave unreachable objects, but not delete a reachable one.
    pub fn collect(&self,roots:&BTreeSet<String>)->Result<usize>{
        let mut keep=roots.clone();
        for hash in roots{
            let manifest=self.manifest(hash)?;
            keep.insert(manifest.scalar);
            for page in manifest.tables.values().flat_map(|t|t.values()){keep.insert(page.clone());}
        }
        // Verify reachability before removing anything.
        for hash in &keep{ensure(self.object(hash)?.is_file(),"reachable index object missing")?;}
        let mut removed=0;
        for prefix in fs::read_dir(self.root.join("objects")).map_err(err)?{
            let prefix=prefix.map_err(err)?;
            if !prefix.file_type().map_err(err)?.is_dir(){continue;}
            for file in fs::read_dir(prefix.path()).map_err(err)?{
                let file=file.map_err(err)?;let name=file.file_name().to_string_lossy().into_owned();
                if let Some(hash)=name.strip_suffix(".json"){
                    if hash_id(hash)&&!keep.contains(hash){fs::remove_file(file.path()).map_err(err)?;removed+=1;}
                }else if name.starts_with(".tmp-"){fs::remove_file(file.path()).map_err(err)?;removed+=1;}
            }
            Self::sync_dir(&prefix.path())?;
        }Ok(removed)
    }
}
