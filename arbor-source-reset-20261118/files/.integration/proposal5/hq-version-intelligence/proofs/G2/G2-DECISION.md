> **Latest main reconciliation:** While G2 was running, ARBOR main advanced to `9b79d732da8b92713d399040db4a02d505bc7cf1` through unrelated PR #23 Rust Rescore documents. G2 preserved the 11 disjoint exact blobs in two-parent non-force integration merge `08abdd1f71471fa9d87ab89c3c46c449ff9d9b52`. Detailed `proofs/G2/MAIN-DRIFT-RECONCILIATION.md`. This source-only ancestry reconciliation makes **no change** to G1 acceptance, runtime evidence, main write authority or the `REPAIR_REQUIRED` gate.

# HVI G2 — Serial integration decision, 2026-11-12

**Decision: `REPAIR_REQUIRED`.** G2 source repairs are committed and can continue on the remote integration branch under the existing owner development waiver. **`GO_FOR_READINESS_REVIEW` is NOT issued.** `PREPARED_NOT_PROMOTED`, `HVI_TOOLING_INSTALLED_NOT_VERIFIED`. No beta GO.

## Verified source and authority pins
- G1 candidate base: `69447c09f420310aa84c24a1908dede91c963ec8`; accepted G1 `main` SHA **null**.
- H source: `d24daa9966abb222cd25210f41dcbb4758aa1209`, source-stage commit `546132d92639af78f70d398c8a5a86d5d6a64738`.
- U source: `5836e9c9f5ba3bee36dbe6896ebfd22364b4c450`, source-stage commit `4bb70bfbadc9b35f13b36232b44044c16900976c`.
- Z2 independent proof: `725132c640a1f21d3af5c4b9123b314e4df3133d`, proof-stage commit `9aac22f151e89ea7554ef7ee5cc65b30e5b34e21`; Z2 gave `REPAIR_REQUIRED`, not a positive gate.
- Pre-G2 `main` observed: `3d45fbf1c3963e4834a5e51ee7f6e63f3a0da298`; never rewritten or merged.
- G2 synthetic source/test staging through commit `bc644e2e3314ed80a3f9891695d8ad4798cee6b9`; machine receipt `2a512b12d4027dc498dd3b22e583ed186adc4490`; human integration report `472a2be7ea69f4edd94774aba1bda26cb3011ddb`.

## What was accomplished
H and U were serially imported with 20 total scoped files including Z2 proofs; no X branch/source dependency. H→U typed sanitized detailed report bridge, per-grant before/after projector checks, source-only digest lineage, and TierFlow synthetic-only preview guard are staged. Grouped test assertion corrected. End-to-end 3-SHA same-repository and 5-invented-repository test source authored. No new Host effect writer, development grant authority, graph store, scheduler, browser provider sender or TierFlow publisher; no actual repository comparison and no activation of HVI remote source reads.

## Executed vs UNRUN
- **Executed:** six GitHub-source modules/test files isolated V8 transformed syntax evaluation, no exceptions; synthetic pure bridge and TierFlow projector V8 assertions **12/12 passed**, with a noncryptographic digest test-double. These are **NOT** OS/Node process exits or complete G2 runtime acceptance.
- **GitHub Actions:** run `43315512755` at synthetic test staging SHA `bc644e2e3314ed80a3f9891695d8ad4798cee6b9`: `startup_failure`, **zero jobs**, no exit.
- **UNRUN, exit=null:** integrated Node24, G1/Host/U/G2 Node tests, strict AJV Draft 2020-12, synthetic CLI offline replay, real Host grant/session expiration/revoke, canonical NL dispatch, UiCore publisher revision/CAS, live security, physical PWA/Android/Windows, rollback drill.
- **Defer:** X real-repository audit and 361 original-path semantic review; fixed S0/D historical counts were not remeasured.

## Owner action and stop
G1+CI: attain executable synthetic test receipts and accepted main via reviewed PR. Host security/H: wire real canonical revocable development-source grants and admitted action (default disabled until proof). U/G2: attest live UiCore revision and run full synthetic UI tests. Independent Z2: review repairs and exits. Only then may G2 request a reviewed main promotion and possibly issue `GO_FOR_READINESS_REVIEW`. Keep `integration/beta-gates.json` blocked.

Detailed evidence: `integration/reports/HVI-G2.md` and `proofs/G2/G2-RECEIPT.json`.

## Subsequent G2 source-security closeout (2026-11-12)

After the earlier G2 source stage, the integration master committed four further in-scope source/test repairs: `77bec293a7f0144d1c0bae01a1102a26bf1a7e9f`, `06858bf389338695969cff12e55ea1f1138bb20e`, `eec8a16f00e1e26459c6ad6a7e4c4c9851300d23`, and `33b6059615aab0ec5571a3acf5674725b67fcd9f`. The low-level source read helper and direct comparator now share credential-path refusal semantics; synthetic regression test cases are authored. Isolated V8 source-slice checks: **14/14**, not Node24, OS test exits or integrated acceptance. GitHub Actions [43319733234](https://github.com/owenservera/ARBOR/actions/runs/43319733234) still ended `startup_failure` with zero jobs; retry returned 403 and was disallowed. See [issue #24](https://github.com/owenservera/ARBOR/issues/24).

**Fresh decision: `REPAIR_REQUIRED`, no promotion.** PR #21 remains a draft without independent review. The original `G1_NOT_ACCEPTED`, `Z2_REPAIR_REQUIRED`, Node24/AJV `UNRUN`, live grant/dispatcher and publisher-revision gates still block an honest `GO_FOR_READINESS_REVIEW` or R certification. No real repository comparisons, main merge, live access or beta changes were executed.

**CI preparation update:** `d2411bae45964f533b5c48f0faac727c31140932` adds a dedicated read-only Node24 synthetic acceptance workflow. Its run [43320303173](https://github.com/owenservera/ARBOR/actions/runs/43320303173) failed at startup with zero jobs; tests remain UNRUN. Draft PR #21 updated. Gate unchanged.
