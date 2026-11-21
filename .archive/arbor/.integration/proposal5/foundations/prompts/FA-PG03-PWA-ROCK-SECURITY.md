# ARBOR Proposal 5 — FA-PG03 / PWA-First, Host Trust, Private Knowledge and Rock Storage

**EXECUTE NOW with connected GitHub** as isolated preparation. Read `.integration/proposal5/foundations/prompts/00-SHARED-PRE-G-CONTRACT.md`, `PWA-FIRST-DELIVERY.md`, `NATIVE-MACHINERY-AND-CONTRACTS.md`, `SOURCE-ATLAS.v1.json`, `GAP-REGISTER.v1.json`, `WAVE-MANIFEST.v1.json` first.

**Repository:** `owenservera/ARBOR`.
**Create:** `work/p5-fa-pg03-pwa-storage-r001` from the **current pinned** `work/p5-foundational-architecture-prep-r001` HEAD.
**Exclusive writes:** `.integration/proposal5/foundations/proofs/PG03-pwa-storage/` ONLY.
**Tasks:** `FA-PG03-01..FA-PG03-08`.
**Owner policy:** **PWA FIRST; full APK postponed**. Prepare exact B/E/A/G handoffs, not active security or packaging patches. **Disposition:** `PREPARED_NOT_PROMOTED`.

## FIRST — read actual source and verify pinned refs

- **E PWA/TierFlow:** `work/p5-w01-e-ui@1a5cf26ce14ebce4e31d9a4373060ace61852645`
  - `apps/mobile-relay/{manifest.webmanifest,sw.js,browser-device-key.mjs,device-enrollment.mjs,control-client.mjs,control-language.mjs,pairing-link.mjs}`.
  - `apps/mobile-relay/generative-ui/{compiler,phone-surface,host-revision-guard}.mjs`.
  - `packages/tierflow/src/visual-runtime/{device,host,revisions,render}/` and actual UI World source.
- **B Host:** `work/p5-w01-b-host@78a5105a9c1f1880d658f350e130b1e2632d3532`
  - `apps/vivim-host/{direct-phone.mjs,server.mjs,apk/enrollment.mjs,apk/pairing.mjs,phone-sessions.mjs}`, `apps/vivim-host/control/{dispatcher,journal,intent-planner}.mjs`, `apps/vivim-host/bootstrap/ledger.mjs`.
  - Static allowlist is explicit in `direct-phone.mjs` (observed blob `6bb604cf7a64e4623c28bc86495f39a1f40f1626`): includes `device-enrollment.mjs`, `sw.js`, `manifest.webmanifest`. **`browser-device-key.mjs` was not found in this observed allowlist**; verify current file and all routes before claiming a patch.
- **A Tauri/native:** `work/p5-w01-a-native@dda546a11294920d6521a0f64ab7ee62ec993c70`
  - `src-tauri/{tauri.conf.json,capabilities/default.json,src/main.rs}`, Host child runtime and process token boot source.
  - `tauri.conf.json` blob `f9474531f0194268c6688ac56c1b48d7ba3fff3a` explicitly names required `resources/vivim.apk`. Initial beta must remove this dependency through A/G **after G**, not fabricate a binary.
- **D HQ:** `work/p5-w01-d-hq@ad55fe28c2c331841675f308f9199ee4c2add86a`
  - `hq/lib/{wiki,refresh,schema,kb,arbor-service,kernel/store}.mjs`, actual SQLite/FTS/wiki writes and rebuildable vs protected classes.
- **F interop/verification:** `work/p5-w01-f-verify@be458f3c4cdb7eb38d47b1de2a00d441b7cdc704`, actual test and source contracts.
- **HQ Rock pure research:** `owenservera/repos-hq`, `research/rock-core-storage-bounds-20261112`: `research/rock-core-storage/{storage-policy.mjs,rock-membership.mjs,PRD-RC-001.md,ARCHITECTURE-RC-001.md,test/*.test.mjs}`. Compiler blob for `storage-policy.mjs`: `4606b90c2635564e74c712e19771c5d244576e3e`.
- **Labs private source:** `owenservera/vivim-labs` main: `personal-knowledge-lifecycle/runtime/src/{t0-core.mjs,host-archive-wiring.mjs,chatgpt-import.mjs,zip-reader.mjs,scoped-graph-proposal.mjs}` and real tests; no real user archives.
- **Existing Phase 2 K/T:** `work/p5-p2-preg-knowledge-r001`, `.integration/proposal5/phase2/pre-g/K-knowledge/`, and `work/p5-p2-preg-tests-r001`, `.integration/proposal5/phase2/pre-g/T-tests/`. **Read both** to avoid rebuilding existing consent/test preparations.

## Mission — make fast PWA safe without turning it into an APK project

### Track A: PWA current source, concrete requests and actual security owner

