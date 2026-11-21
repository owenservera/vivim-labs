//! R004 drop-in extension to ARBOR native/hqkg/src/autowiki_context_bridge.rs.
//! Append to the existing R003 staged bridge after moving it into HQKG.
//! Not compiled in Labs as a standalone module.
//! Host must construct TrustedQueryScope from current grant and graph-epoch
//! bound COMPLETE exclusions. No client-supplied allowlists or completeness.
use crate::{Graph, Error as HqkgError};
use crate::context_retrieval::{query_scoped, ScopedQuery};
use vivim_programmability_core::autowiki::query_admission::{
    admit_query, TrustedQueryScope, QueryAdmissionError,
};
use vivim_programmability_core::autowiki::bundle_projection::{
    LensProjection, LensRead, ProjectionError,
};
use vivim_programmability_core::autowiki::context_lens::Depth;

#[derive(Debug)]
pub enum AuthorizedReadError {
    Admission(QueryAdmissionError),
    Retrieval(HqkgError),
    Projection(ProjectionError),
    Changed,
}

/// Read through incumbent scoped HQKG BFS. No provider call, action, journal
/// write, second graph query or private source-index phone bypass.
/// The source owner MUST attest complete excluded graph IDs at this epoch.
pub fn read_authorized_context(
    graph: &Graph,
    grant: &TrustedQueryScope<'_>,
    request: &LensRead<'_>,
    depth: Depth,
    trusted_scope_id: &str,
    trusted_wiki_revision: u64,
) -> Result<LensProjection, AuthorizedReadError> {
    if graph.epoch() != grant.graph_epoch ||
       request.scope_id != trusted_scope_id ||
       request.expected_wiki_revision != trusted_wiki_revision ||
       request.depth != depth {
        return Err(AuthorizedReadError::Changed);
    }
    let admitted = admit_query(grant, depth).map_err(AuthorizedReadError::Admission)?;
    if admitted.seeds.first().map(String::as_str) != Some(request.focus_entity) {
        return Err(AuthorizedReadError::Admission(QueryAdmissionError::UnauthorizedSeed));
    }
    if request.expected_graph_epoch != admitted.graph_epoch {
        return Err(AuthorizedReadError::Changed);
    }
    if request.allowed_entities != grant.authorized_entities {
        return Err(AuthorizedReadError::Admission(QueryAdmissionError::UnauthorizedSeed));
    }
    let query = ScopedQuery {
        seeds: admitted.seeds,
        scope: admitted.scope,
        excluded: admitted.excluded,
        budget_tokens: admitted.budget_tokens,
        max_items: admitted.max_items,
        watermark: None,
    };
    let bundle = query_scoped(graph, &query).map_err(AuthorizedReadError::Retrieval)?;
    if graph.epoch() != grant.graph_epoch || bundle.epoch != grant.graph_epoch {
        return Err(AuthorizedReadError::Changed);
    }
    project_authorized_context(
        &bundle, trusted_scope_id, trusted_wiki_revision, request,
    ).map_err(AuthorizedReadError::Projection)
}
