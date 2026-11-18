# Lane F — independent HVI contract and test plan

**Gate:** PREPARED_NOT_PROMOTED (source-only worker, no G1 integration or production acceptance).
**Worker branch:** `work/p5-hvi-w1-f-verify-r001`. **BASE_SHA:** `f9823a5131e73e7562db5a4ec14e1ec481557d69`.
**S0:** `work/p5-hvi-s0-source-lock-r001@2ef7a9f66419d681e040b459ba4762a63aa9564f`, `GO_TO_PARALLEL`, `WORKER-BASE.json` blob `a3b0602d148b93842481785f65a527df22fc479b`.
**Scope:** `contracts/hq-version-intelligence/**`, `tests/hq-version-intelligence/**`, `proofs/F/**` only.

## Source-truth independent inspection (actual GitHub pinned blobs)
| Source | Commit SHA | Git blob |
|---|---|---|
| ARBOR `hq/lib/catalogue-delta.mjs` | `f9823a5131e73e7562db5a4ec14e1ec481557d69` | `6849e6df26bd136ee50ae08a260d8ab9e9445ac7` |
| ARBOR `hq/lib/semantic-registry.mjs` | `f9823a5131e73e7562db5a4ec14e1ec481557d69` | `ec982381c77147119f62e702a162a203df8e7d4e` |
| ARBOR `hq/lib/arbor-service.mjs` | `f9823a5131e73e7562db5a4ec14e1ec481557d69` | `a47c232c188d69758daa8d6a2f18070ce8c9a27b` |
| ARBOR `hq/test/catalogue-delta.test.mjs` | `f9823a5131e73e7562db5a4ec14e1ec481557d69` | `c377a19a1a5a9b44abc4a1aa00e74d76edf183b9` |
| ARBOR `hq/aii/fixtures/HQ-114-compare-registered-repos.mjs` | `f9823a5131e73e7562db5a4ec14e1ec481557d69` | `800f7119cad59228577b029772bd98592ec9bc2a` |
| repos-hq `hq/catalogue-identity/crosswalk.mjs` | `9b922ea049dbadc94e504b5b9432743995c020ca` | `ed4cb6f561e055e1dd3145b61de6f50f2555687d` |
| repos-hq `hq/catalogue-identity/test/crosswalk.test.mjs` | `9b922ea049dbadc94e504b5b9432743995c020ca` | `3da1b5f7f14affb1f206213453832bf5fe2f8844` |
| repos-hq `hq/catalogue-identity/source-adapters.mjs` | `9b922ea049dbadc94e504b5b9432743995c020ca` | `0100d7ce7ff5cd6b4dbc366ab93a74a1a33e4fed` |
| ARBOR `integration/beta-gates.json` | `f9823a5131e73e7562db5a4ec14e1ec481557d69` | `81255176eaec6728043777ec7d2c62d41cf16c66` |
| ARBOR `integration/source-lock.json` | `f9823a5131e73e7562db5a4ec14e1ec481557d69` | `b19d595c775024d5be5e70f37c20508f02aba676` |

S0 snapshots: ARBOR tree `13eda0d9c7f717a46d00eee46a22b1a19edb65be`; repos-hq tree `a31e926bff898b02273b2044a871881f58ca61a9`; harness `835ea6c1a7e05a73fea5b5fc4c02409a034d4525`; tierflow `2117f5dde23d4769db38e8fdde0b4871051129f7`; labs `22c96f972b31153497c845c12e28efcf912278f9`. Exact commit pins also in `01-SOURCE-PINS.json` and S0 `SOURCE-REVISIONS.json`; original authored ARBOR SHA `370d18d92dfa463f58bd145b307b488a64746ee5` is historical, **not** workerBaseSha.

## Test matrix (all T01–T24)
The Node suite in `tests/hq-version-intelligence/hvi-contract-qa.test.mjs` authors one named test per T-ID. T01–T08 source/pin/offline/entry guard; T09–T16 lineage/census/capability/authority; T17–T24 security, bounded resources, C4 replay, grants, effects, privacy and prompt injection. Structural and conditional schema tests are separate, including owner receipt, runtime-proof, grant success and symlink denials. The actual `hq/lib/catalogue-delta.mjs` is imported for T13 and T20; all other helpers are clearly marked **independent fixture oracle, not A–E production**.

### Execution instructions for G1 (do not mark executed here)
```sh
git rev-parse HEAD
node --version  # require Node 24 per work order
node --test tests/hq-version-intelligence/*.test.mjs
node --test hq/test/catalogue-delta.test.mjs tests/hq-version-intelligence/*.test.mjs
```
Run from actual integrated ARBOR checkout, after A–F have been reconciled by G1. Capture OS, exact branch SHA, Node version, full stdout/stderr and **process exit code**. Do not use a donor checkout. If a standards-compliant Draft 2020-12 JSON Schema validator is already available, validate all nine examples plus conditional-negative cases; do not silently download dependencies.

### Additional G1/G2 matrix (not covered by worker oracle)
G1 must import **actual A/B/C/D/E production modules** for T01–T12 and T15–T20, compare two and 3+ SHAs, verify five-repo and named research pins without implicit HEAD, validate offline replay from ingested manifest and first-run Git source grants, run source/secret scanner, test symlink/submodule/LFS/binary/over-limit handling, assert all 361 missing-path rows are classified individually and any semantic dispositions evidenced. Bench minimum 10k records and record elapsed time, RSS/peak memory, truncation/pagination/low-disk data; no invented threshold success.

G2 owns real Host actor/session/grant read and mid-operation revocation, personal-World isolation, no provider-browser effect, and inert TierFlow view T21–T24. Must produce real Host receipts and report separately from device/Windows/Android and beta gate. HQ-114 is a seeded catalogue fixture, **not** a real N-way comparator; existing C4 source does not establish pin acquisition or runtime integration.

## Exact owner handoff
Z1: inspect committed F branch at exact HEAD, schemas, fixture oracle labels, per-ID mapping, security negatives, raw diff against BASE_SHA. G1: include F in serial integration **after** A–E and reconcile schemas with implemented interfaces; do not add a second registry/graph/scheduler/authority. H/G2: only Host may authorize live read; deny revoked grants, preserve privacy. X: classify 361 source paths, never auto-retire missing-path records.

If Node tests fail or cannot run, mark `REPAIR_REQUIRED` / `G1_SOURCE_STAGED_TESTS_UNRUN` as applicable, **not** `GO_FOR_W2`. Existing `integration/beta-gates.json` remains `BLOCKED_NOT_BETA` and is not modified.
