//! R004: presentation-side stale-result check for the existing AutoWiki lens.
//! No extra source of truth, polling, network, permission grant or state store.
//! The existing Lens owns input sequence, scope, revision and selection.
//! Host must update Lens on access revoke/source change before accepting a read.
use super::bundle_projection::LensProjection;
use super::context_lens::{Depth, Lens};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadTicket {
    pub epoch: String,
    pub scope_id: String,
    pub wiki_revision: u64,
    pub graph_epoch: u64,
    pub latest_input_sequence: u64,
    pub page_id: String,
    pub depth: Depth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadbackError {
    Revoked, MissingPage, EpochChanged, ScopeChanged, WikiChanged,
    GraphChanged, Superseded, FocusChanged, DepthChanged,
}

fn current_page(lens: &Lens) -> Option<&str> {
    if lens.pinned {
        lens.selected_page.as_deref()
    } else {
        lens.suggested_page.as_deref().or(lens.selected_page.as_deref())
    }
}

/// Create an ephemeral ticket AFTER the incumbent Lens::apply admitted a
/// canonical page ID. The epoch is Lens's Host/source identity, not graph epoch.
pub fn issue_read(lens: &Lens, graph_epoch: u64) -> Result<ReadTicket, ReadbackError> {
    if lens.revoked { return Err(ReadbackError::Revoked); }
    let page_id = current_page(lens).ok_or(ReadbackError::MissingPage)?;
    Ok(ReadTicket {
        epoch: lens.epoch.clone(), scope_id: lens.scope_id.clone(),
        wiki_revision: lens.source_revision, graph_epoch,
        latest_input_sequence: lens.latest_input_sequence,
        page_id: page_id.to_owned(), depth: lens.depth,
    })
}

/// Fail closed when a delayed native/Host result returns after human/agent
/// typing, revocation, navigation, permission or graph/source revision change.
/// current_graph_epoch MUST be freshly source-attested by the Host, not copied
/// from the result or supplied by the browser. No text or graph data is logged.
pub fn accept_readback(
    lens: &Lens,
    ticket: &ReadTicket,
    projection: &LensProjection,
    current_graph_epoch: u64,
) -> Result<(), ReadbackError> {
    if lens.revoked { return Err(ReadbackError::Revoked); }
    if lens.epoch != ticket.epoch { return Err(ReadbackError::EpochChanged); }
    if lens.scope_id != ticket.scope_id || projection.scope_id != ticket.scope_id {
        return Err(ReadbackError::ScopeChanged);
    }
    if lens.source_revision != ticket.wiki_revision ||
       projection.wiki_revision != ticket.wiki_revision {
        return Err(ReadbackError::WikiChanged);
    }
    if ticket.graph_epoch != current_graph_epoch ||
       projection.graph_epoch != ticket.graph_epoch {
        return Err(ReadbackError::GraphChanged);
    }
    if lens.latest_input_sequence != ticket.latest_input_sequence {
        return Err(ReadbackError::Superseded);
    }
    if lens.depth != ticket.depth { return Err(ReadbackError::DepthChanged); }
    if current_page(lens) != Some(ticket.page_id.as_str()) ||
       projection.focus_page != ticket.page_id {
        return Err(ReadbackError::FocusChanged);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::bundle_projection::ContextCard;
    use super::super::context_lens::{Change, LensEvent};
    use std::collections::BTreeSet;

    fn pages() -> BTreeSet<String> {
        ["wiki.home", "wiki.system"].into_iter().map(str::to_owned).collect()
    }
    fn lens() -> Lens {
        Lens::new("host:epoch", "owner:scope", 7, "wiki.home", false, &pages()).unwrap()
    }
    fn projection(page: &str) -> LensProjection {
        LensProjection {
            focus_page: page.into(), scope_id: "owner:scope".into(),
            wiki_revision: 7, graph_epoch: 11,
            cards: vec![ContextCard {
                entity_id: "node:1".into(), page_id: page.into(),
                title: "Visible".into(), version: 1, distance: 0,
                source_refs: vec!["source:1".into(),
            ]}], related: vec![], truncated: false,
        }
    }
    #[test] fn current_ticket_admits_existing_projection() {
        let l = lens();
        let t = issue_read(&l, 11).unwrap();
        assert_eq!(accept_readback(&l, &t, &projection("wiki.home"), 11), Ok(()));
    }
    #[test] fn later_human_or_agent_input_supersedes_response() {
        let mut l = lens();
        let t = issue_read(&l, 11).unwrap();
        assert_eq!(l.apply(LensEvent::Draft {
            sequence: 1, epoch: "host:epoch".into(), source_revision: 7,
            candidate_page: Some("wiki.system".into()), grounded: true,
        }, &pages()), Ok(Change::Suggested));
        assert_eq!(accept_readback(&l, &t, &projection("wiki.home"), 11),
            Err(ReadbackError::Superseded));
    }
    #[test] fn scope_replacement_and_revocation_refuse_private_readback() {
        let mut l = lens();
        let t = issue_read(&l, 11).unwrap();
        l.apply(LensEvent::SourceChanged {
            epoch: "other:epoch".into(), scope_id: "owner:other".into(),
            revision: 7, home: "wiki.home".into(), trace_allowed: false,
        }, &pages()).unwrap();
        assert_eq!(accept_readback(&l, &t, &projection("wiki.home"), 11),
            Err(ReadbackError::EpochChanged));
        l.apply(LensEvent::AccessRevoked, &pages()).unwrap();
        assert_eq!(accept_readback(&l, &t, &projection("wiki.home"), 11),
            Err(ReadbackError::Revoked));
    }
    #[test] fn actual_graph_epoch_cannot_be_taken_from_cached_reply() {
        let l = lens();
        let t = issue_read(&l, 11).unwrap();
        assert_eq!(accept_readback(&l, &t, &projection("wiki.home"), 12),
            Err(ReadbackError::GraphChanged));
    }
    #[test] fn pinned_selection_displaces_ambient_suggestion() {
        let mut l = lens();
        l.apply(LensEvent::SetPinned(true), &pages()).unwrap();
        l.apply(LensEvent::Draft {
            sequence: 1, epoch: "host:epoch".into(), source_revision: 7,
            candidate_page: Some("wiki.system".into()), grounded: true,
        }, &pages()).unwrap();
        let t = issue_read(&l, 11).unwrap();
        assert_eq!(t.page_id, "wiki.home");
        assert_eq!(accept_readback(&l, &t, &projection("wiki.home"), 11), Ok(()));
    }
    #[test] fn navigation_or_depth_change_invalidates_old_reply() {
        let mut l = lens();
        let t = issue_read(&l, 11).unwrap();
        l.apply(LensEvent::Navigate {
            epoch: "host:epoch".into(), source_revision: 7,
            page: "wiki.system".into(),
        }, &pages()).unwrap();
        assert_eq!(accept_readback(&l, &t, &projection("wiki.home"), 11),
            Err(ReadbackError::FocusChanged));
        let t = issue_read(&l, 11).unwrap();
        l.apply(LensEvent::SetDepth(Depth::Explain), &pages()).unwrap();
        assert_eq!(accept_readback(&l, &t, &projection("wiki.system"), 11),
            Err(ReadbackError::DepthChanged));
    }
}
