# VLang master-thread convergence — R002 pre-development reconciliation
Date: 2026-11-12. Scope: preparation only, no runtime implementation or acceptance.

## Decision / truth hierarchy
1. ARBOR `main` and approved source owners are production source truth. Observed `main` at review: `3d45fbf1c3963e4834a5e51ee7f6e63f3a0da298`; re-pin before work.
2. HVI `10-NATIVE-TOOLING-ONLY-SCOPE.md` governs HVI activity: build native tool, synthetic invented Git fixtures only, no real donor/ARBOR/VIVIM/Labs migration comparison, no automatic launch.
3. PR #13 `work/p5-semantic-capability-fabric-r001` at `0e35c54105f7b8ee8a772b3159d243c6788fb7d4` is the **implementation specification for SCF+SV**, including 44-file engineering kit, 32 target-file actions, 15 prompts, 34 acceptance scenarios. It is draft/unmerged and must be reconciled with current main and accepted HVI interfaces at S0; its API designs are not production contracts.
4. PR #19 `work/p5-semantic-engineering-program-scaffold-r001` originally `9e91e973d6a99ba89eb6b3c02a941a0b122e2807` is the **umbrella program** for long-horizon semantic engineering, independent verifier, proof-carrying explanations, broader language/runner support and eventual SDK. Do not create a second A–F implementation plan in PR #19 duplicating PR #13.
5. Prior HQ SWC and Labs semantic wiki proposals remain *research/reference*, not runtime-authoritative. Current HQ v1/v2, indexer, SQLite, semantic ID registry, wiki, HQKG, Host, Studio, TierFlow have their existing separate owners.

## Naming to eliminate ambiguity
- 'VLang' = informal program/thread umbrella, **not** an independently accepted programming language or executable runtime.
- `@hq:semantic/3` = proposed opt-in, additive annotation language; older v1/v2 semantics are preserved.
- SCF = compiler + semantic facts/passports built on existing HQ systems.
- SV = independent claim/spec/proof/explanation checking; compiler cannot certify itself.
- SEVP = coordinating program; does not replace SCF master engineering order.
- HVI = existing native pin/N-way comparator; SCF supplies optional semantic enrichments after accepted interface.
- AutoWiki/TierFlow = read-only projections, not evidence sources or authorization systems.

## First vertical slice and explicit proof ladder
A selected pure source function and its actual source blob -> registered annotation -> independently observed AST/symbol/contract witness -> reviewed ClaimSpec/oracle -> intentionally false implementation -> concrete REFUTED counterexample -> source/build-pinned proof receipt -> truth-constrained explanation -> existing wiki. This is the first *implementation objective*, not a completed test. For positive cases, finite executed tests produce SUPPORTED_BY_TESTS, not universal PROVED. No proof based only on compiler output or tests supplied by the code-writing agent.

## Critical hardening gaps to resolve in S0/contract review (before code)
| Gap | Required closure/evidence | Owner |
|---|---|---|
| Branch/source drift | exact main, PR13, PR19, HVI accepted heads and actual code diff; freeze workerBaseSha | S0 |
| Parallel ownership collision | reconcile PR13 A/B/C/D/E/F/H/U versus PR19 proposed A–F; one file writer at a time | G/S0 |
| Grammar/scope | opt-in syntax; comment escaping; duplicate tags; symbol attachment ambiguity; multi-capability membership; v1/v2 fallback | SCF A + independent reviewer |
| Registry/schema migration | accepted ID kinds, alias drift, additive SQLite schema upgrade/rollback, deterministic canonicalization | SCF B |
| Trust roots and spec weakening | who approves ClaimSpec/oracle changes; independent baseline comparison; downgrade/revocation; compromised checker scenario | independent SV/QA |
| Proof identity | Git algo vs SHA256, source bytes, dependency graph, toolchain, artifact digest, environment, bounds and stale invalidation | SV |
| Verdict semantics | PROVED under explicit model, REFUTED, SUPPORTED_BY_TESTS, UNKNOWN/BLOCKED; no false equivalence | SV/ABI reviewer |
| Observation completeness | absence of events never proves absence of effects; distinguish Host origin receipt, test witness, source-only claim | Host + SV |
| Explanations | clause-level checked proof binding; unsupported universals blocked; contradictory claims surfaced | SV + wiki |
| HVI coupling | only accepted native HVI APIs; schema version negotiation, denied/partial/revoked read, 2..N synthetic pins | HVI owner + SCF C |
| Agent tooling | native discovery and diagnostics, source waivers, CI/IDE SDK scope, no annotation-granted privileges | SCF E |
| Privacy and effects | no provider API, donor execution, personal corpus, live effects, unauthenticated read; redaction at output and cache | Host + security |
| Robustness | incremental invalidation, file rename/split/merge lineage, cancellation, ENOSPC, limits, reproducibility | HQ + QA |
| Acceptance truth | observed executable command+exit, negative mutation suite, independent Z, serial G, rollback, UNRUN explicitly | F/Z/G |

## State classification / do not overclaim
- HQ parser, source indexing, registry, SQLite, wiki: EXISTING SOURCE; exact current integration/tests to reverify in S0.
- HQ SWC research bridge, Labs R010: RESEARCH / NOT PROMOTED.
- SCF+SV typed ABI, verifier and integrated product capability: DESIGN + NEAR-CODE KIT / NOT IMPLEMENTED.
- SCF 34 acceptance scenarios and SV red-team cases: SPECIFIED / NOT EXECUTED.
- PR19 SDK, general behavior/runtime witness and broader language families: PROGRAM GOALS / NOT IMPLEMENTED.
- HVI synthetic readiness and Host/phone gates: independently governed; do not infer from waivers or preliminary status.
- Neither PR currently establishes `SCF_TOOLING_READY`, SV verification acceptance, HVI real migration acceptance, or beta/device GO.

## Execution topology — no duplicate workers
S0 exact source/ABI/gate lock -> SCF A/B/F independently scoped -> SCF C/D/E -> independent Z1 -> serial G1 -> H/U (only after Host/HVI prerequisites) -> independent Z2 -> serial G2 -> R synthetic tooling readiness. This is PR13's implementation plan, **not** an authorization to start before S0, independent review and accepted prerequisites. SEVP's additional evidence/SDK/extended verifier ambitions become *post-SCF extension tracks* with separate future source-owner review. Never equate SEVP's conceptual G0-G8 with HVI/SCF G1/G2 gates.

## Pre-development acceptance definition
Documentation closure is achieved only when (a) both branch heads and exact current HVI gate/ABI evidence are repinned, (b) a single owner-approved collision-free write map exists, (c) spec/oracle trust roots and false-case acceptance are signed off, (d) every worker prompt resolves to a real source/test interface, (e) synthetic test and rollback rules are explicit, (f) separate product security gates remain untouched. This document closes *conceptual overlap* but not (a)-(d); mark `PREDEV_CONVERGED_WITH_OPEN_GATES`, not `PREDEV_COMPLETE`.

## Next action for implementing orchestrator
Read PR13 `implementation-ready/prompts/00-MASTER-IMPLEMENT-ALL.md`, the current SEVP convergence addendum, HVI scope, and real files; execute **S0 only** first. Report source pins, conflict map, accepted gate evidence and blocked interfaces. Do not launch all worker lanes or merge drafts until resolved.
