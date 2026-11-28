//! Durable effect intent and reconciliation. This module does not perform I/O
//! effects; the trusted host owns adapters and supplies independently checkable
//! receipts. An uncertain effect fences retries until explicitly reconciled.
use super::model::*;
use crate::{digest,Result};
use serde_json::{Value,json};

impl World {
    pub fn has_abandoned_effect(&self,task:&str)->bool{self.effects.values().any(|e|e.task==task&&e.phase==EffectPhase::Abandoned)}
    pub fn has_uncertain_effect(&self,task:&str)->bool{
        self.effects.values().any(|e|e.task==task&&matches!(e.phase,EffectPhase::Prepared|EffectPhase::Unknown))
    }
    pub fn mark_prepared_unknown(&mut self,task:&str,reason:&str){
        let seq=self.sequence.saturating_add(1);
        for e in self.effects.values_mut().filter(|e|e.task==task&&e.phase==EffectPhase::Prepared){
            e.phase=EffectPhase::Unknown;e.history.push(EffectEvent{phase:EffectPhase::Unknown,sequence:seq,reason:reason.into(),receipt:None});
        }
    }
    pub fn prepare_effect(&mut self,task:&str,generation:u64,key:Id,payload:Value,oracle:Oracle)->Result<Value>{
        id(&key)?;oracle.validate(0)?;bounded(&payload)?;
        let t=self.tasks.get(task).ok_or("task missing")?;
        ensure(t.phase==TaskPhase::Running&&t.generation==generation&&t.lease.as_ref().is_some_and(|l|self.clock_ms<l.deadline_ms),"effect requires current lease")?;
        ensure(self.goals.get(&t.spec.goal).is_some_and(|g|g.phase==GoalPhase::Open),"goal closed")?;
        let effect_id=format!("effect-{}",digest(&json!([task,&key])));
        let hash=digest(&payload);let oracle_hash=digest(&serde_json::to_value(&oracle).map_err(|e|e.to_string())?);
        if let Some(existing)=self.effects.get(&effect_id){
            ensure(existing.payload==hash&&digest(&serde_json::to_value(&existing.oracle).map_err(|e|e.to_string())?)==oracle_hash,"stable effect key conflict")?;
            match existing.phase{
                EffectPhase::Committed=>return Ok(json!({"effect":effect_id,"phase":"committed","receipt":existing.receipt,"execute":false})),
                EffectPhase::Prepared|EffectPhase::Unknown=>return Ok(json!({"effect":effect_id,"phase":existing.phase,"execute":false})),
                EffectPhase::Abandoned=>return Err("effect abandoned".into()),
                EffectPhase::Absent|EffectPhase::Proposed=>{}
            }
        }
        let payload=self.put_payload(payload)?;let sequence=self.sequence.saturating_add(1);
        let e=self.effects.entry(effect_id.clone()).or_insert_with(||Effect{id:effect_id.clone(),task:task.into(),generation,key,payload:payload.clone(),phase:EffectPhase::Proposed,oracle,receipt:None,sequence,history:vec![]});
        e.generation=generation;e.phase=EffectPhase::Prepared;e.receipt=None;
        e.history.push(EffectEvent{phase:EffectPhase::Prepared,sequence,reason:"host admitted effect intent".into(),receipt:None});
        Ok(json!({"effect":effect_id,"phase":"prepared","intent_only":true,"execute":false}))
    }
    pub fn effect_unknown(&mut self,effect:&str,reason:String)->Result<()>{
        ensure(!reason.trim().is_empty(),"reason required")?;
        let e=self.effects.get_mut(effect).ok_or("effect missing")?;
        ensure(matches!(e.phase,EffectPhase::Prepared|EffectPhase::Unknown),"effect not outstanding")?;
        e.phase=EffectPhase::Unknown;e.history.push(EffectEvent{phase:EffectPhase::Unknown,sequence:self.sequence.saturating_add(1),reason,receipt:None});Ok(())
    }
    pub fn resolve_effect(&mut self,effect:&str,outcome:EffectPhase,receipt:Value)->Result<Value>{
        ensure(matches!(outcome,EffectPhase::Committed|EffectPhase::Absent|EffectPhase::Abandoned),"invalid reconciliation outcome")?;
        let e=self.effects.get(effect).ok_or("effect missing")?.clone();
        let hash=digest(&receipt);
        if e.phase==outcome&&e.receipt.as_ref()==Some(&hash){return Ok(json!({"effect":effect,"phase":outcome,"receipt":hash}));}
        ensure(matches!(e.phase,EffectPhase::Prepared|EffectPhase::Unknown),"effect not outstanding")?;
        let payload=self.payload(&e.payload)?;
        let input=json!({"effect":effect,"task":e.task,"generation":e.generation,"payload":payload,"outcome":outcome});
        ensure(receipt.get("effect").and_then(Value::as_str)==Some(effect)&&receipt.get("outcome")==Some(&serde_json::to_value(&outcome).map_err(|e|e.to_string())?),"receipt scope or outcome mismatch")?;
        ensure(e.oracle.evaluate(&input,&receipt)?,"effect receipt rejected")?;
        let receipt=self.put_payload(receipt)?;let x=self.effects.get_mut(effect).unwrap();
        x.phase=outcome.clone();x.receipt=Some(receipt.clone());
        x.history.push(EffectEvent{phase:outcome.clone(),sequence:self.sequence.saturating_add(1),reason:"host reconciled receipt".into(),receipt:Some(receipt.clone())});
        if outcome==EffectPhase::Abandoned{
            self.mark_prepared_unknown(&e.task,"effect abandoned");
            let t=self.tasks.get_mut(&e.task).ok_or("task missing")?;
            if t.phase==TaskPhase::Running{
                if let Some(l)=t.lease.take(){if let Some(a)=self.agents.get_mut(&l.worker){
                    if let Some(i)=a.incarnations.last_mut(){if i.number==l.incarnation&&i.ended_ms.is_none(){i.ended_ms=Some(self.clock_ms);i.reason=Some("effect abandoned".into());}}
                    if a.phase!=AgentPhase::Archived{a.phase=AgentPhase::Recovering;}
                }}
                t.phase=TaskPhase::Reconciling;
            }
        }
        if !self.has_uncertain_effect(&e.task){
            let abandoned=self.effects.values().any(|x|x.task==e.task&&x.phase==EffectPhase::Abandoned);
            let t=self.tasks.get_mut(&e.task).ok_or("task missing")?;
            if t.phase==TaskPhase::Reconciling{
                t.phase=if abandoned{TaskPhase::Blocked}else if t.attempts>=t.spec.max_attempts{TaskPhase::Exhausted}else{TaskPhase::Pending};
                t.reason=Some("effect reconciliation completed".into());
            }
        }
        Ok(json!({"effect":effect,"phase":outcome,"receipt":receipt}))
    }
}
