# G2 main drift — reconciled without promoting HVI

2026-11-12 during G2 source staging, ARBOR `main` advanced **12 commits** from `3d45fbf1c3963e4834a5e51ee7f6e63f3a0da298` to `9b79d732da8b92713d399040db4a02d505bc7cf1` (merge PR #23, Rust Rescore 1,000-target inventory).

## Exact overlap analysis
`compare_commits(oldMain, newMain)` found **11 added/modified files**, every path in `.integration/proposal5/rescore-expansion/**`; no G1/H/U/Z2/G2 source, Host, TierFlow, HVI proof or integration report path was changed. All rescore additions remained untouched and were preserved by exact Git blob identities. This inspection was repository history/source-diff reconciliation, **not** a real HVI donor or user corpus comparison.

## Action taken
- Starting G2 source and proof HEAD: `54326ad8c1391567f2f64b06f406e492b388f28f`.
- New main SHA: `9b79d732da8b92713d399040db4a02d505bc7cf1`.
- Created serial non-force integration-branch merge commit: **`08abdd1f71471fa9d87ab89c3c46c449ff9d9b52`**.
- First parent is the prior G2 candidate HEAD; second parent is new main. All 11 rescore blobs imported unmodified into G2's Git tree; no G2 source/proof files overwritten.
- CAS-protected branch update succeeded on `integrate/p5-hq-version-intelligence-r001`. **No write to `main`.**
- This is branch ancestry/source reconciliation ONLY. It does not accept G1, pass Node/Host security tests, authorize a main PR merge, or upgrade Z2's `REPAIR_REQUIRED` gate.

## New runtime-test truth
Any new main advancements, G1 acceptance, or post-drift Node24 tests still require renewed inspection. GitHub Actions currently `startup_failure` with zero executed jobs; G2 test process exits stay UNRUN. Beta remains blocked. X real repository comparisons remain deferred.

Gate: **`REPAIR_REQUIRED` / `PREPARED_NOT_PROMOTED`**; advance remote development, not runtime or release acceptance.