1. Enumerate the exact **direct Host HTTPS route** for `/mobile`, shell, JS/CSS/manifest/SW, pairing, permissions, control planner and effect endpoint. Map current source module imports to the Host static allowlist and response headers. Distinguish browser P-256 `CryptoKey` in IndexedDB and **Host-approved grant/nonce**; a nonextractable key is not a blanket permission and same-origin XSS remains a risk.
2. Build exact B/E protocol crosswalk for `device-enrollment.mjs` calling `browser-device-key.mjs`, signature transcript/challenge/nonce format, Host grant writer, revocation, per-request binding, audience, method/path/body and plan/receipt. Mark unverified runtime paths explicitly.
3. Analyze A's `src-tauri/capabilities/default.json` broad `127.0.0.1:*` remote scope and child/localhost persistent-token defect. Specify A/B post-G **source-file-level repairs**, never store credentials in a research fixture.
4. Analyze E `sw.js`: it caches only the read-only shell; authenticated API/effects must NEVER be cached or automatically retried. Preserve real offline limitations. Read `control-client.mjs` for actual receipt and uncertainty semantics.
5. Define physical phone acceptance with actual Android Chrome/secure HTTPS, installation, lost-key/re-enrollment, owner grant revoke, session timeout, keyboard+IME, screen reader, paired focus and offline/reconnect. Desktop browser tests are not device acceptance.

### Track B: PWA-first packaging and capability UX

6. Enumerate all A/G release asset dependencies and **exact** missing `resources/vivim.apk` references. The existing `apps/vivim-host/apk/` holds reusable grant logic; do not recommend deleting it because the directory name says "apk." Draft the smallest post-G packaging change (remove mandatory binary resource/installer references, keep grants). Full APK signing/Android bridge packaging remains **future** and excluded from release.
7. Map E UI/TierFlow PWA and B Host/Studio into one permitted semantic composer and AutoWiki source read interface; no phone effect authority or duplicated PWA server. Profile `read_only_shell`, `offline_unverified`, `source_stale`, `reconnect_requires_current_world`.

### Track C: Rock Core and protected source versus derived data

8. Read actual D HQ `wiki.mjs`, `schema.mjs`, `refresh.mjs`, `kernel/store.mjs`, A journal code and B user data. Build **actual byte inventory categories** (SQLite DB/WAL/SHM/FTS/wiki pages/graph journal/staging/cache/source vault/Host receipts/TierFlow World/PWA shell). Classify `PROTECTED_UNIQUE | DERIVED_RECOVERABLE | DERIVED_UNPROVEN | TEMPORARY` with required recovery proof.
9. Compare HQ Rock pure policy/membership code against actual writer APIs. The "Full" mode cannot override OS reserve; don't delete original personal exports or accepted records. Do not mark notes, receipts, World or kernel store as disposable.
10. Prepare a small **isolated read-only synthetic storage dry-run** (if source compatible): disk pressure/reserve, unbounded wiki page materialization, incomplete source recovery proof, permission revoke, crash/scratch-space guard, PWA offline. Tests may call pure `planRockStorage` via checked donor source in temporary workspace; no actual disk deletion or VACUUM.
11. Integrate P2-K consent/checkpoint/erasure findings **as source ownership constraints only**; no personal corpus import or secondary DB.

## Required outputs (exclusive subtree only)

- `README.md` — pinned source paths/current HEAD, B/E/A/D/F owner and runtime evidence truth.
- `SOURCE-DELTA.json` — per-module imported/prepared/missing/design-only with exact SHA and active source owner.
- `PWA-AUTH-AND-PACKAGING-MATRIX.md` — exact client↔Host endpoints, allowed origin, grants, signer, static assets, Tauri APK resource and smallest post-G B/E/A/G fixes.
- `STORAGE-CLASSIFICATION.json` — physical kind, source owner/writer, protected vs recoverable, positive proof needed, safe compaction gate.
- `PROOF-PLAN.md` — real post-G test commands and source-specific patch seams; optional pure local test/fixture.
- `NEGATIVE-ACCEPTANCE.md` — spoofed native Host, stolen device grant, revoked origin, replay, offline SW POST, key lost, ENOSPC, original record deletion, unverified source rehydrate.
- `OWNER-HANDOFF.md` — **no more than five** real code-owner tasks (P0 security first, PWA before APK).
- `RUN-LEDGER.json` — immutable source HEADs, actual tests vs UNRUN, `PREPARED_NOT_PROMOTED`.

## Stop

No patch to `src-tauri`, Host static map, real grant store, wiki/index DB, phone service worker or original user files from this lane. No edits outside `.integration/proposal5/foundations/proofs/PG03-pwa-storage/`. Commit, GitHub diff scope check, report exact HEAD and **PREPARED_NOT_PROMOTED**, STOP. Don't hold G or misrepresent security acceptance.
