# Architecture triage after 30-project intake — VIVIM Android/browser parity

Date: 2026-11-06. This is Labs research, not production approval or device proof.
Reads: EXTERNAL-PROJECTS-30-INVENTORY.md, SOURCE-PRIMITIVES.md, HOST-CAPABILITY-DEPENDENCIES.md, BROWSER-ENGINE-DECISION.md, WAVE-PROGRAM-ALIGNMENT.md.

## New comparison: three viable routes, one existing APK

| Route | What it solves | Best prior art | What does NOT follow | Test order |
| --- | --- | --- | --- | --- |
| **E — Extension client → authenticated APK-owned local broker** | DOM/selector/tab operations inside an external supported Android browser, using outbound WebSocket/HTTP from extension | #13 wujie272 Firefox+Termux; #14 Firefox AMO; #16 Edge+Termux; #4 DevDroid; Vivaldi Android 8.3 extension install | Browser background reliability, app privileges, Host permissions, worker crash recovery, true desktop CDP parity | **First** (synthetic) |
| **W — Existing APK WebView with restricted synthetic CDP/relay lab** | Closest to current VIVIM Android Java WebView and desktop `Browser/Page` CDP abstraction; no customer-installed browser dependency | #10 So7o (plain aapt2/javac/d8 like VIVIM), #1 mobilebridge for external developer control | Source's unauthenticated CDP loopback listener is an unacceptable release posture; debug capability cannot silently ship; no browser extensions; Android WebView provider UX/provider site login parity untested | **First** isolated debug research, never ship relay unchanged |
| **G — App-owned GeckoView with built-in privileged WebExtension** | Controlled app-owned browser and Gecko native messaging, own extension packaging | Mozilla GeckoView WebExtension docs, Firefox Android add-ons | Requires new engine and build/dependency toolchain; provider mobile compatibility and runtime memory unknown; not equivalent to Firefox user-installed add-on privileges | **Second**, after Android build feasibility |

**Mandatory baseline:** current `inc.vivim.app` APK (Pocket + paired `apps/mobile-relay`), Windows Studio/CDP sole browser-effect owner, Host canonical World writer, Repos HQ KB source, TierFlow shared UI (only after gated W6 cutover). The user's near-term phone-controlled PC harness path remains intact even if phone autonomous browser research succeeds.

## Critical architecture correction from the shortlist

So7o's project README reports:
- `WebView.setWebContentsDebuggingEnabled(true)` allows the app WebView's DevTools socket.
- An in-app `RelayServer` can proxy that socket on **127.0.0.1:9334**, after installation, so a Node CDP client can connect without a contemporaneous ADB forward.
- The companion Termux/adb path and keep-alive service are *optional in certain test states*; background survivability has been measured only in documented devices/conditions.
- **Localhost is not an app-authentication boundary**: any Android app with INTERNET permission may reach a listening TCP socket on the same phone. The source warns of that exposure.
- Android `Target.createTarget` and /json/new do not behave like desktop Chrome: don't assume multi-tab parity.
- The repo's app is intentionally a debug/lab shell, not a production trust/security authority.

**VIVIM policy:** current `studio/android/src/inc/vivim/app/MainActivity.java` enables debugging only for debuggable builds. Keep it that way. Never add unauthenticated CDP, arbitrary `Runtime.evaluate`, shell execution or all-origin native bridge to a shipping VIVIM APK. For an eventual consumer-grade path, research a *typed constrained in-process adapter* instead of exposing raw debugger functions. Full CDP may remain a dev-only upper-bound test oracle. No code import is approved.

## The contract must remain ONE typed trust boundary

```
Owner text/tap → shared deterministic typed action → owner/device-scope proposal
    ├── local phone browser observation (no effect)
    └── proposed browser effect → Host grant/plan/journal/effect owner
                                  └── existing Studio/PC (release path)
      extension ↔ authenticated, origin-bound, nonce-fresh capability proxy (lab only)
        ^         no privileged generic eval / secrets / provider sends in current release
        |
 current one VIVIM APK: Pocket + paired mobile UI, NOT a second autonomous app

Device-local raw capture: independent explicit owner-intake evidence, NOT KB revision.
```

For future owner-authorized mobile autonomy, the owner would need a *specific new authority decision* preserving one logical effect owner and fresh approval/journal semantics, not a stealth alternate effect execution path. Wave 4 research may measure the maximum technical capability without adopting it as current policy.

## Security proof requirements BEFORE any native bridge/extension pilot

1. Explicit extension/app installation provenance and update integrity; no silent sideload.
2. Strong app↔extension pairing with independently generated short-lived proof; binding of origin/browser tab, owner/device and allowed actions; no raw Studio token or NativeSigningPort secrets shared.
3. Non-guessable capabilities plus freshness/replay rejection; note that WebSocket Origin may be spoofed by native apps, so origin check alone is insufficient.
4. No generic CDP or JS evaluation exposed to arbitrary installed apps on TCP localhost.
5. Local listener loopback only, no LAN unless independently owner-approved secure transport; a listener on 127.0.0.1 is still accessible from other phone apps.
6. Browser origin/frame permission changes, tab takeover, concurrent clients, new navigation between preflight/act, site-auth changes, offline/journal restart, Android force-stop; require fail-closed outcomes.
7. Normal Android OS restrictions and notification/foreground-service policy; don't claim a 24/7 resident daemon based on one short foreground test.
8. Real provider sends remain forbidden in lab; synthetic web fixture only; no cookies/tokens/full conversations in published evidence.
9. Test both Android app delivery and browser extension delivery: never confuse an extension's queue ACK with proof the Host performed an effect.
10. Browser extension site access is not OS-wide native app access. Accessibility/ADB/UiAutomator only under separately reviewed privileges.

## Lab work ownership (no added TierFlow waves)

- **M1 primitive truth**: exact Browser/Page method and effect-owner semantics; prove why extension scripting or WebView differs from `harness/src/core/cdp.mjs` on each method.
- **M2 browser experiments**: first Vivaldi Android 8.3 MV3 synthetic extension, Firefox Android #13/#14 reference, Edge #16, Yandex/Quetta comparison; verify WebSocket loopback / worker lifecycle and permissions.
- **M3 native broker experiments**: architecture drawing and debug-only study of So7o #10 inside *isolated Labs design/fixtures*, with secure capability proxy design; no alteration to `MainActivity.java`, APK build or manifest yet.
- **M4 test/evidence**: R0 source inspect → R1 synthetic contract tests → R2 physical Android/Windows local proof; issue separate reviewed W4 owner file grants only after I3 GO.

## Decision checkpoints

D0 source intake all 30 / exact URLs: **complete enough for shortlisting**, source descriptions 27 primary + 3 partial; not 30 code reviews.
D1 detailed code-level inspection of #10/#13/#16 origin authentication, transport and lifecycle: **pending**.
D2 Android browser installs and synthetic fixture: **not executed**.
D3 physical device background/energy and provider read-only compatibility: **not executed**.
D4 owner chooses a single released path / compatibility fallback after evidence: **not decided**.

**Provisional testing priority:** (E) Vivaldi/Firefox extension, (W) existing APK WebView in *debug-only* control, (G) GeckoView as alternate owned-engine design. Selection depends on measured correctness, security, build complexity and lifecycle—never AI implementation cost.
