> **2026-11-12 owner override:** GO_FOR_W2_TOOLING issued for H/U source development only, from G1 waiver commit bbe7e4896eea02bcdec6f8fedb66eab885250161. The requirements below for independent runtime verification and accepted main remain gating **promotion/readiness**, not preliminary H/U coding. Existing H/U branches were created from the exact waiver commit. See W2-OWNER-OVERRIDE.json. 

# Exact G1 blockers, owner sequence and gate handoff

1. **G1 / CI runner owner — P0:** restore or provide real Node24 execution against the exact integration candidate. GitHub Actions run 43303858096 ended startup_failure with jobs=0. The scoped synthetic workflow is at .github/workflows/hvi-g1-node24-synthetic.yml. Run all A-F + G1 tests with observed exit/stdout/stderr. Do not treat startup_failure as pass.
2. **G1 integration contract owner — P0:** adjudicate any failing A/C dual digest, C/E evidence loss, D parity, B lineage or C/F strict schema conformance tests using real observed receipts. Existing source compatibility is authored but acceptance unverified. AJV Draft 2020-12 package is isolated at tests/hq-version-intelligence/package.json.
3. **G1 independent reviewer — P0:** independently read exact integration branch diffs, validate authority/security/secret exposure, inspect test receipts and review the PR only AFTER node24+negative suite legitimately passes. No PR merge before that.
4. **H Host authority owner — W2, not G1:** AFTER G1 GO_FOR_W2_TOOLING, register the stable arbor.hvi.compare.pinned/1 capability in the existing Host/agent route. Verify actor/session/live source grants, source-reader attestation, revoke/refuse behavior and source read bounds. Do not claim Host live acceptance from G1 operator callbacks.
5. **U TierFlow owner — W2, not G1:** AFTER accepted G1 main SHA, display generic read-only version inspection/empty states using invented fixtures only. Never introduce a second publisher or claim runtime parity.
6. **Z2/G2/R — later sequence:** certify H/U and installed tooling readiness using synthetic inputs, while retaining BLOCKED_NOT_BETA and UNRUN device gates. X and real repo comparisons remain DEFERRED_USER_TRIGGER_ONLY.

No 338-path semantic review, real five-repository version comparison or migration recommendation should be generated in this wave.
