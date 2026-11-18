# ARBOR Proposal 5 — PRE-G FINAL FREEZE, LANE B

**Freeze declaration:** `FREEZE_READY` — source/evidence frozen for sole integrator G; **NOT** integrated, build-verified, security-accepted or device-accepted.
**Repository:** `owenservera/ARBOR`
**Branch:** `work/p5-w01-b-host`
**Final frozen HEAD:** commit containing this `PREG-B.md` report; the immutable branch tip must be read from GitHub after this commit. This avoids impossible self-referential commit hashes.
**Immediately preceding HEAD:** `c12f4a27a8c700c9df4cba17a9ff96efd5897748`
**Original common ARBOR base:** `dd245d78ff3c110ba8630cad557ea0479c51a635` (`architecture/proposal5-beta-first-centralization` at branching). No W0-G approval or integration-base existence inferred.
**Work-order source:** `work/p5-pre-g-convergence-r001:.integration/proposal5/pre-g/prompts/R2-B-FINAL-FREEZE.md`

## 1 — Physical import and complete source inventory

- Donor: `owenservera/vivim-web-harness` on `main`, pinned commit `cae97afe848d79957c9d6c698a90996a758bfd56`.
- Donor directory: `apps/vivim-host/`; donor Git tree `f7dc72f3045f7fcd4a67e680e52b97ec29aefb64`.
- Original source landing was committed in 11 source-copy commits culminating in **`5fc913251ccfdf400edefd3c720c5eb5371a7204`**. The source import was followed by the inventory/report commit `9c52c35d473e15c86939c786f0d5e3819da5c302`.
- **135 tracked donor source/assets/test files imported (1,231,234 source bytes), exact original internal paths; source Git blob identity 135/135 matches, zero mismatches/missing.** Original imported content remains unmodified on this worker branch. A nontruncated Git tree read at `c12f4a27` confirmed 135 Host blobs. Git compare from common base reported 138 added files: 135 Host files plus three evidence/report files at that point.
- **Exhaustive per-file source blob SHA, size and Git mode inventory:** `integration/worker-reports/W01-B-files.json` — the canonical complete inventory; no need to duplicate 135 entries here.
- **Import details and 20 cross-owner interface/path requirements:** `integration/worker-reports/W01-B.md`.
- Retained top-level Host file counts: root (9); `apk` (3); `bootstrap` (15); `control` (19); `conversations` (4); `corpus` (9); `data-intelligence` (2); `grokbot` (3); `media` (1); `onboarding` (3); `public` (11); `recovery` (4); `runtime` (1); `test` (51).
- Only two excluded donor files, both generated TAP results (NOT authoritative tests): `apps/vivim-host/corpus/INTEGRATION-5-ISOLATED-TESTS.tap` and `apps/vivim-host/corpus/transport/test-results-2026-11-06.tap`.
- No personal database, credential, Chrome profile, protected World data, live effect journal, local owner source index or user histories were imported. A real content/secret scanning and packaging check remains required for release, not claimed passed by this inventory.

## 2 — Exact Worker commits and change classification

The import was committed incrementally (11 import commits) before the two source inventory/report commits. Final immutable points and first-follow-up commit:

| Commit | Purpose | Code changed? |
| --- | --- | --- |
| `d08261e8649307e964d55b6fc0da86c65a2c247e` | Host import batch 1 | Source added, original donor bytes |
| `df773ee6a70694eb67bdc9482fff5a1c307a7c08` | Host import batch 2 | Source added, original donor bytes |
| `15681e31e3a0bc575a4f2825491ba5af506ba664` | Host import batch 3 | Source added, original donor bytes |
| `5de9d4420aedd90be63c980fca05d10cecd37c21` | Host import batch 4 | Source added, original donor bytes |
| `eaa24560ea79f0470fa7d3bb7b03ae877960c435` | Host import batch 5 | Source added, original donor bytes |
| `90646c4a8672ba8ea14f13827171421ad942c988` | Host import batch 6 | Source added, original donor bytes |
| `81db449f8fca486f6cbedb5a4c4879e0c4917748` | Host import batch 7 | Source added, original donor bytes |
| `57c55529bf87b5513a340c82605fee6ee7aa2923` | Host import batch 8 | Source added, original donor bytes |
| `e3a86234509aacd94dc37240215765dc404db3c6` | Host import batch 9 | Source added, original donor bytes |
| `230c656c069d1a2c186f24365ea17e6688312e1f` | Host import batch 10 | Source added, original donor bytes |
| `5fc913251ccfdf400edefd3c720c5eb5371a7204` | Host import batch 11 — all 135 landed | Source added, original donor bytes |
| `9c52c35d473e15c86939c786f0d5e3819da5c302` | `W01-B.md` and `W01-B-files.json` | Documentation / provenance only |
| `c12f4a27a8c700c9df4cba17a9ff96efd5897748` | First focused follow-up `W01-B-FOLLOWUP.md` | Documentation / design only |
| **This final report commit** | `PREG-B.md` | Documentation / freeze only |

