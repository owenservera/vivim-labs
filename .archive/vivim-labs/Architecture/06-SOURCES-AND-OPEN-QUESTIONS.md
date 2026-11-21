# Source Evidence, Status and Open Questions

## Primary inspected code and architecture
- [Rust HQKG crate](https://github.com/owenservera/repos-hq/tree/main/research/rust-knowledge-graph): `src/lib.rs`, `engine.rs`, `database.rs`, `builtins.rs`, `algorithm_adapter.rs`, `utility_adapter.rs`, `control.rs`, `import.rs`, `web_memory.rs`, `src/main.rs`, `src/bin/hqkg-control.rs`, `src/bin/hqkg-web-feed.rs`, `Cargo.toml`, `README.md`.
- [Rust core runtime protocol](https://github.com/owenservera/repos-hq/blob/main/research/graph-kernel-integration/CORE-RUNTIME-PROTOCOL.md).
- [Graph research and unified export](https://github.com/owenservera/repos-hq/tree/main/research/graph-kernel-integration).
- [HQ indexer](https://github.com/owenservera/repos-hq/blob/main/hq/lib/indexer.mjs), [extractor](https://github.com/owenservera/repos-hq/blob/main/hq/lib/extract.mjs), [graph](https://github.com/owenservera/repos-hq/blob/main/hq/lib/graph.mjs), [index schema](https://github.com/owenservera/repos-hq/blob/main/hq/lib/schema.mjs).
- [HQ KB read model](https://github.com/owenservera/repos-hq/blob/main/hq/lib/kb.mjs), [KB intake](https://github.com/owenservera/repos-hq/blob/main/hq/lib/kb-intake.mjs), [durable kernel store](https://github.com/owenservera/repos-hq/blob/main/hq/lib/kernel/store.mjs).
- [KB link](https://github.com/owenservera/repos-hq/blob/main/docs/KB-LINK.md), [Shared KB kernel decision](https://github.com/owenservera/repos-hq/blob/main/docs/SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md).
- [C4 identity projection](https://github.com/owenservera/repos-hq/tree/main/hq/catalogue-identity), [C4 delta](https://github.com/owenservera/repos-hq/blob/main/hq/lib/catalogue-delta.mjs), [C4 integration handoff](https://github.com/owenservera/repos-hq/blob/main/docs/integration/C4-LANE-C-HANDOFF.md).
- [HQ CLI](https://github.com/owenservera/repos-hq/blob/main/hq/bin/hq.mjs), [HQ wiki](https://github.com/owenservera/repos-hq/blob/main/hq/lib/wiki.mjs), [Git lake](https://github.com/owenservera/repos-hq/blob/main/hq/lib/lake.mjs).
- [VIVIM Labs capability observatory](https://github.com/owenservera/vivim-labs/tree/main/capability-observatory).
- [TierFlow visual runtime](https://github.com/owenservera/tierflow/tree/main/VIVIM-VISUAL-RUNTIME).
- [VIVIM Web Harness](https://github.com/owenservera/vivim-web-harness).

## Evidence categories
**Observed code**: modules, node kinds, engine registry, DB nodes, journal, KB intake, indexer, CLI paths and C4 projection source were inspected from GitHub.
**Documented target**: unified KB kernel, universal interface mapping, one-graph visualization, external-engine adapters, continuous end-to-end refresh and federated future.
**Not validated**: local Windows/Cargo tests, running live services, actual graph corpus population, device browser feed, cross-repo propagation, production release, TierFlow/Host writer specifics.
Never convert a documented target or research-only node into an implemented or production claim.

## Unresolved decisions / investigations
1. Where should canonical curated KB revisions live as physical storage while represented in Rust, and how do we migrate without competing writers?
2. How can HQ indexes, kernel SQLite and Rust journal exchange stable revisioned deltas continuously without two graph authorities?
3. What minimum external-engine adapter should represent Visual Runtime, TierFlow and Host without converting them into unsafe in-process Rust plugins?
4. What are the full Visual document, UI programming and Host write boundaries?
5. What is the exact all-CLI operation inventory, and how do commands map to one graph capability identity?
6. How should revocation, deletion, secrets, sensitivity and personal-data minimization propagate across derived graph projections?
7. How do we rebuild or compact the Rust journal at scale, and reconcile a crash after engine hot swap but before lineage save?
8. Should multi-core federation ever be introduced? Default no until verified load/isolation reasons.
9. Which source-registered algorithm definitions are actually executable and tested?
10. What receipt and source-version evidence is necessary for Labs to present preprod/prod state?

## Note on conversational completeness
These files preserve the substantive architecture discussion and tables as structured notes rather than a literal verbatim transcript. The original discussion remains the detailed rationale. No file movement, runtime cutover, core refactor or live deployment is authorized by this record.
