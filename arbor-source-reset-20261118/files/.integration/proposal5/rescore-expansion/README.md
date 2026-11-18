# Rescore expansion — ARBOR Rust core computational opportunity map

**State:** DISCOVERY / FIRST-PASS DESIGN ONLY — no implementation, port approval, runtime benchmark, migration comparison, or beta acceptance.  
**Baseline:** `ARBOR main@539146ac741516e7d9711c63f4570585b3c3367e` (observed 2026-11-12). **Owners:** Rust/HQKG native computation; HQ for development data; Host for personal World and effects; Studio for signed-in browser; TierFlow for visual publication.

## What we are trying to do

Build a source-grounded, **performance-first map** of the non-Rust processing already present inside ARBOR, then decide which high-value **engines, transformations, data streams and algorithms** should become reusable native Rust computation. The goal is *faster, more consistent, more deterministic and resource-bounded VIVIM* across HQ, Host, Studio, desktop and paired PWA—not moving code into Rust simply because it is written in JavaScript or TypeScript.

Start with HQ because it offers reliable source fixtures and measurable indexing, graph, KB, context and wiki dataflows. Expand into Studio's high-volume decoding/replay/corpus analysis, Host's authorized personal evidence pipeline, Harness DCB planning, mobile NLCL and TierFlow's pure command/revision computation.

## Outputs

- [**100 ranked candidates**](./100-COMPUTATIONAL-TARGETS.md): exact ARBOR source paths with workload, frequency, reuse, native portability and proposed computation boundary.
- Priority is **estimated**, not measured. It is a discovery backlog, not a claim that components are missing, broken, untested in isolation or duplicative.
- The first design task after this folder is a **source-to-native parity crosswalk**: determine whether each algorithm is already in `native/hqkg/` or `core/programmability/`, partially represented, or actually absent.
- The following design task is a profiling/benchmark protocol; do not pre-approve a rewrite from this ranking alone.

## Top opportunity families (not atomic design)

1. **HQ source → graph knowledge acceleration:** incremental parsing, indexing, import graph, versioned snapshots, dependency/impact, context ranking, AutoWiki.
2. **Large captured evidence:** streamed messages, protobuf/CSV/delta, replay, identity resolution, timeline/temporal truth, multi-ledger join.
3. **Living personal KB:** Host-authorized ingest evidence, conflicts, temporal changes, domain isolation, revocation-aware derivation.
4. **DCB as shared pure computation:** context selection, budgets, graph lineage, policy *candidates*, digest/invalidation; Host owns disclosure.
5. **Command/visual cross-surface semantics:** inert NLCL/IR compilation, schema checks and deterministic state/revision transforms for desktop and PWA.
6. **Operational robustness:** diagnostics, recovery candidate scoring and bounded archive/index integrity verification.

## Non-goals and strict architectural constraints

- **Do not** migrate Host authority, user-consent decisions, personal World writer, browser sender/CDP automation, disk/network transport, UI renderer or effect journals into an autonomous Rust executor.
- **Do not** create a second graph, scheduler, native database authority, identity registry or plugin engine. Reuse `native/hqkg` and `core/programmability` with typed adapters and Host-owned policy.
- **Do not** treat source presence as runtime wiring: HQ's imported JS refresh still references its legacy donor crate path, while the ARBOR product HQ service uses JS graph/context/wiki and is not fully mounted through Host/native IPC. Repair and test the bridge before native promotion.
- **Do not** perform a real repo migration comparison. This is an ARBOR-only source inventory, not the HVI migration run.
- Research-only algorithms in Labs or former HQ remain candidates until imported, benchmarked and accepted; they are not assumed to be ARBOR implementations.
- Keep PWA-first and small-disk constraints. Shipping another 100 native engines would be a failure of focus: select shared primitives with demonstrated wins.

## Readiness to advance

1. Build reproducible Cargo/Node/Bun source closure and capture real test exits (current beta remains blocked).
2. Inspect 100 crosswalk entries for native reuse and owner boundaries, **without** editing product source.
3. Profile highest-ranked workloads with generated synthetic corpora; collect p50/p95, CPU, memory, disk, startup and power where available.
4. Choose **at most 3–5** initial shared engines for an independently reviewed proof, keeping JS baseline and rollback.
5. Complete HQ native parity, then use the same Rust interfaces for personal authorized knowledge and UI projections only after proof.

