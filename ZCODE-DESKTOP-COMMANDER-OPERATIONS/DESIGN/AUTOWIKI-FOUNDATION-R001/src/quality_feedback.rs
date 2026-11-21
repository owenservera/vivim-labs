//! R003: compile read-only, source-revisioned AutoWiki quality feedback.
//! This is an inert review candidate, not an owner-task writer, code modifier,
//! model invocation, automatic VLANG registration or permission grant.
use std::collections::BTreeSet;
use crate::bundle_projection::LensProjection;
use crate::narrative::{Narrative, RealizationCode};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    SourceEvidence, SemanticTyping, LanguageRealization,
    WikiNavigation, ContextSelection, RuntimeVerification,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity { Informational, NeedsReview }
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum QualityCode {
    MissingProvenance, UnverifiedStatement, TemporalMismatch,
    InvalidLanguage, DuplicateClaim, NoUsableExplanation,
    TruncatedContext, OrphanedFocus,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QualityCandidate {
    pub page_id: String,
    pub wiki_revision: u64,
    pub graph_epoch: u64,
    pub code: QualityCode,
    pub stage: Stage,
    pub severity: Severity,
    pub claim_id: Option<String>,
    pub suggested_review: &'static str,
    /// Never includes raw draft content, rejected source metadata or secrets.
    pub requires_owner_review: bool,
}
fn route(code: RealizationCode) -> (QualityCode, Stage, Severity, &'static str) {
    match code {
        RealizationCode::InvalidWording => (
            QualityCode::InvalidLanguage,Stage::LanguageRealization,
            Severity::NeedsReview,"Review subject/object phrase and safe grammar template"),
        RealizationCode::Unverified => (
            QualityCode::UnverifiedStatement,Stage::SourceEvidence,
            Severity::Informational,"Check original claim status before publishing"),
        RealizationCode::UnsupportedTime => (
            QualityCode::TemporalMismatch,Stage::SourceEvidence,
            Severity::NeedsReview,"Revalidate current source revision and supersession"),
        RealizationCode::MissingEvidence => (
            QualityCode::MissingProvenance,Stage::SourceEvidence,
            Severity::NeedsReview,"Supply admissible evidence or keep claim unpublished"),
        RealizationCode::DuplicateClaim => (
            QualityCode::DuplicateClaim,Stage::SemanticTyping,
            Severity::NeedsReview,"Canonicalize duplicate claim identity"),
    }
}
pub fn review_candidates(
    context: &LensProjection,
    narrative: &Narrative,
) -> Vec<QualityCandidate> {
    // A split source fence must never produce a repair suggestion using a
    // different graph / Wiki snapshot; the caller must re-read instead.
    if narrative.page_id != context.focus_page ||
        narrative.source_revision != context.wiki_revision {
        return Vec::new()
    }
    let mut output = Vec::new();
    let mut seen = BTreeSet::new();
    let mut push = |code:QualityCode,stage:Stage,severity:Severity,
                    claim_id:Option<String>,suggested_review:&'static str| {
        if output.len() >= 48 { return }
        if !seen.insert((code,claim_id.clone())) { return }
        output.push(QualityCandidate {
            page_id:context.focus_page.clone(),
            wiki_revision:context.wiki_revision,
            graph_epoch:context.graph_epoch,
            code,stage,severity,claim_id,suggested_review,
            requires_owner_review:true,
        });
    };
    for d in &narrative.diagnostics {
        let (code,stage,severity,review)=route(d.code);
        push(code,stage,severity,Some(d.claim_id.clone()),review);
    }
    if narrative.sentences.is_empty() {
        push(QualityCode::NoUsableExplanation,Stage::LanguageRealization,
            Severity::Informational,None,
            "Check whether any current verified semantic claims support an introduction");
    }
    if context.truncated {
        push(QualityCode::TruncatedContext,Stage::ContextSelection,
            Severity::Informational,None,
            "Inspect bounded context omissions before stating page completeness");
    }
    if context.cards.iter().all(|c|c.page_id!=context.focus_page) {
        push(QualityCode::OrphanedFocus,Stage::WikiNavigation,
            Severity::NeedsReview,None,
            "Repair exact authorized focus-to-page identity crosswalk");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle_projection::ContextCard;
    use crate::narrative::{Narrative,RealizationDiagnostic};
    fn sample() -> (LensProjection,Narrative) {
        (LensProjection {
            focus_page:"wiki.system".into(),scope_id:"owner:1".into(),
            graph_epoch:3,wiki_revision:5,
            cards:vec![ContextCard{
                entity_id:"graph:core".into(),page_id:"wiki.system".into(),
                title:"Core".into(),version:1,distance:0,
                source_refs:vec!["source:1".into()]
            }],related:vec![],truncated:false
        },Narrative {
            page_id:"wiki.system".into(),source_revision:5,
            sentences:vec![],paragraph:"No verified explanation.".into(),
            diagnostics:vec![RealizationDiagnostic {
                claim_id:"claim:1".into(),code:RealizationCode::MissingEvidence
            }],complete:false
        })
    }
    #[test] fn source_problem_routes_to_evidence_review() {
        let (p,n)=sample();let c=review_candidates(&p,&n);
        assert_eq!(c[0].stage,Stage::SourceEvidence);
        assert_eq!(c[0].code,QualityCode::MissingProvenance);
        assert!(c.iter().all(|x|x.requires_owner_review));
    }
    #[test] fn mismatch_yields_no_cross_revision_feedback() {
        let (mut p,n)=sample();p.wiki_revision+=1;
        assert!(review_candidates(&p,&n).is_empty());
    }
    #[test] fn does_not_emit_draft_text_or_source_contents() {
        let (p,n)=sample();let c=review_candidates(&p,&n);
        let msg=format!("{c:?}");
        assert!(!msg.contains("No verified explanation"));
        assert!(!msg.contains("source:1"));
    }
    #[test] fn truncated_context_is_not_marked_complete() {
        let (mut p,n)=sample();p.truncated=true;
        let c=review_candidates(&p,&n);
        assert!(c.iter().any(|e|e.code==QualityCode::TruncatedContext));
    }
}
