# Rust HQKG — Existing Construction and Conceptual Components

## Observed Rust implementation
Repository: `repos-hq/research/rust-knowledge-graph`.
- `src/lib.rs`: Graph, stable node IDs, typed weighted directed edges, revisions, adjacency indexes, traversal, PageRank, serialization, journal, engine-swap lineage.
- `NodeKind`: Fact, Concept, Document, Dataset, ComputeEngine, EngineSlot, Database, Other(String). Other is an extensible kind, not grounds for an unrelated shadow graph.
- `src/engine.rs`: ComputeEngine trait, EngineDescriptor {slot, implementation, interface, capabilities}, EngineRegistry, EngineLease, generational CAS install, Scheduler with dependent tasks, budgets and receipts.
- `src/builtins.rs`: GraphCompute (path/neighbors/reachable/topological/rank/search/stats) and DatabaseCompute (query/catalog/health).
- `src/database.rs`: DatabaseSpec representing whole SQLite/PostgreSQL databases as graph nodes, feature-gated bounded read-only query adapters; secrets referenced by environment-variable name, not persisted.
- `src/algorithm_adapter.rs`: 30 research algorithm identities can be seeded as *inert* nodes; trusted AlgorithmDriver implementations alone become executable engines.
- `src/utility_adapter.rs`: bounded utility compute operations, reusing existing engine registry and scheduler.
- `src/control.rs`: typed dependency resources (Engine, Database, Dataset, Capability, Schema, Other), version range and digest checks, configuration CAS, telemetry, health/quarantine and constrained cache repair.
- `src/import.rs`: optional validated HQKGX1 JSONL import with SHA-256, staging/conflict checks and journal snapshot.
- `src/web_memory.rs`: sanitized browser-observation ingest, event dedupe, additive journal delta and readback.
- `src/main.rs`, `src/bin/hqkg-*.rs`: dedicated Rust CLI commands.

## Native engine model
An EngineSlot is the logical address. A ComputeEngine generation is the active implementation, pinned by EngineLease for in-flight tasks. A CAS replacement retains old generation leases and rejects loss of interface/capability compatibility. The graph records generation nodes, an active_engine edge and supersedes lineage. The actual executable objects remain in process and must be reinstalled on restart.

A research-method node is distinct from an executable generation; merely registering a node does not run arbitrary code. External OS/browser/visual engines should initially be modeled as engine nodes with external adapters, not falsely claimed as native ComputeEngine implementations.

## Persistence model
The `.hqj` journal is single-writer. Most graph writes are full snapshot frames; the web-memory path uses additive HQD1 frames. Writes require locks and replay validation; torn-tail repair is explicit. The journal stores topology/metadata, not executable engines or full external DB contents. Engine registry installation, lineage update and journal save are not one crash-atomic transaction.

## Twelve conceptual components and maturity
| Component | Current underpinning | Conceptual expansion |
|---|---|---|
| 1 Graph substrate | Node, Edge, Graph, traversal | Global address space, cross-system IDs |
| 2 Semantic knowledge | Fact/Concept/Document, provenance | Curated KB, claims, contradictions, temporal facts |
| 3 Data resource network | Dataset, Database, DB adapters | Native stores, corpora, streams, schema/model resources |
| 4 Transformation network | Import, web-memory transformations, utility adapters | Parse, normalize, dedupe, enrichment, projection DAGs |
| 5 Engine network | EngineSlot, ComputeEngine, EngineRegistry, generations | External Visual/Host/TierFlow/indexer engine descriptors |
| 6 Computation | GraphCompute, DatabaseCompute, AlgorithmDriver, UtilityCompute | Search, rank, planned research drivers |
| 7 Orchestration | Scheduler, dependency DAG | Multi-process workflows, agent scheduling, event coordination |
| 8 State/persistence | Journal, epoch, deltas, recovery | Incremental data at scale, durable cross-store projection cursors |
| 9 Control/lifecycle | CoreConfig, dependency resolver, telemetry, health | Supervisor, external engine lifecycle |
| 10 Authority/provenance | Sources, dependency checks, evidence refs | Owner grants and receipts as references, no bypass |
| 11 Interfaces | hqkg/hqkg-control/import/web-feed binaries | Shared capability bindings to CLI/UI/NL |
| 12 Observation/evolution | Telemetry, health, receipts, graph history | Testing, verification, compounding learning |

## Critical limitations
- Rust core remains prototype/research relative to operational HQ Bun/SQLite; native acceptance unverified in reviewed sources.
- Scheduler synchronous, cannot preempt hung native drivers.
- In-process drivers are trusted, not sandboxed; no universal external process/plugin runtime.
- Current engine requests/outputs are string-shaped, not generic structured typed cross-process interfaces.
- Snapshot-based journal and lack of crash-atomic engine-install+save are scaling and recovery constraints.
- Database adapters are bounded read-only and intentionally not KB canonical writers.
- No multi-writer distributed consensus or verified federation.
- Rust research algorithms, language resources, model weights and paper implementations remain inert until separately installed and verified.

## Original files
https://github.com/owenservera/repos-hq/tree/main/research/rust-knowledge-graph
https://github.com/owenservera/repos-hq/blob/main/research/graph-kernel-integration/CORE-RUNTIME-PROTOCOL.md
