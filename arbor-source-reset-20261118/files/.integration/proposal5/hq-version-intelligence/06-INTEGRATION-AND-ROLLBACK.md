# Integration, dependency closure, acceptance and rollback

> **2026-11-12 SCOPE CORRECTION — NATIVE TOOLING ONLY.** Read [10-NATIVE-TOOLING-ONLY-SCOPE.md](../10-NATIVE-TOOLING-ONLY-SCOPE.md) when this is a prompts/ file; for root docs see [10-NATIVE-TOOLING-ONLY-SCOPE.md](10-NATIVE-TOOLING-ONLY-SCOPE.md). This correction governs any conflicting original instructions below.

## Corrected integration boundary
- G1 uses synthetic 2/3/5-repo test manifests and portable CLI/runtime adapters, not actual donor comparisons; H wires existing native agent registry and per-invocation Host authorization; U wires the reusable inspector.
- X real audit remains **deferred**, no branch/worker execution required or allowed. Z2 inspects H/U only. G2 integrates H/U only. R checks native readiness using synthetic inputs only.
- Operational source access is opt-in at future **user invocation**; provider tokens/credentials are not scanned or pre-loaded. No scheduled scans, periodic indexing, pending migration plans or automatic source comparison.


---

1. Planning files only merge to ARBOR `main` in a reviewable preparation PR. No source functionality changes during preparation.
2. S0 uses GitHub **read-only** to resolve all current refs, obtains full commit/tree SHAs, repeats mechanical path parity against fixed baseline and records variance. Missing repo/source => explicit incomplete; no guess.
3. Workers A–F isolate source and proof commits. Their interfaces are prescribed in `03-NWAY-CONTRACT.md`; no cross-owner imports which require code not present in the worker's base to pass local tests. Instead author pure seam adapters or mark combined tests UNRUN until G1.
4. Z1 independently fetches exact worker heads and source-diffs every lane against workerBaseSha; checks negative security and identity assumptions. It cannot merge or alter owners.
5. G1 creates integration branch from current main (review ancestry; no reset). Integrate A/B/C/D/E/F then S0/Z1 proofs **serially** using merge/PR or controlled commit with exact SHA and path boundaries; run combined Node 24 test with exit codes, two-repo and N-way snapshot smoke, no-donor offline replay. Fix contract integration under G ownership or return to owner, not silently rewrite.
6. If tests executable and accepted, G1 submits one PR into `main`, reviews actual diff and merges **without force push**, records exact resulting main SHA. If no runner (including CI startup_failure), keep `G1_SOURCE_STAGED_TESTS_UNRUN`; W2 product wiring cannot be promoted. A conservative review-only W2 planning session may inspect evidence but not mark operational acceptance.
7. H/U/X start from G1 accepted `main`, not prep base. H mounts a read-only source comparison via live Host grant checks; U builds no-effect display; X runs the actual five-repo migration audit and source pin reconciliation, none alters canonical owners.
8. Z2 independently verifies security/privacy/permission and real scan completeness; G2 alone integrates H/U/X, performs end-to-end denial/replay/report tests and merges to main after GO. R independently runs final self-comparison and commits receipt.
9. Source tests passed ≠ release. Existing Beta P0 on native loopback, PWA replay, Cargo/Bun locks and Windows/phone acceptance remain BLOCKED until independently repaired.
10. Recovery: all original donor branches and pins remain immutable. Snapshot data is derived/rebuildable. If G1/G2 breaks HQ, revert the reviewed merge commit on `main` via PR and restore prior accepted report/snapshot pin; no force pushes, no history deletion, no private data cutover. Record rollback drill UNRUN unless executed.
11. If `main` advances between worker base and integration, G must inspect every intervening commit, compare changed paths, and re-run affected tests before merging; no stale SHA overwrite.
