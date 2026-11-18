# CHATGPT COPY PROMPT — PHASE 2 C1: CORE DEVELOPMENT/MIGRATION PREPARATION

You own **ARBOR Proposal 5 Phase 2 / CORE**. Work through connected GitHub directly, in this ChatGPT conversation, without interfering with W1 A–F.

Destination: `owenservera/ARBOR`. Source branch: `architecture/proposal5-beta-first-centralization`. Create branch `work/p5-p2-core-labs`. Donor Labs: `owenservera/vivim-labs` current `main`, `lab/user-io-rust-core-20261112`, `work/chronos-toolkit-2026-11-08`; relevant Harness R001 branch. Read Proposal 5, `10-PHASE2-LABS-INTAKE.md`, `11-PHASE2-DECISION-MATRIX.md` and beta gates.

**Investigate and prepare core source migrations:**
1. **Personal knowledge T=0**: deeply inspect `personal-knowledge-lifecycle/runtime/src/` including ZIP32/ZIP64 parsing, streaming JSON, ChatGPT normalization, fast-lane snapshot, Host ledger port, checkpoint/reconciliation and scoped graph proposal. Evaluate integration into existing **B-owned Host/World** and **A-owned graph** without new persistent DB/authority. Map exact functions and contracts; detect unsafe archive traversal, memory/storage bound, revoked consent, identity mismatch, private data retention, retry/idempotency gaps.
2. **Node 0 Prime**: review R001 Rust prototype, user-io contracts and current source/native beta obligations. Propose **minimal safe replacement** for mutex-held fan-out and actor/session problems. Decompose exact A-native, B-Host, E-UI and F-contract contributions—do not implement a competing native ingress.
3. **Chronos**: inspect staged `chronos.rs`, `hqkg-time.rs`, source/contract tests, Host Studio read projection and TierFlow read model. Compare against existing HQKG/scheduler and release surface; identify small native/pure imports without a second timer daemon.
4. **Voice-to-tasks**: audit Labs design and *actual* PocketRecorder/PocketStore/Host capture caller source where accessible. Distinguish working capture, transcription, structured task proposal and approved Host commit; identify the minimal verified Day-0 path, if any.
5. Create a **source-specific integration patch/contract dossier** with: source SHA/path, target ARBOR owner and path, adapter API, data/actor/source scopes, import dependency closure, tests/gates, expected failure modes, proof status and whether promotion is critical for beta or deferred.
6. Optionally add **small synthetic contract tests/fixtures** under the Phase 2 prep directory; never claim to wire source into ARBOR live runtime before W1-G.

**Write only** `.integration/proposal5/phase2/core/` in your own branch; commit `INTAKE.md`, `interfaces.md`, `migration-order.md`, `core-manifest.json`, `HANDOFF-G.md` and self-contained test fixtures where safe. Do not edit W1 A–F owner paths, root locks, current integration/main, or Labs. No provider APIs, Playwright, new graph store, separate Host journal or external Labs runtime.

Return exact branch/HEAD, individually scoped owner-ready work orders for W2/W3, real tests and blockers. **This is serious source work, but parallel-safe preparation—not a stealth rewrite.**