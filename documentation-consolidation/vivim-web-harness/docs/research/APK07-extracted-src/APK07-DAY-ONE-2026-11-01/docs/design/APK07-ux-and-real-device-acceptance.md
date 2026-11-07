# APK07 user journeys, screens, end-to-end and negative acceptance

**Scope:** first releasable customized APK; full package/Host/World/native/offline acceptance. Extend, never overwrite, `docs/superpowers/specs/2026-10-31-gui07-android-acceptance-spec.md` (the existing narrower GenUI test suite).

## 1. Screen map

```text
PC: VIVIM setup
   ├── Tailscale/Host readiness & trusted identity
   ├── My Android App (standard / personalize / independent signer)
   │   ├── App preview (label, icon, theme, package, permissions, size)
   │   ├── Signing ownership & key backup (explicit consequence)
   │   ├── Build receipt (version, fingerprint, SHA, status)
   │   └── QR install landing + connection diagnosis
   ├── Pair approval (device name, match code, key fingerprint, scopes)
   └── Devices, releases, backups, updates, revoke

Android: native VIVIM
   ├── Local shell / locked / disconnected / recover state
   ├── Device pairing challenge and owner confirmation
   ├── Talk (provider readiness + drafts + history + receipts)
   ├── Build (shared GenUI canvas/list + preview/CAS/Undo)
   ├── World (knowledge, source confidence, edit/search/export)
   ├── Control (natural-language planner, scoped approval, outcomes)
   ├── Native Inbox (notification/share intake, explicit send)
   └── Settings (app identity/version/signer, Host/device scope, offline/storage)
```

## 2. Main user stories and interaction states

**Install:** As a non-developer, I want the PC to compile my branded app and give me one clear QR to start installation. Errors are visible and actionable. The browser landing shows package identity, signer fingerprint and download; Android OS may require 'install unknown apps' or advanced verification path. VIVIM cannot bypass the OS.

**Pair:** As an owner, I want to know which phone is requesting access, inspect its proposed scopes and approve once. The phone and PC display matching short code. A QR screenshot alone cannot grant authority.

**Operate:** As a user, I want to describe settings and actions in natural language from phone/PC, preview expected effect/risk and see a verified result. Exact known controls should not require an AI model. Unavailable effect shows unavailable, not green.

**Create:** As a user, I want a nearly blank but familiar Build surface with my safe theme, first grounded cards then optional views, and ability to select/edit/undo. Talk draft remains intact during all Build interactions.

**Offline:** As a user, I want to open VIVIM while my PC is off, read my authorized cached World, make drafts and export. The UI must say that AI/browser actions can't run, never pretend it sent anything.

**Update:** As a custom app owner, I want my new branded APK update to keep data and signing lineage. If signer/package mismatch, I want a specific recovery/export/import flow rather than an overwrite failure.

## 3. State language and visual signaling

Use one stable vocabulary across mobile, Host and receipts:

| Status | Meaning | Allowed UI affordance |
|---|---|---|
| `offline-local` | No verified Host, usable local UI/data | Read/edit drafts/export only |
| `connecting` | Network check in progress | Cancel/retry; no send |
| `unpaired` | No approved device key/grant | Enroll, not provider commands |
| `paired-read-only` | Authenticated but current grant permits reads only | View, propose |
| `ready` | Host, device scope and exact executor health verified | Supported actions |
| `queued` | Host accepted durable request | Show ID, cancel where supported |
| `claimed` | Exclusive owner started effect | Show progress, do not resend |
| `uncertain` | Effect may have happened; must reconcile | Explicit investigate, never retry automatically |
| `verified` | Effect owner observed completed outcome | Receipt with correlation |
| `stale` | Last known data no longer current | Show freshness and limits |
| `revoked` | Current device no longer authorized | Lock Host operations, recovery |

No optimistic `signedIn:true` from hardcoded fallback. Do not equate Tailscale joined, Host HTTP reachable, relay queued, webapp tab loaded, provider sent, Host committed and Android repainted.

## 4. Mandatory physical-device acceptance matrix

Use real installed APK, not PWA browser tab and not `adb install` alone. Each test logs `APK07-Txx` ID, timestamp, source SHA, Android device/OS and WebView version, appId/versionCode/cert SHA (redacted where necessary), Host/Tailscale build, request IDs, expected vs actual, screenshots/video with private content masked.

