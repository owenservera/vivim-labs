# R002 real-source crosswalk — inspected GitHub main 2026-11-21
Snapshot seen in GitHub search URLs: ARBOR 671a82f3f8348323000aac408d2ae983762f7497. Verify fresh HEAD before any actual integration. Reading code is not end-to-end device proof.

| Exact source path | Observed mechanism | R002 disposition | Gap / verification |
|---|---|---|---|
| native/hqkg/src/context_retrieval.rs | retrieve/query_scoped: bounded BFS, ordered seeds, scope prefix, exclusion, token/max item caps, epoch/watermark and provenance | REUSE native graph retrieval as sole graph walk | Need existing caller bridge and authorized seed mapping from page ID; test scope fencing |
| core/programmability/src/wiki_read.rs | bounded Rust wiki read, links, search index, revision digest and source_transition | EXTEND existing read projection rather than new wiki source | First inspect actual output adapter and source rev/digest route |
| apps/mobile-relay/app.js | wiki tab, Host-authorized read, search UI, source context button and refresh | WIRE thin presentation/interaction adapter | Existing main wiki tab is not automatically intent-following; check old rendered DOM retention |
| apps/mobile-relay/first-contact-composer.ts | human input, compositionstart/end, monotonic local revision, Host observation fence | OBSERVE existing source; do not add separate textarea or synthetic typing loop | Read-only intent subscription hook required; never consume incoming agent draft as user text |
| apps/mobile-relay/composer-assist.mjs | debounced latest-input-wins scoped interpretation, suggestions, no-send, 2-TAB behavior | REUSE read-only interpretation event; keep auto-wiki separate from command execution | Need stable typed concept IDs from existing outcome, not text-to-command second router |
| apps/vivim-host/public/pages/wiki.js | source-backed Host wiki console, patch-on-revision, source cards, verified context, page links | KEEP desktop wiki authority; consume projection/diagnostics | Avoid silently changing editor or source-refresh behavior |
| apps/vivim-host/chat/wiki-semantics.mjs | wiki:page/section IDs, provenance classification, typed target resolution, structural links | REUSE canonical IDs/claim classifications | Link semantic IDs to HQKG without inventing parallel registry |
| apps/vivim-host/control/command-context-grounding.mjs + core/programmability/src/context_grounding.rs | pinned source-reference resolution with explicit ambiguity and stale refusals | REUSE for exact 'that/this' command references only | A wiki preview is not sufficient command authorization |
| harness/src/context/dcb-composer.mjs | scoped deterministic dynamic context bundles, profile/layers, source digest and sensitivity rules | REUSE source policy concepts; no generic raw copy into phone | Current profile/task/sensitivity semantics differ from live wiki lens; add adapter only |
| apps/mobile-relay/wiki-source-view.mjs | digest/anchor validation and verified source-context link; stale/conflict detection | REUSE verified read-detail interaction | Avoid granting raw source previews beyond current audience |
| core/programmability/src/live_system.rs | live system revision and region boundaries | CONSUME existing Rust live observation/projection | Do not treat typing event as authoritative graph fact |
| hq/lib/wiki.mjs, hq/lib/context.mjs, hq/lib/semantic-registry.mjs | historical HQ wiki, context composer and semantic identity surfaces present | MIGRATION INTEL only until verified parity | No parallel JS implementation of new language or graph logic |

## Exact first code adjacency
1. Rust: current wiki_read.rs + existing HQKG query_scoped adapter; derive bounded WikiLens read-only result from scoped IDs and graph revisions.
2. Host: existing Wiki/context read owner sends validated source/actor scope, epoch, revision; returns only approved page snippets and evidence IDs.
3. PWA: subscribe to existing interpretation result and attach ambient lens renderer to current Wiki tab/panel; keep source/read routes and composer behavior unchanged.
4. Diagnostics: use existing owner tracker candidate path, never a self-mutating VLANG compiler loop.
5. Tests: unit native transitions, cross-boundary freshness/privacy, phone live typing/IME, link navigation and no-effects.

## Specific unknowns (do not infer)
- Whether HQKG graph retrieval is called by the currently running phone Wiki tab.
- Whether live semantic identity is fully present in composer interpretation envelopes.
- Whether fresh rendered human paragraphs exist outside user-authored Wiki sections.
- Whether Host already has one reusable endpoint for revisioned lens projections.
- Whether 'wiki.home' etc are aliases to existing persistent canonical Wiki page IDs.
- Whether the native Wiki graph and Host journal share exact authorization epoch and identity.
- Runtime e2e on Android, build, performance and integration HEAD are UNRUN.