No second Host, action catalog, journal, personal store, UI, provider, API bridge or actual bootstrap repair was added after donor import. Full worker compare had no out-of-scope paths before the freeze report.

## 3 — Canonical Host authority and source semantics

- **B alone owns** `apps/vivim-host/`, World state, Host action catalog v6 (**19 source actions**), existing NLCL/intent planner, dispatcher, approvals, Host bootstrap ledger, effect claim/receipt journal and recovery.
- `control/action-catalog.v1.json`: v6, 19 actions, same risk/readiness semantics as donor. F's catalog comparison is a test baseline only, never a second executable catalog.
- `control/dispatcher.mjs`: checks exact plan digest, expiry, current action/risk/catalog and matching approval channel/source, rejects R3; R2 changes require desktop owner token.
- `control/journal.mjs`: effect fence is exclusive `<id>.claimed` create (`wx`); a claim without receipt becomes `uncertain` on restart. Recorded receipt replay does not execute another effect. This is SOURCE INSPECTION, not a newly executed crash/recovery test.
- Native/Rust (A), Studio/browser (C), HQ/dev intelligence (D), mobile/TierFlow (E) and contracts/tests (F) cannot grant/write World effects independently. No real-world data or journal migration was performed.

## 4 — Security P0 S1: native ↔ Host launch, A+B+G

**Observed unfixed:** A's imported `src-tauri/src/main.rs` trusts HTTP success from fixed `127.0.0.1:43187/api/host/status`; `desktop_launch_url` then sends durable owner Bearer to `/api/host/control/ui-session`. A foreign localhost listener could impersonate readiness and receive a sensitive credential. No verified process binding exists.

**Proposed, NOT IMPLEMENTED:** `arbor.host-launch/1`, jointly reviewed A/B candidate described in `W01-B-FOLLOWUP.md`. A owns secure OS-spawned child, per-launch secret/IPC and process identity; B owns Host challenge verification, ephemeral one-time launch code issuance via the **existing** control UI-session store and fail-closed replay/port handling; G owns integrated lifecycle and lockfiles. No new owner authentication authority or token store. Preferred inherited OS channel/pipe must be assessed on Windows; same-user env secrets have limitations. Native must stop delivering persistent bearer to unauthenticated port. Test forged status, port occupied, mismatched child ID/PID, nonce and proof replay, version, concurrent launch, crash during launch, no token disclosure and existing CLI/ACL compatibility. Until physically patched and adversarially tested: **SECURITY_BLOCKED / S1 P0**.

## 5 — Security P0 S2: E browser PWA, Host static patch and active bearer replay

**E's separate branch evidence:** `work/p5-w01-e-ui` added browser WebCrypto P-256 / IndexedDB device key code and synthetic tests; E's `integration/worker-reports/W02-E-PREINTEGRATION.md` documents this. Not proof of physical/browser pairing.

