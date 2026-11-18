# G1 current-main drift reconciliation

G1 source integration began at immutable main aece3618395f215d825bf4eb41dcc07d6d656ab7.

While the candidate was being staged, main advanced to 3d45fbf1c3963e4834a5e51ee7f6e63f3a0da298 by eight commits. An exact GitHub compare identified 19 paths, all limited to HVI Vision Lab optimization documentation and Rescore Expansion documentation. All 19 paths were checked against the G1 candidate: each still matched the earlier main blob/missing state before this reconciliation (zero overlap).

This commit carries exact newer-main Git blobs into the integration tree and records newer main as the second commit parent. No source worker, G1 adapter, contracts, test suite, beta gate or old main history is reset or overwritten. No normal PR/merge to main occurred. This reconciliation does not create an executable test receipt or approve GO_FOR_W2_TOOLING.

Re-run the Node24 suite and independent diff review on the reconciled candidate before any promotion.
