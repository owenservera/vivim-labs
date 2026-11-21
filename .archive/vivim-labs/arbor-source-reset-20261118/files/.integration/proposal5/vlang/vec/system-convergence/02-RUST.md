# Rust core deep research — two crates, distinct responsibilities

## HQKG: graph/revision/compute
The imported `native/hqkg` crate is real Rust source with graph algorithms, journal, generational `ComputeEngine` slots and leases, bounded scheduler, typed DAG, scoped Development/Personal namespaces, source watermarks, incremental invalidation, change sets/tombstones, metadata-only durable computation descriptors, diagnostics and optional database/importer adapters. `algorithm_adapter.rs` catalogues **30 research algorithm IDs**, not 30 confirmed installed engines.

`typed_dag.rs` advertises a PURE_INTERFACE contract **inside trusted Rust**, not a sandbox. A native engine implementation can still perform OS I/O if given code privileges. For future untrusted or model-produced Rust capsules, use isolated pinned builds, an artifact signature/review chain and an actual execution sandbox with fuel/memory/time ceilings and no ambient file/network/process/credential interfaces. EngineSlot hot-swap remains appropriate for reviewed *trusted* implementations and gradual rollback, not arbitrary comment code.

`scoped_graph.rs` checks provenance shape, not identity authenticity. The Host must authenticate source/owner/session before providing data. `web_memory.rs` models scoped observations but does not control browsers. Durable DAG stores redacted descriptions, not output payloads. Source import is optional and data-only, with size limits and tombstones; it is not external donor comparison authorization.

## Product-native Rust
`core/programmability` defines `vivim.native-program/1`: bounded `Program`, `TransformStep`, source revisions/digests, risk/data sensitivity, a strict JSON wire and metadata-only lineage. It has no timers, grants, storage, effect executor, browser sender or UI. The currently written `GRAPH-INTEGRATION.md` describes source-level Tauri wiring but **Node Host/native ABI and Android paths lack accepted end-to-end proof**.

## Proposed division
- **Product Rust core:** small portable deterministic validator for user-program candidates and typed transformation IR, called behind an authenticated Host native boundary.
- **HQKG:** bounded pure graph algorithms, cache/invalidation and optional context ranking reused when data scopes are explicitly supplied and current.
- **VEC capsule runtime:** reviewed, precompiled Rust SDK transformations in isolated sandbox for untrusted source, preferably built once and referenced by immutable ID. Not compiled per utterance.
- **Host:** only effect, grant, schedule and journal authority. Accepting a Rust plan is never authorization.

