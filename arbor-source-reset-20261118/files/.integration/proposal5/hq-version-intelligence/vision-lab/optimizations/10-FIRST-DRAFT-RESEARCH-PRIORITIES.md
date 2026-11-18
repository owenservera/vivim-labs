# 10 — First draft: where to direct optimization research
These are **investigation priorities**, not a sprint calendar, approved ports or a permission to change product source.

## First horizon — measure what could hurt the entire experience
**A. Disk truth and low-space recovery.** Measure full ownership footprint: HQ SQLite/WAL, TierFlow state.db, Rust journals, Studio raw logs/indexes, immutable profiles and Rust target/debug. Review `OPT-002`, `OPT-003`, `OPT-004`, `OPT-005`, `OPT-007`, `OPT-056`, `OPT-057`, `OPT-059`. Aim: byte-accurate breakdown and safe reserve; no silent cleanup.

**B. Continuous/background load.** Instrument *actual active* HQ watcher, Studio log flush and TierFlow frames (`OPT-030`, `OPT-035`, `OPT-036`). Collect idle wakes, energy, queue occupancy and blocked time; distinguish timer exists from daemon actually running. This is often the cheapest conceptual optimization: don't do unnecessary work.

**C. High-frequency stream transformation.** Compare Rust sidecar vs JS fallback, Studio protobuf/delta and web-memory parse (`OPT-022`, `OPT-025`, `OPT-028`, `OPT-038`, `OPT-039`). Only pure synthetic stream fixtures; do not bypass redaction, gap receipts or durability.

**D. Rust incremental DAG and graph context.** Use the existing ignored benchmark harnesses (`OPT-013`, `OPT-016`) with bounded synthetic graph sizes; measure cache hit/replay and RSS, not only raw CPU.

**E. UI verification and compiler latency.** Measure `OPT-043`, `OPT-048` and Host NLCL without changing refusal or approval semantics. Any false accept is a hard regression.

## Second horizon — after measurements suggest a genuine bottleneck
**F.** Query plan and FTS tradeoffs, including count+page overhead, indexes, WAL checkpoints and materialized views.

**G.** Time sequencing and logical clocks: low idle wake budget, safe coalescing, cancellation overshoot, skew and event causality. Don't add a clock service before inspecting the canonical owner.

**H.** Encodings and IPC: NDJSON versus framed protocols, JSON stringify/hash/copy overhead, true zero-copy feasibility and cost of FFI.

**I.** Immutable cache and compaction: reuse source-pinned calculations, guard namespace invalidation and prove restoration and deletion safety.

**J.** Developer flow: build artifact growth, test graph, lock-file/source closure, target/debug historical measurement reproduction, cross-platform compile.

## Third horizon — highly creative but not automatically useful
Adaptive resource budgets; context-aware prefetch without background surveillance; formal verifier equivalence checks; a compiler that emits proof hints; energy-aware scheduling; content-addressed independent snapshots; pure Rust computation across desktop/PWA; temporal IR optimizer; cross-layer live performance provenance; guided “cost of truth” visualization.

## Ten specific research questions to answer before any implementation
1. Which **actual running** processes dominate idle CPU and disk writes?
2. How large are SQLite base and WAL files, and how quickly do they grow per 10k source events?
3. How expensive is a full verified compaction when free disk is critically low?
4. What are p95/p99 decode and replay costs at real active-user burst rates?
5. Does Rust HQKG's cache materially reduce latency without growing RSS too much?
6. Is TierFlow frame aggregation limited by JSON IPC, render or Rust work?
7. How many redundant per-revision JSON parses or hashes occur on the command path?
8. What delays arise from conservative one-writer sequencing versus pure preflight?
9. How much developer time and disk are spent rebuilding the same native artifacts?
10. Which potential optimization has the strongest **measured** payoff for the least semantic/authority risk?

## Decision rule
Choose **at most 3–5** measured candidates for an initial approved implementation wave, with measured baseline + independent owner review, retaining fallback/rollback. This folder does **not** itself start that wave.
