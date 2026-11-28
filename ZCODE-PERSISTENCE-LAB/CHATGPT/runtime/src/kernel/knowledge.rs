//! Evidence-backed assimilation of recurring typed artifacts. Rules are explicit
//! semantic contracts, not permission to execute arbitrary external commands.
use super::model::*;
use crate::{digest,Result};
use serde_json::{Value,json};
use std::collections::{BTreeMap,BTreeSet};
impl World{
    pub fn register_knowledge_rule(&mut self,rule:KnowledgeRule)->Result<Id>{
        let key=version_key(&rule.id,rule.version)?;id(&rule.family)?;
        ensure(!self.knowledge_rules.contains_key(&key)&&(2..=128).contains(&rule.minimum_examples)&&(1..=128).contains(&rule.max_publications),"invalid knowledge rule")?;
        self.knowledge_rules.insert(key.clone(),rule);Ok(key)
    }
    fn recurring_artifacts(&self,rule:&KnowledgeRule)->BTreeMap<Digest,Vec<Id>>{
        let mut grouped:BTreeMap<Digest,Vec<Id>>=BTreeMap::new();
        let mut inputs:BTreeMap<Digest,BTreeSet<Digest>>=BTreeMap::new();
        for (id,a) in &self.artifacts{
            let Some(task)=self.tasks.get(&a.task) else{continue;};
            if task.phase!=TaskPhase::Verified||task.spec.family.as_ref()!=Some(&rule.family){continue;}
            let Some(e)=self.evidence.get(&a.evidence) else{continue;};
            if !e.accepted||e.candidate!=a.payload{continue;}
            if !self.payload(&a.payload).is_ok_and(|v|rule.output.accepts(v)){continue;}
            if inputs.entry(a.payload.clone()).or_default().insert(e.input.clone()){
                grouped.entry(a.payload.clone()).or_default().push(id.clone());
            }
        }
        grouped.retain(|_,examples|examples.len()>=rule.minimum_examples);grouped
    }
    pub fn ready_knowledge(&self)->Vec<(Id,Digest)>{
        let mut result=Vec::new();
        for (key,rule) in &self.knowledge_rules{
            let prior:Vec<_>=self.knowledge_publications.values().filter(|p|&p.rule==key).collect();
            if prior.len()>=rule.max_publications{continue;}
            let available=rule.max_publications-prior.len();
            let seen:BTreeSet<_>=prior.iter().map(|p|&p.payload).collect();
            for (payload,_) in self.recurring_artifacts(rule).into_iter().filter(|(p,_)|!seen.contains(p)).take(available){
                result.push((key.clone(),payload));
            }
        }
        result
    }
    pub fn assimilate_recurring(&mut self,rule:&str,payload:&str)->Result<Value>{
        ensure(self.ready_knowledge().iter().any(|(r,p)|r==rule&&p==payload),"knowledge publication not ready")?;
        let spec=self.knowledge_rules.get(rule).ok_or("rule missing")?.clone();
        let examples=self.recurring_artifacts(&spec).remove(payload).ok_or("examples missing")?;
        let name=format!("knowledge-{}",digest(&json!([rule,payload])));
        let evidence:Vec<_>=examples.iter().map(|a|self.artifacts[a].evidence.clone()).collect();
        let first=examples.first().ok_or("no artifact")?.clone();
        // Reusable artifacts can be a typed procedure description, repair,
        // coordination plan, context map, fact or proof. Executability requires
        // a separately admitted class, never inferred merely from a label.
        let k=self.publish_knowledge(Knowledge{id:name,version:1,kind:spec.kind,family:Some(spec.family),artifacts:vec![first],class:None,evidence,supersedes:None,enabled:true})?;
        self.knowledge_publications.insert(k.clone(),KnowledgePublication{rule:rule.into(),knowledge:k.clone(),payload:payload.into(),examples,sequence:self.sequence.checked_add(1).ok_or("sequence overflow")?});
        Ok(json!({"knowledge":k,"payload":payload}))
    }
    pub fn select_knowledge(&self,family:Option<&str>,kind:&KnowledgeKind,shape:&Shape)->Result<&Value>{
        let family=family.ok_or("knowledge selection requires declared family")?;
        let mut candidates:Vec<_>=self.knowledge.iter().filter(|(_,k)|k.enabled&&k.family.as_deref()==Some(family)&&&k.kind==kind&&k.artifacts.len()==1)
            .filter_map(|(key,k)|self.knowledge_value(key,Some(family)).ok().filter(|v|shape.accepts(v)).map(|v|(k.evidence.len(),k.version,key,v))).collect();
        candidates.sort_by(|a,b|(b.0,b.1,b.2).cmp(&(a.0,a.1,a.2)));
        // Different verified answers for the same family and kind are ambiguous.
        // The caller must pin a version rather than silently choosing a fact.
        let first=candidates.first().ok_or("no applicable knowledge")?.3;
        ensure(candidates.iter().all(|x|x.3==first),"ambiguous knowledge selection; pin a version")?;
        Ok(first)
    }
}