**Exactly one required B-owned static allowlist repair (NOT APPLIED):**
Source `apps/vivim-host/direct-phone.mjs`, static `assets` map, insert immediately after `device-enrollment.mjs`:
```diff
   'device-enrollment.mjs': ['device-enrollment.mjs', 'text/javascript; charset=utf-8'],
+  'browser-device-key.mjs': ['browser-device-key.mjs', 'text/javascript; charset=utf-8'],
```
Patch provenance: E's `integration/worker-reports/W02-E-B-HOST-STATIC.patch` on `work/p5-w01-e-ui`. **Owner decision:** mark REQUIRED_G/W2_B, not safe to apply pre-G since E's source and B's Host are in isolated branches. Without patch the browser module is 404, so **PWA browser onboarding remains BLOCKED**. After G source integration, B applies one-line Host patch and E/F run Host HTTP smoke + service-worker asset parity.

**Separate active-session bearer-replay threat (B/F/G review required):**
E's source audit of `apps/vivim-host/apk/enrollment.mjs` found `validateGrant({grantId,sessionId})` accepts active unrevoked grant/session ID, expiry and equality but no *per-request* private-key proof. Thus theft of the **active grant ID + session bearer** may permit replay for the remaining session validity window (E reported up to four hours), whereas a stolen grant ID alone cannot mint a fresh signed session. Nonextractable IndexedDB keys are not guaranteed hardware-backed or invulnerable to compromised same-origin script; existing origin/CSP/auth remain important.

**Decision/verification work order, not implemented:** B and F should first enumerate every `/mobile` Host route using `validateGrant` and distinguish private reads from approved effect submissions. Threat model local malware, malicious same-origin browser script, network-proxy exposure, stolen headers, offline cache, revoked grant, device clock drift and legacy native Android signer. Determine required protection with A/E: request-bound proof of possession (method, normalized URL/path/query, body digest, nonce, timestamp, grant/session/actor/source revision) verified by existing Host grant authority and one-shot replay cache; or explicit compatible mitigation backed by policy/security testing. Preserve pending/approval flows, one Host writer, old native `VivimDeviceKey` compatibility, and safe client rollout. Do **not** silently require new request headers from deployed old clients until backward-compatibility and version negotiation are covered. A shorter TTL limits exposure but does NOT eliminate active bearer replay; do not claim stolen bearer rejection. Required negative tests: stolen grant alone, stolen active headers without signed request, nonce replay, method/body/path tamper, grant revoked, session expired/replaced, cross-origin use, duplicate request, older signed native client and actual Windows/phone network path. Classify **S2 AUTH_REPLAY_REVIEW_REQUIRED / SECURITY_BLOCKED**, no unobserved security acceptance.

## 6 — Cross-owner exact unresolved linker dependencies

These imports existed in the donor and must be supplied **inside ARBOR** by their source owners after G combines trees; no external donor checkout/process or invented stub:

| Owner | Exact required paths / interfaces |
| --- | --- |
| **A** | `src-tauri/src/main.rs` Host child start/exit; trusted native ↔ Host launch; origin-limited IPC, patched Tauri, packaging and bundled runtime |
| **C** | `studio/src/config.mjs`, `studio/src/workspace-manifest.mjs`, `studio/bin/studio.mjs`, `studio/src/studio/bridge-commands.mjs`, `studio/src/studio/bootstrap-owner-adapter.mjs`, `studio/src/studio/relay-journal-correlation.mjs`; test-only `studio/src/studio/worker-heartbeat.mjs`, `studio/src/capture/thread-turn-journal.mjs` |
| **D** | `hq/lib/arbor-service.mjs`: Node24+ dev-source read/index/context/wiki, Host-authorized roots and per-user data dir only, not a World writer or implicit personal-source access |
| **E** | `apps/mobile-relay/lib/relay-core.mjs`, `control-language.mjs`, `nlcl/index.mjs`, `generative-ui/catalog.mjs`, `generative-ui/compiler.mjs`, `bootstrap-f01.mjs`, `bootstrap-f02.mjs`, `bootstrap-pass.mjs`, `theme-plan.mjs`, `bootstrap-prompts.mjs`, `consumer-commands/registry.mjs`; Host serves `browser-device-key.mjs` after required B patch |
| **F** | `tools/kpi/utc-maturity.mjs` test-only dependency of `test/kpi-utc.test.mjs`; Host v6 action/risk diff, approval, actor scope, privacy and fault-injection acceptance |
| **G** | Root Cargo/Bun manifests/locks, source-lock, mixed branch merge, exact internal cwd/import closure, bundled assets, low-disk/storage, clean no-donor build and release gates; no second authority |

