# AutoWiki Foundation R001 — pre-integration only
Owner request: OW-20261121-017 (proposed new milestone; do not assume registry entry is already appended).
Status: DESIGN + STAGED REFERENCE CODE. Product integration, PWA, Android verification: UNRUN.
Boundary: ChatGPT authors in cloud -> GitHub main. Desktop Commander is read-only local context / authorized runtime verification. No local ZCode source edits.

## Goal
A small, coherent Wikipedia-like home and its top-level pages, with verified cross-links, readable prose, and drill-down into typed facts. A raw VLANG/graph projection is *not* user-facing prose.

## Read order
1. [PAGE-TAXONOMY](PAGE-TAXONOMY.md): concrete first pages and navigation
2. [CONTRACT](CONTRACT.md): graph fact -> semantic claim -> explanatory prose; quality/command feedback
3. [INTEGRATION](INTEGRATION.md): existing-source disposition, precise integration slice and gates
4. [Rust pre-code](src/lib.rs): dependency-free candidate projection/quality checker; not connected to ARBOR

The wiki is a materialized view over one graph, not a new database, content authority, router, permission subsystem, or compiler. Do not imply a capability runs because it has a page. Each page must show provenance and freshness. Unknown facts must remain unknown; no fabricated causal summaries.
