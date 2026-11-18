# HVI C — scoped G1 integration handoff
Status: **PREPARED_NOT_PROMOTED**, source-only, not merged. Source branch: `work/p5-hvi-w1-c-compare-r001`. Base: `f9823a5131e73e7562db5a4ec14e1ec481557d69`.

## Stable exports
`hq/version-intelligence/compare/index.mjs` exports `comparePinned`, `digestManifest`, `HviCompareError`, `COMPARISON_SCHEMA`, `MANIFEST_SCHEMA`, `SOURCE_STATES`.
Signature: `comparePinned({ pins, manifests, targetPinId, pairwise=false, scope, budgets={}, identityDecisions=[], signal })`.
No ambient branch HEAD, I/O, graph updates, Host permissions, effect execution, browser send, or donor dependency.

### Injected seam, not coupled to pending lanes
Lane A supplies `PinnedSource[]` with `pinId`, `provider`, string `repoIdentity`, resolved 40-hex commit SHA, 40-hex tree SHA, 64-hex `manifestDigest`, `completeness` and `scope:'development'`.
Lane A supplies matching `SourceManifest[]`, each with `schema:'arbor.hvi.source-manifest/1'`, `pinId`, `completeness`, `truncated:boolean`, `entries`, and `digest` from `digestManifest`. Entries use POSIX `path`, Git `blobSha`, `mode`, `kind`, optional claimed `capabilityIds[]`, and `excludedReason`. Other schema formats **must use explicit G1 adapters** rather than silent coercion. Do not change C4 or semantic-registry schemas.
Lane B may inject source-scoped capability ID hints via manifest entries. These yield review-required split/combine/similarity candidates only. Nonempty `identityDecisions` fail closed with `EQUIVALENCE_UNPROVEN` until G1/H provides an owner-authenticated, evidence-bound decision verifier; never infer equivalence from a shared name.
G1 owns combining these seams with A/B/D/E/F and any contract fixes; C does not touch their write paths.

## Output constraints and known limits
Output `arbor.hvi.version-comparison/1` has `sourcePins`, `targetPinId`, optional all-pair records, `coverage`, `summary`, `sourceDeltas`, candidate-only `capabilityDeltas`/`identityCandidates`, `unported` (physical source rows only), `unobserved`, `evidenceIndex`, `limits`, `errors`, `reproducibilityDigest`. `runtimeProof` and authority/merge plan fields are intentionally empty, not certificates.
Same-path diff and identical blob relocation are source facts; capability identity collision or split/combine are provisional. Partial/missing scans create `UNOBSERVABLE`, never tombstones. Truncated reports announce omitted records.
The source comparator does not itself verify a Host actor/session or source grant: **H owns live authorization**. The `scope:'development'` pin check is a data-contract guard, not an access-control substitute.
Release beta still blocked in `integration/beta-gates.json`; no Node 24 / Bun / Cargo / Windows / Android or Host end-to-end acceptance was executed.

## Source pins and owners
S0 gate: `GO_TO_PARALLEL`, S0 HEAD `2ef7a9f66419d681e040b459ba4762a63aa9564f`; immutable worker base `f9823a5131e73e7562db5a4ec14e1ec481557d69`.
Fixed donor mains in `01-SOURCE-PINS.json`: repos-hq `9b922ea049dbadc94e504b5b9432743995c020ca`; vivim-web-harness `cae97afe848d79957c9d6c698a90996a758bfd56`; tierflow `50822f11ee63c3d80f631f8fea7dcb9b19bffc38`; vivim-labs `022608f5797e7b4a14ccc4c393bd3c5a4704a4be`; baseline ARBOR `370d18d92dfa463f58bd145b307b488a64746ee5`. The S0 fresh ARBOR target was `f9823a5131e73e7562db5a4ec14e1ec481557d69`.
S0 also verified six research-branch pins; they were **not loaded into a real comparator run**. The actual migration-corpus audit belongs to X after G1 acceptance, not C.

Next owner: **Z1** independently review diff/evidence, then **G1** serially integrate and execute Node 24 conformance against actual A/B adapters. Resolve adapter schemas and owner-decision verification there. Do not promote to W2 without explicit G1 GO.
