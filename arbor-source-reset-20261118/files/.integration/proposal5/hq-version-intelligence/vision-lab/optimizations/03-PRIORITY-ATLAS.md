# 03 — Opportunity prioritization atlas, v0.1
**59 pinned source-backed optimization research candidates.** Scores below are **estimated screening heuristics**, not measured speedups or bug severity. All candidate rows and blob SHA evidence are in [opportunities.json](opportunities.json) and [opportunities.csv](opportunities.csv).

## Transparent score, not a claim of performance
`score = 5×Impact + 4×FrequencySensitivity + 3×FootprintPressure + 3×CrossSurfaceLeverage + 3×MeasurementFeasibility − 2×ChangeRisk`
Each dimension is an editorial hypothesis (integer 1–5). Larger score suggests **measure sooner**; lower score never means unimportant for safety. Ties sort by stable candidate ID. The risk subtraction is only triage guidance; any correctness/privacy/authority violation is hard NO_GO regardless of numerical score.

- **P0_MEASURE_FIRST:** top 12 screening ranks; *profile and establish baseline*, not an approved implementation.
- **P1_RESEARCH:** next 18; inspect real invocation and cost, characterize worst-case.
- **P2_WATCH:** remaining 29; keep a source-backed hypothesis, move up only with evidence.
- **Critical override:** if actual correctness/privacy/durability/recovery failures are discovered, escalate as a separate blocker independent of performance ranking.

## Top 30 (human-readable)
| Rank | ID | Optimization question | Area | Score | Cadence hypothesis | Grounding |
|---:|---|---|---|---:|---|---|
| 1 | OPT-016 | Rust typed DAG cache effectiveness | Computation | 78 | FREQUENT | `native/hqkg/src/typed_dag.rs` |
| 2 | OPT-022 | Studio delta assembly and snapshots | Computation | 78 | HIGH_FREQUENCY | `studio/src/decode/delta.mjs` |
| 3 | OPT-025 | TierFlow Rust sidecar hotpath | Computation | 78 | HIGH_FREQUENCY | `packages/tierflow/src/hotpath.ts` |
| 4 | OPT-038 | Studio bounded protobuf decoder | Codecs | 78 | HIGH_FREQUENCY | `studio/src/decode/protobuf.mjs` |
| 5 | OPT-057 | Rust debug build disk footprint | Maintainability | 78 | DEV_ADMIN | `native/hqkg/Cargo.toml` |
| 6 | OPT-001 | HQ incremental FTS write amplification | Storage | 77 | DEV_ADMIN | `hq/lib/indexer.mjs` |
| 7 | OPT-004 | Studio raw-log queue and segmentation | Storage | 76 | HIGH_FREQUENCY | `studio/src/store/raw-log.mjs` |
| 8 | OPT-013 | Rust bounded BFS and ranking | Computation | 76 | USER_DEFINED | `native/hqkg/src/context_retrieval.rs` |
| 9 | OPT-028 | Rust web-memory batch/replay | Computation | 76 | FREQUENT | `native/hqkg/src/web_memory.rs` |
| 10 | OPT-036 | TierFlow frame coalescing cadence | Time | 75 | ALWAYS_ON | `packages/tierflow/src/hotpath.ts` |
| 11 | OPT-015 | HQ incremental index invalidation | Computation | 74 | DEV_ADMIN | `hq/lib/indexer.mjs` |
| 12 | OPT-021 | HQ context token budget efficiency | Computation | 74 | USER_DEFINED | `hq/lib/context-budget.mjs` |
| 13 | OPT-048 | TierFlow document verification | Verifier | 74 | USER_DEFINED | `packages/tierflow/src/visual-runtime/document/validate.ts` |
| 14 | OPT-002 | HQ SQLite WAL and database growth | Storage | 73 | DEV_ADMIN | `hq/lib/schema.mjs` |
| 15 | OPT-056 | Rust workspace build closure | Maintainability | 73 | DEV_ADMIN | `Cargo.toml` |
| 16 | OPT-039 | Studio generic capture decode | Codecs | 72 | HIGH_FREQUENCY | `studio/src/decode/decoders.mjs` |
| 17 | OPT-007 | Rust HQKG verified journal compaction | Storage | 71 | MEDIUM_FREQUENCY | `native/hqkg/src/lib.rs` |
| 18 | OPT-030 | HQ polling/debounce idle cost | Time | 71 | ALWAYS_ON | `hq/lib/automation.mjs` |
| 19 | OPT-035 | Studio timer/fsync interval | Time | 71 | ALWAYS_ON | `studio/src/store/raw-log.mjs` |
| 20 | OPT-003 | TierFlow durable UI revision footprint | Storage | 69 | USER_DEFINED | `packages/tierflow/src/ui/store.ts` |
| 21 | OPT-014 | HQ SQL graph impact traversal | Computation | 69 | DEV_ADMIN | `hq/lib/graph.mjs` |
| 22 | OPT-027 | Host web memory intake dedup | Computation | 69 | FREQUENT | `apps/vivim-host/bootstrap/web-memory.mjs` |
| 23 | OPT-043 | TierFlow command IR compilation | Compiler | 69 | USER_DEFINED | `packages/tierflow/src/visual-runtime/command/compile.ts` |
| 24 | OPT-054 | Rust-versus-JS pure compute overlap | Maintainability | 69 | DEV_ADMIN | `native/hqkg/src/typed_dag.rs` |
| 25 | OPT-012 | HQ search count and paging overhead | Storage | 68 | USER_DEFINED | `hq/lib/search.mjs` |
| 26 | OPT-020 | HQ pure semantic optimizers | Computation | 68 | USER_DEFINED | `hq/lib/semantic-optimizers.mjs` |
| 27 | OPT-055 | HQ adapter import closure | Maintainability | 68 | DEV_ADMIN | `hq/lib/arbor-service.mjs` |
| 28 | OPT-041 | TierFlow sidecar NDJSON serialization | Codecs | 67 | HIGH_FREQUENCY | `packages/tierflow/src/hotpath.ts` |
| 29 | OPT-045 | Host natural-language intent parse | Compiler | 67 | USER_DEFINED | `apps/vivim-host/control/command-intent.mjs` |
| 30 | OPT-053 | Host approval readiness verification | Verifier | 67 | USER_DEFINED | `apps/vivim-host/bootstrap/readiness-gate.mjs` |

