# Rescore Expansion — Rust migration midstream SITREP

**Observed:** 12 November 2026 · **main SHA:** `3d45fbf1c3963e4834a5e51ee7f6e63f3a0da298` · **1,000-assessment pinned baseline:** `98f15e327957956ec94189772d5e5813e7b6b07b`.  
**Status:** **DESIGN SOURCE-MAPPED; RUST MIGRATION NOT EXECUTED**. New human and machine inventories are on `work/p5-rescore-1000-r001`; this SITREP narrows the adjacent integration programs to their consequences for Rust migration.

## ERA 1 — What is implemented versus what is only proposed

The repository already has two relevant **native Rust codebases**:

- `native/hqkg/src/lib.rs` exports graph primitives, algorithms, typed DAG, engine registry, scoped graph, invalidation, reconciliation, context retrieval, journaling and optional adapters. This is **real Rust source**, but it is not evidence that all HQ, Studio, Host and TierFlow callers are already wired into one accepted product runtime.
- `core/programmability/src/lib.rs` supplies typed **inert** transform-program validation with policy/source metadata. It explicitly has **no network, storage, timers, grants, Host effects or scheduler**, so command/IR expansion must reuse it without promoting it into an action executor.

The **new 1,000-entry inventory is not Rust implementation**: **100** previous curated non-Rust module assessments + **626** further tracked source-module loci + **274** functions literally found in source code. There is deliberate module/function overlap; neither 1,000 implementations nor 1,000 performance gains have been shown. All 1,000 JSON IDs/ranks and Markdown rows were checked; source blob IDs and declaration-receipt joins passed structural validation. **No native builds, executable tests, profiler traces or real donor-repository comparisons were run.** The release manifest remains `BLOCKED_NOT_BUILT`.

### Midstream Proposal 5 integration dependencies

