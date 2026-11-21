//! Standalone pre-integration contract reference. No ARBOR runtime effects.
//! Intended for adaptation into existing Rust graph/wiki projection, not deployment as a peer engine.
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verification { Verified, Candidate, Disputed, Unknown }

#[derive(Debug, Clone)]
pub struct Claim<'a> {
    pub id: &'a str,
    pub subject: &'a str,
    pub predicate: &'a str,
    pub object: &'a str,
    pub evidence: &'a [&'a str],
    pub revision: u64,
    pub verification: Verification,
    pub visible: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: &'static str,
    pub claim_id: String,
}
#[derive(Debug, Clone)]
pub struct Page {
    pub id: String,
    pub revision: u64,
    pub facts: Vec<String>,
    pub claim_ids: Vec<String>,
    pub links: Vec<String>,
    pub diagnostics: Vec<Diagnostic>,
}
pub const HUBS: [&str; 7] = [
    "wiki.home", "wiki.system", "wiki.capabilities", "wiki.knowledge",
    "wiki.commands", "wiki.activity", "wiki.development"
];
pub fn project<'a>(page_id: &str, revision: u64, claims: &[Claim<'a>], links: &[&str]) -> Page {
    let mut out = Page { id: page_id.to_owned(), revision, facts: vec![],
        claim_ids: vec![], links: vec![], diagnostics: vec![] };
    let known: BTreeSet<&str> = HUBS.into_iter().collect();
    for target in links {
        if known.contains(target) { out.links.push((*target).to_owned()); }
        else { out.diagnostics.push(Diagnostic { code: "DANGLING_LINK", claim_id: (*target).into() }); }
    }
    let mut seen = BTreeSet::new();
    for claim in claims.iter().filter(|c| c.subject == page_id && c.revision <= revision) {
        if !seen.insert(claim.id) {
            out.diagnostics.push(Diagnostic { code: "DUPLICATE_CLAIM", claim_id: claim.id.into() });
            continue;
        }
        if !claim.visible { continue; }
        if claim.verification != Verification::Verified {
            out.diagnostics.push(Diagnostic { code: "UNVERIFIED_CLAIM", claim_id: claim.id.into() });
            continue;
        }
        if claim.evidence.is_empty() || claim.evidence.iter().any(|x| x.is_empty()) {
            out.diagnostics.push(Diagnostic { code: "MISSING_EVIDENCE", claim_id: claim.id.into() });
            continue;
        }
        if claim.predicate.is_empty() || claim.object.is_empty() {
            out.diagnostics.push(Diagnostic { code: "INCOMPLETE_SEMANTICS", claim_id: claim.id.into() });
            continue;
        }
        // Deliberately NOT presented as human prose: a realizer must produce and fidelity-check it.
        out.facts.push(format!("{}: {}", claim.predicate, claim.object));
        out.claim_ids.push(claim.id.into());
    }
    out
}
pub fn validate_hubs(pages: &[Page]) -> Vec<&'static str> {
    let by_id: BTreeMap<&str, &Page> = pages.iter().map(|p| (p.id.as_str(), p)).collect();
    let mut errors = Vec::new();
    for id in HUBS {
        let Some(p) = by_id.get(id) else { errors.push("MISSING_HUB"); continue };
        if p.links.iter().any(|l| !by_id.contains_key(l.as_str())) { errors.push("MISSING_TARGET"); }
    }
    errors
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn only_grounded_visible_verified_claims() {
        let claims = [
            Claim { id:"a", subject:"wiki.system", predicate:"uses", object:"one graph",
                evidence:&["source:1"], revision:2, verification:Verification::Verified, visible:true },
            Claim { id:"b", subject:"wiki.system", predicate:"executes", object:"all commands",
                evidence:&["source:2"], revision:2, verification:Verification::Candidate, visible:true },
            Claim { id:"c", subject:"wiki.system", predicate:"secret", object:"hidden",
                evidence:&["source:3"], revision:2, verification:Verification::Verified, visible:false },
        ];
        let p = project("wiki.system", 2, &claims, &["wiki.home", "wiki.nonexistent"]);
        assert_eq!(p.claim_ids, ["a"]);
        assert_eq!(p.links, ["wiki.home"]);
        assert_eq!(p.diagnostics.iter().map(|d|d.code).collect::<Vec<_>>(),
            ["DANGLING_LINK", "UNVERIFIED_CLAIM"]);
    }
    #[test] fn revisions_and_missing_evidence() {
        let c = Claim { id:"a", subject:"wiki.home", predicate:"is", object:"ready",
            evidence:&[], revision:4, verification:Verification::Verified, visible:true };
        assert!(project("wiki.home", 3, &[c.clone()], &[]).facts.is_empty());
        assert_eq!(project("wiki.home", 4, &[c], &[]).diagnostics[0].code, "MISSING_EVIDENCE");
    }
    #[test] fn requires_all_hubs() { assert_eq!(validate_hubs(&[]).len(), 7); }
}
