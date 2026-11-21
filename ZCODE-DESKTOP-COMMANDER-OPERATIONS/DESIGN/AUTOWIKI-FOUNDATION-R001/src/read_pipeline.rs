//! R005 read-only vertical composition over incumbent R003/R004 modules.
//! This is not another compiler, graph store, verifier or Host effect path.
//! Inputs are pre-admitted by Host. Fresh epoch/Wiki digest must come from
//! source authority, NEVER from the response being accepted or a PWA field.
use std::collections::BTreeSet;
use super::{
    bundle_projection::LensProjection,
    context_lens::Lens,
    narrative::realize,
    page_alias_projection::ExactHubMap,
    page_assembly::{assemble_human_page, AssembleError, HumanWikiPage},
    readback_guard::{accept_readback, ReadTicket, ReadbackError},
    semantic_evidence::{admit_semantic_sentences, CurrentEvidenceScope,
        EvidenceAdmissionError, IndependentProofReader, ProposedSentence},
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineError {
    Readback(ReadbackError), Evidence(EvidenceAdmissionError),
    Page(AssembleError), MismatchedSource,
}
/// Reuse the SAME read-only flow for owner human and Host-approved agent typing.
/// The caller updates Lens through the existing typed input/source observer.
/// The caller must independently attest current graph epoch and Wiki digest.
/// No effect, provider route, persistence, task creation or prompt logging.
pub fn project_human_wiki_page(
    lens: &Lens,
    ticket: &ReadTicket,
    context: &LensProjection,
    current_graph_epoch: u64,
    current_wiki_digest: &str,
    current_scope: &CurrentEvidenceScope<'_>,
    proposed_claims: &[ProposedSentence<'_>],
    proof_reader: Option<&dyn IndependentProofReader>,
    hub_map: &ExactHubMap,
    allowed_hubs: &BTreeSet<String>,
) -> Result<HumanWikiPage, PipelineError> {
    accept_readback(
        lens,ticket,context,current_graph_epoch,current_wiki_digest,
    ).map_err(PipelineError::Readback)?;
    if current_scope.page_id != ticket.page_id ||
       current_scope.wiki_revision != context.wiki_revision {
        return Err(PipelineError::MismatchedSource);
    }
    let admitted = admit_semantic_sentences(
        proposed_claims,current_scope,proof_reader,
    ).map_err(PipelineError::Evidence)?;
    let narrative = realize(
        &context.focus_page,context.wiki_revision,
        lens.depth,&admitted,current_scope.allowed_source_refs,
    );
    assemble_human_page(
        context,&narrative,hub_map,allowed_hubs,
        current_scope.allowed_source_refs,lens.depth,lens.trace_allowed,
    ).map_err(PipelineError::Page)
}
#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{
        bundle_projection::ContextCard,
        context_lens::{LensEvent, Depth},
        readback_guard::issue_read,
    };
    const PIN:&str="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    fn hubs()->BTreeSet<String> {
        ["wiki.home","wiki.system"].into_iter().map(str::to_owned).collect()
    }
    fn source_refs()->BTreeSet<String> {
        ["source:one"].into_iter().map(str::to_owned).collect()
    }
    fn ids()->ExactHubMap {
        ExactHubMap {
            page_for_entity:[("entity:a".into(),"wiki.home".into())].into_iter().collect(),
            canonical_for_hub:[("wiki.home".into(),"wiki:page:home".into())].into_iter().collect(),
            committed_name_for_hub:[("wiki.home".into(),"home".into())].into_iter().collect(),
        }
    }
    fn lens()->Lens {
        Lens::new("host:epoch","owner:scope",7,"wiki.home",false,&hubs()).unwrap()
    }
    fn context()->LensProjection {
        LensProjection {
            focus_page:"wiki.home".into(),scope_id:"owner:scope".into(),
            graph_epoch:9,wiki_revision:7,truncated:false,related:vec![],
            cards:vec![ContextCard {
                entity_id:"entity:a".into(),page_id:"wiki.home".into(),
                title:"Current".into(),version:1,distance:0,
                source_refs:vec!["source:one".into()],
            }],
        }
    }
    fn digest()->String { "b".repeat(64) }
    #[test] fn without_live_verifier_current_page_is_true_unavailable() {
        let l=lens();let ticket=issue_read(&l,9,&digest()).unwrap();
        let refs=source_refs();let scope=CurrentEvidenceScope {
            page_id:"wiki.home",wiki_revision:7,
            semantic_source_revision:9,source_commit:PIN,
            allowed_source_refs:&refs,
        };
        let x=project_human_wiki_page(
            &l,&ticket,&context(),9,&digest(),&scope,&[],None,&ids(),&hubs(),
        ).unwrap();
        assert_eq!(x.status,super::super::page_assembly::ExplanationState::Unavailable);
        assert_eq!(x.committed_canonical_id,"wiki:page:home");
        assert_eq!(x.navigation.iter().filter(|h|h.committed_canonical_id.is_some()).count(),1);
    }
    #[test] fn user_or_agent_new_draft_rejects_earlier_read() {
        let mut l=lens();let ticket=issue_read(&l,9,&digest()).unwrap();
        let refs=source_refs();let scope=CurrentEvidenceScope {
            page_id:"wiki.home",wiki_revision:7,
            semantic_source_revision:9,source_commit:PIN,
            allowed_source_refs:&refs,
        };
        l.apply(LensEvent::Draft {
            sequence:1,epoch:"host:epoch".into(),source_revision:7,
            candidate_page:Some("wiki.system".into()),grounded:true,
        },&hubs()).unwrap();
        assert_eq!(project_human_wiki_page(
            &l,&ticket,&context(),9,&digest(),&scope,&[],None,&ids(),&hubs(),
        ),Err(PipelineError::Readback(ReadbackError::Superseded)));
    }
    #[test] fn same_revision_changed_wiki_digest_cannot_publish() {
        let l=lens();let ticket=issue_read(&l,9,&digest()).unwrap();
        let refs=source_refs();let scope=CurrentEvidenceScope {
            page_id:"wiki.home",wiki_revision:7,
            semantic_source_revision:9,source_commit:PIN,
            allowed_source_refs:&refs,
        };
        assert_eq!(project_human_wiki_page(
            &l,&ticket,&context(),9,&"c".repeat(64),&scope,&[],None,&ids(),&hubs(),
        ),Err(PipelineError::Readback(ReadbackError::DigestChanged)));
    }

    // Fixtures below exercise the Rust plumbing only; they DO NOT establish
    // that the future independent Semantic Verifier is deployed or trusted.
    struct FakeReader;
    impl IndependentProofReader for FakeReader {
        fn read_checked(&self, id:&str, property:&str)
            -> Option<super::super::semantic_evidence::IndependentlyCheckedProof> {
            use super::super::semantic_evidence::{IndependentlyCheckedProof,ProofVerdict};
            Some(IndependentlyCheckedProof {
                receipt_id:"fixture:proof-1".into(),claim_id:id.into(),
                property_id:property.into(),source_commit:PIN.into(),
                spec_digest:"d".repeat(64),verifier_digest:"e".repeat(64),
                checker_digest:"f".repeat(64),artifact_digest:"1".repeat(64),
                verdict:ProofVerdict::ProvedUnderModel,scope_complete:true,
                unknown_paths:0,counterexample:false,evidence_refs:source_refs(),
                independently_authenticated:true,
                explanation_checked:true,model_scope_checked:true,
            })
        }
    }
    #[test] fn checked_model_prose_reaches_one_existing_wiki_page() {
        use super::super::semantic_evidence::{ProposedSentence,StatementKind};
        use super::super::narrative::Relationship;
        let l=lens();let ticket=issue_read(&l,9,&digest()).unwrap();
        let refs=source_refs();
        let scope=CurrentEvidenceScope {
            page_id:"wiki.home",wiki_revision:7,
            semantic_source_revision:9,source_commit:PIN,
            allowed_source_refs:&refs,
        };
        let candidates=[ProposedSentence {
            claim_id:"claim:one",property_id:"property:one",page_id:"wiki.home",
            subject_title:"Under model M, the core",relation:Relationship::Uses,
            object_phrase:"bounded reads",negated:false,
            evidence_refs:&["source:one"],semantic_source_revision:9,
            source_commit:PIN,
            spec_digest:"dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
            kind:StatementKind::ModelBoundContract,model_qualified:true,
            visible:true,
        }];
        let x=project_human_wiki_page(
            &l,&ticket,&context(),9,&digest(),&scope,
            &candidates,Some(&FakeReader),&ids(),&hubs(),
        ).unwrap();
        assert_eq!(x.intro,"Under model M, the core uses bounded reads.");
        assert_eq!(x.status,super::super::page_assembly::ExplanationState::Verified);
    }
    #[test] fn no_actual_verifier_never_promotes_the_same_fixture_sentence() {
        use super::super::semantic_evidence::{ProposedSentence,StatementKind};
        use super::super::narrative::Relationship;
        let l=lens();let ticket=issue_read(&l,9,&digest()).unwrap();
        let refs=source_refs();
        let scope=CurrentEvidenceScope {
            page_id:"wiki.home",wiki_revision:7,
            semantic_source_revision:9,source_commit:PIN,
            allowed_source_refs:&refs,
        };
        let candidates=[ProposedSentence {
            claim_id:"claim:one",property_id:"property:one",page_id:"wiki.home",
            subject_title:"Under model M, the core",relation:Relationship::Uses,
            object_phrase:"bounded reads",negated:false,
            evidence_refs:&["source:one"],semantic_source_revision:9,
            source_commit:PIN,
            spec_digest:"dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
            kind:StatementKind::ModelBoundContract,model_qualified:true,
            visible:true,
        }];
        let x=project_human_wiki_page(
            &l,&ticket,&context(),9,&digest(),&scope,
            &candidates,None,&ids(),&hubs(),
        ).unwrap();
        assert_eq!(x.status,super::super::page_assembly::ExplanationState::Unavailable);
        assert!(!x.intro.contains("bounded reads"));
    }
    #[test] fn source_mismatch_and_trace_without_permission_fail_closed() {
        let mut l=lens();let ticket=issue_read(&l,9,&digest()).unwrap();
        let refs=source_refs();let scope=CurrentEvidenceScope {
            page_id:"wiki.system",wiki_revision:7,
            semantic_source_revision:9,source_commit:PIN,
            allowed_source_refs:&refs,
        };
        assert_eq!(project_human_wiki_page(
            &l,&ticket,&context(),9,&digest(),&scope,&[],None,&ids(),&hubs(),
        ),Err(PipelineError::MismatchedSource));
        assert!(l.apply(LensEvent::SetDepth(Depth::Trace),&hubs()).is_err());
    }
}
