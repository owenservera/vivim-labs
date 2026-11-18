# ARBOR P5 W1D — HQ intelligence physical migration

**Status:** SOURCE_IMPORTED / RUNTIME_UNVERIFIED (not beta GO).
**Lane:** D (HQ).
**Work branch:** work/p5-w01-d-hq.
**Starting ARBOR commit:** dd245d78ff3c110ba8630cad557ea0479c51a635
(Proposal 5; shared integrate/proposal5-beta-r001 did not exist at start).
**Donor:** owenservera/repos-hq main
9b922ea049dbadc94e504b5b9432743995c020ca, hq tree
48e48313187e3747ac70206276a1a286c94ab002.

## Physically imported (byte-identical to source)

- hq/lib: all 127 operational JS modules, including indexer, schema, registry,
  graph/search/context, semantic IDs, capability source analysis, deterministic
  wiki, freshness/parsing, CLI transitive library and helper submodules.
- hq/bin: all 9 original CLI/support sources including hq.mjs.
- hq/test: 47 selected source regression tests for indexing, search, graph,
  context, capability and source analysis, semantic IDs and wiki. Other tests
  cover unrelated HQ agent/media/automation execution and are not copied.
- hq/capabilities/registry: 2 versioned capability data contracts.
- hq/types: 1 test typing support file.
- hq/registry: 3 HQ canonical identity/taxonomy/binding registries.
- hq/aii/fixtures: 126 source-grounded verification fixtures including shared
  bounded read-only fixture harness (no prior assessment results).
- Import map and exact per-file Git blob identities: source-provenance.json.
- Added package.json, local adapter/diagnostic CLI, isolation tests, README.

**Blob audit:** 315/315 imported source blobs match their original Git SHA;
no copied blob modified. Native research/rust-knowledge-graph is excluded
because Lane A owns native/hqkg. Donor data/.hq index, session dumps, secrets,
profiles, work logs, generated artifacts, node_modules and caches excluded.

## New internal boundary (not an external bridge)

The ARBOR-owned module hq/lib/arbor-service.mjs exposes the retained HQ
algorithms *in-process*, plus ARBOR-local data path and approvedRoots guard.
It does not spawn or import from repos-hq. Dev source roots are an explicit
Host-granted set (or explicit CLI --allow); empty list means no source access.
Index, wiki and context data are stored under a per-user ARBOR development
data root; no separate HQ server, external DB or checkout dependency.

The compatibility wrapper is **not yet mounted into Host or native IPC**;
that is W2/G integration work. Do not mark it RUNTIME_WIRED or AVAILABLE_NOW.

## Requirements / tests / current proof

- Node 24+ with node:sqlite and FTS5 (donor requirement); SQLite on local disk.
- Git installed for revision metadata; local developer sources only if
  selected/authorized, and writable per-user ARBOR data directory.
- Bun 1.4.x for donor's optional Bun-specific cases/aggregate test work.
- No root package/lockfile changes by Lane D (reserved for G).
- No production npm dependency added by this lane.
- Performed: Git donor-versus-ARBOR staged-tree SHA comparison, 186/186 equal.
- **Not executed:** node/bun test, Node syntax check, live indexing, runtime
  Host/native IPC, Windows packaging, donor-offline smoke or device testing.
  GitHub connector grants repository read/write but not a Node/Bun execution
  environment; these runtime checks remain release gates.
- Run: cd hq && node --test test/arbor-service.test.mjs test/in-repo-closure.test.mjs
- Run: cd hq && node --test test/indexer.test.mjs test/search.test.mjs test/graph.test.mjs test/context.test.mjs test/wiki.test.mjs test/semantic-registry.test.mjs
- Run: cd hq && node --check lib/arbor-service.mjs && node --check bin/arbor-hq.mjs

## Hand-offs (blocking until exercised)

**To G (integration):** add hq to root Bun workspace and source-lock/release
manifest; run Node test suite with runtime versions; ensure hq runtime JS/CLI
is included in the installed beta; verify build with donor repos absent.
Reconcile selected test fixtures and other cross-repo test references before
enabling all 47 legacy tests in CI. Do not overwrite concurrent lane files.

**To B (Host):** pass only Host-authorized dev roots and Host-selected
per-user dataDir to createDevIntelligence; enforce actor/consent checks
before read or rebuild; do not expose legacy all-purpose HQ CLI. Route any
effects through existing Host admission and receipts; HQ has no effect writer.

**To A (Rust):** use indexWatermarks() and sourceEvidence() read contracts:
arbor.dev-source-evidence/1 {sourceId, projectRevision, indexWatermark,
indexedHash, lineStart, lineEnd, text}. Semantic registry stays source-owned.
Feed authorized DevSource snapshots into native/hqkg Development namespace.
Never introduce another HQKG crate/database or read Host PersonalSource
without separately authorized Host scope.

**Risk/review:** Source-preserving CLI contains legacy optional operations
not intended for beta UI. New safe adapter must be exercised under real
Node24, on Windows with per-user storage ACLs, stale/corrupt state, revoke,
symlink escape, and low-disk conditions. Imported wiki and graph source
algorithms still require cross-surface contract verification.
