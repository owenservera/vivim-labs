//! R006 proposed native HQKG source-owner adapter.
//! Install at native/hqkg/src/autowiki_grant_coverage.rs only after Dev ACK.
//! This scans the incumbent graph and derives a COMPLETE excluded in-prefix
//! set from an actual Host actor grant. No separate graph, index, permission
//! store, journal, provider route or Wiki writer.
//! NOT compiled inside Labs: requires crate hqkg and existing native-program.
use std::collections::BTreeSet;
use crate::Graph;
use crate::autowiki_context_bridge::{read_authorized_context, AuthorizedReadError};
use vivim_programmability_core::autowiki::{
    bundle_projection::{LensProjection,LensRead},
    context_lens::Depth,
    query_admission::TrustedQueryScope,
};

const MAX_SCANNED_NODES: usize = 65_536;
const MAX_ALLOWED: usize = 2_048;
const MAX_PREFIX_BYTES: usize = 128;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentHostGrant {
    pub actor_id: String,
    pub grant_revision: u64,
    /// The *actual* native graph namespace authorized by Host, with delimiter.
    pub scope_prefix: String,
    pub focus_entity: String,
    pub allowed_entities: BTreeSet<String>,
    pub allowed_source_refs: BTreeSet<String>,
}

/// Implement this trait only in the existing Host actor/grant authority. This
/// Rust interface does NOT authenticate a caller or make an untrusted browser
/// grant valid. An absent source should return an error/None, never fall back.
pub trait CurrentHostGrantSource {
    fn current_grant(&self) -> Option<CurrentHostGrant>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverageError {
    GrantUnavailable, ActorMismatch, InvalidGrant, ScopeTooLarge,
    MissingGrantedEntity, UngroundedAllowedEntity, GraphChanged,
    GrantChanged, ScopeMismatch,
}
#[derive(Debug, Clone)]
pub struct CompleteGraphCoverage {
    pub actor_id: String,
    pub grant_revision: u64,
    pub scope_prefix: String,
    pub focus_entity: String,
    pub allowed_entities: BTreeSet<String>,
    pub allowed_source_refs: BTreeSet<String>,
    pub excluded_entities: BTreeSet<String>,
    pub graph_epoch: u64,
}
impl CompleteGraphCoverage {
    /// Borrow immutable trusted selection into incumbent R004 query_admission.
    /// This boolean becomes true ONLY after a complete bounded graph scan.
    pub fn read_scope(&self)->TrustedQueryScope<'_> {
        TrustedQueryScope {
            scope_prefix:&self.scope_prefix,
            focus_entity:&self.focus_entity,
            authorized_entities:&self.allowed_entities,
            excluded_entities:&self.excluded_entities,
            exclusions_complete:true,
            graph_epoch:self.graph_epoch,
            expected_graph_epoch:self.graph_epoch,
        }
    }
}
#[derive(Debug)]
pub enum FencedReadError {
    Coverage(CoverageError),
    Read(AuthorizedReadError),
}

