# FA-PWA-001 — PWA-First Mobile, APK Deferred

**Owner decision:** **Ship and develop the Host-served HTTPS PWA first.** Native Android APK, bridge maintenance, signing, distribution and Play Store are deferred **beyond** the first VIVIM ARBOR beta. A source folder still named `apk/` is not automatically an Android delivery dependency: Host currently uses some device enrollment modules from that directory; do not delete them by name.

## Source-observed facts from frozen ARBOR branches

| Concern | Actual source | Exact Git blob identity | Finding |
|---|---|---|---|
| PWA manifest | E `apps/mobile-relay/manifest.webmanifest` | `77056942424d144a40885321e48b380a617906cd` | `start_url=/mobile`, `display=standalone`, SVG icon |
| Service worker | E `apps/mobile-relay/sw.js` | `0fb3fb52d57eb81d82bc10cc48e1f2e58ffd0b48` | Read-only app shell, NO auth/API/write caching or replay |
| Device key | E `apps/mobile-relay/browser-device-key.mjs` | `2c7d16ee93a18ad41e0681295d9e94522273df84` | Browser WebCrypto nonextractable P-256 key in IndexedDB; session/grant remains Host-owned |
| Device enrollment | E `apps/mobile-relay/device-enrollment.mjs` | `881b30c03b042aa7a69f070990e06d68d877065b` | Browser or legacy native bridge; pending Host owner grant proof, fail-closed lost-key behavior |
| PWA control | E `apps/mobile-relay/control-client.mjs` | `59ee97e51f6fddbd99fe3ef9adfd03ed5170d02d` | One Host source/receipt, no cloud fallback, no uncertain-effect auto-retry |
| Host serving | B `apps/vivim-host/direct-phone.mjs` | `6bb604cf7a64e4623c28bc86495f39a1f40f1626` | Exact HTTPS origin and server static allowlist; B-owned browser signer allowlist patch still needed |
| Host device grants | B `apps/vivim-host/apk/enrollment.mjs` | `cf8e44b12e9a81e0dfc348246dcc3d46c3649519` | Shared grant store and revocation logic; folder name `apk` is historical |
| Native package | A `src-tauri/tauri.conf.json` | `f9474531f0194268c6688ac56c1b48d7ba3fff3a` | Explicit `resources/vivim.apk` packaging resource currently required in original source |
| Native IPC | A `src-tauri/capabilities/default.json` | `3883b784c0e211a32dcac645c4f0664025c14680` | Original remote URL scope `http://127.0.0.1:*` is too broad, P0 security defect |

**Source status:** E's browser signer and service worker patches are isolated worker source; B Host static + request-bound checks and G packaging are **not yet integrated or device accepted**.

## Recommended PWA product topology

```text
WINDOWS HOST (authoritative)
  Native Tauri shell -- verified private child attestation --> Host process
    ├── /mobile HTTPS via existing authenticated Serve proxy
    ├── /api ... scoped read/plan/approval/receipt
    ├── existing Studio/CDP exact-thread browser owner
    ├── HQ/semantic/AutoWiki internal permitted reads
    ├── media byte-range read endpoints (if accepted)
    └── native Rust pure engine/graph/ingress through trusted boundary
                                   │
                 authenticated origin + challenge/grant/session
                                   │
PHONE HTTPS PWA (not APK)
  /mobile (standalone manifest)
    ├── text composer / Tab / agent feed / source-driven UI / AutoWiki
    ├── nonextractable browser key and owner-approved device grant
    ├── read-only offline app shell; local draft retention only if owner accepted
    ├── UI/semantic hints clearly marked offline or provisional
    └── actual effect/receipt through live Host ONLY
```

**No second phone backend, cloud relay fallback, provider API or phone-side effect writer.** Preserve the current direct PWA Host route and one device-grant semantics. PWA UX improvements go through E/TierFlow and B Host.

## Exact work needed before beta

