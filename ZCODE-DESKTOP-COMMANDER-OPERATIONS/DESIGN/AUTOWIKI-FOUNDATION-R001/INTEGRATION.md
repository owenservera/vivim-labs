# Implementation disposition and gates — OW-20261121-017
Source baseline: Labs Architecture/09-UNIFIED-INTELLIGENCE-COMMAND-WIKI.md and 2026-11-21 ZCode SITREP/Owner NOW. These are architecture and reported snapshots, not live verification of current ARBOR HEAD.

## Candidate reuse (verify at latest main before edit)
- HQ indexer/KB/graph: hq/lib/indexer.mjs, kb.mjs, graph.mjs — source/evidence intake; identify whether current ARBOR import or Rust equivalent owns each behavior.
- HQ semantic IDs: hq/lib/semantic-registry.mjs, nomenclature.mjs — identity and terminology; **do not** add a second registry.
- HQ action-registry.mjs, action-router.mjs — command discovery and NO_MATCH routing; distinguish declared from wired.
- HQ wiki.mjs, wiki-program.mjs, comment-wiki-bridge.mjs — existing deterministic materialization, configuration preview/activation, bridge; retain existing authority.
- Rust HQKG importer/core: reuse existing node/edge IDs and graph revision.
- ARBOR PWA wiki search-as-you-type: already reported for OW-007; use as the first *read-only consumer*, not an independent content engine.
- Existing LiveSystemState: subscribe to accepted revision/projection changes, not raw DOM as truth.

## Ordered integration slice
0. Refresh canonical ARBOR main, inspect real source bodies/call-sites/tests and running PWA file graph. Record exact SHA, verified vs candidate map, repo status; do not change other lanes' dirty files.
1. Normalize 7 hub page IDs to existing canonical semantic IDs. If no equivalent exists, create only an alias/projection mapping with explicit migration plan.
2. Add typed Claim->Section projection in existing Rust wiki/HQKG location. Reference [src/lib.rs](src/lib.rs) as contract guidance; DO NOT insert a parallel engine.
3. Render human summaries and evidence links from verified claims; where prose cannot be justified, emit a useful 'Not yet established' explanation. Keep semantic and trace views developer-gated.
4. Connect current PWA wiki read model and search input to these revisioned pages. Typing may preview; no write/effect from type events.
5. Send structural/semantic/readability failures into existing typed diagnostics/owner tracker candidate channel; no new background agent or privilege.
6. Implement commands only through existing canonical command/Host authority. Validate UI/phrase/CLI refer to same typed operation.
7. Verify browser/phone links, provenance, redaction, revision updates, no send requirement, and command refusal.

## Explicit gaps to verify
Current ARBOR live wiki renderer path and Rust HQKG projection APIs; source claim format; page revision transport; PWA live typing subscriptions; existing language-realizer; command registry integration; visibility/consent; no-parallel-writer proof. Never claim these are solved from folder inventory.

## Gate ledger
G0 source/facts audit: UNRUN. G1 Rust candidate tests: UNRUN. G2 ARBOR unit integration: UNRUN. G3 PWA read-only render and typing: UNRUN. G4 Android unchanged URL: UNRUN. G5 command preview/approval and redaction: UNRUN.
Integration is explicitly not authorized by this preparation package.
