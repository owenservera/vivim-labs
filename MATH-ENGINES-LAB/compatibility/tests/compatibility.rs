use vivim_math_compatibility_lab::*;
use vivim_math_compatibility_lab::extensions::*;
use vivim_math_compatibility_lab::standard::*;
use vivim_math_engines_lab::MathError;

fn ready()->Registry{
    let mut r=Registry::baseline();
    r.admit(providers(),&reference_fixtures()).unwrap();r
}
fn text(s:&str)->Value{Value::Text(s.into())}
fn run(c:&mut Calculator,r:&Registry,seq:&[Press])->Projection{
    let mut p=c.projection(r).unwrap();
    for &k in seq{p=c.press(r,k).unwrap();}p
}
use Press::*;
fn op(b:Binary)->Press{Operator(b)}

#[test]
fn discovery_then_admission_then_real_consumer(){
    let mut r=Registry::baseline();
    let gaps=Calculator::gaps(&r);
    assert_eq!(gaps.len(),4);
    for needed in [PARSE,DISPLAY,PERCENT,SQRT]{
        assert!(gaps.iter().any(|g|g.required==needed&&g.error==Error::Missing(needed.key)));
    }
    assert!(matches!(Calculator::new(&r),Err(Error::Missing(_))));
    let old=r.plan(&[ADD]).unwrap();
    r.admit(providers(),&reference_fixtures()).unwrap();
    assert_eq!((r.len(),r.generation()),(9,2));
    assert!(Calculator::gaps(&r).is_empty());
    assert_eq!(r.call(&old,ADD,&[number(1,1),number(2,1)],64),Err(Error::Stale));
    let mut c=Calculator::new(&r).unwrap();
    assert_eq!(run(&mut c,&r,&[Digit(2),op(Binary::Add),Digit(3),Equals]).display,"5");
}

