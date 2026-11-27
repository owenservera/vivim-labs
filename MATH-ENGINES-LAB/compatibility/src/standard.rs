//! Standard four-function calculator consumer: interaction state, not a math engine.
//! All mathematical operations and decimal projections use admitted capabilities.
use super::*;
use super::extensions::{PARSE,DISPLAY,PERCENT,SQRT};
pub const CONSUMER:Key=key("calculator.standard.immediate");
pub const REQUIREMENTS:&[Requirement]=&[ADD,SUB,MUL,DIV,NEG,PARSE,DISPLAY,PERCENT,SQRT];
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Binary{Add,Subtract,Multiply,Divide}
impl Binary{
    fn requirement(self)->Requirement{
        match self{Self::Add=>ADD,Self::Subtract=>SUB,Self::Multiply=>MUL,Self::Divide=>DIV}
    }
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Press{
    Digit(u8),Point,Backspace,Operator(Binary),Equals,Percent,Sign,Square,Sqrt,Reciprocal,
    ClearEntry,AllClear,MemoryClear,MemoryRecall,MemoryStore,MemoryAdd,MemorySubtract
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct Num{value:Rational,approx:bool}
impl Num{fn zero()->Self{Self{value:Rational::zero(),approx:false}}}
#[derive(Clone,Debug)]
struct State{
    current:Num,entry:Option<String>,pending:Option<(Num,Binary)>,
    repeat:Option<(Binary,Num)>,awaiting:bool,memory:Option<Num>
}
impl State{
    fn new()->Self{Self{current:Num::zero(),entry:None,pending:None,repeat:None,awaiting:false,memory:None}}
}
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Projection{pub display:String,pub value:Rational,pub approximate_value:bool,pub memory:bool,pub pending:Option<Binary>}
pub struct Calculator{plan:Plan,state:State}
impl Calculator{
    pub fn gaps(reg:&Registry)->Vec<Gap>{reg.gaps(CONSUMER,REQUIREMENTS)}
    pub fn new(reg:&Registry)->Result<Self>{Ok(Self{plan:reg.plan(REQUIREMENTS)?,state:State::new()})}
    /// Explicit rebind after another capability has been registered. Existing
    /// operation keys cannot be overwritten by the admission mechanism.
    pub fn rebind(&mut self,reg:&Registry)->Result<()> {self.plan=reg.plan(REQUIREMENTS)?;Ok(())}
    pub fn projection(&self,reg:&Registry)->Result<Projection>{project(reg,&self.plan,&self.state)}
    /// Stage interaction state, validate its resulting projection, then commit.
    /// Errors preserve the complete previous state, including memory.
    pub fn press(&mut self,reg:&Registry,p:Press)->Result<Projection>{
        let mut next=self.state.clone();
        apply(reg,&self.plan,&mut next,p)?;
        let result=project(reg,&self.plan,&next)?;
        self.state=next;Ok(result)
    }
}
fn unary(reg:&Registry,plan:&Plan,r:Requirement,a:Num)->Result<Num>{
    let value=reg.call(plan,r,&[Value::Number(a.value)],64)?.number()?;
    Ok(Num{value,approx:a.approx||r.signature.assurance==Assurance::Rounded})
}
fn binary(reg:&Registry,plan:&Plan,r:Requirement,a:Num,b:Num)->Result<Num>{
    let value=reg.call(plan,r,&[Value::Number(a.value),Value::Number(b.value)],64)?.number()?;
    Ok(Num{value,approx:a.approx||b.approx||r.signature.assurance==Assurance::Rounded})
}
fn operand(reg:&Registry,plan:&Plan,s:&State)->Result<Num>{
    if let Some(entry)=&s.entry{
        let value=reg.call(plan,PARSE,&[Value::Text(entry.clone())],64)?.number()?;
        Ok(Num{value,approx:false})
    }else{Ok(s.current)}
}
fn project(reg:&Registry,plan:&Plan,s:&State)->Result<Projection>{
    let n=operand(reg,plan,s)?;
    let formatted=reg.call(plan,DISPLAY,&[Value::Number(n.value)],64)?;
    let mut display=s.entry.clone().unwrap_or(formatted.text()?.to_owned());
    if n.approx&&!display.starts_with('~'){display.insert(0,'~');}
    Ok(Projection{display,value:n.value,approximate_value:n.approx,memory:s.memory.is_some(),pending:s.pending.map(|(_,op)|op)})
}
fn put(s:&mut State,n:Num){s.current=n;s.entry=None;s.awaiting=false;}
fn start_entry(s:&mut State){
    if s.entry.is_none(){s.entry=Some(String::new());s.awaiting=false;s.repeat=None;}
}
fn apply(reg:&Registry,plan:&Plan,s:&mut State,p:Press)->Result<()> {
    match p{
        Press::Digit(d)=>{
            if d>9{return Err(Error::Invalid);}
            start_entry(s);
            let e=s.entry.as_mut().ok_or(Error::Invalid)?;
            if e=="0"{e.clear();}else if e=="-0"{e.truncate(1);}
            if e.len()>=39{return Err(Error::Limit);}
            e.push(char::from(b'0'+d));
        }
        Press::Point=>{
            start_entry(s);let e=s.entry.as_mut().ok_or(Error::Invalid)?;
            if !e.contains('.') {if e.is_empty(){e.push('0');}e.push('.');}
        }
        Press::Backspace=>{
            if let Some(e)=s.entry.as_mut(){e.pop();if e.is_empty()||e=="-"{*e="0".into();}}
        }
        Press::Operator(op)=>{
            let mut lhs=operand(reg,plan,s)?;
            if let Some((previous,old))=s.pending{
                lhs=if s.awaiting{previous}else{binary(reg,plan,old.requirement(),previous,lhs)?};
            }
            s.current=lhs;s.entry=None;s.pending=Some((lhs,op));s.awaiting=true;s.repeat=None;
        }
        Press::Equals=>{
            let rhs=operand(reg,plan,s)?;
            if let Some((lhs,op))=s.pending{
                let result=binary(reg,plan,op.requirement(),lhs,rhs)?;
                put(s,result);s.pending=None;s.repeat=Some((op,rhs));
            }else if let Some((op,last))=s.repeat{
                let result=binary(reg,plan,op.requirement(),rhs,last)?;
                put(s,result);
            }else{put(s,rhs);}
        }
        Press::Percent=>{
            let rhs=operand(reg,plan,s)?;
            let fraction=unary(reg,plan,PERCENT,rhs)?;
            let result=match s.pending{
                Some((lhs,Binary::Add|Binary::Subtract))=>binary(reg,plan,MUL,lhs,fraction)?,
                _=>fraction,
            };
            put(s,result);s.repeat=None;
        }
        Press::Sign=>{
            if let Some(e)=s.entry.as_mut(){
                if e.starts_with('-'){e.remove(0);}else{e.insert(0,'-');}
            }else{
                let rhs=operand(reg,plan,s)?;
                let result=unary(reg,plan,NEG,rhs)?;
                put(s,result);
            }
            s.repeat=None;
        }
        Press::Square|Press::Sqrt|Press::Reciprocal=>{
            let rhs=operand(reg,plan,s)?;
            let result=match p{
                Press::Square=>binary(reg,plan,MUL,rhs,rhs)?,
                Press::Sqrt=>unary(reg,plan,SQRT,rhs)?,
                Press::Reciprocal=>binary(reg,plan,DIV,Num{value:Rational::one(),approx:false},rhs)?,
                _=>return Err(Error::Invalid),
            };
            put(s,result);s.repeat=None;
        }
        Press::ClearEntry=>{
            put(s,Num::zero());s.repeat=None;
        }
        Press::AllClear=>{
            let memory=s.memory;*s=State::new();s.memory=memory;
        }
        Press::MemoryClear=>s.memory=None,
        Press::MemoryRecall=>{
            put(s,s.memory.unwrap_or_else(Num::zero));s.repeat=None;
        }
        Press::MemoryStore=>s.memory=Some(operand(reg,plan,s)?),
        Press::MemoryAdd|Press::MemorySubtract=>{
            let rhs=operand(reg,plan,s)?;
            s.memory=Some(binary(reg,plan,if p==Press::MemoryAdd{ADD}else{SUB},s.memory.unwrap_or_else(Num::zero),rhs)?);
        }
    }
    Ok(())
}
