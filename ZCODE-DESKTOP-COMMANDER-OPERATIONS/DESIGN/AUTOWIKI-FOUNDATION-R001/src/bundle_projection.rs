//! R003: admission-only projection of ALREADY-RETRIEVED HQKG context.
//! This module does NOT traverse a graph or infer a canonical identity.
//! Inputs must be adapted from hqkg::context_retrieval::ContextBundle after
//! the existing Host has checked actor/item/source permissions.
//! For staging: std-only; no I/O, network, journaling, effect dispatch or cache.
use std::collections::{BTreeMap, BTreeSet};
use super::context_lens::{read_budget, Depth};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceItem<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub version: u64,
    pub depth: usize,
    pub stale: bool,
    pub sources: &'a [&'a str],
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceEdge<'a> {
    pub from: &'a str,
    pub to: &'a str,
    pub relation: &'a str,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextReceipt<'a> {
    /// Attested by source owner; cannot be derived from an untrusted browser field.
    pub scope_id: &'a str,
    pub graph_epoch: u64,
    pub wiki_revision: u64,
    pub items: &'a [SourceItem<'a>],
    pub edges: &'a [SourceEdge<'a>],
    pub truncated: bool,
}
#[derive(Clone, Debug)]
pub struct LensRead<'a> {
    pub focus_entity: &'a str,
    pub scope_id: &'a str,
    pub expected_graph_epoch: u64,
    pub expected_wiki_revision: u64,
    pub depth: Depth,
    /// Exact policy decision from Host, not an argument from the PWA.
    pub trace_authorized: bool,
    pub allowed_entities: &'a BTreeSet<String>,
    pub allowed_source_refs: &'a BTreeSet<String>,
    /// Authority-owned exact mapping. No fuzzy labels and no implicit IDs.
    pub page_for_entity: &'a BTreeMap<String, String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextCard {
    pub entity_id: String,
    pub page_id: String,
    pub title: String,
    pub version: u64,
    pub distance: usize,
    pub source_refs: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelatedLink {
    pub from_page: String,
    pub to_page: String,
    pub relation_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LensProjection {
    pub focus_page: String,
    pub scope_id: String,
    pub graph_epoch: u64,
    pub wiki_revision: u64,
    pub cards: Vec<ContextCard>,
    pub related: Vec<RelatedLink>,
    pub truncated: bool,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProjectionError {
    SourceFenceChanged, Unauthorized, MissingFocus, StaleSource,
    MalformedContext, TraceNotAuthorized,
}
fn safe_label(value: &str) -> bool {
    !value.trim().is_empty() && value.chars().count() <= 180
        && !value.chars().any(char::is_control)
}
fn safe_id(value: &str) -> bool {
    !value.is_empty() && value.len() <= 256
        && value.bytes().all(|b| b.is_ascii_alphanumeric()
            || matches!(b, b'.' | b':' | b'_' | b'-' | b'/'))
}
/// Note: the original native context retrieval applies *estimated token* bounds.
/// We reuse those bounds through its query_scoped call; this final adapter only
/// validates item count and current authorization, not a second selection rule.
/// Unauthorized content yields one generic refusal: no partial disclosure, counts,
/// source labels or reason-dependent private metadata.
pub fn project_context(
    request: &LensRead<'_>,
    receipt: &ContextReceipt<'_>,
) -> Result<LensProjection, ProjectionError> {
    if request.depth == Depth::Trace && !request.trace_authorized {
        return Err(ProjectionError::TraceNotAuthorized);
    }
    if receipt.scope_id != request.scope_id ||
        receipt.graph_epoch != request.expected_graph_epoch ||
        receipt.wiki_revision != request.expected_wiki_revision {
        return Err(ProjectionError::SourceFenceChanged);
    }
    if !safe_id(request.focus_entity) || !safe_id(request.scope_id) ||
        receipt.items.len() > read_budget(request.depth).max_items ||
        receipt.edges.len() > 512 {
        return Err(ProjectionError::MalformedContext);
    }
    let mut seen = BTreeSet::new();
    let mut cards = Vec::new();
    for item in receipt.items {
        // Check admission before even validating a private item's metadata,
        // so a denied title/source cannot affect the outward refusal class.
        if !request.allowed_entities.contains(item.id) ||
            item.sources.iter().any(|s| !request.allowed_source_refs.contains(*s)) {
            return Err(ProjectionError::Unauthorized);
        }
        if !safe_id(item.id) || !safe_label(item.title) || item.sources.len() > 24 ||
            item.sources.iter().any(|s| !safe_id(s)) ||
            item.depth > 3 || !seen.insert(item.id) {
            return Err(ProjectionError::MalformedContext);
        }
        if item.stale { return Err(ProjectionError::StaleSource) }
        // A source-less occurrence cannot become a factual display card.
        if item.sources.is_empty() { continue }
        let Some(page_id) = request.page_for_entity.get(item.id) else { continue };
        if !safe_id(page_id) { return Err(ProjectionError::MalformedContext) }
        cards.push(ContextCard {
            entity_id: item.id.to_owned(), page_id: page_id.clone(),
            title: item.title.to_owned(), version: item.version,
            distance: item.depth,
            source_refs: item.sources.iter().map(|s| (*s).to_owned()).collect(),
        });
    }
    let Some(focus) = cards.iter().find(|c| c.entity_id == request.focus_entity) else {
        return Err(ProjectionError::MissingFocus)
    };
    let focus_page = focus.page_id.clone();
    let known: BTreeMap<&str, &ContextCard> =
        cards.iter().map(|c| (c.entity_id.as_str(), c)).collect();
    let mut related = Vec::new();
    let mut edge_seen = BTreeSet::new();
    for edge in receipt.edges {
        // Do not inspect metadata of edges outside the admitted subset.
        if !known.contains_key(edge.from) || !known.contains_key(edge.to) {
            continue;
        }
        if !safe_id(edge.from) || !safe_id(edge.to) || !safe_id(edge.relation) {
            return Err(ProjectionError::MalformedContext);
        }
        // Raw graph edges grant neither navigation nor textual semantics.
        // An edge is exposed only between two admitted source-backed cards.
        if let (Some(from), Some(to)) = (known.get(edge.from), known.get(edge.to)) {
            if from.page_id != to.page_id &&
                edge_seen.insert((from.page_id.as_str(), to.page_id.as_str(),
                    edge.relation)) {
                related.push(RelatedLink {
                    from_page: from.page_id.clone(),
                    to_page: to.page_id.clone(),
                    relation_id: edge.relation.to_owned(),
                });
            }
        }
    }
    Ok(LensProjection {
        focus_page, scope_id: request.scope_id.to_owned(),
        graph_epoch: receipt.graph_epoch, wiki_revision: receipt.wiki_revision,
        cards, related, truncated: receipt.truncated,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn allowed() -> (BTreeSet<String>, BTreeSet<String>, BTreeMap<String,String>) {
        let nodes = ["graph:wiki", "graph:core"].iter().map(|s|s.to_string()).collect();
        let sources = ["source:wiki", "source:core"].iter().map(|s|s.to_string()).collect();
        let pages = [("graph:wiki","wiki.knowledge"),("graph:core","wiki.system")]
            .iter().map(|(a,b)|(a.to_string(),b.to_string())).collect();
        (nodes,sources,pages)
    }
    fn inputs<'a>(
        nodes: &'a BTreeSet<String>, sources: &'a BTreeSet<String>,
        pages: &'a BTreeMap<String,String>, items: &'a [SourceItem<'a>],
        edges: &'a [SourceEdge<'a>],
    ) -> (LensRead<'a>,ContextReceipt<'a>) {
        (LensRead{focus_entity:"graph:wiki",scope_id:"owner:1",
            expected_graph_epoch:8,expected_wiki_revision:4,
            depth:Depth::Overview,trace_authorized:false,
            allowed_entities:nodes,allowed_source_refs:sources,page_for_entity:pages},
        ContextReceipt{scope_id:"owner:1",graph_epoch:8,wiki_revision:4,
            items,edges,truncated:false})
    }
    #[test] fn current_scoped_graph_has_only_exact_related_links() {
        let (nodes,sources,pages)=allowed();
        let items=[
            SourceItem{id:"graph:wiki",title:"Wiki",version:2,depth:0,
                stale:false,sources:&["source:wiki"]},
            SourceItem{id:"graph:core",title:"Rust Core",version:2,depth:1,
                stale:false,sources:&["source:core"]},
        ];
        let edges=[SourceEdge{from:"graph:wiki",to:"graph:core",relation:"depends_on"}];
        let (q,r)=inputs(&nodes,&sources,&pages,&items,&edges);
        let result=project_context(&q,&r).unwrap();
        assert_eq!(result.cards.len(),2);
        assert_eq!(result.focus_page,"wiki.knowledge");
        assert_eq!(result.related.len(),1);
        assert_eq!(result.related[0].to_page,"wiki.system");
    }
    #[test] fn source_epoch_mismatch_is_refused() {
        let (nodes,sources,pages)=allowed();
        let items=[SourceItem{id:"graph:wiki",title:"Wiki",version:2,depth:0,
            stale:false,sources:&["source:wiki"]}];
        let (q,mut r)=inputs(&nodes,&sources,&pages,&items,&[]);
        r.graph_epoch+=1;
        assert_eq!(project_context(&q,&r),Err(ProjectionError::SourceFenceChanged));
    }
    #[test] fn unauthorized_item_is_not_partially_exposed() {
        let (mut nodes,sources,pages)=allowed();
        nodes.remove("graph:core");
        let items=[
            SourceItem{id:"graph:wiki",title:"Wiki",version:2,depth:0,
                stale:false,sources:&["source:wiki"]},
            SourceItem{id:"graph:core",title:"SECRET",version:1,depth:1,
                stale:false,sources:&["source:core"]},
        ];
        let (q,r)=inputs(&nodes,&sources,&pages,&items,&[]);
        assert_eq!(project_context(&q,&r),Err(ProjectionError::Unauthorized));
    }
    #[test] fn stale_context_is_refused() {
        let (nodes,sources,pages)=allowed();
        let items=[SourceItem{id:"graph:wiki",title:"Wiki",version:2,depth:0,
            stale:true,sources:&["source:wiki"]}];
        let (q,r)=inputs(&nodes,&sources,&pages,&items,&[]);
        assert_eq!(project_context(&q,&r),Err(ProjectionError::StaleSource));
    }
    #[test] fn source_less_focus_never_becomes_fact() {
        let (nodes,sources,pages)=allowed();
        let items=[SourceItem{id:"graph:wiki",title:"Wiki",version:2,depth:0,
            stale:false,sources:&[]}];
        let (q,r)=inputs(&nodes,&sources,&pages,&items,&[]);
        assert_eq!(project_context(&q,&r),Err(ProjectionError::MissingFocus));
    }
    #[test] fn trace_is_not_client_authorized() {
        let (nodes,sources,pages)=allowed();
        let items=[SourceItem{id:"graph:wiki",title:"Wiki",version:2,depth:0,
            stale:false,sources:&["source:wiki"]}];
        let (mut q,r)=inputs(&nodes,&sources,&pages,&items,&[]);
        q.depth=Depth::Trace;
        assert_eq!(project_context(&q,&r),Err(ProjectionError::TraceNotAuthorized));
    }
    #[test] fn denied_source_ref_refuses_entire_bundle() {
        let (nodes,mut sources,pages)=allowed();
        sources.remove("source:wiki");
        let items=[SourceItem{id:"graph:wiki",title:"Wiki",version:2,depth:0,
            stale:false,sources:&["source:wiki"]}];
        let (q,r)=inputs(&nodes,&sources,&pages,&items,&[]);
        assert_eq!(project_context(&q,&r),Err(ProjectionError::Unauthorized));
    }
}
