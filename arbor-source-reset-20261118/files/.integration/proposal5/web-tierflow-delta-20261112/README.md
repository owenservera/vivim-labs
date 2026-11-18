# Proposal 5 — Web UI + TierFlow Original-to-ARBOR Delta (2026-11-12)

**Audit only. No runtime edits, no release promotion, no writes to main.**

This packet answers: **What original Harness/Studio/PWA/TierFlow source actually landed in ARBOR, what was intentionally excluded, what remains unconnected, and which tests would turn source parity into an accepted UI?**

## Read in this order

1. [WEB-TIERFLOW-SITREP.md](WEB-TIERFLOW-SITREP.md) — human source-based investigation, original intent, four UI surfaces, real missing tests, architecture/cutover and prioritized repair table.
2. [DELTA-ATLAS.v1.json](DELTA-ATLAS.v1.json) — **33 machine-readable UI/visual capability rows**, owners, status, priority, release journey IDs and next proof. Not a new runtime capability registry.
3. [SOURCE-BLOBS.v1.json](SOURCE-BLOBS.v1.json) — **1,522 donor-path records** across 11 mapped original subtrees, each with source/destination blob SHA and `BYTE_IDENTICAL`, `ADAPTED` or `NOT_IMPORTED`; total is mapped donor paths, not all ARBOR files.
4. [ARBOR-ADDITIONS.v1.json](ARBOR-ADDITIONS.v1.json) — reverse-direction inventory of 5 ARBOR-only mapped product paths (four PWA browser signer/fixture files and one native security preparation note).
5. [ACCEPTANCE-PLAN.md](ACCEPTANCE-PLAN.md) — owner-sequenced implementation/test/physical device gates; **18 planned checks**, **16 negative scenarios**, none misreported as run.

## Pinned snapshot and guardrails

- ARBOR: `main@f9823a5131e73e7562db5a4ec14e1ec481557d69`
- Harness: `main@cae97afe848d79957c9d6c698a90996a758bfd56`
- TierFlow: `main@50822f11ee63c3d80f631f8fea7dcb9b19bffc38`
- Labs context: `main@022608f5797e7b4a14ccc4c393bd3c5a4704a4be`
- Gate: **SOURCE_LANDED / RUNTIME_UNVERIFIED / BETA_NO_GO**.
- Canonical release truth is **`integration/beta-gates.json` and `integration/release-manifest.json`**, not this packet.
- Preserve current Host as the sole effect/World writer, the existing mobile PWA, the signed-in Chrome Studio sender and TierFlow as an inert internal visual layer.
- This audit did **not** promote HVI/Phase 2/pre-G foundation branches or merge an installer.

## Critical findings in 30 seconds

- Mobile PWA 87 donor paths: **83 exact**, **4 adapted**, **0 missing**.
- TierFlow source: **85/86** with external HQ-spawning adapter intentionally removed; **80/80 original tests** and **12/12 browser assets** preserved.
- Studio source: **269/269** `studio/src/` files byte-identical. **0/305** donor `studio/test/` files migrated despite imported test scripts referencing them.
- Browser PWA signer and Host static allowlist patch are on main, but per-request nonce/replay refusal and physical browser acceptance remain unproved.
- TierFlow final-wave, World bridge, typed commands and renderer are source-present; no verified real Host/Desktop/PWA mount or single-publisher cutover.
- Root `Cargo.lock` and `bun.lock` missing; Windows/MSI, live provider, no-donor isolation, full visual/a11y testing remain UNRUN.
- Current HVI main is proposal documentation only, not a version-diff inspector deployed inside the UI.

## Method and interpretation

Comparisons used the connected GitHub recursive Git trees at the pinned SHAs and selected file inspections, with each file classified by identical Git blob IDs at mapped destination paths. `NOT_IMPORTED` is directional **source inventory**, not automatically a beta bug: excluded Android-native material, historical docs, output logs, TierFlow executors and unsafe donor adapters should not be reimported by reflex. Human analysis assigns consequence and recommended owner.

**Next step:** reviewer/integrator should check this packet, then reconcile it into the canonical Proposal 5 acceptance ledger after actual build and device tests. Do not mark an unrun test PASS.
