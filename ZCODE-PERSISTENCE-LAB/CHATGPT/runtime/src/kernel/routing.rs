//! Provider-neutral intelligence allocation. No provider calls or credential handling.
//! All accounting uses integer units, explicit timestamps and durable reservations.
use super::model::{Id,ensure,id};
use crate::Result;
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::collections::{BTreeMap,BTreeSet};

pub const DAY_MS:u64=86_400_000;
fn default_true()->bool{true}
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(tag="kind",rename_all="snake_case")]
pub enum Window {Fixed{period_ms:u64,anchor_ms:u64},Rolling{period_ms:u64},Lifetime}
impl Window {
    pub(crate) fn bounds(&self,now:u64)->Result<(u64,Option<u64>)>{
        match *self{
            Self::Lifetime=>Ok((0,None)),
            Self::Rolling{period_ms}=>{ensure(period_ms>0,"invalid rolling period")?;Ok((now.saturating_sub(period_ms-1),None))}
            Self::Fixed{period_ms,anchor_ms}=>{
                ensure(period_ms>0&&now>=anchor_ms,"invalid fixed window or clock before anchor")?;
                let start=now-(now-anchor_ms)%period_ms;
                Ok((start,Some(start.checked_add(period_ms).ok_or("window overflow")?)))
            }
        }
    }
}
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="snake_case")]
pub enum Unit{Requests,InputTokens,OutputTokens,ReasoningTokens,TotalTokens,MicroCurrency,Credits}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Pool{
    pub id:Id,pub unit:Unit,pub currency:Option<String>,pub window:Window,pub limit:u64,
    #[serde(default)] pub require_observation:bool,
    #[serde(default)] pub observation_ttl_ms:u64,
    pub frontier:Option<FrontierPolicy>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct FrontierPolicy{
    /// Fraction of the pool withheld for critical work, in basis points.
    pub emergency_bps:u16,
    /// Initial allowance plus linear release over a fixed reset window.
    pub opening_bps:u16,
    pub critical_priority:u16,
    pub minimum_uplift_bps:u16,
    /// Per-goal share of the whole daily pool, unless critical.
    pub per_goal_bps:u16,
}
#[derive(Clone,Debug,Serialize,Deserialize,Default)]
pub struct Usage{pub requests:u64,pub input:u64,pub output:u64,pub reasoning:u64,pub credits:u64}
#[derive(Clone,Debug,Serialize,Deserialize,Default)]
pub struct Tariff{
    pub currency:Option<String>,pub per_request_micro:u64,
    pub input_per_million_micro:u64,pub output_per_million_micro:u64,pub reasoning_per_million_micro:u64,
}
impl Tariff{
    fn charge(&self,u:&Usage)->Result<u64>{
        let mut n=u128::from(self.per_request_micro)*u128::from(u.requests);
        for (count,rate) in [(u.input,self.input_per_million_micro),(u.output,self.output_per_million_micro),(u.reasoning,self.reasoning_per_million_micro)]{
            let v=u128::from(count)*u128::from(rate);
            let q=v/1_000_000+u128::from(v%1_000_000!=0);
            n=n.checked_add(q).ok_or("cost overflow")?;
        }
        u64::try_from(n).map_err(|_|"cost overflow".into())
    }
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Profile{
    pub id:Id,pub provider:Id,pub account:Id,pub model:String,pub thinking:String,pub adapter:Id,
    pub enabled:bool,pub frontier:bool,pub capabilities:BTreeSet<String>,
    pub pools:BTreeSet<Id>,pub max_context:u64,pub max_output:u64,pub max_reasoning:u64,
    pub credit_ceiling:u64,pub tariff:Tariff,pub prior_success_bps:u16,pub prior_latency_ms:u64,
    pub max_parallel:u32,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Policy{
    pub version:u64,pub max_attempts:u32,pub quality_weight:u32,pub latency_weight:u32,
    pub scarcity_weight:u32,pub price_weight:u32,pub diversity_bonus:u32,
    pub failure_cooldown_ms:u64,pub quota_cooldown_ms:u64,pub reservation_ttl_ms:u64,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Config{
    pub policy:Policy,pub pools:BTreeMap<Id,Pool>,pub profiles:BTreeMap<Id,Profile>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Demand{
    pub id:Id,pub goal:Id,pub task:Id,pub family:Id,pub capabilities:BTreeSet<String>,
    pub input_tokens:u64,pub output_tokens:u64,pub reasoning_tokens:u64,
    #[serde(default)] pub adaptive_thinking:bool,
    #[serde(default)] pub minimum_reasoning_tokens:u64,
    pub priority:u16,pub uncertainty_bps:u16,pub information_gain_bps:u16,
    pub minimum_success_bps:u16,pub deadline_ms:Option<u64>,
    pub allowed_profiles:BTreeSet<Id>,pub allowed_providers:BTreeSet<Id>,
    #[serde(default)] pub excluded_providers:BTreeSet<Id>,
    pub allow_frontier:bool,pub independent_review:bool,
    #[serde(default="default_true")] pub prefer_native:bool,
    /// Optional extra pools for experiment/goal/task caps.
    pub extra_pools:BTreeSet<Id>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Observation{
    pub pool:Id,pub at_ms:u64,pub remaining:u64,pub window_start_ms:u64,
}
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="snake_case")]
pub enum Status{Reserved,Dispatched,Unknown,Settled,Released}
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="snake_case")]
pub enum Outcome{Completed,Rejected,Unavailable,RateLimited,Timeout,Cancelled,NotSent}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Reservation{
    pub id:Id,pub demand:Id,pub goal:Id,pub task:Id,pub family:Id,pub profile:Profile,
    pub policy_version:u64,pub created_ms:u64,pub started_ms:Option<u64>,pub ended_ms:Option<u64>,
    pub status:Status,pub ceiling:Usage,pub holds:BTreeMap<Id,u64>,pub actual:BTreeMap<Id,u64>,
    pub outcome:Option<Outcome>,pub receipt:Option<String>,pub latency_ms:Option<u64>,
    pub verified:Option<bool>,pub evidence:Option<Id>,pub estimated_success_bps:u16,
    pub task_generation:Option<u64>,pub agent:Option<Id>,pub incarnation:Option<u64>,
}
#[derive(Clone,Debug,Serialize,Deserialize,Default)]
pub struct Performance{pub verified:u64,pub passed:u64,pub latency_sum_ms:u64,pub failures:u64}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Decision{
    pub profile:Id,pub expected_success_bps:u16,pub expected_latency_ms:u64,
    pub holds:BTreeMap<Id,u64>,pub score:i128,pub uplift_bps:u16,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Recommendation{pub chosen:Option<Decision>,pub eligible:Vec<Decision>,pub excluded:BTreeMap<Id,String>,pub reason:String}
#[derive(Clone,Debug,Serialize,Deserialize,Default)]
pub struct Router{
    pub clock_ms:u64,pub config:Option<Config>,pub demands:BTreeMap<Id,Demand>,
    pub reservations:BTreeMap<Id,Reservation>,pub observations:BTreeMap<Id,Observation>,
    pub performance:BTreeMap<Id,BTreeMap<Id,Performance>>,pub cooldowns:BTreeMap<Id,u64>,
    pub provider_cooldowns:BTreeMap<Id,u64>,pub decisions:BTreeMap<Id,Recommendation>,
    #[serde(default)] pub planning:super::routing_planning::Planning,
    #[serde(default)] pub adapters:super::routing_adapter::AdapterState,
}
fn active(r:&Reservation)->bool{matches!(r.status,Status::Reserved|Status::Dispatched|Status::Unknown)}
fn bps(v:u64,b:u16)->u64{(u128::from(v)*u128::from(b)/10_000) as u64}
fn sum(a:u64,b:u64)->Result<u64>{a.checked_add(b).ok_or("budget sum overflow".into())}
pub(crate) fn amount(pool:&Pool,p:&Profile,u:&Usage)->Result<u64>{
    match pool.unit{
        Unit::Requests=>Ok(u.requests),Unit::InputTokens=>Ok(u.input),Unit::OutputTokens=>Ok(u.output),
        Unit::ReasoningTokens=>Ok(u.reasoning),Unit::TotalTokens=>sum(sum(u.input,u.output)?,u.reasoning),
        Unit::Credits=>Ok(u.credits),Unit::MicroCurrency=>{
            ensure(pool.currency.is_some()&&pool.currency==p.tariff.currency,"currency mismatch; no implicit conversion")?;
            p.tariff.charge(u)
        }
    }
}
impl Demand{
    pub(crate) fn usage_for(&self,p:&Profile)->Usage{
        Usage{requests:1,input:self.input_tokens,output:self.output_tokens,reasoning:if self.adaptive_thinking{self.reasoning_tokens.min(p.max_reasoning)}else{self.reasoning_tokens},credits:p.credit_ceiling}
    }
}
impl Router{
    pub fn is_empty(&self)->bool{self.config.is_none()&&self.demands.is_empty()&&self.reservations.is_empty()&&self.adapters.bindings.is_empty()&&self.planning.guard.is_none()}
    fn config(&self)->Result<&Config>{self.config.as_ref().ok_or("routing not configured".into())}
    pub fn configure(&mut self,c:Config)->Result<()>{
        ensure(c.policy.version>0&&c.policy.max_attempts>0&&c.policy.max_attempts<=64,"invalid routing policy")?;
        ensure(c.policy.failure_cooldown_ms<=DAY_MS&&c.policy.quota_cooldown_ms<=7*DAY_MS&&(1..=DAY_MS).contains(&c.policy.reservation_ttl_ms),"invalid cooldown or reservation expiry")?;
        ensure(!c.pools.is_empty()&&!c.profiles.is_empty()&&c.pools.len()<=4096&&c.profiles.len()<=4096,"invalid registry size")?;
        if let Some(old)=&self.config{
            ensure(c.policy.version>old.policy.version,"policy version must increase")?;
            ensure(old.profiles.keys().all(|k|c.profiles.contains_key(k)),"existing profiles must be disabled, not removed")?;
            for (key,p) in &old.pools{
                let next=c.pools.get(key).ok_or("existing pool cannot be removed")?;
                ensure(next.unit==p.unit&&next.currency==p.currency&&next.window==p.window,"pool accounting identity immutable")?;
            }
        }
        for (key,p) in &c.pools{
            id(key)?;ensure(key==&p.id&&p.limit>0,"invalid pool")?;
            match p.window{Window::Fixed{period_ms,..}|Window::Rolling{period_ms}=>ensure(period_ms>0,"zero window")?,Window::Lifetime=>{}}
            ensure((p.unit==Unit::MicroCurrency)==p.currency.is_some(),"currency only on money pool")?;
            ensure(!p.require_observation||p.observation_ttl_ms>0,"observation TTL required")?;
            if let Some(f)=&p.frontier{
                ensure(matches!(p.window,Window::Fixed{..})&&f.emergency_bps<=10_000&&f.opening_bps<=10_000&&f.minimum_uplift_bps<=10_000&&f.per_goal_bps<=10_000&&f.critical_priority<=1000,"invalid frontier policy")?;
            }
        }
        if c.policy.price_weight>0{
            let currencies:BTreeSet<_>=c.profiles.values().filter(|p|p.enabled).filter_map(|p|p.tariff.currency.as_ref()).collect();
            ensure(currencies.len()<=1&&c.profiles.values().filter(|p|p.enabled).all(|p|p.tariff.currency.is_some()||p.tariff.per_request_micro==0&&p.tariff.input_per_million_micro==0&&p.tariff.output_per_million_micro==0&&p.tariff.reasoning_per_million_micro==0),"price weighting requires a common currency")?;
        }
        for (key,p) in &c.profiles{
            id(key)?;id(&p.provider)?;id(&p.account)?;id(&p.adapter)?;
            ensure(key==&p.id&&!p.model.is_empty()&&!p.thinking.is_empty()&&p.max_context>0&&p.max_output>0&&p.max_parallel>0&&p.prior_success_bps<=10_000,"invalid profile")?;
            ensure(!p.pools.is_empty()&&p.pools.iter().all(|k|c.pools.contains_key(k)),"profile needs quota pools")?;
            ensure(!p.frontier||p.pools.iter().any(|k|c.pools[k].frontier.is_some()),"frontier profile needs protected pool")?;
            ensure(p.frontier||p.pools.iter().all(|k|c.pools[k].frontier.is_none()),"ordinary profile may not spend protected frontier pool")?;
            for k in &p.pools{amount(&c.pools[k],p,&Usage::default())?;}
            if let Some(old)=self.config.as_ref().and_then(|x|x.profiles.get(key)){
                ensure(old.provider==p.provider&&old.account==p.account&&old.model==p.model&&old.thinking==p.thinking&&old.adapter==p.adapter,"profile identity immutable; use a new ID")?;
                ensure(old.pools==p.pools,"profile quota membership immutable; use new profile and reconcile pool usage")?;
                ensure(!old.frontier||p.frontier,"frontier classification cannot be removed")?;
            }
        }
        self.config=Some(c);Ok(())
    }
    pub fn tick(&mut self,now:u64)->Result<()>{
        ensure(now>=self.clock_ms,"routing clock reversed")?;self.clock_ms=now;
        if let Some(c)=&self.config{for r in self.reservations.values_mut().filter(|r|r.status==Status::Reserved){if now.saturating_sub(r.created_ms)>=c.policy.reservation_ttl_ms{r.status=Status::Released;r.outcome=Some(Outcome::NotSent);r.ended_ms=Some(now);}}}Ok(())
    }
    pub fn observe(&mut self,o:Observation)->Result<()>{
        let p=self.config()?.pools.get(&o.pool).ok_or("pool missing")?;
        ensure(o.at_ms<=self.clock_ms&&p.window.bounds(o.at_ms)?.0==o.window_start_ms&&o.remaining<=p.limit,"invalid provider observation")?;
        if let Some(old)=self.observations.get(&o.pool){ensure(o.at_ms>old.at_ms,"stale quota observation")?;}
        self.observations.insert(o.pool.clone(),o);Ok(())
    }
    pub fn queue(&mut self,d:Demand)->Result<()>{
        let c=self.config()?;
        id(&d.id)?;id(&d.goal)?;id(&d.task)?;id(&d.family)?;
        ensure(!self.demands.contains_key(&d.id)&&d.priority<=1000&&d.uncertainty_bps<=10_000&&d.information_gain_bps<=10_000&&d.minimum_success_bps<=10_000&&d.output_tokens>0&&d.minimum_reasoning_tokens<=d.reasoning_tokens,"invalid demand")?;
        ensure(d.extra_pools.iter().all(|k|c.pools.contains_key(k)),"unknown demand quota pool")?;
        ensure(d.extra_pools.iter().all(|k|c.pools[k].frontier.is_none()),"frontier pools must be attached to frontier profiles")?;
        ensure(d.allowed_profiles.iter().all(|k|c.profiles.contains_key(k)),"unknown allowed profile")?;
        ensure(!self.demands.values().any(|x|x.task==d.task),"task already has routing demand")?;
        self.demands.insert(d.id.clone(),d);Ok(())
    }
    fn totals(&self,pool:&Pool,goal:Option<&str>)->Result<(u64,u64)>{
        let (start,_)=pool.window.bounds(self.clock_ms)?;let mut spent=0;let mut held=0;
        for r in self.reservations.values(){
            if goal.is_some_and(|g|g!=r.goal){continue;}
            if active(r){held=sum(held,*r.holds.get(&pool.id).unwrap_or(&0))?;}
            else if r.status==Status::Settled&&r.ended_ms.is_some_and(|t|t>=start){
                spent=sum(spent,*r.actual.get(&pool.id).unwrap_or(&0))?;
            }
        }Ok((spent,held))
    }
    pub fn available(&self,key:&str)->Result<u64>{
        let p=self.config()?.pools.get(key).ok_or("pool missing")?;
        let (spent,held)=self.totals(p,None)?;
        let local=p.limit.saturating_sub(spent.saturating_add(held));
        let (start,_)=p.window.bounds(self.clock_ms)?;
        let valid=self.observations.get(key).filter(|o|{
            let same_window=match p.window{Window::Fixed{..}=>o.window_start_ms==start,_=>true};
            same_window&&self.clock_ms.saturating_sub(o.at_ms)<=p.observation_ttl_ms
        });
        if let Some(o)=valid{
            let mut after=0u64;
            for r in self.reservations.values().filter(|r|r.status==Status::Settled&&r.ended_ms.is_some_and(|t|t>=o.at_ms)){
                after=sum(after,*r.actual.get(key).unwrap_or(&0))?;
            }
            // Conservative: count all live/unknown holds even if provider may already have counted them.
            Ok(local.min(o.remaining.saturating_sub(after.saturating_add(held))))
        }else if p.require_observation{Ok(0)}else{Ok(local)}
    }
    fn estimate(&self,p:&Profile,d:&Demand)->(u16,u64){
        let s=self.performance.get(&p.id).and_then(|x|x.get(&d.family));
        let Some(s)=s else{return (p.prior_success_bps,p.prior_latency_ms);};
        let denom=u128::from(s.verified)+8;
        let quality=((u128::from(s.passed)*10_000+u128::from(p.prior_success_bps)*8)/(denom+u128::from(s.failures))).min(10_000) as u16;
        let latency=((u128::from(s.latency_sum_ms)+u128::from(p.prior_latency_ms)*8)/denom).min(u128::from(u64::MAX)) as u64;
        (quality,latency)
    }
    fn holds(&self,p:&Profile,d:&Demand,u:&Usage)->Result<BTreeMap<Id,u64>>{
        let c=self.config()?;p.pools.union(&d.extra_pools).map(|k|{
            let pool=c.pools.get(k).ok_or("pool missing")?;
            Ok((k.clone(),amount(pool,p,u)?))
        }).collect()
    }
    fn frontier_check(&self,pool:&Pool,d:&Demand,charge:u64,uplift:u16)->Result<()>{
        let Some(f)=&pool.frontier else{return Ok(());};
        if d.priority>=f.critical_priority{return Ok(());}
        ensure(uplift>=f.minimum_uplift_bps,"insufficient frontier uplift")?;
        let (start,end)=pool.window.bounds(self.clock_ms)?;let end=end.ok_or("frontier requires fixed window")?;
        let elapsed=self.clock_ms.saturating_sub(start);
        let paced=super::frontier::paced_allowance(pool.limit,f,elapsed,end-start)?;
        let (spent,held)=self.totals(pool,None)?;
        ensure(charge<=paced.saturating_sub(spent.saturating_add(held)),"frontier paced or emergency reserve protected")?;
        let (spent,held)=self.totals(pool,Some(&d.goal))?;
        ensure(charge<=bps(pool.limit,f.per_goal_bps).saturating_sub(spent.saturating_add(held)),"frontier per-goal allocation cap")
    }
    pub fn recommend(&self,key:&str)->Result<Recommendation>{
        let c=self.config()?;let d=self.demands.get(key).ok_or("routing demand missing")?;
        let history:Vec<_>=self.reservations.values().filter(|r|r.demand==key).collect();
        if history.iter().any(|r|active(r)||r.verified==Some(true)||(r.status==Status::Settled&&r.outcome==Some(Outcome::Completed)&&r.verified.is_none())){
            return Ok(Recommendation{chosen:None,eligible:vec![],excluded:BTreeMap::new(),reason:"active, uncertain or already verified demand".into()});
        }
        if history.iter().filter(|r|r.status!=Status::Released).count()>=c.policy.max_attempts as usize||d.deadline_ms.is_some_and(|t|t<=self.clock_ms){
            return Ok(Recommendation{chosen:None,eligible:vec![],excluded:BTreeMap::new(),reason:"attempt budget or deadline exhausted".into()});
        }
        let usage_for=|p:&Profile|d.usage_for(p);
        let mut feasible=Vec::new();let mut excluded=BTreeMap::new();
        for (key,p) in &c.profiles{
            let result=(||->Result<(u16,u64,BTreeMap<Id,u64>)>{
                ensure(p.enabled&&(!p.frontier||d.allow_frontier),"profile disabled or frontier not allowed")?;
                ensure(d.allowed_profiles.is_empty()||d.allowed_profiles.contains(key),"profile not allowed")?;
                ensure((d.allowed_providers.is_empty()||d.allowed_providers.contains(&p.provider))&&!d.excluded_providers.contains(&p.provider),"provider not allowed")?;
                ensure(d.capabilities.is_subset(&p.capabilities),"missing capability")?;
                let usage=d.usage_for(p);
                ensure(sum(sum(usage.input,usage.output)?,usage.reasoning)?<=p.max_context&&usage.output<=p.max_output&&usage.reasoning<=p.max_reasoning&&usage.reasoning>=d.minimum_reasoning_tokens,"context or generation limit")?;
                ensure(self.cooldowns.get(key).copied().unwrap_or(0)<=self.clock_ms&&self.provider_cooldowns.get(&format!("{}:{}",p.provider,p.account)).copied().unwrap_or(0)<=self.clock_ms,"profile or account cooling down")?;
                ensure(self.reservations.values().filter(|r|active(r)&&r.profile.id==*key).count()<p.max_parallel as usize,"profile concurrency limit")?;
                ensure(!history.iter().any(|r|r.profile.id==*key&&(r.verified==Some(false)||r.outcome==Some(Outcome::Rejected))),"previous verified rejection; use different profile")?;
                if d.independent_review{ensure(!history.iter().any(|r|r.profile.provider==p.provider),"review requires different provider")?;}
                let (quality,latency)=self.estimate(p,d);
                ensure(d.deadline_ms.is_none_or(|t|self.clock_ms.saturating_add(latency)<=t),"expected latency exceeds deadline")?;
                let holds=self.holds(p,d,&usage_for(p))?;
                for (pool,charge) in &holds{ensure(*charge<=self.available(pool)?,"quota exhausted or observation unavailable")?;}
                Ok((quality,latency,holds))
            })();
            match result{Ok((q,l,h))=>feasible.push((key,p,q,l,h)),Err(e)=>{excluded.insert(key.clone(),e);}}
        }
        let ordinary=feasible.iter().filter(|(_,p,_,_,_)|!p.frontier).map(|x|x.2).max().unwrap_or(0);
        let mut eligible=Vec::new();
        for (key,p,q,l,h) in feasible{
            if q<d.minimum_success_bps{excluded.insert(key.clone(),"quality floor".into());continue;}
            let uplift=q.saturating_sub(ordinary);
            let gate=(||->Result<()>{
                for (pool,charge) in &h{self.frontier_check(&c.pools[pool],d,*charge,uplift)?;self.forecast_gate(d,pool,*charge)?;}Ok(())
            })();
            if let Err(e)=gate{excluded.insert(key.clone(),e);continue;}
            let mut scarcity=0i128;let mut opportunity=0i128;
            for (pool,charge) in &h{
                let avail=self.available(pool)?.max(1);
                scarcity+=i128::from(*charge)*10_000/i128::from(avail);opportunity+=self.opportunity_penalty(d,pool,*charge)?;
            }
            let price=p.tariff.charge(&usage_for(p))?;
            let diversity=if history.iter().any(|r|r.profile.provider==p.provider){0}else{c.policy.diversity_bonus};
            let value=i128::from(q)*(1000+i128::from(d.priority)+i128::from(d.information_gain_bps)/10+i128::from(d.uncertainty_bps)/10);
            // Price is only comparable within a common currency. The configured
            // pool scarcity is the cross-currency signal; price weight is optional.
            let score=value*i128::from(c.policy.quality_weight)/1000-i128::from(l)*i128::from(c.policy.latency_weight)
                -scarcity*i128::from(c.policy.scarcity_weight)-i128::from(price)*i128::from(c.policy.price_weight)+i128::from(diversity)-opportunity;
            eligible.push(Decision{profile:key.clone(),expected_success_bps:q,expected_latency_ms:l,holds:h,score:score.clamp(i128::from(i64::MIN),i128::from(i64::MAX)),uplift_bps:uplift});
        }
        eligible.sort_by(|a,b|b.score.cmp(&a.score).then_with(||a.profile.cmp(&b.profile)));
        Ok(Recommendation{chosen:eligible.first().cloned(),reason:if eligible.is_empty(){"wait_or_require_policy_review"}else{"ranked_feasible_profiles"}.into(),eligible,excluded})
    }
    pub fn reserve(&mut self,key:Id,demand:&str)->Result<Value>{
        id(&key)?;ensure(!self.reservations.contains_key(&key)&&!self.decisions.contains_key(&key),"reservation or decision exists")?;
        let rec=self.recommend(demand)?;
        let Some(choice)=rec.chosen.clone() else{self.decisions.insert(key,rec.clone());return Ok(json!({"reserved":false,"decision":rec}));};
        let c=self.config()?;let p=c.profiles[&choice.profile].clone();let d=self.demands[demand].clone();
        let version=c.policy.version;
        let ceiling=d.usage_for(&p);
        self.reservations.insert(key.clone(),Reservation{id:key.clone(),demand:demand.into(),goal:d.goal,task:d.task,family:d.family,profile:p,policy_version:version,created_ms:self.clock_ms,started_ms:None,ended_ms:None,status:Status::Reserved,ceiling,holds:choice.holds,actual:BTreeMap::new(),outcome:None,receipt:None,latency_ms:None,verified:None,evidence:None,estimated_success_bps:choice.expected_success_bps,task_generation:None,agent:None,incarnation:None});
        self.decisions.insert(key.clone(),rec.clone());
        Ok(json!({"reserved":true,"reservation":key,"decision":rec}))
    }
    pub fn dispatch(&mut self,key:&str,generation:u64,agent:Id,incarnation:u64)->Result<()>{
        let r=self.reservations.get(key).ok_or("reservation missing")?;
        ensure(r.status==Status::Reserved,"reservation not dispatchable")?;
        let d=&self.demands[&r.demand];
        ensure(d.deadline_ms.is_none_or(|x|x>self.clock_ms),"deadline expired")?;
        let c=self.config()?;
        let p=c.profiles.get(&r.profile.id).ok_or("profile removed")?;
        ensure(p.enabled&&c.policy.version==r.policy_version,"reservation policy changed; release and reroute")?;
        ensure(self.cooldowns.get(&p.id).copied().unwrap_or(0)<=self.clock_ms&&self.provider_cooldowns.get(&format!("{}:{}",p.provider,p.account)).copied().unwrap_or(0)<=self.clock_ms,"profile or account cooling down")?;
        for pool in r.holds.keys(){
            let (spent,held)=self.totals(&c.pools[pool],None)?;
            if matches!(c.pools[pool].window,Window::Fixed{..}){ensure(c.pools[pool].window.bounds(r.created_ms)?.0==c.pools[pool].window.bounds(self.clock_ms)?.0,"quota reset; release and reroute")?;}
            ensure(spent.saturating_add(held)<=c.pools[pool].limit,"budget reduced below reservations")?;
            let frontier=&c.pools[pool].frontier;
            if let Some(f)=frontier{if d.priority<f.critical_priority{
                let own=r.holds[pool];let remaining=self.available(pool)?.saturating_add(own);
                ensure(own<=remaining.saturating_sub(self.forecast_protection(pool)?),"future demand protection changed")?;
            }}
            let p=&c.pools[pool];
            let fresh=self.observations.get(pool).filter(|o|self.clock_ms.saturating_sub(o.at_ms)<=p.observation_ttl_ms&&(!matches!(p.window,Window::Fixed{..})||p.window.bounds(self.clock_ms).is_ok_and(|b|b.0==o.window_start_ms)));
            if p.require_observation||fresh.is_some(){
                let o=fresh.ok_or("fresh quota observation missing")?;
                ensure(self.clock_ms.saturating_sub(o.at_ms)<=p.observation_ttl_ms,"quota observation expired")?;
                ensure(!matches!(p.window,Window::Fixed{..})||o.window_start_ms==p.window.bounds(self.clock_ms)?.0,"quota reset requires new observation")?;
                let after=self.reservations.values().filter(|x|x.status==Status::Settled&&x.ended_ms.is_some_and(|t|t>=o.at_ms)).try_fold(0u64,|n,x|sum(n,*x.actual.get(pool).unwrap_or(&0)))?;
                ensure(after.saturating_add(held)<=o.remaining,"observed quota reduced below reservations")?;
            }
        }
        let r=self.reservations.get_mut(key).unwrap();r.status=Status::Dispatched;r.started_ms=Some(self.clock_ms);r.task_generation=Some(generation);r.agent=Some(agent);r.incarnation=Some(incarnation);Ok(())
    }
    pub fn unknown(&mut self,key:&str)->Result<()>{
        let r=self.reservations.get_mut(key).ok_or("reservation missing")?;
        ensure(r.status==Status::Dispatched||r.status==Status::Unknown,"not a dispatched reservation")?;
        r.status=Status::Unknown;Ok(())
    }
    pub fn release_unsent(&mut self,key:&str)->Result<()>{
        let r=self.reservations.get_mut(key).ok_or("reservation missing")?;
        ensure(r.status==Status::Reserved,"only undispatched reservations can be released")?;
        r.status=Status::Released;r.outcome=Some(Outcome::NotSent);r.ended_ms=Some(self.clock_ms);Ok(())
    }
    pub fn settle(&mut self,key:&str,u:Usage,outcome:Outcome,receipt:String,latency_ms:u64)->Result<Value>{
        ensure(!receipt.trim().is_empty()&&receipt.len()<=4096,"usage receipt required")?;
        let r=self.reservations.get(key).ok_or("reservation missing")?.clone();
        ensure(matches!(r.status,Status::Dispatched|Status::Unknown),"not an unsettled dispatch")?;
        ensure(outcome!=Outcome::NotSent||u.requests==0&&u.input==0&&u.output==0&&u.reasoning==0&&u.credits==0,"not-sent receipt contains usage")?;
        let c=self.config()?;
        let mut actual=BTreeMap::new();let mut overrun=Vec::new();
        for (k,hold) in &r.holds{
            let charge=amount(&c.pools[k],&r.profile,&u)?;
            if charge>*hold{overrun.push(k.clone());}actual.insert(k.clone(),charge);
        }
        let failure=c.policy.failure_cooldown_ms;let quota=c.policy.quota_cooldown_ms;
        let r=self.reservations.get_mut(key).unwrap();r.actual=actual;r.status=Status::Settled;r.outcome=Some(outcome.clone());r.receipt=Some(receipt);r.latency_ms=Some(latency_ms);r.ended_ms=Some(self.clock_ms);
        if outcome==Outcome::RateLimited{self.provider_cooldowns.insert(format!("{}:{}",r.profile.provider,r.profile.account),self.clock_ms.saturating_add(quota));}
        if matches!(outcome,Outcome::Unavailable|Outcome::Timeout){self.cooldowns.insert(r.profile.id.clone(),self.clock_ms.saturating_add(failure));}
        if !overrun.is_empty(){self.cooldowns.insert(r.profile.id.clone(),self.clock_ms.saturating_add(quota));}
        let p=self.performance.entry(r.profile.id.clone()).or_default().entry(r.family.clone()).or_default();
        if outcome!=Outcome::Completed{p.failures=p.failures.saturating_add(1);}
        Ok(json!({"settled":key,"overrun_pools":overrun}))
    }
    pub fn reject_unusable_output(&mut self,key:&str)->Result<()>{
        let r=self.reservations.get_mut(key).ok_or("reservation missing")?;
        ensure(r.status==Status::Settled&&r.outcome==Some(Outcome::Completed)&&r.verified.is_none(),"not a completed unverified result")?;
        r.outcome=Some(Outcome::Rejected);
        let p=self.performance.entry(r.profile.id.clone()).or_default().entry(r.family.clone()).or_default();p.failures=p.failures.saturating_add(1);Ok(())
    }
    pub fn record_verified(&mut self,key:&str,accepted:bool,evidence:Id)->Result<()>{
        let r=self.reservations.get_mut(key).ok_or("reservation missing")?;
        ensure(r.status==Status::Settled&&r.outcome==Some(Outcome::Completed)&&r.verified.is_none(),"not eligible for verified feedback")?;
        r.verified=Some(accepted);r.evidence=Some(evidence);
        let p=self.performance.entry(r.profile.id.clone()).or_default().entry(r.family.clone()).or_default();
        p.verified=p.verified.saturating_add(1);if accepted{p.passed=p.passed.saturating_add(1);}
        p.latency_sum_ms=p.latency_sum_ms.saturating_add(r.latency_ms.unwrap_or(0));Ok(())
    }
    pub fn reconcile_stale(&mut self,live:&BTreeSet<(Id,u64)>){
        for r in self.reservations.values_mut().filter(|r|r.status==Status::Dispatched){
            if !r.task_generation.is_some_and(|g|live.contains(&(r.task.clone(),g))){r.status=Status::Unknown;}
        }
    }
    pub fn cancel_goal(&mut self,goal:&str){
        for r in self.reservations.values_mut().filter(|r|r.goal==goal){
            if r.status==Status::Reserved{r.status=Status::Released;r.outcome=Some(Outcome::Cancelled);r.ended_ms=Some(self.clock_ms);}
            else if r.status==Status::Dispatched{r.status=Status::Unknown;}
        }
    }
    /// Adapter-facing read-only polling plan. No network requests are made.
    pub fn quota_refresh_plan(&self)->Result<Value>{
        let c=self.config()?;let mut due=vec![];
        for (key,p) in &c.pools{
            if !p.require_observation&&p.observation_ttl_ms==0{continue;}
            let (start,end)=p.window.bounds(self.clock_ms)?;
            let o=self.observations.get(key);
            let stale=o.is_none_or(|o|self.clock_ms.saturating_sub(o.at_ms)>=p.observation_ttl_ms||matches!(p.window,Window::Fixed{..})&&o.window_start_ms!=start);
            if !stale{continue;}
            let adapters:BTreeSet<_>=c.profiles.values().filter(|x|x.enabled&&x.pools.contains(key)).map(|x|x.adapter.clone()).collect();
            if adapters.is_empty(){continue;}
            due.push(json!({"pool":key,"unit":p.unit,"currency":p.currency,"window_start_ms":start,"reset_ms":end,"eligible_adapters":adapters,"required":p.require_observation}));
        }
        Ok(json!({"clock_ms":self.clock_ms,"observations_due":due}))
    }
    pub fn budget_report(&self)->Result<Value>{
        let c=self.config()?;let mut pools=BTreeMap::new();
        for (key,p) in &c.pools{
            let (spent,held)=self.totals(p,None)?;let (start,end)=p.window.bounds(self.clock_ms)?;
            pools.insert(key,json!({"unit":p.unit,"currency":p.currency,"limit":p.limit,"spent":spent,"reserved_or_unknown":held,"available":self.available(key)?,"window_start_ms":start,"reset_ms":end,"observed":self.observations.get(key),"frontier":p.frontier,"forecast_protected":self.forecast_protection(key)?}));
        }
        Ok(json!({"clock_ms":self.clock_ms,"policy_version":c.policy.version,"pools":pools}))
    }
}
