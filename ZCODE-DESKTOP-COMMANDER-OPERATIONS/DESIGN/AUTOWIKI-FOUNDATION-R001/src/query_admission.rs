//! R004 pre-query validation. It does NOT query a graph or mint grants.
//! Existing Host/graph authority must supply an exact, current and complete
//! set of excluded node IDs before native HQKG query_scoped() is invoked.
//! This conservative gate avoids treating a prefix match as authorization.
use std::collections::BTreeSet;
use super::context_lens::{read_budget, Depth};

#[derive(Debug, Clone)]
pub struct TrustedQueryScope<'a> {
    pub scope_prefix: &'a str,
    pub focus_entity: &'a str,
    pub authorized_entities: &'a BTreeSet<String>,
    /// Every non-authorized graph ID reachable inside the query prefix MUST
    /// be represented here by the native graph/source owner.
    pub excluded_entities: &'a BTreeSet<String>,
    /// Attested after inspecting the current graph epoch; do NOT read this
    /// boolean from a browser/client or infer from a supplied prefix.
    pub exclusions_complete: bool,
    pub graph_epoch: u64,
    pub expected_graph_epoch: u64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedQuery {
    pub seeds: Vec<String>,
    pub scope: String,
    pub excluded: BTreeSet<String>,
    pub budget_tokens: u64,
    pub max_items: usize,
    pub graph_epoch: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryAdmissionError {
    UntrustedCoverage, SourceChanged, InvalidScope, UnauthorizedSeed,
    ConflictingExclusion, InvalidId, TooManyExclusions,
}
fn safe_id(value: &str) -> bool {
    !value.is_empty() && value.len() <= 256 &&
        value.bytes().all(|b| b.is_ascii_alphanumeric() ||
            matches!(b, b'.' | b':' | b'_' | b'-' | b'/'))
}
pub fn admit_query(
    scope: &TrustedQueryScope<'_>, depth: Depth,
) -> Result<AdmittedQuery, QueryAdmissionError> {
    if !scope.exclusions_complete { return Err(QueryAdmissionError::UntrustedCoverage); }
    if scope.graph_epoch != scope.expected_graph_epoch {
        return Err(QueryAdmissionError::SourceChanged);
    }
    if !safe_id(scope.scope_prefix) ||
       !safe_id(scope.focus_entity) ||
       !scope.focus_entity.starts_with(scope.scope_prefix) {
        return Err(QueryAdmissionError::InvalidScope);
    }
    if !scope.authorized_entities.contains(scope.focus_entity) {
        return Err(QueryAdmissionError::UnauthorizedSeed);
    }
    if scope.excluded_entities.len() > 65_536 {
        return Err(QueryAdmissionError::TooManyExclusions);
    }
    for entity in scope.authorized_entities {
        if !safe_id(entity) || !entity.starts_with(scope.scope_prefix) {
            return Err(QueryAdmissionError::InvalidId);
        }
        if scope.excluded_entities.contains(entity) {
            return Err(QueryAdmissionError::ConflictingExclusion);
        }
    }
    for entity in scope.excluded_entities {
        if !safe_id(entity) || !entity.starts_with(scope.scope_prefix) {
            return Err(QueryAdmissionError::InvalidId);
        }
    }
    let budget = read_budget(depth);
    Ok(AdmittedQuery {
        seeds: vec![scope.focus_entity.to_owned()],
        scope: scope.scope_prefix.to_owned(),
        excluded: scope.excluded_entities.clone(),
        budget_tokens: budget.budget_tokens,
        max_items: budget.max_items,
        graph_epoch: scope.graph_epoch,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn allowed() -> BTreeSet<String> {
        ["scope/a", "scope/b"].into_iter().map(str::to_owned).collect()
    }
    fn denied() -> BTreeSet<String> {
        ["scope/private"].into_iter().map(str::to_owned).collect()
    }
    fn scope<'a>(a: &'a BTreeSet<String>, d: &'a BTreeSet<String>) -> TrustedQueryScope<'a> {
        TrustedQueryScope {
            scope_prefix: "scope/", focus_entity: "scope/a", authorized_entities: a,
            excluded_entities: d, exclusions_complete: true,
            graph_epoch: 9, expected_graph_epoch: 9,
        }
    }
    #[test] fn current_exactly_admitted_focus_only() {
        let a=allowed();let d=denied();let s=scope(&a,&d);
        let q=admit_query(&s,Depth::Overview).unwrap();
        assert_eq!(q.seeds,vec!["scope/a"]);
        assert_eq!(q.excluded,d);
        assert!(q.max_items<=8);
    }
    #[test] fn prefix_without_complete_exclusions_is_not_a_grant() {
        let a=allowed();let d=denied();let mut s=scope(&a,&d);
        s.exclusions_complete=false;
        assert_eq!(admit_query(&s,Depth::Overview),Err(QueryAdmissionError::UntrustedCoverage));
    }
    #[test] fn disallowed_seed_is_refused() {
        let a=allowed();let d=denied();let mut s=scope(&a,&d);
        s.focus_entity="scope/private";
        assert_eq!(admit_query(&s,Depth::Overview),Err(QueryAdmissionError::UnauthorizedSeed));
    }
    #[test] fn changed_graph_epoch_refuses_old_exclusions() {
        let a=allowed();let d=denied();let mut s=scope(&a,&d);
        s.graph_epoch+=1;
        assert_eq!(admit_query(&s,Depth::Overview),Err(QueryAdmissionError::SourceChanged));
    }
    #[test] fn exclusion_cannot_conflict_with_authorized_set() {
        let a=allowed();let mut d=denied();d.insert("scope/b".into());
        let s=scope(&a,&d);
        assert_eq!(admit_query(&s,Depth::Overview),Err(QueryAdmissionError::ConflictingExclusion));
    }
    #[test] fn unrelated_prefix_ids_refused() {
        let mut a=allowed();a.insert("other/a".into());let d=denied();
        assert_eq!(admit_query(&scope(&a,&d),Depth::Overview),Err(QueryAdmissionError::InvalidId));
    }
}