## Seven grouped opportunity families
- **Computation (16):** Rust typed DAG cache effectiveness; Studio delta assembly and snapshots; TierFlow Rust sidecar hotpath; Rust bounded BFS and ranking.
- **Codecs (5):** Studio bounded protobuf decoder; Studio generic capture decode; TierFlow sidecar NDJSON serialization; Mobile generative UI compiler payload.
- **Maintainability (6):** Rust debug build disk footprint; Rust workspace build closure; Rust-versus-JS pure compute overlap; HQ adapter import closure.
- **Storage (12):** HQ incremental FTS write amplification; Studio raw-log queue and segmentation; HQ SQLite WAL and database growth; Rust HQKG verified journal compaction.
- **Time (9):** TierFlow frame coalescing cadence; HQ polling/debounce idle cost; Studio timer/fsync interval; HQ refresh critical-path sequencing.
- **Verifier (6):** TierFlow document verification; Host approval readiness verification; Rust pure-program verifier; Studio integrity proof scan.
- **Compiler (5):** TierFlow command IR compilation; Host natural-language intent parse; TierFlow command crosswalk lookup; HQ semantic identity resolution.

## Important non-numerical priorities
1. **Protect low-disk operation.** Observe current free space and peak extra copy bytes before tuning/compacting anything.
2. **Find actual idle activity.** A 5s poll or a 50ms flush interval costs nothing when not running; first verify which processes remain resident.
3. **Measure data truth, not just speed.** Durable ACK, immutable originals and honest gap records outrank throughput.
4. **Source closure/build debt is development cost.** An imported source file is not automatically a runnable Hot Path.
5. **Avoid double optimization registries.** This observatory complements Proposal 5's [rescore expansion](../../../rescore-expansion/README.md) and does not replace it.

## How to interpret ranking
A high-ranked optional decoder can be dormant for weeks; a lower-ranked always-on poller can dominate lifetime CPU. Re-rank using **actual call rate × per-call cost**, tail peak, and measured memory-duration once receipts exist. No automatic code change is authorized from this table.
