# CLI Surfaces, Integration Graph and First Operational Pass

## Existing CLI surfaces
- HQ umbrella CLI: `repos-hq/hq/bin/hq.mjs`; operations include index/reindex/check, search, KB intake/status/query, context bundles, wiki, verify, routes, capabilities, project administration and MCP stdio.
- Rust graph CLI: `research/rust-knowledge-graph/src/main.rs` → `hqkg`; init/stats/nodes/edges/add-node/add-edge/path/search/rank/db-add/db-query.
- Rust control CLI: `hqkg-control`; doctor, config show/propose/apply via revision CAS and explicit --apply, repair-tail with expected epoch.
- Rust import CLI: `hqkg-import` (optional importer feature).
- Rust web-memory CLI: `hqkg-web-feed` (optional importer; stdin batches, persistent serve, read-only query).
- Rust research CLI: `hqkg-research`.
- Rust dependency CLI: `hqkg-deps`.
- HQ kernel CLI: `repos-hq/hq/lib/kernel/cli.mjs`.
- Other code surfaces requiring exact audit: TierFlow CLI/workflow execution, Visual Runtime editing/programming, Host/Studio/Android commands, SOFTWARE_HQ/OpenCode CLI.

The CLI is an interface to graph-represented operations, not a separate canonical knowledge or engine registry. An eventual unified operator facade may dispatch typed operations to these existing binaries and owners, but is not yet an installed command.

## Existing cross-system seams
| Connection | Existing | Missing |
|---|---|---|
| HQ source → HQ index | Indexer, FTS, schema | Register all sources; measure coverage/freshness |
| Web Harness KB → HQ | Package/KB intake | Durable canonical migration/authority reconciliation |
| HQ semantic → Rust | HQKGX1 exporter and importer | Continuous cursor/delta, source freshness and verified replay |
| HQ source registries → C4 | Identity/crosswalk and catalogue delta | Cross-lane acceptance and Labs live data |
| Rust → DB | DatabaseSpec + DatabaseCompute read adapters | Schema identity, real asset hashes, immutable pinned data |
| Rust → native algorithms | ComputeEngine/EngineRegistry/AlgorithmDriver | Validated installed drivers and lifecycle |
| Rust → Visual UI | Conceptual graph read model | Real typed external-engine adapter and source-linked UI |
| Rust/HQ → TierFlow | Shared KB kernel target | Tested generic client and execution-status seam |
| Host/Studio → Rust/HQ | KB packages and browser observation feed | Actual device/session evidence, privacy gates, reconciled receipts |
| HQ → Labs | C4 snapshot/delta interfaces | Live preprod/prod projections with independent evidence validation |
| Oracle/CLI/UI → same operation | Typed action/capability target | Complete command census, adapter routing, effect boundaries |

## Four operational lanes
1. **Engine census**: identify genuine engines vs libraries/adapters and research candidates. List slot, implementation, runtime, capabilities, source location, status and resource dependencies.
2. **Data and transformation census**: register SQLite stores, KB packages, journals, schemas, datasets, event feeds, inputs, reads, transformations and writes. Tag durable versus rebuildable.
3. **CLI and interface census**: enumerate actual command trees and effects, map commands to common operations and engines, identify duplicate paths.
4. **Integration vertical slice**: source edit → HQ index → curated semantic identity → versioned HQKGX1 export → Rust journal → query through CLI → read-only UI/Labs view with provenance and replay.

One integration owner validates fresh main SHAs, Windows/Bun/Cargo native tests, no duplicate writer/registry, missing source semantics, restart, bad inputs, idempotent reindex and unchanged replay.

## Acceptance targets (not current claims)
- All four designated repos registered and coverage classified (eligible/ignored/missing).
- Every indexed engine/data/CLI operation links to an implementation or explicit research-only status.
- All mutations have declared source-native owner and receipts.
- Unchanged refresh is no-op; changed refresh is observable and revision-pinned.
- Conflicting identities fail closed, no auto-merge or guessed tombstones.
- Rust query and HQ query point to the same source identity after import.
- UI/Labs derived views cite exact source revision, and production state derives from actual Host/device/deployment evidence.
- Recover after restart and stale/partial batches without deleting prior evidence.

## Core placement decision
First use ONE physical Rust graph with MANY EngineSlot, ComputeEngine, Database, Dataset, Fact, Concept, Document and extensible node types. Register external systems by metadata/adapters. Revisit multiple physical Rust graph cores only after a measured necessity for isolation, distribution or scaling, preserving one logical namespace.
