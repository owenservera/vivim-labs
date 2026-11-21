# Rust implementation workstream (running)

**Status:** The G1–G6 gates for replacing HQ's active Bun runtime remain mandatory. The owner separately authorized isolated Rust knowledge-graph research under research/rust-knowledge-graph; it is not an accepted runtime replacement.
**Rule:** Rust starts only after the architecture it replaces is fully tested,
accepted, and measured. Until then this document is a decision ledger, not a
build plan.

## Why this exists

HQ v1 runs on Bun 1.4.2 with `node:sqlite` + FTS5. That is intentionally
boring: one local database, no services, no cloud. It is fast enough for the
current workspace. Rust becomes justified only when telemetry proves a
sustained bottleneck or when a capability needs something JS cannot provide
(native concurrency, durable watchers, predictable memory, single-binary start).

The point of this workstream is to make the "when" explicit and evidence-based
instead of opinion-based.

## Gates — all must pass before any Rust lane starts

| Gate | Requirement | Evidence artifact |
| --- | --- | --- |
| G1 Architecture accepted | Contract frozen; index, context, receipts, wiki, telemetry tests green | `hq verify`, full test suite |
| G2 Measured bottleneck | `hq timing` shows the operation over budget for 14 days, or a hard limit (memory/startup/concurrency) | `hq timing budgets`, benchmark records |
| G3 Parity plan | Identical outputs for the same inputs; golden files; digest equality | parity test matrix |
| G4 Stable boundary | JSON/stdio protocol only; no shared mutable state; no new authority | protocol spec |
| G5 Rollback | Bun path remains; Rust is opt-in behind `HQ_ENGINE=rust`; uninstall is clean | feature-flag test |
| G6 Owner approval | Explicit owner decision recorded here | running log entry |

If any gate fails, the answer is no.

## Candidate map

Status values: `not-started`, `measuring`, `gated`, `approved`, `in-progress`,
`accepted`, `rejected`.

| ID | Candidate | Why Rust could win | Preconditions | Expected gain | Status |
| --- | --- | --- | --- | --- | --- |
| R001 | File walking + hashing | Parallel stat/hash across 100k files; predictable memory | G1, G2 | 5–20× on cold index | not-started |
| R002 | Symbol extraction | Parser-grade ASTs without JS regex limits; tree-sitter or native grammars | G1, G3 | correctness + 3–10× | not-started |
| R003 | Import graph + impact BFS | Cache-friendly graph traversal at scale | G1, G2 | 5–10× on deep graphs | not-started |
| R004 | FTS/search ranking | Custom BM25 + hybrid ranking beyond SQLite FTS5 | G1, G2 | 2–5× or better recall | not-started |
| R005 | Context bundle scorer + knapsack | Tight loops over candidate sets; deterministic knapsack | G1, G3 | 5–10× at 1k candidates | not-started |
| R006 | Git log parsing + lake ingestion | Parallel commit parsing; no per-process overhead | G1, G2 | 5–15× on 100k commits | not-started |
| R007 | Wiki generation | Parallel page rendering + atomic writes | G1, G2 | 3–10× at 10k pages | not-started |
| R008 | Watch daemon | Native OS watchers, single small binary, low memory | G1, G2 | lower idle cost | not-started |
| R009 | SQLite bulk indexing | Batched writer/pragmas; prepared statement reuse | G1, G2 | 2–5× | not-started |
| R010 | MCP server | Single binary transport; no runtime dependency | G1, G3 | startup + distribution | not-started |
| R011 | Telemetry aggregation | Columnar scans / percentile computation | G1, G2 | 2–10× on large spans | not-started |
| R012 | CLI startup | Single binary, <10ms cold start | G1 | startup only | not-started |
| R013 | Cross-project graph | Whole-workspace graph algorithms | G1, G2 | 5–10× | not-started |
| R014 | Incremental diff engine | Content-addressed diffing across projects | G1, G3 | 3–10× | not-started |
| R015 | Secret scanning | Fast regex/Aho-Corasick over every file | G1, G3 | 5–20× | not-started |
| R016 | Wiki link/anchor checker | Parallel validation | G1 | 3–5× | not-started |
| R017 | Benchmark harness | Deterministic native measurement | G1 | enables G2 | measuring |
| R018 | Embedding/semantic index | Local vector search without a service | future scope | new capability | gated |
| R019 | WASM plugin runtime | Safe third-party extensions | future scope | new capability | gated |
| R020 | Windows-native watcher/FS | USN journal, long paths, file locks | G1, G2 | correctness + speed | not-started |

