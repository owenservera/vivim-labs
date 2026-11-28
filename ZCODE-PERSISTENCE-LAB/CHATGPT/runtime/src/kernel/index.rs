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
const TARGET_PAGE:usize=1024*1024;
const NESTED:&[&str]=&["router.demands","router.reservations","router.observations","router.performance","router.cooldowns","router.provider_cooldowns","router.decisions","router.waits","router.adapters.bindings","router.adapters.handoffs","router.adapters.receipts","router.adapters.quota_receipts","router.adapters.mailbox_takes","router.planning.forecasts","router.planning.pending_critical","router.evolution.trials","router.evolution.promotions","router.config.pools","router.config.profiles","assimilation.policies","assimilation.cohorts","assimilation.candidates","assimilation.verdicts","assimilation.promotions","assimilation.runs","reuse_economics.plans","reuse_economics.reports"];
const MAPS:&[&str]=&["goals","tasks","agents","contexts","operators","classes","artifacts","evidence","blobs","checkpoints","messages","capabilities","learning_plans","learning_attempts","knowledge","run_costs","goal_versions","task_versions","effects","receipts","delivery","rollback_history","attempt_history","learning_costs","knowledge_rules","knowledge_publications","cohorts","frozen_candidates","custody_receipts","recipe_policies","recipes","recipe_scans","resource_receipts","recipe_repairs","host_costs"];
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct IndexRoot{
    pub schema:u32,pub revision:u64,pub state_digest:String,
    pub scalar:String,pub tables:BTreeMap<String,BTreeMap<String,String>>,
    #[serde(default,skip_serializing_if="BTreeMap::is_empty")] pub directories:BTreeMap<String,String>,
}
/// Opaque continuation bound to an immutable index root. An updated store
/// rejects the old cursor rather than silently mixing two revisions.
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScanCursor{pub root:Digest,pub table:String,pub bucket:String,pub after_key:Option<String>}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct ScanPage{pub revision:u64,pub root:Digest,pub entries:Vec<(String,Value)>,pub pages_read:usize,pub next:Option<ScanCursor>}
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
    fn bucket(key:&str,schema:u32)->String{digest(&Value::String(key.into()))[..if schema==1{2}else{4}].to_owned()}
    fn table_parent<'a>(object:&'a mut Map<String,Value>,name:&str)->Option<(&'a mut Map<String,Value>,String)>{
        let mut parts=name.split('.').peekable();let mut parent=object;
        while let Some(part)=parts.next(){if parts.peek().is_none(){return Some((parent,part.into()));}parent=parent.get_mut(part)?.as_object_mut()?;}None
    }
    fn valid_table(table:&str)->bool{MAPS.contains(&table)||NESTED.contains(&table)}
    fn valid_prefix(key:&str)->bool{(2..=64).contains(&key.len())&&key.len()%2==0&&key.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b))}
    fn directory(&self,root:&IndexRoot,table:&str)->Result<BTreeMap<String,String>>{
        if root.schema<3{return Ok(root.tables.get(table).cloned().unwrap_or_default());}
        let Some(hash)=root.directories.get(table) else{return Ok(BTreeMap::new());};
        let refs:BTreeMap<String,String>=serde_json::from_value(self.get(hash)?).map_err(err)?;
        for (prefix,hash) in &refs{
            ensure(Self::valid_prefix(prefix)&&hash_id(hash),"invalid adaptive page locator")?;
            // A directory is a prefix-free partition of the key space.
            for n in (2..prefix.len()).step_by(2){ensure(!refs.contains_key(&prefix[..n]),"overlapping adaptive pages")?;}
        }
        Ok(refs)
    }
    fn leaf(&self,prefix:&str,page:Map<String,Value>,refs:&mut BTreeMap<String,String>,previous:&BTreeMap<String,String>)->Result<()>{
        let v=Value::Object(page);let bytes=serde_json::to_vec(&v).map_err(err)?.len();
        if bytes<=TARGET_PAGE||v.as_object().unwrap().len()<=1||prefix.len()==64{
            ensure(bytes<=PAGE_LIMIT,"single entity exceeds index object budget")?;
            let hash=digest(&v);
            if previous.get(prefix)==Some(&hash){ensure(self.object(&hash)?.is_file(),"unchanged adaptive page missing")?;}else{self.put(&v)?;}
            refs.insert(prefix.into(),hash);return Ok(());
        }
        let mut children:BTreeMap<String,Map<String,Value>>=BTreeMap::new();
        for (key,value) in v.as_object().unwrap(){
            let hash=digest(&Value::String(key.clone()));
            children.entry(hash[..prefix.len()+2].into()).or_default().insert(key.clone(),value.clone());
        }
        for (key,page) in children{self.leaf(&key,page,refs,previous)?;}
        Ok(())
    }
    pub fn manifest(&self,hash:&str)->Result<IndexRoot>{
        let root:IndexRoot=serde_json::from_value(self.get(hash)?).map_err(err)?;
        ensure((1..=4).contains(&root.schema)&&hash_id(&root.scalar)&&hash_id(&root.state_digest),"invalid index root")?;
        if root.schema>=3{ensure(root.tables.is_empty(),"schema 3 cannot use inline tables")?;for (name,hash) in &root.directories{ensure(Self::valid_table(name)&&hash_id(hash),"invalid index directory")?;}}
        else{ensure(root.directories.is_empty(),"legacy index cannot have adaptive directories")?;}
        if root.schema==4{
            ensure(root.state_digest==Self::tree_digest(&root.scalar,&root.directories),"incremental root commitment mismatch")?;
        }
        for (table,buckets) in &root.tables{
            ensure((MAPS.contains(&table.as_str())||root.schema==2&&NESTED.contains(&table.as_str())),"unknown index table")?;
            for (key,hash) in buckets{
                ensure(key.len()==(if root.schema==1{2}else{4})&&key.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b))&&hash_id(hash),"invalid bucket")?;
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
        let mut directories=BTreeMap::new();
        for &name in MAPS.iter().chain(NESTED.iter()){
            let Some((parent,key))=Self::table_parent(object,name) else{continue;};
            let Some(table)=parent.remove(&key) else{continue;};
            let table=table.as_object().ok_or("index table must be map")?;
            let mut pages:BTreeMap<String,Map<String,Value>>=BTreeMap::new();
            for (key,val) in table{pages.entry(Self::bucket(key,1)).or_default().insert(key.clone(),val.clone());}
            let mut refs=BTreeMap::new();
            let prior=match &old{Some(root)=>self.directory(root,name)?,None=>BTreeMap::new()};
            for (bucket,page) in pages{self.leaf(&bucket,page,&mut refs,&prior)?;}
            directories.insert(name.into(),self.put(&serde_json::to_value(refs).map_err(err)?)?);
        }
        let scalar=self.put(&state)?;
        let root=IndexRoot{schema:3,revision,state_digest,scalar,tables:BTreeMap::new(),directories};
        self.put(&serde_json::to_value(root).map_err(err)?)
    }
    fn tree_digest(scalar:&str,dirs:&BTreeMap<String,String>)->Digest{
        digest(&serde_json::json!({"schema":4,"scalar":scalar,"directories":dirs}))
    }
    fn write_table(&self,table:&Map<String,Value>,prior:&BTreeMap<String,String>)->Result<Digest>{
        let mut pages=BTreeMap::<String,Map<String,Value>>::new();
        for (key,val) in table{pages.entry(Self::bucket(key,1)).or_default().insert(key.clone(),val.clone());}
        let mut refs=BTreeMap::new();
        for (bucket,page) in pages{self.leaf(&bucket,page,&mut refs,prior)?;}
        self.put(&serde_json::to_value(refs).map_err(err)?)
    }
    /// Update one receipt in its existing adaptive page, preserving all other
    /// leaf hashes. Directory rewriting is still proportional to page count.
    fn insert_receipt(&self,mut refs:BTreeMap<String,String>,key:&str,receipt:&Receipt)->Result<Digest>{
        let key_hash=digest(&Value::String(key.into()));
        let existing=(2..=64).step_by(2).find_map(|n|refs.get(&key_hash[..n]).map(|h|(key_hash[..n].to_owned(),h.clone())));
        let (prefix,mut page)=if let Some((prefix,h))=existing{
            let v=self.get(&h)?;
            let m=v.as_object().ok_or("invalid receipt page")?.clone();
            for k in m.keys(){ensure(digest(&Value::String(k.clone())).starts_with(&prefix),"receipt in wrong bucket")?;}
            (prefix,m)
        }else{
            // When siblings have split, never create a parent overlapping an
            // existing descendant. Choose the shortest unused non-overlapping prefix.
            let prefix=(2..=64).step_by(2).map(|n|&key_hash[..n]).find(|p|!refs.keys().any(|k|k.starts_with(*p)))
                .ok_or("no receipt partition available")?.to_owned();
            (prefix,Map::new())
        };
        ensure(!page.contains_key(key),"duplicate receipt insertion")?;
        page.insert(key.into(),serde_json::to_value(receipt).map_err(err)?);
        let old=refs.clone();refs.remove(&prefix);
        self.leaf(&prefix,page,&mut refs,&old)?;
        self.put(&serde_json::to_value(refs).map_err(err)?)
    }
    /// World tables still materialize; receipt history does not. Legacy roots
    /// must have been fully loaded and checked before conversion.
    pub fn write_incremental(&self,revision:u64,world:&World,legacy_receipts:&BTreeMap<Id,Receipt>,key:&str,receipt:&Receipt,previous:Option<&str>)->Result<Digest>{
        ensure(receipt.revision==revision&&world.sequence==revision,"incremental revision mismatch")?;
        let old=previous.map(|h|self.manifest(h)).transpose()?;
        let mut state=serde_json::to_value(world).map_err(err)?;
        let object=state.as_object_mut().ok_or("world not record")?;
        let mut directories=BTreeMap::new();
        for &name in MAPS.iter().chain(NESTED.iter()){
            if name=="receipts"{continue;}
            let Some((parent,key))=Self::table_parent(object,name) else{continue;};
            let Some(table)=parent.remove(&key) else{continue;};
            let table=table.as_object().ok_or("index table must be map")?;
            let prior=match &old{Some(r)=>self.directory(r,name)?,None=>BTreeMap::new()};
            directories.insert(name.into(),self.write_table(table,&prior)?);
        }
        let receipt_root=if let Some(r)=old.as_ref().filter(|r|r.schema>=3){
            ensure(r.directories.contains_key("receipts"),"receipt directory missing")?;
            self.insert_receipt(self.directory(r,"receipts")?,key,receipt)?
        }else{
            let mut all=serde_json::to_value(legacy_receipts).map_err(err)?.as_object().ok_or("legacy receipts not map")?.clone();
            ensure(all.insert(key.into(),serde_json::to_value(receipt).map_err(err)?).is_none(),"duplicate legacy receipt")?;
            self.write_table(&all,&BTreeMap::new())?
        };
        directories.insert("receipts".into(),receipt_root);
        let scalar=self.put(&state)?;
        let state_digest=Self::tree_digest(&scalar,&directories);
        self.put(&serde_json::to_value(IndexRoot{schema:4,revision,state_digest,scalar,tables:BTreeMap::new(),directories}).map_err(err)?)
    }
    pub fn load_world(&self,hash:&str,revision:u64)->Result<World>{
        let root=self.manifest(hash)?;
        ensure(root.schema==4,"lazy receipt loading requires schema 4")?;
        Ok(self.load_inner(root,revision,false)?.0)
    }
    pub fn load(&self,hash:&str,revision:u64)->Result<(World,BTreeMap<Id,Receipt>)>{
        let root=self.manifest(hash)?;
        self.load_inner(root,revision,true)
    }
    fn load_inner(&self,root:IndexRoot,revision:u64,include_receipts:bool)->Result<(World,BTreeMap<Id,Receipt>)>{
        ensure(root.revision==revision,"index revision mismatch")?;
        ensure(include_receipts||root.schema==4,"legacy root requires full load")?;
        let mut state=self.get(&root.scalar)?;
        let object=state.as_object_mut().ok_or("scalar state not object")?;
        let schema=root.schema;
        let names:Vec<_>=if schema>=3{root.directories.keys().cloned().collect()}else{root.tables.keys().cloned().collect()};
        for name in names{
            if !include_receipts&&name=="receipts"{continue;}
            let refs=self.directory(&root,&name)?;
            let mut table=Map::new();
            for (bucket,hash) in refs{
                let page=self.get(&hash)?;
                for (key,val) in page.as_object().ok_or("invalid index page")?{
                    ensure(if schema>=3{digest(&Value::String(key.clone())).starts_with(&bucket)}else{Self::bucket(key,schema)==bucket},"index key in wrong bucket")?;
                    ensure(table.insert(key.clone(),val.clone()).is_none(),"duplicate index key")?;
                }
            }
            let (parent,key)=Self::table_parent(object,&name).ok_or("missing nested table parent")?;
            ensure(parent.insert(key,Value::Object(table)).is_none(),"duplicate scalar/table")?;
        }
        if schema<4{ensure(digest(&state)==root.state_digest,"index state digest mismatch")?;}
        let receipts=if include_receipts{
            state.as_object_mut().unwrap().remove("receipts").ok_or("receipt index missing")?
        }else{
            ensure(root.directories.contains_key("receipts")&&!state.as_object().unwrap().contains_key("receipts"),"invalid lazy receipt directory")?;
            serde_json::json!({})
        };
        Ok((serde_json::from_value(state).map_err(err)?,serde_json::from_value(receipts).map_err(err)?))
    }
    pub fn lookup(&self,hash:&str,revision:u64,table:&str,key:&str)->Result<Option<Value>>{
        ensure(MAPS.contains(&table)||NESTED.contains(&table),"unknown entity table")?;
        let root=self.manifest(hash)?;ensure(root.revision==revision,"index revision mismatch")?;
        let refs=self.directory(&root,table)?;
        let key_hash=digest(&Value::String(key.into()));
        let page=if root.schema>=3{(2..=64).step_by(2).find_map(|n|refs.get(&key_hash[..n]))}else{refs.get(&Self::bucket(key,root.schema))};
        let Some(page)=page else{
            if root.schema==1&&NESTED.contains(&table){let scalar=self.get(&root.scalar)?;let mut v=&scalar;for p in table.split('.'){let Some(next)=v.get(p) else{return Ok(None);};v=next;}return Ok(v.get(key).cloned());}
            return Ok(None);
        };
        let v=self.get(page)?;
        Ok(v.as_object().ok_or("invalid index page")?.get(key).cloned())
    }
    /// Bounded hash-bucket order scan; verifies every visited page and its key
    /// placement without materializing the World. This is not a full-store audit.
    pub fn scan(&self,hash:&str,revision:u64,table:&str,cursor:Option<ScanCursor>,limit:usize,max_pages:usize,max_bytes:usize)->Result<ScanPage>{
        ensure(Self::valid_table(table),"unknown entity table")?;
        ensure((1..=1024).contains(&limit)&&(1..=64).contains(&max_pages)&&(1..=PAGE_LIMIT).contains(&max_bytes),"invalid index scan budget")?;
        let root=self.manifest(hash)?;ensure(root.revision==revision,"index revision mismatch")?;
        ensure(!(root.schema==1&&NESTED.contains(&table)),"legacy nested table requires migration before paged scan")?;
        let refs=self.directory(&root,table)?;
        if let Some(c)=&cursor{
            ensure(c.root==hash&&c.table==table&&refs.contains_key(&c.bucket),"stale or invalid scan cursor")?;
        }
        let mut entries=Vec::new();let mut pages_read=0usize;let mut bytes=0usize;
        let mut next=None;
        'pages: for (bucket,page_hash) in &refs{
            if cursor.as_ref().is_some_and(|c|bucket<&c.bucket){continue;}
            let resume=cursor.as_ref().filter(|c|c.bucket.as_str()==bucket.as_str()).and_then(|c|c.after_key.as_deref());
            if pages_read>=max_pages{
                next=Some(ScanCursor{root:hash.into(),table:table.into(),bucket:bucket.clone(),after_key:None});break;
            }
            let page=self.get(page_hash)?;pages_read+=1;
            let map=page.as_object().ok_or("invalid scan page")?;
            if let Some(k)=resume{ensure(map.contains_key(k),"scan cursor key not present")?;}
            let mut last=resume.map(str::to_owned);
            for (key,val) in map{
                ensure(if root.schema>=3{digest(&Value::String(key.clone())).starts_with(bucket)}else{Self::bucket(key,root.schema).as_str()==bucket.as_str()},"scan key in wrong bucket")?;
                if resume.is_some_and(|k|key.as_str()<=k){continue;}
                let size=serde_json::to_vec(&(key,val)).map_err(err)?.len();
                if entries.len()>=limit||bytes.checked_add(size).is_none_or(|n|n>max_bytes){
                    ensure(!entries.is_empty()||size<=max_bytes,"entity exceeds scan response budget; use point lookup or larger budget")?;
                    next=Some(ScanCursor{root:hash.into(),table:table.into(),bucket:bucket.clone(),after_key:last});break 'pages;
                }
                bytes+=size;entries.push((key.clone(),val.clone()));last=Some(key.clone());
            }
        }
        Ok(ScanPage{revision,root:hash.into(),entries,pages_read,next})
    }
    /// Caller holds writer lock and supplies every retained committed root.
    /// A crash can leave unreachable objects, but not delete a reachable one.
    pub fn collect(&self,roots:&BTreeSet<String>)->Result<usize>{
        let mut keep=roots.clone();
        for hash in roots{
            let manifest=self.manifest(hash)?;
            keep.insert(manifest.scalar.clone());
            if manifest.schema>=3{
                for (table,hash) in &manifest.directories{keep.insert(hash.clone());for page in self.directory(&manifest,table)?.values(){keep.insert(page.clone());}}
            }else{for page in manifest.tables.values().flat_map(|t|t.values()){keep.insert(page.clone());}}
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
