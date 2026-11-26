use crate::Record;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
#[derive(Debug, Clone)]
pub struct Goal {
    terms: Vec<String>,
}
impl Goal {
    pub fn new(s: &str) -> Self {
        let mut terms: Vec<_> = tokens(s)
            .into_iter()
            .filter(|t| {
                ![
                    "the", "a", "an", "of", "for", "find", "me", "where", "is", "get", "in", "to",
                    "and",
                ]
                .contains(&t.as_str())
            })
            .collect();
        terms.sort();
        terms.dedup();
        Self { terms }
    }
}
fn tokens(s: &str) -> Vec<String> {
    let mut normalized = String::new();
    let mut prev = false;
    for c in s.chars() {
        if prev && c.is_uppercase() {
            normalized.push(' ')
        }
        normalized.push(c);
        prev = c.is_lowercase()
    }
    normalized
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| match s.to_lowercase().as_str() {
            "message" | "content" | "reply" | "answer" | "text" | "delta" => "text".into(),
            "cost" | "price" => "price".into(),
            "identifier" | "id" => "id".into(),
            v => v.into(),
        })
        .collect()
}
#[derive(Debug, Serialize)]
pub struct Hit {
    pub source: String,
    pub path: String,
    pub schema_path: String,
    pub score: f32,
    pub matched: Vec<String>,
    pub start: u64,
    pub end: u64,
    pub value: Value,
}
pub struct Locator {
    goal: Goal,
    counts: BTreeMap<(String, String), u64>,
}
impl Locator {
    pub fn new(goal: Goal) -> Self {
        Self {
            goal,
            counts: BTreeMap::new(),
        }
    }
    pub fn locate(&mut self, source: &str, r: &Record) -> Vec<Hit> {
        let mut hits = vec![];
        let mut stack = vec![("".to_string(), "".to_string(), &r.value)];
        while let Some((p, s, v)) = stack.pop() {
            match v {
                Value::Object(m) => {
                    for (k, x) in m {
                        let k = k.replace('~', "~0").replace('/', "~1");
                        stack.push((format!("{p}/{k}"), format!("{s}/{k}"), x))
                    }
                }
                Value::Array(a) => {
                    for (i, x) in a.iter().enumerate() {
                        stack.push((format!("{p}/{i}"), format!("{s}/*"), x))
                    }
                }
                _ => {
                    let pt = tokens(&s);
                    let vt = tokens(&v.to_string());
                    let st = tokens(source);
                    let mut score = 0.;
                    let mut matched = vec![];
                    for t in &self.goal.terms {
                        let w = if pt.contains(t) {
                            3.
                        } else if vt.contains(t) {
                            2.
                        } else if st.contains(t) {
                            0.5
                        } else {
                            0.
                        };
                        if w > 0. {
                            matched.push(t.clone());
                            score += w
                        }
                    }
                    if score >= 2. {
                        let key = (source.to_string(), s.clone());
                        if let Some(n) = self.counts.get_mut(&key) {
                            *n = n.saturating_add(1)
                        } else if self.counts.len() < 4096 {
                            self.counts.insert(key, 1);
                        }
                        hits.push(Hit {
                            source: source.into(),
                            path: p,
                            schema_path: s,
                            score: score / (3. * self.goal.terms.len().max(1) as f32),
                            matched,
                            start: r.start,
                            end: r.end,
                            value: v.clone(),
                        });
                    }
                }
            }
        }
        hits.sort_by(|a, b| b.score.total_cmp(&a.score).then(a.path.cmp(&b.path)));
        hits.truncate(32);
        hits
    }
    pub fn learned(&self) -> Vec<(&str, &str, u64)> {
        let mut v: Vec<_> = self
            .counts
            .iter()
            .map(|((a, b), n)| (a.as_str(), b.as_str(), *n))
            .collect();
        v.sort_by(|a, b| b.2.cmp(&a.2));
        v
    }
}