## Measurement protocol

1. Collect at least 14 days of `hq timing` spans, or a deterministic benchmark
   run with warm and cold caches.
2. Record for every candidate: p50, p95, max, memory peak, binary size, cold
   start, and error rate.
3. Store raw evidence under `docs/rust/benchmarks/<date>-<candidate>.json`.
4. Compare against the Bun baseline with the same dataset and machine.
5. Replacement threshold: **10× on the bottleneck**, or removal of a hard limit
   (memory, concurrency, startup, single-binary distribution). A 2× win is not
   enough to justify a second runtime.
6. Update this table and the running log. No silent adoption.

## Architecture options

| Option | Shape | Pros | Cons | When |
| --- | --- | --- | --- | --- |
| A. N-API addon | `napi-rs` module loaded by Bun/Node | low call overhead | ABI/version coupling, crash blast radius | only hot loops after parity |
| B. Sidecar binary | long-lived process over newline JSON-RPC, same shape as MCP | isolation, language-neutral, easy rollback | IPC overhead, lifecycle management | default for R001–R016 |
| C. WASM | portable module | sandboxed, portable | lower peak performance, memory limits | plugins (R019), not core |
| D. Replace CLI | full Rust binary | single binary, startup | full rewrite, two codebases | last, only if B is accepted |

**Default decision:** sidecar first. N-API only for a proven hot loop with a
stable ABI. Never let Rust write outside the same read-only boundaries as the
Bun engine.

## Integration contract (must not change)

- Same JSON schemas: `hq.context-bundle/1`, `hq.feed/1`, `hq.status/1`,
  `hq.timing-*/1`, wiki page ids.
- Same canonical digests, receipts, and omissions.
- Same read-only authority: no sends, no browser effects, no external writes.
- Telemetry records `engine: bun|rust` and the same budget/optimizer model.
- Feature flag: `HQ_ENGINE=bun` default; `HQ_ENGINE=rust` opt-in per operation.
- A Rust failure must fall back to Bun or fail closed; never corrupt the DB.

## Do not start checklist

- [ ] G1 accepted architecture with frozen contracts
- [ ] G2 14-day telemetry or deterministic benchmark evidence
- [ ] G3 golden parity tests for the candidate
- [ ] G4 stable JSON/stdio boundary
- [ ] G5 rollback and feature flag proven
- [ ] G6 owner approval recorded

## Running log

| Date | Entry | Evidence | Owner |
| --- | --- | --- | --- |
| 2026-11-02 | Workstream opened. No Rust authorized. Tracking candidates R001–R020. | v1 Bun + `node:sqlite` + FTS5 verified; architecture still being accepted | owner |
| 2026-11-02 | Benchmark harness (R017) marked `measuring`; telemetry ledger added with budgets, priorities, and conflict-aware optimizer. | `hq timing` module + tests | agent |

## Revisit triggers

Re-open the decision when any of these happens:

- `hq timing budgets` shows a high-priority operation over budget for 14 days.
- The workspace exceeds 100k indexed files or 100 projects.
- Cold index exceeds 60 seconds on the reference machine.
- Memory peak exceeds 1 GB during index or wiki generation.
- A required capability cannot be built safely in Bun (native watcher, binary
  distribution, sandboxed plugins).
- An owner explicitly asks for a Rust experiment.

## Related documents

- `docs/NORTH-STARS.md` — product scenarios (U/A/V/W).
- `hq/CONTRACT.md` — module contracts and read-only boundaries.
- `hq/AGENT-PROTOCOL.md` — inline-comment and context protocol.
- `hq/README.md` — architecture and commands.


## 2026-11-03 scoped research exception

The owner requested the experimental Rust graph core and then directed its source migration into this repo. This approves source development and test fixtures, **not** replacing `hq/lib/graph.mjs`, the Bun/SQLite index, the default CLI, or existing permissions. Operating-system, provider and browser effects remain outside the knowledge graph. G1–G6 still apply to activation.
