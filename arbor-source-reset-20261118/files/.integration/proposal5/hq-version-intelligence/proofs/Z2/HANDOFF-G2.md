# Remote-first G2 tooling repair handoff — from independent Z2

**DO NOT BLOCK REMOTE DEVELOPMENT ON THE LOCAL WINDOWS PC.** Owner has already authorized W2 source development despite infrastructure-only G1 testing blockage. This is **NOT** permission to turn on privileged remote access, declare tests passed, merge main or issue beta acceptance. Z2 decision is **REPAIR_REQUIRED**, so start owner-scoped G2 **repair development**, not G2 promotion.

## Ordered actionable repairs
1. **G1_NOT_ACCEPTED** (P0, owner G1/CI): Run actual Node24/AJV synthetic tests, review G1 and accept via normal PR; no direct main write. Evidence: G1 main accepted SHA null. Waiver bbe7e489 only authorizes W2 development.

2. **HOST_NATIVE_DISPATCH_DISABLED** (P0, owner H/G2): Implement admitted typed Host NL path and real grant checks without a second dispatcher; keep remote path off until tested. Evidence: H catalog v7 action versions.compare.pinned is enforcement_missing and has null executor. H read port is not installed into canonical live Host dispatch.

3. **LIVE_SOURCE_GRANT_MISSING** (P0, owner Host security): Use canonical revocable actor/session/repository development grant, exact allowlist; recheck every pin/page/comparison/report. Prove revoke and expiry. Evidence: H createHviLocalOwnerGate verifies existing owner token/UI session but verifyDevelopmentSourceGrant remains an injected callback, no attested production owner.

4. **H_U_REPORT_CONTRACT_MISMATCH** (P0, owner H/G2): Add one explicitly authorized sanitized detailed inspector DTO derived from G1, preserve uncertainty/digest and bound to current session/grants and publisher revision. Synthetic end-to-end H→U tests. Evidence: H returns vivim.host.hvi-safe-report/1 summary+digest; U accepts arbor.hvi.adapter.comparison-report/1 containing originalComparison, coverage, sourceDeltas, targetPinId.

5. **PUBLISHER_REVISION_UNATTESTED** (P1, owner U/G2): Read current revision from canonical publisher, enforce CAS and revoke before render, retain feature-off and use existing UiCore only. Evidence: U renderer calls canonical validateUiDocument/renderUiDocument, but expectedRevision and currentRevision are caller-supplied; no actual UiCore/UiStore mount.

6. **U_TEST_ASSERTION_SUSPECT** (P1, owner U): Correct assertion to inspect grouped descendants; run full Node TS-strip suite, record real exits. Evidence: U test requires root document.nodes[0].children.length>4; normal 3-pin makeDocument grouping creates a section node with fewer direct root children.

7. **REMOTE_CI_STARTUP_FAILURE** (P0, owner CI owner): Resolve GitHub Actions startup/runner/permissions/quota with GitHub UI or alternate owner-approved remote runner; execute synthetic-only jobs with real exits. Evidence: G1 runs 43307067218/43307026813, H run 43310564183, U run 43311616409 all startup_failure; H run has zero jobs.

8. **EXECUTABLE_EVIDENCE_UNRUN** (P0, owner G1/H/U/G2 QA): Run on remote checked-out Node24/Bun environment, record command, OS, version, exact SHA, log and exit; no real donor comparisons. Evidence: H/U/G1 Node suites, AJV Draft 2020-12, offline replay, live grant and end-to-end UI have no command exit receipts.

## Mandatory remote G2 acceptance tests
- 2/3/5 synthetic invented Git repositories and three immutable SHAs of one invented repo; no actual ARBOR/VIVIM/Labs comparisons.
- Real canonical NL text → Host dispatcher → owner/session → explicit active per-repo grant → A resolvePin/snapshotAt → G1 runHviTool → authorized sanitized TierFlow detail DTO → canonical TierFlow renderer with trusted publisher revision.
- Negative tests: no source grant, unsupported repo, stale/expired session, revoke between pin/read/pages/report/render, secret path, digest mismatch, target change, partial/unobservable, cancellation, repeat and publisher revision conflict.
- Prove no second source grant DB, Host effect executor, scheduler, graph, browser sender or TierFlow publisher.
- GitHub Actions latest G1/H/U `startup_failure` suggests repair runner startup first. Inspect workflow YAML/runner label/availability/permissions/quota in GitHub UI; if unavailable, choose another **owner-authorized remote runner**, checked-out exact SHA, synthetic-only tests. Do not infer status from authored tests.
- G1 accepted main SHA is null. G1 must first pass executable tests/review and merge by normal PR; G2 must reconcile subsequent main advancement and integrate H/U serially after approvals. Do not force push.
- Separate beta security and Windows/Android physical acceptance stays blocked. X real audit remains deferred by user scope.

## Completion criteria for new independent review
Only after actual command + exit evidence, live Host development-source grants, H→U typed detailed read seam, and UiCore revision ownership exist may a future Z2/G2 review consider `GO_FOR_G2_TOOLING_REVIEW`. Current `REPAIR_REQUIRED` is compatible with continued **source-only remote repair** under the existing owner waiver.