| Program | Verified GitHub state | Consequence for Rust expansion |
|---|---|---|
| [Optimization Observatory PR #20](https://github.com/owenservera/ARBOR/pull/20) | **Merged main**, commit `3d45fbf1c3963e4834a5e51ee7f6e63f3a0da298`; 59 research opportunities | Common profiling lens: CPU, RSS, disk, power, cadence, maintainability |
| [HVI G1 PR #21](https://github.com/owenservera/ARBOR/pull/21) | **OPEN DRAFT**; title says W2 owner waiver and runtime tests **UNRUN** | Its version compare + agent-discoverability path is **not** accepted native execution, and real repo comparisons remain forbidden |
| [SCF + Verifier PR #13](https://github.com/owenservera/ARBOR/pull/13) | **OPEN DRAFT** | Potential future source-semantic/provenance contracts, not a current Rust ABI |
| [Semantic Engineering Program PR #19](https://github.com/owenservera/ARBOR/pull/19) | **OPEN DRAFT** | A proposed verification/research system, not live native runtime |
| [VLang program SITREP PR #22](https://github.com/owenservera/ARBOR/pull/22) | **OPEN DRAFT** | Useful language direction; do not infer compiler or VM accepted |
| Rescore Expansion 1,000 | Documentation on isolated branch, **not yet on main at observation** | Ranked Rust inspection queue, not migration work order |

## ERA 2 — Highest-value native engine candidates

This is a **crosswalk priority**, not a claim about missing Rust functionality or benchmark-measured CPU load. Keep **execution cost × frequency** distinct from **peak RAM** and **idle duty cycle**.

| Order | Engine family | Current non-Rust sources | Expected performance leverage | What Rust may own |
|---:|---|---|---|---|
| 1 | Streaming evidence decoding and deterministic replay | `studio/src/decode/decoders.mjs`, `studio/src/decode/protobuf.mjs`, `studio/src/replay/reducer.mjs` | Potential per-message CPU, intermediate allocation and stream-buffer pressure | Bounded decode, event fold, delta reconstruction; browser capture stays Studio |
| 2 | Knowledge entity matching and temporal truth | `studio/src/intel/join.mjs`, `studio/src/intel/similarity.mjs`, `studio/src/intel/temporal-truth.mjs` | Potential quadratic candidate matching, graph joins and high peak RSS | Reusable scoped graph algorithms; never auto-promote personal identity facts |
| 3 | Incremental source index, graph-delta and import | `hq/lib/indexer.mjs`, `hq/lib/graph.mjs`, `hq/lib/kg-export.mjs` | Dev/admin source refresh and repeated graph rebuilds | Pure content change-set and graph compute via existing `native/hqkg`; HQ remains index writer |
| 4 | Context retrieval / DCB bounded planning | `hq/lib/context.mjs`, `harness/src/context/dcb-composer.mjs`, `harness/src/context/dcb-source-selection.mjs` | Repeated user chat context selection, heap buffering and budget pressure | Bounded ranking, packing, invalidation; Host owns disclosure decisions |
| 5 | Deterministic command and visual-IR checks | `apps/mobile-relay/nlcl/interpret.mjs`, `packages/tierflow/src/visual-runtime/command/compile.ts` | Cross-device parity, low-latency hot path, shared grammar | Extend `core/programmability` pure typed validation; TierFlow remains publisher |

### Sources not to move just to satisfy a Rust target count

**Host** retains grants, effects, one-writer journals, World original store and receipts; **Studio** retains browser/CDP and provider sends; **HQ** keeps source selection, existing development index transactions and Wiki publication; **TierFlow** retains DOM renderer/revision publisher; **mobile PWA** retains browser transport and UI. A Rust helper can compute inert results for each owner, but cannot silently take its authority.

The Rescore 1,000 classifications include `HIGH_FREQUENCY`, `FREQUENT`, `USER_DEFINED` high/medium/low, `MEDIUM_FREQUENCY`, `LOW_FREQUENCY`, `DEV_ADMIN` high/medium/low, and an explicit `always_on_candidate` flag. **Zero paths are claimed verified always-on** in the source-only evidence. Always-on candidates require an actual scheduler/caller and process duty-cycle trace.

## ERA 3 — Rust migration roadmap and gating

1. **Crosswalk and deduplicate:** For the first 50–100 top-ranked assessment rows, map each pure function to existing `native/hqkg` and `core/programmability` APIs; status **ALREADY_NATIVE / PARTIALLY_NATIVE / MISSING_PURE_KERNEL / KEEP_JS / UNKNOWN**. Do **not** treat the 1,000 source records as 1,000 missing engines.
2. **Fix vertical source closure:** verify the HQ → Rust native import adapter and its crate path, Cargo/Bun reproducibility, and authenticated domain namespace boundaries. Current main is **not** an accepted unified KB runtime.
3. **Profile real costs safely:** generated synthetic corpora, active high-frequency streams, per-turn user commands, rare multi-GB-equivalent batch tests (within hardware limits), dev/admin indexing and actual idle process power. Measure p50/p95/p99, CPU seconds per minute, peak RSS, memory-duration, disk write amplification, binary/target size and fallback.
4. **Choose 3–5 native proofs**, not 1,000 ports. Use typed ABI, deterministic output parity against existing JS, cancellation and bounded allocation; prove cross-surface fairness, recovery and revoke semantics where applicable.
5. **Preserve gates:** HVI's native comparison tooling only uses synthetic invented Git fixtures, never real donor comparisons before explicit authorization. No unaccepted SCF/VLang draft is a dependency for this read-only assessment. Product runtime/device/security acceptance stays independently blocked.

**Machine-readable version:** [MIDSTREAM-INTEGRATION-SITREP.json](./MIDSTREAM-INTEGRATION-SITREP.json).  
**1000 ranked humans:** [1000-COMPUTATIONAL-TARGETS.md](./1000-COMPUTATIONAL-TARGETS.md).  
**1000 machine records:** [1000-COMPUTATIONAL-TARGETS.json](./1000-COMPUTATIONAL-TARGETS.json).

**Gate decision:** `GO_FOR_RUST_CROSSWALK_AND_BENCHMARK_DESIGN` / `NO_GO_FOR_BLIND_MIGRATION` / `PRODUCT_RELEASE_BLOCKED`.
