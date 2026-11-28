//! Independent holdout custody. Private cases live outside canonical state.
//! A frozen class is evaluated once per registered cohort; only commitments
//! and the scoped verifier's signed verdict enter the host journal.
use super::{model::*,evolution::SearchReceipt};
use crate::{digest,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::collections::BTreeSet;
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Cohort {
    pub id:Id,pub family:Id,pub goal:Id,pub verifier:Id,pub commitment:Digest,
    pub input_fingerprints:BTreeSet<Digest>,pub minimum_cases:usize,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct FrozenCandidate{
    pub id:Id,pub capability:Capability,pub cohort:Id,pub class_digest:Digest,
    pub training_fingerprints:BTreeSet<Digest>,pub sequence:u64,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Verdict{
    pub candidate:Id,pub cohort:Id,pub commitment:Digest,pub class_digest:Digest,
    pub accepted:bool,pub evaluations:u64,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct CustodyReceipt{pub verifier:Id,pub verdict:Verdict,pub sequence:u64}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct PrivateCase {pub input:Value,pub expected:Value}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct PrivateCohort {pub salt:String,pub cases:Vec<PrivateCase>}
impl PrivateCohort{
    pub fn commitment(&self)->Result<Digest>{
        ensure(self.salt.len()>=32&&self.salt.len()<=256&&(2..=128).contains(&self.cases.len()),"invalid private cohort")?;
        let mut inputs=BTreeSet::new();
        for c in &self.cases{bounded(&c.input)?;bounded(&c.expected)?;ensure(inputs.insert(digest(&c.input)),"duplicate holdout input")?;}
        Ok(digest(&serde_json::to_value(self).map_err(|e|e.to_string())?))
    }
    pub fn evaluate(&self,world:&World,candidate:&str)->Result<Verdict>{
        let f=world.frozen_candidates.get(candidate).ok_or("candidate missing")?;
        let c=world.cohorts.get(&f.cohort).ok_or("cohort missing")?;
        ensure(!world.custody_receipts.contains_key(&c.id),"cohort already consumed")?;
        ensure(self.commitment()?==c.commitment&&self.cases.len()>=c.minimum_cases,"cohort commitment mismatch")?;
        let fingerprints:BTreeSet<_>=self.cases.iter().map(|x|digest(&x.input)).collect();
        ensure(fingerprints==c.input_fingerprints&&fingerprints.is_disjoint(&f.training_fingerprints),"holdout overlap or mismatch")?;
        ensure(world.class_fingerprint(&f.capability.class)?==f.class_digest,"frozen class changed")?;
        let mut accepted=true;let mut evaluations=0;
        for case in &self.cases{
            evaluations+=1;
            accepted &= world.run_class(&f.capability.class,&case.input).is_ok_and(|v|v==case.expected);
        }
        Ok(Verdict{candidate:candidate.into(),cohort:c.id.clone(),commitment:c.commitment.clone(),class_digest:f.class_digest.clone(),accepted,evaluations})
    }
}
impl World{
    pub fn class_fingerprint(&self,key:&str)->Result<Digest>{
        let class=self.classes.get(key).ok_or("class missing")?;
        let mut operators=Vec::new();
        for node in &class.nodes{operators.push(self.operators.get(&node.operator).ok_or("operator missing")?);}
        Ok(digest(&json!({"class":class,"operators":operators})))
    }
    pub fn register_cohort(&mut self,c:Cohort)->Result<()>{
        id(&c.id)?;id(&c.family)?;id(&c.verifier)?;
        ensure(self.goals.contains_key(&c.goal)&&!self.cohorts.contains_key(&c.id)&&hash_id(&c.commitment),"invalid cohort")?;
        ensure((2..=128).contains(&c.minimum_cases)&&c.input_fingerprints.len()>=c.minimum_cases&&c.input_fingerprints.len()<=128&&c.input_fingerprints.iter().all(|x|hash_id(x)),"invalid cohort commitments")?;
        ensure(self.cohorts.values().all(|old|old.commitment!=c.commitment&&old.input_fingerprints.is_disjoint(&c.input_fingerprints)),"cohort overlap")?;
        // Registering a cohort makes independent custody mandatory for that family.
        self.custody_families.insert(c.family.clone());self.cohorts.insert(c.id.clone(),c);Ok(())
    }
    pub fn freeze_candidate(&mut self,key:Id,capability:Capability,cohort:Id)->Result<Value>{
        id(&key)?;let capkey=version_key(&capability.id,capability.version)?;
        let c=self.cohorts.get(&cohort).ok_or("cohort missing")?;
        ensure(capability.enabled&&capability.family==c.family&&capability.holdouts.is_empty()&&!self.capabilities.contains_key(&capkey)&&!self.frozen_candidates.contains_key(&key),"invalid frozen candidate")?;
        ensure(!self.frozen_candidates.values().any(|x|x.cohort==cohort||version_key(&x.capability.id,x.capability.version).ok().as_ref()==Some(&capkey)),"cohort or revision already frozen")?;
        let class=self.classes.get(&capability.class).ok_or("class missing")?;
        ensure((2..=128).contains(&capability.training.len()),"invalid training cohort")?;
        let mut fingerprints=BTreeSet::new();
        for sample in &capability.training{
            let (x,y)=self.example(sample,&capability.family,&class.input,&class.output)?;
            ensure(fingerprints.insert(digest(&x))&&self.run_class(&capability.class,&x)?==y,"training validation failed")?;
        }
        ensure(self.cohorts.values().all(|h|h.input_fingerprints.is_disjoint(&fingerprints)),"training contains reserved holdout")?;
        let class_digest=self.class_fingerprint(&capability.class)?;
        self.frozen_candidates.insert(key.clone(),FrozenCandidate{id:key.clone(),capability,cohort,class_digest:class_digest.clone(),training_fingerprints:fingerprints,sequence:self.sequence.checked_add(1).ok_or("sequence overflow")?});
        Ok(json!({"candidate":key,"class_digest":class_digest}))
    }
    pub fn attest_candidate(&mut self,verifier:&str,verdict:Verdict)->Result<Value>{
        let f=self.frozen_candidates.get(&verdict.candidate).ok_or("candidate missing")?.clone();
        let c=self.cohorts.get(&f.cohort).ok_or("cohort missing")?.clone();
        ensure(c.verifier==verifier&&verdict.cohort==c.id&&verdict.commitment==c.commitment&&verdict.class_digest==f.class_digest,"verdict scope mismatch")?;
        ensure(!self.custody_receipts.contains_key(&c.id)&&verdict.evaluations==c.input_fingerprints.len() as u64,"cohort already consumed or incomplete evaluation")?;
        ensure(self.class_fingerprint(&f.capability.class)?==f.class_digest,"frozen class changed")?;
        let accepted=verdict.accepted;
        self.custody_receipts.insert(c.id.clone(),CustodyReceipt{verifier:verifier.into(),verdict,sequence:self.sequence.checked_add(1).ok_or("sequence overflow")?});
        Ok(json!({"cohort":c.id,"accepted":accepted}))
    }
    pub fn admit_frozen(&mut self,key:&str)->Result<Value>{
        let f=self.frozen_candidates.get(key).ok_or("candidate missing")?.clone();
        let receipt=self.custody_receipts.get(&f.cohort).ok_or("independent verdict missing")?;
        ensure(receipt.verdict.accepted&&receipt.verdict.candidate==key&&receipt.verdict.class_digest==self.class_fingerprint(&f.capability.class)?,"independent verdict rejected")?;
        let c=f.capability;let capkey=version_key(&c.id,c.version)?;
        ensure(!self.capabilities.contains_key(&capkey),"capability already admitted")?;
        let class=self.classes.get(&c.class).ok_or("class missing")?;
        // Public regression examples remain mandatory in addition to the private cohort.
        let mut evaluated=0u64;
        for prior in self.capabilities.values().filter(|p|p.family==c.family){
            let old=self.classes.get(&prior.class).ok_or("prior class missing")?;
            if old.input==class.input&&old.output==class.output{
                for sample in prior.training.iter().chain(prior.holdouts.iter()){
                    let (x,y)=self.example(sample,&c.family,&class.input,&class.output)?;
                    evaluated+=1;ensure(self.run_class(&c.class,&x)?==y,"prior public regression failed")?;
                }
            }
        }
        self.capabilities.insert(capkey.clone(),c);self.metrics.learned=self.metrics.learned.saturating_add(1);
        Ok(json!({"capability":capkey,"private_cohort":f.cohort,"public_regression_evaluations":evaluated}))
    }
}
