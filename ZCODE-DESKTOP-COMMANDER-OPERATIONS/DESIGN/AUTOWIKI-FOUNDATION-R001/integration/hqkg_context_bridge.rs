//! R003 integration-only bridge for placement inside ARBOR's
//! native/hqkg/src/autowiki_context_bridge.rs (NOT compiled in Labs staging).
//! Add feature-gated pub mod autowiki_context_bridge to native/hqkg/src/lib.rs
//! only when the Rust projection is integrated.
//! This existing feature already depends on vivim-programmability-core.
//!
//! IMPORTANT: A Host source owner must supply trusted_scope, trusted_wiki_revision
//! and a LensRead built from current actor/item/source permissions, NOT a
//! browser-provided allowlist. No trust can be conferred by this adapter.
use crate::context_retrieval::ContextBundle;
use vivim_programmability_core::autowiki::bundle_projection::{
    LensProjection, LensRead, ProjectionError,
};
use vivim_programmability_core::autowiki::owned_context::{
    OwnedContextReceipt, OwnedSourceEdge, OwnedSourceItem,
};

/// No BFS, graph lookup or effect: adapt the existing query_scoped receipt.
/// The source owner should have called HQKG query_scoped with its actual
/// authorized prefix, exclusions, watermarks and read_budget limits.
pub fn project_authorized_context(
    bundle: &ContextBundle,
    trusted_scope: &str,
    trusted_wiki_revision: u64,
    request: &LensRead<'_>,
) -> Result<LensProjection, ProjectionError> {
    let borrowed = OwnedContextReceipt {
        scope_id:trusted_scope.to_owned(),
        graph_epoch:bundle.epoch,
        wiki_revision:trusted_wiki_revision,
        items:bundle.items.iter().map(|item| OwnedSourceItem {
            id:item.id.clone(),
            title:item.title.clone(),
            version:item.version,
            depth:item.depth,
            stale:item.stale,
            sources:item.sources.iter().cloned().collect(),
        }).collect(),
        edges:bundle.edges.iter().map(|edge| OwnedSourceEdge {
            from:edge.from.clone(),
            to:edge.to.clone(),
            relation:edge.relation.clone(),
        }).collect(),
        truncated:bundle.budget_exhausted,
    };
    borrowed.project(request)
}
