//! The calculator's missing capabilities are supplied as independent providers.
//! They extend the simulated host registry, not the frozen kernel.
use super::*;
pub const PARSE:Requirement=req("math.decimal.parse",Signature{args:&[Kind::Text],output:Kind::Number,assurance:Assurance::Exact});
pub const DISPLAY:Requirement=req("math.decimal.display.truncate12",Signature{args:&[Kind::Number],output:Kind::Text,assurance:Assurance::Rounded});
pub const PERCENT:Requirement=req("math.scalar.percent",UNARY);
pub const SQRT:Requirement=req("math.scalar.sqrt.floor6",Signature{args:&[Kind::Number],output:Kind::Number,assurance:Assurance::Rounded});

fn parse(_: &mut Call<'_>,input:&[Value])->Result<Value>{
    let s=input[0].text()?;
    if s.is_empty()||s.len()>39{return Err(Error::Invalid);}
    let (negative,s)=match s.strip_prefix('-'){Some(t)=>(true,t),None=>(false,s)};
    let(mut n,mut d,mut point,mut digits)=(0i128,1i128,false,0usize);
    for c in s.bytes(){
        if c==b'.'&&!point{point=true;continue;}
        if !c.is_ascii_digit(){return Err(Error::Invalid);}
        n=n.checked_mul(10).and_then(|n|if negative{n.checked_sub((c-b'0') as i128)}else{n.checked_add((c-b'0') as i128)}).ok_or(MathError::Overflow)?;
        if point{d=d.checked_mul(10).ok_or(MathError::Overflow)?;}
        digits+=1;
    }
    if digits==0{return Err(Error::Invalid);}
    Ok(Value::Number(Rational::new(n,d)?))
}
fn display(_: &mut Call<'_>,input:&[Value])->Result<Value>{
    let q=input[0].number()?;
    let d=q.denominator() as u128;let n=q.numerator().unsigned_abs();
    let mut rem=n%d;
    let mut out=format!("{}{}",if q.numerator()<0{"-"}else{""},n/d);
    if rem>0{
        out.push('.');
        for _ in 0..12{
            // Compute 10*rem/d without overflowing u128.
            let(mut digit,mut acc)=(0u8,0u128);
            for _ in 0..10{acc+=rem;if acc>=d{acc-=d;digit+=1;}}
            out.push(char::from(b'0'+digit));rem=acc;
            if rem==0{break;}
        }
    }
    if rem>0{out.insert(0,'~');}
    Ok(Value::Text(out))
}
fn percent(c:&mut Call<'_>,input:&[Value])->Result<Value>{
    c.call(DIV,&[input[0].clone(),number(100,1)])
}
/// Bounded floor square root, 6 decimal places. Exact squares are not
/// specially optimized; the registered assurance is Rounded even for 4.
fn sqrt(_: &mut Call<'_>,input:&[Value])->Result<Value>{
    let q=input[0].number()?;
    if q.numerator()<0{return Err(MathError::Domain.into());}
    let scaled=q.numerator().checked_mul(1_000_000_000_000).ok_or(MathError::Overflow)?/q.denominator();
    let(mut lo,mut hi)=(0u128,(scaled as u128).min(1u128<<64)+1);
    while lo+1<hi{
        let mid=lo+(hi-lo)/2;
        if mid<=scaled as u128/mid{lo=mid;}else{hi=mid;}
    }
    Ok(Value::Number(Rational::new(lo as i128,1_000_000)?))
}
pub fn providers()->Vec<Provider>{
    vec![
        Provider{contract:PARSE,requires:&[],run:parse},
        Provider{contract:DISPLAY,requires:&[],run:display},
        Provider{contract:PERCENT,requires:&[DIV],run:percent},
        Provider{contract:SQRT,requires:&[],run:sqrt},
    ]
}
/// External reference fixtures for this experiment; not general certification.
pub fn reference_fixtures()->Vec<Fixture>{
    let t=|s:&str|Value::Text(s.into());
    vec![
        Fixture{capability:PARSE,input:vec![t("-12.50")],expected:Ok(number(-25,2))},
        Fixture{capability:PARSE,input:vec![t("1.2.3")],expected:Err(Error::Invalid)},
        Fixture{capability:DISPLAY,input:vec![number(1,8)],expected:Ok(t("0.125"))},
        Fixture{capability:DISPLAY,input:vec![number(1,3)],expected:Ok(t("~0.333333333333"))},
        Fixture{capability:DISPLAY,input:vec![t("bad")],expected:Err(Error::Type)},
        Fixture{capability:PERCENT,input:vec![number(25,1)],expected:Ok(number(1,4))},
        Fixture{capability:PERCENT,input:vec![number(1,i128::MAX)],expected:Err(Error::Math(MathError::Overflow))},
        Fixture{capability:SQRT,input:vec![number(2,1)],expected:Ok(number(1414213,1000000))},
        Fixture{capability:SQRT,input:vec![number(-1,1)],expected:Err(Error::Math(MathError::Domain))},
    ]
}
