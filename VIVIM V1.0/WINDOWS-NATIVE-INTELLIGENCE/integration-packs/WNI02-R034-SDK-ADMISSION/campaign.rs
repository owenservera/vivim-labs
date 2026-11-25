//! In-memory campaign coordination: bounded leases, isolated task IDs, evidence and independent review.
//! @hq:context Scale experimental work without equating queued jobs with running AI agents.
//! @hq:invariant Reports are proposals; no report can promote itself.
//! @hq:why Separate logical experimental scale from actual admitted concurrency.
//! @hq:todo Integrate authenticated actor identity, durable leases and incumbent Chronos time.
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task { pub id: String, pub source: String, pub objective: String }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lease { pub task: Task, pub worker: String, pub token: u64, pub expires: u64 }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report { pub evidence: Vec<String>, pub observation: String, pub recommendation: String }
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error { Invalid, Duplicate, Full, Unknown, Stale, SelfReview, AlreadyReviewed, Overflow }
#[derive(Clone, Debug)]
struct ResultRow { worker: String, report: Report, review: Option<(String,bool)> }

/// @hq:context Coordinate pending, active, completed and independently reviewed task records in memory.
/// @hq:invariant Lease expiry returns work to the pending queue; obsolete tokens cannot submit.
pub struct Campaign {
    tasks: BTreeMap<String,Task>, pending: VecDeque<String>, active: BTreeMap<String,Lease>,
    completed: BTreeMap<String,ResultRow>, limit: usize, next: u64, clock: u64,
}
fn valid(s:&str,max:usize)->bool{!s.trim().is_empty()&&s.len()<=max&&!s.chars().any(char::is_control)}
impl Campaign {
    /// @hq:context Admit unique bounded task definitions and an explicit concurrency limit.
    pub fn new(tasks:Vec<Task>,limit:usize)->Result<Self,Error>{
        if tasks.is_empty()||tasks.len()>10000||limit==0||limit>1000{return Err(Error::Invalid)}
        let mut map=BTreeMap::new();let mut pending=VecDeque::new();
        for t in tasks{
            if !valid(&t.id,96)||!valid(&t.source,512)||!valid(&t.objective,2048){return Err(Error::Invalid)}
            if map.contains_key(&t.id){return Err(Error::Duplicate)}
            pending.push_back(t.id.clone());map.insert(t.id.clone(),t);
        }
        Ok(Self{tasks:map,pending,active:BTreeMap::new(),completed:BTreeMap::new(),limit,next:1,clock:0})
    }
    /// @hq:invariant Supplied time cannot move backward inside this campaign.
    pub(crate) fn advance(&mut self,now:u64)->Result<(),Error>{
        if now<self.clock{return Err(Error::Stale)}
        self.clock=now;
        let expired:Vec<_>=self.active.iter().filter(|(_,v)|v.expires<=now).map(|(k,_)|k.clone()).collect();
        for id in expired{self.active.remove(&id);self.pending.push_back(id);}
        Ok(())
    }
    /// @hq:invariant One active lease per worker and no more than the configured limit.
    pub fn claim(&mut self,worker:&str,now:u64,ttl:u64)->Result<Option<Lease>,Error>{
        self.claim_selected(worker,now,ttl,None)
    }
    // Resource admission selects from the same pending queue; no competing campaign.
    pub(crate) fn pending_ids(&self)->impl Iterator<Item=&str>{self.pending.iter().map(String::as_str)}
    pub(crate) fn abandon(&mut self,lease:&Lease){
        if self.active.get(&lease.task.id)==Some(lease){
            self.active.remove(&lease.task.id);
            self.pending.push_back(lease.task.id.clone());
        }
    }
    pub(crate) fn claim_selected(&mut self,worker:&str,now:u64,ttl:u64,selected:Option<&str>)->Result<Option<Lease>,Error>{
        if !valid(worker,96)||ttl==0{return Err(Error::Invalid)}
        self.advance(now)?;
        if self.active.len()>=self.limit||self.active.values().any(|l|l.worker==worker){return Err(Error::Full)}
        let expires=now.checked_add(ttl).ok_or(Error::Overflow)?;
        let next=self.next.checked_add(1).ok_or(Error::Overflow)?;
        let id=match selected {
            Some(id)=>{let Some(pos)=self.pending.iter().position(|v|v==id) else{return Ok(None)}; self.pending.remove(pos)},
            None=>self.pending.pop_front()
        };
        let Some(id)=id else{return Ok(None)};
        let l=Lease{task:self.tasks[&id].clone(),worker:worker.into(),token:self.next,expires};
        self.next=next;self.active.insert(id,l.clone());Ok(Some(l))
    }
    /// @hq:invariant The submitted lease must match the current token and unexpired task.
    pub fn submit(&mut self,lease:&Lease,now:u64,report:Report)->Result<(),Error>{
        self.advance(now)?;
        if self.active.get(&lease.task.id)!=Some(lease){return Err(Error::Stale)}
        if report.evidence.is_empty()||report.evidence.len()>32||report.evidence.iter().any(|e|!valid(e,512))
            ||!valid(&report.observation,4096)||!valid(&report.recommendation,4096){return Err(Error::Invalid)}
        self.active.remove(&lease.task.id);
        self.completed.insert(lease.task.id.clone(),ResultRow{worker:lease.worker.clone(),report,review:None});Ok(())
    }
    /// @hq:why Keep a worker from accepting its own report; string identity is not authentication.
    pub fn review(&mut self,id:&str,reviewer:&str,accepted:bool)->Result<(),Error>{
        if !valid(reviewer,96){return Err(Error::Invalid)}
        let row=self.completed.get_mut(id).ok_or(Error::Unknown)?;
        if row.worker==reviewer{return Err(Error::SelfReview)}
        if row.review.is_some(){return Err(Error::AlreadyReviewed)}
        row.review=Some((reviewer.into(),accepted));Ok(())
    }
    pub fn counts(&self)->(usize,usize,usize,usize){
        (self.pending.len(),self.active.len(),self.completed.len(),self.completed.values().filter(|r|r.review.is_some()).count())
    }
    pub fn reports(&self)->impl Iterator<Item=(&str,&Report,Option<bool>)>{
        self.completed.iter().map(|(id,r)|(id.as_str(),&r.report,r.review.as_ref().map(|(_,v)|*v)))
    }
    pub fn evidence_refs(&self)->BTreeSet<&str>{
        self.completed.values().flat_map(|r|r.report.evidence.iter().map(String::as_str)).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn task(i:usize)->Task{Task{id:format!("t{i}"),source:"actual-source-path".into(),objective:"inspect".into()}}
    fn report()->Report{Report{evidence:vec!["receipt-reference".into()],observation:"observed".into(),recommendation:"evaluate".into()}}
    #[test]
    fn thousand_logical_tasks_bounded_leases(){
        let mut c=Campaign::new((0..1000).map(task).collect(),4).unwrap();
        let a:Vec<_>=(0..4).map(|i|c.claim(&format!("worker{i}"),0,10).unwrap().unwrap()).collect();
        assert_eq!(c.counts(),(996,4,0,0));assert_eq!(c.claim("extra",0,10),Err(Error::Full));
        for l in &a{c.submit(l,1,report()).unwrap();}
        assert_eq!(c.counts(),(996,0,4,0));
        assert_eq!(c.review(&a[0].task.id,&a[0].worker,true),Err(Error::SelfReview));
        c.review(&a[0].task.id,"independent-reviewer",true).unwrap();
        assert_eq!(c.counts(),(996,0,4,1));
    }
    #[test]
    fn expired_worker_cannot_submit(){
        let mut c=Campaign::new(vec![task(0)],1).unwrap();
        let old=c.claim("a",0,5).unwrap().unwrap();
        let new=c.claim("b",5,5).unwrap().unwrap();
        assert_ne!(old.token,new.token);assert_eq!(c.submit(&old,5,report()),Err(Error::Stale));
        c.submit(&new,6,report()).unwrap();assert_eq!(c.counts(),(0,0,1,0));
    }
}
