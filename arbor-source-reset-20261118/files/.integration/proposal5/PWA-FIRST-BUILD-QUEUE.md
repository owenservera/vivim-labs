# ARBOR Proposal 5 — PWA-first direct-main build queue

**Directive (2026-11-13):** Execute sequential, small, reviewable commits **directly on `main`**, not in new branches or pull requests. Build the real `apps/mobile-relay` PWA first, visible as Host's default entry. Use existing working sources; do not make a second phone app or rewrite Host's authority.

**Shipped source:** PWA-first cutover `931d08bf1d4f6af051c06cc2f293b8fcf51c115c`; read-only loopback preview, preserved Tailscale Direct handler, legacy Host tools only on explicit menu choice. **Execution truth:** 10 pure byte-range and focused PWA render checks executed in the connected V8 environment; no Node, Android, Windows or browser integration suite completed in this connected GitHub environment. GitHub Actions has reported startup failures, not passing test results. This is not a production acceptance claim.

## ERA 1 — Visible phone first

- **PWA-01 · Host root opens canonical PWA** — committed_unverified. root / PWA preview; legacy admin explicit; Direct routing isolated.
- **PWA-02 · Protect Direct phone and preview routing** — source_tests_added_unrun. paired and preview HTTP contract; negative POST; CI Node24 and physical phone still required.
- **PWA-03 · Surface inventory and real visual navigation** — source_implemented_tests_unrun. Today/Talk/Build/Control/World/Me visible at 390px, 420px and narrow; no stale stub; menu works.
- **PWA-04 · Authenticated read-only Host projection on desktop** — source_implemented_tests_unrun. current state/receipts only with Host session; empty/offline and denied truthful; no phone grant impersonation.

## ERA 2 — Make existing features live

- **PWA-05 · Phone pairing and reconnect UX** — source_implemented_tests_unrun. trusted HTTPS, browser P-256 key, approvals, revoke, lost-key, reconnect; no APK requirement.
- **PWA-06 · Universal text command preview in PWA** — paired_host_plan_approval_and_receipt_visualized_tests_unrun. plan, permission, approve/deny and receipt through one Host executor; no silent send.
- **PWA-07 · Build canvas visual proposal and activation** — existing_paired_flow_and_readonly_desktop_tests_unrun. preview and safe activate/undo against Host revision; stale edits rejected.
- **PWA-08 · VEC deterministic UI edits and compiler contract** — source_implemented_tests_unrun. known text edits compile locally and safely; unsupported requests use existing optional model flow; tests run.
- **PWA-09 · Mobile media library and playback entry** — source_implemented_http_tests_unrun_device_unrun. authorized library cards and streamed playback with range/cache/abort proofs.

## ERA 3 — Polish and ship

- **PWA-10 · Offline shell, drafts and installability** — offline_and_install_assets_source_implemented_node_unrun. offline read-only shell, clear stale state, no API caching/replayed mutations, valid PWA icons.
- **PWA-11 · Adaptive UI, accessibility and one-thumb polish** — accessible_mobile_source_implemented_tests_unrun. Android Chrome, keyboard, safe areas, reduced motion, screen reader; device validation.
- **PWA-12 · Windows+paired Android operational acceptance** — real_local_verifier_authored_unrun_device_unrun. Node24/Bun/Cargo plus real Windows+Android Tailscale pairing, grants, restarts and offline; no external donor runtimes.

## Working rules

- **Queue pointer:** PWA-12. For each item: inspect current `main`, implement the smallest source slice, exercise relevant executable tests when a runner is available, commit to `main` with an expected-head check, update JSON and advance.
- A recorded static inspection is not an executable test. Never mark unexecuted test suites or real devices as PASS.
- Keep Windows Host as the sole identity, approval, effect, receipt and recovery writer. Browser preview cannot impersonate a paired device; no provider APIs, duplicate registries, new cloud backend or APK dependency.
- Preserve working Today, Talk, Build, Control, World, Me, phone pairing, command planning, media and TierFlow module surfaces. Correct unsupported or unconnected states visibly instead of inventing data.
- Use this queue with the machine-readable companion `PWA-FIRST-BUILD-QUEUE.json`. The Proposal 5 foundations and VEC PWA documents remain the implementation references; prior branch/PR gate procedures are superseded **for this direct-main sequence only**.

