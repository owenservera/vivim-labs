//! Durable provider-neutral runtime. No external effects or AI adapters.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{collections::{BTreeMap, BTreeSet}, fs::{self, File, OpenOptions}, io::Write, path::{Path, PathBuf}, time::{SystemTime, UNIX_EPOCH}};

pub type Result<T> = std::result::Result<T, String>;
fn err(e: impl std::fmt::Display) -> String { e.to_string() }
fn require(ok: bool, reason: &str) -> Result<()> { if ok { Ok(()) } else { Err(reason.into()) } }
fn valid_id(id: &str) -> Result<()> {
    require(!id.is_empty() && id.len() <= 128 && id.bytes().all(|b| b.is_ascii_alphanumeric() || b"_-.:".contains(&b)), "invalid identifier")
}
fn hash(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }
pub fn digest(v: &Value) -> String { hash(&serde_json::to_vec(v).expect("JSON value serialization")) }
fn is_hash(s: &str) -> bool { s.len()==64 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all="snake_case")]
pub enum Operator { Observe, Discover, Reason, Decompose, Plan, Construct, Execute, Experiment, Challenge, Verify, Repair, Synthesize, Optimize, Delegate, Learn }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Class { pub id: String, pub version: u64, pub operators: Vec<Operator>, pub capabilities: Vec<String> }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Goal { pub id: String, pub objective: String, pub status: GoalStatus, pub claim_budget: u64, pub claims: u64, pub sealed: bool }
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all="snake_case")]
pub enum GoalStatus { Open, Complete, Cancelled }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context { pub id: String, pub artifacts: Vec<String>, pub data: Value }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent { pub id: String, pub goal: String, pub class: String, pub parent: Option<String>, pub incarnation: u64, pub active: bool, pub context: String }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lease { pub agent: String, pub incarnation: u64, pub generation: u64, pub expires: u64 }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String, pub goal: String, pub objective: String, pub depends: Vec<String>,
    pub input: Value, pub expected: String, pub capability: Option<String>, pub max_attempts: u64,
    pub attempts: u64, pub generation: u64, pub lease: Option<Lease>, pub checkpoint: Option<Value>,
    pub proposal: Option<Value>, pub artifact: Option<String>, pub status: TaskStatus
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all="snake_case")]
pub enum TaskStatus { Pending, Running, Submitted, Committed, Exhausted, Cancelled }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artifact { pub id: String, pub task: String, pub generation: u64, pub digest: String, pub value: Value }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Evidence { pub task: String, pub generation: u64, pub expected: String, pub actual: String, pub accepted: bool }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message { pub id: String, pub from: String, pub to: String, pub body: Value, pub acknowledged: bool }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Capability { pub id: String, pub version: u64, pub program: Expr, pub examples: Vec<String>, pub active: bool }

