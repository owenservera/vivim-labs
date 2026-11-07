> **Follow-up (2026-11-03):** The Rust core moved to Repos HQ and a bounded JSONL importer has been implemented as the optional `importer` Cargo feature (`src/import.rs` and `src/bin/hqkg-import.rs`). It uses optional `serde_json` and `sha2` only when requested; the default kernel remains std-only. The original assessment below remains historical, and runtime tests are still outstanding.

# Minimum Rust core delta — agentic knowledge graph compatibility

**Assessment date:** 2026-11-03. **Decision:** **Zero mandatory public-library/core modifications** for V1, **one small ingestion adapter required**, plus **two strictly optional core quality improvements**. Source verified against current `owenservera/repos-hq/research/rust-knowledge-graph` main: `src/lib.rs` Git blob `506b85277b73c2999e1dde0badeaf2f671a7cec7`, `src/engine.rs` `e8d605c2dbf47c28176fa02775e43a174c6deb49`, `src/algorithm_adapter.rs` `a94fc3ae46195add823aceeaee1ad0a4eec334a8`.

**Scope:** Integrate Repos HQ's 217 exact-hash archived agentic file sources, 1,010 observed heading sections, 297 prior schema candidates, ten-family scoring, 10 existing pure HQ optimizers, 12 candidate primitives and 30 researched KGSI algorithms. Treat scores and candidate classifications as provisional. Preserve std-only core, optional database drivers, read-only HQ planning and Host-owned permissions.

## Core compatibility checklist

| Requirement | Current Rust implementation | Mandatory core change |
| --- | --- | --- |
| Typed file/section/schema/algorithm nodes | `NodeKind::Other(String)`, `Node.properties`, `Node.sources`, versioned `Graph` | **No** |
| Typed evidence/provenance edges | `Edge::new`, string relation, weight/properties | **No** |
| Stable 69-byte bridge IDs | `identity()` accepts up to 256 bytes; `BTreeMap` determinism | **No** |
| Graph traversal/dependency/rank | BFS, reachability, Dijkstra, topo, PageRank, keyword search | **No** |
| SQLite/PostgreSQL as whole-database nodes | Existing `DatabaseSpec` and optional read-only adapters | **No** |
| Hot-swappable algorithm engines | `EngineRegistry::install/pin`, `EngineLease`, CAS generation | **No** |
| 30 research algorithm definitions | New `algorithm_adapter.rs` seeds inert KGSI-001..030; trusted driver bridge | **No** |
| Scoped placements and relocations | Compose graph placement nodes/edges and versioned `EngineRequest.data` in adapter | **No** |
| Bulk ingest 1k–100k nodes without N full journal writes | `Graph::put_node/put_edge` are public, `Graph: Clone`, `Journal::save` public | **No to library; YES to separate importer** |
| Deterministic reimport / drift detection | Compare source manifest and existing node/edge values before invoking mutations | **No** |
| Bounded read-only computation | `Scheduler` budgets, `GraphCompute` and optional `DatabaseCompute` | **No** for trusted V1 |

### Exactly one required integration component (not a core rewrite)

**Add a small, std-only HQKGX1 importer executable** (ideally `research/rust-knowledge-graph/src/bin/hqkg-import.rs`, or a standalone program depending on the HQKG library) to consume Repos HQ's versioned graph export. No changes needed to `src/lib.rs`, `src/engine.rs`, `src/database.rs`, `src/algorithm_adapter.rs`, or the default Cargo dependency list.

1. Repos HQ's current `scripts/export-hqkg.mjs` emits **JSONL**, but std-only Rust has **no JSON parser**. Either add an **opt-in `serde_json` adapter feature** (NOT a default core dependency), or — recommended to maintain zero dependencies — have Bun emit an additional **bounded length-prefixed binary HQKGX1B exchange** that a small std-only Rust reader can parse. **Do not** parse JSON with regex or hand-write a general JSON parser. **Do not** write native HQKG journal frames directly from Bun.
2. Stage changes in a **cloned `Graph`** (or a freshly loaded graph) in memory. Validate manifest SHA-256 in the importer, schema version, count/byte limits, UTF-8, stable IDs, node/edge uniqueness and endpoint existence. Apply nodes before edges, then verify output parity. Persist exactly **one accepted graph snapshot** via `Journal::save`. Avoid one CLI invocation or journal frame per item.
3. For repeated imports, **compare canonical values and skip unchanged nodes/edges** before calling `put_node/put_edge`: those methods increment version/epoch even when the payload is equal. For deletions, stage explicit missing-ID removals only if this is a full authoritative snapshot; never delete non-owned or live runtime nodes.
4. Store source crosswalk and large body/section text in SQLite. Import only metadata + graph topology into Rust. A database can itself be a first-class graph node; do not import every database row to graph memory.
5. Keep two algorithm namespaces distinct: Repos HQ `algorithm:A01` / `algorithm:normalize_text` etc. and Rust `KGSI-001..030` (research identities). Repos HQ's hashed bridge IDs are *transport identities*, **not** a replacement for `algorithm_adapter.rs`'s catalog IDs or engine slots. Maintain explicit maps: canonical semantic ID ↔ bridge ID ↔ algorithm ID ↔ slot.
6. The Rust algorithm adapter already supplies `ensure_algorithm_node`, `AlgorithmDriver`, `AlgorithmCompute` and `record_algorithm_engine_swap`; it does **not** mean the catalog contains executable EdgeMem/Graphiti/etc. implementations.

