//! @hq:context Resident pure evaluation, typed composition and pull-based semantic observations.
//! @hq:why Preserve reusable work and evolve higher-level behaviors without recompiling the kernel.
//! @hq:invariant The existing protocol Runtime owns generation and dependency invalidation.
//! @hq:see docs/META-VLANG-TYPED-KERNEL-R001.md
use std::collections::{BTreeMap,BTreeSet,VecDeque};
use crate::{meta_language::{Diagnostic,Evaluation,Function,Program,Type,Value},protocol::{Definition,Delta,Error,Runtime,Snapshot}};
const CACHE_LIMIT:usize=64;
const WATCH_LIMIT:usize=32;
const FUEL:usize=10_000;
const COMPOSITION_LIMIT:usize=16;

/// @hq:context Effective runtime configuration is computed from versioned pure definitions.
/// @hq:invariant Configuration can tighten resource limits but cannot enlarge the trusted kernel ceiling.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Config{pub cache_limit:usize,pub watch_limit:usize,pub fuel:usize,pub composition_limit:usize}
impl Config{
    fn read(p:&Program)->Result<Self,Error>{
        fn setting(p:&Program,id:&str,ceiling:usize)->Result<usize,Error>{
            let fail=|code|Error::InvalidTypedDefinition(diagnostic(id,code));
            if p.get(id).is_none(){return Ok(ceiling);}
            let v=p.evaluate(id,vec![],FUEL).map_err(Error::InvalidTypedDefinition)?.value;
            match v{Value::Int(n) if n>0 && n as u64<=ceiling as u64=>Ok(n as usize),_=>Err(fail("CONFIG_BOUND"))}
        }
        Ok(Self{cache_limit:setting(p,"sdk.runtime.cache_limit",CACHE_LIMIT)?,watch_limit:setting(p,"sdk.runtime.watch_limit",WATCH_LIMIT)?,fuel:setting(p,"sdk.runtime.fuel",FUEL)?,composition_limit:setting(p,"sdk.runtime.composition_limit",COMPOSITION_LIMIT)?})
    }
}

