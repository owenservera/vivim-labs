//! Candidate matching, independent-list pairing and controlled random exploration.
use crate::catalogue::{self, Area};
use crate::geometry::{angle, Band, Encoder, Rng, Vector};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Candidate {
    pub role: String,
    pub name: String,
    pub meaning: String,
}
impl Candidate {
    pub fn vector(&self, encoder: &Encoder) -> Result<Vector, String> {
        encoder.encode(&[(&self.name, 1.0), (&self.meaning, 1.0), (&self.role, 0.15)])
    }
}

pub fn parse_candidates(tsv: &str) -> Result<Vec<Candidate>, String> {
    let mut lines = tsv.trim_start_matches('\u{feff}').lines();
    if lines.next() != Some("role\tname\tmeaning") {
        return Err("candidate TSV must begin with role<TAB>name<TAB>meaning".into());
    }
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for (i, line) in lines.enumerate() {
        if line.trim().is_empty() { continue; }
        let f: Vec<_> = line.split('\t').map(str::trim).collect();
        if f.len() != 3 || f.iter().any(|s| s.is_empty() || s.chars().any(char::is_control)) {
            return Err(format!("candidate line {}: expected three nonempty clean fields", i + 2));
        }
        let key = (f[0].to_lowercase(), f[1].to_lowercase());
        if !seen.insert(key) {
            return Err(format!("duplicate candidate in role {}: {}", f[0], f[1]));
        }
        out.push(Candidate { role: f[0].into(), name: f[1].into(), meaning: f[2].into() });
    }
    if out.is_empty() { return Err("no candidates".into()); }
    Ok(out)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hit {
    pub index: usize,
    pub degrees: f64,
}

pub struct Index {
    pub areas: Vec<Area>,
    vectors: Vec<Vector>,
}
impl Index {
    pub fn build(encoder: &Encoder) -> Result<Self, String> {
        let areas = catalogue::build()?;
        let vectors = areas.iter().map(|a| encoder.encode(&[
            (a.anchor, 1.4), (a.discipline, 0.30), (a.lens, 0.15)
        ])).collect::<Result<Vec<_>, _>>()?;
        Ok(Self { areas, vectors })
    }
    pub fn rank(&self, candidate: &[f64]) -> Vec<Hit> {
        let mut ranked: Vec<_> = self.vectors.iter().enumerate()
            .map(|(index, v)| Hit { index, degrees: angle(candidate, v) }).collect();
        ranked.sort_by(|a, b| a.degrees.total_cmp(&b.degrees).then(a.index.cmp(&b.index)));
        ranked
    }
    /// At most one research lens per discipline/anchor, avoiding repeated
    /// variations of the same concept overwhelming the random shortlist.
    pub fn shortlist(&self, ranked: &[Hit], top: usize, band: Band) -> Result<Vec<Hit>, String> {
        if top == 0 { return Err("top must be positive".into()); }
        let mut seen = BTreeSet::new();
        let mut pool = Vec::new();
        for &hit in ranked {
            if !band.contains(hit.degrees) { continue; }
            let a = &self.areas[hit.index];
            if seen.insert((a.discipline, a.anchor)) {
                pool.push(hit);
                if pool.len() == top { break; }
            }
        }
        if pool.is_empty() {
            return Err(format!("no knowledge areas in angular band {}..{}", band.min, band.max));
        }
        Ok(pool)
    }
    pub fn draw(&self, ranked: &[Hit], top: usize, band: Band, rng: &mut Rng) -> Result<Hit, String> {
        let pool = self.shortlist(ranked, top, band)?;
        let angles: Vec<_> = pool.iter().map(|h| h.degrees).collect();
        Ok(pool[rng.choose(&angles, band.kappa)])
    }
    pub fn area_distance(&self, a: usize, b: usize) -> f64 {
        angle(&self.vectors[a], &self.vectors[b])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamePair {
    pub a: usize,
    pub b: usize,
    pub degrees: f64,
    /// Empirical percentile over all same-role A/B combinations before filtering.
    /// This is descriptive, not a p-value.
    pub percentile: f64,
}

/// Compare two independently authored lists. Draw distinct same-role pairs
/// without replacement from the requested angular band. The role is a
/// categorical matching key, not a coordinate on the sphere.
pub fn pair_lists(
    a: &[Candidate], av: &[Vector], b: &[Candidate], bv: &[Vector],
    count: usize, band: Band, rng: &mut Rng,
) -> Result<Vec<NamePair>, String> {
    if a.len() != av.len() || b.len() != bv.len() {
        return Err("candidate/vector count mismatch".into());
    }
    if count == 0 { return Err("pair count must be positive".into()); }
    let b_roles: Vec<_> = b.iter().map(|c| c.role.to_lowercase()).collect();
    let mut population = Vec::new();
    for (i, x) in a.iter().enumerate() {
        let role = x.role.to_lowercase();
        for (j, y) in b_roles.iter().enumerate() {
            if role == *y {
                population.push((i, j, angle(&av[i], &bv[j])));
            }
        }
    }
    if population.is_empty() { return Err("lists share no role keys".into()); }
    let mut distribution: Vec<_> = population.iter().map(|p| p.2).collect();
    distribution.sort_by(f64::total_cmp);
    let mut pool: Vec<_> = population.into_iter().filter(|p| band.contains(p.2)).collect();
    if pool.len() < count {
        return Err(format!("requested {} pairs but only {} qualify in the band", count, pool.len()));
    }
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let angles: Vec<_> = pool.iter().map(|p| p.2).collect();
        let choice = rng.choose(&angles, band.kappa);
        let (i, j, degrees) = pool.swap_remove(choice);
        let percentile = 100.0 * distribution.partition_point(|d| *d <= degrees) as f64
            / distribution.len() as f64;
        out.push(NamePair { a: i, b: j, degrees, percentile });
    }
    Ok(out)
}

/// Stable identifier over both candidates and seed; the same pair from an
/// altered list cannot silently reuse a previous feedback identifier.
pub fn pair_id(a: &Candidate, b: &Candidate, seed: u64) -> String {
    let mut h = 0xcbf29ce484222325_u64 ^ seed;
    for part in [&a.role, &a.name, &a.meaning, &b.role, &b.name, &b.meaning] {
        for byte in part.as_bytes().iter().copied().chain(std::iter::once(0)) {
            h = (h ^ u64::from(byte)).wrapping_mul(0x100000001b3);
        }
    }
    format!("P{:016x}", h)
}

/// Uniform random catalogue pairs, conditional on an angular band. Rejection
/// sampling avoids enumerating nearly 50 million unordered combinations.
/// Narrow bands can exhaust the explicit attempt budget; report that fact.
pub fn random_areas(
    index: &Index, count: usize, band: Band, rng: &mut Rng,
) -> Result<Vec<(usize, usize, f64)>, String> {
    if count == 0 { return Err("pair count must be positive".into()); }
    if band.kappa != 0.0 { return Err("random-area sampling uses kappa=0".into()); }
    let n = index.areas.len();
    if count > n.saturating_mul(n.saturating_sub(1)) / 2 {
        return Err("requested more than all unique unordered area pairs".into());
    }
    let mut seen = BTreeSet::new();
    let mut out = Vec::with_capacity(count);
    let attempts = count.saturating_mul(1000).saturating_add(1000);
    for _ in 0..attempts {
        let a = rng.index(n);
        let b = rng.index(n);
        if a == b { continue; }
        let (a, b) = if a < b { (a, b) } else { (b, a) };
        if seen.contains(&(a, b)) { continue; }
        let degrees = index.area_distance(a, b);
        if !band.contains(degrees) { continue; }
        seen.insert((a, b));
        out.push((a, b, degrees));
        if out.len() == count { return Ok(out); }
    }
    Err(format!("angular rejection sampling exhausted {} attempts; found {} of {} pairs; widen band or lower count",
        attempts, out.len(), count))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_lists_and_reject_duplicates() {
        let a = parse_candidates("role\tname\tmeaning\r\nMCP\tNexus\tconnection\r\n").unwrap();
        assert_eq!(a[0].name, "Nexus");
        assert!(parse_candidates("role\tname\tmeaning\nMCP\tNexus\tconnection\nmcp\tNEXUS\tportal\n").is_err());
        assert!(parse_candidates("role\tname\tmeaning\nMCP\tNexus\t\n").is_err());
    }
    #[test]
    fn role_pairs_are_distinct_and_reproducible() {
        let a = parse_candidates("role\tname\tmeaning\nMCP\tNexus\tconnection\nSDK\tHelix\tbuilding\n").unwrap();
        let b = parse_candidates("role\tname\tmeaning\nMCP\tOstium\tinterface\nMCP\tSynapse\tcommunication\nSDK\tCodon\tprimitive\n").unwrap();
        let av = vec![vec![1., 0.], vec![0., 1.]];
        let bv = vec![vec![1., 0.], vec![0.5, 0.5], vec![0., 1.]];
        let result = pair_lists(&a, &av, &b, &bv, 3, Band::all(), &mut Rng::new(42)).unwrap();
        let again = pair_lists(&a, &av, &b, &bv, 3, Band::all(), &mut Rng::new(42)).unwrap();
        assert_eq!(result, again);
        assert_eq!(result.iter().map(|p| (p.a, p.b)).collect::<BTreeSet<_>>().len(), 3);
        assert!(result.iter().all(|p| a[p.a].role == b[p.b].role));
        assert!(pair_lists(&a, &av, &b, &bv, 4, Band::all(), &mut Rng::new(42)).is_err());
    }
    #[test]
    fn distinct_concepts_in_shortlist_and_reproducible_area_pairs() {
        let e = Encoder::new().unwrap();
        let ix = Index::build(&e).unwrap();
        let ranked = ix.rank(&e.encode(&[("nexus connection", 1.)]).unwrap());
        assert_eq!(ranked.len(), 10_000);
        let shortlist = ix.shortlist(&ranked, 20, Band::all()).unwrap();
        assert_eq!(shortlist.len(), 20);
        assert_eq!(shortlist.iter().map(|p| {
            let a = &ix.areas[p.index]; (a.discipline, a.anchor)
        }).collect::<BTreeSet<_>>().len(), 20);
        let x = random_areas(&ix, 10, Band::all(), &mut Rng::new(4)).unwrap();
        let y = random_areas(&ix, 10, Band::all(), &mut Rng::new(4)).unwrap();
        assert_eq!(x, y);
    }
}
