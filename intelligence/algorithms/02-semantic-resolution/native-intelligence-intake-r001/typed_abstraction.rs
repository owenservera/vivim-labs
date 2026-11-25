//! Labs-only, independently authored bounded typed sequence anti-unification.
//! Research inspiration: Stitch (POPL 2023), pinned source in README.
//! No execution, capability admission, persistence, or effect authority.
//! Compare this restricted baseline to actual VEC typed programs before promotion.

use std::collections::BTreeMap;

const MAX_STEPS: usize = 64;
const MAX_ARGS: usize = 16;
const MAX_TEXT: usize = 1024;
const MAX_HOLES: usize = 32;
const MAX_EXAMPLES: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Value {
    pub kind: String,
    pub text: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Step {
    pub capability: String,
    pub signature_revision: String,
    pub args: Vec<Value>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Trace {
    pub scope: String,
    pub catalog_revision: String,
    pub steps: Vec<Step>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Arg {
    Fixed(Value),
    Hole { id: usize, kind: String },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TemplateStep {
    pub capability: String,
    pub signature_revision: String,
    pub args: Vec<Arg>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Template {
    pub scope: String,
    pub catalog_revision: String,
    pub steps: Vec<TemplateStep>,
    pub hole_kinds: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Refusal {
    Bounds, Shape, Scope, Revision, Type, Binding, NoVariation,
}

fn valid_text(s: &str) -> bool { !s.is_empty() && s.len() <= MAX_TEXT }
fn valid_value(v: &Value) -> bool {
    valid_text(&v.kind) && v.text.len() <= MAX_TEXT
}
fn validate(trace: &Trace) -> Result<(), Refusal> {
    if !valid_text(&trace.scope) || !valid_text(&trace.catalog_revision)
        || trace.steps.is_empty() || trace.steps.len() > MAX_STEPS {
        return Err(Refusal::Bounds);
    }
    for s in &trace.steps {
        if !valid_text(&s.capability) || !valid_text(&s.signature_revision)
            || s.args.len() > MAX_ARGS || s.args.iter().any(|a| !valid_value(a)) {
            return Err(Refusal::Bounds);
        }
    }
    Ok(())
}

/// Common shape plus typed holes. A hole is shared only when its entire
/// ordered vector of observed values matches across all training examples.
/// Equality in training is a hypothesis, not proof of semantic equivalence.
pub fn generalize(examples: &[Trace]) -> Result<Template, Refusal> {
    if !(2..=MAX_EXAMPLES).contains(&examples.len()) {
        return Err(Refusal::Bounds);
    }
    let first = &examples[0];
    for e in examples {
        validate(e)?;
        if e.scope != first.scope { return Err(Refusal::Scope); }
        if e.catalog_revision != first.catalog_revision { return Err(Refusal::Revision); }
        if e.steps.len() != first.steps.len() { return Err(Refusal::Shape); }
    }
    let mut holes: BTreeMap<Vec<Value>, usize> = BTreeMap::new();
    let mut hole_kinds = Vec::new();
    let mut steps = Vec::new();
    for (i, step) in first.steps.iter().enumerate() {
        for e in examples {
            let other = &e.steps[i];
            if other.capability != step.capability || other.args.len() != step.args.len() {
                return Err(Refusal::Shape);
            }
            if other.signature_revision != step.signature_revision {
                return Err(Refusal::Revision);
            }
        }
        let mut args = Vec::new();
        for (j, arg) in step.args.iter().enumerate() {
            let column: Vec<Value> = examples.iter().map(|e| e.steps[i].args[j].clone()).collect();
            if column.iter().any(|v| v.kind != arg.kind) { return Err(Refusal::Type); }
            if column.iter().all(|v| v == arg) {
                args.push(Arg::Fixed(arg.clone()));
            } else {
                let id = if let Some(&id) = holes.get(&column) { id } else {
                    if hole_kinds.len() >= MAX_HOLES { return Err(Refusal::Bounds); }
                    let id = hole_kinds.len();
                    hole_kinds.push(arg.kind.clone());
                    holes.insert(column, id);
                    id
                };
                args.push(Arg::Hole { id, kind: arg.kind.clone() });
            }
        }
        steps.push(TemplateStep {
            capability: step.capability.clone(),
            signature_revision: step.signature_revision.clone(), args,
        });
    }
    if hole_kinds.is_empty() { return Err(Refusal::NoVariation); }
    Ok(Template { scope: first.scope.clone(), catalog_revision: first.catalog_revision.clone(), steps, hole_kinds })
}

/// Structural matching only. Caller must separately check current source,
/// registry contracts, permissions and value schemas before any admission.
pub fn match_trace(t: &Template, trace: &Trace) -> Result<Vec<Value>, Refusal> {
    validate(trace)?;
    if t.hole_kinds.len() > MAX_HOLES || t.steps.len() > MAX_STEPS {
        return Err(Refusal::Bounds);
    }
    if t.scope != trace.scope { return Err(Refusal::Scope); }
    if t.catalog_revision != trace.catalog_revision { return Err(Refusal::Revision); }
    if t.steps.len() != trace.steps.len() { return Err(Refusal::Shape); }
    let mut bindings = vec![None::<Value>; t.hole_kinds.len()];
    for (a,b) in t.steps.iter().zip(&trace.steps) {
        if a.capability != b.capability || a.args.len() != b.args.len() {
            return Err(Refusal::Shape);
        }
        if a.signature_revision != b.signature_revision { return Err(Refusal::Revision); }
        for (p,v) in a.args.iter().zip(&b.args) {
            match p {
                Arg::Fixed(expected) if expected != v => return Err(Refusal::Binding),
                Arg::Fixed(_) => {},
                Arg::Hole { id, kind } => {
                    if kind != &v.kind || t.hole_kinds.get(*id) != Some(kind) {
                        return Err(Refusal::Type);
                    }
                    let slot = bindings.get_mut(*id).ok_or(Refusal::Binding)?;
                    if let Some(old) = slot.as_ref() {
                        if old != v { return Err(Refusal::Binding); }
                    } else { *slot = Some(v.clone()); }
                }
            }
        }
    }
    bindings.into_iter().map(|v| v.ok_or(Refusal::Binding)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn value(kind: &str, text: &str) -> Value {
        Value { kind: kind.into(), text: text.into() }
    }
    fn trace(path: &str) -> Trace {
        Trace { scope: "lab-public".into(), catalog_revision: "r1".into(),
            steps: ["read", "inspect"].iter().map(|cap| Step {
                capability: cap.to_string(), signature_revision: "s1".into(),
                args: vec![value("path",path),value("format","summary")],
            }).collect() }
    }
    fn template() -> Template { generalize(&[trace("a.rs"),trace("b.rs")]).unwrap() }
    #[test] fn unseen_binding_matches() {
        let t=template();
        assert_eq!(t.hole_kinds,vec!["path".to_string()]);
        assert_eq!(match_trace(&t,&trace("c.rs")),Ok(vec![value("path","c.rs")]));
    }
    #[test] fn shared_binding_enforced() {
        let mut x=trace("c.rs"); x.steps[1].args[0]=value("path","d.rs");
        assert_eq!(match_trace(&template(),&x),Err(Refusal::Binding));
    }
    #[test] fn fixed_arg_enforced() {
        let mut x=trace("c.rs"); x.steps[0].args[1]=value("format","full");
        assert_eq!(match_trace(&template(),&x),Err(Refusal::Binding));
    }
    #[test] fn refuses_scope_and_revision_drift() {
        let mut x=trace("c.rs"); x.scope="other".into();
        assert_eq!(match_trace(&template(),&x),Err(Refusal::Scope));
        x.scope="lab-public".into(); x.catalog_revision="r2".into();
        assert_eq!(match_trace(&template(),&x),Err(Refusal::Revision));
        x.catalog_revision="r1".into(); x.steps[0].signature_revision="s2".into();
        assert_eq!(match_trace(&template(),&x),Err(Refusal::Revision));
    }
    #[test] fn refuses_type_and_shape_changes() {
        let mut x=trace("c.rs"); x.steps[0].args[0]=value("url","c.rs");
        assert_eq!(generalize(&[trace("a.rs"),x.clone()]),Err(Refusal::Type));
        x.steps[0].capability="write".into();
        assert_eq!(generalize(&[trace("a.rs"),x]),Err(Refusal::Shape));
    }
    #[test] fn no_variation_and_bounds() {
        assert_eq!(generalize(&[trace("a.rs"),trace("a.rs")]),Err(Refusal::NoVariation));
        assert_eq!(generalize(&[]),Err(Refusal::Bounds));
        let mut x=trace("a.rs"); x.steps[0].args[0].text="x".repeat(MAX_TEXT+1);
        assert_eq!(generalize(&[trace("b.rs"),x]),Err(Refusal::Bounds));
    }
    #[test] fn independent_holes_when_evidence_differs() {
        let mut a=trace("a.rs"); let mut b=trace("b.rs");
        a.steps[1].args[0]=value("path","x.rs"); b.steps[1].args[0]=value("path","y.rs");
        let t=generalize(&[a,b]).unwrap();
        assert_eq!(t.hole_kinds.len(),2);
        let mut c=trace("c.rs"); c.steps[1].args[0]=value("path","z.rs");
        assert_eq!(match_trace(&t,&c),Ok(vec![value("path","c.rs"),value("path","z.rs")]));
    }
}