## Source pointers

- `native/hqkg/src/lib.rs`, `native/hqkg/src/engine.rs`, `native/hqkg/src/typed_dag.rs`, `core/programmability/src/lib.rs`
- `hq/lib/arbor-service.mjs`, `hq/lib/indexer.mjs`, `hq/lib/graph.mjs`, `hq/lib/context.mjs`, `hq/lib/wiki.mjs`
- `apps/vivim-host/bootstrap/runtime.mjs`, `studio/src/intel/`, `harness/src/context/`, `packages/tierflow/src/visual-runtime/`
- `.integration/proposal5/02-RUST-CORE-AND-HQ-WIRING.md` and `.integration/proposal5/hq-version-intelligence/`

**No test commands executed.** GitHub tree/path verification is API source evidence, not an OS process exit or measured runtime speedup.

## Cadence classification added

The [100-target matrix](./100-COMPUTATIONAL-TARGETS.md) now distinguishes `ALWAYS_ON`, `HIGH_FREQUENCY`, `FREQUENT`, `USER_DEFINED`, `MEDIUM_FREQUENCY`, `LOW_FREQUENCY`, and `DEV_ADMIN`. User-driven H/M/L and development/admin H/M/L are **independent workload-intensity hypotheses**, not throughput measurements. This prevents a sporadic but CPU-heavy batch operation being mistaken for 24/7 background load, or an active-chat hot path being interpreted as continuous work. No always-on workload was established by this inventory. Counted bands are ALWAYS_ON: 0, HIGH_FREQUENCY: 14, FREQUENT: 11, USER_DEFINED: 36, MEDIUM_FREQUENCY: 6, LOW_FREQUENCY: 3, DEV_ADMIN: 30.


## Rescore 1,000 expansion — current authoritative assessment

The expanded atlas is now documented in two complementary formats:

| Artifact | Format | Purpose |
|---|---|---|
| [1000-COMPUTATIONAL-TARGETS.md](./1000-COMPUTATIONAL-TARGETS.md) | Human-readable Markdown | Ranked, filterable-by-text list with estimated CPU, peak memory, run cadence, user/admin frequencies, native suitability and evidence tier |
| [1000-COMPUTATIONAL-TARGETS.json](./1000-COMPUTATIONAL-TARGETS.json) | Machine-readable JSON | Exactly 1,000 typed records including IDs, source paths, Git blob SHA, score derivation, cadence fields, boundaries and provenance |
| [MIDSTREAM-INTEGRATION-SITREP.md](./MIDSTREAM-INTEGRATION-SITREP.md) | Human-readable Markdown | ARBOR integration state, version pins, dependent workstreams, merge and verification gates |
| [MIDSTREAM-INTEGRATION-SITREP.json](./MIDSTREAM-INTEGRATION-SITREP.json) | Machine-readable JSON | Exact observed gate statuses, commit and PR pins and counts |
| [evidence/symbols-01.json](./evidence/symbols-01.json) through [symbols-06.json](./evidence/symbols-06.json) | Source-declaration receipts | Named function identifiers extracted from pinned source files; not runtime behavior proofs |

**Population:** 100 original curated source modules + 626 additional non-test, non-fixture source modules + 274 exact named declarations. These are **nested assessment opportunities**, not 1,000 proven separate engines or 1,000 justified Rust ports. The original [100-item matrix](./100-COMPUTATIONAL-TARGETS.md) remains available as the initial curated view.

**Caution:** All resource/cadence scores are estimates. No performance benchmark, native source parity run, test executable exit, or real repository-migration comparison was performed. Active HVI tooling, SCF, and semantic engineering drafts retain their own merge/readiness gates and cannot be promoted just because this atlas is complete.

**Review rule:** Select promising *pure computations* only after actual owner/caller analysis; keep browser, personal-data consent, Host ledger effects, TierFlow renderer and PWA network handling in their current authorities. Extend one shared `native/hqkg` / `core/programmability` implementation when benchmarking justifies it; do not create a second store or scheduler.
