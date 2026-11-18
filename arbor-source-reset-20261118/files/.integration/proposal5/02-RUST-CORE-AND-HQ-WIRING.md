# Proposal 5 / 02 — Internal Rust Core and HQ Intelligence Migration

**Hard rule:** no runtime, build, developer-test or packaging requirement on an external `repos-hq` checkout or HQ service. Selected HQ *code* must be physically imported into ARBOR, versioned and integrated.

## A. Two HQ products, both move internally

1. **HQKG native computation:** `repos-hq/research/rust-knowledge-graph/` → `ARBOR/native/hqkg/`. Preserve the original `hq-knowledge-graph` Cargo package/library API (`hqkg`), graph algorithms, typed DAG, engine registry, generation leases, bounded scheduler, change sets, importer, invalidation, namespaces and tests.
2. **HQ operational development intelligence:** selected `repos-hq/hq/bin/`, `hq/lib/` and dependencies/tests → `ARBOR/hq/`. Preserve existing code indexing, source search, semantic IDs, capability tracking, deterministic context bundles, source-linked incremental wiki, project/registry handling and inspection CLI. Make it an ARBOR internal runtime package and diagnostic capability, not a call to `repos-hq/hq/bin/hq.mjs` through another repo.

HQ source *data* does not migrate indiscriminately. ARBOR's internal HQ package starts with its own local developer-intelligence index of user-selected repositories, separate from owner-private personal World. It must not import the developer's historical HQ database, personal conversations, session files or credentials into Git or installation images.

## B. Native crate migration, source-preserving

1. Pin HQ donor commit and subtree digest; include branch/commit/tree/checksum in `integration/source-lock.json`. Baseline observed in this round: HQ main `9b922ea049dbadc94e504b5b9432743995c020ca`; refresh at actual import.
2. Inspect unmerged HQ graph branches and reconcile existing tested source. Do not assume Z.ai R001/R002 was integrated just because one document uses that label.
3. Import `research/rust-knowledge-graph/` as `native/hqkg/`, preserving public `hqkg` API and unit tests; exclude `target/`, private DB files, caches and generated binaries.
4. Add to ARBOR Cargo root. Move effective build profiles to workspace root; HQ main source notes `debug = 0`, `incremental = false` to avoid multi-GB target caches. Keep the storage-conscious toolchain configuration.
5. Run pure donor tests before changes; run the same after import, then verify typed DAG namespace isolation, generation leases, stale/revoked inputs, tombstones, import idempotency and deterministic transforms.
6. Expose a minimal typed `SourceRef/GraphRequest/GraphEvidence` adapter through `native/protocol`; no new graph store, engine framework or native daemon.
7. Once integrated, ARBOR's imported `native/hqkg` is the single **product canonical native implementation**. The old HQ research path is historical source provenance, not a second live product graph. For future HQ development, deliberately sync/extract changes as audited code promotions, not runtime linkage or dual-writer database coupling.

**If native graph code cannot compile in ARBOR, the internal-integration gate fails.** A temporary external pinned Git dependency would violate this owner's no-outside-wiring requirement and is not an acceptable beta shortcut.

## C. Operational HQ code import, not network bridges

Import the audited transitive dependency closure of:
- `hq/bin/hq.mjs` (source-level command/index/export interface);
- `hq/lib/indexer.mjs`, `schema.mjs`, `graph.mjs`, `registry.mjs`, `search.mjs`;
- `semantic-registry.mjs`, `capabilities.mjs` and related deterministic source ID/contract logic;
- `context.mjs`, `wiki.mjs`, `wiki-program.mjs`, freshness checks and required source parsers;
- supporting tests and metadata; only as required for a source-preserving working package.

**Do not assume the above is a complete dependency list.** Generate the actual import/call graph from donor source and include required modules. The `hq/bin/hq.mjs` entrypoint itself is large and multi-purpose; its read-only and write/config commands require an internal capability boundary. We do **not** blindly expose every HQ CLI option to end users.

