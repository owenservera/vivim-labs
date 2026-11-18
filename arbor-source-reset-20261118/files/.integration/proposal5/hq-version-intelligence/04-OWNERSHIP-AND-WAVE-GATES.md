# HVI wave owners, exclusive paths, gate and launch ordering

> **2026-11-12 SCOPE CORRECTION — NATIVE TOOLING ONLY.** Read [10-NATIVE-TOOLING-ONLY-SCOPE.md](../10-NATIVE-TOOLING-ONLY-SCOPE.md) when this is a prompts/ file; for root docs see [10-NATIVE-TOOLING-ONLY-SCOPE.md](10-NATIVE-TOOLING-ONLY-SCOPE.md). This correction governs any conflicting original instructions below.

## Updated gate overrides
Current user scope: **tooling and native discoverability only**. X real audit is deferred indefinitely pending explicit user-triggered comparison request. H + U are the only W2 parallel lanes. Z2 follows H/U, G2 integrates H/U, R certifies tools using synthetic fixtures. S0/D's existing data remains pinned provenance, not a command to continue the 361 review. Z1X already launched and must remain untouched.
`GO_FOR_W2_TOOLING`, `GO_FOR_G2_TOOLING_REVIEW`, `GO_FOR_READINESS_REVIEW` and `HVI_TOOLING_READY` supersede original gate labels for this correction. No requirement to finish X or complete 361 semantic disposition to install tools. Only a later explicit user request initiates an actual corpus audit.


---

`S0 (freeze & census) → [A source pins | B identities | C N-way diff | D parity atlas | E reports | F contracts/tests] IN PARALLEL → Z1 independent referee → G1 serial integration → [H Host | U TierFlow | X real audit] IN PARALLEL → Z2 referee → G2 serial integration → R final self-audit`

## Branch foundation
- Documentation is committed/merged to ARBOR `main` **before any worker** starts. The fixed `01-SOURCE-PINS.json` is the initial auditor dataset, not automatically updated.
- At S0 launch inspect current ARBOR `main` and record **one immutable workerBaseSha** (post-plan-merge). All Wave 1 A–F/Z1 branches fork that exact SHA, NOT the older `370d18d92dfa463f58bd145b307b488a64746ee5` pre-plan source pin. If main advanced, S0 reconcile source changes, commit proof, pin workerBaseSha; stop on risky drift.
- S0 validates inputs and emits `GO_TO_PARALLEL` or `BLOCKED_SOURCE_DRIFT`; commits only `proofs/S0/`. Workers can read its committed report, but are not allowed to merge it. Baseline S0 source pin SHA is stable even if main advances.
- G1 alone creates/uses `integrate/p5-hq-version-intelligence-r001` from verified current main, merges worker changes serially. G1 decides whether production code is ready for main. **No worker merges or force-pushes.**
- H/U/X fork exact **accepted G1 merged main head**; all require G1 `GO_FOR_W2`. Z2 is independent review. G2 serially integrates via same integration branch after reconciling intervening main changes and verified dependencies. R runs only after G2 `GO_FOR_SELF_AUDIT`.

## Stable ownership (cross-branch write boundaries)
| Lane | Exclusive write root | Duties |
|---|---|---|
| S0 | `proofs/S0/**` | immutable pins, baseline census, drift detection |
| A | `hq/version-intelligence/sources/**`, `proofs/A/**` | source adapters, Git object snapshots, pin verification |
| B | `hq/catalogue-identity/**`, `hq/version-intelligence/identity/**`, `proofs/B/**` | pure identity lineage and original crosswalk migration |
| C | `hq/version-intelligence/compare/**`, `proofs/C/**` | N-way comparator, change classifications |
| D | `hq/version-intelligence/parity/**`, `proofs/D/**` | HQ parity & Labs/Host/TierFlow migration atlas |
| E | `hq/version-intelligence/report/**`, `proofs/E/**` | JSON/Markdown report projection and operator export |
| F | `contracts/hq-version-intelligence/**`, `tests/hq-version-intelligence/**`, `proofs/F/**` | independent schema, deterministic fixtures, negative tests |
| Z1 | `proofs/Z1/**` | independent contract/evidence referee (no merge authority) |
| G1 | integration branch, `proofs/G1/**`, `integration/reports/HVI-G1.md` | serial staging, shared interface reconciliation, real runnable tests |
| H | `apps/vivim-host/**`, `hq/lib/arbor-service.mjs`, `proofs/H/**` | actor-authenticated development read integration |
| U | `packages/tierflow/**`, `proofs/U/**` | inert inspection UI, no visual publisher duplication |
| X | `proofs/X/**` | real five-repo audit and 361-item classification (read-only) |
| Z2 | `proofs/Z2/**` | independent runtime/ownership/refusal referee |
| G2 | integration branch, `proofs/G2/**`, `integration/reports/HVI-G2.md` | Host/UI/test consolidation and final gate |
| R | `proofs/R/**` | self-audit / reproducibility receipt and final decision |

## No implicit write grants
S0, Z1, Z2, X, R report into their own proof folders only. If code owner D, B, A etc must change the other lane's code, write a source-pin handoff, do not edit it. H is the **only** late lane allowed to patch existing `hq/lib/arbor-service.mjs`. G owns root manifests, lockfiles, shared release gates, .github and merge collision resolution; do not overwrite beta security blockers.

## Gate vocabulary
- `GO_TO_PARALLEL`: S0 pins and scopes verified.
- `PREPARED_NOT_PROMOTED`: source/proof exists on worker branch, not integrated.
- `GO_FOR_G1_REVIEW`: Z1 accepted evidence and no uncontrolled authority conflict.
- `G1_SOURCE_STAGED_TESTS_UNRUN`: integration source staged but executable evidence missing; cannot claim W2 accepted.
- `GO_FOR_W2`: G1 verified usable library interface/necessary tests, source closure and no new dangerous authority; runtime Host acceptance remains separate.
- `GO_FOR_G2_REVIEW`: Z2 accepted source truth, security gates not misreported.
- `GO_FOR_SELF_AUDIT`: G2 accepted runtime read-only feature and tests; only R may certify actual dataset coverage.
- `BLOCKED`, `REPAIR_REQUIRED`, `NO_GO`: hard stop, exact owner handoff, never silently skip.
- `BETA_GO`: **out of scope**, cannot be issued by HVI workers.

## Each lane's required receipt
`BASE_SHA`, `SOURCE_PINS`, `WORK_BRANCH`, `COMMIT_SHA`, `CHANGED_PATHS`, `DONOR_BLOBS`, `TESTS_WITH_COMMAND_AND_EXIT`, `UNRUN_TESTS`, `SECURITY/PRIVACY`, `BLOCKERS`, `PROPOSED_DISPOSITION`, `NEXT_OWNER_HANDOFF`.
