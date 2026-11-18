# HVI mission and measured source baseline — 2026-11-12
**Destination:** ARBOR `main@370d18d92dfa463f58bd145b307b488a64746ee5` when plan authored. Check actual main before branch creation/merge. **Verification method:** connected GitHub `git/trees/main?recursive=1`, blob SHA comparisons. Not Node/Cargo/product runtime tests.

## Why
Original HQ correctly preserved its core indexing/source code but not the entire surrounding analytical/agentic HQ. The retained `hq/lib/catalogue-delta.mjs` offers two-snapshot digests and replay, while `HQ-114-compare-registered-repos.mjs` exercises a **seeded** overlap classifier, not real N-way snapshots. Original `repos-hq/hq/catalogue-identity` supports identity crosswalk/orphan evidence but was omitted. `hq/lib/arbor-service.mjs` has no N-way compare API and Host mount is not accepted.

## Measured tracked trees (not capabilities)
| Repository | Main SHA | Tracked blob files |
|---|---|---:|
| ARBOR | `370d18d92dfa463f58bd145b307b488a64746ee5` | 1474 |
| repos-hq | `9b922ea049dbadc94e504b5b9432743995c020ca` | 2024 |
| vivim-web-harness | `cae97afe848d79957c9d6c698a90996a758bfd56` | 1712 |
| tierflow | `50822f11ee63c3d80f631f8fea7dcb9b19bffc38` | 637 |
| vivim-labs | `022608f5797e7b4a14ccc4c393bd3c5a4704a4be` | 885 |

## Direct-path HQ parity
- Original `repos-hq/hq/**`: **676** tracked blobs; ARBOR same path: **315**; byte-identical Git blobs: **314**; source-patched same path: **1** (`hq/lib/walk.mjs`); not present at original path: **361**. Zero of the 361 share an identical Git blob at another ARBOR path in the inspected main tree. This is NOT a semantic relocation audit.
- Absent original HQ path families: `hq/test` 144, `hq/live-graph` 75, `hq/convergence` 38, `hq/wave1-rust-graph` 29, `hq/work` 25, `hq/aii` 21, `hq/catalogue-identity` 14, `hq/audits` 6, `hq/supervisor` 4, and 5 scattered files. **196** of 361 absent paths are JS/TS/Rust/Python extensions according to this census (all 196 are .mjs).
- Original W1D documents `315` selected donor files (127 JS operational files under `hq/lib`, nine CLI, 47 tests, 126 AII fixtures, three registries, two capability contracts, one typing support). That was a **selective** migration by design.

## Known source-only vs runtime
- `hq/lib/catalogue-delta.mjs`: `buildCatalogueSnapshot`, `diffCatalogueSnapshots`, `replayCatalogueDelta` — **existing source**; not N-way repo comparer.
- `hq/lib/git-context.mjs`: `detectRepositoryDuplicates` groups checkout/remote identities; does not compare capabilities.
- `hq/lib/semantic-registry.mjs`: versioned semantic identities, source link drift; not equivalent to actual source version pins.
- `hq/lib/arbor-service.mjs`: read-only HQ development adapter and source evidence; no N-way API, no evidenced accepted Host mount.
- `repos-hq/hq/catalogue-identity`: implemented source-only identity/orphan crosswalk, not imported.
- PG00–PG03/PGZ, SWC, visual, PWA, research preparation: review source/contract and exact branch but **PREPARED_NOT_PROMOTED**.
- Main consolidation: source merged, **Cargo.lock / bun.lock absent; CI startup failure reported; Windows/Android/security checks unrun**. HVI is NOT permission to bypass those release gates.

## First target
Pin five main commits plus the named Labs/HQ research branches in `01-SOURCE-PINS.json`. Reproduce the 676/315/314/1/361 HQ result. Then produce all-file, logical-capability, runtime-evidence, test/provenance and migration-ownership matrices for all five repositories, with transparent unknowns.
