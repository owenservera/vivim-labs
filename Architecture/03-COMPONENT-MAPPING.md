# VIVIM Systems → Rust Graph Conceptual Components

This is a placement map, not a claim that the subsystems already implement a native Rust ComputeEngine or that source writers have been migrated. One physical Rust core / one logical graph is the default direction.

## Full system mapping
| Existing asset | Primary conceptual Rust component | Other graph links | Physical implementation / source |
|---|---|---|---|
| HQ incremental code indexer | Transformation + Engine | Dataset, symbols, dependencies, evidence | repos-hq/hq/lib/indexer.mjs |
| HQ file walking/extraction | Transformation | Input source, symbols, import edges | repos-hq/hq/lib/walk.mjs and extract.mjs |
| HQ full text search | Computation + Engine | Search projection, KB corpus | repos-hq/hq/lib/search.mjs; SQLite FTS5 |
| HQ semantic registry | Semantic knowledge | Aliases, stable IDs, provenance | repos-hq/hq/lib/semantic-registry.mjs |
| HQ KB reader | Semantic knowledge + Computation | Documents, decisions, sessions, notes | repos-hq/hq/lib/kb.mjs |
| HQ context bundles | Knowledge + Transformation | Query, evidence, omission receipts, agents | repos-hq/hq/lib/context.mjs |
| HQ KB package intake | Transformation + Authority reference | Source manifest, validation, proposals | repos-hq/hq/lib/kb-intake.mjs |
| HQ durable kernel store | Persistence + Semantic knowledge | Revisions, entries, types, schemas, flows | repos-hq/hq/lib/kernel/store.mjs |
| HQ git event lake | Data + Transformation | Commits, diffs, tracking | repos-hq/hq/lib/lake.mjs |
| HQ generated wiki | Interfaces + Materialization | Indexed knowledge and source links | repos-hq/hq/lib/wiki.mjs |
| HQ identity/crosswalk | Semantic knowledge + Transformation | Duplicate candidates, source records, conflicts | repos-hq/hq/catalogue-identity |
| HQ C4 snapshots/deltas | Transformation + Observation | Revision digest, replay, evidence | repos-hq/hq/lib/catalogue-delta.mjs |
| Rust GraphCompute | Graph + Computation | Topology, path, PageRank, search | rust-knowledge-graph/src/builtins.rs |
| Rust DatabaseCompute | Data + Computation | SQLite/PostgreSQL resource nodes | rust-knowledge-graph/src/database.rs and builtins.rs |
| Rust algorithms | Engine + Computation | Algorithm definition vs trusted driver | rust-knowledge-graph/src/algorithm_adapter.rs |
| Rust utility engine | Engine + Computation | Script/echo/sum, bounded outputs | rust-knowledge-graph/src/utility_adapter.rs |
| Rust engine registry | Engine network | Slots, active generation, supersedes | rust-knowledge-graph/src/engine.rs |
| Rust scheduler | Orchestration | Task DAG, output receipts | rust-knowledge-graph/src/engine.rs |
| Rust journal | State and persistence | Epoch, frames, delta replay | rust-knowledge-graph/src/lib.rs |
| Rust control/doctor | Control and lifecycle | Config, dependencies, health, quarantine | rust-knowledge-graph/src/control.rs |
| Rust web-memory feed | Transformation + Knowledge + Persistence | Browser observations, site/visit nodes | rust-knowledge-graph/src/web_memory.rs |
| Visual/UI Runtime | Engine + Interfaces | Visual document, renderer, layouts, event links | TierFlow visual runtime; detailed write audit pending |
| UI layout + editing | Engine subcomponents + Transformations | Document revisions, constraints, views | TierFlow/Host; verify exact ownership |
| UI programmability | Interfaces + Semantic knowledge | Typed intent, UI actions, configuration | TierFlow/HQ wiki bridge |
| TierFlow three tiers | Orchestration + Engines | Tasks, dependencies, completion receipts | TierFlow; runtime writer stays there |
| VIVIM Host and Studio | External engines + Authority | Effects, approvals, browser evidence | vivim-web-harness; native write owners |
| Android broker and paired phone | External engine/interface | Platform-specific actions, grants | Host/Android; independent platform boundary |
| Oracle | Orchestration + Knowledge | Plans, delegation, available engine operations | Product-level intended authority; consent bounded |
| HQ CLI | Interfaces + Control | All HQ knowledge and index operations | repos-hq/hq/bin/hq.mjs |
| Rust CLIs | Interfaces + Control | Graph, database, import, doctor | rust-knowledge-graph/src/main.rs and src/bin |
| SOFTWARE_HQ / OpenCode | External agent/runtime engine + Interfaces | CLI, agent sessions, skills and context | Separate project; detailed verification pending |
| VIVIM Labs | Observation + Interfaces | Dev/preprod and production tracker projections | vivim-labs; not production certifier by itself |
| Capability lists and 600+ roadmap | Knowledge + Observation | Goals, implementation, evidence, lifecycle | Existing source registries; avoid new authority |
| Historical conversations/corpus | Data + Knowledge | Session, time, provenance, retrieval | Native sources and KB adapter |
| Schemas, transports, protocols | Knowledge + Data resources | Version constraints and compatibility | Source-owned schema/protocol registries |
| Testing, telemetry, receipts | Observation + Evidence | Test target, engine generation, release | Multiple owners |

## Engines as graph entities
Logical slot → particular implementation generation → source code/binary → offered operations → required schemas and datasets → data stores → produced outputs and evidence. One engine can have sub-engines and adapters. An external Visual Engine or Host engine can be registered descriptively without placing its effects inside native Rust.

## Data as graph entities
A Database node can refer to an entire SQLite/PostgreSQL database. Dataset nodes can refer to files, corpora, snapshots and source-native stores. Document/Fact/Concept nodes encode curated knowledge and observed assertions. Relationships attach input/output lineage and provenance. Large raw data stays in appropriate stores until a deliberate verified migration.

## CLI as graph entities
The CLI command, its stable operation, the engine implementing it, the resources it reads/writes, permission level, receipts and equivalent UI/NL interaction should be discoverable. CLI does not become a second semantic authority.

## Ownership
Repos HQ remains initial implementation home for canonical knowledge and graph compute; TierFlow owns its execution state; VIVIM Host/Studio/Android own privileged effects; the Visual Runtime keeps its canonical document writer; Labs consumes evidence and renders status. Do not rewrite any owner based on a graph relationship alone.
