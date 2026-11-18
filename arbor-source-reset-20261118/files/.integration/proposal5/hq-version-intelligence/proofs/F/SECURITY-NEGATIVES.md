# HVI F — independent security negatives and integration risks

**Source-only fixture oracles are not product security acceptance.** No signed-in Chrome, user World, provider API, personal corpus, session token, file-system write, new database, effect or scheduler was exercised. This folder contains no credential contents.

| Threat / negative case | F evidence | Required next owner |
|---|---|---|
| Branch ref advanced after pin | T01 `PIN_MOVED`; immutable SHA | A/G1 actual Git adapter |
| Partial or missing donor => false deletion | T06/T09/T13 retain `UNOBSERVABLE` and no tombstone | C/D/G1 |
| Similar name or alias => authority merge | T12, identity schema confirmed requires owner receipt | B/Z1 |
| Host action collides with HQ development action | T16 owner separation | H/Z2 |
| `../`, absolute, .git, drive, secret path, symlink/junction/submodule | T07/T17 and manifest safety contract | A/G1 actual object reader |
| Malicious git refs/markdown/comments treated as agent command | T18/T24 inert fixture data | A/E/H/Z2 |
| Binary/LFS/oversized/sparse/unreadable source | T07/T19 budget oracle; real adapter UNRUN | A/G1 |
| Corrupted digest, duplicate replay, fork | T05/T20 use real C4 replay (T20); N-way UNRUN | C/G1 |
| Expired/revoked Host grant or wrong actor/session | T21 oracle denial, host schema | H/G2 live revoke |
| Personal World text and credential leakage | T17/T23 oracle filter; real scanner UNRUN | H/G2 |
| Cross-lane effect, provider sender, extra graph, scheduler, TierFlow publisher | T22 test-oracle static scope only | Z1/G1/Z2 |
| Source present/test file present falsely declared runtime | T14; evidence-receipt schema constrains runtime receipt kind | Z1/G1/G2 |
| 10k records, cancellation, low disk, pagination | T19 source oracle bench (NOT EXECUTED HERE); real runs UNRUN | A/C/G1 |

## Independent source risks found
1. Existing `hq/lib/catalogue-delta.mjs` requires an evidenceRef present in the tombstone record and refuses replay forks; it **does not itself verify independent Host/owner confirmation** for tombstone evidence. Donor `hq/catalogue-identity/crosswalk.mjs` requires `owner_confirmed` plus a receipt. B/C/G1 must reconcile before using old C4 tombstones as authoritative retirement.
2. Existing HQ-114 at `hq/aii/fixtures/HQ-114-compare-registered-repos.mjs` compares seeded synthetic capability sets, not immutably pinned repository revisions. Do not claim real N-way coverage.
3. Existing `hq/lib/arbor-service.mjs` host mode demands actor/session and authorize callback; that is an interface boundary, **not evidence of a deployed Host-mounted HVI read API**. H/G2 must verify live grants at each operation.
4. Existing beta gate records include unverified native loopback/bearer, PWA replay, Tauri scope, missing lockfiles and non-running Windows/Android acceptance. None are waived by this wave.
5. Source snapshot only: test oracles refuse secret-looking paths; this is **not** comprehensive secret/content scanning of source, artifacts or runtime output.

## Fail-closed policies
No automatic tombstone or retirement from absence. No owner equivalence inferred from similarity. No implicit HEAD. No grant inferred from a persistent registry/cache. No donor program execution. No untrusted content construed as an instruction. No source-only evidence assigned `RUNTIME_VERIFIED`. No approved cross-lane product writes by F.

**Disposition:** PREPARED_NOT_PROMOTED / no runtime acceptance / beta BLOCKED_NOT_BETA.
