# ARBOR PWA — Pre-local comprehensive upgrade

Source: `93a6fc922b225dabbdc03414c925e70c3a062b9d` on `owenservera/ARBOR/main` (2026-11-13). This is a **source implementation**, not a Windows/Android release acceptance. No new branch, pull request, provider API, phone executor or customer Git dependency was added.

## What you will see

**Today** — The existing six-tab PWA now offers **Find any feature**, **Quick capture · works offline**, and the same real Host-aware next actions. Press `Ctrl+K` or `/` outside a text field to find a capability. The finder searches a fixed catalogue of 15 existing features locally, disables unavailable actions and checks current device authority before navigating. It **does not send commands**.

**Me** — Device readiness lists 10 checks: online status, secure browser, Host authority, Direct/Cloud, permission scope, signed-in AI availability, media metadata, offline shell, installation and unresolved sends. **Check connection** only re-reads or resumes existing authority. **Copy safe report** omits Host content, addresses and credentials.

**Quick capture** — Notes require explicit Save. Maximum 12 notes, 1,200 characters each; a full notebook **refuses** another save and **never evicts** previous data. Notes remain in this browser's **unencrypted** localStorage, with explicit Copy, Download JSON, Delete and double-confirmed Delete all. They never sync to Host, execute tasks or submit AI requests. If storage is malformed, it is **not silently overwritten**. Native Pocket and Host voice/task workflows remain separate.

**Build fix** — `generative-ui/host-revision-guard.mjs` was an actual browser-import dependency missing from both the Direct Host static allowlist and offline shell. Both now serve/cache it. The new `pwa-asset-graph.test.mjs` checks the complete relative import closure, source files, script tags, Direct static whitelist and offline list.

**Offline and installation** — Existing cache evolved to `vivim-relay-shell-v8`, with 58 declared shell assets including the upgrade modules. No private Host state, stream bytes or API responses are service-worker cached. Manifest PNGs and a maskable icon remain available from the prior upgrade. Browser install confirmation is still device-dependent.

## One local command — no extra agent/gate

From the **ARBOR repository root**, run:

```powershell
node integration/scripts/pwa-run-now.mjs
```

It runs real Node source checks and the dedicated mobile, Host security and media HTTP tests; prints per-suite exits plus a machine-readable summary; returns a nonzero process exit on failure. This is developer verification only. Customers do not need Git or this test harness.

## What to inspect in the actual app

1. Start Host normally. Its visible root is the phone-sized Today PWA. Desktop owner view is read-only; Host Control is deliberately a separate utility.
2. Open paired Android Chrome over your Tailscale Direct HTTPS URL, confirm the current device approval and `control:read` grant. Do not infer approval from Wi-Fi proximity or the Host owner preview.
3. Check Today launcher/search and Me readiness in online/Direct/Cloud and denied or unpaired states. A revoked session must remove private views and disable protected shortcuts.
4. Install PWA when Chrome offers it. Switch offline: Today and Quick Capture remain available; provider sends and protected Host actions must not replay.
5. Save a note, copy/export JSON, reload offline, delete it, test the 12-note limit. Note storage is explicit and unencrypted; do not put secrets in it.
6. In Build, preview a deterministic card edit. A draft must never become a Host change without explicit approval and guarded revision.
7. In World, browse beyond 25 HQ titles. Test streaming/seek/close/revoke **only if** an owner-approved explicit `playback.json` file mapping exists; a fuzzy torrent filename match is not sufficient proof.
8. Read a pending/uncertain send from Control. Reconnect must not repeat it automatically.

## Exact evidence and remaining uncertainty

Source-level V8 test callbacks and production pure modules: **70 pass / 0 fail**:
- Phone UI 34/34
- New readiness/notes 11/11
- New launcher/notes interactions 3/3
- VEC compiler 9/9
- Build routing and composer 10/10
- Media-stream grant guard 3/3

This does **not** mean Node, actual HTTP, Windows Host, Chrome installation, phone pairing, codecs or Tailscale recovery passed. Those remain **UNRUN**, as does the new Node import graph test. GitHub Actions has repeatedly returned `startup_failure` before starting jobs. Keep original queue item **PWA-12** open for the local acceptance. No synthetic test result should be promoted as a real device pass.