## Performance targets to measure, not claim
Source/context BFS, ordered DAG execution, graph deltas, ingestion normalization, coded UI diff, repeated alias parsing and bounded immutable context. Classify workloads as ALWAYS_ON, HIGH_FREQUENCY, FREQUENT, USER_DEFINED or DEV_ADMIN. The existing [Rescore](https://github.com/owenservera/ARBOR/tree/main/.integration/proposal5/rescore-expansion) and [Optimization Observatory](https://github.com/owenservera/ARBOR/tree/main/.integration/proposal5/hq-version-intelligence/vision-lab/optimizations) already map hundreds of targets; shortlist **3–5 measured pure kernels** after product proof, rather than duplicating the 1000-row backlog.

`native/hqkg/Cargo.toml` records `[profile.dev] debug=0, incremental=false` to reduce historic multi-GB target churn. This is source rationale, **not current disk measurement**.

## Rust source atlas
| Inspected source | Finding | Evidence |
|---|---|---|
| [lib.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/lib.rs) | Graph, std-first algorithms and durable journal | SOURCE_PRESENT_NATIVE_UNACCEPTED |
| [engine.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/engine.rs) | ComputeEngine registry, generation leases, scheduler/budgets | SOURCE_PRESENT_NATIVE_UNACCEPTED |
| [typed_dag.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/typed_dag.rs) | Pure typed DAG, namespaces and cache; not a security sandbox | SOURCE_PRESENT_NATIVE_UNACCEPTED |
| [scoped_graph.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/scoped_graph.rs) | Development/Personal namespace; provenance not authenticated | SOURCE_PRESENT_NATIVE_UNACCEPTED |
| [durable_dag.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/durable_dag.rs) | Redacted durable run descriptions, not cached results | SOURCE_PRESENT_NATIVE_UNACCEPTED |
| [context_retrieval.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/context_retrieval.rs) | Bounded deterministic BFS and context ranking | SOURCE_PRESENT_NATIVE_UNACCEPTED |
| [invalidate.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/invalidate.rs) | Watermark+epoch cache invalidation | SOURCE_PRESENT_NATIVE_UNACCEPTED |
| [import.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/import.rs) | HQKGX1 bounded JSONL source exchange | SOURCE_PRESENT_OPTIONAL_FEATURE |
| [graph_change_set.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/graph_change_set.rs) | Source graph deltas/tombstones/merge lineage | SOURCE_PRESENT_NATIVE_UNACCEPTED |
| [apply_ledger.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/apply_ledger.rs) | Durable replay idempotency, torn-tail/lock behavior | SOURCE_PRESENT_NATIVE_UNACCEPTED |
| [reconcile.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/reconcile.rs) | Read-only exact duplicate/orphan candidate plans | SOURCE_PRESENT_NATIVE_UNACCEPTED |
| [control.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/control.rs) | Dependencies, diagnostic engine control and bounded recovery | SOURCE_PRESENT_NATIVE_UNACCEPTED |
| [observability.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/observability.rs) | Evidence-grade diagnostics, not a live Host observer | SOURCE_PRESENT_NATIVE_UNACCEPTED |
| [web_memory.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/web_memory.rs) | Scoped web observation journal and revocation | SOURCE_PRESENT_OPTIONAL_FEATURE |
| [algorithm_adapter.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/algorithm_adapter.rs) | 30 algorithm descriptor entries, not 30 active engines | SOURCE_PRESENT_RESEARCH |
| [database.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/src/database.rs) | DB node references + optional read-only drivers | SOURCE_PRESENT_OPTIONAL_FEATURE |
| [Cargo.toml](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/Cargo.toml) | std-only default, optional features, disk-conscious dev profile | SOURCE_PRESENT |
| [typed_dag.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/tests/typed_dag.rs) | Synthetic Rust namespace/cache tests authored but not run in this review | TEST_SOURCE_UNRUN |
| [import.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/tests/import.rs) | Importer/tombstone tests authored | TEST_SOURCE_UNRUN |
| [keeper_recovery_matrix.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/native/hqkg/tests/keeper_recovery_matrix.rs) | Durable recovery characterization tests authored | TEST_SOURCE_UNRUN |
| [lib.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/core/programmability/src/lib.rs) | vivim.native-program/1 validation, risk/data classes, no effects | SOURCE_PRESENT_PURE_UNACCEPTED |
| [wire.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/core/programmability/src/wire.rs) | Strict JSON adapter; supplied policy not authenticated | SOURCE_PRESENT_PURE_UNACCEPTED |
| [lineage.rs](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/core/programmability/src/lineage.rs) | Opaque metadata-only DAG lineage | SOURCE_PRESENT_PURE_UNACCEPTED |
| [GRAPH-INTEGRATION.md](https://github.com/owenservera/ARBOR/blob/9b79d732da8b92713d399040db4a02d505bc7cf1/core/programmability/GRAPH-INTEGRATION.md) | Tauri source path yes, Node Host/Android accepted execution no | SOURCE_REPORT |

**Tests authored, not run here:** HQKG Cargo core/optional importer, typed DAG, tombstone and recovery matrices; Product Rust Cargo tests; Node Host/Windows child and physical PWA parity. No test-exit evidence or product GO was observed.