## Optional tiny core changes — only if integration tests justify them

**O1. Idempotent graph upserts** — Add `Graph::put_node_if_changed` and `Graph::put_edge_if_changed`, returning `Result<bool>`. Compare all semantically relevant fields except auto-increment `version`; an equal node/edge must not increase epoch, change the journal or invalidate leases. This is useful for frequent sync and content drift but can be implemented in the importer first; **not a V1 blocker**.

**O2. Scoped neighborhood query** — Expose a bounded `Graph::neighbors(id, relation, limit)` or adjacency projection helper that returns only requested data. Current `outgoing/incoming` scan all edges, while `AlgorithmDriver::execute` receives access to the full `&Graph` under an `RwLock` read guard. For trusted in-process algorithms and ~2,771 edges, existing behavior is adequate. For large graphs, performance or isolation needs, first benchmark; add indexes or scoped views only after measured need. A read-only Rust borrow **does not** sandbox malicious drivers.

### Operational correctness issue (adapter-side, not required core API)

`EngineRegistry::install` and `record_algorithm_engine_swap` plus `Journal::save` are **not** one atomic transaction. If journal persistence fails after an in-process install, active generation and recorded topology may disagree. Add an external **prepare/commit/reconcile receipt** to the integration owner and on restart re-install engine implementations and reconcile against durable lineage. Do not claim crash-atomic live swaps without this. The synchronous scheduler cannot interrupt a hung third-party/native driver; run untrusted or long-running plugins in a separate supervised process later.

### Explicitly avoid now

- No replacement graph engine, no second engine registry or scheduler.
- No forced `serde`/`tokio`/Postgres/SQLite dependency in the default Rust crate.
- No redesign of Rust node/edge types, no mandatory FFI/WASM/plugin loader, no unbounded graph query language.
- No source document text/secret import or promotion of donor documents into instructions.
- No rewrite of snapshot journal or full-graph indexing before measured performance thresholds.
- No execution-authority expansion into HQ, graph algorithms or knowledge entries.

## Acceptance gates for a minimal integration

| Gate | Passing condition |
| --- | --- |
| Source consistency | Repos HQ 217/217 archive hashes match registry and original crosswalk |
| Import structural parity | 1,582 nodes and 2,771 edges from current projection, plus any separately agreed algorithm/catalog additions; exact counts must be derived from a pinned manifest and tested |
| Idempotence | Re-import unchanged manifest produces no graph epoch or version change and no redundant journal snapshot |
| Identity | Every bridge ID maps back to one canonical source ID; no duplicate KGSI/semantic algorithm conflation |
| Durability | Torn tail recovers last valid frame; stale locks fail closed; single snapshot per accepted import |
| Runtime safety | Inert research algorithms stay inert; existing generation-pinned leases survive swaps |
| Compatibility | `cargo test` default passes; optional database tests when enabled; Bun bridge/check tests pass |
| Benchmarks | Record CPU/RAM, cold import, path latency, and journal file growth with 1k, 10k and 100k nodes |

**Tests:** Not executed in this assessment; current findings are based on GitHub source inspection, not local Rust/Bun runtime results.

## Existing owners and links

- Rust kernel: [owenservera/repos-hq/research/rust-knowledge-graph](https://github.com/owenservera/repos-hq/tree/main/research/rust-knowledge-graph)
- Agentic graph library: [owenservera/repos-hq/research/agentic-knowledge-graph](https://github.com/owenservera/repos-hq/tree/main/research/agentic-knowledge-graph)
- Exchange exporter: `scripts/export-hqkg.mjs`
- Integration architecture: `HQKG-INTEGRATION.md`

Proposed Windows paths: `C:\0-BlackBoxProject-0\repos-hq\research\rust-knowledge-graph` and `C:\0-BlackBoxProject-0\repos-hq\research\agentic-knowledge-graph`. These are conventions, not proof of a local checkout.
