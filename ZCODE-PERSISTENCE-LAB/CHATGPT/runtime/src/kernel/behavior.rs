//! A small deterministic behavioral algebra. These are bounded operations, not
//! claims of human-level reasoning; new behaviors compose through typed graphs.
use super::model::{bounded,ensure,MAX_GRAPH_NODES};
use crate::{Operator,Result,digest};
use serde_json::{Value,json,Map};
use std::collections::{BTreeMap,BTreeSet};
fn array<'a>(v:&'a Value,k:&str)->Result<&'a Vec<Value>>{
    let a=v.get(k).and_then(Value::as_array).ok_or_else(||format!("missing array {k}"))?;
    ensure(a.len()<=MAX_GRAPH_NODES,"array budget exceeded")?;Ok(a)
}
fn string<'a>(v:&'a Value,k:&str)->Result<&'a str>{v.get(k).and_then(Value::as_str).ok_or_else(||format!("missing string {k}"))}
fn integer(v:&Value,k:&str)->Result<i64>{v.get(k).and_then(Value::as_i64).ok_or_else(||format!("missing integer {k}"))}
fn strings(v:&Value,k:&str)->Result<BTreeSet<String>>{
    array(v,k)?.iter().map(|x|x.as_str().map(str::to_owned).ok_or("expected strings".into())).collect()
}
fn operation(v:&Value)->Result<Value>{
    let op=string(v,"op")?;let x=v.get("left").ok_or("missing left operand")?;let y=v.get("right").ok_or("missing right operand")?;
    match op{
        "equal"=>Ok(json!(x==y)),
        "add"|"multiply"|"subtract"=>{
            let a=x.as_i64().ok_or("expected integer")?;let b=y.as_i64().ok_or("expected integer")?;
            let r=match op{"add"=>a.checked_add(b),"multiply"=>a.checked_mul(b),_=>a.checked_sub(b)}.ok_or("integer overflow")?;Ok(json!(r))
        }
        "concat"=>Ok(json!(format!("{}{}",x.as_str().ok_or("expected string")?,y.as_str().ok_or("expected string")?))),
        _=>Err("unsupported deterministic operation".into())
    }
}
pub fn evaluate(op:&Operator,v:&Value)->Result<Value>{
    bounded(v)?;
    let out=match op{
        Operator::Observe=>{
            let value=v.get("value").ok_or("missing observed value")?;
            json!({"digest":digest(value),"shape":match value{Value::Null=>"null",Value::Bool(_)=>"boolean",Value::Number(_)=>"number",Value::String(_)=>"string",Value::Array(_)=>"array",Value::Object(_)=>"record"},"bytes":serde_json::to_vec(value).map_err(|e|e.to_string())?.len()})
        }
        Operator::Discover=>{
            let needle=v.get("value").ok_or("missing value")?;
            let matches:Vec<_>=array(v,"candidates")?.iter().enumerate().filter(|(_,x)|*x==needle).map(|(i,_)|i).collect();json!(matches)
        }
        Operator::Reason=>{
            let mut facts=strings(v,"facts")?;
            let rules=array(v,"rules")?;
            for _ in 0..=rules.len(){
                let mut changed=false;
                for r in rules{let required=strings(r,"requires")?;
                    if required.is_subset(&facts){changed|=facts.insert(string(r,"concludes")?.into());}
                }
                if !changed{break;}
            }json!(facts)
        }
        Operator::Decompose=>{
            let size=usize::try_from(integer(v,"batch_size")?).map_err(|_|"invalid batch size")?;
            ensure(size>0&&size<=MAX_GRAPH_NODES,"invalid batch size")?;
            json!(array(v,"items")?.chunks(size).collect::<Vec<_>>())
        }
        Operator::Plan=>{
            let mut tasks:BTreeMap<String,BTreeSet<String>>=BTreeMap::new();
            for t in array(v,"tasks")?{let key=string(t,"id")?.to_owned();ensure(tasks.insert(key,strings(t,"dependencies")?).is_none(),"duplicate task")?;}
            for deps in tasks.values(){ensure(deps.iter().all(|d|tasks.contains_key(d)),"missing dependency")?;}
            let mut done=BTreeSet::new();let mut order=vec![];
            while done.len()<tasks.len(){
                let ready:Vec<_>=tasks.iter().filter(|(k,d)|!done.contains(*k)&&d.is_subset(&done)).map(|(k,_)|k.clone()).collect();
                ensure(!ready.is_empty(),"dependency cycle")?;
                for key in ready{done.insert(key.clone());order.push(key);}
            }json!(order)
        }
        Operator::Construct=>{
            let mut obj=Map::new();for p in array(v,"fields")?{let name=string(p,"key")?.to_owned();let value=p.get("value").ok_or("missing field value")?.clone();ensure(obj.insert(name,value).is_none(),"duplicate field")?;}Value::Object(obj)
        }
        Operator::Execute=>operation(v)?,
        Operator::Experiment=>{
            let mut results=vec![];
            for case in array(v,"cases")?{
                let result=operation(case);results.push(match result{Ok(value)=>json!({"ok":true,"value":value}),Err(reason)=>json!({"ok":false,"reason":reason})});
            }json!(results)
        }
        Operator::Challenge=>{
            let cases=array(v,"cases")?;let mut failures=vec![];
            for (i,c) in cases.iter().enumerate(){let expected=c.get("expected").ok_or("missing expected output")?;
                match operation(c){Ok(actual) if &actual==expected=>{},Ok(actual)=>failures.push(json!({"index":i,"expected":expected,"actual":actual})),Err(reason)=>failures.push(json!({"index":i,"reason":reason}))}
            }json!(failures)
        }
        Operator::Verify=>{
            let actual=v.get("actual").ok_or("missing actual")?;
            let accepted=if let Some(expected)=v.get("expected"){actual==expected}else{digest(actual)==string(v,"expected_digest")?};
            json!({"accepted":accepted,"digest":digest(actual)})
        }
        Operator::Repair=>{
            let excluded=strings(v,"excluded")?;let mut eligible=vec![];
            for c in array(v,"alternatives")?{let key=string(c,"id")?;let cost=integer(c,"cost")?;
                if !excluded.contains(key)&&cost>=0{eligible.push((cost,key,c));}
            }eligible.sort_by(|a,b|(a.0,a.1).cmp(&(b.0,b.1)));eligible.first().map(|x|x.2.clone()).unwrap_or(Value::Null)
        }
        Operator::Synthesize=>{
            let mut result=Map::new();
            for r in array(v,"records")?{for (k,x) in r.as_object().ok_or("expected record")?{
                if let Some(old)=result.get(k){ensure(old==x,"conflicting evidence")?;}else{result.insert(k.clone(),x.clone());}
            }}Value::Object(result)
        }
        Operator::Optimize=>{
            let mut best:Option<(i64,&str,&Value)>=None;
            for c in array(v,"candidates")?{
                let cost=integer(c,"cost")?;let key=string(c,"id")?;
                if c.get("feasible").and_then(Value::as_bool)==Some(true)&&cost>=0&&best.as_ref().is_none_or(|b|(cost,key)<(b.0,b.1)){best=Some((cost,key,c));}
            }best.map(|x|x.2.clone()).unwrap_or(Value::Null)
        }
        Operator::Delegate=>{
            let slots=usize::try_from(integer(v,"slots")?).map_err(|_|"invalid slots")?;
            ensure((1..=MAX_GRAPH_NODES).contains(&slots),"invalid slots")?;
            let mut jobs=vec![];let mut seen=BTreeSet::new();
            for j in array(v,"jobs")?{let key=string(j,"id")?;let cost=integer(j,"cost")?;ensure(cost>=0&&seen.insert(key),"invalid job")?;jobs.push((cost,key));}
            jobs.sort_by(|a,b|(std::cmp::Reverse(a.0),a.1).cmp(&(std::cmp::Reverse(b.0),b.1)));
            let mut loads=vec![0i128;slots];let mut assignments=vec![];
            for (cost,key) in jobs{let slot=loads.iter().enumerate().min_by_key(|(i,x)|(**x,*i)).unwrap().0;loads[slot]+=i128::from(cost);assignments.push(json!({"job":key,"slot":slot}));}json!(assignments)
        }
        Operator::Learn=>{
            let samples=array(v,"examples")?;ensure(samples.len()>=2,"need two examples")?;
            let x0=i128::from(integer(&samples[0],"input")?);let y0=i128::from(integer(&samples[0],"output")?);
            let mut candidate=None;
            for e in samples.iter().skip(1){let dx=i128::from(integer(e,"input")?)-x0;let dy=i128::from(integer(e,"output")?)-y0;
                if dx!=0&&dy%dx==0{let m=dy/dx;let b=m.checked_mul(x0).and_then(|x|y0.checked_sub(x)).ok_or("coefficient overflow")?;candidate=Some((m,b));break;}
            }
            let (m,b)=candidate.ok_or("no integer affine candidate")?;
            for e in samples{let x=i128::from(integer(e,"input")?);let y=i128::from(integer(e,"output")?);ensure(m.checked_mul(x).and_then(|v|v.checked_add(b))==Some(y),"non-affine examples")?;}
            json!({"slope":i64::try_from(m).map_err(|_|"slope overflow")?,"offset":i64::try_from(b).map_err(|_|"offset overflow")?})
        }
    };bounded(&out)?;Ok(out)
}
