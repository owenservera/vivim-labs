# First-run UX integration — code and acceptance ledger

Status: implementation integrated in Git tree, real Windows MSI + Android acceptance pending.
Date: 2026-11-01. Base main: c71b308. Claude reference: PR #41 (b089283).

## The user journey
1. Install VIVIM Host and open its local onboarding route. The existing signed APK registry and Windows installer remain authoritative.
2. Choose a local workspace and start the existing dedicated Studio browser. The Host never creates a second browser controller.
3. Sign in personally to existing AI webapps. Per-provider checks must show uncertainty rather than claim account authentication on a mere page load.
4. Select the primary AI. For now only ChatGPT has a connected F02/F01 personalization adapter. Review both exact prompt texts and approve an expiring, generation-scoped grant. Reconciliation never auto-resends an uncertain send.
5. On the Phone step, offer the APK download from the registered Host release over private Direct HTTPS. The installer process and Android OS retain responsibility for approving installation.
6. Once installed, show a separate short-lived QR pairing code. The first camera scan enters the browser, which offers **Open VIVIM app** before redemption. The installed APK opens a confirmed deep link, redeems the token via POST /mobile/api/pair, and stores only the stable Host origin.
7. Host reports the persistent paired device ID; the mobile shell shows Welcome, personal bootstrap results and Build drafts. User activates each proposal before it becomes a Host revision; undo remains available.

## Important contracts and known limits
- The existing APK registry and single-use pairing store on main were not replaced.
- Tokens remain URL fragments rather than request paths or query parameters; the UI never logs them. The device ID from a token is **not** cryptographic Keystore proof of possession.
- A Tailscale HTTPS Direct origin is required for an actual phone pairing URL. Do not use localhost, private loopback or a public Funnel as a default.
- The earlier MSI exists per the local agent, but it predates this integration. The new Tauri staging script now packages Node, Host, Studio and the signed APK release manifest, and the Rust launcher resolves installed resource paths. **A new MSI must be rebuilt** and its launch from Program Files must still be accepted on Windows; this research environment cannot rebuild or run it.
- The Android release script now emits the actual manifest JSON rather than its filename. Rebuild the release APK and regenerate its signed manifest before building the new MSI.
- First-run packaging command is `node tools/packaging/prepare-host-runtime.mjs` (normally invoked through Tauri beforeBuildCommand). The build fails closed for absent or mismatched APK signatures, digests or manifest.
- Real login verification, non-ChatGPT bootstrap, deep D03–D07 scheduling, Android signer attestation, device-scoped access grants, revocation, offline-native sync and signed updates require separate acceptance.
- Tests in apps/vivim-host/test/onboarding.test.mjs use a fake Studio. Run the real F02/F01 provider journey on the dedicated existing owner before calling it accepted.
- Rollback: existing Host APK, control and pairing contracts remain in place; no destructive account/profile migration was added.

## Acceptance checklist (do not mark green on synthetic evidence)
- [ ] Fresh install of MSI on Windows without access to the source checkout; Host starts and owns its runtime.
- [ ] Setup wizard completes with real workspace and persistent dedicated browser profile.
- [ ] Real signed-in AI readiness and a complete F02/F01 run with evidence and no duplicate sends.
- [ ] Hosted signed APK downloads and installs on physical Android with correct package and signer.
- [ ] Camera opens browser handoff; native APK redeems unique token; PC shows the same device ID.
- [ ] Already-used/expired/revoked token is rejected; no other device is silently paired.
- [ ] Phone Welcome, Talk, natural-language Build, proposal preview, Activate and Undo operate against the Host.
- [ ] Restart Host and Android; same trusted device reconnects, with no automatic provider resend.
- [ ] Accessibility, small display, offline, error and recovery screens checked on device.

## Code entrypoints
- Host onboarding service: apps/vivim-host/onboarding/service.mjs
- Host routes and APK: apps/vivim-host/server.mjs
- Direct phone route: apps/vivim-host/direct-phone.mjs
- Guided UI: apps/vivim-host/public/onboarding.js
- Phone pairing entry: apps/mobile-relay/pairing-link.mjs and apps/mobile-relay/app.js
- Android native deep link: studio/android/src/inc/vivim/app/MainActivity.java
- Claude design tokens: docs/design/design-tokens.v3.json
