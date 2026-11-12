# R008 — AutoWiki: Every Screen State Has a Living Conceptual Mirror

**Status:** expanded product vision and HQ architecture handoff, 2026-11-12. No live AutoWiki implementation is claimed.
**Authoritative technical workstream:** `repos-hq/research/autowiki-screen-mirror/` on draft research branch `research/autowiki-screen-mirror-20261112`.
**Full links:** [Product PRD](https://github.com/owenservera/repos-hq/blob/research/autowiki-screen-mirror-20261112/research/autowiki-screen-mirror/PRD-AW-001.md) · [Architecture](https://github.com/owenservera/repos-hq/blob/research/autowiki-screen-mirror-20261112/research/autowiki-screen-mirror/ARCHITECTURE-AW-001.md) · [Source and gap census](https://github.com/owenservera/repos-hq/blob/research/autowiki-screen-mirror-20261112/research/autowiki-screen-mirror/AUTOWIKI-SOURCE-AUDIT.v0.json) · [48 work orders](https://github.com/owenservera/repos-hq/blob/research/autowiki-screen-mirror-20261112/research/autowiki-screen-mirror/IMPLEMENTATION-ORDERS-AW-001.md).

## Product principle

**Flip any eligible VIVIM screen into a conceptual wiki of the active task, created from the complete *available and authorized* corpus, automatically and with current verified information first.** Return to the original screen without losing working state.

The display is NOT a separate search surface, an AI transcript paraphraser or a new permanent wiki generator. It is an **ephemeral, reversible, source-linked perspective** on the same activity.

This builds directly on:
- R004 — signal-dense composer visual grammar;
- R005 — source-aware visual text interpretation and safe menus;
- R006 — prediction from sparse commands and prewarming;
- R007 — VIEW ALL multi-layer semantic interpretation;
- R008 — **current-screen context → auto-generated conceptual knowledge mirror**.

## ERA 1 — What the owner screenshot means to AutoWiki

Observed from a user-uploaded screenshot, not from an automatic Host reader: the active application is ChatGPT, the selected chat is the Visual Command Lab discussion, the visible content is about HQ intelligence/semantic interpretations, adjacent conversation titles include Rust core interaction design, and the unsent text proposes AutoWiki.

A proper AutoWiki would ground **three concurrent contexts**:
1. **Project/space:** VIVIM (strong current-topic candidate; not a verified project ID without crosswalk);
2. **Workstream:** Living Composer, interpretation X-ray, HQ shared Rust intelligence and wiki;
3. **Immediate goal:** investigate AutoWiki and document architecture/PRD, **provisional because unsent draft**.

It should answer, with source links:
- Where does the current work fit into VIVIM's larger design?
- Which modules and contracts already exist?
- What changed most recently *in the actual sources*?
- Which decisions are accepted, proposed, stale or contradicted?
- Which repositories/files/PRDs should I inspect?
- Which open integration gaps block the next capability?
- Which relevant DCBs or prior conversations are actually available under current owner authorization?
- What is not known because this source is incomplete?

### Example projection (illustrative, not a product screenshot)

~~~text
┌─ AutoWiki: VIVIM / Living Intelligence ─────────────────────────────┐
│ SOURCE: bound ChatGPT thread · visible-context PARTIAL · 12 Nov 26  │
│ Focus: Visual Command Lab  /  Interpretation X-ray  /  AutoWiki      │
│                                                                     │
│ WHAT THIS SCREEN IS ABOUT                                            │
│  Designing VIVIM's interface where language, graph understanding     │
│  and knowledge become inspectable. The current unsent prompt         │
│  explores an automatic wiki mirror for each screen.                  │
│                                                                     │
│ MOST RECENT RELEVANT WORK                                            │
│  • R007 full-interpretation PRD / 43 facets / 22 gaps [draft]        │
│  • R006 predictive two-Tab command research [research]              │
│  • Existing HQ wiki and source reads [source-observed]               │
│                                                                     │
│ CONCEPT MAP                                                         │
│  Living Composer ─── Interpretation Trace ─── Rust Graph             │
│         │                   │                  │                     │
│      AutoWiki ───── Screen Context ───────── HQ Wiki/KB              │
│                                  └───────── DCB/Memory [Host]        │
│                                                                     │
│ NEEDS WIRING                                                        │
│  Trusted current-screen focus · four-repo HQ scope · authorized KB   │
│  client · temporal ranking · flip preserving state                   │
│                                                                     │
│ WHAT WE DO NOT KNOW                                                 │
│  Older chat turns beyond rendered DOM · private archive status ·     │
│  current installed HQ index coverage · device acceptance            │
│                                                                     │
│  [VIEW ALL LINKS]   [FOLLOW SCREEN]   [PIN FOCUS]   [FLIP BACK]      │
└─────────────────────────────────────────────────────────────────────┘
~~~

The design must display real status from each source. Counts/dates in this illustration describe the known research artifacts, not a live current-index read.

## ERA 2 — Expanded interface behaviors (beyond MVP)

### Mirror as an alternate representation of the same world

- **Zero-prompt flip.** VIVIM already knows the active view and uses that *typed* state to query existing knowledge. The user never has to write “Tell me about this screen.”
- **Semantic position indicator.** A compact `Project › Subsystem › Topic › Active object` chain shows where the current screen sits in VIVIM's graph.
- **Knowledge time axis.** Prioritize the newest *verified source events*, while anchoring accepted architectural decisions regardless of age.
- **Latest-versus-canonical.** Separate “what changed recently” from “what is currently true.” This is one of the highest-value protections against misleading summaries.
- **Conceptual map without a giant canvas.** Render relationships between the screen's selected entities and known graph neighbors, with source-linked wires and unresolved-node marks.
- **Source-matched article structure.** A code file mirror should show ownership/consumers/risk; a conversation mirror should show ideas/decisions/related work; a DCB mirror should show lifecycle/source/disclosure.
- **Screen follow versus focus pin.** Follow updates as screen changes; focus pin holds the conceptual page topic without writing it permanently.
- **Contextual counterpart.** Selecting a source row in AutoWiki can highlight its related entity in the original view when the original UI actually exposes such a source anchor.
- **One-tap expansion.** A user sees concise useful cards; VIEW ALL developers can reveal relevance weights, index head, source digest, missing producer states and suppressed evidence.
- **Cross-screen concept continuity.** Flipping from a TierFlow scene to the Host repository file corresponding to a selected control can maintain a source-linked conceptual route.
- **Memory and DCB contextual relationships.** Display personal/owner-authorized memories and DCB definitions linked to the current activity—but only when exact refs/grants are established; lexical flavor is not a saved bundle.
- **Historical lens.** Compare “what was known last week” with “what is currently verified” using the source's actual event timeline.
- **Open issue lens.** Only surface known issues/blockers from source-backed records; do not create a false bug just because a model could imagine one.
- **Return-state integrity.** Reversing the flip restores the original draft, selection, caret, scroll and view when those values exist.
- **Permission-sensitive mirror.** AutoWiki is always less or equally privileged than the original screen. It cannot become an alternate path into hidden conversations.

### Anti-patterns

- **Not OCR:** app-owned screen state is already typed; generic screen scraping increases privacy and error risk.
- **Not permanent wiki flood:** tens of screen changes should not generate tens of durable wiki markdown files.
- **Not a second AI history store:** personal corpus stays under Host; HQ's development wiki remains development intelligence.
- **Not a new search router:** source-owning read APIs continue to do their existing work.
- **Not an excuse for unsupported completeness:** current ChatGPT DOM is only what is rendered and attached, not all past conversation content.

### Screen-to-concept mapping

| Source view | Desired conceptual article |
|---|---|
| Active chat thread | Current idea, prior context, decisions, contradictions, relevant project source and open work |
| Composer with partially typed text | Active provisional intent, compatible commands, contextual evidence and unknowns |
| VIVIM project dashboard | Project identity, progress, latest verified changes, architecture and blockers |
| TierFlow world/editor | UI scene intent, component relationships, World revision, implementation pointers |
| Rust graph/code file | System role, upstream/downstream dependents, related tests, accepted invariants |
| DCB definition | Purpose, family, source provenance, parent references, revision/expiry/disclosure |
| Personal task/mission | Active goal, prior observations, pending approvals and authorized relevant knowledge |
| Unbound external website | No automatic confidential reading; optionally request manual topic or explicit trusted attachment |

### Product-specific status language

`Current topic` may be **inferred** from visual text. `Current project` may be **grounded** by native app selection. `Latest commit` may be **observed** from indexed repo/HEAD. `Accepted decision` must be **source-confirmed**. `Personal fact` requires **owner authorized** source. `Unknown` and `partial` are first-class screen states.

## ERA 3 — HQ and integration handoff

The hard work is not HTML or card layouts. The P0 dependencies identified in the HQ deep source audit are:

1. Admit the four canonical VIVIM repositories into HQ's actual local index and verify HEAD/freshness.
2. Emit source-owned, scoped and revisioned `ScreenContext` from native UI; optionally attach read-only bound provider chat DOM.
3. Convert trusted screen focus and provisional task terms into a query over *existing* HQ wiki/KB/context/graph and authorized Host personal sources.
4. Rank source results by topic/project/recent verified work, with explicit source status and conflict/omission.
5. Generate one `AutoWikiPage` read-only ephemeral view, not a second source of truth.
6. Wire real read client to TierFlow's existing knowledge feed and implement an instant reversible screen flip.
7. Prove in-flight race rejection, privacy, source revision and incomplete-history behavior.
8. Build mobile/PWA parity, then later the VIEW ALL interpretability cross-link and predictive prewarm.

**Contract files and full atomic work orders live in HQ.** Avoid implementing the same semantic ranking, identity store or source indexing twice in Labs.

The follow-on experiment should demonstrate one **native VIVIM screen→AutoWiki→back** and one **exactly bound signed-in ChatGPT thread→AutoWiki→back** with source-linked content, then expand to more screen classes under the same contract.

This R008 document is a new vision layer, not an assertion that current product source has accepted or run AutoWiki.
