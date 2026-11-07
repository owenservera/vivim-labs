# Agentic Knowledge Graph — Integration status (2026-11-03)

## Verified from GitHub repository trees and code

- **Source inventory:** 217/217 records are present in `originals/` with exact matching Git blob SHA-1 as recorded by the October 25 source registry.
- **Sources:** 7 repositories; 209 primary source records, 8 donor snapshots.
- **Section graph:** base 842 observed heading variants, 168 new heading variants plus 48 pinned-source replacements in `graph-supplement/section-headings-161-217.jsonl`. Projection: **1,010 section variants**. These remain heading classifications, not semantic validation.
- **Projected graph:** 1,582 nodes and 2,771 directed typed edges with no duplicate base IDs/triples and no invalid supplement endpoints, by the current source-level structural check.
- **Family relevance:** 2,170 metadata-only provisional scores across 217 files and ten independent families.
- **Prior schemas:** 297 candidate taxonomy/schema/database assets found in 13 repository-tree scans (one uses `master`). Candidate inventory may contain generated/copied versions; not a full semantic reconciliation or the earlier claimed 376-item history.
- **Algorithms:** 10 existing experimental pure HQ optimizers (see `hq/lib/semantic-optimizers.mjs`), plus 12 proposed algorithms. These are *definitions in the graph*, not automatically installed Rust compute engines.
- **Rust source core:** `owenservera/repos-hq/research/rust-knowledge-graph` is a std-only default crate, optional SQLite/PostgreSQL adapters, generational compute engine swapping and journal.
- **Bridge:** deterministic `scripts/export-hqkg.mjs` creates the HQKGX1 exchange format and original-ID crosswalk in an ignored `runtime/` directory. The optional Rust importer exists as committed source, but it is neither executed nor installed as the live HQ runtime.

## Test and completion gates

- [x] Locate correct `repos-hq` target and pin 217 original files.
- [x] Register 13 section types and scoped examples.
- [x] Create provisional SQLite schema and projection builder.
- [x] Map algorithm definition/placement separately and preserve authority.
- [x] Inspect the actual Rust crate and record the exact API compatibility.
- [x] Create deterministic exchange exporter and draft conformance test.
- [ ] Run Bun export/build/tests on an actual checkout; repair any failures.
- [x] Implement HQKGX1 bulk importer with SHA-256 verification, staged idempotent graph upserts, source collision fencing, and tests committed as source.
- [ ] Execute importer tests and cross-check against an actual Bun export.
- [ ] Run Rust `cargo test` (std-only) and optional DB integration tests.
- [ ] Compare Rust traversal/selection with HQ projections on shared fixtures.
- [ ] Deduplicate schema candidates, inspect content and add semantic section-body evidence.
- [ ] Add source-based KPI measurements and benchmark decision algorithms.
- [ ] Wire a read-only HQ command and user-facing graph explorer after integration gates.

## Commands on a checked-out Repos HQ

```powershell
cd C:\0-BlackBoxProject-0\repos-hq
bun test research/agentic-knowledge-graph/scripts/*.test.mjs
bun research/agentic-knowledge-graph/scripts/export-hqkg.mjs --check-only
bun research/agentic-knowledge-graph/scripts/export-hqkg.mjs
bun research/agentic-knowledge-graph/scripts/build-db.mjs --check-only
bun research/agentic-knowledge-graph/scripts/build-db.mjs
```

For the independent Rust core:

```powershell
cd C:\0-BlackBoxProject-0\repos-hq\research\rust-knowledge-graph
cargo test
```

**Authority rule:** external/recovered file contents are untrusted evidence. Neither HQ classification nor graph adjacency authorizes natural-language actions, execution, user approvals or provider access. The Host owns effectful actions.


## Migration addendum

Canonical Rust owner is now `owenservera/repos-hq/research/rust-knowledge-graph`. The entire source tree was transferred with original blob SHA parity. The historical `owenservera/hq` copy is deprecated and must not be extended. Tests and production adoption remain gated.