#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord)]
struct Key { id:String,args:Vec<Value> }
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Query { pub generation:u64,pub cache_hit:bool,pub evaluation:Evaluation }
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Rank{pub generation:u64,pub required:bool,pub priority:i64}
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Observation {
    pub generation:u64,pub semantic_revision:u64,pub changed:bool,
    pub value:Result<Value,Diagnostic>,
}
#[derive(Clone)]
struct Watch { key:Key,revision:u64,last:Result<Value,Diagnostic> }
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Composition {
    pub base_generation:u64,pub definition:Definition,pub delta:Option<Delta>,
}
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Trial {
    pub before:Option<Result<Evaluation,Diagnostic>>,pub after:Result<Evaluation,Diagnostic>,
}
pub struct Resident {
    runtime:Runtime,program:Program,cache:BTreeMap<Key,Evaluation>,order:VecDeque<Key>,
    watches:BTreeMap<String,Watch>,config:Config,
}
fn type_name(t:Type)->&'static str{
    match t{Type::Bool=>"bool",Type::Int=>"int",Type::Text=>"text"}
}
fn diagnostic(id:&str,code:&'static str)->Diagnostic{Diagnostic{definition:id.into(),code}}
impl Resident {
    pub fn start(snapshot:Snapshot)->Result<Self,Error>{
        Self::from_runtime(Runtime::start(snapshot))
    }
    fn from_runtime(runtime:Runtime)->Result<Self,Error>{
        let program=Program::compile(runtime.snapshot()).map_err(Error::InvalidTypedDefinition)?;
        let config=Config::read(&program)?;
        Ok(Self{runtime,program,cache:BTreeMap::new(),order:VecDeque::new(),watches:BTreeMap::new(),config})
    }
    /// @hq:context Restore accepted language and effective config; query caches and observers start fresh.
    pub fn from_checkpoint(text:&str)->Result<Self,Error>{Self::from_runtime(Runtime::from_checkpoint(text)?)}
    pub fn checkpoint(&self)->Result<String,Error>{self.runtime.checkpoint()}
    /// @hq:context Preview selected historical behaviors as compatible new revisions.
    pub fn preview_recover(&self,expected:u64,text:&str,ids:&BTreeSet<String>)->Result<Option<Delta>,Error>{
        let historical=Runtime::from_checkpoint(text)?;
        let patch=self.runtime.recovery_patch(expected,&historical,ids)?;
        self.preview(expected,patch,BTreeSet::new())
    }
    /// @hq:invariant Recovery uses the same configuration, compatibility, CAS and invalidation path as ordinary changes.
    pub fn recover(&mut self,expected:u64,text:&str,ids:&BTreeSet<String>)->Result<Option<Delta>,Error>{
        let historical=Runtime::from_checkpoint(text)?;
        let patch=self.runtime.recovery_patch(expected,&historical,ids)?;
        self.apply(expected,patch,BTreeSet::new())
    }
    pub fn generation(&self)->u64{self.runtime.generation()}
    pub fn ids(&self)->impl Iterator<Item=&str>{self.program.ids()}
    pub fn inspect(&self,id:&str)->Option<&Function>{self.program.get(id)}
    pub fn definition(&self,id:&str)->Option<&Definition>{self.runtime.snapshot().get(id)}
    pub fn changes_since(&self,cursor:u64)->Result<Vec<&Delta>,Error>{self.runtime.changes_since(cursor)}
    pub fn cache_len(&self)->usize{self.cache.len()}
    pub fn config(&self)->Config{self.config}
    /// @hq:context Validate effective configuration during candidate evaluation, not only after activation.
    fn candidate(&self,expected:u64,upserts:Vec<Definition>,removals:BTreeSet<String>)->Result<(Runtime,Program,Config,Option<Delta>),Error>{
        let mut runtime=self.runtime.clone();let delta=runtime.patch(expected,upserts,removals)?;
        let program=Program::compile(runtime.snapshot()).map_err(Error::InvalidTypedDefinition)?;
        let config=Config::read(&program)?;
        if self.watches.len()>config.watch_limit{return Err(Error::InvalidTypedDefinition(diagnostic("sdk.runtime.watch_limit","ACTIVE_WATCH_LIMIT")));}
        Ok((runtime,program,config,delta))
    }
    /// @hq:context Memoize successful deterministic queries by complete typed input.
    /// @hq:invariant No errors are cached; effective fuel and bounded values define the cache domain; fuel changes invalidate all prior results.
    pub fn query(&mut self,id:&str,args:Vec<Value>)->Result<Query,Diagnostic>{
        let key=Key{id:id.into(),args};
        if let Some(evaluation)=self.cache.get(&key){
            return Ok(Query{generation:self.generation(),cache_hit:true,evaluation:evaluation.clone()});
        }
        let evaluation=self.program.evaluate(&key.id,key.args.clone(),self.config.fuel)?;
        if self.cache.len()>=self.config.cache_limit{
            if let Some(old)=self.order.pop_front(){self.cache.remove(&old);}
        }
        self.order.push_back(key.clone());self.cache.insert(key,evaluation.clone());
        Ok(Query{generation:self.generation(),cache_hit:false,evaluation})
    }
    /// @hq:context Classify semantic inputs using two versioned pure functions instead of hardcoded selection policy.
    /// @hq:invariant Ranking results are bounded advisory data, never authority or verified behavior.
    pub fn rank(&mut self,required:&str,priority:&str,args:Vec<Value>)->Result<Rank,Diagnostic>{
        let required=match self.query(required,args.clone())?.evaluation.value{
            Value::Bool(v)=>v,_=>return Err(diagnostic(required,"RANK_REQUIRED_TYPE"))
        };
        let priority=match self.query(priority,args)?.evaluation.value{
            Value::Int(n) if (0..=1000).contains(&n)=>n,_=>return Err(diagnostic(priority,"RANK_PRIORITY_BOUND"))
        };
        Ok(Rank{generation:self.generation(),required,priority})
    }
    pub fn preview(&self,expected:u64,upserts:Vec<Definition>,removals:BTreeSet<String>)->Result<Option<Delta>,Error>{
        Ok(self.candidate(expected,upserts,removals)?.3)
    }
    /// @hq:context Publish a validated candidate, invalidating only affected query results.
    /// @hq:invariant Failed candidates leave accepted state, caches and observations unchanged.
    pub fn apply(&mut self,expected:u64,upserts:Vec<Definition>,removals:BTreeSet<String>)->Result<Option<Delta>,Error>{
        let (candidate,compiled,config,delta)=self.candidate(expected,upserts,removals)?;
        let Some(d)=delta else{return Ok(None)};
        // A fuel change alters query admissibility; even unrelated prior results must be invalidated.
        if config.fuel!=self.config.fuel{self.cache.clear();self.order.clear();}
        self.cache.retain(|key,_|!d.affected.contains(&key.id));
        self.order.retain(|key|!d.affected.contains(&key.id));
        while self.cache.len()>config.cache_limit{if let Some(old)=self.order.pop_front(){self.cache.remove(&old);}else{break;}}
        self.runtime=candidate;self.program=compiled;self.config=config;
        Ok(Some(d))
    }
    /// @hq:context Synthesize a typed function from existing callable stages; first stage may have multiple parameters.
    /// @hq:invariant Every later stage must accept exactly the prior result type; preview grants no activation.
    pub fn compose(&self,expected:u64,id:&str,revision:u64,stages:&[String])->Result<Composition,Error>{
        let invalid=|code|Error::InvalidTypedDefinition(diagnostic(id,code));
        if expected!=self.generation(){return Err(Error::StaleGeneration);}
        if stages.is_empty()||stages.len()>self.config.composition_limit{return Err(invalid("COMPOSITION_LIMIT"));}
        let first=self.inspect(&stages[0]).ok_or_else(||invalid("UNKNOWN_STAGE"))?;
        let params=first.params.iter().map(|(n,t)|format!("({n} {})",type_name(*t))).collect::<Vec<_>>().join(" ");
        let mut body=format!("(call {}",stages[0]);
        for (n,_) in &first.params{body.push_str(&format!(" (arg {n})"));}
        body.push(')');
        let mut result=first.result;
        for name in &stages[1..]{
            let f=self.inspect(name).ok_or_else(||invalid("UNKNOWN_STAGE"))?;
            if f.params.len()!=1||f.params[0].1!=result{return Err(invalid("STAGE_TYPE"));}
            result=f.result;body=format!("(call {name} {body})");
        }
        let definition=Definition{id:id.into(),revision,dependencies:stages.iter().cloned().collect(),
            body:format!("meta/1 (fn ({params}) {} {body})",type_name(result))};
        let delta=self.preview(expected,vec![definition.clone()],BTreeSet::new())?;
        Ok(Composition{base_generation:expected,definition,delta})
    }
    /// @hq:context Trial a candidate against explicit inputs in isolated state, without changing current definitions or caches.
    /// @hq:invariant A sample comparison is not a general proof of behavioral equivalence.
    pub fn trial(&self,expected:u64,definition:Definition,target:&str,args:Vec<Value>)->Result<Trial,Error>{
        let (_,compiled,config,_)=self.candidate(expected,vec![definition],BTreeSet::new())?;
        Ok(Trial{before:self.inspect(target).map(|_|self.program.evaluate(target,args.clone(),self.config.fuel)),
            after:compiled.evaluate(target,args,config.fuel)})
    }
    /// @hq:context Register a named query and obtain its initial semantic observation.
    pub fn watch(&mut self,name:&str,id:&str,args:Vec<Value>)->Result<Observation,Diagnostic>{
        if name.is_empty()||name.len()>64||!name.bytes().all(|b|b.is_ascii_alphanumeric()||b"._-".contains(&b)){return Err(diagnostic(id,"WATCH_NAME"));}
        if self.watches.contains_key(name){return Err(diagnostic(id,"WATCH_EXISTS"));}
        if self.watches.len()>=self.config.watch_limit{return Err(diagnostic(id,"WATCH_LIMIT"));}
        let value=Ok(self.query(id,args.clone())?.evaluation.value);
        self.watches.insert(name.into(),Watch{key:Key{id:id.into(),args},revision:1,last:value.clone()});
        Ok(Observation{generation:self.generation(),semantic_revision:1,changed:true,value})
    }
    /// @hq:context Pull the current semantic value; suppress semantically equal revisions.
    /// @hq:invariant Pull observation is not push delivery or durable acknowledgement.
    pub fn poll(&mut self,name:&str)->Result<Observation,Diagnostic>{
        let mut w=self.watches.get(name).cloned().ok_or_else(||diagnostic(name,"WATCH_UNKNOWN"))?;
        let value=self.query(&w.key.id,w.key.args.clone()).map(|q|q.evaluation.value);
        let changed=w.last!=value;
        if changed{
            w.revision=w.revision.checked_add(1).ok_or_else(||diagnostic(name,"WATCH_REVISION_OVERFLOW"))?;
            w.last=value.clone();self.watches.insert(name.into(),w.clone());
        }
        Ok(Observation{generation:self.generation(),semantic_revision:w.revision,changed,value})
    }
    pub fn unwatch(&mut self,name:&str)->bool{self.watches.remove(name).is_some()}
}

#[cfg(test)]
mod tests{
    use super::*;
    fn start()->Resident{
        Resident::start(Snapshot::parse("a\t1\t\tmeta/1 (fn ((n int)) int (add (arg n) (int 1)))\nb\t1\ta\tmeta/1 (fn ((n int)) int (call a (arg n)))\nc\t1\t\tmeta/1 (fn () bool (bool true))").unwrap()).unwrap()
    }
    fn d(rev:u64,body:&str)->Definition{Definition{id:"a".into(),revision:rev,dependencies:BTreeSet::new(),body:body.into()}}
    #[test]
    fn selective_cache_invalidation_and_preview(){
        let mut r=start();
        assert!(!r.query("b",vec![Value::Int(1)]).unwrap().cache_hit);
        assert!(r.query("b",vec![Value::Int(1)]).unwrap().cache_hit);
        r.query("c",vec![]).unwrap();
        let new=d(2,"meta/1 (fn ((n int)) int (add (arg n) (int 2)))");
        r.preview(1,vec![new.clone()],BTreeSet::new()).unwrap();
        assert_eq!(r.generation(),1);assert_eq!(r.cache_len(),2);
        r.apply(1,vec![new],BTreeSet::new()).unwrap();
        assert_eq!(r.cache_len(),1);
        let b=r.query("b",vec![Value::Int(1)]).unwrap();assert!(!b.cache_hit);assert_eq!(b.evaluation.value,Value::Int(3));
        let c=r.query("c",vec![]).unwrap();assert!(c.cache_hit);assert_eq!(c.generation,2);
    }
    #[test]
    fn watch_equality_suppression_and_changed_result(){
        let mut r=start();let first=r.watch("w","b",vec![Value::Int(1)]).unwrap();
        assert_eq!(first.semantic_revision,1);
        let same=d(2,"meta/1 (fn ((n int)) int (add (int 1) (arg n)))");
        r.apply(1,vec![same],BTreeSet::new()).unwrap();
        let p=r.poll("w").unwrap();assert!(!p.changed);assert_eq!(p.semantic_revision,1);assert_eq!(p.generation,2);
        let different=d(3,"meta/1 (fn ((n int)) int (add (arg n) (int 2)))");
        r.apply(2,vec![different],BTreeSet::new()).unwrap();
        let p=r.poll("w").unwrap();assert!(p.changed);assert_eq!(p.semantic_revision,2);assert_eq!(p.value,Ok(Value::Int(3)));
        assert!(!r.poll("w").unwrap().changed);assert!(r.unwatch("w"));assert!(r.poll("w").is_err());
    }
    #[test]
    fn failure_atomicity_and_cache_bounds(){
        let mut r=start();r.query("a",vec![Value::Int(1)]).unwrap();
        assert!(r.apply(1,vec![d(2,"meta/1 (fn ((n int)) int (bool true))")],BTreeSet::new()).is_err());
        assert_eq!(r.generation(),1);assert!(r.query("a",vec![Value::Int(1)]).unwrap().cache_hit);
        for n in 0..100{r.query("a",vec![Value::Int(n)]).unwrap();}
        assert_eq!(r.cache_len(),CACHE_LIMIT);
        assert!(r.query("a",vec![Value::Int(i64::MAX)]).is_err());assert_eq!(r.cache_len(),CACHE_LIMIT);
    }
    #[test]
    fn synthesis_trial_activation_and_recomposition(){
        let mut r=start();
        let proposed=r.compose(1,"x",1,&["a".into(),"b".into()]).unwrap();
        assert_eq!(r.generation(),1);assert!(r.inspect("x").is_none());
        let t=r.trial(1,proposed.definition.clone(),"x",vec![Value::Int(5)]).unwrap();
        assert_eq!(t.before,None);assert_eq!(t.after.unwrap().value,Value::Int(7));
        r.apply(proposed.base_generation,vec![proposed.definition],BTreeSet::new()).unwrap();
        assert_eq!(r.query("x",vec![Value::Int(5)]).unwrap().evaluation.value,Value::Int(7));
        let proposed=r.compose(2,"x",2,&["a".into(),"b".into(),"a".into()]).unwrap();
        let t=r.trial(2,proposed.definition.clone(),"x",vec![Value::Int(5)]).unwrap();
        assert_eq!(t.before.unwrap().unwrap().value,Value::Int(7));assert_eq!(t.after.unwrap().value,Value::Int(8));
        r.apply(2,vec![proposed.definition],BTreeSet::new()).unwrap();
        assert_eq!(r.query("x",vec![Value::Int(5)]).unwrap().evaluation.value,Value::Int(8));
        assert!(r.compose(2,"y",1,&["a".into()]).is_err());
        assert!(r.compose(3,"y",1,&["a".into(),"c".into()]).is_err());
        assert!(r.compose(3,"a",2,&["a".into()]).is_err());
        assert_eq!(r.generation(),3);
    }

    #[test]
    fn versioned_configuration_is_atomic_and_reconfigures_cache(){
        let mut r=start();
        let setting=|id:&str,rev:u64,n:i64|Definition{id:id.into(),revision:rev,dependencies:BTreeSet::new(),body:format!("meta/1 (fn () int (int {n}))")};
        r.query("a",vec![Value::Int(1)]).unwrap();r.query("b",vec![Value::Int(1)]).unwrap();
        let d=setting("sdk.runtime.cache_limit",1,1);
        r.preview(1,vec![d.clone()],BTreeSet::new()).unwrap();assert_eq!(r.cache_len(),2);
        r.apply(1,vec![d],BTreeSet::new()).unwrap();assert_eq!(r.config().cache_limit,1);assert_eq!(r.cache_len(),1);
        let invalid=setting("sdk.runtime.cache_limit",2,65);
        assert!(r.preview(2,vec![invalid.clone()],BTreeSet::new()).is_err());
        assert!(r.apply(2,vec![invalid],BTreeSet::new()).is_err());assert_eq!(r.generation(),2);
        let fuel=setting("sdk.runtime.fuel",1,1);
        r.apply(2,vec![fuel],BTreeSet::new()).unwrap();assert_eq!(r.cache_len(),0);
        assert!(r.query("b",vec![Value::Int(1)]).is_err());
    }
    #[test]
    fn configuration_cannot_orphan_existing_observers(){
        let mut r=start();
        r.watch("w1","c",vec![]).unwrap();r.watch("w2","c",vec![]).unwrap();
        let d=Definition{id:"sdk.runtime.watch_limit".into(),revision:1,dependencies:BTreeSet::new(),body:"meta/1 (fn () int (int 1))".into()};
        assert!(r.preview(1,vec![d.clone()],BTreeSet::new()).is_err());
        assert!(r.apply(1,vec![d.clone()],BTreeSet::new()).is_err());
        r.unwatch("w2");r.apply(1,vec![d],BTreeSet::new()).unwrap();assert_eq!(r.config().watch_limit,1);
        assert!(r.watch("w3","c",vec![]).is_err());assert!(!r.poll("w1").unwrap().changed);
    }
    #[test]
    fn classification_is_live_typed_and_bounded(){
        let mut r=start();
        let rule=|id:&str,rev:u64,body:&str|Definition{id:id.into(),revision:rev,dependencies:BTreeSet::new(),body:body.into()};
        r.apply(1,vec![rule("required",1,"meta/1 (fn ((tag text)) bool (eq (arg tag) (text \"invariant\")))"),rule("priority",1,"meta/1 (fn ((tag text)) int (int 10))")],BTreeSet::new()).unwrap();
        let args=vec![Value::Text("invariant".into())];
        assert_eq!(r.rank("required","priority",args.clone()).unwrap(),Rank{generation:2,required:true,priority:10});
        r.apply(2,vec![rule("priority",2,"meta/1 (fn ((tag text)) int (int 90))")],BTreeSet::new()).unwrap();
        assert_eq!(r.rank("required","priority",args.clone()).unwrap(),Rank{generation:3,required:true,priority:90});
        r.apply(3,vec![rule("priority",3,"meta/1 (fn ((tag text)) int (int 1001))")],BTreeSet::new()).unwrap();
        assert_eq!(r.rank("required","priority",args).unwrap_err().code,"RANK_PRIORITY_BOUND");
    }

    #[test]
    fn checkpoint_restores_config_and_forward_recovery_reuses_invalidation(){
        let mut r=start();
        let original=r.checkpoint().unwrap();
        r.query("c",vec![]).unwrap();
        r.watch("w","b",vec![Value::Int(5)]).unwrap();
        r.apply(1,vec![d(2,"meta/1 (fn ((n int)) int (add (arg n) (int 3)))")],BTreeSet::new()).unwrap();
        assert_eq!(r.poll("w").unwrap().value,Ok(Value::Int(8)));
        let ids=["a".into()].into_iter().collect();
        let preview=r.preview_recover(2,&original,&ids).unwrap().unwrap();
        assert_eq!(r.generation(),2);
        assert_eq!(r.recover(2,&original,&ids).unwrap().unwrap(),preview);
        assert_eq!(r.generation(),3);assert_eq!(r.definition("a").unwrap().revision,3);
        assert_eq!(r.poll("w").unwrap().value,Ok(Value::Int(6)));
        assert!(r.query("c",vec![]).unwrap().cache_hit);
        let config=Definition{id:"sdk.runtime.cache_limit".into(),revision:1,dependencies:BTreeSet::new(),body:"meta/1 (fn () int (int 2))".into()};
        r.apply(3,vec![config],BTreeSet::new()).unwrap();
        let mut restored=Resident::from_checkpoint(&r.checkpoint().unwrap()).unwrap();
        assert_eq!(restored.generation(),4);assert_eq!(restored.config().cache_limit,2);
        assert_eq!(restored.cache_len(),0);assert!(restored.poll("w").is_err());
        assert_eq!(restored.query("b",vec![Value::Int(5)]).unwrap().evaluation.value,Value::Int(6));
        assert_eq!(restored.changes_since(3),Err(Error::ResyncRequired));
    }
}
