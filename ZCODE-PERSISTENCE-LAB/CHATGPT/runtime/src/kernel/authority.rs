//! Authenticated request admission for an explicitly configured trusted host.
//! Keys remain outside the runtime store. No listener, network or OS ACL changes.
use super::{model::*,transition::{Actor,Action,Request},persistence::KernelStore};
use crate::Result;
use serde::{Serialize,Deserialize};
use sha2::{Sha256,Digest as _};
use serde_json::{Value,json};
use std::{collections::{BTreeMap,BTreeSet},path::Path};

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="snake_case")]
pub enum Role {Host,Worker,Verifier,Adapter}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Principal{
    pub role:Role,pub subject:Id,pub secret_hex:String,pub incarnation:Option<u64>,
    pub goals:BTreeSet<Id>,pub expires_ms:u64,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Authority{
    pub audience:Id,pub principals:BTreeMap<Id,Principal>,
    pub maximum_window_ms:u64,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct SignedRequest{
    pub audience:Id,pub principal:Id,pub issued_ms:u64,pub expires_ms:u64,
    pub request:Request,pub signature:String,
}
fn decode(s:&str)->Result<Vec<u8>>{
    ensure(s.len()%2==0&&s.len()<=256,"invalid hex length")?;
    (0..s.len()).step_by(2).map(|i|{
        let pair=s.as_bytes().get(i..i+2).ok_or("hex pair missing")?;
        let p=std::str::from_utf8(pair).map_err(|_|"invalid hex")?;
        u8::from_str_radix(p,16).map_err(|_|"invalid hex".into())
    }).collect()
}
fn hmac(secret:&[u8],message:&[u8])->[u8;32]{
    let mut key=[0u8;64];
    if secret.len()>64{key[..32].copy_from_slice(&Sha256::digest(secret));}
    else{key[..secret.len()].copy_from_slice(secret);}
    let mut inner=[0x36u8;64];let mut outer=[0x5cu8;64];
    for i in 0..64{inner[i]^=key[i];outer[i]^=key[i];}
    let mut h=Sha256::new();h.update(inner);h.update(message);
    let first=h.finalize();let mut h=Sha256::new();h.update(outer);h.update(first);
    h.finalize().into()
}
fn same(a:&[u8],b:&[u8])->bool{
    if a.len()!=b.len(){return false;}
    a.iter().zip(b).fold(0u8,|d,(x,y)|d|(x^y))==0
}
impl SignedRequest{
    fn bytes(&self)->Result<Vec<u8>>{
        serde_json::to_vec(&json!({"domain":"vivim-kernel-auth-v1","audience":self.audience,"principal":self.principal,"issued_ms":self.issued_ms,"expires_ms":self.expires_ms,"request":self.request})).map_err(|e|e.to_string())
    }
    /// Signing is provided for a separately trusted issuer, never by a worker
    /// subprocess with access to the host's authority file.
    pub fn sign(&mut self,secret_hex:&str)->Result<()>{
        let key=decode(secret_hex)?;ensure(key.len()>=32,"key must contain at least 256 bits")?;
        self.signature=hmac(&key,&self.bytes()?).iter().map(|b|format!("{b:02x}")).collect::<Vec<_>>().join("");Ok(())
    }
}
impl Authority{
    pub fn load(path:&Path)->Result<Self>{
        let bytes=std::fs::read(path).map_err(|e|e.to_string())?;
        ensure(bytes.len()<=MAX_VALUE_BYTES,"authority file exceeds budget")?;
        let a:Self=serde_json::from_slice(&bytes).map_err(|e|e.to_string())?;
        id(&a.audience)?;ensure((1..=3_600_000).contains(&a.maximum_window_ms),"invalid admission window")?;
        for (key,p) in &a.principals{
            id(key)?;id(&p.subject)?;
            ensure(decode(&p.secret_hex)?.len()>=32,"short principal secret")?;
            ensure(p.role!=Role::Worker||p.incarnation.is_some(),"worker incarnation required")?;
        }Ok(a)
    }
    pub fn authenticate(&self,signed:&SignedRequest,now_ms:u64)->Result<&Principal>{
        ensure(signed.audience==self.audience,"wrong audience")?;
        ensure(signed.issued_ms<=now_ms&&now_ms<signed.expires_ms&&signed.expires_ms.saturating_sub(signed.issued_ms)<=self.maximum_window_ms,"request outside admission window")?;
        let p=self.principals.get(&signed.principal).ok_or("unknown principal")?;
        ensure(now_ms<p.expires_ms,"principal expired")?;
        let secret=decode(&p.secret_hex)?;let mac=decode(&signed.signature)?;
        ensure(secret.len()>=32&&same(&hmac(&secret,&signed.bytes()?),&mac),"signature rejected")?;
        id(&signed.request.id)?;
        match (&p.role,&signed.request.actor){
            (Role::Host,Actor::Host)=>{},
            (Role::Worker,Actor::Worker{id,incarnation})=>ensure(id==&p.subject&&Some(*incarnation)==p.incarnation,"worker scope mismatch")?,
            (Role::Verifier,Actor::Verifier{id})=>ensure(id==&p.subject,"verifier scope mismatch")?,
            (Role::Adapter,Actor::Adapter{id})=>ensure(id==&p.subject,"adapter scope mismatch")?,
            _=>return Err("actor role mismatch".into()),
        }
        Ok(p)
    }
    pub fn authorize_replay(&self,signed:&SignedRequest,world:&World,now_ms:u64)->Result<()>{
        let p=self.authenticate(signed,now_ms)?;
        match p.role{
            Role::Host=>Ok(()),
            Role::Worker=>{let a=world.agents.get(&p.subject).ok_or("worker missing")?;ensure(p.goals.contains(&a.goal),"worker goal not authorized")?;ensure(a.incarnations.last().is_some_and(|i|Some(i.number)==p.incarnation),"superseded worker")},
            Role::Verifier|Role::Adapter=>self.authorize(signed,world,now_ms),
        }
    }
    pub fn authorize(&self,signed:&SignedRequest,world:&World,now_ms:u64)->Result<()>{
        let p=self.authenticate(signed,now_ms)?;
        match p.role{
            Role::Host=>Ok(()),
            Role::Worker=>{
                let a=world.agents.get(&p.subject).ok_or("worker missing")?;
                ensure(p.goals.contains(&a.goal),"worker goal not authorized")?;
                ensure(world.current_incarnation(&p.subject)?==p.incarnation.ok_or("incarnation missing")?,"stale worker principal")?;
                ensure(matches!(&signed.request.action,Action::Renew{..}|Action::Checkpoint{..}|Action::Submit{..}|Action::Fail{..}|Action::Send{..}|Action::Ack{..}|Action::WaitMessage{..}|Action::WaitDependency{..}|Action::Receive{..}),"worker action not authorized")
            }
            Role::Adapter=>{
                let envelope=world.authorize_adapter_action(&p.subject,&signed.request.action)?;
                let task=world.tasks.get(&envelope.task).ok_or("adapter task missing")?;
                ensure(p.goals.contains(&task.spec.goal),"adapter goal outside signed scope")
            }
            Role::Verifier=>{
                if let Action::AttestCandidate{verdict}=&signed.request.action{
                    let c=world.cohorts.get(&verdict.cohort).ok_or("cohort missing")?;
                    return ensure(c.verifier==p.subject&&p.goals.contains(&c.goal),"custody scope denied");
                }
                let Action::Verify{task}= &signed.request.action else{return Err("verifier may only verify or attest a cohort".into());};
                let t=world.tasks.get(task).ok_or("verification task missing")?;
                ensure(p.goals.contains(&t.spec.goal),"verifier goal not authorized")?;
                ensure(t.lease.as_ref().is_none_or(|l|l.worker!=p.subject),"verifier cannot be submitting worker")
            }
        }
    }
}


#[cfg(test)]
mod qualification {
    use super::*;
    #[test]
    fn rfc4231_hmac_sha256_case_one(){
        let mac=hmac(&[0x0b;20],b"Hi There");
        let expected=decode("b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7").unwrap();
        assert!(same(&mac,&expected));
        assert!(!same(&mac,&[0u8;32]));
    }
    #[test]
    fn signed_adapter_cannot_act_as_host(){
        let secret="aa".repeat(32);
        let mut a=Authority{audience:"lab".into(),principals:BTreeMap::new(),maximum_window_ms:1000};
        a.principals.insert("transport".into(),Principal{role:Role::Adapter,subject:"adapter".into(),secret_hex:secret.clone(),incarnation:None,goals:BTreeSet::from(["goal".into()]),expires_ms:2000});
        let mut r=SignedRequest{audience:"lab".into(),principal:"transport".into(),issued_ms:1,expires_ms:100,request:Request{id:"req".into(),actor:Actor::Host,action:Action::Slots{count:64}},signature:String::new()};
        r.sign(&secret).unwrap();
        assert!(a.authenticate(&r,2).is_err());
        r.request.actor=Actor::Adapter{id:"adapter".into()};r.sign(&secret).unwrap();
        assert!(a.authenticate(&r,2).is_ok());
        assert!(a.authorize(&r,&World::default(),2).is_err());
    }
}
