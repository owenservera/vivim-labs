# R002 integration-ready wiring sprint — do not deploy from this design folder
Owner-requested scope: UX progression -> source-backed AutoWiki focus lens -> VLANG/command feedback. Product coding is a separate explicit integration step on refreshed ARBOR main.

## Verified source signatures
- apps/mobile-relay/app.js renderWiki() is the wiki tab DOM producer. It renders all pages into scroll, has rustWikiSearch(S.wiki,wikiSearchQuery), and only invokes runWikiSearch() on Enter/Search button. It uses pageCards scrollIntoView and a revision-checked Wiki source.
- apps/mobile-relay/app.js refreshWiki() is the existing authenticated Host Wiki read boundary, with wikiReadSequence, device grant session headers, scope fence and revision conflict guards. Reuse; never bypass or load a private owner endpoint from paired phone.
- apps/mobile-relay/composer-assist.mjs mountComposerAssist() already has debounce, latest-input-wins, source scope invalidation and inert suggestions. It does not expose a generic current semantic topic ID or a real-time wiki focus event; adding a scoped read-only observer hook is needed.
- apps/mobile-relay/first-contact-composer.ts has existing true human input/IME/agent-draft identification. Do not create another phone textbox, another write observer, or a raw draft logging feed.
- core/programmability/src/wiki_read.rs supplies native read/search and revision/digest guard; extend the **existing** Rust read projection to produce bounded read-only suggestion cards.
- native/hqkg/src/context_retrieval.rs query_scoped(graph, ScopedQuery) is the sole graph traversal primitive; map page semantic identities to graph seeds on source-owner side.
- apps/vivim-host/chat/wiki-semantics.mjs supplies wiki:page/section IDs and journal/source evidence classes; graph IDs and page names require an explicit verified crosswalk.

## Slice 1 — Ambient matches using existing read model, no novel NLP
1. In mobile Wiki UI, add a separate small ambient suggestions area adjacent to the existing Wiki canvas. Preserve main page and scroll; do not make whole-page rendering occur on every composer input.
2. Subscribe to the single existing composer input source via a read-only hook, respecting IME, accessibility and source scope. Do not modify text/caret, and do not fire a synthetic input event.
3. Normalize and bound lookup text; use existing Rust wiki search result from in-memory authorized S.wiki as tentative **search matches**, not semantic certainty.
4. Use scoped generation counter + current wiki revision/digest + device grant fence to suppress late replies/revoked data. If offline, show explicit unavailable; no broad polling or provider fallback.
5. Render max 3 compact match cards. Clicking a card changes focus while preserving one PWA composer, requiring exact page ID present in authorized S.wiki. No Host effects.
6. Unit tests: newest edit wins; COMPOSITION_START/END; no automatic navigation; blank input; no results; no raw typed content in diagnostics; link click; changed Wiki revision; auth revoke.

## Slice 2 — Meaningful topic bundles via one Rust fabric
1. On refreshed current main, find the actual working Host->native HQKG read bridge and current actor scope. If absent, report a precise missing seam before adding a *read-only* owner-authorized adapter.
2. The incumbent semantic interpreter maps a request to zero/one/several **canonical IDs** with grounded status and evidence. Do not build a second parser in the wiki.
3. Adapter calls existing query_scoped with exact authorized IDs, prefix scope, excluded sources, watermarks, token bound and item bound. Fail closed if the graph seed is unknown.
4. Return typed WikiLensProjection from already selected graph items/links; keep graph and wiki revision fences. Prefix-scope behavior alone is insufficient for arbitrarily fine-grained access: enforce the actual Host principal/project/item permissions before and after retrieval.
5. User profile selects Overview/Explain/Trace only for eligible source scope. Trace requires verified audience authorization, never a client-supplied boolean alone.
6. A verified claim can render deterministic canonical language template. If no safe prose realization exists, show useful abstract + labelled evidence instead of raw English-ish nonsense.
7. Preserve source authority distinctions: a journaled user statement is not a verified fact; applied receipt occurrence is not proof current capability health.

## Slice 3 — AutoWiki quality -> VLANG and command feedback
1. Attach qualitative diagnostics to current graph/page revision: MISSING_PROVENANCE, UNRESOLVED_REFERENT, TEMPORAL_CONFLICT, MISLEADING_ACTION_STATUS, BROKEN_LINK, UNREADABLE_TEXT, STALE_CONTEXT.
2. Route diagnostics to existing reviewed owner task proposal path. The wiki read model never mutates VLANG definitions, wiki pages, command catalog or source index automatically.
3. Display a read-only 'Related commands' affordance when the same canonical semantic operation has verified readiness; explicit Submit then uses incumbent Host Control plan/approval/receipt.
4. Add deterministic replay: graph revision + lens focus + audience + chosen context items reproduces section ordering and links. Any optional natural-language enhancement is a candidate with recorded claim/support IDs and reviewer acceptance.
5. Build native code and run full browser/phone proof only after separate integration authorization.

## Proposed envelope — illustrative contract, not a running API
Request from trusted Host source owner:
- schema: vivim.autowiki-lens-request/1
- callerScope / actor / deviceEpoch: **server-validated**, not inferred from client payload
- inputSequence / sourceEpoch / wikiRevision / wikiDigest / graphEpoch
- canonicalFocusId, navigationDepth, maxItems, maxEstimatedTokens, excludedSourceIds
- existing HQKG query_scoped output reference, or empty when no valid seed
Response:
- schema: vivim.autowiki-lens-projection/1
- outcome: matched | ambiguous | no_match | stale | refused
- selectedPageId / suggestionPageIds (authorized only)
- summary: humanTextCandidate with supportingClaimIds and explicit verification state; no claim -> no authoritative summary
- related: typed link IDs restricted by same actor scope
- evidenceRefs / coverage / omissions / staleCount / source stamps
- authority: none; executionAuthorized:false; effectPerformed:false
Never put raw draft, secrets, grant tokens, personally sensitive snippets or unsupported source paths into the envelope.

## Explicit integration ordering
A. Prove Slice 1 only through current S.wiki + native search, including device grant checks.
B. Establish canonical ID mapping and Host-backed query_scoped adapter with deterministic property tests.
C. Add long-form English templates and claim fidelity review.
D. Connect quality diagnostics to reviewed tracker candidate channel.
E. Run real Android PWA verification on unchanged existing URL, then update source disposition with real commit/test evidence.

## Known design collision
The current renderWiki replaces the entire wiki scroll container after a fetch, which can discard focus, scroll and expanded sections. A lens implementation must preserve stable focus/scroll keys or reconcile per-card nodes; never rebuild on each keystroke. The existing composerScope includes tab, session, grant and source revisions: the lens may span Talk/Wiki UI presentation but must never reuse a projection after grant/source scope changes.

## Today's evidence status
R002 stage docs and Rust focus policy exist only in vivim-labs; no ARBOR integration. Native Rust compile/test, Host API path integration, PWA e2e, Android and VLANG quality loop are UNRUN. No claims of measured performance.
