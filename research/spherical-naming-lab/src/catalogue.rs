//! Reproducible knowledge-area catalogue. A row is a research perspective,
//! not a claim of an independently established academic discipline.
use std::collections::BTreeSet;

pub const EXPECTED_AREAS: usize = 10_000;
pub const ANCHORS: &str = include_str!("../data/anchors.tsv");
pub const LENSES: &str = include_str!("../data/lenses.txt");
pub const MATERIALIZED: &str = include_str!("../data/catalogue.tsv");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Area {
    pub id: usize,
    pub discipline: &'static str,
    pub anchor: &'static str,
    pub lens: &'static str,
}
impl Area {
    pub fn key(&self) -> String {
        format!("K{:05}", self.id)
    }
    pub fn label(&self) -> String {
        format!("{} / {} / {}", self.discipline, self.anchor, self.lens)
    }
}

pub fn build() -> Result<Vec<Area>, String> {
    let lenses: Vec<_> = LENSES.lines().filter(|l| !l.is_empty()).collect();
    if lenses.len() != 10 || lenses.iter().copied().collect::<BTreeSet<_>>().len() != 10 {
        return Err("expected ten unique research lenses".into());
    }
    let mut disciplines = BTreeSet::new();
    let mut areas = Vec::with_capacity(EXPECTED_AREAS);
    for (line_no, line) in ANCHORS.lines().enumerate() {
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 11 || fields.iter().any(|s| s.trim().is_empty()) {
            return Err(format!("anchors.tsv line {}: expected discipline and ten anchors", line_no + 1));
        }
        if !disciplines.insert(fields[0]) {
            return Err(format!("duplicate discipline: {}", fields[0]));
        }
        if fields[1..].iter().copied().collect::<BTreeSet<_>>().len() != 10 {
            return Err(format!("duplicate anchor within {}", fields[0]));
        }
        for &anchor in &fields[1..] {
            for &lens in &lenses {
                areas.push(Area { id: areas.len() + 1, discipline: fields[0], anchor, lens });
            }
        }
    }
    if disciplines.len() != 100 || areas.len() != EXPECTED_AREAS {
        return Err(format!("catalogue cardinality mismatch: {} disciplines, {} areas", disciplines.len(), areas.len()));
    }
    Ok(areas)
}

pub fn render(areas: &[Area]) -> String {
    let mut out = String::from("id\tdiscipline\tanchor\tlens\n");
    for area in areas {
        out.push_str(&format!("{}\t{}\t{}\t{}\n", area.key(), area.discipline, area.anchor, area.lens));
    }
    out
}

pub fn validate() -> Result<(), String> {
    let areas = build()?;
    let unique: BTreeSet<_> = areas.iter().map(|a| (a.discipline, a.anchor, a.lens)).collect();
    if unique.len() != EXPECTED_AREAS {
        return Err("duplicate knowledge areas".into());
    }
    if render(&areas) != MATERIALIZED.replace("\r\n", "\n") {
        return Err("materialized catalogue does not match the generator".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cardinality_uniqueness_and_materialization() {
        validate().unwrap();
        let a = build().unwrap();
        assert_eq!(a.first().unwrap().key(), "K00001");
        assert_eq!(a.last().unwrap().key(), "K10000");
    }
}
