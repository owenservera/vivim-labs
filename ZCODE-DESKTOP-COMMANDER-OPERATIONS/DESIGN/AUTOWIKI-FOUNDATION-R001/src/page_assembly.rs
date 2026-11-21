//! R005: read-only hub page composition over *existing* R003 source projections.
//! Neither a Wiki writer nor another graph; missing hubs remain unavailable.
//! Human text comes exclusively from existing narrative::realize output.
//! All strings must be rendered as textContent, never interpreted as HTML.
use std::collections::BTreeSet;
use super::{HUBS, bundle_projection::LensProjection,
    context_lens::Depth, narrative::Narrative,
    page_alias_projection::ExactHubMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplanationState { Verified, Partial, Unavailable }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageLink {
    pub alias: String,
    pub canonical_id: String,
    pub title: &'static str,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HubEntry {
    pub alias: &'static str,
    pub title: &'static str,
    /// None = uncommitted, unlinked, or unauthorized. Never guess a slug.
    pub committed_canonical_id: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HumanWikiPage {
    pub page_alias: String,
    pub committed_canonical_id: String,
    pub title: &'static str,
    pub wiki_revision: u64,
    pub graph_epoch: u64,
    pub status: ExplanationState,
    pub intro: String,
    pub supporting_claim_ids: Vec<String>,
    pub related: Vec<PageLink>,
    pub navigation: Vec<HubEntry>,
    pub depth: Depth,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssembleError {
    SourceMismatch, NoCommittedPage, InvalidPage, Unauthorized,
    SourceEvidenceMismatch, UntrustedNarrative, TraceNotAllowed,
}
fn title(id: &str) -> Option<&'static str> {
    match id {
        "wiki.home"=>Some("Home"),
        "wiki.system"=>Some("System"),
        "wiki.capabilities"=>Some("Capabilities"),
        "wiki.knowledge"=>Some("Knowledge"),
        "wiki.commands"=>Some("Commands"),
        "wiki.activity"=>Some("Activity"),
        "wiki.development"=>Some("Development"),
        _=>None,
    }
}
/// Build a purely read-only page from current Host-admitted graph, exact
/// committed identity joins and independently gated narrative. Caller must
/// have accepted a current readback ticket (source epoch/digest/request seq).
pub fn assemble_human_page(
    context: &LensProjection,
    narrative: &Narrative,
    ids: &ExactHubMap,
    allowed_hubs: &BTreeSet<String>,
    allowed_sources: &BTreeSet<String>,
    depth: Depth,
    trace_allowed: bool,
) -> Result<HumanWikiPage, AssembleError> {
    if depth==Depth::Trace && !trace_allowed {
        return Err(AssembleError::TraceNotAllowed);
    }
    if context.focus_page != narrative.page_id ||
       context.wiki_revision != narrative.source_revision {
        return Err(AssembleError::SourceMismatch);
    }
    let page_id=&context.focus_page;
    let Some(page_title)=title(page_id) else {
        return Err(AssembleError::InvalidPage)
    };
    if !allowed_hubs.contains(page_id) {
        return Err(AssembleError::Unauthorized);
    }
    let Some(canonical)=ids.canonical_for_hub.get(page_id) else {
        return Err(AssembleError::NoCommittedPage);
    };
    if !ids.committed_name_for_hub.contains_key(page_id) ||
       !context.cards.iter().any(|c|c.page_id.as_str()==page_id.as_str()) {
        return Err(AssembleError::NoCommittedPage);
    }
    let included_refs:BTreeSet<&str>=context.cards.iter()
        .flat_map(|card|card.source_refs.iter().map(String::as_str)).collect();
    if context.cards.iter().any(|card|card.source_refs.iter()
        .any(|s|!allowed_sources.contains(s))) {
        return Err(AssembleError::SourceEvidenceMismatch);
    }
    let mut claims=BTreeSet::new();
    for sentence in &narrative.sentences {
        if sentence.claim_id.is_empty() ||
            sentence.evidence_refs.is_empty() ||
            sentence.evidence_refs.iter().any(|s|
                !allowed_sources.contains(s) || !included_refs.contains(s.as_str())) ||
            !claims.insert(sentence.claim_id.as_str()) {
            return Err(AssembleError::SourceEvidenceMismatch);
        }
    }
    let actual=narrative.sentences.iter()
        .map(|s|s.text.as_str()).collect::<Vec<_>>().join(" ");
    if !narrative.sentences.is_empty() && actual!=narrative.paragraph {
        return Err(AssembleError::UntrustedNarrative);
    }
    let status=if narrative.sentences.is_empty() {
        ExplanationState::Unavailable
    } else if narrative.complete && !context.truncated {
        ExplanationState::Verified
    } else {
        ExplanationState::Partial
    };
    let intro=if status==ExplanationState::Unavailable {
        "No current, verified explanation is available for this topic.".into()
    } else {
        narrative.paragraph.clone()
    };
    let mut seen=BTreeSet::new();
    let related=context.related.iter()
        .filter(|edge|edge.from_page.as_str()==page_id.as_str())
        .filter_map(|edge|{
            let target=&edge.to_page;
            if !allowed_hubs.contains(target) || !seen.insert(target.as_str()) {
                return None;
            }
            Some(PageLink {
                alias: target.clone(),
                canonical_id: ids.canonical_for_hub.get(target)?.clone(),
                title: title(target)?,
            })
        }).collect();
    let navigation=HUBS.iter().map(|alias|HubEntry {
        alias: *alias,title:title(alias).unwrap_or("Unavailable"),
        committed_canonical_id:if allowed_hubs.contains(*alias) &&
            ids.committed_name_for_hub.contains_key(*alias) {
            ids.canonical_for_hub.get(*alias).cloned()
        } else {None},
    }).collect();
    Ok(HumanWikiPage {
        page_alias:page_id.clone(), committed_canonical_id:canonical.clone(),
        title:page_title,wiki_revision:context.wiki_revision,
        graph_epoch:context.graph_epoch,status,intro,
        supporting_claim_ids:claims.into_iter().map(str::to_owned).collect(),
        related,navigation,depth,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use super::super::bundle_projection::{ContextCard,RelatedLink};
    use super::super::narrative::{Narrative,Sentence};

    fn ids()->ExactHubMap {
        ExactHubMap {
            page_for_entity:[("graph:core".into(),"wiki.system".into()),
                ("graph:wiki".into(),"wiki.knowledge".into())].into_iter().collect(),
            canonical_for_hub:[("wiki.system".into(),"wiki:page:system".into()),
                ("wiki.knowledge".into(),"wiki:page:knowledge".into())].into_iter().collect(),
            committed_name_for_hub:[("wiki.system".into(),"system".into()),
                ("wiki.knowledge".into(),"knowledge".into())].into_iter().collect(),
        }
    }
    fn allowed()->BTreeSet<String> {
        ["wiki.system","wiki.knowledge"].into_iter().map(str::to_owned).collect()
    }
    fn sources()->BTreeSet<String> {
        ["source:core","source:wiki"].into_iter().map(str::to_owned).collect()
    }
    fn context()->LensProjection {
        LensProjection {
            focus_page:"wiki.system".into(),scope_id:"owner:1".into(),
            graph_epoch:9,wiki_revision:7,truncated:false,
            cards:vec![
                ContextCard {entity_id:"graph:core".into(),page_id:"wiki.system".into(),
                    title:"Core".into(),version:1,distance:0,source_refs:vec!["source:core".into()]},
                ContextCard {entity_id:"graph:wiki".into(),page_id:"wiki.knowledge".into(),
                    title:"Wiki".into(),version:1,distance:1,source_refs:vec!["source:wiki".into()]},
            ],
            related:vec![RelatedLink{
                from_page:"wiki.system".into(),to_page:"wiki.knowledge".into(),
                relation_id:"related".into(),
            }],
        }
    }
    fn narrative()->Narrative {
        Narrative {
            page_id:"wiki.system".into(),source_revision:7,complete:true,
            paragraph:"Under model M, the core uses a bounded query.".into(),
            sentences:vec![Sentence{
                text:"Under model M, the core uses a bounded query.".into(),
                claim_id:"verified-1".into(),
                evidence_refs:vec!["source:core".into()],
            }], diagnostics:vec![],
        }
    }
    #[test] fn existing_verified_text_and_canonical_links_are_retained() {
        let x=assemble_human_page(&context(),&narrative(),&ids(),&allowed(),
            &sources(),Depth::Overview,false).unwrap();
        assert_eq!(x.status,ExplanationState::Verified);
        assert_eq!(x.related[0].canonical_id,"wiki:page:knowledge");
        assert_eq!(x.supporting_claim_ids,["verified-1"]);
        assert_eq!(x.intro,"Under model M, the core uses a bounded query.");
    }
    #[test] fn missing_hubs_show_unavailable_not_synthetic_links() {
        let x=assemble_human_page(&context(),&narrative(),&ids(),&allowed(),
            &sources(),Depth::Overview,false).unwrap();
        assert_eq!(x.navigation.len(),7);
        assert_eq!(x.navigation.iter().find(|i|i.alias=="wiki.home").unwrap()
            .committed_canonical_id,None);
    }
    #[test] fn mismatched_wiki_revision_fails_closed() {
        let mut n=narrative();n.source_revision=8;
        assert_eq!(assemble_human_page(&context(),&n,&ids(),&allowed(),
            &sources(),Depth::Overview,false),Err(AssembleError::SourceMismatch));
    }
    #[test] fn missing_canonical_page_not_fabricated() {
        let mut map=ids();map.canonical_for_hub.remove("wiki.system");
        assert_eq!(assemble_human_page(&context(),&narrative(),&map,&allowed(),
            &sources(),Depth::Overview,false),Err(AssembleError::NoCommittedPage));
    }
    #[test] fn private_source_sentence_is_rejected_without_partial_page() {
        let mut n=narrative();n.sentences[0].evidence_refs=vec!["secret:source".into()];
        assert_eq!(assemble_human_page(&context(),&n,&ids(),&allowed(),
            &sources(),Depth::Overview,false),Err(AssembleError::SourceEvidenceMismatch));
    }
    #[test] fn untrusted_paragraph_cannot_replace_proven_sentence_text() {
        let mut n=narrative();n.paragraph="Everything works now!".into();
        assert_eq!(assemble_human_page(&context(),&n,&ids(),&allowed(),
            &sources(),Depth::Overview,false),Err(AssembleError::UntrustedNarrative));
    }
    #[test] fn empty_narrative_is_truthfully_unavailable() {
        let mut n=narrative();n.sentences.clear();n.paragraph.clear();n.complete=false;
        let x=assemble_human_page(&context(),&n,&ids(),&allowed(),
            &sources(),Depth::Overview,false).unwrap();
        assert_eq!(x.status,ExplanationState::Unavailable);
        assert!(x.intro.contains("No current, verified explanation"));
    }
    #[test] fn truncated_graph_does_not_claim_complete_verification() {
        let mut c=context();c.truncated=true;
        let x=assemble_human_page(&c,&narrative(),&ids(),&allowed(),
            &sources(),Depth::Overview,false).unwrap();
        assert_eq!(x.status,ExplanationState::Partial);
    }
    #[test] fn trace_depth_requires_independent_host_permission() {
        assert_eq!(assemble_human_page(&context(),&narrative(),&ids(),&allowed(),
            &sources(),Depth::Trace,false),Err(AssembleError::TraceNotAllowed));
    }
}
