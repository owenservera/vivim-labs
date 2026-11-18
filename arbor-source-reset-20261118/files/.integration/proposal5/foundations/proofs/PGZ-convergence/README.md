# ARBOR Proposal 5 — PGZ Independent Foundation Convergence Referee

**Disposition: PREPARED_NOT_PROMOTED.** This is an independent source/contract referee, **not** W01-G; it did not merge, cherry-pick, run a product, create a user grant, ingest personal data, or edit any A–F, Phase 2, G, main, runtime, root lock or manifest.

## Observed refs (2026-11-12, connected GitHub)
- Exact foundation parent: `80c29fbc0d45afbce0fadde5388a8f912cdd2762` (tree `f6d4d621fab456f99eaa2dbe14bcba750e284b72`); PGZ branch created directly from it.
- PG00 `f8415bb793b1a88b920b3ac509f4820e7a4f5e46` (1 commit, 9 scoped files), PG01 `78e43165f76c1f6ad18a63b725d75d40501259ce` (1/9), PG02 `021864846783d28679b29e563490d4bb100ac079` (2/12), PG03 `82fcffc1217ac58dd323627c698d71ed422a9534` (1/10). **40 changed worker files, 0 out of scope** verified via four GitHub compares from foundation parent.
- Frozen A `dda546a11294920d6521a0f64ab7ee62ec993c70`, B `78a5105a9c1f1880d658f350e130b1e2632d3532`, C `f950b91685927f37b48d020433aa8d2d3c271419`, D `ad55fe28c2c331841675f308f9199ee4c2add86a`, E `1a5cf26ce14ebce4e31d9a4373060ace61852645`, F `be458f3c4cdb7eb38d47b1de2a00d441b7cdc704`: Git refs match shared contract; PREG-A–F are FREEZE_READY **for source only**.
- S `d7e59a389a6ec5278595e317dfa904a801783c5b`; V `7ad5b3c49aefdf1fdd8b212e5b7eea0e7b92338e`; W `827d21d1c33aa8597c600b45179452abc35c7af9`; N0 `cde3b6e5521d5b17215ab10088cb8090f8b87e6f`. P2-T `c807010e52b4a9f82ea9578a979befd38f74cb31`, P2-K `4db9e0c2068be36ab42663a76186422c8f9813a1`, P2-C `73b1ea769de6023dfff857d7e9c4003731f3b112`, H1 `3e6954d21bd70eeccc6e00a310bcb337c3164f50`.
- **Original G:** `integrate/proposal5-beta-r001` returned 404 during PGZ preflight; **G_NOT_RUN**, not G_ACCEPTED. The sole G protocol is `.integration/proposal5/prompts/W1/G.md` on `work/p5-pre-g-convergence-r001` (Git blob `192f56e5cabc0432a7b18ce3d2a2e2aa8917dd79`). No `integration/reports/W01-GATE.md` on an accepted G ref exists to inspect. No PG proof becomes a prerequisite that delays or usurps original G.

## Independent findings
1. All worker proof outputs are scoped and source-only. PG00's canonical owner map is consistent with the narrower PG01–03 source maps: A = existing pure Rust/HQKG engine/typed DAG; B = Host text, policy/World, claim/approval/receipt and private source grant; C = signed-in Chrome provider sender; D = dev wiki/source registry; E = PWA/TierFlow UI publisher; F = inert ABI/test mirrors; G = root serial integrator.
2. Host v6 catalog has 19 declared IDs: 17 `source_wired`, one `enforcement_missing`, one `unsupported`. `source_wired` is **not** RUNTIME_VERIFIED. Synthetic full-project-update is not a registered Host action. Source semantic tags never create a handler.
3. PG01's generational registry and typed DAG are real source, but unsigned caller descriptors, untrusted SourceBinding, nonpreemptible execute and untested Cargo mean **native reprogrammability NOT YET VERIFIED**.
4. PG02's inert semantic/visual crosswalk has a worker-reported local Node.js v22.16.0 test receipt: `node --test test/semantic-visual-crosswalk.test.mjs`, exit 0, **20/20 synthetic**; code/test blob pins were independently read. PGZ did **not** rerun it. This proves neither live source ownership nor Host/Chrome/device.
5. P0: B direct-phone omits E browser-device-key.mjs static route; active grant/session bearer lacks per-privileged-request signing/replay proof; A native launcher trusts fixed loopback before owner-session exchange and has wildcard Tauri remote capability. APK resource is mandatory in frozen Tauri config despite **PWA FIRST**.
6. Rock source is pure planning; actual physical bytes, safe row/store classification, WAL/-shm/temp, source rehydrate and ENOSPC/crash invariants remain untested. No user ZIP/memory or protected original shall be ingested/deleted.
7. Existing S/V/W/N0/P2-T/K/C/H1 modules were read as sources/fixtures, not recopied or represented as new shipped behavior. No duplicate scheduler, wiki compiler/index, graph, UI publisher, actor writer or browser sender is approved.

## Evidence and gates
See `FOUNDATION-CONVERGENCE-MATRIX.json` (per-source decision, exact commit/blob where checked), `CONTRACT-COLLISIONS.md`, `MINIMUM-SOURCE-PATCHES.md` (five narrowly scoped **conditional after accepted G** owner tasks), `READINESS-MATRIX.md`, `NEGATIVE-ACCEPTANCE.md`, `RUN-LEDGER.json`.
GitHub source/branch/compare reads and eight in-tool V8 source-policy assertions were executed; **PGZ Node/Bun/Cargo/product/Windows/Android tests UNRUN**. All four worker compares show zero foreign paths. This proof branch has no adoption/release authority. Final self-SHA must be read from GitHub HEAD after commit; a commit cannot embed its own SHA.

**STOP:** only PGZ proof files; do not touch G or any worker branch.
