# SCF S0 gate — BLOCKED_SOURCE_DRIFT (BLOCKED_HVI_GATE)
**S0 branch:** `work/p5-scf-s0-lock-r001`; **main/workerBaseSha:** `3d45fbf1c3963e4834a5e51ee7f6e63f3a0da298`; **current moving HVI integration candidate observed:** `0539d23404ef5b1de12ad646d807a2fda57c6ab6`. Scope: only `.integration/proposal5/semantic-capability-fabric/proofs/S0/**`.

## Gate evidence
- G1 reviewed SHA `69447c09f420310aa84c24a1908dede91c963ec8` has `G1_SOURCE_STAGED_TESTS_UNRUN` and accepted-main SHA null. W2 owner waiver allows H/U **development** only, not HVI/SCF runtime or release acceptance.
- Z2 reviewed SHA `725132c640a1f21d3af5c4b9123b314e4df3133d` says `REPAIR_REQUIRED`; H live grants and H→U report compatibility not accepted. HVI integration branch is actively changing; current candidate must not be treated as accepted source. No accepted G2 merged to main at S0 observation.
- Canonical ARBOR main has the existing HQ parser/registry/indexer/wiki/service, but no SCF runtime implementation or accepted HVI native tool modules. SCF and SEVP drafts are diverged/unmerged. SCF v3, receipts, checker and HVI bridges need actual reviewed ABI.
- 13 SCF/SV JSON plans/schemas/fixtures parsed using V8 `JSON.parse` (13/13), **static syntax only**. S0 executed **zero OS tests**; no Node24, AJV, Host grant or TierFlow executable exit. HVI G1 GitHub Actions previously reported startup_failure with zero jobs. All commands in BASE-LOCK.json are `UNRUN, exitCode=null`.
- Beta gate from `main` is `BLOCKED_NOT_BETA`. No donor/ARBOR/VIVIM/Labs content comparisons, effects, remote privileges or beta claims were performed.

## Blocker owners and exact clearance
1. HVI G/G2 + independent Z: reconcile latest candidate, run synthetic Node24 integrated suite and strict Draft 2020-12 AJV with actual exits, fix/verify H→U sanitizer, independently review and merge via prescribed gate only.
2. Host security H + TierFlow U: live development source-grant owner/session/pin, revoke and cache redaction, authoritative typed result and real current UiCore revision; production route stays disabled until verified.
3. SV independent owner/F: ratify source/spec/oracle/checker/attested runner trust roots, negative false-case/mutation replay and clause-proof checking.
4. HQ parser/schema/Wiki owners + SCF G: reconcile v2/v3, 2..16 HVI against 2..32 draft SCF, additive SQLite and one-writer plan against **accepted** merged API and newest main; issue fresh S0 workerBaseSha/gate after actual review.

**Decision:** `BLOCKED_SOURCE_DRIFT`, reason `BLOCKED_HVI_GATE`. **No `GO_FOR_IMPLEMENTATION`**. Frozen main SHA is valid for S0 proof, not A/B/F admission. No other lane launched; no draft merged; no product source altered.