**One-way authority:** Studio only supplies browser evidence; mobile is an authenticated Host client; HQ provides scoped development reads; TierFlow only projects; native can compute/read and launch but must never duplicate Host effect admission. Preserve existing source files and current navigation/media/mission functions where operational; missing actual runtime proof is `UNRUN`.

## 7 — Tests: executed vs UNRUN, with exact exit codes

| Verification | Observed | Exit code |
| --- | --- | --- |
| Git donor/import blob parity | **135/135 matching**, zero missing; static source-object comparison | N/A (Git API read, not process execution) |
| GitHub worker tree / scope check | **135 Host files**, compare from base showed source plus only reports | N/A (Git API read) |
| Source catalog JSON v6 / 19 actions | Read/parsed via GitHub tooling; source-present | N/A (not an executable Host test) |
| Source inspection of Host claim/receipt/uncertain logic | Read-only, preserved | N/A |
| `node --test apps/vivim-host/test/*.test.mjs` | **UNRUN** | N/A |
| `node --test apps/vivim-host/bootstrap/*.test.mjs` | **UNRUN** | N/A |
| `node --test apps/vivim-host/corpus/**/*.test.mjs` (shell glob compatibility depends on runner) | **UNRUN** | N/A |
| Native → Host bootstrap / port impostor security tests | **UNRUN** | N/A |
| Host journal crash/uncertain/receipt fault injection | **UNRUN** | N/A |
| Browser WebCrypto PWA + Host HTTP asset and enrollment | **UNRUN by B** | N/A |
| Physical Android PWA pairing/revoke | **UNRUN** | N/A |
| Windows/MSI / Tauri security / donor-absent runtime | **UNRUN** | N/A |
| Real owner World data export/migration/restore/rollback | **NOT PERFORMED** | N/A |

Other lane E's isolated Node synthetic pass is not a B test and is not proof of integrated ARBOR Host readiness. No made-up exit codes or device acceptance.

## 8 — Classification and G integration handoff

- **SOURCE_IMPORTED: YES** — 135/135 identical Host source blobs in `apps/vivim-host/`.
- **SOURCE_PATCHED: NO** — all original imported Host runtime modules unchanged; first focused follow-up only added a documentation/security work packet.
- **BUILD_UNRUN: YES** — no Node/Bun executable run or combined app build in this lane.
- **DEVICE_UNRUN: YES** — no actual Windows/MSI/browser/physical phone acceptance.
- **HOST_RUNTIME_WIRED: NO** — combined A/C/D/E/F imports and root launcher not yet exercised.
- **SECURITY_ACCEPTED: NO** — S1 native-process trust and S2 active-bearer replay remain P0 gate items.
- **FROZEN_WORKER: FREEZE_READY** — for **source merge input only**, NOT beta GO.

**Precise G order:** (1) freeze/read each worker PREG-A..F immutable tip and verify real lineage; (2) create shared integration from accepted Proposal 5 architecture baseline; (3) import this B branch once, preserving Host source and W01 reports; (4) merge C/E before running full Host tests and ensure all 20 import paths plus test utility resolve internally; (5) apply E's exact one-line **B-owned** static patch only after merged source is present; (6) task W2 A+B with trusted child handshake and remove native durable-token-to-HTTP path, then adversarial Windows tests; (7) task B/E/F with active-bearer replay threat model, versioned signed-request policy and backwards compatibility tests, and record an explicit security disposition; (8) link D's `hq/lib/arbor-service.mjs` through Host authorized dev-root read port, no personal World writes; (9) run signed-in provider, Host effect journal fault-injection, PWA physical pairing, owner-safe backup/restore, Windows package and **donor-checkouts-removed** acceptance; (10) do not authorize user World data cutover or beta GO without observed gates and explicit owner approval.

**Stop rule:** after final PREG-B commit, stop editing the worker branch. Only G may merge. This report is complete evidence for that freeze, not a request to run the implementation inside this same pre-G lane.