1. **G/A packaging:** decide removal of stale mandatory `resources/vivim.apk` reference from the Windows Tauri release configuration, retaining necessary runtime manifest/resources; exclude generated APK without adding fake binary. Review other APK build scripts before deciding what to retire.
2. **B + E static:** include `browser-device-key.mjs` in Host's explicit `direct-phone.mjs` static map, preserving exact file type and origin allowlist. E's prepared patch is evidence, not applied B source.
3. **B + F authentication:** enumerate all privileged PWA Host endpoints, real grant/session request binding, revocation, nonce freshness, method/path/body binding if required and browser-origin same-origin threat; no bearer-only replay loophole.
4. **A + B desktop security:** secure native child→Host process identity and bound owner token/one-time UI code; no foreign listener on fixed port can accept durable owner bearer; tighten Tauri remote IPC URL scopes.
5. **E + F PWA core:** verify install/launch, landscape/portrait, thumb access, persisted key/lost-key refusal, pairing QR, revocation, reconnect, screen reader, IME composition, keyboard Tab, accessibility and reduced animations on actual Android browser.
6. **D/B/E intelligent surface:** source-linked AutoWiki and Quiet/Assisted/VIEW ALL may be projected in PWA without a secondary parser/executor. Show `source only`, `unavailable`, `stale` and `permission required` accurately.
7. **E + F offline:** service worker must never intercept authenticated API or mutations. Local drafts are user-owned, encrypted/protected if persisted and never requeued as effects automatically. Offline shell cannot claim current source verification.
8. **G + F release:** Windows installer+native Host+PWA must run without sibling repositories and without any native APK. A real phone device acceptance report is required; a browser desktop emulator is insufficient.

## Advanced mobile work intentionally deferred

- Full APK build, signed package, Android Keystore bridge, native notification service and Play Store release.
- Android-specific background services, native local model/WASM engine with independent effects, push notification permission system.
- Mobile app architecture migration that forks from existing direct PWA, duplicates World/Host/Studio source ownership or introduces a cloud intermediary.
- Unbounded offline corpus, unencrypted private transcripts or replaying queued actions while Host cannot verify world/approval.

**Allowed future PWA increments:** self-contained shell update, thin offline drafts, selected read-only cached help source with explicit expiry/policy, controlled media playback/cache, keyboard and screen-reader parity, local-pure semantic hints from the approved compiler pack (with `offline_unverified`). Each is gated by actual PWA device/security tests and per-volume/storage resource budgets.

## Device and security acceptance matrix

| Test | Expected acceptance | Failure signal |
|---|---|---|
| Fresh Android Chrome PWA install from trusted Host HTTPS origin | Manifest, icons, standalone shell load; valid server trust | Wrong origin, mixed content or missing manifest |
| No Android native bridge present | Browser WebCrypto signer works without APK/JS native injection | Any forced `VivimDeviceKey` prerequisite |
| Lost browser IndexedDB private key | Existing approved grant cannot silently regenerate identity; user must re-pair | Implicit identity rotation or reused stale grant |
| Revoke grant while phone active | Privileged Host operation refused; private views invalidated | Action allowed after revoke |
| Stolen grant ID/session without key proof | Reject unauthorized privileged command | Replayed effect from bearer identifiers |
| Offline with typed text | Local draft remains responsive, no submit/approval send, shows offline knowledge limitations | Hidden queued mutation or fake receipt |
| Source wiki changes while draft active | Stale source packs no longer authorize/validate actions until refreshed | Old command silently changes effect |
| Uncertain provider send during network loss | Phone displays `uncertain`, prompts owner to reconcile on PC | Blind auto resend on reconnect |
| No packaged `vivim.apk` | Windows beta launcher/install still works, phone PWA separately works | Installer depends on missing generated APK |
| Real keyboard/IME/accessibility | Original text, caret/Unicode spans and accessible status messages preserved | Autocomplete submits mid-composition |

**Release gate:** PWA is first-class product mobile path. A future APK can wrap/use the same canonical Host contract only after independent project decision; it cannot define another authority model.
