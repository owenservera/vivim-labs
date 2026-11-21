# R002 — Context Lens: user-following AutoWiki
Status: pre-integration design only. This is a projection over the existing Rust graph, wiki read model and Host authority. No new graph, cache-of-record, agent, router, permissions, source writer or model call.

## Product model
AutoWiki is a persistent companion to Talk, not an intrusive search-results replacement. It consists of:
- **Canvas**: a stable selected page, starting with wiki.home (seven top-level hubs remain navigable).
- **Ambient rail**: at most three relevant suggestions from the current draft, updated as focus changes. Never forcibly navigates the canvas.
- **Focus lens**: clicking a suggested concept, wiki link, breadcrumb or explicit Open intent selects a page.
- **Depth switch**: Overview -> Explain -> Evidence/Trace. This is presentation depth, not the graph traversal-depth configuration.
- **Pinned focus**: pinned page remains stable while ambient rail follows new text; pin does not grant broader source access.
- **Breadcrumbs/history**: within-session navigation path with back/forward; must validate access and revision on revisit, not cache revealed data after revocation.
- **Source footer**: source revision, proof state, freshness, scope, omissions; visible details vary by authorization.
- **Inert command affordance**: Can I do this? shows registered operation, current readiness and permissions, not execution.

## User journey and expected affordances
| User does | Immediate view | After focused navigation |
|---|---|---|
| Opens PWA, no draft | Home with 6 hubs and last *verified* changes (if authorized) | Hub index |
| Starts 'What's VLANG...' | Ambient VLANG definition and 2-3 scoped links, clearly preview | VLANG concept, related SDK, Rust/command relation |
| Types 'why isn't wiki updating?' | Pinned/current canvas stays; suggested troubleshooting evidence / latest receipts / known gaps | Troubleshooting page with verified tests, source status and unknowns |
| Types 'show work on PWA' | Matched project/activity bundle, no guessed ownership | Milestones, relevant source anchors, verified statuses |
| Types 'can you edit X?' | Capability card: available/declared/blocked, approval explanation | Command details; submit routes through existing Host only |
| Clicks a link, then another | Depth increases by explicit choice, breadcrumb visible | More relevant evidence, relationships, tests and dependency references |
| Pins the wiki page and changes draft | Canvas does not jump; ambient suggestions still change | Pin ends only by explicit unpin/nav/revocation |
| Enters ambiguous or very short text | Preserve current canvas; no spurious page changes | Ask for topic selection; avoid false certainty |
| Scrolls deeply, then graph changes | Existing reading location stays; show freshness indicator and non-disruptive update | User can inspect new revision or refresh selected section |

## Focus state machine
- HOME: a default stable hub with ambient suggestions.
- SUGGESTING: draft produces eligible topic candidates; current canvas unchanged.
- FOCUSED: explicit selection/navigate changes canvas and records breadcrumb.
- PINNED: canvas fixed by user; only ambient rail and freshness state may change.
- REVOKED: active content and cached excerpts cleared; safe unavailable state.

A typing sequence carries a client-local monotonically increasing revision; interpretation answers are accepted only for the current session/epoch/revision. IME composition suppresses speculation until compositionend. The browser uses debounce and cancellation for efficiency, but the Rust read projector validates sequence and source revision so late responses cannot replace newer context. Clearing the draft clears ambient suggestions, not deliberate navigation.

## Dynamic bundles: reuse, not replacement
Existing native HQKG query_scoped performs breadth-limited graph retrieval under caller-authorized scope, ordered seeds, token and item budgets, source provenance, stale flags, and graph epoch. A thin **WikiContextLens projection** consumes its output and decides which *already allowed* sections/cards to render.

Page detail profiles (proposed targets, not measured latency):
- OVERVIEW: 8 items / 1200 estimated tokens; title, 1-2 paragraph explanation, 3 relevant links, quality/status.
- EXPLAIN: 20 items / 3500 estimated tokens; richer relationships, examples, tasks/decisions, why/where it fits.
- TRACE: 48 items / 7000 estimated tokens; claim IDs, source anchors, tests, revisions, raw VLANG and read-only command traces, restricted to eligible developer audience.

An empty or omitted bundle is not silently upgraded to a confident narrative. Show what is known, what could not be checked, and the narrow next step.

## Relevance order and continuity
Authoritative inputs, in priority: explicit page/link selection; pinned page; exact resolved semantic identity from incumbent typed parser; authorized project/activity focus; latest admissible source context; verified graph neighbors. Raw keyword guesses are eligible only as *search suggestions*, not typed resolution or permission. Ambiguity is represented as multiple candidates. Existing core retrieval performs BFS and evidence selection; don't reimplement it.

Relevance ties break by verified identity match, graph depth, source freshness, evidence class, stable ID. Introduce focus hysteresis at browser-presentation boundary: do not reorder visible suggestions on every character unless topic change is substantive; preserve card identity and scroll; show at most three concise previews. Never invent a numeric relevance confidence.

## Privacy / scope / authority
No draft text in analytics, lasting wiki pages or server diagnostics; use ephemeral session-scoped interpretation and bounded IDs. Host/source owner supplies authorized scope; authorization is checked before retrieving, before rendering, and again at link/open. A secret/refused node must not leak through count, title, source path, search snippets, backlinks or 'similar' recommendations. Device/session epoch and user-scope changes invalidate the entire lens. Nothing auto-sends, auto-edits, promotes a claim or runs a command.

## Semantic quality feedback
Capture a typed diagnostic per bad context card or page: page ID, focus revision, graph epoch, claim/ref IDs, failure family, severity, relevant stage (source acquisition, semantic registry/VLANG, context selection, English realization, navigation, command coverage). Propose a reviewed owner tracker entry; no autonomous code edits or silent source writes.
