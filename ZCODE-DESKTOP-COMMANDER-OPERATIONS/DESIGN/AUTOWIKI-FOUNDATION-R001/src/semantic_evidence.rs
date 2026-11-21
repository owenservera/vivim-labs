//! R005: source-pinned, fail-closed admission for already-checked semantic proofs.
//! This is a narrow adapter to R003 narrative, NOT the planned SV proof engine.
//! The independent SV in ARBOR remains DESIGN/NOT_IMPLEMENTED; without a
//! trusted implementation, capability prose stays UNKNOWN or CANDIDATE.
//! No graph reads, verifier invocation, network requests, writes or authority.
use std::collections::BTreeSet;
use super::{Verification, narrative::{Relationship, SentenceClaim}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatementKind {
    /// Claim about behavior proven under an expressly limited formal model.
    ModelBoundContract,
    /// Claim about actual runtime/device/Host effects, which requires a real
    /// independent Host execution receipt in a future separate adapter.
    RuntimeEffect,
    /// Indexed source annotation or graph occurrence; not behavioral proof.
    SourceOccurrence,
    /// Proposed or user-authored statement; never truth by assertion alone.
    DeclaredFact,
}
#[derive(Debug, Clone)]
pub struct ProposedSentence<'a> {
    pub claim_id: &'a str,
    pub property_id: &'a str,
    pub page_id: &'a str,
    pub subject_title: &'a str,
    pub relation: Relationship,
    pub object_phrase: &'a str,
    pub negated: bool,
    pub evidence_refs: &'a [&'a str],
    pub source_revision: u64,
    pub source_commit: &'a str,
    pub spec_digest: &'a str,
    pub kind: StatementKind,
    /// The actual sentence must contain a meaningful model limitation;
    /// a true value cannot be accepted from a browser or generated claim.
    pub model_qualified: bool,
    pub visible: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentEvidenceScope<'a> {
    pub page_id: &'a str,
    pub wiki_revision: u64,
    pub source_commit: &'a str,
    pub allowed_source_refs: &'a BTreeSet<String>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofVerdict {
    ProvedUnderModel, SupportedByTests, Refuted, Unknown, Blocked,
}
#[derive(Debug, Clone)]
pub struct IndependentlyCheckedProof {
    pub receipt_id: String,
    pub claim_id: String,
    pub property_id: String,
    pub source_commit: String,
    pub spec_digest: String,
    pub verifier_digest: String,
    pub checker_digest: String,
    pub artifact_digest: String,
    pub verdict: ProofVerdict,
    pub scope_complete: bool,
    pub unknown_paths: u32,
    pub counterexample: bool,
    pub evidence_refs: BTreeSet<String>,
    /// Must be true only after trusted independent checker/runner validation.
    /// A source tag, fixture or user-provided boolean is NOT authentication.
    pub independently_authenticated: bool,
}
/// Implement ONLY in a trusted Host/SV source-owner adapter after full schema,
/// artifact/source pin, checker independence and witness verification. Never
/// use a PWA JSON object or a generated source annotation to implement this.
/// Returning None is the correct production default until SV is operational.
pub trait IndependentProofReader {
    fn read_checked(&self, claim_id: &str, property_id: &str)
        -> Option<IndependentlyCheckedProof>;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceAdmissionError {
    InvalidSourcePin, InvalidClaim, SourceRevisionConflict, FutureClaim,
}
fn sha40(s: &str) -> bool {
    s.len()==40 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn sha64(s: &str) -> bool {
    s.len()==64 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn bounded_id(s: &str) -> bool {
    !s.is_empty() && s.len()<=256 && !s.chars().any(char::is_control)
}
fn credible(
    p: &IndependentlyCheckedProof,
    claim: &ProposedSentence<'_>,
    scope: &CurrentEvidenceScope<'_>,
) -> bool {
    p.independently_authenticated && p.scope_complete && p.unknown_paths==0 &&
    !p.counterexample && bounded_id(&p.receipt_id) &&
    p.claim_id==claim.claim_id && p.property_id==claim.property_id &&
    p.source_commit==claim.source_commit && p.source_commit==scope.source_commit &&
    p.spec_digest==claim.spec_digest &&
    sha40(&p.source_commit) && sha64(&p.spec_digest) &&
    sha64(&p.verifier_digest) && sha64(&p.checker_digest) &&
    sha64(&p.artifact_digest) && !p.evidence_refs.is_empty() &&
    claim.evidence_refs.iter().all(|x|p.evidence_refs.contains(*x)) &&
    p.evidence_refs.iter().all(|x|scope.allowed_source_refs.contains(x))
}
fn classification(
    claim: &ProposedSentence<'_>,
    scope: &CurrentEvidenceScope<'_>,
    reader: Option<&dyn IndependentProofReader>,
) -> Verification {
    // Source occurrence/user assertions cannot be promoted to behavior proof.
    if claim.kind!=StatementKind::ModelBoundContract {
        return Verification::Unknown;
    }
    let Some(proof)=reader.and_then(|r|r.read_checked(claim.claim_id,claim.property_id))
        else {return Verification::Unknown};
    // Independently checked REFUTED is a dispute, not a proven positive.
    if !credible(&proof,claim,scope) {return Verification::Unknown}
    match proof.verdict {
        ProofVerdict::Refuted => Verification::Disputed,
        ProofVerdict::ProvedUnderModel if claim.model_qualified =>
            Verification::Verified,
        ProofVerdict::SupportedByTests => Verification::Candidate,
        _ => Verification::Unknown,
    }
}
/// Converts existing semantics into existing R003 SentenceClaim, without a
/// second claim store/realizer. Private/ineligible claims are silently omitted.
/// This function does NOT attest the caller/reader is honest; Host must already
/// enforce source and actor grants and inject the trusted proof reader.
pub fn admit_semantic_sentences<'a>(
    claims: &'a [ProposedSentence<'a>],
    scope: &CurrentEvidenceScope<'_>,
    reader: Option<&dyn IndependentProofReader>,
) -> Result<Vec<SentenceClaim<'a>>, EvidenceAdmissionError> {
    if !sha40(scope.source_commit) {
        return Err(EvidenceAdmissionError::InvalidSourcePin);
    }
    let mut output=Vec::new();
    if claims.len()>256 {return Err(EvidenceAdmissionError::InvalidClaim)}
    for c in claims {
        // Perform permission filtering before retaining any private claim IDs,
        // diagnostics or renderable text; refusal does not leak hidden sources.
        if !c.visible || c.page_id!=scope.page_id ||
            c.evidence_refs.is_empty() ||
            c.evidence_refs.iter().any(|s|!scope.allowed_source_refs.contains(*s)) {
            continue;
        }
        if !sha40(c.source_commit) || !sha64(c.spec_digest) ||
            !bounded_id(c.claim_id) || !bounded_id(c.property_id) ||
            !bounded_id(c.page_id) || c.evidence_refs.len()>24 {
            return Err(EvidenceAdmissionError::InvalidClaim);
        }
        if c.source_commit!=scope.source_commit {
            return Err(EvidenceAdmissionError::SourceRevisionConflict);
        }
        if c.source_revision>scope.wiki_revision {
            return Err(EvidenceAdmissionError::FutureClaim);
        }
        let verification=classification(c,scope,reader);
        output.push(SentenceClaim {
            claim_id:c.claim_id,subject_id:c.page_id,
            subject_title:c.subject_title,relation:c.relation,
            object_phrase:c.object_phrase,negated:c.negated,
            evidence_refs:c.evidence_refs,source_revision:c.source_revision,
            current_at_source:c.source_revision==scope.wiki_revision,
            verification,visible:true,
        });
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    use super::super::context_lens::Depth;
    use super::super::narrative::realize;

    const PIN:&str="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const SPEC:&str="bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    fn sources()->BTreeSet<String> {
        ["source:one".to_owned()].into_iter().collect()
    }
    fn sample()->ProposedSentence<'static> {
        ProposedSentence {
            claim_id:"claim:v1",property_id:"property:no-write",
            page_id:"wiki.system",subject_title:"Under model M, the core",
            relation:Relationship::Uses,object_phrase:"a bounded source policy",
            negated:false,evidence_refs:&["source:one"],
            source_revision:7,source_commit:PIN,spec_digest:SPEC,
            kind:StatementKind::ModelBoundContract,model_qualified:true,visible:true,
        }
    }
    fn scope<'a>(allowed:&'a BTreeSet<String>)->CurrentEvidenceScope<'a> {
        CurrentEvidenceScope {page_id:"wiki.system",wiki_revision:7,
            source_commit:PIN,allowed_source_refs:allowed}
    }
    struct Reader { verdict:ProofVerdict, authentic:bool, complete:bool }
    impl IndependentProofReader for Reader {
        fn read_checked(&self, claim_id:&str, property_id:&str)
            -> Option<IndependentlyCheckedProof> {
            Some(IndependentlyCheckedProof {
                receipt_id:"sv:receipt:001".into(),claim_id:claim_id.into(),
                property_id:property_id.into(),source_commit:PIN.into(),
                spec_digest:SPEC.into(),verifier_digest:"c".repeat(64),
                checker_digest:"d".repeat(64),artifact_digest:"e".repeat(64),
                verdict:self.verdict,scope_complete:self.complete,
                unknown_paths:0,counterexample:false,
                evidence_refs:sources(),independently_authenticated:self.authentic,
            })
        }
    }
    #[test] fn absent_verifier_never_promotes_claim() {
        let refs=sources();let c=[sample()];
        let admitted=admit_semantic_sentences(&c,&scope(&refs),None).unwrap();
        assert_eq!(admitted[0].verification,Verification::Unknown);
        assert!(realize("wiki.system",7,Depth::Overview,&admitted,&refs).sentences.is_empty());
    }
    #[test] fn current_model_bound_independent_proof_may_render_with_qualification() {
        let refs=sources();let c=[sample()];
        let checker=Reader{verdict:ProofVerdict::ProvedUnderModel,authentic:true,complete:true};
        let admitted=admit_semantic_sentences(&c,&scope(&refs),Some(&checker)).unwrap();
        assert_eq!(admitted[0].verification,Verification::Verified);
        let page=realize("wiki.system",7,Depth::Overview,&admitted,&refs);
        assert!(page.paragraph.starts_with("Under model M"));
    }
    #[test] fn tested_is_not_formally_proved() {
        let refs=sources();let c=[sample()];
        let checker=Reader{verdict:ProofVerdict::SupportedByTests,authentic:true,complete:true};
        assert_eq!(admit_semantic_sentences(&c,&scope(&refs),Some(&checker)).unwrap()[0].verification,
            Verification::Candidate);
    }
    #[test] fn untrusted_or_incomplete_witness_stays_unknown() {
        let refs=sources();let c=[sample()];
        for checker in [Reader{verdict:ProofVerdict::ProvedUnderModel,authentic:false,complete:true},
                        Reader{verdict:ProofVerdict::ProvedUnderModel,authentic:true,complete:false}] {
            assert_eq!(admit_semantic_sentences(&c,&scope(&refs),Some(&checker)).unwrap()[0].verification,
                Verification::Unknown);
        }
    }
    #[test] fn source_annotations_and_runtime_effects_cannot_become_verified() {
        let refs=sources();let checker=Reader{verdict:ProofVerdict::ProvedUnderModel,authentic:true,complete:true};
        for kind in [StatementKind::SourceOccurrence,StatementKind::RuntimeEffect,StatementKind::DeclaredFact] {
            let mut c=sample();c.kind=kind;
            assert_eq!(admit_semantic_sentences(&[c],&scope(&refs),Some(&checker)).unwrap()[0].verification,
                Verification::Unknown);
        }
    }
    #[test] fn unqualified_model_statement_must_not_claim_universal_truth() {
        let refs=sources();let mut c=sample();c.model_qualified=false;
        let checker=Reader{verdict:ProofVerdict::ProvedUnderModel,authentic:true,complete:true};
        assert_eq!(admit_semantic_sentences(&[c],&scope(&refs),Some(&checker)).unwrap()[0].verification,
            Verification::Unknown);
    }
    #[test] fn private_evidence_never_reaches_narrative_or_diagnostics() {
        let refs=sources();let mut c=sample();c.claim_id="private:claim";c.evidence_refs=&["secret:ref"];
        assert!(admit_semantic_sentences(&[c],&scope(&refs),None).unwrap().is_empty());
    }
    #[test] fn changed_commit_or_future_revision_refused() {
        let refs=sources();let mut c=sample();c.source_commit="cccccccccccccccccccccccccccccccccccccccc";
        assert!(matches!(admit_semantic_sentences(&[c],&scope(&refs),None),
            Err(EvidenceAdmissionError::SourceRevisionConflict)));
        let mut c=sample();c.source_revision=8;
        assert!(matches!(admit_semantic_sentences(&[c],&scope(&refs),None),
            Err(EvidenceAdmissionError::FutureClaim)));
    }
    #[test] fn independent_refutation_is_never_published_as_verified() {
        let refs=sources();let c=[sample()];
        let checker=Reader{verdict:ProofVerdict::Refuted,authentic:true,complete:true};
        assert_eq!(admit_semantic_sentences(&c,&scope(&refs),Some(&checker)).unwrap()[0].verification,
            Verification::Disputed);
    }
}
