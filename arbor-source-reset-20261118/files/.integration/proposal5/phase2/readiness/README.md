# ARBOR Proposal 5 / Phase 2 — Labs Readiness and Placement (Round 1)

**Status:** SOURCE AUDIT COMPLETE, PRODUCT PROMOTION NOT STARTED.  
**Date:** 2026-11-12  
**ARBOR branch:** work/p5-phase2-readiness-docs-r001.  
**Labs branch:** work/p5-phase2-readiness-r001.

## Contents

- [Full 44-component placement matrix](PLACEMENT-MATRIX.md) — each selected Labs path, source maturity, target ARBOR package, owner, beta decision.
- [Machine-readable source-to-ARBOR snapshot](SOURCE-TO-ARBOR.json) — exact pinned Labs branch SHA, source file count, synthetic test claims distinguished from actual tests, destination and category.
- [Owner handoff and collision map](OWNER-HANDOFFS.md) — A through G responsibilities and safe ordering.
- [Labs source scan human report](https://github.com/owenservera/vivim-labs/blob/work/p5-phase2-readiness-r001/readiness/reports/FIRST-PASS-2026-11-12.md)
- [Labs machine snapshot](https://github.com/owenservera/vivim-labs/blob/work/p5-phase2-readiness-r001/readiness/reports/FIRST-PASS-2026-11-12.json)
- [Repeatable source scanner and tests](https://github.com/owenservera/vivim-labs/tree/work/p5-phase2-readiness-r001/readiness)
- [GitHub Actions test run attempt](https://github.com/owenservera/vivim-labs/actions/runs/43150144128) — STARTUP_FAILURE before jobs; NOT a test PASS.

## First-pass findings

| Group | Entries | Guidance |
| --- | ---: | --- |
| Easy migration | 8 | Contracts, fixture tests, pure observers and small read-only projections; prepare while W1 A–F are running |
| Core dev/migration | 10 | Needs real A–F product-owner interface and source auth; cannot promote before W1-G/W2-G |
| High-value pending dev | 9 | Predictive composer, R007, AutoWiki, Rock Core, app builder, advanced autonomy: post-beta unless independently source-proven and trivial |
| Retain / privacy review | 17 | Labs-local tooling, historical docs, personal or third-party corpora, PC model router and source archaeology; no default product copy |

**Source scan:** 44/44 paths found at pinned Git commits. Labs main 885 tracked blobs; Node 0 branch 902; UI branch 888. These are per-branch counts, not unique addition counts. Relevant test files were inventoried, but new unit tests and product runtime verification have **not** executed in this first pass.

**Fresh CI evidence:** the new Labs GitHub Actions job triggered twice and both startup attempts failed before executing jobs. The specific reason was not exposed; no actual Node suite PASS is asserted. Keep original donor-reported test counts marked historical until a runner is available.

## Constraints to protect the beta

- Nothing in this readiness branch may modify concurrent ARBOR W1 A–F trees or merge into the shared integration branch without serial G review.
- Import only the minimal, source-tested dependency closure of approved Labs components. No Labs checkout, service, runtime or DB may remain an ARBOR dependency after cutover.
- Preserve exactly one Host effects/World authority, HQKG Rust graph implementation, TierFlow UI publisher, paired PWA, signed-in Chrome sender and status vocabulary.
- Do not misrepresent prototypes: voice workbench exists but not real ASR; T=0 archive import exists but not real Host evidence/graph commit; R005 prototype exists but R007 remains mostly design; app builder implementation not located in inspected Labs paths.
- Existing beta reliability gates (Tauri/Host security, no-donor run, crash-safe journal, physical Windows/phone) outrank optional Labs promotion.
- No personal corpora or recovered source dumps enter Git/product without explicit owner review.

## When to promote

**After W1-G GO**: reconcile actual ARBOR import tree and F/E/D work against the readiness manifest; deduplicate.
**After W2-G GO**: assign limited C-category adapters to B/A/C/D/E, with exact type, actor/source and regression tests.
**After W3 / beta acceptance**: consider H-category development independently, retaining same capability contracts.

The fastest result is often **promote a test, not a feature**. This round makes that choice traceable.

**Current release verdict:** PREPARED_NOT_PROMOTED. No Windows/phone/product cutover is implied.
