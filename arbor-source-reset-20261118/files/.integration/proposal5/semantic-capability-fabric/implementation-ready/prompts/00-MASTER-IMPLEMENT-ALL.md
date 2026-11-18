# MASTER WORK ORDER — implement SCF + SV integration to accepted code; no redesign
**For ONE coding agent acting as implementation orchestrator.** Not an instruction to merge source without gates; not an instruction to run real donor comparisons.
## Shared invariant and required reading
Before work inspect the current exact SHA and full source of ARBOR `main`, the SCF draft PR #13 branch, accepted HVI source branches, and the **governing HVI policy** `.integration/proposal5/hq-version-intelligence/10-NATIVE-TOOLING-ONLY-SCOPE.md`. Read **every** SCF `implementation-ready/` root document, all `recipes/`, `contracts/`, `fixtures/`, `tests/`, `work-orders.json`, parent `semantic-capability-fabric/README.md` and SV-R012 `verifier/README.md` plus original `verifier/contracts/*.v0.schema.json`.
HVI is **native-tooling-only** in this wave. Real VIVIM/ARBOR/Labs migration comparisons are forbidden during installation or tests; use only synthetic invented Git-source fixtures and normal production source-module imports for build/compatibility. No provider APIs, Playwright, Puppeteer, personal corpora, browser sends, direct source effects or new canonical graph/wiki/registry.
Pin source exact SHA; do not silently use moving HEAD, do not force-push or merge without G gate. Report source pin/branch/changed files/test commands+exits vs UNRUN, actual blockers and gate status. A source-only import is not runtime verification.
## Goal
Take existing PR #13 design and this entire near-code kit to **actual runnable code** in approved ARBOR source paths, with honest source/test/runtime readiness. Concretely implement v1/v2 compatible v3 typed reference parser, source-symbol binding, incremental compiler, registry/SQLite adapters, capability passports, HVI N-way semantic adapters, proof-carrying SV read projections, agent protocol/diagnostic CLI, existing HQ service and Wiki projections. Product Host/TierFlow only after native HVI/Host safety gates accepted.
**Do not rewrite architecture.** Each target source file, signatures and acceptance test is specified in `02-TARGET-FILE-PLAN.json`, `03-DEPS-AND-INTERFACES.md`, `04-ABI-ERROR-SEMANTICS.md`, `recipes/`, `05-TEST-AND-RELEASE-GATES.md`.

## Ordered orchestration
1. Execute S0 fully, verify native HVI accepted baseline and exact workerBaseSha. If blocked, commit S0 proof and stop. Do not spin A-F on unaccepted HVI main as though ready.
2. Implement A and B from same frozen baseline (separately scoped branches or single serial feature branch under exact allowed files); F creates independent source/negative test fixtures without changing A/B. Keep F structurally independent.
3. Once A/B exported seams exist, C, D and E code to their exact prescribed interfaces. Do not silently modify other owners; resolve through G after code-diff audit. D verifier cannot use author-written tests as sole independent proof; retain counterexample support.
4. Require Z1 independent verification by a separate reviewing agent/conversation if possible; no self-declared reviewer acceptance. If independent tool/reviewer unavailable, label `Z1_UNRUN`, do not call gate GO.
5. G1 integrates exact worker heads serially using reviewed PR; runs Node24 baseline, full SCF tests, synthetic N-way and permission negatives with actual commands/exits; update source-closure ledger. A failed test requires owner repair and re-review. No force push.
6. Only if G1 `GO_FOR_PRODUCT_WIRING` and HVI native Host tool accepted, launch H and U isolated source branches. H checks actor/session/grant per source read. U only inert view and UI rev guard. Preserve PWA security gate.
7. Require independent Z2, then G2 reviewed serial integration and real product read/deny/revoke tests. No runtime/device claim without actual runner. R final readiness proof uses synthetic fixtures and no real migration.
8. Report all exact commit SHAs, merge PRs, test exits, blockers, final `SCF_TOOLING_READY` vs `PARTIAL_SOURCE_PREP` vs `BLOCKED`. Do not mark beta GO.

## Non-negotiable scope
No donor/Labs/ARBOR migration comparison, no untrusted execution, no credential reads, no extra source-of-truth DB, no standalone native comparator, no second wiki writer, no source code in comments as permissions, no silent unknown->equivalence, no new browser automation. Use official existing HQ, HVI and SV contracts.

## One-agent fallback
If parallel branches unavailable, perform same modules in exact A→B→F fixtures→C→D→E→tests→independent Z1→G1 sequence on one isolated feature branch; however, **do not self-certify independence**. Independent review and merge gate remain hard dependencies. Never overwrite another active HVI worker branch.
