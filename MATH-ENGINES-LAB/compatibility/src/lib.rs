//! Compatibility experiment, not a production registry. The R001 kernel is unchanged.
#![forbid(unsafe_code)]
use std::collections::{BTreeMap, BTreeSet};
use vivim_math_engines_lab::{Rational, MathError, Scalar, Operation, Add, Subtract, Multiply, Divide, Negate};
pub mod standard;
pub mod extensions;

pub const DOMAIN: &str = "rational.i128";
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key { pub id: &'static str, pub rev: u32, pub domain: &'static str }
pub const fn key(id: &'static str) -> Key { Key { id, rev: 1, domain: DOMAIN } }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind { Number, Text }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assurance { Exact, Rounded }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Signature { pub args: &'static [Kind], pub output: Kind, pub assurance: Assurance }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Requirement { pub key: Key, pub signature: Signature }
pub const BINARY: Signature = Signature { args: &[Kind::Number,Kind::Number],output:Kind::Number,assurance:Assurance::Exact };
pub const UNARY: Signature = Signature { args: &[Kind::Number],output:Kind::Number,assurance:Assurance::Exact };
pub const fn req(id: &'static str, signature: Signature) -> Requirement { Requirement { key:key(id), signature } }
pub const ADD: Requirement = req("math.scalar.add",BINARY);
pub const SUB: Requirement = req("math.scalar.subtract",BINARY);
pub const MUL: Requirement = req("math.scalar.multiply",BINARY);
pub const DIV: Requirement = req("math.scalar.divide",BINARY);
pub const NEG: Requirement = req("math.scalar.negate",UNARY);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value { Number(Rational), Text(String) }
impl Value {
    fn kind(&self) -> Kind { match self { Self::Number(_)=>Kind::Number, Self::Text(_)=>Kind::Text } }
    pub fn number(&self)->Result<Rational,Error> { match self { Self::Number(n)=>Ok(*n), _=>Err(Error::Type) } }
    pub fn text(&self)->Result<&str,Error> { match self { Self::Text(s)=>Ok(s), _=>Err(Error::Type) } }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Missing(Key), Incompatible(Key), Duplicate(Key), Cycle(Key), Undeclared(Key),
    Unqualified(Key), FixtureFailed(Key), Stale, Type, Fuel, Limit, Invalid, Math(MathError)
}
impl From<MathError> for Error { fn from(x:MathError)->Self { Self::Math(x) } }
pub type Result<T> = std::result::Result<T,Error>;
pub type Eval = fn(&mut Call<'_>, &[Value])->Result<Value>;
#[derive(Clone)]
pub struct Provider {
    pub contract: Requirement, pub requires: &'static [Requirement], pub run: Eval
}
#[derive(Clone, Debug)]
pub struct Fixture { pub capability: Requirement, pub input: Vec<Value>, pub expected: Result<Value> }

#[derive(Clone)]
pub struct Registry { entries:BTreeMap<Key,Provider>, generation:u64 }
#[derive(Clone,Debug)]
pub struct Plan { generation:u64, roots:Vec<Requirement> }
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Gap { pub consumer:Key, pub required:Requirement, pub error:Error }

impl Registry {
    pub fn baseline()->Self {
        let mut entries=BTreeMap::new();
        for p in [
            Provider{contract:ADD,requires:&[],run:|_,v| Ok(Value::Number(Add.evaluate((v[0].number()?,v[1].number()?))?))},
            Provider{contract:SUB,requires:&[],run:|_,v| Ok(Value::Number(Subtract.evaluate((v[0].number()?,v[1].number()?))?))},
            Provider{contract:MUL,requires:&[],run:|_,v| Ok(Value::Number(Multiply.evaluate((v[0].number()?,v[1].number()?))?))},
            Provider{contract:DIV,requires:&[],run:|_,v| Ok(Value::Number(Divide.evaluate((v[0].number()?,v[1].number()?))?))},
            Provider{contract:NEG,requires:&[],run:|_,v| Ok(Value::Number(Negate.evaluate(v[0].number()?)?))}
        ] { entries.insert(p.contract.key,p); }
        Self{entries,generation:1}
    }
    pub fn generation(&self)->u64 { self.generation }
    pub fn len(&self)->usize { self.entries.len() }
    pub fn is_empty(&self)->bool { self.entries.is_empty() }
    fn lookup(&self,r:Requirement)->Result<&Provider> {
        let p=self.entries.get(&r.key).ok_or(Error::Missing(r.key))?;
        if p.contract!=r { return Err(Error::Incompatible(r.key)); } Ok(p)
    }
    /// Reports root and transitive missing/incompatible dependencies, without mutation.
    pub fn gaps(&self,consumer:Key, roots:&[Requirement])->Vec<Gap> {
        fn visit(reg:&Registry,consumer:Key,r:Requirement,seen:&mut BTreeSet<Key>,out:&mut Vec<Gap>) {
            match reg.lookup(r) {
                Err(error)=>out.push(Gap{consumer,required:r,error}),
                Ok(p)=>if seen.insert(r.key) { for &dep in p.requires { visit(reg,r.key,dep,seen,out); } }
            }
        }
        let mut seen=BTreeSet::new();let mut out=Vec::new();
        for &r in roots {visit(self,consumer,r,&mut seen,&mut out);}
        out
    }
    pub fn plan(&self,roots:&[Requirement])->Result<Plan> {
        self.validate(roots)?;
        Ok(Plan{generation:self.generation,roots:roots.to_vec()})
    }
    fn validate(&self,roots:&[Requirement])->Result<()> {
        fn walk(reg:&Registry,r:Requirement,visiting:&mut BTreeSet<Key>,done:&mut BTreeSet<Key>,depth:usize)->Result<()> {
            if depth>64 {return Err(Error::Limit);}
            let p=reg.lookup(r)?;
            if done.contains(&r.key){return Ok(());}
            if !visiting.insert(r.key){return Err(Error::Cycle(r.key));}
            for &d in p.requires{walk(reg,d,visiting,done,depth+1)?;}
            visiting.remove(&r.key);done.insert(r.key);Ok(())
        }
        let(mut visiting,mut done)=(BTreeSet::new(),BTreeSet::new());
        for &r in roots{walk(self,r,&mut visiting,&mut done,0)?;} Ok(())
    }
    /// Candidate bundle admission is all-or-nothing. Old entries are immutable;
    /// new versions have different keys. Fixtures are supplied by the caller, not
    /// by providers. These checks are not a security sandbox or formal proof.
    pub fn admit(&mut self, bundle:Vec<Provider>, fixtures:&[Fixture])->Result<()> {
        if bundle.is_empty() || bundle.len()>128 || fixtures.len()>4096 || self.entries.len()+bundle.len()>2048 {return Err(Error::Limit);}
        let mut next=self.clone();let mut roots=Vec::new();
        for p in bundle {
            let k=p.contract.key;
            if k.rev==0||k.id.is_empty()||k.domain.is_empty()||p.requires.len()>64{return Err(Error::Invalid);}
            if next.entries.contains_key(&k){return Err(Error::Duplicate(k));}
            roots.push(p.contract);next.entries.insert(k,p);
        }
        next.validate(&roots)?;
        for &r in &roots {
            let cases:Vec<_>=fixtures.iter().filter(|f|f.capability==r).collect();
            if !cases.iter().any(|f|f.expected.is_ok())||!cases.iter().any(|f|f.expected.is_err()){
                return Err(Error::Unqualified(r.key));
            }
            for f in cases {
                if next.execute(r,&f.input,1024)!=f.expected{return Err(Error::FixtureFailed(r.key));}
            }
        }
        next.generation=self.generation.checked_add(1).ok_or(Error::Limit)?;
        *self=next;Ok(())
    }
    pub fn call(&self,p:&Plan,r:Requirement,input:&[Value],fuel:u32)->Result<Value>{
        if p.generation!=self.generation{return Err(Error::Stale);}
        if !p.roots.contains(&r){return Err(Error::Undeclared(r.key));}
        self.execute(r,input,fuel)
    }
    fn execute(&self,r:Requirement,input:&[Value],mut fuel:u32)->Result<Value>{
        dispatch(self,r,input,&mut fuel,0)
    }
}
pub struct Call<'a>{registry:&'a Registry,allowed:&'static [Requirement],fuel:&'a mut u32,depth:usize}
impl Call<'_>{
    pub fn call(&mut self,r:Requirement,input:&[Value])->Result<Value>{
        if !self.allowed.contains(&r){return Err(Error::Undeclared(r.key));}
        dispatch(self.registry,r,input,self.fuel,self.depth+1)
    }
}
fn dispatch(reg:&Registry,r:Requirement,input:&[Value],fuel:&mut u32,depth:usize)->Result<Value>{
    if depth>64{return Err(Error::Limit);}
    if *fuel==0{return Err(Error::Fuel);}*fuel-=1;
    let p=reg.lookup(r)?;
    if input.len()!=r.signature.args.len()||input.iter().zip(r.signature.args).any(|(v,k)|v.kind()!=*k){return Err(Error::Type);}
    if input.iter().any(|v|matches!(v,Value::Text(s) if s.len()>256)){return Err(Error::Limit);}
    let mut call=Call{registry:reg,allowed:p.requires,fuel,depth};
    let output=(p.run)(&mut call,input)?;
    if output.kind()!=r.signature.output{return Err(Error::Type);}
    if matches!(&output,Value::Text(s) if s.len()>256){return Err(Error::Limit);}
    Ok(output)
}
pub fn number(n:i128,d:i128)->Value { Value::Number(Rational::new(n,d).expect("valid fixture literal")) }