ARBOR now runs its own internal HQ algorithms and indexing code. It may read the user's local Git repositories as **content sources** if selected/authorized. It must never need another HQ process, external developer SQLite file or per-user environment path to operate.

### Development and personal graph data remain distinct

- `DevSource`: internal HQ index and repository/code/documentation facts, with source SHA/index watermark. Owner-selectable project roots, data-class authorization and storage cap.
- `PersonalSource`: Host World/private corpus, consent and revocation; existing Host writer is authoritative.
- `SharedCompute`: one HQKG native algorithm implementation that can process either scoped source domain after authorization, using separate graph namespaces and caches.

The fact that the *code* is internal does not grant one domain unlimited access to another. The internal capability contract enforces boundaries; a source cross-reference is an authenticated, permissioned read operation.

## D. Exact internal dataflow

```text
User-selected local Git/project folder ──► ARBOR/hq/indexer (internal Bun/SQLite)
                                                │
                           search / source revisions / semantic registry /
                           context assembler / deterministic dev wiki
                                                │
                                    ARBOR native protocol
                                                │
                         ARBOR/native/hqkg graph engine (Development domain)
                                                │
                     ARBOR Node0 source-linked interpretation and view projection
                                                │
                    ARBOR bundled TierFlow UI / CLI / AutoWiki / project screen

Host World source (internally owned) ─► authorized personal source read adapter
                                                │
                         SAME ARBOR native/hqkg computation (Personal domain)
                                                │
                   DCB candidate / owner personal view / provenance / question
                                                │
                   Host effect/writer only on separately committed authorized acts
```

No arrow crosses into an HQ/TierFlow/Harness development process or repository. All code flows inside ARBOR packages and current app processes.

## E. Internal integration ports: minimal and typed

- **Development index read:** source identity, index watermark, selected path/line spans, search/graph evidence, stale/unavailable statuses.
- **Context read:** task/source set, byte/token budget, deterministic source excerpt ranks and exact source refs.
- **Wiki read:** source-linked wiki projection, as-of watermark, no separate authoritative wiki data store.
- **Graph computation:** namespace, operation/engine generation, source snapshot vector, bounded resource budget, typed results.
- **Capability introspection:** source-owned action/operation identifiers, implementation pointers and actual build/runtime proof.

These are **in-process/native or internal service interfaces**, not HTTP calls to separately checked-out repositories. The first version can use versioned JSON over an authenticated loopback/child IPC boundary between native Tauri and the internal JS Host/HQ modules, with exact access restrictions. Generated Rust↔TS runtime validators prevent silent schema drift.

## F. Internal consistency tests (must pass)

1. Rename/remove all donor checkouts and block any access to them; ARBOR still builds, installs, starts and runs indexing, graph queries and UI.
2. Stop all old donor background processes; internal source index and native graph work.
3. Turn off network access except local test resources: pure HQ and Rust computation still works; provider webapp operations degrade honestly.
4. Deny a personal-source scope; HQ developer intelligence cannot read that World data.
5. Run original HQ indexing/context/wiki golden fixtures and original HQKG native tests from *ARBOR internal paths*.
6. Same source project observed at one revision gives consistent CLI/Node0/TierFlow source IDs and honest as-of index freshness.
7. Import/revision/erasure and low-storage compaction do not silently lose protected owner facts or effect receipts.
8. No `../repos-hq` import, named external HQ CLI, Git submodule, outside SQLite path or cross-repo runtime API remains in the ARBOR dependency graph.
9. Host source and personal namespace revocation causes dependent materialized graph/context/wiki invalidation.
10. One release manifest pins internal HQ JS version, native hqkg source tree, Node0/Host/TierFlow protocol versions and compatible data migrations.

## G. What does **not** happen during beta

No rewrite of HQ indexing into Rust, no new dev graph DB, no independent HQ web server, no copied developer's live HQ index, no conflation of development Wiki with personal AutoWiki, no requirement to optimize all algorithms before building the beta.

**Beta GO requires an internally operating HQ+Rust product, not an externally wired ARBOR facade.**
