//! R003: conservative plain-English realization of VERIFIED semantic claims.
//! The module does not ask a model to invent connections, does not classify
//! evidence itself and does not assert current device/runtime health.
//! Sentence strings MUST be rendered via textContent, not innerHTML.
use std::collections::BTreeSet;
use crate::{Verification, context_lens::Depth};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Relationship {
    Is, Uses, DependsOn, ConnectsTo, Describes, PartOf, Contains, Produces,
}
#[derive(Clone, Debug)]
pub struct SentenceClaim<'a> {
    pub claim_id: &'a str,
    pub subject_id: &'a str,
    pub subject_title: &'a str,
    pub relation: Relationship,
    /// Already-authorized grammatical noun phrase (including article, if needed).
    pub object_phrase: &'a str,
    pub negated: bool,
    pub evidence_refs: &'a [&'a str],
    pub source_revision: u64,
    /// Attested by the source owner; history does not imply current truth.
    pub current_at_source: bool,
    pub verification: Verification,
    pub visible: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sentence {
    pub text: String,
    pub claim_id: String,
    pub evidence_refs: Vec<String>,
}
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum RealizationCode {
    InvalidWording, Unverified, UnsupportedTime, MissingEvidence, DuplicateClaim,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealizationDiagnostic {
    pub claim_id: String,
    pub code: RealizationCode,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Narrative {
    pub page_id: String,
    pub source_revision: u64,
    pub sentences: Vec<Sentence>,
    pub paragraph: String,
    pub diagnostics: Vec<RealizationDiagnostic>,
    pub complete: bool,
}

fn grammatical(s: &str, max_chars: usize) -> bool {
    let chars: Vec<char> = s.chars().collect();
    !chars.is_empty() && chars.len() <= max_chars && s.trim() == s
        && !s.chars().any(char::is_control)
        && !matches!(s.chars().last(), Some('.' | '!' | '?'))
}
fn sentence(c: &SentenceClaim<'_>) -> String {
    let verb = match (c.relation, c.negated) {
        (Relationship::Is, false) => "is",
        (Relationship::Is, true) => "is not",
        (Relationship::Uses, false) => "uses",
        (Relationship::Uses, true) => "does not use",
        (Relationship::DependsOn, false) => "depends on",
        (Relationship::DependsOn, true) => "does not depend on",
        (Relationship::ConnectsTo, false) => "connects to",
        (Relationship::ConnectsTo, true) => "does not connect to",
        (Relationship::Describes, false) => "describes",
        (Relationship::Describes, true) => "does not describe",
        (Relationship::PartOf, false) => "is part of",
        (Relationship::PartOf, true) => "is not part of",
        (Relationship::Contains, false) => "contains",
        (Relationship::Contains, true) => "does not contain",
        (Relationship::Produces, false) => "produces",
        (Relationship::Produces, true) => "does not produce",
    };
    format!("{} {} {}.", c.subject_title, verb, c.object_phrase)
}
fn capacity(depth: Depth) -> usize {
    match depth { Depth::Overview => 2, Depth::Explain => 6, Depth::Trace => 16 }
}

/// Produce text only from the caller's exact, current, source-verified claims.
/// This is NOT a truth verifier: upstream must prove evidence and source owner
/// must attest current_at_source. This code only prevents unsupported promotion.
/// Unsupported private claims are omitted WITHOUT a diagnostic identifying them.
pub fn realize(
    page_id: &str,
    source_revision: u64,
    depth: Depth,
    claims: &[SentenceClaim<'_>],
    allowed_source_refs: &BTreeSet<String>,
) -> Narrative {
    let mut sentences = Vec::new();
    let mut diagnostics = Vec::new();
    let mut seen = BTreeSet::new();
    let mut omitted_for_capacity = false;
    for c in claims.iter().filter(|c| c.subject_id == page_id && c.visible) {
        // Filter source authorization BEFORE recording any claim ID or diagnostic.
        if c.evidence_refs.iter().any(|r| !allowed_source_refs.contains(*r)) { continue }
        if !seen.insert(c.claim_id) {
            diagnostics.push(RealizationDiagnostic{
                claim_id:c.claim_id.into(), code:RealizationCode::DuplicateClaim
            });
            continue;
        }
        let reject = if c.verification != Verification::Verified {
            Some(RealizationCode::Unverified)
        } else if !c.current_at_source || c.source_revision > source_revision {
            Some(RealizationCode::UnsupportedTime)
        } else if c.evidence_refs.is_empty() || c.evidence_refs.iter().any(|s| s.is_empty()) {
            Some(RealizationCode::MissingEvidence)
        } else if !grammatical(c.subject_title, 120) || !grammatical(c.object_phrase, 180) ||
            c.claim_id.is_empty() {
            Some(RealizationCode::InvalidWording)
        } else { None };
        if let Some(code) = reject {
            diagnostics.push(RealizationDiagnostic {claim_id:c.claim_id.into(),code});
            continue;
        }
        if sentences.len() >= capacity(depth) {
            omitted_for_capacity = true;
            break;
        }
        sentences.push(Sentence {
            text: sentence(c), claim_id: c.claim_id.to_owned(),
            evidence_refs: c.evidence_refs.iter().map(|r| (*r).to_owned()).collect(),
        });
    }
    let complete = !omitted_for_capacity && diagnostics.is_empty() && !sentences.is_empty();
    let paragraph = if sentences.is_empty() {
        "No current, verified explanation is available for this topic.".to_owned()
    } else {
        sentences.iter().map(|s| s.text.as_str()).collect::<Vec<_>>().join(" ")
    };
    Narrative {
        page_id: page_id.to_owned(), source_revision, sentences,
        paragraph, diagnostics, complete,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn source() -> BTreeSet<String> { ["source:1".to_string()].into_iter().collect() }
    fn claim<'a>() -> SentenceClaim<'a> {
        SentenceClaim{
            claim_id:"claim:1",subject_id:"wiki.system",subject_title:"The Rust core",
            relation:Relationship::Uses,object_phrase:"one shared graph",
            negated:false,evidence_refs:&["source:1"],source_revision:2,
            current_at_source:true,verification:Verification::Verified,visible:true
        }
    }
    #[test] fn accurate_verified_prose_has_claim_trace() {
        let c=claim();
        let text=realize("wiki.system",2,Depth::Overview,&[c],&source());
        assert_eq!(text.paragraph,"The Rust core uses one shared graph.");
        assert_eq!(text.sentences[0].claim_id,"claim:1");
        assert_eq!(text.sentences[0].evidence_refs,vec!["source:1".to_string()]);
        assert!(text.complete);
    }
    #[test] fn negation_preserves_truth_conditions() {
        let mut c=claim();
        c.negated=true;
        assert_eq!(realize("wiki.system",2,Depth::Overview,&[c],&source()).paragraph,
            "The Rust core does not use one shared graph.");
    }
    #[test] fn candidate_claim_cannot_become_factual_sentence() {
        let mut c=claim();
        c.verification=Verification::Candidate;
        let text=realize("wiki.system",2,Depth::Overview,&[c],&source());
        assert_eq!(text.sentences.len(),0);
        assert_eq!(text.diagnostics[0].code,RealizationCode::Unverified);
    }
    #[test] fn historical_claim_cannot_become_current() {
        let mut c=claim();c.current_at_source=false;
        assert_eq!(realize("wiki.system",2,Depth::Overview,&[c],&source())
            .diagnostics[0].code,RealizationCode::UnsupportedTime);
    }
    #[test] fn private_source_and_hidden_claim_leak_no_reference() {
        let mut c=claim();
        c.claim_id="private:claim";c.evidence_refs=&["private:source"];
        let text=realize("wiki.system",2,Depth::Overview,&[c],&source());
        assert_eq!(text.diagnostics.len(),0);
        assert!(!text.paragraph.contains("private"));
        assert_eq!(text.sentences.len(),0);
    }
    #[test] fn invalid_wording_never_printed_as_verified_prose() {
        let mut c=claim();c.object_phrase="hello\nworld";
        let text=realize("wiki.system",2,Depth::Overview,&[c],&source());
        assert_eq!(text.diagnostics[0].code,RealizationCode::InvalidWording);
    }
    #[test] fn exact_subject_and_revision_only() {
        let c=claim();
        assert!(realize("wiki.commands",2,Depth::Overview,&[c.clone()],&source()).sentences.is_empty());
        assert!(realize("wiki.system",1,Depth::Overview,&[c],&source()).sentences.is_empty());
    }
    #[test] fn detail_is_progressive_and_deterministic() {
        let ids=["a","b","c","d","e","f","g"];
        let c:Vec<_>=ids.iter().map(|id| SentenceClaim {
            claim_id:id, subject_id:"wiki.system", subject_title:"The core",
            relation:Relationship::Describes,object_phrase:"a system capability",
            negated:false,evidence_refs:&["source:1"],source_revision:1,
            current_at_source:true,verification:Verification::Verified,visible:true
        }).collect();
        let a=realize("wiki.system",1,Depth::Overview,&c,&source());
        let b=realize("wiki.system",1,Depth::Explain,&c,&source());
        assert_eq!(a.sentences.len(),2);
        assert_eq!(b.sentences.len(),6);
        assert_eq!(a.sentences[0],b.sentences[0]);
        assert!(!a.complete);
    }
}
