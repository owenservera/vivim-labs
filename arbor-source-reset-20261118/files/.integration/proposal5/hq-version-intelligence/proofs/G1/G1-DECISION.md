# G1 decision — owner-authorized GO_FOR_W2_TOOLING, source-only exception

**W2 authorization: GO_FOR_W2_TOOLING** (OWNER_EXCEPTION_SOURCE_ONLY). **G1 executable verification: G1_SOURCE_STAGED_TESTS_UNRUN / REPAIR_REQUIRED.** **Runtime acceptance: NO.** **Accepted main SHA: null.**

Owner expressly waived the *infrastructure-only start prerequisite* on 2026-11-12. H and U may begin parallel source work from the pinned G1 waiver commit bbe7e4896eea02bcdec6f8fedb66eab885250161. Do not count UNRUN tests as passes, promote G1 to main, deploy, enable remote Host features or claim security acceptance. The owner waiver is recorded in W2-OWNER-OVERRIDE.json and W2-OWNER-OVERRIDE.md. H/U are development forks of the unaccepted G1 candidate, not accepted-main implementations.

# G1 integration decision — G1_SOURCE_STAGED_TESTS_UNRUN

**Decision: REPAIR_REQUIRED. Promotion: NO.**
**Candidate code HEAD:** 0dfb08406c50df2e385c0ff924694e0aeaa33565
**Branch:** integrate/p5-hq-version-intelligence-r001
**Main baseline:** aece3618395f215d825bf4eb41dcc07d6d656ab7
**Acceptance main SHA:** none.

All S0/A-F/Z1X source and referee proofs have been serially staged on the exclusive G1 branch. The key incompatibilities have explicit versioned G-owned source repair adapters: A/C dual digest, C/E evidence-rich report, D parity join, B identity advisory, stricter v2 schemas, bounded grant-aware native library and synthetic-only CLI. A synthetic Node24 suite, 2/3/5 invented repos, T01-T24 negatives and offline replay are committed.

However **no actual Node24 G1 combined suite has executed**. The available local environment has only Node22 and no checkout; GitHub Actions reports startup_failure with zero jobs. The available evidence supports static source staging, not validated runtime or schema conformance. Main must not be updated from this candidate without observed tests and independent review.

No normal merge to main was performed. The later owner exception permits W2 **development only**, without making this branch an accepted main pin. The beta gate remains BLOCKED_NOT_BETA. X real audit remains deferred pending explicit later user request.

Resume only after obtaining Node24 run receipts, repairing any failures, independent security review, a reviewed PR and normal merge. Until then, retain this branch and its exact blocker ledger.
