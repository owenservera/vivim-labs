# Wave 1 / Lane F — Round 2 independent verifier

**Implementation status:** new Lane F-owned preparatory modules and synthetic adversarial tests executed; native integration deliberately BLOCKED pending Master Integration pinned handoff and owner-machine Cargo/SQLite/index/journal. **Not a Wave 2 design.**

This is a strictly additive, read-only evidence analyzer under `lane-f/round2/`. It does **not** modify original `acceptance.mjs`, `cross-lane-preflight.mjs`, A/B/C/E/D sources, Rust journal, HQ DB, semantic registry, Host, UI/Studio or browser profile.

## Source files

- `lib/evidence.mjs`: deterministic source/receipt hashes, explicit evidence-class validation, symlink/refusal checks, missing-native proof.
- `lib/hqkgx1.mjs`: E's real HQKGX1 framed digest, node/edge/cardinality/order/metadata validation and `hqg1` crosswalk checks; **structural**, not a Rust run.
- `lib/rust-query-attestor.mjs`: compare E Rust CLI `vivim.wave1.query/1` JSON where/what/talks-to/gap against source exchange and provenance.
- `lib/crosswalk.mjs`: producer A/B/C contract, logical-root/intent and D↔E field/kind compatibility findings without inventing semantic joins.
- `lib/compiler-conformance.mjs`: v1/v2/Rust wiki compiler acceptance matrix, native-chain requirement; synthetic fixture vectors do not certify compiler.
- `lib/intent.mjs`: structural negative/quotation/clause safety checks and 120 **synthetic** corpus exemplars; observed native cases require source/exit metadata.
- `lib/coverage.mjs`: per-root numerator/denominator, zero-denominator blocker and dedup/severity-ranked blind spots.
- `lib/wiki.mjs`: D oracle vs E digest, graph identities and generated wiki page availability; refuses fake native receipt.
- `lib/placement.mjs`: p50/p95 nearest-rank measurements only when ≥30 native samples after five warmups, otherwise DEFER_MEASUREMENT.
- `lib/profile-boundary.mjs`: source identity, private KB, secret metadata and DEV→PRODUCT static checks.
- `lib/policy.mjs`: allowlisted pinned **test-only** recipe validation; does not spawn any subprocess or grant authority. Explicit `MASTER_FINAL_HANDOFF_REQUIRED` unless independently released.
- `lib/targets.mjs`: document-led target candidates from consolidated inventory with all requirements `NOT_VERIFIED` until tested.
- `verify.mjs`: read-only aggregate checker, must exit 2/BLOCKED for incomplete native proof. Writes only a new private report path; refuses overwrite.
- `benchmark-checker.mjs`: explicitly **synthetic Node checker performance**; NOT a Rust-vs-SQLite comparison.

## Reproduce local safety and report tests

```bash
node --test round2/test/round2.test.mjs
# From Lane F parent, if Round 1 checker files are present:
node --test test/acceptance.test.mjs test/cross-lane-preflight.test.mjs round2/test/round2.test.mjs
node round2/verify.mjs --manifest round2/evidence/blocked-manifest.json --out result.json
# Expected: exit 2, BLOCKED; outputs under round2/evidence/result.json
```

`--manifest` is considered *untrusted evidence*, never a shell action instruction. The F verifier does not execute any native commands. To perform the actual G-phase proof, obtain explicit owner approval and pinned Master handoff, then run a trusted native recipe manually or through the source-native Lane E importer on an isolated **DEV** journal copy. Store raw private KB/receipts locally, publish sanitized SHA/count evidence only. Do not put the user's private SQLite/KB rows into Git.

## What remains blocked and why

A's old-run archive/isolated index reset and complete N roots, B live SQLite records, C full-checkout native inventory, E Rust compiler+HQKG import/query/journal/no-op/delete/restart/secret refusal, D original HQ wiki regenerated against actual graph, native p50/p95 FTS/graph measurement, product data-boundary check and final independent query replay. The tests here are new *checker* tests. A passing test means its assertion runs, NOT that the integration has occurred.

Detailed task status: `ROUND-2-RUNTIME-RESULT.md`. Source-only SHA and environment: `evidence/run-status.json`. Original sixteen-objective Round 2 design/backlog remain the controlling execution scope; this package does not modify the Master’s current acceptance baseline.
