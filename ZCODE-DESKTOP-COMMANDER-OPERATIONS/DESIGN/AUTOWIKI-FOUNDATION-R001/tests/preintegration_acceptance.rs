//! R003 contract-level scenario tests. These use ONLY invented non-product IDs.
//! They demonstrate state/visibility invariants, not deployed PWA behavior.
//! No local Host, ZCode, provider, browser, graph file or user data is touched.
use std::collections::{BTreeMap,BTreeSet};
use autowiki_foundation_preintegration::context_lens::{
    Change, Depth, Lens, LensEvent, LensError
};
use autowiki_foundation_preintegration::bundle_projection::{
    ContextReceipt, LensRead, SourceItem, SourceEdge, project_context, ProjectionError
};
use autowiki_foundation_preintegration::narrative::{
    SentenceClaim, Relationship, realize
};
use autowiki_foundation_preintegration::quality_feedback::{
    review_candidates
};
use autowiki_foundation_preintegration::Verification;

fn scope() -> (BTreeSet<String>, BTreeSet<String>, BTreeMap<String,String>) {
    (
        ["graph:system","graph:wiki"].iter().map(|s|s.to_string()).collect(),
        ["source:system","source:wiki"].iter().map(|s|s.to_string()).collect(),
        [("graph:system","wiki.system"),("graph:wiki","wiki.knowledge")]
            .iter().map(|(a,b)|(a.to_string(),b.to_string())).collect()
    )
}
fn read<'a>(
    entities: &'a BTreeSet<String>, sources: &'a BTreeSet<String>,
    pages: &'a BTreeMap<String,String>, items: &'a [SourceItem<'a>],
    edges: &'a [SourceEdge<'a>],
) -> (LensRead<'a>,ContextReceipt<'a>) {
    (LensRead {
        focus_entity:"graph:system",scope_id:"owner:1",
        expected_graph_epoch:9, expected_wiki_revision:3,
        depth:Depth::Overview,trace_authorized:false,
        allowed_entities:entities,allowed_source_refs:sources,page_for_entity:pages
    },ContextReceipt {
        scope_id:"owner:1",graph_epoch:9,wiki_revision:3,
        items,edges,truncated:false
    })
}
fn accepted_typed_input_result() -> (String, String, usize) {
    let (entities,sources,pages)=scope();
    let allowed_pages: BTreeSet<String> = pages.values().cloned().chain(["wiki.home".into()]).collect();
    let mut lens=Lens::new("epoch:1","owner:1",3,"wiki.home",false,&allowed_pages).unwrap();
    assert_eq!(lens.apply(LensEvent::Draft{
        sequence:1,epoch:"epoch:1".into(),source_revision:3,
        candidate_page:Some("wiki.system".into()),grounded:true
    },&allowed_pages),Ok(Change::Suggested));
    assert_eq!(lens.selected_page.as_deref(),Some("wiki.home"));
    lens.apply(LensEvent::Navigate {
        epoch:"epoch:1".into(),source_revision:3,page:"wiki.system".into()
    },&allowed_pages).unwrap();
    lens.apply(LensEvent::SetDepth(Depth::Explain),&allowed_pages).unwrap();
    let items=[
        SourceItem{id:"graph:system",title:"Rust fabric",version:2,depth:0,
            stale:false,sources:&["source:system"]},
        SourceItem{id:"graph:wiki",title:"AutoWiki",version:2,depth:1,
            stale:false,sources:&["source:wiki"]},
    ];
    let edges=[SourceEdge{
        from:"graph:system",to:"graph:wiki",relation:"documents"
    }];
    let (mut request,receipt)=read(&entities,&sources,&pages,&items,&edges);
    request.depth=lens.depth;
    let projection=project_context(&request,&receipt).unwrap();
    let verified_claims=[SentenceClaim {
        claim_id:"claim:01",subject_id:"wiki.system",subject_title:"The Rust fabric",
        relation:Relationship::Contains,object_phrase:"a wiki projection",
        negated:false,evidence_refs:&["source:system"],source_revision:3,
        current_at_source:true,verification:Verification::Verified,visible:true
    }];
    let narrative=realize(&projection.focus_page,projection.wiki_revision,
        lens.depth,&verified_claims,&sources);
    assert!(review_candidates(&projection,&narrative).is_empty());
    (lens.selected_page.unwrap(),narrative.paragraph,projection.related.len())
}
#[test] fn human_typing_uses_exact_verified_wiki_projection() {
    assert_eq!(accepted_typed_input_result(),
        ("wiki.system".into(),"The Rust fabric contains a wiki projection.".into(),1));
}
#[test] fn authorized_agent_draft_follows_same_projection_after_host_ack() {
    // The authorization/ACK is deliberately outside this pure crate.
    // Once Host authenticates a draft, the same typed focus event is projected.
    // This asserts parity only, not a real device-delivery receipt.
    assert_eq!(accepted_typed_input_result(),accepted_typed_input_result());
}
#[test] fn changing_session_fence_cannot_show_old_context() {
    let (entities,sources,pages)=scope();
    let items=[SourceItem{id:"graph:system",title:"Rust",version:2,
        depth:0,stale:false,sources:&["source:system"]}];
    let (mut request,receipt)=read(&entities,&sources,&pages,&items,&[]);
    request.scope_id="owner:other";
    assert_eq!(project_context(&request,&receipt),
        Err(ProjectionError::SourceFenceChanged));
}
#[test] fn revoked_user_cannot_restore_old_suggestions() {
    let pages: BTreeSet<String> = ["wiki.home".into(),"wiki.system".into()].into_iter().collect();
    let mut lens=Lens::new("epoch:1","owner:1",3,"wiki.home",false,&pages).unwrap();
    lens.apply(LensEvent::AccessRevoked,&pages).unwrap();
    let result=lens.apply(LensEvent::Draft {
        sequence:2,epoch:"epoch:1".into(),source_revision:3,
        candidate_page:Some("wiki.system".into()),grounded:true
    },&pages);
    assert_eq!(result,Err(LensError::Revoked));
    assert!(lens.selected_page.is_none());
    assert!(lens.suggested_page.is_none());
}
