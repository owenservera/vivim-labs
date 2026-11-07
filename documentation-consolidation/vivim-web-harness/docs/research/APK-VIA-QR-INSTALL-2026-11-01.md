# Research: Custom user-built APK install via QR code after Tailscale setup

**Date:** 2026-11-01
**Objective:** Research how to add a customised user-built APK the user installs on their Android phone during install/setup. Concept: once the local PC is onboarded and set up and Tailscale is installed on both PC and phone, the user is given a QR code to install the APK.

**Status:** Research only — no implementation decisions made yet. Evidence-based; every claim references an inspected file or an external source.

---

## 1. What already exists in the repo

### 1.1 Android APK build (Gradle-free, already verified to build)

| File | Purpose |
|---|---|
| `studio/android/AndroidManifest.xml` | Package `inc.vivim.app`, minSdk 26, targetSdk 34, `INTERNET` + `ACCESS_NETWORK_STATE` permissions, `usesCleartextTraffic="true"` (for LAN `http://` pairing), `allowBackup="false"`, exported `MainActivity` launcher activity with `vivim://pair` app-link and `ACTION_SEND` text/plain filter. |
| `studio/android/src/inc/vivim/app/MainActivity.java` | Full-screen WebView shell around the Studio mobile PWA (`/mobile`). First-run pairing screen; pairing URL persisted in SharedPreferences; `vivim://pair?pairUrl=` app-link intent; Share-sheet support. No Gradle, no AndroidX, no bundled web assets. |
| `studio/android/build.ps1` | Windows build: `aapt2 compile → aapt2 link → javac --release 11 → d8 → zip (add classes.dex) → zipalign → apksigner sign → apksigner verify --print-certs`. Output `studio/android/build/vivim.apk`. Debug keystore `studio/android/.keystore` (gitignored), alias `vivim`, storepass/keypass `android`, RSA 2048, validity 10000 days, `CN=VIVIM`. `--min-sdk-version 26`. Requires JDK 17+ and Android SDK `platforms;android-34` + `build-tools;34.0.0`. |
| `studio/android/build.sh` | Linux/macOS mirror of `build.ps1`. |
| `studio/android/README.md` | Build without emulator; verification receipt 2026-10-29: second build invocation passed all seven stages incl. `apksigner verify --print-certs`; `vivim.apk` 20.6 KB, debug-signed, cert SHA-256 `7c9172356e2a288f354c3094bf929f2bd7f6bb36fe72ec84de58f3dc83ea0b32`. **No device install, no emulator, no phone UI observed.** |
| `studio/docs/architecture/ANDROID-APP.md` | Architectural overview; `adb install -r android/build/vivim.apk` is the documented install path (USB debugging), not QR/download. |

Verification evidence: `studio/android/README.md` lines 66-83 record a live, signature-verified build (20.6 KB APK). `git diff --check -- android` passed. This is a **build-only** receipt — no install/device evidence.

### 1.2 Tailscale direct PC-to-phone transport (source-implemented, live PC-originated receipts)

| File | Purpose |
|---|---|
| `apps/vivim-host/direct-phone.mjs` | Direct phone traffic accepted only behind a trusted local Tailscale Serve proxy. Requires `tailscale-user-login` header matching configured user, loopback remote address, `x-forwarded-proto: https`, exact Host match. Serves `/mobile/` assets from `apps/mobile-relay/` (index.html, app.js, transport.mjs, generative-ui, manifest, icon). Routes: `api/relay`, `api/control/catalog`, `api/control/plan`, `connection.json`. CSP: `connect-src 'self'; img-src 'self' data:;`. No APK download route exists yet. |
| `apps/vivim-host/server.mjs` | Host HTTP server, loopback-only by default (`127.0.0.1`). Delegates `/mobile/` to direct-phone handler. Static asset MIME map (`public/`) does not include `.apk`. No file-download route. |
| `apps/mobile-relay/transport.mjs` | Phone-side transport: `direct` mode targets `https://<tailscale-host>/mobile/`; `cloud` mode targets Vercel relay. `directOrigin()` accepts `https:` origins ending `.ts.net` or loopback. |
| `docs/ARCHITECTURE-CURRENT-AND-TARGET.md` | Phone ↔ Direct: "Tailscale Serve: authenticated Host /mobile direct transport". Real PC-originated Tailslate commands succeeded (`cmd-fa54e0eb5fe1605c` "VIVIM direct chat works"); Android repaint not observed. |
| `docs/agents/HANDOFF-2026-10-31-DIRECT-PHONE.md` | Direct transport: Tailslate Serve `https://desktop-9kb03lu.taild7271a.ts.net/mobile/` proxies to Host `127.0.0.1:43187/mobile`, owner identity `owservera@gmail.com`. |