## Incremental media and offline evidence (2026-11-13)

- Media implementation: direct authenticated stream requires optional owner-configured `playback.json` and `VIVIM_MEDIA_FILES_ROOT`. See `PWA-MEDIA-STREAMING.md`. No auto-discovered playable files or unbounded native cache.
- Read-only desktop World receives the HQ media metadata through the existing short-lived owner session. Playback still requires the paired phone.
- Offline: public shell only; Host projections, receipts and playbacks are revoked locally when the connection/device session ends. The service worker never intercepts `/api/` media streams or private requests.
- Node24 tests, actual Windows Host, Android Chrome codecs, pairing and network chaos: **UNRUN**; Actions is reporting startup failures with zero jobs. Do not promote as accepted.

## Current next step

PWA-12 is now the only remaining original queue item without source completion. It requires executable Node24, Windows and real Android/Tailscale proof. GitHub Actions is returning startup_failure with zero jobs, so that acceptance has **not** happened. Continue code fixes directly on main; never claim release GO from V8 syntax checks.

Run the original tests in one command from the real repo checkout: `node integration/scripts/pwa-run-now.mjs`. It prints real runtime version and per-suite exit codes. This is a local verifier, not another PR gate or service; Node/Windows and physical Android acceptance are still UNRUN until executed.

## Current source-check record

At `952834db330142ded0e08657c7eb22a4639cf491` (2026-11-13), source-level V8 execution of the checked-in test callbacks and the corresponding pure JS implementation returned **33/33 PWA UI + 9/9 VEC + 10/10 Build = 52/52 PASS**. This is **not** the Node test runner, real HTTP stack, Windows launch, or Android phone. It does not prove PWA release acceptance. Main also includes Direct media library paging (25 items per page), full-catalog matched-file resolution, stream close on World tab exit, private Control/history redaction and abort-safe opening.

GitHub Actions run for this work is still `startup_failure` with no jobs. The next operational step is to run `node integration/scripts/pwa-run-now.mjs` in the real Windows ARBOR checkout and exercise a paired Android browser over Tailscale Direct. Do not claim device acceptance from the source checks.

## Android PWA install assets (2026-11-13)

The manifest now includes SVG-any, PNG 192×192, PNG 512×512, and a full-bleed maskable PNG 512×512. All three raster files are committed to the canonical PWA, served by the existing Direct Host allowlist and listed in service-worker shell v6. A committed Node test checks the binary PNG signatures, IHDR dimensions, bit depth, color type, and manifest roles. **Actual install on Android Chrome and this Node binary test remain UNRUN**.

## Latest main-bound source checks

Source checked at `7b9987884cb8aca1e1d568bac7da42135fa06c71` on 2026-11-13: **34/34 UI**, **9/9 VEC**, **10/10 Build**, and **3/3 media-stream-grant source checks**, for **56/56 passing source-level checks**. These use V8 test callbacks and Node/browser API shims as needed, not the actual Node or Android runtime. New source includes 52-item SW v6 install shell with SVG/PNG/maskable icons; install action appears only when the browser offers it; media pager, remote read-only authorization and active-stream grant reevaluation remain Host-owned. **Node24, actual HTTP sessions, Windows Host, Tailscale phone, Android installation and media codecs are all UNRUN.** Keep PWA-12 open until actual results exist.

## Single pre-local comprehensive upgrade (93a6fc922b22)

The consolidated source upgrade is documented in [PWA-PRELOCAL-UPGRADE.md](PWA-PRELOCAL-UPGRADE.md), with machine-readable [PWA-PRELOCAL-UPGRADE.json](PWA-PRELOCAL-UPGRADE.json). It adds a local 15-item capability finder; a 10-check redacted device-readiness view; explicit, exportable offline Quick Capture with no silent data eviction; session-aware permissions; a repaired Build revision-guard static import; and a Node import-closure verifier. Service-worker shell v8 now lists 58 assets. Source-level V8 checks are **70/70 pass**; real Node/Windows/Android remain **UNRUN** and PWA-12 stays open.
