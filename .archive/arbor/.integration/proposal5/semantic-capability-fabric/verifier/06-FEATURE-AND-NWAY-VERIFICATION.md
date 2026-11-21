# Verification from function → feature → app and across immutable revisions
## Feature-wide claims need aggregation
Feature X can span 20 files and 7 functions. All 20 file annotations may identify X, but that does not establish behavior. Construct a feature-level proof graph from actual runtime reachability and proof obligations for owner entrypoints, contracts, edges, external boundaries and negative behaviors. Unverified subpath or missing owner receipt is `UNKNOWN`, not silent success.

**Required coverage dimensions:** annotated eligible source, linked exports, observed imports/entrypoint reachability, test obligations/observed tests, effect boundary coverage, input/environment coverage, runtime exercised paths and rejected/unknown claims. Aggregator can support a feature only under explicit end-to-end criteria; one correct function cannot certify the whole application.

## HVI integration
HVI still owns pinning/delta, identity crosswalk, and report export. SV adds receipt joins, not a duplicate compare engine. For `repo@A,repo@B,repo@C` (same repo three different SHAs) and external donor pins:
- Source lineage: blob/path/unit moved, renamed, split/combine candidate.
- Semantic contract lineage: property IDs and reviewed versioned spec revisions, new/deprecated/changed preconditions and effects.
- Build lineage: binary/lock/toolchain changes.
- Proof lineage: claim verdict at each pin; proof gaps, refuted properties, retested/new oracles.
- Explanation lineage: sentences newly supported, weakened, contradicted, removed or stale.
- Runtime lineage: observed Host/client/device receipts scoped per build and environment.

## Illustrative version matrix
| Claim | V1 source | V2 migrated source | V3 runtime |
|---|---|---|---|
| `pin SHA preserved` | DECLARED, tests UNRUN | SUPPORTED_BY_TESTS | SUPPORTED_BY_TESTS (new artifact) |
| `same output as V1` | baseline fixtures | UNKNOWN cross-language equivalence | PROVED_UNDER_MODEL only if formal bounded reference model + checked proof |
| `no Host write` | no trace: UNKNOWN | incomplete static analysis: UNKNOWN | SUPPORTED_BY_TESTS for observed paths, not "never" |
| `all users can use feature` | UNKNOWN | UNKNOWN | requires owner auth + supported runtime/device scope |
This table is a **hypothetical example**, not claims about actual ARBOR.

## Migration semantic preservation
For a genuine replacement, require an approved `@hq:migration` lineage spec: old/new unit+artifact pins, mapped input/output schema, expected equivalence relation, legal exclusions, counterexample policy and at least independently run differential/regression or bounded formal proof. Same capability ID ≠ same behavior. Strongest form: test old and new implementations against same independent oracle using deterministic recorded input, safe sandbox and source/build hashes. When unable to execute donor, preserve `MIGRATION_EQUIVALENCE_UNPROVEN`.

## HVI comparison output upgrades
Add optional `verificationDelta` with `newlySupported`, `regressedOrRefuted`, `newlyUnknown`, `staleReceipts`, `unprovenEquivalence`, `changedAssumptions`, `unobservedEffects`, `newlyVerified`, including evidence and exact SHA. This is a versioned adapter to existing HVI `VersionComparison`, not a breaking change to its schemas while worker lanes are active.
