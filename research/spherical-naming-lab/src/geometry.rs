//! Inspectable directional feature model and geodesic sampling.
//! This is a hand-authored lexical/semantic proxy, not an LLM embedding model.
use std::collections::{BTreeMap, BTreeSet};

pub type Vector = Vec<f64>;
const HASH_DIM: usize = 256;
const LEXICON: &str = include_str!("../data/semantic-lexicon.tsv");

fn normalize_token(word: &str) -> String {
    let mut s = word.to_lowercase();
    if s.len() > 4 && s.ends_with('s') && !s.ends_with("ss") {
        s.pop();
    }
    s
}
fn tokens(text: &str) -> BTreeSet<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(normalize_token)
        .collect()
}
fn hash(s: &str) -> u64 {
    s.bytes().fold(0xcbf29ce484222325_u64, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x100000001b3)
    })
}

pub struct Encoder {
    aliases: BTreeMap<String, Vec<usize>>,
    pub tags: Vec<&'static str>,
}
impl Encoder {
    pub fn new() -> Result<Self, String> {
        let mut aliases: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        let mut tags = Vec::new();
        for (i, line) in LEXICON.lines().enumerate() {
            let (tag, words) = line.split_once('\t')
                .ok_or_else(|| format!("lexicon line {} missing tab", i + 1))?;
            if tag.is_empty() || tags.contains(&tag) || words.is_empty() {
                return Err(format!("invalid semantic channel on line {}", i + 1));
            }
            let index = tags.len();
            tags.push(tag);
            for w in tokens(words) {
                aliases.entry(w).or_default().push(index);
            }
        }
        Ok(Self { aliases, tags })
    }

    pub fn encode(&self, parts: &[(&str, f64)]) -> Result<Vector, String> {
        let mut result = vec![0.0; self.tags.len() + HASH_DIM];
        for &(text, weight) in parts {
            if !weight.is_finite() || weight < 0.0 {
                return Err("feature weights must be finite and nonnegative".into());
            }
            for word in tokens(text) {
                // Equal weight per distinct token within each input part.
                if let Some(channels) = self.aliases.get(&word) {
                    let share = 2.0 * weight / (channels.len() as f64).sqrt();
                    for &i in channels {
                        result[i] += share;
                    }
                }
                let h = hash(&word);
                let i = self.tags.len() + (h as usize % HASH_DIM);
                let sign = if h & (1_u64 << 63) == 0 { 1.0 } else { -1.0 };
                result[i] += sign * 0.5 * weight;
            }
        }
        let norm = result.iter().map(|x| x * x).sum::<f64>().sqrt();
        if !norm.is_finite() || norm == 0.0 {
            return Err("input has no usable directional features".into());
        }
        for x in &mut result {
            *x /= norm;
        }
        Ok(result)
    }
}

pub fn angle(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len(), "vector dimension mismatch");
    let dot = a.iter().zip(b).map(|(x, y)| x * y).sum::<f64>();
    let na = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let nb = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    assert!(na.is_finite() && nb.is_finite() && na > 0.0 && nb > 0.0);
    (dot / (na * nb)).clamp(-1.0, 1.0).acos().to_degrees()
}

#[derive(Clone, Copy, Debug)]
pub struct Band {
    pub min: f64,
    pub max: f64,
    pub kappa: f64,
}
impl Band {
    pub fn new(min: f64, max: f64, kappa: f64) -> Result<Self, String> {
        if !min.is_finite() || !max.is_finite() || !kappa.is_finite()
            || min < 0.0 || max > 180.0 || min > max || !(0.0..=1000.0).contains(&kappa) {
            return Err("expected 0 <= min <= max <= 180 degrees and 0 <= kappa <= 1000".into());
        }
        Ok(Self { min, max, kappa })
    }
    pub fn all() -> Self {
        Self { min: 0.0, max: 180.0, kappa: 0.0 }
    }
    pub fn contains(&self, degrees: f64) -> bool {
        degrees >= self.min && degrees <= self.max
    }
}