fn valid(s: &str) -> bool {
    !s.is_empty() && s.len()<=256 && s.bytes().all(|b|
        b.is_ascii_alphanumeric() || matches!(b,b'.'|b':'|b'_'|b'-'|b'/'))
}
fn prefix_valid(s: &str) -> bool {
    s.len()<=MAX_PREFIX_BYTES && valid(s) &&
        (s.ends_with('/') || s.ends_with(':'))
}
/// A full bounded graph scan provides absence/denial coverage without creating
/// another index. Too large/unknown = refuse, never promote a partial scan.
/// The owner must re-read the actor grant AFTER a native query as well, because
/// permission revocation does not necessarily advance HQKG graph epoch.
pub fn derive_complete_coverage(
    graph:&Graph,
    host:&dyn CurrentHostGrantSource,
    expected_actor:&str,
) -> Result<CompleteGraphCoverage,CoverageError> {
    let grant=host.current_grant().ok_or(CoverageError::GrantUnavailable)?;
    if !valid(expected_actor) || grant.actor_id!=expected_actor {
        return Err(CoverageError::ActorMismatch)
    }
    if grant.grant_revision==0 ||
       !prefix_valid(&grant.scope_prefix) ||
       !valid(&grant.focus_entity) ||
       !grant.focus_entity.starts_with(&grant.scope_prefix) ||
       grant.allowed_entities.is_empty() ||
       grant.allowed_entities.len()>MAX_ALLOWED ||
       !grant.allowed_entities.contains(&grant.focus_entity) ||
       grant.allowed_source_refs.is_empty() ||
       grant.allowed_source_refs.len()>MAX_ALLOWED ||
       grant.allowed_entities.iter().any(|id|!valid(id)||!id.starts_with(&grant.scope_prefix)) ||
       grant.allowed_source_refs.iter().any(|id|!valid(id)) {
        return Err(CoverageError::InvalidGrant)
    }
    let epoch=graph.epoch();
    let mut excluded=BTreeSet::new();
    let mut found=BTreeSet::new();
    // Use exact current Graph::nodes(), which is the ONLY graph authority.
    for (index,node) in graph.nodes().enumerate() {
        if index>=MAX_SCANNED_NODES {
            return Err(CoverageError::ScopeTooLarge)
        }
        if !node.id.starts_with(&grant.scope_prefix) {continue}
        if !valid(&node.id) {return Err(CoverageError::InvalidGrant)}
        if !grant.allowed_entities.contains(&node.id) {
            excluded.insert(node.id.clone());
        } else {
            // Never show a node whose provenance contains any unauthorized
            // source, and do not make a source-less node into a Wiki fact.
            if node.sources.is_empty() ||
               !node.sources.is_subset(&grant.allowed_source_refs) {
                return Err(CoverageError::UngroundedAllowedEntity)
            }
            found.insert(node.id.clone());
        }
    }
    if found!=grant.allowed_entities {
        return Err(CoverageError::MissingGrantedEntity)
    }
    if graph.epoch()!=epoch {return Err(CoverageError::GraphChanged)}
    // Do NOT silently promote a set whose trusted grant was revoked during
    // the scan even if HQKG epoch and node versions remain unchanged.
    if host.current_grant()!=Some(grant.clone()) {
        return Err(CoverageError::GrantChanged)
    }
    Ok(CompleteGraphCoverage {
        actor_id:grant.actor_id,
        grant_revision:grant.grant_revision,
        scope_prefix:grant.scope_prefix,
        focus_entity:grant.focus_entity,
        allowed_entities:grant.allowed_entities,
        allowed_source_refs:grant.allowed_source_refs,
        excluded_entities:excluded,
        graph_epoch:epoch,
    })
}

/// One authorized native retrieval through existing query_scoped, with both
/// preflight and postflight Host-grant checks. The supplied LensRead must
/// agree with the source owner, not an independent user-chosen allowlist.
pub fn read_with_current_grant(
    graph:&Graph,
    host:&dyn CurrentHostGrantSource,
    actor_id:&str,
    request:&LensRead<'_>,
    depth:Depth,
    trusted_scope_id:&str,
    trusted_wiki_revision:u64,
) -> Result<LensProjection,FencedReadError> {
    let complete=derive_complete_coverage(graph,host,actor_id)
        .map_err(FencedReadError::Coverage)?;
    if request.allowed_entities!=&complete.allowed_entities ||
       request.allowed_source_refs!=&complete.allowed_source_refs ||
       request.focus_entity!=complete.focus_entity.as_str() ||
       request.scope_id!=trusted_scope_id ||
       request.depth!=depth {
        return Err(FencedReadError::Coverage(CoverageError::ScopeMismatch))
    }
    let result=read_authorized_context(
        graph,&complete.read_scope(),request,depth,
        trusted_scope_id,trusted_wiki_revision,
    ).map_err(FencedReadError::Read)?;
    if graph.epoch()!=complete.graph_epoch {
        return Err(FencedReadError::Coverage(CoverageError::GraphChanged))
    }
    let current=host.current_grant().ok_or(
        FencedReadError::Coverage(CoverageError::GrantUnavailable))?;
    if current.actor_id!=complete.actor_id ||
       current.grant_revision!=complete.grant_revision ||
       current.scope_prefix!=complete.scope_prefix ||
       current.focus_entity!=complete.focus_entity ||
       current.allowed_entities!=complete.allowed_entities ||
       current.allowed_source_refs!=complete.allowed_source_refs {
        return Err(FencedReadError::Coverage(CoverageError::GrantChanged))
    }
    Ok(result)
}
