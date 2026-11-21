//! R006 Dev-only integration tests. Install in
//! native/hqkg/tests/autowiki_grant_coverage.rs after Dev registers the
//! feature-gated autowiki_grant_coverage module and R003/R004 native bridge.
//! NOT a passed test or an authorized live Host simulation.
#![cfg(feature="native-program")]
use hqkg::{Graph,Node,NodeKind,Edge};
use hqkg::autowiki_grant_coverage::{
    CurrentHostGrant, CurrentHostGrantSource, derive_complete_coverage,
    read_with_current_grant, CoverageError, FencedReadError,
};
use std::collections::{BTreeMap,BTreeSet};
use std::cell::Cell;
use vivim_programmability_core::autowiki::{
    bundle_projection::LensRead,
    context_lens::Depth,
};

fn graph() -> Graph {
    let mut g=Graph::new();
    g.put_node(Node::new("scope/a",NodeKind::Concept,"Allowed").source("source:a")).unwrap();
    g.put_node(Node::new("scope/private",NodeKind::Concept,"Sensitive")
        .source("source:secret")).unwrap();
    g.put_edge(Edge::new("e-a-private","scope/a","related","scope/private")).unwrap();
    g
}
fn grant() -> CurrentHostGrant {
    CurrentHostGrant {
        actor_id:"desktop-owner".into(),grant_revision:12,
        scope_prefix:"scope/".into(),focus_entity:"scope/a".into(),
        allowed_entities:["scope/a"].into_iter().map(str::to_owned).collect(),
        allowed_source_refs:["source:a"].into_iter().map(str::to_owned).collect(),
    }
}
struct StaticSource(Option<CurrentHostGrant>);
impl CurrentHostGrantSource for StaticSource {
    fn current_grant(&self)->Option<CurrentHostGrant> {self.0.clone()}
}
struct FlippingSource {
    base:CurrentHostGrant, calls:Cell<usize>, flip_at:usize,
}
impl CurrentHostGrantSource for FlippingSource {
    fn current_grant(&self)->Option<CurrentHostGrant> {
        let index=self.calls.get()+1;self.calls.set(index);
        let mut value=self.base.clone();
        if index>=self.flip_at {value.grant_revision+=1;}
        Some(value)
    }
}
#[test]
fn source_owner_scans_full_graph_and_excludes_private_neighbour() {
    let g=graph();let scope=derive_complete_coverage(
        &g,&StaticSource(Some(grant())),"desktop-owner").unwrap();
    assert!(scope.excluded_entities.contains("scope/private"));
    assert!(!scope.excluded_entities.contains("scope/a"));
    assert!(scope.read_scope().exclusions_complete);
    assert_eq!(scope.graph_epoch,g.epoch());
}
#[test]
fn absent_actor_grant_never_broadens_the_scope() {
    assert!(matches!(
        derive_complete_coverage(&graph(),&StaticSource(None),"desktop-owner"),
        Err(CoverageError::GrantUnavailable)
    ));
}
#[test]
fn mismatched_identity_refuses_before_any_graph_projection() {
    assert!(matches!(
        derive_complete_coverage(&graph(),&StaticSource(Some(grant())),"other-user"),
        Err(CoverageError::ActorMismatch)
    ));
}
#[test]
fn missing_allowed_entity_cannot_be_asserted_by_a_client() {
    let mut gnt=grant();
    gnt.allowed_entities.insert("scope/missing".into());
    assert!(matches!(
        derive_complete_coverage(&graph(),&StaticSource(Some(gnt)),"desktop-owner"),
        Err(CoverageError::MissingGrantedEntity)
    ));
}
#[test]
fn allowed_entity_with_secret_source_is_refused() {
    let mut gnt=grant();
    gnt.allowed_entities.insert("scope/private".into());
    assert!(matches!(
        derive_complete_coverage(&graph(),&StaticSource(Some(gnt)),"desktop-owner"),
        Err(CoverageError::UngroundedAllowedEntity)
    ));
}
#[test]
fn revocation_without_graph_epoch_change_fails_preflight() {
    let source=FlippingSource{base:grant(),calls:Cell::new(0),flip_at:2};
    assert!(matches!(
        derive_complete_coverage(&graph(),&source,"desktop-owner"),
        Err(CoverageError::GrantChanged)
    ));
}
#[test]
fn native_projection_uses_only_existing_bounded_hqkg_query() {
    let g=graph();let source=StaticSource(Some(grant()));let gnt=grant();
    let pages:BTreeMap<String,String>=[
        ("scope/a".into(),"wiki.system".into())
    ].into_iter().collect();
    let read=LensRead {
        focus_entity:"scope/a",scope_id:"owner:scope",
        expected_graph_epoch:g.epoch(),expected_wiki_revision:7,
        depth:Depth::Overview,trace_authorized:false,
        allowed_entities:&gnt.allowed_entities,
        allowed_source_refs:&gnt.allowed_source_refs,
        page_for_entity:&pages,
    };
    let result=read_with_current_grant(
        &g,&source,"desktop-owner",&read,Depth::Overview,"owner:scope",7,
    ).unwrap();
    assert_eq!(result.cards.len(),1);
    assert_eq!(result.cards[0].entity_id,"scope/a");
    assert_eq!(result.focus_page,"wiki.system");
}
#[test]
fn mid_read_grant_change_does_not_publish() {
    let g=graph();let gnt=grant();
    let source=FlippingSource{base:gnt.clone(),calls:Cell::new(0),flip_at:3};
    let pages:BTreeMap<String,String>=[
        ("scope/a".into(),"wiki.system".into())
    ].into_iter().collect();
    let read=LensRead {
        focus_entity:"scope/a",scope_id:"owner:scope",
        expected_graph_epoch:g.epoch(),expected_wiki_revision:7,
        depth:Depth::Overview,trace_authorized:false,
        allowed_entities:&gnt.allowed_entities,
        allowed_source_refs:&gnt.allowed_source_refs,
        page_for_entity:&pages,
    };
    assert!(matches!(
        read_with_current_grant(
            &g,&source,"desktop-owner",&read,Depth::Overview,"owner:scope",7
        ),Err(FencedReadError::Coverage(CoverageError::GrantChanged))
    ));
}