/// SplitMix64: fixed, reproducible, non-cryptographic sampling.
#[derive(Debug)]
pub struct Rng(u64);
impl Rng {
    pub fn new(seed: u64) -> Self { Self(seed) }
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    pub fn index(&mut self, len: usize) -> usize {
        assert!(len > 0);
        let n = len as u64;
        let threshold = n.wrapping_neg() % n;
        loop {
            let x = self.next();
            if x >= threshold { return (x % n) as usize; }
        }
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / ((1_u64 << 53) as f64)
    }
    /// Finite candidate sampling weighted by exp(kappa*cos(angle)).
    /// This is a discrete directional kernel, not a continuous vMF draw.
    pub fn choose(&mut self, angles: &[f64], kappa: f64) -> usize {
        assert!(!angles.is_empty());
        if kappa == 0.0 { return self.index(angles.len()); }
        let cosines: Vec<_> = angles.iter().map(|a| a.to_radians().cos()).collect();
        let best = cosines.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let weights: Vec<_> = cosines.iter().map(|c| (kappa * (c - best)).exp()).collect();
        let mut remaining = self.unit() * weights.iter().sum::<f64>();
        let mut fallback = 0;
        for (i, w) in weights.into_iter().enumerate() {
            if w > 0.0 {
                fallback = i;
                if remaining < w { return i; }
                remaining -= w;
            }
        }
        fallback
    }
}

/// Mean resultant length and its complement (directional variance).
pub fn resultant(vectors: &[Vector]) -> Result<(f64, f64), String> {
    let first = vectors.first().ok_or("no directions")?;
    if first.is_empty() { return Err("empty direction".into()); }
    let mut sum = vec![0.0; first.len()];
    for v in vectors {
        if v.len() != sum.len() { return Err("dimension mismatch".into()); }
        let norm = v.iter().map(|x| x * x).sum::<f64>().sqrt();
        if !norm.is_finite() || norm == 0.0 { return Err("invalid direction".into()); }
        for (s, x) in sum.iter_mut().zip(v) { *s += x / norm; }
    }
    let r = (sum.iter().map(|x| x * x).sum::<f64>().sqrt() / vectors.len() as f64).clamp(0.0, 1.0);
    Ok((r, 1.0 - r))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn spherical_distances() {
        assert!(angle(&[1.0, 0.0], &[1.0, 0.0]) < 1e-8);
        assert!((angle(&[1.0, 0.0], &[0.0, 1.0]) - 90.0).abs() < 1e-8);
        assert!((angle(&[1.0, 0.0], &[-1.0, 0.0]) - 180.0).abs() < 1e-8);
        assert_eq!(angle(&[1.0, 2.0], &[3.0, 1.0]), angle(&[3.0, 1.0], &[1.0, 2.0]));
    }
    #[test]
    fn semantic_bridge_is_not_just_spelling() {
        let e = Encoder::new().unwrap();
        let nexus = e.encode(&[("nexus", 1.0)]).unwrap();
        let anastomosis = e.encode(&[("anastomosis", 1.0)]).unwrap();
        let frost = e.encode(&[("ablation", 1.0)]).unwrap();
        assert!(angle(&nexus, &anastomosis) < angle(&nexus, &frost));
    }
    #[test]
    fn reproducible_sampling_and_band_validation() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..50 {
            assert_eq!(a.choose(&[20.0, 45.0, 75.0], 4.0), b.choose(&[20.0, 45.0, 75.0], 4.0));
        }
        assert!(Band::new(90.0, 10.0, 0.0).is_err());
        assert!(Band::new(0.0, 180.0, f64::NAN).is_err());
        assert!(Band::new(20.0, 40.0, 0.0).unwrap().contains(30.0));
    }
    #[test]
    fn directional_variance() {
        let (r, v) = resultant(&[vec![1.0, 0.0], vec![-1.0, 0.0]]).unwrap();
        assert_eq!((r, v), (0.0, 1.0));
    }
}