| Test ID | Scenario | Required observation |
|---|---|---|
| T01 | New independent custom APK build | Valid final package appId/brand, release cert, monotonic version, SHA manifest; owner-safe key backup |
| T02 | PC displays QR → Android browser downloads APK | Correct HTTPS Host and artifact details; OS installer invoked via user tap |
| T03 | Unregistered/new/older Android installer case | Honest unknown-app and current verification flow, not bypassed |
| T04 | First APK launch before network | Local packaged UI appears; no blank WebView dependency |
| T05 | Enroll with matching code and owner approval | Device public key bound and Host grants only requested scopes |
| T06 | Other phone same Tailscale identity | Cannot mutate via enrolled device's grant |
| T07 | Physical phone Talk | Exact provider session ready, one send, exact thread and response visibly painted |
| T08 | Unsent draft + Build navigation | Draft checksum unchanged |
| T09 | Existing GUI07 full sequence | BF00, F02/F01, D03–D07, selected edit, CAS/Undo and reconnect receipts |
| T10 | Natural-language deterministic theme change | Text and GUI use same action ID, approval, receipt and effect |
| T11 | Ambiguous/AI-derived action | Candidate only, correct clarification/review; no permission escalation |
| T12 | Save or correct personal World | Provenance/owner vs provider evidence preserved; Host CAS effect seen on phone |
| T13 | D06 private data | No default phone projection, export, log or external provider disclosure |
| T14 | Android Share text/file | User draft opens; no auto-upload/send; unsupported file case explained |
| T15 | Notification approved and denied | Correct Android 13+ permission handling; fallback inbox works |
| T16 | Host shutdown / Tailscale disconnect | Local World, drafts, export work with stale status; no queued automatic external action |
| T17 | Offline conflicting edit then reconnect | Conflict review, no canonical overwrite or data loss |
| T18 | Drop network mid-provider send | Uncertain state retained; no duplicate send; manual reconciliation |
| T19 | Revoke phone, attempt replay | Device denied, sessions invalid, previously authorized command not replayed |
| T20 | v1→v2 same signer/appID update | Native data/drafts/schema retained, package installed and verified |
| T21 | Wrong signing key/version/appId update | Explicit refusal or parallel-app migration, no silent overwrite |
| T22 | Export→clean import (phone and PC) | Checksum/schema integrity, selected data restored, no leaked credentials |
| T23 | Android lifecycle stress | Back, rotate, IME, process death, renderer crash, lock/unlock preserve drafts |
| T24 | Screen-reader/accessibility | Tap alternatives for gestures, labeled approvals and safe focus |
| T25 | Restore a lost device / new phone | New device enrollment; old revoked; data recovery limits honest |
| T26 | Windows cold start and Linux build | No duplicate workers/Chrome owner; platform build receipts and Direct rejoin |
| T27 | Package/archive inspection | No owner token, signing password, sensitive memory or hardcoded device grant |
| T28 | False readiness | No provider/model advertised without live executable readiness evidence |
| T29 | Slow/broken network/captive portal | Signed APK download/retry safe; Host action not duplicated |
| T30 | CPU, memory, storage, battery | Measured install size, cold launch, cache limits, background power with named device |

**Minimum release device matrix**: reference owner phone (document model/version), one older Android 8/9 installation path if available, Android 13+ notification behavior, current Android security/verification behavior, at least one different OEM/browser if accessible. Where physical matrix unavailable, do not invent results; mark `awaiting_device`. Emulator complements but does not replace physical-phone acceptance.

## 5. Negative acceptance fixtures

- Tampered APK/signing cert, expired build/install route, unexpected package ID, version downgrade, untrusted update source.
- Different tailnet user, same-tailnet unauthorized device, forged `tailscale-user-login`, unexpected Host name, stale/reused enrollment code, intercepted deep link.
- Old device grant and session after revocation; expired app lock; mobile modified `approved=true`, `risk=R0` or guessed `deviceId`.
- Fake provider 'signed-in' record, stale model cache, unsupported model/effort/attachment target; UI doesn't say ready.
- Malformed/fenced/over-depth GenUI, duplicate IDs, stale baseRevision, D06 data, wrong-object view mutation, XSS in shared data.
- Preexisting Talk draft, share text exceeding bounds, malicious file picker URI; no unsanctioned send/permission.
- App updates during pending send, network packet loss after Host accepts action, device process killed before response; no uncertain resend.
- Host unavailable when user chooses disabling network; R3 action denied without local recovery.
- Offline merge of rejected/forgotten memory from phone to PC; deletion tombstone prevails.

## 6. Performance budget and evidence

Targets inherited from GUI07: BF00 provisional paint <500ms; local tap feedback <100ms p95; Host UI commit-to-phone visible <500ms p95 (stable Direct). New proposed cold offline launch target <=1.5s p95. Measure end-to-end Tailscale paths separately for LAN, remote direct, DERP-relayed and unavailable; do not claim a route metric from a synthetic localhost test. Record p50/p95, device/storage/network and sample sizes.

## 7. Exit declaration

`APK07_accepted` requires completed all P0 T01–T30 (or documented bounded device-matrix exceptions explicitly approved by owner, not agent-waived), security negative receipts and no open high-severity critical path gaps. Test IDs map to `docs/roadmap/APK07-task-ledger.v1.json`, source SHA and canonical roadmap rows. Source-implemented + unit-tested is not Android-accepted.
