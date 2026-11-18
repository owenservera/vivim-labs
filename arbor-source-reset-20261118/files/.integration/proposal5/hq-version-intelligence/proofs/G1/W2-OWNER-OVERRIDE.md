# HVI W2 owner override — GO_FOR_W2_TOOLING

**Decision:** `GO_FOR_W2_TOOLING` for **H/U source development only**, issued at the user's explicit instruction on 2026-11-12.

**Authority:** Owner waiver of the G1 Node 24 test-execution prerequisite to **start** W2 development. It is NOT a test receipt, runtime acceptance, security sign-off, merger approval or beta acceptance.

**Exact source pin:** `ec5b446400d410e4b9adf3d9b9fe35fe6c562317`, branch `integrate/p5-hq-version-intelligence-r001`. G1 has **not** been merged to main; accepted main SHA remains `null`. Existing H/U instructions that require fork from accepted main are superseded ONLY for development start: fork both branches from the exact G1 candidate SHA with this explicit owner exception. W2 branches cannot be claimed independent of this unaccepted base.

**Evidence:** G1 staged A–F in order, S0/Z1X proofs, cross-lane adapters, native API, strict schema/test sources and synthetic-only CLI. Last observed GitHub Actions run 43305285225 returned `startup_failure` and no successful Node24 job; Node24 tests, offline synthetic replay and actual Host live-grant acceptance are **UNRUN**. Never label them PASS.

**Permitted now:** H implements Host agent registration and denied-by-default source authorization; U implements inert generic inspector using invented fixtures; parallel work and tests may be authored and committed on their exclusive branches. Retain access-disabled until H live-grant tests are executed and independently assessed.

**Not permitted by this waiver:** direct merge to main, deployment, source-read privilege escalation, autonomous repository scan, comparison against actual ARBOR/VIVIM/Labs sources, ownership/tombstone acceptance, beta/device/runtime release claims or unsupported GO_FOR_READINESS_REVIEW.

**Later reconciliation:** run the G1 real Node24 combined tests and AJV Draft 2020-12 conformance, review and merge G1 normally; then rebase/merge or verify H/U source against the accepted main tree before G2. Do not turn this limited exception into broader automatic release authority.

Machine-readable controlling decision: `proofs/G1/W2-OWNER-OVERRIDE.json`.
