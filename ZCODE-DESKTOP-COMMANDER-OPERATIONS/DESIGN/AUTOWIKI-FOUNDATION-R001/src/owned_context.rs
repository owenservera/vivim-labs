//! R003: ergonomic bounded Host/HQKG boundary adapter.
//! Inputs are COPIES of an existing HQKG ContextBundle receipt, not a graph.
//! The existing native graph owns traversal, source identity and freshness.
//! This adapter intentionally does not issue a source read or a Host request.
use crate::bundle_projection::{
    project_context, ContextReceipt, LensProjection, LensRead,
    ProjectionError, SourceEdge, SourceItem,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedSourceItem {
    pub id: String,
    pub title: String,
    pub version: u64,
    pub depth: usize,
    pub stale: bool,
    pub sources: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedSourceEdge {
    pub from: String,
    pub to: String,
    pub relation: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedContextReceipt {
    pub scope_id: String,
    pub graph_epoch: u64,
    pub wiki_revision: u64,
    pub items: Vec<OwnedSourceItem>,
    pub edges: Vec<OwnedSourceEdge>,
    pub truncated: bool,
}
impl OwnedContextReceipt {
    pub fn project(&self, request: &LensRead<'_>)
        -> Result<LensProjection, ProjectionError> {
        // Vec<Vec<&str>> owns the slice arrays; each &str points into self.
        // This avoids a self-referential ContextReceipt and keeps all lifetimes
        // within the single read-only projection call.
        let source_slices: Vec<Vec<&str>> = self.items.iter()
            .map(|item| item.sources.iter().map(String::as_str).collect())
            .collect();
        let borrowed_items: Vec<_> = self.items.iter().zip(source_slices.iter())
            .map(|(item,refs)| SourceItem {
                id:&item.id,title:&item.title,version:item.version,
                depth:item.depth,stale:item.stale,sources:refs.as_slice(),
            }).collect();
        let borrowed_edges: Vec<_> = self.edges.iter()
            .map(|e|SourceEdge {
                from:&e.from,to:&e.to,relation:&e.relation,
            }).collect();
        project_context(request,&ContextReceipt {
            scope_id:&self.scope_id,graph_epoch:self.graph_epoch,
            wiki_revision:self.wiki_revision,items:&borrowed_items,
            edges:&borrowed_edges,truncated:self.truncated,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::context_lens::Depth;
    use std::collections::{BTreeMap,BTreeSet};
    #[test]
    fn owned_receipt_adapts_without_another_graph_walk() {
        let nodes: BTreeSet<String> = ["graph:root".into()].into_iter().collect();
        let sources: BTreeSet<String> = ["source:root".into()].into_iter().collect();
        let pages: BTreeMap<String,String> =
            [("graph:root".into(),"wiki.system".into())].into_iter().collect();
        let q=LensRead {
            focus_entity:"graph:root",scope_id:"owner:1",
            expected_graph_epoch:8,expected_wiki_revision:3,
            depth:Depth::Overview,trace_authorized:false,
            allowed_entities:&nodes,allowed_source_refs:&sources,
            page_for_entity:&pages,
        };
        let receipt=OwnedContextReceipt {
            scope_id:"owner:1".into(),graph_epoch:8,wiki_revision:3,
            items:vec![OwnedSourceItem {
                id:"graph:root".into(),title:"Root".into(),
                version:1,depth:0,stale:false,
                sources:vec!["source:root".into()]
            }], edges:vec![],truncated:false,
        };
        let result=receipt.project(&q).unwrap();
        assert_eq!(result.focus_page,"wiki.system");
        assert_eq!(result.cards.len(),1);
    }
}
