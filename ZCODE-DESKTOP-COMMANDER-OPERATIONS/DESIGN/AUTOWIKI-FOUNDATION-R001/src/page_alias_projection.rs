//! R004 exact transient join over EXISTING Host committed Wiki page IDs.
//! No new semantic registry, generated identity, graph, journal, grant or I/O.
//! Host supplies wiki:page: semantic IDs itself: do NOT recompute URI encoding.
use std::collections::{BTreeMap, BTreeSet};
use super::HUBS;

#[derive(Clone, Copy, Debug)]
pub struct CommittedPage<'a> {
    pub name: &'a str,
    pub semantic_page_id: &'a str,
}
#[derive(Clone, Copy, Debug)]
pub struct HubBinding<'a> {
    pub hub_alias: &'a str,
    pub semantic_page_id: &'a str,
    pub entity_id: &'a str,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactHubMap {
    pub page_for_entity: BTreeMap<String, String>,
    pub canonical_for_hub: BTreeMap<String, String>,
    pub committed_name_for_hub: BTreeMap<String, String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubMapError {
    EmptyBinding, UnknownAlias, InvalidCanonicalId, UnknownCommittedPage,
    Unauthorized, DuplicateHub, DuplicateEntity, DuplicateCanonicalPage,
    AmbiguousCommittedPage,
}
fn valid_canonical(id: &str) -> bool {
    id.starts_with("wiki:page:") && id.len() > "wiki:page:".len()
        && id.len() <= 400 && !id.chars().any(char::is_control)
}
fn valid_entity(id: &str) -> bool {
    !id.is_empty() && id.len() <= 256 && id.bytes().all(|b|
        b.is_ascii_alphanumeric() || matches!(b, b'.' | b':' | b'_' | b'-' | b'/'))
}
/// Committed and bindings must be fresh source-owner data. Authorized sets
/// MUST be current Host decisions, never supplied by an untrusted PWA.
/// Missing hubs remain unavailable; they are not synthesized.
pub fn resolve_exact_hubs(
    committed: &[CommittedPage<'_>],
    bindings: &[HubBinding<'_>],
    authorized_page_ids: &BTreeSet<String>,
    authorized_entities: &BTreeSet<String>,
) -> Result<ExactHubMap, HubMapError> {
    if bindings.len() > HUBS.len() { return Err(HubMapError::DuplicateHub); }
    let mut committed_ids = BTreeMap::new();
    let mut committed_names = BTreeSet::new();
    for page in committed {
        if page.name.trim().is_empty() || !valid_canonical(page.semantic_page_id) {
            return Err(HubMapError::InvalidCanonicalId);
        }
        if committed_ids.insert(page.semantic_page_id, page.name).is_some() ||
            !committed_names.insert(page.name) {
            return Err(HubMapError::AmbiguousCommittedPage);
        }
    }
    let mut page_for_entity = BTreeMap::new();
    let mut canonical_for_hub = BTreeMap::new();
    let mut committed_name_for_hub = BTreeMap::new();
    let mut seen_pages = BTreeSet::new();
    for binding in bindings {
        if binding.entity_id.is_empty() || binding.semantic_page_id.is_empty() {
            return Err(HubMapError::EmptyBinding);
        }
        if !HUBS.contains(&binding.hub_alias) {
            return Err(HubMapError::UnknownAlias);
        }
        if !valid_canonical(binding.semantic_page_id) || !valid_entity(binding.entity_id) {
            return Err(HubMapError::InvalidCanonicalId);
        }
        // Deny before probing committed page existence to avoid a private-page oracle.
        if !authorized_page_ids.contains(binding.semantic_page_id) ||
            !authorized_entities.contains(binding.entity_id) {
            return Err(HubMapError::Unauthorized);
        }
        let Some(name) = committed_ids.get(binding.semantic_page_id) else {
            return Err(HubMapError::UnknownCommittedPage);
        };
        if canonical_for_hub.contains_key(binding.hub_alias) {
            return Err(HubMapError::DuplicateHub);
        }
        if !seen_pages.insert(binding.semantic_page_id) {
            return Err(HubMapError::DuplicateCanonicalPage);
        }
        if page_for_entity.insert(binding.entity_id.to_owned(), binding.hub_alias.to_owned()).is_some() {
            return Err(HubMapError::DuplicateEntity);
        }
        canonical_for_hub.insert(binding.hub_alias.to_owned(), binding.semantic_page_id.to_owned());
        committed_name_for_hub.insert(binding.hub_alias.to_owned(), (*name).to_owned());
    }
    Ok(ExactHubMap { page_for_entity, canonical_for_hub, committed_name_for_hub })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn committed() -> [CommittedPage<'static>; 2] {
        [CommittedPage { name: "system", semantic_page_id: "wiki:page:system" },
         CommittedPage { name: "knowledge", semantic_page_id: "wiki:page:knowledge" }]
    }
    fn binding() -> HubBinding<'static> {
        HubBinding { hub_alias: "wiki.system", semantic_page_id: "wiki:page:system",
            entity_id: "graph:system" }
    }
    fn allowed() -> (BTreeSet<String>, BTreeSet<String>) {
        (["wiki:page:system", "wiki:page:knowledge"].into_iter().map(str::to_owned).collect(),
         ["graph:system", "graph:knowledge"].into_iter().map(str::to_owned).collect())
    }
    #[test] fn joins_exact_committed_ids() {
        let (p,n) = allowed();
        let x = resolve_exact_hubs(&committed(), &[binding()], &p, &n).unwrap();
        assert_eq!(x.page_for_entity["graph:system"], "wiki.system");
        assert_eq!(x.canonical_for_hub["wiki.system"], "wiki:page:system");
        assert_eq!(x.committed_name_for_hub["wiki.system"], "system");
    }
    #[test] fn revoked_page_denied_before_existence_test() {
        let (mut p,n) = allowed();p.remove("wiki:page:system");
        assert_eq!(resolve_exact_hubs(&committed(), &[binding()], &p, &n),
            Err(HubMapError::Unauthorized));
    }
    #[test] fn nonexistent_page_never_synthesized() {
        let (mut p,n) = allowed();p.insert("wiki:page:missing".into());
        let bad = HubBinding { semantic_page_id: "wiki:page:missing", ..binding() };
        assert_eq!(resolve_exact_hubs(&committed(), &[bad], &p, &n),
            Err(HubMapError::UnknownCommittedPage));
    }
    #[test] fn duplicate_entity_or_alias_refused() {
        let (p,n) = allowed();
        assert_eq!(resolve_exact_hubs(&committed(), &[binding(),binding()], &p, &n),
            Err(HubMapError::DuplicateHub));
        let second = HubBinding { hub_alias: "wiki.knowledge",
            semantic_page_id: "wiki:page:knowledge", entity_id: "graph:system" };
        assert_eq!(resolve_exact_hubs(&committed(), &[binding(),second], &p, &n),
            Err(HubMapError::DuplicateEntity));
    }
    #[test] fn ambiguous_committed_name_refused() {
        let (p,n) = allowed();
        let bad = [committed()[0], CommittedPage {
            name: "system", semantic_page_id: "wiki:page:another" }];
        assert_eq!(resolve_exact_hubs(&bad, &[binding()], &p, &n),
            Err(HubMapError::AmbiguousCommittedPage));
    }
    #[test] fn unknown_hub_rejected() {
        let (p,n) = allowed();
        let bad = HubBinding { hub_alias: "wiki.secret", ..binding() };
        assert_eq!(resolve_exact_hubs(&committed(), &[bad], &p, &n),
            Err(HubMapError::UnknownAlias));
    }
}
