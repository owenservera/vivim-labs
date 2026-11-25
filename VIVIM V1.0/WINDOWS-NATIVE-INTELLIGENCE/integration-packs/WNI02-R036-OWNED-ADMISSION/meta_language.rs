//! @hq:context Typed pure meta-language IR, compiler and evaluator over the incumbent protocol snapshot.
//! @hq:why Definitions must describe executable semantics rather than only prose.
//! @hq:invariant Typed code cannot grant effects, access external state, mutate the kernel or escape resource bounds.
//! @hq:see docs/META-VLANG-LANGUAGE-ARCHITECTURE-R001.md
use crate::protocol::Snapshot;
use std::collections::{BTreeMap, BTreeSet};

const MAX_DEPTH: usize = 64;
const MAX_TOKENS: usize = 1024;
const MAX_TEXT: usize = 4096;
const MAX_PARAMS: usize = 32;
const MAX_FUEL: usize = 100_000;
pub const MARKER: &str = "meta/1 ";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type { Bool, Int, Text }
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value { Bool(bool), Int(i64), Text(String) }
impl Value {
    pub fn ty(&self) -> Type {
        match self { Self::Bool(_) => Type::Bool, Self::Int(_) => Type::Int, Self::Text(_) => Type::Text }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op { Add, Sub, Mul, Eq, Not, And, Or, Concat, Len }
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Literal(Value), Arg(String), Call(String, Vec<Expr>), Apply(Op, Vec<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function { pub params: Vec<(String, Type)>, pub result: Type, pub body: Expr }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic { pub definition: String, pub code: &'static str }
fn diag(id: &str, code: &'static str) -> Diagnostic { Diagnostic { definition: id.into(), code } }

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token { Open, Close, Atom(String), Text(String) }
fn lex(s: &str) -> Result<Vec<Token>, &'static str> {
    if s.len() > 4096 { return Err("INPUT_LIMIT"); }
    let mut chars = s.chars().peekable();
    let mut tokens = Vec::new();
    while let Some(c) = chars.next() {
        let token = match c {
            c if c.is_whitespace() => continue,
            '(' => Token::Open, ')' => Token::Close,
            '"' => {
                let mut text = String::new();
                loop {
                    match chars.next().ok_or("UNTERMINATED_TEXT")? {
                        '"' => break,
                        '\\' => match chars.next().ok_or("BAD_ESCAPE")? {
                            '"' => text.push('"'), '\\' => text.push('\\'),
                            'n' => text.push('\n'), 't' => text.push('\t'), _ => return Err("BAD_ESCAPE")
                        },
                        c if c.is_control() => return Err("CONTROL_CHARACTER"),
                        c => text.push(c),
                    }
                }
                Token::Text(text)
            }
            c if c.is_control() => return Err("CONTROL_CHARACTER"),
            c => {
                let mut a = String::from(c);
                while let Some(&next) = chars.peek() {
                    if next.is_whitespace() || matches!(next, '(' | ')' | '"') { break; }
                    if next.is_control() { return Err("CONTROL_CHARACTER"); }
                    a.push(chars.next().unwrap());
                }
                Token::Atom(a)
            }
        };
        tokens.push(token);
        if tokens.len() > MAX_TOKENS { return Err("TOKEN_LIMIT"); }
    }
    Ok(tokens)
}
struct Parser { tokens: Vec<Token>, pos: usize }
impl Parser {
    fn new(s: &str) -> Result<Self, &'static str> { Ok(Self { tokens: lex(s)?, pos: 0 }) }
    fn take(&mut self) -> Result<Token, &'static str> {
        let token = self.tokens.get(self.pos).cloned().ok_or("UNEXPECTED_END")?;
        self.pos += 1; Ok(token)
    }
    fn expect(&mut self, token: Token) -> Result<(), &'static str> {
        if self.take()? == token { Ok(()) } else { Err("UNEXPECTED_TOKEN") }
    }
    fn atom(&mut self) -> Result<String, &'static str> {
        match self.take()? { Token::Atom(a) => Ok(a), _ => Err("EXPECTED_ATOM") }
    }
    fn ty(&mut self) -> Result<Type, &'static str> {
        match self.atom()?.as_str() { "bool" => Ok(Type::Bool), "int" => Ok(Type::Int), "text" => Ok(Type::Text), _ => Err("UNKNOWN_TYPE") }
    }
    fn close_next(&self) -> bool { self.tokens.get(self.pos) == Some(&Token::Close) }
    fn expr(&mut self, depth: usize) -> Result<Expr, &'static str> {
        if depth >= MAX_DEPTH { return Err("EXPRESSION_DEPTH"); }
        self.expect(Token::Open)?;
        let head = self.atom()?;
        let result = match head.as_str() {
            "bool" => Expr::Literal(Value::Bool(match self.atom()?.as_str() { "true" => true, "false" => false, _ => return Err("BAD_BOOL") })),
            "int" => Expr::Literal(Value::Int(self.atom()?.parse().map_err(|_| "BAD_INT")?)),
            "text" => match self.take()? { Token::Text(t) => Expr::Literal(Value::Text(t)), _ => return Err("EXPECTED_TEXT") },
            "arg" => Expr::Arg(self.atom()?),
            "if" => Expr::If(Box::new(self.expr(depth+1)?), Box::new(self.expr(depth+1)?), Box::new(self.expr(depth+1)?)),
            "call" => { let id = self.atom()?; Expr::Call(id, self.args(depth+1)?) }
            head => {
                let op = match head {
                    "add" => Op::Add, "sub" => Op::Sub, "mul" => Op::Mul, "eq" => Op::Eq, "not" => Op::Not,
                    "and" => Op::And, "or" => Op::Or, "concat" => Op::Concat, "len" => Op::Len,
                    _ => return Err("UNKNOWN_OPERATOR"),
                };
                Expr::Apply(op, self.args(depth+1)?)
            }
        };
        self.expect(Token::Close)?;
        Ok(result)
    }
    fn args(&mut self, depth: usize) -> Result<Vec<Expr>, &'static str> {
        let mut args = Vec::new();
        while !self.close_next() {
            if args.len() == MAX_PARAMS { return Err("ARITY_LIMIT"); }
            args.push(self.expr(depth)?);
        }
        Ok(args)
    }
}
fn valid_param(s: &str) -> bool {
    !s.is_empty() && s.len() <= 64 && s.as_bytes()[0].is_ascii_lowercase() &&
        s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
}
/// @hq:context Decode a version-marked definition into an executable typed syntax tree.
pub fn parse(body: &str) -> Result<Function, &'static str> {
    let mut p = Parser::new(body.strip_prefix(MARKER).ok_or("UNSUPPORTED_META_VERSION")?)?;
    p.expect(Token::Open)?;
    if p.atom()? != "fn" { return Err("EXPECTED_FUNCTION"); }
    p.expect(Token::Open)?;
    let mut params = Vec::new();
    let mut names = BTreeSet::new();
    while !p.close_next() {
        if params.len() == MAX_PARAMS { return Err("PARAM_LIMIT"); }
        p.expect(Token::Open)?;
        let name = p.atom()?;
        if !valid_param(&name) || !names.insert(name.clone()) { return Err("BAD_PARAMETER"); }
        let ty = p.ty()?;
        p.expect(Token::Close)?;
        params.push((name, ty));
    }
    p.expect(Token::Close)?;
    let result = p.ty()?;
    let body = p.expr(0)?;
    p.expect(Token::Close)?;
    if p.pos != p.tokens.len() { return Err("TRAILING_INPUT"); }
    Ok(Function { params, result, body })
}
/// @hq:context Parse CLI argument literals using the same expression parser, with no evaluation or effects.
pub fn parse_values(input: &str) -> Result<Vec<Value>, &'static str> {
    let mut p = Parser::new(input)?;
    let mut values = Vec::new();
    while p.pos < p.tokens.len() {
        if values.len() == MAX_PARAMS { return Err("ARITY_LIMIT"); }
        match p.expr(0)? { Expr::Literal(v) => values.push(v), _ => return Err("EXPECTED_LITERAL") }
    }
    Ok(values)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program { functions: BTreeMap<String, Function> }
