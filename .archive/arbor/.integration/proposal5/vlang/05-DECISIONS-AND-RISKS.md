# Decisions, conflicts, risks and operating doctrine

## Settled by existing R002 convergence
- VLang = program nickname; do NOT ship a competing standalone executable language.
- PR13 owns SCF near-code implementation plan and SV minimum, PR19 owns wider long-horizon program. No duplicated workers or conflicting ABI for the same feature.
- Main HQ index/semantic registry/SQLite/wiki remain owners. Host effect authority, Studio browser sending, native HQKG and TierFlow publisher remain exclusive.
- Existing v1/v2 annotation contract stays compatible. Future v3 is additive opt-in.
- Comments, prototypes and model output never grant capabilities or independently verify runtime.
- HVI is native-tooling-only in this wave; synthetic invented Git fixtures only.

## High-value architectural choices
1. Model capability membership as many-to-many, with independent evidence dimensions and stable unit/source lineage.
2. Store typed source facts in additive, versioned side tables with safe rollback; keep generated Wiki as read-only projection.
3. Trust the *independent checker and reviewed oracle roots*, not compiler prose. Bind artifact, source, spec and toolchain revisions.
4. Make semantic query revocable and permission scoped: source may be indexable only if Host-approved; cache must be invalidated or redacted after revocation.
5. Agent instructions should request minimal headers on touched first-party files; don't demand mass edits or unsupported tags before rollout.
6. Treat historical repo comparisons, migration authorization, personal data and browser effects as separate later commands.

## Open decisions assigned to S0 / verifier owners
- Exact owner of new registry `unit`, `role`, `relation`, `migration` kinds; path-level write scopes with parallel HVI.
- What independently approves ClaimSpec/oracle changes and checks for weakening after refactor?
- Trusted verifier release and CI attestation model; signed receipt vs local digest and offline status.
- Actual HVI export/import ABI revision and Host native discovery/read port accepted status.
- Source-to-AST binding language coverage, multiple capability duplicate rules, explicit unknown/partial semantics.
- Allowed pure test runner environments and resource budgets across Node/Windows/Rust.
- Required information reduction for cached Wiki pages after development-source grant revocation.
- Performance/low-disk gate thresholds and whether any annotation coverage mandates should wait until a measured baseline.

## Failure modes to probe
Self-certifying compiler; unsupported universal explanations; conflicting owner tags; symlink/secret escaping; forged test receipts; spec weakening; SHA mix-ups; partial event traces; same-name migration overclaims; stale HVI/TierFlow revision; duplicate writer from PR19; ENOSPC and interrupted SQLite writes. Missing evidence blocks promotion, not reporting.

## Version and drift control
Only `source-pins.json` says which Git heads were **observed** by this SITREP. Those are not a frozen implementation baseline. Any S0 runner must re-pin and record exact accepted ref and gate artifacts before source writes; never assume drafts are merged or use a previous work order's obsolete base.
