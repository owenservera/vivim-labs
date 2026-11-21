//! R002 pure contextual navigation policy for AutoWiki.
//! This does not interpret raw text, retrieve graph facts, write Wiki pages,
//! execute commands, grant permissions, persist drafts or render human prose.
//! It consumes an EXISTING interpreter's canonical page ID under an EXISTING
//! source owner's authorized page list and the EXISTING graph revision.
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Depth { Overview, Explain, Trace }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LensError {
    UnauthorizedPage, StaleInput, SourceRevisionMismatch,
    SourceEpochMismatch, SourceRevisionRegressed, Revoked,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Change { NoChange, Suggested, Selected, ChangedDepth, Invalidated, Revoked }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lens {
    pub selected_page: Option<String>,
    pub suggested_page: Option<String>,
    pub pinned: bool,
    pub depth: Depth,
    pub epoch: String,
    pub scope_id: String,
    pub source_revision: u64,
    pub latest_input_sequence: u64,
    pub revoked: bool,
}

pub enum LensEvent {
    /// A canonical semantic ID resolved by the incumbent read-only interpreter.
    /// Raw draft text never enters this Rust projection.
    Draft {
        sequence: u64,
        epoch: String,
        source_revision: u64,
        candidate_page: Option<String>,
        grounded: bool,
    },
    Navigate { epoch: String, source_revision: u64, page: String },
    SetPinned(bool),
    SetDepth(Depth),
    /// Source-owner attested revision transition; any authority scope change
    /// invalidates private state even if the numeric revision is unchanged.
    SourceChanged { epoch: String, scope_id: String, revision: u64, home: String },
    ClearSuggestion,
    AccessRevoked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleBudget {
    pub max_items: usize,
    pub budget_tokens: u64,
}
pub fn read_budget(depth: Depth) -> BundleBudget {
    // Design ceilings; existing HQKG query_scoped owns the graph walk.
    match depth {
        Depth::Overview => BundleBudget { max_items: 8, budget_tokens: 1200 },
        Depth::Explain => BundleBudget { max_items: 20, budget_tokens: 3500 },
        Depth::Trace => BundleBudget { max_items: 48, budget_tokens: 7000 },
    }
}

impl Lens {
    pub fn new(
        epoch: &str, scope_id: &str, revision: u64, home: &str,
        allowed: &BTreeSet<String>,
    ) -> Result<Self, LensError> {
        if !allowed.contains(home) { return Err(LensError::UnauthorizedPage) }
        Ok(Self {
            selected_page: Some(home.to_owned()), suggested_page: None,
            pinned: false, depth: Depth::Overview, epoch: epoch.to_owned(),
            scope_id: scope_id.to_owned(), source_revision: revision,
            latest_input_sequence: 0, revoked: false,
        })
    }
    /// Returns only authorized page IDs and information that is *not* evidence
    /// of execution; the caller must independently redact every fact/section.
    pub fn apply(
        &mut self, event: LensEvent, allowed: &BTreeSet<String>,
    ) -> Result<Change, LensError> {
        if let LensEvent::AccessRevoked = event {
            self.selected_page = None;
            self.suggested_page = None;
            self.pinned = false;
            self.revoked = true;
            return Ok(Change::Revoked)
        }
        if let LensEvent::SourceChanged { epoch, scope_id, revision, home } = event {
            let reset = self.revoked || epoch != self.epoch || scope_id != self.scope_id;
            if !reset && revision < self.source_revision {
                return Err(LensError::SourceRevisionRegressed)
            }
            if !allowed.contains(&home) {
                self.selected_page = None;
                self.suggested_page = None;
                self.pinned = false;
                self.revoked = true;
                return Ok(Change::Revoked)
            }
            if reset {
                self.epoch = epoch;
                self.scope_id = scope_id;
                self.source_revision = revision;
                self.selected_page = Some(home);
                self.suggested_page = None;
                self.pinned = false;
                self.depth = Depth::Overview;
                self.latest_input_sequence = 0;
                self.revoked = false;
                return Ok(Change::Invalidated)
            }
            if revision == self.source_revision { return Ok(Change::NoChange) }
            self.source_revision = revision;
            self.suggested_page = None;
            if !self.selected_page.as_ref().is_some_and(|p| allowed.contains(p)) {
                self.selected_page = Some(home);
                self.pinned = false;
                self.depth = Depth::Overview;
            }
            return Ok(Change::Invalidated)
        }
        if self.revoked { return Err(LensError::Revoked) }
        // All event handlers validate the current selected page against
        // current authorized scope. A stale pinned page cannot persist.
        if !self.selected_page.as_ref().is_some_and(|p| allowed.contains(p)) {
            self.selected_page = None;
            self.suggested_page = None;
            self.pinned = false;
            self.revoked = true;
            return Ok(Change::Revoked)
        }
        match event {
            LensEvent::Draft { sequence, epoch, source_revision, candidate_page, grounded } => {
                if epoch != self.epoch { return Err(LensError::SourceEpochMismatch) }
                if source_revision != self.source_revision {
                    return Err(LensError::SourceRevisionMismatch)
                }
                if sequence <= self.latest_input_sequence {
                    return Err(LensError::StaleInput)
                }
                self.latest_input_sequence = sequence;
                // Reading/typing never replaces an explicitly selected page.
                self.suggested_page = candidate_page.filter(|p| grounded && allowed.contains(p));
                Ok(Change::Suggested)
            }
            LensEvent::Navigate { epoch, source_revision, page } => {
                if epoch != self.epoch { return Err(LensError::SourceEpochMismatch) }
                if source_revision != self.source_revision {
                    return Err(LensError::SourceRevisionMismatch)
                }
                if !allowed.contains(&page) { return Err(LensError::UnauthorizedPage) }
                self.selected_page = Some(page);
                self.suggested_page = None;
                self.pinned = false; // Explicit navigation always chooses new focus.
                Ok(Change::Selected)
            }
            LensEvent::SetPinned(pinned) => {
                self.pinned = pinned;
                Ok(Change::NoChange)
            }
            LensEvent::SetDepth(depth) => {
                self.depth = depth;
                Ok(Change::ChangedDepth)
            }
            LensEvent::ClearSuggestion => {
                self.suggested_page = None;
                Ok(Change::NoChange)
            }
            LensEvent::SourceChanged { .. } | LensEvent::AccessRevoked => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn allow() -> BTreeSet<String> {
        ["wiki.home", "wiki.system", "wiki.commands"].into_iter()
            .map(str::to_owned).collect()
    }
    fn initial() -> Lens { Lens::new("epoch-a", "scope-a", 8, "wiki.home", &allow()).unwrap() }
    fn draft(sequence: u64, page: &str) -> LensEvent {
        LensEvent::Draft { sequence, epoch: "epoch-a".into(), source_revision: 8,
            candidate_page: Some(page.into()), grounded: true }
    }
    #[test] fn typing_suggests_but_does_not_navigate() {
        let mut s = initial();
        assert_eq!(s.apply(draft(1, "wiki.system"), &allow()), Ok(Change::Suggested));
        assert_eq!(s.selected_page.as_deref(), Some("wiki.home"));
        assert_eq!(s.suggested_page.as_deref(), Some("wiki.system"));
    }
    #[test] fn stale_input_cannot_replace_newer_suggestion() {
        let mut s = initial();
        s.apply(draft(5, "wiki.system"), &allow()).unwrap();
        assert_eq!(s.apply(draft(4, "wiki.commands"), &allow()), Err(LensError::StaleInput));
        assert_eq!(s.suggested_page.as_deref(), Some("wiki.system"));
    }
    #[test] fn unknown_or_ungrounded_suggestion_not_exposed() {
        let mut s = initial();
        s.apply(draft(1, "secret.page"), &allow()).unwrap();
        assert_eq!(s.suggested_page, None);
        s.apply(LensEvent::Draft { sequence: 2, epoch: "epoch-a".into(),
            source_revision: 8, candidate_page: Some("wiki.system".into()),
            grounded: false }, &allow()).unwrap();
        assert_eq!(s.suggested_page, None);
    }
    #[test] fn explicit_navigation_is_only_navigation() {
        let mut s = initial();
        s.apply(draft(1, "wiki.commands"), &allow()).unwrap();
        s.apply(LensEvent::Navigate { epoch: "epoch-a".into(),
            source_revision: 8, page: "wiki.commands".into() }, &allow()).unwrap();
        assert_eq!(s.selected_page.as_deref(), Some("wiki.commands"));
        s.apply(LensEvent::SetPinned(true), &allow()).unwrap();
        s.apply(draft(2, "wiki.system"), &allow()).unwrap();
        assert!(s.pinned);
        assert_eq!(s.selected_page.as_deref(), Some("wiki.commands"));
    }
    #[test] fn stale_revision_rejected_and_new_revision_invalidates_preview() {
        let mut s = initial();
        s.apply(draft(1, "wiki.system"), &allow()).unwrap();
        assert_eq!(s.apply(LensEvent::SourceChanged { epoch:"epoch-a".into(),
            scope_id:"scope-a".into(), revision: 9, home:"wiki.home".into() }, &allow()),
            Ok(Change::Invalidated));
        assert_eq!(s.suggested_page, None);
        assert_eq!(s.apply(draft(2, "wiki.system"), &allow()),
            Err(LensError::SourceRevisionMismatch));
    }
    #[test] fn scope_change_clears_pin_and_selection() {
        let mut s = initial();
        s.apply(LensEvent::Navigate { epoch:"epoch-a".into(), source_revision:8,
            page:"wiki.system".into() }, &allow()).unwrap();
        s.apply(LensEvent::SetPinned(true), &allow()).unwrap();
        let only_home: BTreeSet<String> = ["wiki.home".to_string()].into_iter().collect();
        s.apply(LensEvent::SourceChanged { epoch:"epoch-a".into(),
            scope_id:"scope-b".into(), revision:8, home:"wiki.home".into() },
            &only_home).unwrap();
        assert_eq!(s.selected_page.as_deref(), Some("wiki.home"));
        assert!(!s.pinned);
        assert_eq!(s.depth, Depth::Overview);
    }
    #[test] fn revoked_access_cannot_leak_earlier_page() {
        let mut s = initial();
        s.apply(LensEvent::AccessRevoked, &allow()).unwrap();
        assert_eq!(s.selected_page, None);
        assert_eq!(s.suggested_page, None);
        assert_eq!(s.apply(draft(1, "wiki.system"), &allow()), Err(LensError::Revoked));
    }
    #[test] fn depth_budget_is_monotone() {
        let a = read_budget(Depth::Overview);
        let b = read_budget(Depth::Explain);
        let c = read_budget(Depth::Trace);
        assert!(a.max_items < b.max_items && b.max_items < c.max_items);
        assert!(a.budget_tokens < b.budget_tokens && b.budget_tokens < c.budget_tokens);
    }
    #[test] fn explicit_link_cannot_open_unauthorized_page() {
        let mut s = initial();
        assert_eq!(s.apply(LensEvent::Navigate {epoch:"epoch-a".into(),
            source_revision:8,page:"private.page".into()}, &allow()),
            Err(LensError::UnauthorizedPage));
        assert_eq!(s.selected_page.as_deref(), Some("wiki.home"));
    }
}