impl Program {
    /// @hq:context Resolve all typed definitions against the same incumbent protocol snapshot.
    /// @hq:invariant Call dependencies must exactly match declared dependency identities; legacy prose is never callable.
    pub fn compile(snapshot: &Snapshot) -> Result<Self, Diagnostic> {
        let mut functions = BTreeMap::new();
        for id in snapshot.ids() {
            let d = snapshot.get(id).unwrap();
            if d.body.starts_with("meta/") {
                functions.insert(id.to_owned(), parse(&d.body).map_err(|code| diag(id, code))?);
            }
        }
        let program = Self { functions };
        for (id, f) in &program.functions {
            let mut dependencies = BTreeSet::new();
            let actual = program.check(&f.body, f, &mut dependencies).map_err(|code| diag(id, code))?;
            if actual != f.result { return Err(diag(id, "RESULT_TYPE")); }
            if snapshot.get(id).unwrap().dependencies != dependencies { return Err(diag(id, "DEPENDENCY_MISMATCH")); }
        }
        Ok(program)
    }
    pub fn get(&self, id: &str) -> Option<&Function> { self.functions.get(id) }
    pub fn ids(&self) -> impl Iterator<Item=&str> { self.functions.keys().map(String::as_str) }
    fn check(&self, e: &Expr, f: &Function, deps: &mut BTreeSet<String>) -> Result<Type, &'static str> {
        match e {
            Expr::Literal(v) => Ok(v.ty()),
            Expr::Arg(name) => f.params.iter().find(|(n,_)| n == name).map(|(_,t)| *t).ok_or("UNKNOWN_ARGUMENT"),
            Expr::Call(id, args) => {
                let target = self.get(id).ok_or("UNKNOWN_TYPED_FUNCTION")?;
                deps.insert(id.clone());
                if args.len() != target.params.len() { return Err("CALL_ARITY"); }
                for (arg, (_,ty)) in args.iter().zip(&target.params) {
                    if self.check(arg, f, deps)? != *ty { return Err("CALL_TYPE"); }
                }
                Ok(target.result)
            }
            Expr::If(c,t,e) => {
                if self.check(c,f,deps)? != Type::Bool { return Err("IF_CONDITION_TYPE"); }
                let t = self.check(t,f,deps)?;
                if t != self.check(e,f,deps)? { return Err("IF_BRANCH_TYPE"); }
                Ok(t)
            }
            Expr::Apply(op,args) => {
                let types = args.iter().map(|a| self.check(a,f,deps)).collect::<Result<Vec<_>,_>>()?;
                use Type::*;
                match (op,types.as_slice()) {
                    (Op::Add | Op::Sub | Op::Mul, [Int,Int]) => Ok(Int),
                    (Op::Eq,[a,b]) if a == b => Ok(Bool),
                    (Op::Not,[Bool]) | (Op::And | Op::Or,[Bool,Bool]) => Ok(Bool),
                    (Op::Concat,[Text,Text]) => Ok(Text), (Op::Len,[Text]) => Ok(Int),
                    _ => Err("OPERATOR_TYPE_OR_ARITY")
                }
            }
        }
    }
    /// @hq:context Preserve callable signatures for compatible live definition updates.
    /// @hq:invariant Deletion, typed-to-prose downgrade and signature changes require a separate migration path.
    pub fn compatible_with(&self, candidate: &Self) -> Result<(), Diagnostic> {
        for (id, old) in &self.functions {
            let new = candidate.get(id).ok_or_else(|| diag(id,"TYPED_REMOVAL_REQUIRES_MIGRATION"))?;
            if old.params != new.params || old.result != new.result {
                return Err(diag(id,"SIGNATURE_CHANGE_REQUIRES_MIGRATION"));
            }
        }
        Ok(())
    }
    /// @hq:context Evaluate a checked pure definition with a shared fuel and call-depth budget.
    pub fn evaluate(&self,id:&str,args:Vec<Value>,fuel:usize) -> Result<Evaluation, Diagnostic> {
        if fuel == 0 || fuel > MAX_FUEL { return Err(diag(id,"FUEL_LIMIT")); }
        let mut remaining = fuel;
        let value = self.call(id,args,&mut remaining,0).map_err(|c|diag(id,c))?;
        Ok(Evaluation { value, steps: fuel-remaining })
    }
    fn call(&self,id:&str,args:Vec<Value>,fuel:&mut usize,depth:usize) -> Result<Value,&'static str> {
        if depth >= MAX_DEPTH { return Err("CALL_DEPTH"); }
        let f=self.get(id).ok_or("UNKNOWN_TYPED_FUNCTION")?;
        if args.len()!=f.params.len() || args.iter().zip(&f.params).any(|(v,(_,t))| v.ty()!=*t) { return Err("CALL_ARGUMENTS"); }
        if args.iter().any(|v| matches!(v, Value::Text(t) if t.len()>MAX_TEXT)) { return Err("TEXT_LIMIT"); }
        let env=f.params.iter().map(|(n,_)|n.as_str()).zip(args).collect();
        self.eval(&f.body,&env,fuel,depth)
    }
    fn eval(&self,e:&Expr,env:&BTreeMap<&str,Value>,fuel:&mut usize,depth:usize)->Result<Value,&'static str> {
        *fuel=fuel.checked_sub(1).ok_or("FUEL_EXHAUSTED")?;
        match e {
            Expr::Literal(v)=>Ok(v.clone()),
            Expr::Arg(name)=>env.get(name.as_str()).cloned().ok_or("UNKNOWN_ARGUMENT"),
            Expr::Call(id,args)=>{
                let values=args.iter().map(|a|self.eval(a,env,fuel,depth)).collect::<Result<Vec<_>,_>>()?;
                self.call(id,values,fuel,depth+1)
            }
            Expr::If(c,t,e)=>match self.eval(c,env,fuel,depth)? {
                Value::Bool(true)=>self.eval(t,env,fuel,depth),Value::Bool(false)=>self.eval(e,env,fuel,depth),_=>Err("IF_CONDITION_TYPE")
            },
            Expr::Apply(op,args)=>{
                // Both branches are checked at compile time; runtime short-circuits boolean operators.
                if matches!(op,Op::And|Op::Or) {
                    let left=self.eval(&args[0],env,fuel,depth)?;
                    match (op,left) {
                        (Op::And,Value::Bool(false))=>return Ok(Value::Bool(false)),
                        (Op::Or,Value::Bool(true))=>return Ok(Value::Bool(true)),
                        (Op::And,Value::Bool(true))|(Op::Or,Value::Bool(false))=>return self.eval(&args[1],env,fuel,depth),
                        _=>return Err("OPERATOR_TYPE_OR_ARITY")
                    }
                }
                let values=args.iter().map(|a|self.eval(a,env,fuel,depth)).collect::<Result<Vec<_>,_>>()?;
                use Value::*;
                match (op,values.as_slice()) {
                    (Op::Add,[Int(a),Int(b)])=>a.checked_add(*b).map(Int).ok_or("INTEGER_OVERFLOW"),
                    (Op::Sub,[Int(a),Int(b)])=>a.checked_sub(*b).map(Int).ok_or("INTEGER_OVERFLOW"),
                    (Op::Mul,[Int(a),Int(b)])=>a.checked_mul(*b).map(Int).ok_or("INTEGER_OVERFLOW"),
                    (Op::Eq,[a,b])=>Ok(Bool(a==b)),(Op::Not,[Bool(a)])=>Ok(Bool(!a)),
                    (Op::Concat,[Text(a),Text(b)]) if a.len()+b.len()<=MAX_TEXT=>Ok(Text(format!("{a}{b}"))),
                    (Op::Concat,[Text(_),Text(_)])=>Err("TEXT_LIMIT"),
                    (Op::Len,[Text(a)])=>Ok(Int(a.chars().count() as i64)),
                    _=>Err("OPERATOR_TYPE_OR_ARITY")
                }
            }
        }
    }
}
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Evaluation { pub value: Value, pub steps: usize }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{Definition,Error,Runtime};
    fn pack()->Snapshot {
        Snapshot::parse("legacy\t1\t\tPurpose only\nmeta.inc\t1\t\tmeta/1 (fn ((n int)) int (add (arg n) (int 1)))\nmeta.twice\t1\tmeta.inc\tmeta/1 (fn ((n int)) int (call meta.inc (call meta.inc (arg n))))\nmeta.keep\t1\t\tmeta/1 (fn () bool (bool true))").unwrap()
    }
    fn update(r:&Runtime,body:&str)->Definition {
        let mut d=r.snapshot().get("meta.inc").unwrap().clone();
        d.revision+=1;d.body=body.into();d
    }
    #[test]
    fn compile_evaluate_and_inspect() {
        let p=Program::compile(&pack()).unwrap();
        assert_eq!(p.ids().count(),3);
        assert!(p.get("legacy").is_none());
        assert_eq!(p.evaluate("meta.twice",vec![Value::Int(10)],100).unwrap().value,Value::Int(12));
        assert_eq!(p.get("meta.inc").unwrap().result,Type::Int);
    }
    #[test]
    fn live_revision_changes_only_affected_consumers() {
        let mut r=Runtime::start(pack());
        let d=update(&r,"meta/1 (fn ((n int)) int (add (arg n) (int 3)))");
        let mut candidate=r.clone();
        let preview=candidate.patch(1,vec![d.clone()],BTreeSet::new()).unwrap().unwrap();
        assert_eq!(r.generation(),1);
        let applied=r.patch(1,vec![d],BTreeSet::new()).unwrap().unwrap();
        assert_eq!(preview,applied);
        assert_eq!(applied.affected,["meta.inc","meta.twice"].into_iter().map(str::to_owned).collect());
        assert_eq!(Program::compile(r.snapshot()).unwrap().evaluate("meta.twice",vec![Value::Int(10)],100).unwrap().value,Value::Int(16));
        assert_eq!(r.snapshot().get("meta.keep").unwrap().revision,1);
    }
    #[test]
    fn rejects_invalid_type_and_incompatible_signature_atomically() {
        let mut r=Runtime::start(pack());
        let d=update(&r,"meta/1 (fn ((n int)) int (bool true))");
        assert!(matches!(r.patch(1,vec![d],BTreeSet::new()),Err(Error::InvalidTypedDefinition(_))));
        // A parameter rename preserves positional behavior but requires migration by policy.
        let d=update(&r,"meta/1 (fn ((x int)) int (arg x))");
        assert!(matches!(r.patch(1,vec![d],BTreeSet::new()),Err(Error::InvalidTypedDefinition(_))));
        assert_eq!(r.generation(),1);
        assert_eq!(r.snapshot(),&pack());
    }
    #[test]
    fn rejects_undeclared_and_untyped_dependencies() {
        assert!(Snapshot::parse("a\t1\t\tmeta/1 (fn () int (call b))\nb\t1\t\tmeta/1 (fn () int (int 1))").is_err());
        assert!(Snapshot::parse("a\t1\tb\tmeta/1 (fn () int (call b))\nb\t1\t\tordinary").is_err());
        assert!(Snapshot::parse("a\t1\ta\tmeta/1 (fn () int (call a))").is_err());
        assert!(Snapshot::parse("a\t1\t\tmeta/2 (fn () int (int 1))").is_err());
    }
    #[test]
    fn fuel_overflow_and_argument_checks() {
        let p=Program::compile(&pack()).unwrap();
        assert_eq!(p.evaluate("meta.inc",vec![Value::Int(i64::MAX)],100).unwrap_err().code,"INTEGER_OVERFLOW");
        assert_eq!(p.evaluate("meta.twice",vec![Value::Int(1)],1).unwrap_err().code,"FUEL_EXHAUSTED");
        assert_eq!(p.evaluate("meta.inc",vec![Value::Bool(true)],100).unwrap_err().code,"CALL_ARGUMENTS");
        assert!(parse("meta/1 (fn () int (int 1)) trailing").is_err());
        assert!(parse_values("(call x)").is_err());
    }
    #[test]
    fn text_unicode_and_short_circuit() {
        let s=Snapshot::parse("a\t1\t\tmeta/1 (fn () int (len (concat (text \"é\") (text \"猫\"))))\nb\t1\t\tmeta/1 (fn () bool (and (bool false) (eq (add (int 9223372036854775807) (int 1)) (int 0))))").unwrap();
        let p=Program::compile(&s).unwrap();
        assert_eq!(p.evaluate("a",vec![],100).unwrap().value,Value::Int(2));
        assert_eq!(p.evaluate("b",vec![],100).unwrap().value,Value::Bool(false));
    }
}
