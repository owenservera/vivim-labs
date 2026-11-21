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

## R002: dynamic context lens (user journey / progressive disclosure)
- [Dynamic Lens R002](DYNAMIC-LENS-R002.md): ambient suggestions vs selected/pinned page, three disclosure depths, source-epoch fences, read-only context bundles, privacy rules.
- [Exact source crosswalk](SOURCE-CROSSWALK-R002.md): inspected ARBOR source paths and reuse/gaps, not a runtime guarantee.
- [UX and trust acceptance](ACCEPTANCE-R002.md): 15 user scenarios, source/authorization/IME stress checks and test gates.
- [Human copy and presentation](COPY-AND-PRESENTATION-R002.md): example intros, intent-to-card taxonomy and honest fallback language.
- [Wiring sprint](WIRING-SPRINT-R002.md): three concrete slices, exact insertion points, existing scope fences and proposed transport envelope.
- [Rust lens](src/context_lens.rs): staged pure state/revision/authorization policy with unit tests; it never parses text or calls Host/HQKG.
No production change, GitHub Actions run, phone verification or compile proof is implied by these files.

## R003: full compile-ready pre-integration vertical slice
- [R003 execution handoff](R003-EXECUTION-HANDOFF.md): six-lane owner queue, exact ARBOR module locations, dependency guard, phase gates and commands.
- [HQKG adapter](integration/hqkg_context_bridge.rs): exact conversion of the existing Rust query_scoped result into a scope/revision-fenced wiki projection; place in native/hqkg only after feature-gated core merge.
- [Context projection](src/bundle_projection.rs): privacy-fenced, permission-filtered graph cards and exact linked pages.
- [Owned context](src/owned_context.rs): practical adapter for native HQKG data without another graph walk.
- [English realizer](src/narrative.rs): human-readable, claim-backed sentences with negation/freshness/claim trace checks.
- [Quality feedback](src/quality_feedback.rs): inert reviewed VLANG/AutoWiki diagnostics, no self-modifying compiler.
- [Cross-actor acceptance](tests/preintegration_acceptance.rs): one read-only projection for approved user and agent input.
- [Rust module root](src/lib.rs): exports all five sibling modules and preserves earlier R001 contracts.

**Verification truth (R003):** 37 Rust test cases are present (static count); source/static imports and brace counts were inspected. No Rust compiler or cargo exists in this cloud container; compilation, rustfmt, clippy, test execution, direct integration, live API/Android evidence are **UNRUN**. Do not label these modules compiled or deployed. A static source check is not a behavioral pass.

**Coexistence:** Dev ChatGPT owns active ARBOR PWA typing/AutoWiki and authorized agent typing. The product tracker now describes a separate first-contact live search-as-you-type path, even though the general Wiki tab's Search/Enter UI also exists. R002 documentation describing search only on Enter referred to that **general Wiki tab**, not the complete first-contact implementation. This R003 package deliberately does not change either one.