const A:Requirement=req("test.a",UNARY);
const B:Requirement=req("test.b",UNARY);
const C:Requirement=req("test.c",UNARY);
fn a(c:&mut Call<'_>,v:&[Value])->Result<Value>{c.call(B,v)}
fn b(c:&mut Call<'_>,v:&[Value])->Result<Value>{c.call(A,v)}
fn identity(_: &mut Call<'_>,v:&[Value])->Result<Value>{Ok(v[0].clone())}
fn wrong(_: &mut Call<'_>,_:&[Value])->Result<Value>{Ok(text("wrong"))}
fn sneaky(c:&mut Call<'_>,v:&[Value])->Result<Value>{c.call(NEG,v)}
fn fixtures(r:Requirement)->Vec<Fixture>{
    vec![
        Fixture{capability:r,input:vec![number(2,1)],expected:Ok(number(2,1))},
        Fixture{capability:r,input:vec![text("wrong")],expected:Err(Error::Type)}
    ]
}
#[test]
fn missing_transitive_dependency_is_atomic(){
    let mut r=Registry::baseline();
    let old=r.plan(&[ADD]).unwrap();
    let bundle=vec![Provider{contract:A,requires:&[B],run:a},Provider{contract:B,requires:&[C],run:identity}];
    assert_eq!(r.admit(bundle,&[]),Err(Error::Missing(C.key)));
    assert_eq!((r.len(),r.generation()),(5,1));
    assert_eq!(r.call(&old,ADD,&[number(2,1),number(3,1)],8),Ok(number(5,1)));
}
#[test]
fn cycles_are_rejected(){
    let mut r=Registry::baseline();
    let bundle=vec![Provider{contract:A,requires:&[B],run:a},Provider{contract:B,requires:&[A],run:b}];
    assert!(matches!(r.admit(bundle,&[]),Err(Error::Cycle(_))));
    assert_eq!(r.len(),5);
}
#[test]
fn wrong_signature_and_base_overwrite_are_rejected(){
    let mut r=Registry::baseline();
    let mut changed=ADD;changed.signature.assurance=Assurance::Rounded;
    let p=Provider{contract:A,requires:&[],run:identity};
    let mut fake=p.clone();fake.contract=changed;
    assert_eq!(r.admit(vec![fake],&[]),Err(Error::Duplicate(ADD.key)));
    let wrong_req=Provider{contract:A,requires:Box::leak(vec![changed].into_boxed_slice()),run:identity};
    assert_eq!(r.admit(vec![wrong_req],&[]),Err(Error::Incompatible(ADD.key)));
    assert_eq!(r.len(),5);
}
#[test]
fn wrong_result_and_undeclared_calls_fail_qualification(){
    let mut r=Registry::baseline();
    assert_eq!(r.admit(vec![Provider{contract:A,requires:&[],run:wrong}],&fixtures(A)),Err(Error::FixtureFailed(A.key)));
    assert_eq!(r.admit(vec![Provider{contract:A,requires:&[],run:sneaky}],&fixtures(A)),Err(Error::FixtureFailed(A.key)));
    assert_eq!(r.admit(vec![Provider{contract:A,requires:&[],run:identity}],&[]),Err(Error::Unqualified(A.key)));
    assert_eq!(r.len(),5);
}
#[test]
fn extension_chain_and_fuel_are_real(){
    let mut r=Registry::baseline();
    let mut cases=fixtures(A);cases.extend(fixtures(B));
    r.admit(vec![Provider{contract:A,requires:&[B],run:a},Provider{contract:B,requires:&[],run:identity}],&cases).unwrap();
    let p=r.plan(&[A]).unwrap();
    assert_eq!(r.call(&p,A,&[number(2,1)],1),Err(Error::Fuel));
    assert_eq!(r.call(&p,A,&[number(2,1)],2),Ok(number(2,1)));
    assert_eq!(r.call(&p,B,&[number(2,1)],2),Err(Error::Undeclared(B.key)));
}
#[test]
fn unrelated_and_divergent_snapshots_are_not_interchangeable(){
    let a=ready();let b=ready();
    let p=a.plan(&[ADD]).unwrap();
    assert_eq!(b.call(&p,ADD,&[number(1,1),number(2,1)],8),Err(Error::Stale));
    let mut fork=a.clone();
    assert!(fork.call(&p,ADD,&[number(1,1),number(2,1)],8).is_ok());
    fork.admit(vec![Provider{contract:A,requires:&[],run:identity}],&fixtures(A)).unwrap();
    assert_eq!(fork.call(&p,ADD,&[number(1,1),number(2,1)],8),Err(Error::Stale));
}
#[test]
fn calculator_chain_repeat_and_history(){
    let r=ready();let mut c=Calculator::new(&r).unwrap();
    assert_eq!(run(&mut c,&r,&[Digit(2),op(Binary::Add),Digit(3),op(Binary::Multiply),Digit(4),Equals]).display,"20");
    assert_eq!(c.history().len(),2);
    assert_eq!(run(&mut c,&r,&[Equals]).display,"80");
    assert_eq!(run(&mut c,&r,&[Undo]).display,"20");
    assert_eq!(c.history().len(),2);
    assert_eq!(run(&mut c,&r,&[AllClear]).display,"0");
    assert_eq!(c.history().len(),2);
    run(&mut c,&r,&[ClearHistory]);assert!(c.history().is_empty());
}
#[test]
fn exact_decimal_and_division_display(){
    let r=ready();let mut c=Calculator::new(&r).unwrap();
    assert_eq!(run(&mut c,&r,&[Point,Digit(1),op(Binary::Add),Point,Digit(2),Equals]).display,"0.3");
    let p=run(&mut c,&r,&[AllClear,Digit(1),op(Binary::Divide),Digit(3),Equals]);
    assert_eq!(p.display,"~0.333333333333");
    assert_eq!(Value::Number(p.value),number(1,3));assert!(!p.approximate_value);
    assert_eq!(run(&mut c,&r,&[op(Binary::Multiply),Digit(3),Equals]).display,"1");
}
#[test]
fn contextual_percent_and_standalone_percent(){
    let r=ready();let mut c=Calculator::new(&r).unwrap();
    for (b,expected) in [(Binary::Add,"220"),(Binary::Subtract,"180"),(Binary::Multiply,"20"),(Binary::Divide,"2000")]{
        assert_eq!(run(&mut c,&r,&[AllClear,Digit(2),Digit(0),Digit(0),op(b),Digit(1),Digit(0),Percent,Equals]).display,expected);
    }
    assert_eq!(run(&mut c,&r,&[AllClear,Digit(2),Digit(5),Percent]).display,"0.25");
}
#[test]
fn editing_memory_and_operator_replacement(){
    let r=ready();let mut c=Calculator::new(&r).unwrap();
    assert_eq!(run(&mut c,&r,&[Digit(1),Digit(2),Backspace,Digit(3),Sign]).display,"-13");
    run(&mut c,&r,&[MemoryStore,AllClear,Digit(3),MemoryAdd,AllClear,MemoryRecall]);
    assert_eq!(c.projection(&r).unwrap().display,"-10");
    run(&mut c,&r,&[MemoryClear]);
    assert!(!c.projection(&r).unwrap().memory);
    assert_eq!(run(&mut c,&r,&[AllClear,Digit(8),op(Binary::Add),op(Binary::Subtract),Digit(3),Equals]).display,"5");
    assert_eq!(run(&mut c,&r,&[AllClear,Digit(8),op(Binary::Add),Digit(9),ClearEntry,Digit(2),Equals]).display,"10");
}
#[test]
fn errors_do_not_commit_state(){
    let r=ready();let mut c=Calculator::new(&r).unwrap();
    let before=run(&mut c,&r,&[Digit(9),op(Binary::Divide),Digit(0)]);
    assert_eq!(c.press(&r,Equals),Err(Error::Math(MathError::DivisionByZero)));
    assert_eq!(c.projection(&r).unwrap(),before);
    assert_eq!(run(&mut c,&r,&[ClearEntry,Digit(3),Equals]).display,"3");
    let before=run(&mut c,&r,&[AllClear,Digit(1),Sign]);
    assert_eq!(c.press(&r,Sqrt),Err(Error::Math(MathError::Domain)));
    assert_eq!(c.projection(&r).unwrap(),before);
}
#[test]
fn approximation_is_sticky_across_calculation_and_memory(){
    let r=ready();let mut c=Calculator::new(&r).unwrap();
    let p=run(&mut c,&r,&[Digit(2),Sqrt]);
    assert_eq!(p.display,"~1.414213");assert!(p.approximate_value);
    let p=run(&mut c,&r,&[MemoryStore,AllClear,MemoryRecall,Square]);
    assert!(p.approximate_value);assert!(p.display.starts_with('~'));
    assert_eq!(run(&mut c,&r,&[AllClear,Digit(4),Reciprocal]).display,"0.25");
}
#[test]
fn new_extension_does_not_require_calculator_or_kernel_changes(){
    let mut r=ready();let mut c=Calculator::new(&r).unwrap();
    run(&mut c,&r,&[Digit(2)]);
    r.admit(vec![Provider{contract:A,requires:&[],run:identity}],&fixtures(A)).unwrap();
    assert_eq!(c.press(&r,Digit(3)),Err(Error::Stale));
    c.rebind(&r).unwrap();
    assert_eq!(run(&mut c,&r,&[Digit(3)]).display,"23");
}
#[test]
fn decimal_and_square_root_boundary_workpieces(){
    let r=ready();let p=r.plan(&[PARSE,DISPLAY,SQRT]).unwrap();
    assert_eq!(r.call(&p,PARSE,&[text("-170141183460469231731687303715884105728")],8),Ok(number(i128::MIN,1)));
    assert_eq!(r.call(&p,PARSE,&[text("170141183460469231731687303715884105728")],8),Err(Error::Math(MathError::Overflow)));
    assert_eq!(r.call(&p,DISPLAY,&[number(i128::MAX-1,i128::MAX)],8),Ok(text("~0.999999999999")));
    assert_eq!(r.call(&p,SQRT,&[number(0,1)],8),Ok(number(0,1)));
    assert_eq!(r.call(&p,SQRT,&[number(4,1)],8),Ok(number(2,1)));
}
