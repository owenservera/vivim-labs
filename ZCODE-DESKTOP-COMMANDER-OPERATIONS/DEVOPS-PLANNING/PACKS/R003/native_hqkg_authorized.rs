//! Copy into ARBOR native/hqkg/tests/autowiki_authorized.rs after Dev has
//! installed the staged R003 modules and authorized_query_extension.rs.
//! Integration test is NOT runnable in Labs alone. No synthetic pass claim.
#![cfg(feature = "native-program")]
use hqkg::{Graph, Node, NodeKind, Edge};
use hqkg::autowiki_context_bridge::{read_authorized_context, AuthorizedReadError};
use std::collections::{BTreeMap, BTreeSet};
use vivim_programmability_core::autowiki::context_lens::Depth;
use vivim_programmability_core::autowiki::query_admission::{
    TrustedQueryScope, QueryAdmissionError,
};
use vivim_programmability_core::autowiki::bundle_projection::LensRead;

fn graph() -> Graph {
    let mut g=Graph::new();
    g.put_node(Node::new("scope/a", NodeKind::Concept, "Public A").source("source:a")).unwrap();
    g.put_node(Node::new("scope/b", NodeKind::Concept, "Public B").source("source:b")).unwrap();
    g.put_node(Node::new("scope/private", NodeKind::Concept, "Sensitive").source("source:secret")).unwrap();
    g.put_edge(Edge::new("edge:1","scope/a","related","scope/b")).unwrap();
    g.put_edge(Edge::new("edge:2","scope/a","related","scope/private")).unwrap();
    g
}

fn query<'a>(
    graph: &Graph,
    allowed: &'a BTreeSet<String>,
    excluded: &'a BTreeSet<String>,
) -> TrustedQueryScope<'a> {
    TrustedQueryScope {
        scope_prefix:"scope/",focus_entity:"scope/a",
        authorized_entities:allowed,excluded_entities:excluded,
        exclusions_complete:true,
        graph_epoch:graph.epoch(),expected_graph_epoch:graph.epoch(),
    }
}
#[test]
fn private_neighbour_is_excluded_before_graph_traversal() {
    let g=graph();
    let allowed:BTreeSet<String>=["scope/a","scope/b"].into_iter().map(str::to_owned).collect();
    let excluded:BTreeSet<String>=["scope/private"].into_iter().map(str::to_owned).collect();
    let sources:BTreeSet<String>=["source:a","source:b"].into_iter().map(str::to_owned).collect();
    let pages:BTreeMap<String,String>=[
        ("scope/a","wiki.system"),("scope/b","wiki.knowledge")
    ].into_iter().map(|(a,b)|(a.to_owned(),b.to_owned())).collect();
    let grants=query(&g,&allowed,&excluded);
    let request=LensRead {
        focus_entity:"scope/a",scope_id:"owner:scope",
        expected_graph_epoch:g.epoch(),expected_wiki_revision:3,
        depth:Depth::Overview,trace_authorized:false,
        allowed_entities:&allowed,allowed_source_refs:&sources,
        page_for_entity:&pages,
    };
    let result=read_authorized_context(&g,&grants,&request,Depth::Overview,"owner:scope",3).unwrap();
    assert_eq!(result.focus_page,"wiki.system");
    assert!(result.cards.iter().all(|card|card.entity_id!="scope/private"));
    assert!(result.related.iter().all(|edge|edge.to_page!="wiki.private"));
}
#[test]
fn incomplete_coverage_does_not_turn_prefix_into_grant() {
    let g=graph();
    let allowed:BTreeSet<String>=["scope/a"].into_iter().map(str::to_owned).collect();
    let excluded=BTreeSet::new();
    let sources:BTreeSet<String>=["source:a"].into_iter().map(str::to_owned).collect();
    let pages:BTreeMap<String,String>=[("scope/a".to_string(),"wiki.system".to_string())].into_iter().collect();
    let mut grants=query(&g,&allowed,&excluded);
    grants.exclusions_complete=false;
    let request=LensRead {
        focus_entity:"scope/a",scope_id:"owner:scope",
        expected_graph_epoch:g.epoch(),expected_wiki_revision:3,
        depth:Depth::Overview,trace_authorized:false,
        allowed_entities:&allowed,allowed_source_refs:&sources,
        page_for_entity:&pages,
    };
    assert!(matches!(
        read_authorized_context(&g,&grants,&request,Depth::Overview,"owner:scope",3),
        Err(AuthorizedReadError::Admission(QueryAdmissionError::UntrustedCoverage))
    ));
}
