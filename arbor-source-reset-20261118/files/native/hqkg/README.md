> **Canonical owner:** [Repos HQ](https://github.com/owenservera/repos-hq/tree/main/research/rust-knowledge-graph). This remains an isolated experiment, not the operational Bun HQ indexer.

# HQKG — Rust knowledge graph with hot-swappable compute and database nodes

A self-contained Rust crate under HQ, intentionally **not coupled to PC HQ runtime state** or other project-specific application logic. This is a portable research/prototype kernel, not an installed host service. Uses only Rust's standard library with default features. It does **not** require SQLite, PostgreSQL, Tokio, serde, network access, or a cloud service just to compile the core.

## Architecture

```text
   Knowledge nodes: Fact / Concept / Document / Dataset / Custom
       | typed, directed, weighted, provenance-bearing edges
       v
   Graph (in-memory BTreeMap, stable IDs, revision counters)
       | BFS / reachability / Dijkstra / topo / PageRank / search
       v
   Journal (append-only full snapshots, checksum, replay, crash-tail recovery)
       |
       +-- EngineSlot nodes ---- active_engine ---> ComputeEngine generation nodes
       |       |  Arc pin (old jobs keep running on old implementation)
       |       +-- CAS hot-swap + interface/capability compatibility
       |       +-- generations linked with "supersedes" history
       |
       +-- Database nodes: SQLite / PostgreSQL
               | references only; never embed credentials in graph
               +-- SQLite adapter (opt-in rusqlite)
               +-- PostgreSQL adapter (opt-in postgres)
```

## Build and test

```sh
cargo test                         # std-only; core needs no external crate
cargo run -- ./graph.hqj init
cargo run -- ./graph.hqj add-node paper:1 document "Original paper"
cargo run -- ./graph.hqj add-node concept:1 concept "Knowledge graph"
cargo run -- ./graph.hqj add-edge relation:1 paper:1 supports concept:1
cargo run -- ./graph.hqj path paper:1 concept:1
cargo run -- ./graph.hqj rank
cargo test --features databases     # downloads/compiles the two optional adapters
```

### Entire SQLite database as a node

```sh
cargo run --features sqlite -- ./graph.hqj db-add db:docs sqlite ./docs.sqlite
cargo run --features sqlite -- ./graph.hqj db-query db:docs 50 "SELECT name FROM sqlite_master"
```

The SQLite connection is opened with READ_ONLY flags; only SELECT/WITH is accepted, and rows are bounded. This is a *reference to the whole database*, not an import of all rows into graph memory. Refer to individual entities with graph edges as desired.

### Entire PostgreSQL database as a node

```sh
export HQKG_PG_DSN='host=localhost dbname=kb user=kb_readonly password=...'
cargo run --features postgres -- ./graph.hqj db-add db:warehouse postgres - HQKG_PG_DSN
cargo run --features postgres -- ./graph.hqj db-query db:warehouse 30 "SELECT tablename FROM pg_catalog.pg_tables"
```

The graph stores the *environment variable name*, not its secret value. PostgreSQL queries run in a READ ONLY transaction, with a 5s server statement timeout and bounded cursor fetch. Provision a **least-privilege read-only PostgreSQL role** in addition to these application guards.

## Built-in executable compute nodes

Both built-ins implement the same `ComputeEngine` interface, so you can swap
or replace implementations while keeping callers bound to logical slots:

- `builtins::GraphCompute`: operations `path`, `neighbors`, `reachable`,
  `topological`, `rank`, `search`, and `stats`. Shares a graph through
  `Arc<RwLock<Graph>>` and never requires external dependencies.
- `builtins::DatabaseCompute`: operations `query`, `catalog`, and `health`.
  SQLite/PostgreSQL use their optional feature-gated database adapters.
  The graph can attach this compute engine node to a database node using a
  `queries_database` relationship; slot identity is `database:<node-id>`.

## Universal algorithm node adapter (30 research engines)

The `algorithm_adapter` module adds a **native**, std-only extension for graph
algorithms. It reuses HQKG's existing `ComputeEngine`, `EngineRegistry`,
`EngineLease`, and `Graph::record_engine_swap`; there is **no parallel
registry, scheduler, or graph writer**.

- `ALGORITHMS`: 30 immutable research-method identities from the
  [Repos HQ research catalog](https://github.com/owenservera/repos-hq/blob/main/research/knowledge-base-network-graph-signal-intelligence/algorithms.v1.json).
- `seed_algorithm_nodes(&mut graph)`: idempotently creates `engine:KGSI-001`
  through `engine:KGSI-030` as **inert** `algorithm` nodes. Each node retains
  algorithm ID, family, paper/source URL, and suggested engine-slot ID.
- `ensure_algorithm_node`: the same node-model for any future custom algorithm.
- `AlgorithmDriver`: trusted local algorithm code implementing
  `operations()` and `execute(&EngineRequest, &Graph)`.
- `AlgorithmCompute::new(id, implementation, graph, driver)`: checks the
  canonical algorithm node exists, then implements the native `ComputeEngine`
  trait with interface `hqkg.algorithm.v1` and slot `algorithm:<ID>`.
- Native `EngineRegistry::install` provides generational CAS replacement;
  leases already pinned to the old engine finish on the old implementation.

Example using a **generic demonstration driver** (not an EdgeMem implementation):

```rust
use std::collections::BTreeSet;
use std::sync::{Arc, RwLock};
use hqkg::{Graph, Result, EngineRegistry, EngineRequest, EngineOutput,
           AlgorithmDriver, AlgorithmCompute, seed_algorithm_nodes};

struct CountOutgoing;
impl AlgorithmDriver for CountOutgoing {
    fn operations(&self) -> BTreeSet<String> {
        ["count"].iter().map(|s|s.to_string()).collect()
    }
    fn execute(&self, request: &EngineRequest, graph: &Graph) -> Result<EngineOutput> {
        Ok(EngineOutput {
            data: graph.outgoing(&request.data, None)?.len().to_string()
        })
    }
}
fn demo() -> Result<()> {
    let graph = Arc::new(RwLock::new(Graph::new()));
    seed_algorithm_nodes(&mut graph.write().unwrap())?;
    let engine = AlgorithmCompute::new("KGSI-001", "local-counting-demo",
        Arc::clone(&graph), Arc::new(CountOutgoing))?;
    let descriptor = hqkg::ComputeEngine::descriptor(&engine);
    let registry = EngineRegistry::new();
    let swap = registry.install(Arc::new(engine), None)?;
    let output = registry.pin("algorithm:KGSI-001")?
        .execute(&EngineRequest { operation:"count".into(), data:"engine:KGSI-001".into() })?;
    assert_eq!(output.data, "0");
    hqkg::record_algorithm_engine_swap(
        &mut graph.write().unwrap(), "KGSI-001", &descriptor, &swap)?;
    // For durability, call Journal::save after lineage materialization.
    Ok(())
}
```

**Identity distinction:** `engine:KGSI-001` is a research algorithm-definition
node; `engine-slot:algorithm:KGSI-001` is a logical runtime slot; an installed
`engine:algorithm:KGSI-001:<generation>` is the implementation-generation
node. A research node is never itself an installed executable.

**Critical boundaries:** the catalog is not an implementation library. No
research method runs until an actual driver is compiled and registered.
Algorithm drivers run as **trusted in-process Rust** and can have effects if
malicious; they are not sandboxed. `Graph` passed to drivers is an immutable
borrow held under a shared lock for that call. The core's synchronous budget
and journal behavior remain unchanged: the scheduler cannot preempt a stuck
native driver, and engine installation plus journal persistence is **not**
a single atomic transaction. Journal failure needs explicit reconciliation.
Persisted graph state does not serialize engine objects; reinstall them
after restart. Source data remains in Repos HQ; this module's list is a
materialized projection and should be regenerated/checked when the catalog
changes.

Run `cargo test` in this crate to exercise
`tests/algorithm_adapter.rs`, including idempotent 30-node import,
serialization, native generation hot swapping, and custom-driver rejection.

---

## Engine hot swapping

Implement `ComputeEngine`, return a stable `EngineDescriptor {slot, implementation, interface, capabilities}`, and install `Arc<dyn ComputeEngine>` with `EngineRegistry::install(engine, expected_generation)`.

```rust
let first = registry.install(Arc::new(MyEngineV1), None)?;
let job = registry.pin("search")?; // pinned to generation 1
let changed = registry.install(Arc::new(MyEngineV2), Some(first.generation))?;
graph.record_engine_swap(&MyEngineV2.descriptor(), &changed)?;
journal.save(&graph)?;
```

New leases use generation 2; `job` remains on generation 1 until dropped. Compatibility is conservative: the interface must match and capabilities cannot be removed. Swap receipts can be materialized in the graph with one active edge and immutable generation/history node IDs. This is a **safe in-process replacement**, not arbitrary native shared-library hot reloading. If you want external processes/WASM/native dynamic libraries, implement an adapter and explicit isolation/rollback contract rather than unsafely loading code into this core.

## Invariants and current limits

- Core has **zero non-stdlib dependencies**, all optional integrations are feature-gated. No APIs or model services.
- Deterministic IDs, map order and graph serialization; strings are length-prefixed UTF-8, no JSON dependency.
- Engine descriptors define a logical slot; revisions represent concrete engine instances. A database is a first-class graph node, not a special storage backend for the graph itself.
- Source evidence stays attached to nodes; relationships can capture derivation, ownership, transformations and dependency.
- Journal is **single writer**, using a create-new lock file. A stale lock requires manual review. Journal frames use FNV-1a checksums to detect accidental corruption, **not cryptographic authentication**.
- Journal writes snapshot the entire graph: intended as a minimal correct kernel; large knowledge bases should move to incremental pages/indexes and compaction.
- The persisted journal stores graph topology and metadata, **not executable engine implementations**. Engine objects must be reinstalled on process startup.
- The synchronous scheduler checks budgets between calls; it cannot preempt hung third-party engines. Engines must supply their own cancellation/timeouts for strict execution containment.
- SQL adapters are read-only, with no query parameters or schema introspection API yet. SELECT/WITH screening is not a SQL security parser; OS file permissions and DB roles remain authoritative.
- No networked distributed consensus, MVCC multi-writer coordination, graph query DSL, or live migration are claimed.
- Runtime journal files (`*.hqj`, locks) and secrets should stay local, never committed to Git.

## Next stage

Add incremental storage, secondary indexes, engine capability routing/benchmarks, pluggable compute nodes that run against graph neighborhoods, stable transactional graph+runtime swap coordination, versioned schema catalogs for database nodes, data provenance/lineage, cancellation and sandboxed worker boundaries. Keep the default core std-only.


## Opt-in HQKGX1 importer

Consumes the existing Repos HQ JSONL export; validates manifest SHA-256, schema, counts, duplicate IDs, field limits and endpoint existence; stages all changes on a cloned graph; writes **one** journal snapshot for changed input, none for unchanged input. Ownership collisions fail closed. Default Rust core has no third-party crates; `serde_json` and `sha2` are optional importer-only dependencies.

```sh
# From Repos HQ repository root
bun research/agentic-knowledge-graph/scripts/export-hqkg.mjs
cargo test --manifest-path research/rust-knowledge-graph/Cargo.toml
cargo test --manifest-path research/rust-knowledge-graph/Cargo.toml --features importer
cargo run --manifest-path research/rust-knowledge-graph/Cargo.toml --features importer --bin hqkg-import -- research/agentic-knowledge-graph/runtime/graph.hqj research/agentic-knowledge-graph/runtime/hqkg-import.v1.jsonl
```

**Testing status:** tests have not yet been run in a Rust toolchain.


## Integrated control plane, indexed traversal, and operational lifecycle

The std-only `control` module provides strict versions and dependency DAG
resolution, production immutable resource pins, CAS versioned configuration,
bounded redacted telemetry/TSV, health/quarantine, and scoped ephemeral-cache
self-repair. `hqkg-control` is an explicit operator CLI for `doctor`,
`config show/propose/apply` and verified `repair-tail`. The graph now maintains
derived BTreeSet incoming/outgoing adjacency indexes (rebuilt after journal
deserialization), avoiding full edge scans on every neighbor traversal.

See the [three-stream integration](../graph-kernel-integration/) and
[operational contract](../graph-kernel-integration/CORE-RUNTIME-PROTOCOL.md).

```sh
cargo test --manifest-path research/rust-knowledge-graph/Cargo.toml
cargo test --manifest-path research/rust-knowledge-graph/Cargo.toml --features importer
cargo run --manifest-path research/rust-knowledge-graph/Cargo.toml --bin hqkg-control -- runtime/graph.hqj doctor
```

These tests/commands are committed as source but not executed in this remote
session. This is not a production acceptance or replacement of HQ Bun.


## Attached engine_network prototype — accepted value-add only (2026-11-03)

The attached Qwen `engine_network` implementation was **reviewed, not merged wholesale**.
Only four bounded, std-only utility operations were integrated via
[`UtilityCompute`](src/utility_adapter.rs), using the **existing**
`ComputeEngine`, `EngineRegistry` and `Scheduler`:
`echo`, `upper`, `sum`, `script` (`set/add/concat/emit`).
No alternative graph, registry, TCP server, writable SQL adapter or ephemeral
counter was added. The existing runtime, database read-only guarantees,
permissions and Bun activation gates remain unchanged. See the
[attachment comparison, integration rationale and validation status](ATTACHMENT-VALUE-ADD-2026-11-03.md).

```sh
cargo test --manifest-path research/rust-knowledge-graph/Cargo.toml --test utility_adapter
```

**Source committed, tests not yet executed:** the remote editing runtime has
no installed Rust toolchain; this is not production certification.


## VIVIM web-memory delta journal (Sprint 01 — Lane 1)

**Source status:** implemented for the optional `importer` feature; Rust toolchain
execution and signed-in Windows/Android acceptance remain **unverified** until
the owner runs the commands below. The canonical journal is still the existing
`.hqj` file: `HQJ1` snapshot frames remain readable, and `HQD1` frames
append only new browser observation graph nodes/edges. No second graph, browser
controller, permission database, or Host KB writer is created.

**Write contract:** `vivim.browser-companion/1`, unchanged (up to 32 events,
64 KiB request). Only `visit.observed` with source-sanitized `page.site`,
`page.url`, `page.browserTabId`, event UUID and canonical UTC `at` is
accepted. Repeat event ID plus identical complete canonical fields is a no-op.
A changed digest for the same installation/event ID refuses the **entire** batch
with exit 2. A successful response is printed only after `fsync` of a new
`HQD1` graph delta or verification of an already durable duplicate.

```sh
# From research/rust-knowledge-graph
cargo test --features importer
cargo test --features importer --test web_memory
cargo build --release --features importer --bin hqkg-web-feed

# Existing Studio compatible mode: one JSON batch on stdin
cat visit-batch.json | target/release/hqkg-web-feed ./runtime/web-memory.hqj

# Long-lived mode: one JSON batch per newline, JSON ACK per durable batch
target/release/hqkg-web-feed ./runtime/web-memory.hqj serve

# Read-only contract: nothing sent to browser, model or remote network
target/release/hqkg-web-feed ./runtime/web-memory.hqj query --site example.org --limit 20 --after-cursor 0
target/release/hqkg-web-feed ./runtime/web-memory.hqj query --since 2026-11-04T00:00:00.000Z --until 2026-11-04T23:59:59.999Z --limit 100

# Explicit recovery if doctor reports torn tail; inspect first, never auto-repair
cargo run --bin hqkg-control -- ./runtime/web-memory.hqj doctor
cargo run --bin hqkg-control -- ./runtime/web-memory.hqj repair-tail EXPECTED_EPOCH --apply

# Repeatable synthetic stress (not a live browsing-history import):
HQKG_WEB_BENCH_EVENTS=1000 cargo test --features importer --test web_memory -- --ignored --nocapture
HQKG_WEB_BENCH_EVENTS=10000 cargo test --features importer --test web_memory -- --ignored --nocapture
HQKG_WEB_BENCH_EVENTS=100000 cargo test --release --features importer --test web_memory -- --ignored --nocapture
```

**Query output:** `vivim.web-memory-query/1`, JSON with
`graphEpoch`, `cursor`, `nextCursor`, `hasMore`, and at most 100 `results`.
Each result has `id`, `installationId`, `site`, `url`,
`observedAt`, `provenance`, `source`, `cursor` and `verified`.
The `verified` flag means a digest-bearing observation was recovered from a
checksummed journal frame, **not** an independently authenticated browser event.
Host must still apply its disclosure/privacy policy before sending anything
to an AI provider or mobile view. For legacy visit nodes lacking a complete
digest, the query marks `verified: false` and uses a deterministic fallback
cursor. Pass `nextCursor` to continue reading.

**Performance posture:** incremental frame size is proportional to *new visits*,
not the entire existing graph, and `serve` reuses one in-memory graph across
batches. Cold start and cold read query still replay all frames and scan the
in-memory graph; measured 100k throughput, p50/p95, checkpoint/compaction and
Windows filesystem durability are **not yet certified**. Legacy snapshot
writers still exist; promotion to an always-on service requires G1–G9 gates.

**Crash policy:** an incomplete initial `HQD1` frame or interrupted suffix
is quarantined; new web ingestion refuses it. Operator can inspect
`hqkg-control ... doctor` and explicitly `repair-tail` with the observed
epoch. A stale writer lock must be checked against the process owner before
manual removal; it is never removed automatically. Repeated IDs after a
possibly-persisted/no-ACK attempt are safe to reconcile without resending
provider actions.

