//! Learn extraction from independent, owner-labelled observation windows.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
const MAX_TRIALS: usize = 1024;
const MAX_FIELDS: usize = 4096;
const MAX_TEXT: usize = 1024 * 1024;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Step {
    Field(String),
    Each,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Mode {
    Latest,
    Append,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Plan {
    pub source: String,
    pub path: Vec<Step>,
    pub mode: Mode,
}
/// Stable source is assigned by the capture adapter; stream identifies one request instance.
pub struct Observation<'a> {
    pub source: &'a str,
    pub stream: &'a str,
    pub start: u64,
    pub end: u64,
    pub value: &'a Value,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub stream: String,
    pub start: u64,
    pub end: u64,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Extraction {
    pub text: String,
    pub spans: Vec<Span>,
}
#[derive(Default)]
struct Series {
    latest: String,
    append: String,
    spans: Vec<Span>,
    stream: String,
    ambiguous: bool,
    overflow: bool,
}
type Key = (String, Vec<Step>);
fn gather(records: &[Observation<'_>]) -> Result<BTreeMap<Key, Series>, String> {
    if records.len() > 4096 {
        return Err("observation limit".into());
    }
    let mut map: BTreeMap<Key, Series> = BTreeMap::new();
    let mut bytes = 0usize;
    let mut nodes = 0usize;
    for r in records {
        if r.end < r.start {
            return Err("invalid span".into());
        }
        let mut stack = vec![(vec![], r.value)];
        while let Some((path, v)) = stack.pop() {
            nodes += 1;
            if nodes > 65536 || path.len() > 64 {
                return Err("tree budget".into());
            }
            match v {
                Value::Object(m) => {
                    if m.len() > 65536 - nodes || stack.len() + m.len() > 65536 {
                        return Err("tree budget".into());
                    }
                    for (k, v) in m.iter().rev() {
                        let mut p = path.clone();
                        p.push(Step::Field(k.clone()));
                        stack.push((p, v));
                    }
                }
                Value::Array(a) => {
                    if a.len() > 65536 - nodes || stack.len() + a.len() > 65536 {
                        return Err("tree budget".into());
                    }
                    for v in a.iter().rev() {
                        let mut p = path.clone();
                        p.push(Step::Each);
                        stack.push((p, v));
                    }
                }
                Value::String(s) => {
                    let key = (r.source.to_owned(), path);
                    if !map.contains_key(&key) && map.len() >= MAX_FIELDS {
                        return Err("field budget".into());
                    }
                    let a = map.entry(key).or_default();
                    if !a.spans.is_empty() && a.stream != r.stream {
                        a.ambiguous = true;
                        continue;
                    }
                    if a.spans.is_empty() {
                        a.stream = r.stream.into()
                    }
                    if a.spans.last().is_some_and(|p| r.start < p.start) {
                        return Err("out-of-order observations".into());
                    }
                    bytes = bytes.saturating_add(s.len());
                    if bytes > 8 * MAX_TEXT {
                        return Err("window text budget".into());
                    }
                    if s.len() > MAX_TEXT || a.append.len().saturating_add(s.len()) > MAX_TEXT {
                        a.overflow = true;
                        continue;
                    }
                    a.latest = s.clone();
                    a.append.push_str(s);
                    a.spans.push(Span {
                        stream: r.stream.into(),
                        start: r.start,
                        end: r.end,
                    });
                }
                _ => {}
            }
        }
    }
    Ok(map)
}

impl Series {
    fn result(&self, mode: Mode) -> Option<Extraction> {
        if self.ambiguous || self.overflow || self.spans.is_empty() {
            return None;
        }
        let (text, spans) = match mode {
            Mode::Latest => (self.latest.clone(), vec![self.spans.last()?.clone()]),
            Mode::Append => (self.append.clone(), self.spans.clone()),
        };
        Some(Extraction { text, spans })
    }
}
#[derive(Debug, Default, Clone, Serialize)]
pub struct Evidence {
    pub matches: u32,
    pub wrong: u32,
    pub false_positives: u32,
    pub uncertain: u32,
}
#[derive(Debug, Clone, Serialize)]
pub struct Ranked {
    pub plan: Plan,
    pub evidence: Evidence,
    pub missed: u32,
}
#[derive(Default)]
pub struct Learner {
    ids: BTreeSet<String>,
    positive: u32,
    negative: u32,
    outputs: BTreeSet<(usize, u64)>,
    candidates: BTreeMap<Plan, Evidence>,
}
impl Learner {
    pub fn observe(
        &mut self,
        id: &str,
        expected: Option<&str>,
        records: &[Observation<'_>],
    ) -> Result<(), String> {
        if id.is_empty() || id.len() > 256 || self.ids.contains(id) {
            return Err("invalid or duplicate trial identity".into());
        }
        if self.ids.len() >= MAX_TRIALS {
            return Err("trial budget".into());
        }
        if expected.is_some_and(|s| s.is_empty() || s.len() > MAX_TEXT) {
            return Err("invalid expected output".into());
        }
        let map = gather(records)?;
        let new = map
            .keys()
            .flat_map(|(source, path)| {
                [Mode::Latest, Mode::Append].map(|mode| Plan {
                    source: source.clone(),
                    path: path.clone(),
                    mode,
                })
            })
            .filter(|p| !self.candidates.contains_key(p))
            .count();
        if self.candidates.len() + new > 2 * MAX_FIELDS {
            return Err("candidate budget".into());
        }
        self.ids.insert(id.into());
        if let Some(s) = expected {
            self.positive += 1;
            let mut h = std::collections::hash_map::DefaultHasher::new();
            s.hash(&mut h);
            self.outputs.insert((s.len(), h.finish()));
        } else {
            self.negative += 1
        }
        for ((source, path), series) in map {
            for mode in [Mode::Latest, Mode::Append] {
                let plan = Plan {
                    source: source.clone(),
                    path: path.clone(),
                    mode,
                };
                let e = self.candidates.entry(plan).or_default();
                match (expected, series.result(mode)) {
                    (Some(s), Some(x)) if x.text == s => e.matches += 1,
                    (Some(_), Some(_)) => e.wrong += 1,
                    (None, Some(x)) if !x.text.is_empty() => e.false_positives += 1,
                    (_, None) => e.uncertain += 1,
                    _ => {}
                }
            }
        }
        Ok(())
    }
    pub fn rank(&self) -> Vec<Ranked> {
        let mut rows: Vec<_> = self
            .candidates
            .iter()
            .map(|(p, e)| Ranked {
                plan: p.clone(),
                evidence: e.clone(),
                missed: self.positive.saturating_sub(e.matches + e.wrong),
            })
            .collect();
        rows.sort_by(|a, b| {
            b.evidence
                .matches
                .cmp(&a.evidence.matches)
                .then(
                    (a.evidence.wrong
                        + a.evidence.false_positives
                        + a.evidence.uncertain
                        + a.missed)
                        .cmp(
                            &(b.evidence.wrong
                                + b.evidence.false_positives
                                + b.evidence.uncertain
                                + b.missed),
                        ),
                )
                .then(a.plan.cmp(&b.plan))
        });
        rows
    }
    pub fn select(&self) -> Result<Plan, String> {
        if self.positive < 3 || self.outputs.len() < 2 || self.negative < 1 {
            return Err(
                "need three positive trials, two distinct outputs and one negative control".into(),
            );
        }
        let mut eligible = self.candidates.iter().filter(|(_, e)| {
            e.matches == self.positive && e.wrong == 0 && e.false_positives == 0 && e.uncertain == 0
        });
        let (p, _) = eligible.next().ok_or("no consistent plan")?;
        if eligible.next().is_some() {
            return Err("ambiguous plans: supply a discriminating example".into());
        }
        Ok(p.clone())
    }
}

impl Plan {
    /// Fast path: visit only the learned fields, not every leaf. Missing fields may be control frames.
    pub fn execute(&self, records: &[Observation<'_>]) -> Result<Option<Extraction>, String> {
        if self.path.len() > 64 || records.len() > 4096 {
            return Err("execution budget".into());
        }
        let mut series = Series::default();
        let mut visited = 0usize;
        let mut last_end: BTreeMap<&str, u64> = BTreeMap::new();
        for r in records.iter().filter(|r| r.source == self.source) {
            if r.end < r.start || last_end.get(r.stream).is_some_and(|end| r.start < *end) {
                return Err("invalid or overlapping observations".into());
            }
            last_end.insert(r.stream, r.end);
            let mut current = vec![r.value];
            for step in &self.path {
                let mut next = vec![];
                for v in current {
                    visited += 1;
                    if visited > 65536 {
                        return Err("traversal budget".into());
                    }
                    match (step, v) {
                        (Step::Field(k), Value::Object(m)) => {
                            if let Some(x) = m.get(k) {
                                next.push(x)
                            }
                        }
                        (Step::Each, Value::Array(a)) => {
                            if a.len() > 65536 - next.len() {
                                return Err("fanout budget".into());
                            }
                            next.extend(a)
                        }
                        _ => return Err("container type drift".into()),
                    }
                }
                current = next;
                if current.is_empty() {
                    break;
                }
            }
            for v in current {
                visited += 1;
                if visited > 65536 {
                    return Err("traversal budget".into());
                }
                let s = v.as_str().ok_or("terminal type drift")?;
                if !series.spans.is_empty() && series.stream != r.stream {
                    return Err("ambiguous concurrent stream instances".into());
                }
                if series.spans.is_empty() {
                    series.stream = r.stream.into()
                }
                if s.len() > MAX_TEXT || series.append.len().saturating_add(s.len()) > MAX_TEXT {
                    return Err("output limit".into());
                }
                series.latest = s.into();
                series.append.push_str(s);
                series.spans.push(Span {
                    stream: r.stream.into(),
                    start: r.start,
                    end: r.end,
                });
            }
        }
        Ok(series.result(self.mode))
    }
    /// Independent holdout check. The caller supplies a new observation window.
    pub fn validate(
        &self,
        expected: Option<&str>,
        records: &[Observation<'_>],
    ) -> Result<bool, String> {
        let actual = self.execute(records)?;
        Ok(match (expected, actual) {
            (Some(s), Some(x)) => s == x.text,
            (None, None) => true,
            (None, Some(x)) => x.text.is_empty(),
            _ => false,
        })
    }
}

/// Active-learning signal: prefer a window that separates surviving extraction plans.
/// No candidate output is returned or persisted.
#[derive(Debug, Clone, Serialize)]
pub struct Probe {
    pub candidates: usize,
    pub observable: usize,
    pub partitions: usize,
    pub separating_pairs: u64,
    pub comparable_pairs: u64,
}
impl Learner {
    pub fn probe(&self, records: &[Observation<'_>]) -> Result<Probe, String> {
        let map = gather(records)?;
        let mut groups: BTreeMap<Option<&str>, u64> = BTreeMap::new();
        let mut candidates = 0;
        let mut observable = 0;
        for (p, e) in &self.candidates {
            if self.positive == 0
                || e.matches != self.positive
                || e.wrong != 0
                || e.false_positives != 0
                || e.uncertain != 0
            {
                continue;
            }
            candidates += 1;
            let key = (p.source.clone(), p.path.clone());
            let val = match map.get(&key) {
                None => None,
                Some(s) if s.ambiguous || s.overflow => continue,
                Some(s) => Some(match p.mode {
                    Mode::Latest => s.latest.as_str(),
                    Mode::Append => s.append.as_str(),
                }),
            };
            observable += 1;
            *groups.entry(val).or_default() += 1;
        }
        let pairs = |n: u64| n * n.saturating_sub(1) / 2;
        let comparable_pairs = pairs(observable as u64);
        let same: u64 = groups.values().map(|n| pairs(*n)).sum();
        Ok(Probe {
            candidates,
            observable,
            partitions: groups.len(),
            separating_pairs: comparable_pairs - same,
            comparable_pairs,
        })
    }
}