### 1.3 QR code pairing (in-tree, no dependency)

| File | Purpose |
|---|---|
| `studio/src/studio/lan-pair.mjs` | In-tree QR encoder (byte mode, ECC level L, versions 1-10, max 271 bytes, ISO/IEC 18004). `qrEncode(text)` → boolean matrix; `qrToTerminal(text)` → terminal rendering with half-block glyphs, forced black-on-white, 4-module quiet zone. Pairing URL: `http://<PC-LAN-IP>:<port>/mobile?token=<token>`. Token stored in `pairing.json` (0600, owner-only), persists across restarts. |
| `studio/bin/commands/capture.mjs` | `pair show` command prints the pairing link + terminal QR code (`qrToTerminal`). `start --lan-pair` / `serve --lan-pair` print link + QR. |

### 1.4 Roadmap context

- `P01` "Mobile bootstrap bridge + Linux APK build" — in_progress (grokbot, PR #22, branch `mobile-bootstrap-bridge`).
- `Q11` "Package the host package earlier" — done (Tauri v2 MSI, `wh serve --mobile`).
- `DIR01` "Direct Tailscale PC-to-phone mobile transport and cloud contract parity" — implementation_pending_live_validation.
- `PACK01` "Unify Windows Tauri desktop/Android PWA package plan with installed CLI and Host" — planned, `human_review_needed`.
- `V1` milestone (Visual Personal Bootstrap Test) requires real paired Android but does not include APK-via-QR install.

---

## 2. The target concept (end-to-end flow)

1. **PC onboarded and set up** — VIVIM Host + Studio running locally on the PC (Windows roadmap, authorized 2026-10-31).
2. **Tailscale installed on PC and phone** — both devices joined to the owner's tailnet; PC exposes `https://<pc>.tail<...>.ts.net/mobile/` via Tailslate Serve (direct-phone.mjs, authenticated by `tailscale-user-login`).
3. **User builds (or is given) a custom APK** — the Android WebView shell from `studio/android/`, possibly customized (app name, icon, pairing URL default, theme), signed with the user's own keystore.
4. **PC serves the APK over HTTPS** via the Tailslate direct path.
5. **PC shows a QR code** encoding the APK download URL (e.g. `https://<pc>.tail<...>.ts.net/mobile/apk/download`).
6. **Phone scans the QR code** (native camera, Android 9+) → browser opens the HTTPS URL → browser downloads the APK.
7. **User taps the downloaded APK** → Android Package Installer → on Android 8+ (API 26+, which matches our minSdk 26) the user must first grant "Install unknown apps" permission to the browser → then tap Install.
8. **APK launches** → pairing screen → user scans/pastes the PC's LAN/Tailslate pairing URL → paired.

Key insight: the QR code encodes a **URL**, not the APK itself. The APK (tens of KB) is downloaded separately over the Tailslate HTTPS connection. A Tailslate HTTPS URL (~70 bytes) fits easily in a QR code version 1-2 (the in-tree encoder supports up to 271 bytes).

---

## 3. Technical research findings

### 3.1 Android APK installation from a browser download (QR → URL → install)

**Source:** Android Developers Blog "Making it safer to get apps on Android O" (2017), Stack Overflow, Samsung support, Uptodown help, AOSP PackageInstaller source.

| Step | Detail |
|---|---|
| QR scan | Android native camera (Android 9+) decodes URL QR and offers to open in browser. No dedicated scanner app required. |
| Browser download | Browser GETs the HTTPS URL. Server must respond with `Content-Type: application/vnd.android.package-archive`. If served as `application/octet-stream`, the browser downloads a generic file and the file manager may not offer to install it (observed failure on Android Enthusiasts Stack Exchange). |
| "Install unknown apps" permission (Android 8+/API 26+) | Per-app permission. First install attempt from a browser shows "Installation blocked... not allowed to install unknown apps from this source". User taps Settings → Apps → Special access → Install unknown apps → selects the browser → toggles "Allow from this source". One-time per browser. |
| Package Installer | User taps the downloaded APK (Downloads folder or download notification). PackageInstaller parses the APK, shows app name + requested permissions, user taps Install. **Always requires an explicit user tap** — silent/automatic install is impossible without root/system privileges or a device owner (MDM). |
| APK signing | APK must be signed (debug or release keystore). Unsigned APKs are rejected by the installer. |
| Signature schemes | v1 (JAR signing, legacy), v2 (Android 7.0+/API 24+), v3 (Android 9+/API 28+). For max compatibility sign with v1+v2+v3. `apksigner` decides schemes by default from `--min-sdk-version`/`--max-sdk-version`. Android 11+ (API 30+) requires v2+ for apps targeting API 30 (our targetSdk is 34). |
| MIME type | `application/vnd.android.package-archive` is the registered MIME type for `.apk`. |
| `REQUEST_INSTALL_PACKAGES` | Only needed if the APK itself programmatically installs other APKs. **Not needed** for the browser-download flow; the browser triggers Package Installer via a normal `Intent.ACTION_VIEW` with type `application/vnd.android.package-archive`. |

### 3.2 APK signing (apksigner)

**Source:** Android Developers `apksigner` reference, AOSP APK signing schemes v1/v2/v3 documentation.

- Current build (`build.ps1` line 114): `apksigner sign --ks $keystore --ks-key-alias vivim --ks-pass pass:android --key-pass pass:android --min-sdk-version 26 --out $finalApk $aligned`. This signs with schemes auto-selected by minSdk 26 (v1 + v2). Verified with `apksigner verify --print-certs` (line 118).
- Debug keystore (auto-generated on first build) is **not appropriate for distribution** — debug certificates are insecure and the keystore is gitignored/local-only. A user distributing a custom APK should generate their own release keystore: `keytool -genkeypair -keystore release.keystore -alias vivim -keyalg RSA -keysize 2048 -validity 10000`.
- `apksigner verify --print-certs <apk>` prints the signer certificates (SHA-256 fingerprint) so the user can confirm which key signed the APK.
- Order matters: `zipalign` must run **before** signing. Re-aligning after signing invalidates v2+ signatures. (The current build already zipaligns before signing — line 102 before line 114.)
- `apksigner` supports `--v1-signing-enabled`, `--v2-signing-enabled`, `--v3-signing-enabled` flags for explicit scheme control.

### 3.3 Serving an APK over HTTPS via Tailslate (direct-phone path)

**Source:** inspected `direct-phone.mjs`, `server.mjs`, `lan-pair.mjs`.

- `direct-phone.mjs` is the HTTP handler mounted at `/mobile/` behind Tailslate Serve (server.mjs line 166). It is gated by `tailscale-user-login` identity + loopback remote + `x-forwarded-proto: https` (line 69). This is the correct place to add an APK download route because it is already reachable by the phone over Tailslate HTTPS and already authenticated.
- Adding a route such as `GET /mobile/apk/download` would: read `studio/android/build/vivim.apk` (or a user-configured APK path), set `Content-Type: application/vnd.android.package-archive`, `Content-Disposition: attachment; filename="vivim.apk"`, `Content-Length`, and stream the file. The phone browser downloads it.
- The Host server (`server.mjs`) is loopback-only and its static MIME map has no `.apk` entry — it is **not** reachable by the phone, so it cannot be the APK download endpoint. The direct-phone handler (mounted under `/mobile/`) is the phone-reachable endpoint.
- Security: the download URL is protected by Tailslate identity (`tailscale-user-login`). Only the owner's phone on the tailnet with a valid Tailslate session can reach it. The QR code URL is guessable but the underlying fetch requires Tailslate authentication.

### 3.4 QR code encoding the APK download URL

**Source:** inspected `lan-pair.mjs` QR encoder.

- `qrEncode(text)` supports byte mode, ECC L, versions 1-10, max 271 bytes UTF-8. A Tailslate HTTPS download URL (~60-90 bytes) fits in QR version 1-2.
- `qrToTerminal(text)` renders black-on-white with a 4-module quiet zone — suitable for terminal display. For on-screen display (browser), a QR image (SVG/PNG) would need to be generated or the terminal QR photographed. The existing `pair show` only prints to terminal.
- The QR encoder is a pure function with no dependencies — it can be reused or copied into a new "show APK install QR" command/endpoint.

---

## 4. Implementation options (research, not decisions)

### Option A: Serve the pre-built APK from direct-phone + terminal QR
- Add `GET /mobile/apk/download` to `direct-phone.mjs` streaming `studio/android/build/vivim.apk` with `Content-Type: application/vnd.android.package-archive`.
- Add a CLI command (e.g. `pair apk-qr`) that prints the download URL + terminal QR via `qrToTerminal`.
- User builds the APK themselves (`pwsh -File android/build.ps1`), signs with their own keystore, places it at the expected path.
- Phone scans terminal QR (camera) → downloads → installs.

### Option B: On-screen QR image + APK download page
- Serve a small HTML page at `/mobile/apk/` with a "Download VIVIM APK" button linking to `/mobile/apk/download`, plus a rendered QR image of the download URL (generated server-side or via a JS QR library).
- Phone navigates to the page (after scanning a pairing QR or entering the URL), sees the QR + download button.

### Option C: Full custom APK build pipeline
- Allow the user to customize `AndroidManifest.xml` (package name, label, icon) and `MainActivity.java` (default pairing URL, theme), then run `build.ps1` with their keystore.
- This requires the Android SDK + JDK on the user's PC — a setup step. The current build already documents this requirement.

### Recommended starting point (Option A)
It reuses the most existing code (direct-phone.mjs, lan-pair.mjs QR encoder, build.ps1) and adds the smallest surface: one download route + one QR-display command. Custom APK signing (Option C) can be layered on later.

---

## 5. Security considerations

1. **Tailslate identity gate** — direct-phone.mjs already requires `tailscale-user-login` matching the owner. The APK download inherits this protection; only the owner's tailnet devices can fetch it.
2. **QR code contains a URL, not a secret** — anyone who photographs the QR gets the download URL, but still cannot download without Tailslate authentication. Treat the QR like the pairing link (which already carries a token): "Anyone with this link can use your VIVIM while it runs; do not share it" (capture.mjs line 176).
3. **Debug vs release signing** — the auto-generated debug keystore must not be used for a distributed APK. The user should generate their own keystore and sign; `apksigner verify --print-certs` confirms the signer.
4. **"Install unknown apps" permission** — on Android 8+ the user must manually grant this to their browser once. This is an OS-mandated interactive step; it cannot be bypassed. The UX should tell the user what to expect.
5. **Cleartext traffic** — the APK is served over Tailslate HTTPS, not cleartext LAN HTTP. The WebView shell's `usesCleartextTraffic="true"` is only for the LAN pairing page, not the APK download.
6. **APK integrity** — the user should verify `apksigner verify` output before distributing. A corrupted or re-signed APK will fail installation.

---

## 6. Open questions / decisions needed (not inferable from code)

1. **Who builds and signs the APK?** The user (their own keystore, their PC with Android SDK) or a provided pre-built debug APK? Debug-signed APKs install fine but are not appropriate for distribution; a release keystore requires user action.
2. **Where does the APK live on the PC?** `studio/android/build/vivim.apk` (build output, gitignored) or a user-chosen path? direct-phone.mjs needs a configured path.
3. **How is the QR displayed?** Terminal only (`qrToTerminal`, like `pair show`) or also as an on-screen image in the mobile PWA / Host UI?
4. **Does the custom APK replace or supplement the existing PWA pairing flow?** The APK is a WebView shell that still needs to pair to the PC (`/mobile?token=...`). The QR-installs-the-APK step is separate from the QR-pairs-the-phone step.
5. **Android version support** — minSdk 26 (Android 8.0). "Install unknown apps" per-app permission applies. Devices below Android 8 use the global "Unknown sources" setting instead.

---

## 7. Evidence index (files inspected)

- `studio/android/build.ps1` — full 7-stage build, signing, verification
- `studio/android/build.sh` — Linux/macOS mirror
- `studio/android/AndroidManifest.xml` — manifest, minSdk 26, targetSdk 34
- `studio/android/src/inc/vivim/app/MainActivity.java` — WebView shell, pairing, app-link
- `studio/android/README.md` — build verification receipt (2026-10-29, 20.6 KB APK, cert fingerprint)
- `studio/docs/architecture/ANDROID-APP.md` — architecture + `adb install` path
- `apps/vivim-host/direct-phone.mjs` — direct phone handler, auth gates, asset map (no APK route)
- `apps/vivim-host/server.mjs` — Host server, loopback-only, static MIME map (no .apk)
- `apps/mobile-relay/transport.mjs` — phone transport, direct origin validation
- `studio/src/studio/lan-pair.mjs` — QR encoder (versions 1-10, max 271 bytes), pairing URL, token storage
- `studio/bin/commands/capture.mjs` — `pair show` prints link + terminal QR
- `docs/ARCHITECTURE-CURRENT-AND-TARGET.md`, `docs/agents/HANDOFF-2026-10-31-DIRECT-PHONE.md` — Tailslate direct transport receipts

**External sources:** Android Developers Blog (Android O install-unknown-apps), Android Developers `apksigner` reference, AOSP APK signing schemes v1/v2/v3, Stack Overflow (APK MIME type, install flow), AOSP PackageInstaller source.
