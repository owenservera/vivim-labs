# VIVIM Architecture — One Graph, Many Engines
Date: 2026-11-07
Status: architectural working record; not a claim of implemented integration
Scope: Repos HQ, Rust HQKG, TierFlow / Visual Runtime, VIVIM Web Harness / Host / Studio, VIVIM Labs, Oracle and CLI surfaces.

## North star
**Everything is represented in one logical graph.** Rust HQKG is the candidate central graph, engine, data-resource and compute kernel. All engines, data stores, datasets, transformations, CLI operations, schemas, capabilities, jobs, state, history, evidence and relationships should be addressable in the graph. External processes and database bytes need not physically move into Rust; source systems retain their native write authority until verified migration. Graph representation never confers execution permission.

**Decision direction:** begin with one physical Rust core and one logical graph, many engine and data nodes, and external adapters. Leave open later federation into multiple physical Rust cores under one logical namespace if measured requirements justify it. Do not build a second competing engine registry, graph, or knowledge owner.

## Contents
- [01 — Architecture Vision](01-VISION-AND-CORE-DECISIONS.md): why one graph, one vs multiple physical cores and ownership rules.
- [02 — Rust Core Anatomy](02-RUST-CORE-ANATOMY.md): actual implemented node kinds, engine slots, database nodes, computation, journal, control, and limits.
- [03 — System Mapping](03-COMPONENT-MAPPING.md): all discovered VIVIM systems mapped into conceptual Rust components.
- [04 — Dataflow and Write Authority](04-DATAFLOWS-AND-WRITES.md): sources, transformations, outputs, persistence, change propagation and writer matrix.
- [05 — CLI and Integration](05-CLI-AND-INTEGRATION.md): existing command surfaces, integration edges, first vertical slice and acceptance.
- [06 — Source Evidence](06-SOURCES-AND-OPEN-QUESTIONS.md): original repo paths, evidence boundaries, unresolved questions.
- [07 — Integration Philosophy](07-INTEGRATION-PHILOSOPHY.md): each step accelerates the next; distinct dev and product configurations.
- [08 — Integration Sequence](08-INTEGRATION-SEQUENCE.md): proposed staged implementation and tests.
- [09 — Unified HQ Intelligence, Commands and Wiki](09-UNIFIED-INTELLIGENCE-COMMAND-WIKI.md): correction: indexing is one part of the combined NL/command/programmatic-wiki system.

These are curated discussion notes from the November 7 architecture conversation, not a verbatim transcript or an approved schema. Preserve repository AGENTS.md, contracts, executable configs, tests and local operational docs in their native owners. Nothing here is authority to migrate or delete source files.

## Summary of the evolution
1. Reuse the full Repos HQ source intelligence → semantic command/NL routing → generated and programmable wiki → CLI/MCP loop, not only its incremental indexer.
2. View VIVIM as a network of engines (Rust graph, visual runtime, indexer, KB, Host, TierFlow, Oracle, orchestration).
3. Discover Rust already provides engine slots, executable generations, database nodes, algorithm adapters, scheduler, journal and control primitives.
4. Map inputs → transformations → outputs and writes; separate ephemeral index, durable KB, Rust journal and source-native ledgers.
5. Recenter everything on the existing Rust graph model: engine, data, transformation, capability, command and evidence are nodes/edges/operations.
6. Operationalize one bounded end-to-end path before splitting into multiple physical cores.

No changes to Rust core, indexer or other repositories were made as part of writing this record.
