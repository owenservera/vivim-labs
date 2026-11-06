# Source-inspected Host and Android primitive census — first audit

Status: static source inspection on main 2026-11-06. These are implementation primitives, NOT verified end-to-end phone capabilities. The current Host action-catalog v6 contains 19 actions; this list addresses the distinct lower-level browser/Android dependencies, so action count is not parity coverage.

## Existing executable-path anchors
| Primitive | Actual source path / symbol | Observed source implementation | Mobile parity blocker |
| --- | --- | --- | --- |
| Dedicated browser identity and ownership | vivim-web-harness/studio/src/chrome/browser-ownership.mjs: verifyBrowserOwner/checkBrowserOwner | Validates Chrome profile/DevTools endpoint identity marker; port alone insufficient | Android browser profiles do not provide same controlled CDP ownership |
| CDP browser sessions | studio/src/cdp/client.mjs: CdpConnection/CdpSession/ResilientCdpConnection | Browser WebSocket, flattened session attach, reconnect/status | Standard Android apps/browser extensions do not expose desktop Chrome DevTools socket |
| Browser/Page seam | harness/src/core/cdp.mjs: connect, targets, attach, openPage, page.evaluate/insertText/click/pressEnter/close | Concrete desktop CDP adapter | Extension scripting is limited by origin/API; cannot assume input events and target lifecycle equivalent |
| PinchTab alternative | harness/src/core/pinchtab.mjs: connectPinchtab | Instance-scoped tab list, evaluate, keyboard/click, open; HTTP close is explicitly unsupported | Not an Android native API; requires separately running compatible host/runtime |
| Provider send-once | harness/src/core/send.mjs: snapshotExpression, focusExpression, sendOnce | Preflight auth wall/composer/empty draft/target, exact payload digest, typing and dispatch, observation or uncertain | Exact focus/input, visibility, provider page DOM and durable send journal need cross-browser proof |
| Durable browser turn fence | studio/src/capture/thread-turn-journal.mjs: reserve/markTyping/markDispatching/readiness | Replay/uncertain safety and thread ownership, not a simple click receipt | Background kill/crash must NEVER auto-retry an uncertain external send |
| Passive browser observation | apps/browser-companion/manifest.json + sensor.mjs + protocol.mjs + service-worker.js | MV3 tabs/webNavigation/storage/alarms; no content script/DOM reads/CDP/browser mutation or provider sends | Chrome-only origin/loopback transport and support of MV3 events vary on Android |
| Extension -> Studio validation | studio/src/studio/browser-companion/ingress.mjs: companionIngress | Validates chrome-extension:// origin, loopback socket/token, capped envelope and sanitizer | Cannot blindly reuse on Firefox/GeckoView/external Android browser: transport identity and origin differ |
| Canonical action control | apps/vivim-host/control/{action-catalog.v1.json,dispatcher.mjs,journal.mjs} | v6 typed action catalog (19), Host-only plan/approve/execute/receipt workflow | Existing owner grants and one effect path; browser extension never becomes a permission grant |
| Phone command transport | apps/mobile-relay/transport.mjs: createTransport/request | Device session for Direct, uncertain-mutation fence | Mobile browser adapter must not bypass this approval/auth surface |
| Phone UI projection | apps/mobile-relay/generative-ui/phone-surface.mjs: mountBuildSurface/sendProposal | Host-committed UI revision render + proposal command; no direct provider send | Canonical UI remains shared; native/browser adapter is execution capability, not new UI publisher |
| Existing native APK | studio/android/AndroidManifest.xml; studio/android/src/inc/vivim/app/MainActivity.java | package inc.vivim.app, minSdk26/target34; Java Activity/WebView; Keystore-based signing port; native Pocket capture; renderer loss handling | WebView has no drop-in Chrome MV3 extensions, current APK build is plain aapt2/javac/d8 without Gradle |
| APK source packaging | studio/android/build.sh; build.ps1 | Plain SDK build-tools 34 path, source enrollment, D8/signing | GeckoView embedding requires separately investigated dependency/build packaging; not merely swapping one Java class |
| Pocket local data | studio/android/src/inc/vivim/app/PocketHome.java and PocketStore | Native screen for owner-created local raw captures; not a KB writer or queued effect | Preserve durable local capture + explicit Host intake boundary |

## Blocking dependency chain
Owner/device scoped identity -> typed action/readiness -> provider origin/session binding -> observed DOM/controls -> deterministic proposed action -> Host scoped approval -> exactly-once effect claim/driver -> attributable reply evidence + receipt -> canonical revision/provenance -> phone/PC UI.

Observations remain separate from actions: Browser Companion navigation events may enter an evidence pipeline, but no record, inferred state or extension message grants permission.

## Source-level limits discovered
- Browser Companion's 0.1.0 MV3 manifest has no content_scripts and its sensor explicitly says no DOM/CDP/mutations. Reuse for passive sensor lineage, not as an actuator.
- Its sender POSTs to 127.0.0.1:Studio and its ingress accepts Chrome extension origin. Both are desktop-specific seams requiring an approved portable transport bridge for Android.
- APK WebView includes a restricted native signing message port. This is NOT a general purpose arbitrary browser-control bridge and must remain origin-scoped.
- Android Manifest declares INTERNET, ACCESS_NETWORK_STATE, POST_NOTIFICATIONS and RECORD_AUDIO; does not declare AccessibilityService, notification listener, VPN service, generic foreground service or device-owner capability. These cannot be assumed present.
- Normal signed-in mobile sites may present different DOM/composer/visibility behaviour. Desktop sendOnce assumptions must be tested by provider and browser, including uncertain and double-send cases.
- Source-wired functionality is not guaranteed on a physical Android device; current status: NO mobile extension runtime/device test performed in this lab.

## Remaining M1 evidence before any parity percentage
- Enumerate actual Studio provider driver/profiles, capture L1/L2/L3, active launch route and tests at exact main SHAs.
- Identify whether desktop Host execution can be represented by a bounded cross-platform Page interface, and document method-specific semantic loss.
- Identify which operations are Windows-only, which are source-only and which actually have active callers.
- Do not label anything '100% mobile parity' without device-level primitive test results.