/// Bounded, total (or explicit error) JSON computation, without I/O.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="op", rename_all="snake_case")]
pub enum Expr {
    Input, Const { value: Value }, Field { source: Box<Expr>, key: String },
    Array { items: Vec<Expr> }, Add { left: Box<Expr>, right: Box<Expr> },
    Multiply { left: Box<Expr>, right: Box<Expr> }, Concat { items: Vec<Expr> },
    Equal { left: Box<Expr>, right: Box<Expr> }, Sha256 { value: Box<Expr> },
    If { condition: Box<Expr>, yes: Box<Expr>, no: Box<Expr> }
}
impl Expr {
    pub fn eval(&self, input: &Value) -> Result<Value> {
        fn run(e: &Expr, input: &Value, fuel: &mut usize, depth: usize) -> Result<Value> {
            require(*fuel > 0 && depth <= 64, "program limit exceeded")?;
            *fuel -= 1;
            let v = match e {
                Expr::Input => input.clone(),
                Expr::Const {value} => value.clone(),
                Expr::Field {source,key} => run(source,input,fuel,depth+1)?.get(key).cloned().ok_or("field missing")?,
                Expr::Array {items} => Value::Array(items.iter().map(|i| run(i,input,fuel,depth+1)).collect::<Result<Vec<_>>>()?),
                Expr::Add {left,right} | Expr::Multiply {left,right} => {
                    let l=run(left,input,fuel,depth+1)?.as_i64().ok_or("expected i64")?;
                    let r=run(right,input,fuel,depth+1)?.as_i64().ok_or("expected i64")?;
                    Value::from(match e { Expr::Add{..}=>l.checked_add(r), _=>l.checked_mul(r) }.ok_or("integer overflow")?)
                }
                Expr::Concat {items} => {
                    let mut s=String::new();
                    for i in items { let v=run(i,input,fuel,depth+1)?; s.push_str(v.as_str().ok_or("expected string")?); require(s.len()<=1_048_576,"result too large")?; }
                    Value::String(s)
                }
                Expr::Equal {left,right} => Value::Bool(run(left,input,fuel,depth+1)?==run(right,input,fuel,depth+1)?),
                Expr::Sha256 {value} => Value::String(digest(&run(value,input,fuel,depth+1)?)),
                Expr::If {condition,yes,no} => {
                    let b=run(condition,input,fuel,depth+1)?.as_bool().ok_or("expected boolean")?;
                    run(if b {yes} else {no},input,fuel,depth+1)?
                }
            };
            require(serde_json::to_vec(&v).map_err(err)?.len()<=1_048_576,"value too large")?;
            Ok(v)
        }
        run(self,input,&mut 4096,0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct State {
    pub clock: u64, pub goals: BTreeMap<String,Goal>, pub classes: BTreeMap<String,Class>,
    pub agents: BTreeMap<String,Agent>, pub contexts: BTreeMap<String,Context>,
    pub tasks: BTreeMap<String,Task>, pub artifacts: BTreeMap<String,Artifact>,
    pub evidence: Vec<Evidence>, pub messages: BTreeMap<String,Message>,
    pub capabilities: BTreeMap<String,Capability>
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="cmd",rename_all="snake_case")]
pub enum Command {
    Goal {id:String, objective:String, claim_budget:u64},
    Class {id:String, version:u64, operators:Vec<Operator>, capabilities:Vec<String>},
    Context {id:String, artifacts:Vec<String>, data:Value},
    Agent {id:String, goal:String, class:String, context:String, parent:Option<String>},
    Activate {agent:String},
    Retire {agent:String},
    Task {id:String, goal:String, objective:String, depends:Vec<String>, input:Value, expected:String, capability:Option<String>, max_attempts:u64},
    Seal {goal:String},
    Claim {task:String, agent:String, ttl:u64},
    Checkpoint {task:String, agent:String, generation:u64, data:Value},
    Submit {task:String, agent:String, generation:u64, value:Value},
    Verify {task:String},
    Execute {task:String, agent:String, generation:u64},
    Tick {now:u64},
    Send {id:String, from:String, to:String, body:Value},
    Ack {id:String, agent:String},
    Promote {id:String, version:u64, program:Expr, examples:Vec<String>},
    Disable {capability:String},
    Cancel {goal:String}
}
fn key(id:&str, version:u64) -> Result<String> { valid_id(id)?; require(version>0,"version must be positive")?; Ok(format!("{id}:{version}")) }
impl State {
    fn open_goal(&self,id:&str)->Result<&Goal> {
        let g=self.goals.get(id).ok_or("goal missing")?; require(g.status==GoalStatus::Open,"goal not open")?; Ok(g)
    }
    pub fn ready(&self)->Vec<String> {
        self.tasks.values().filter(|t| t.status==TaskStatus::Pending && t.attempts<t.max_attempts
            && self.goals.get(&t.goal).is_some_and(|g|g.status==GoalStatus::Open && g.claims<g.claim_budget)
            && t.depends.iter().all(|d| self.tasks.get(d).is_some_and(|x|x.status==TaskStatus::Committed)))
            .map(|t|t.id.clone()).collect()
    }
    fn checked_lease(&self, task:&str, agent:&str, generation:u64)->Result<&Task> {
        let t=self.tasks.get(task).ok_or("task missing")?; self.open_goal(&t.goal)?;
        let l=t.lease.as_ref().ok_or("no lease")?;
        let a=self.agents.get(agent).ok_or("agent missing")?;
        require(t.status==TaskStatus::Running && a.active && l.agent==agent && l.incarnation==a.incarnation && l.generation==generation && self.clock<l.expires,"stale or expired lease")?;
        Ok(t)
    }
    fn release(t:&mut Task) {
        t.lease=None; t.proposal=None;
        t.status=if t.attempts>=t.max_attempts {TaskStatus::Exhausted} else {TaskStatus::Pending};
    }
    fn finish(&mut self, goal:&str) {
        let mut ts=self.tasks.values().filter(|t|t.goal==goal).peekable();
        let nonempty=ts.peek().is_some();
        if nonempty && ts.all(|t|t.status==TaskStatus::Committed) {
            if let Some(g)=self.goals.get_mut(goal) { if g.sealed && g.status==GoalStatus::Open {g.status=GoalStatus::Complete;} }
        }
    }
    /// A failed command changes no state when invoked through Store::apply.
    pub fn transition(&mut self, command:Command)->Result<Value> {
        use Command::*;
        match command {
            Goal{id,objective,claim_budget}=>{
                valid_id(&id)?; require(!self.goals.contains_key(&id) && claim_budget>0 && !objective.trim().is_empty(),"invalid or duplicate goal")?;
                self.goals.insert(id.clone(), crate::Goal{id,objective,status:GoalStatus::Open,claim_budget,claims:0,sealed:false});
            }
            Class{id,version,operators,capabilities}=>{
                let k=key(&id,version)?; require(!operators.is_empty()&&!self.classes.contains_key(&k),"invalid or duplicate class")?;
                require(capabilities.iter().all(|c|self.capabilities.get(c).is_some_and(|x|x.active)),"unknown capability")?;
                self.classes.insert(k,crate::Class{id,version,operators,capabilities});
            }
            Context{id,artifacts,data}=>{
                valid_id(&id)?; require(!self.contexts.contains_key(&id)&&artifacts.iter().all(|a|self.artifacts.contains_key(a)),"invalid context")?;
                self.contexts.insert(id.clone(),crate::Context{id,artifacts,data});
            }
            Agent{id,goal,class,context,parent}=>{
                valid_id(&id)?; self.open_goal(&goal)?;
                require(!self.agents.contains_key(&id)&&self.classes.contains_key(&class)&&self.contexts.contains_key(&context),"invalid agent")?;
                require(parent.as_ref().is_none_or(|p|self.agents.get(p).is_some_and(|a|a.goal==goal)),"invalid parent")?;
                self.agents.insert(id.clone(),crate::Agent{id,goal,class,context,parent,incarnation:0,active:false});
            }
            Activate{agent}=>{
                let a=self.agents.get(&agent).ok_or("agent missing")?; self.open_goal(&a.goal)?;
                require(!a.active,"already active")?;
                let a=self.agents.get_mut(&agent).unwrap();
                a.incarnation=a.incarnation.checked_add(1).ok_or("incarnation overflow")?; a.active=true;
                return Ok(serde_json::json!({"incarnation":a.incarnation}));
            }
            Retire{agent}=>{
                let a=self.agents.get_mut(&agent).ok_or("agent missing")?; a.active=false;
                for t in self.tasks.values_mut().filter(|t|t.status==TaskStatus::Running&&t.lease.as_ref().is_some_and(|l|l.agent==agent)) { Self::release(t); }
            }
            Task{id,goal,objective,depends,input,expected,capability,max_attempts}=>{
                valid_id(&id)?; require(!self.open_goal(&goal)?.sealed,"goal sealed")?;
                require(!self.tasks.contains_key(&id)&&!objective.trim().is_empty()&&is_hash(&expected)&&max_attempts>0,"invalid task")?;
                // Dependencies must already exist: cycles cannot be introduced.
                require(depends.iter().all(|d|self.tasks.get(d).is_some_and(|t|t.goal==goal)),"invalid dependency")?;
                require(depends.iter().collect::<BTreeSet<_>>().len()==depends.len(),"duplicate dependency")?;
                require(capability.as_ref().is_none_or(|c|self.capabilities.get(c).is_some_and(|x|x.active)),"capability unavailable")?;
                self.tasks.insert(id.clone(),crate::Task{id,goal,objective,depends,input,expected,capability,max_attempts,attempts:0,generation:0,lease:None,checkpoint:None,proposal:None,artifact:None,status:TaskStatus::Pending});
            }
            Seal{goal}=>{
                self.open_goal(&goal)?;
                require(self.tasks.values().any(|t|t.goal==goal),"cannot seal empty goal")?;
                self.goals.get_mut(&goal).unwrap().sealed=true; self.finish(&goal);
            }
            Claim{task,agent,ttl}=>{
                require(ttl>0,"invalid ttl")?; require(self.ready().contains(&task),"task not ready")?;
                let t=self.tasks.get(&task).unwrap(); let a=self.agents.get(&agent).ok_or("agent missing")?;
                require(a.active&&a.goal==t.goal,"agent not eligible")?;
                let c=self.classes.get(&a.class).ok_or("class missing")?;
                require(c.operators.contains(&Operator::Execute),"class lacks execute")?;
                require(t.capability.as_ref().is_none_or(|k|c.capabilities.contains(k)&&self.capabilities.get(k).is_some_and(|x|x.active)),"class lacks active capability")?;
                let expiry=self.clock.checked_add(ttl).ok_or("clock overflow")?;
                let t=self.tasks.get_mut(&task).unwrap();
                t.attempts+=1; t.generation=t.generation.checked_add(1).ok_or("generation overflow")?;
                t.lease=Some(Lease{agent,incarnation:a.incarnation,generation:t.generation,expires:expiry});
                t.status=TaskStatus::Running; self.goals.get_mut(&t.goal).unwrap().claims+=1;
                return Ok(serde_json::to_value(&t.lease).map_err(err)?);
            }
            Checkpoint{task,agent,generation,data}=>{
                self.checked_lease(&task,&agent,generation)?;
                self.tasks.get_mut(&task).unwrap().checkpoint=Some(data);
            }
            Submit{task,agent,generation,value}=>{
                self.checked_lease(&task,&agent,generation)?;
                let t=self.tasks.get_mut(&task).unwrap();
                t.proposal=Some(value); t.status=TaskStatus::Submitted;
            }
            Verify{task}=>{
                let t=self.tasks.get(&task).ok_or("task missing")?; self.open_goal(&t.goal)?;
                require(t.status==TaskStatus::Submitted,"task not submitted")?;
                let value=t.proposal.clone().ok_or("proposal missing")?;
                let actual=digest(&value); let accepted=actual==t.expected;
                let g=t.goal.clone(); let generation=t.generation;
                self.evidence.push(crate::Evidence{task:task.clone(),generation,expected:t.expected.clone(),actual:actual.clone(),accepted});
                let t=self.tasks.get_mut(&task).unwrap();
                if accepted {
                    // Atomic admission, single artifact per task; verification is independent of worker claims.
                    require(!self.artifacts.contains_key(&task),"duplicate effect")?;
                    self.artifacts.insert(task.clone(),Artifact{id:task.clone(),task:task.clone(),generation,digest:actual,value});
                    t.artifact=Some(task); t.status=TaskStatus::Committed; t.lease=None; t.proposal=None;
                } else { Self::release(t); }
                self.finish(&g);
                return Ok(serde_json::json!({"accepted":accepted}));
            }
            Execute{task,agent,generation}=>{
                let t=self.checked_lease(&task,&agent,generation)?;
                let c=self.capabilities.get(t.capability.as_ref().ok_or("task has no capability")?).ok_or("capability missing")?;
                require(c.active,"capability disabled")?;
                let value=c.program.eval(&t.input)?;
                return self.transition(Submit{task,agent,generation,value});
            }
            Tick{now}=>{
                require(now>=self.clock,"clock cannot reverse")?; self.clock=now;
                for t in self.tasks.values_mut().filter(|t|t.status==TaskStatus::Running&&t.lease.as_ref().is_some_and(|l|l.expires<=now)) { Self::release(t); }
            }
            Send{id,from,to,body}=>{
                valid_id(&id)?; require(self.agents.get(&from).zip(self.agents.get(&to)).is_some_and(|(a,b)|a.goal==b.goal),"invalid mailbox endpoints")?;
                if let Some(m)=self.messages.get(&id) {
                    require(m.from==from&&m.to==to&&m.body==body,"message ID conflict")?;
                } else {self.messages.insert(id.clone(),Message{id,from,to,body,acknowledged:false});}
            }
            Ack{id,agent}=>{
                let m=self.messages.get_mut(&id).ok_or("message missing")?; require(m.to==agent,"wrong recipient")?; m.acknowledged=true;
            }
            Promote{id,version,program,examples}=>{
                let k=key(&id,version)?;
                require(!self.capabilities.contains_key(&k)&&examples.len()>=2,"duplicate capability or insufficient examples")?;
                let mut inputs=BTreeSet::new();
                for e in &examples {
                    let t=self.tasks.get(e).ok_or("example task missing")?;
                    require(t.status==TaskStatus::Committed,"example not verified")?;
                    inputs.insert(digest(&t.input));
                    require(digest(&program.eval(&t.input)?)==t.expected,"program fails verified example")?;
                }
                require(inputs.len()>=2,"need distinct verified inputs")?;
                self.capabilities.insert(k,Capability{id,version,program,examples,active:true});
            }
            Disable{capability}=>{ self.capabilities.get_mut(&capability).ok_or("capability missing")?.active=false; }
            Cancel{goal}=>{
                self.open_goal(&goal)?; self.goals.get_mut(&goal).unwrap().status=GoalStatus::Cancelled;
                for t in self.tasks.values_mut().filter(|t|t.goal==goal&&t.status!=TaskStatus::Committed) {t.status=TaskStatus::Cancelled;t.lease=None;t.proposal=None;}
                for a in self.agents.values_mut().filter(|a|a.goal==goal) {a.active=false;}
            }
        }
        Ok(serde_json::json!({"ok":true}))
    }
}

#[derive(Serialize,Deserialize)]
struct Record { schema:u32, revision:u64, previous:String, command:Command, state:State, result:Value }
#[derive(Serialize,Deserialize)]
struct Envelope { digest:String, record:Record }
/// Immutable revision files. OS file lock serializes all cooperating writers/readers.
/// A record becomes visible by rename only after its temporary file is synced.
pub struct Store { root:PathBuf }
impl Store {
    pub fn open(root:impl AsRef<Path>)->Result<Self> { fs::create_dir_all(root.as_ref()).map_err(err)?; Ok(Self{root:root.as_ref().to_path_buf()}) }
    fn lock(&self)->Result<File> {
        let f=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(self.root.join("writer.lock")).map_err(err)?;
        f.try_lock().map_err(|e|format!("store busy or lock failed: {e:?}"))?; Ok(f)
    }
    fn load(&self)->Result<(State,u64,String)> {
        let mut files=Vec::new();
        for e in fs::read_dir(&self.root).map_err(err)? {
            let e=e.map_err(err)?; let n=e.file_name().to_string_lossy().into_owned();
            if let Some(v)=n.strip_suffix(".rev.json") {
                let rev=v.parse::<u64>().map_err(err)?; files.push((rev,e.path()));
            }
        }
        files.sort_by_key(|x|x.0);
        let mut state=State::default(); let mut rev=0u64; let mut previous="genesis".to_string();
        for (n,path) in files {
            require(n==rev.checked_add(1).ok_or("revision overflow")?,"revision gap")?;
            let e:Envelope=serde_json::from_slice(&fs::read(path).map_err(err)?).map_err(err)?;
            let actual=hash(&serde_json::to_vec(&e.record).map_err(err)?);
            require(actual==e.digest&&e.record.schema==1&&e.record.revision==n&&e.record.previous==previous,"journal integrity failure")?;
            // Replay, not merely trust a serialized snapshot.
            let result=state.transition(e.record.command.clone())?;
            require(result==e.record.result&&serde_json::to_value(&state).map_err(err)?==serde_json::to_value(&e.record.state).map_err(err)?,"journal replay mismatch")?;
            rev=n; previous=e.digest;
        }
        Ok((state,rev,previous))
    }
    pub fn inspect(&self)->Result<State> { let _guard=self.lock()?; Ok(self.load()?.0) }
    pub fn apply(&self,command:Command)->Result<Value> {
        let _guard=self.lock()?; let (mut state,rev,previous)=self.load()?;
        let result=state.transition(command.clone())?;
        let record=Record{schema:1,revision:rev.checked_add(1).ok_or("revision overflow")?,previous,command,state,result:result.clone()};
        let envelope=Envelope{digest:hash(&serde_json::to_vec(&record).map_err(err)?),record};
        let bytes=serde_json::to_vec(&envelope).map_err(err)?;
        let suffix=SystemTime::now().duration_since(UNIX_EPOCH).map_err(err)?.as_nanos();
        let tmp=self.root.join(format!(".tmp-{}-{suffix}",std::process::id()));
        let dest=self.root.join(format!("{:020}.rev.json",envelope.record.revision));
        require(!dest.exists(),"revision already exists")?;
        let mut f=OpenOptions::new().create_new(true).write(true).open(&tmp).map_err(err)?;
        f.write_all(&bytes).map_err(err)?; f.sync_all().map_err(err)?; drop(f);
        fs::rename(&tmp,&dest).map_err(err)?;
        #[cfg(unix)] File::open(&self.root).and_then(|d|d.sync_all()).map_err(err)?;
        Ok(result)
    }
}
