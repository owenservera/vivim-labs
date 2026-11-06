# Android native capability research — recovered and reconciled

Date: 2026-11-06. Status: research synthesis; NOT a physical-device acceptance or Android app feature-complete claim.

## Prior research recovered
- Earlier APK07 investigation (2026-10-31/11-01) distinguished ordinary APK; explicit user-enabled Accessibility/notifications/screen capture; developer-assisted ADB/Shizuku; managed device owner; root. Increasing privilege is not a supported-default customer strategy.
- Prior browser harness options: existing Android WebView, GeckoView plus WebExtensions, extension-capable external Chromium/Firefox, ADB/CDP with attached debugging, and cross-app Accessibility. No option is unrestricted desktop Chrome/CDP equivalence.
- Earlier decision retained a native APK shell + VIVIM Direct/Host pairing rather than a TWA-only foundation. Reprogrammable UI/flows are distinct from privileged native features, which require platform implementation and potentially APK update.
- Earlier research specifically warned that Android Accessibility is an Android UI tree/gesture surface, **not** desktop CDP.
- Source of truth: `vivim-web-harness/docs/design/APK07-mobile-runtime-world-offline.md` (now PARTIALLY SUPERSEDED), `repos-hq/docs/architecture/MOBILE-ONE-AUTHORITATIVE-PATH.md`, `vivim-web-harness/docs/design/UNIVERSAL-TEXT-CONTROL-V1.md`.
- Do not resurrect superseded offline action queue, multi-writer World/KB, CRDT sync, second phone UI, or phone-owned provider sends.

## Android execution tiers
| Tier | Mechanism | What is potentially possible | Important constraint |
| --- | --- | --- | --- |
| A0 | Ordinary signed APK + supported Android APIs | Own UI, sandboxed data, file/share intents, app links, explicit content picker, local compute, configured notifications | Cannot directly read private data/processes/DOM of arbitrary other apps |
| A1 | Explicit opt-in system services | Accessibility UI observation/gestures; notification listener; consented media projection | Sensitive authorization, incomplete UI trees, protected windows, revocation and store/policy limits |
| A2 | Controlled browser with extension | Per-origin content scripts, tab operations, messaging, persistent content extraction where supported | Not OS-wide, browser/version/API-specific, background process survival not guaranteed |
| A3 | Developer assistance (ADB/Shizuku) | Additional shell/debug operations on configured devices | Not consumer-default, security-sensitive, revocable and setup-dependent |
| A4 | Managed device owner / enterprise enrollment | Device-policy scope under provisioning | Not accessible to ordinary user-installed APK |
| A5 | Root / custom OS | Much wider OS access | Not a viable standard consumer requirement |

## Important distinction: OS impossibility vs app limitations vs VIVIM policy
- OS hard boundary: app sandbox; no unrestricted foreign app database/files, browser profile/CDP ownership or OS shell from ordinary APK.
- API/permissions boundary: share, file picker, accessibility UI, notifications, MediaProjection; operation depends on user consent and target app behavior.
- Lifecycle/reliability boundary: foreground/background starts, app kills, idle, OEM restrictions and foreground service types.
- Browser engine boundary: Android WebView is an embeddable page renderer, not a Chrome desktop-equivalent browser with generic extension API.
- VIVIM policy boundary: DEC-MOBILE-001 currently makes PC Host the only canonical World/KB writer and provider effect sender. This is an **architecture decision**, not itself an Android OS prohibition.
- Product gap boundary: native path described in docs may not be built/wired/tested. Source existence must not be labeled proven phone behavior.

## Current primary-source platform references
- Android accessibility service UI observation: https://developer.android.com/guide/topics/ui/accessibility/views/service
- Package visibility filtering: https://developer.android.com/training/package-visibility
- Foreground services / API-specific restrictions: https://developer.android.com/develop/background-work/services/fgs/restrictions-bg-start
- MediaProjection explicit consent: https://developer.android.com/media/grow/media-projection
- WebView is a web content view, not full browser: https://developer.android.com/develop/ui/views/layout/webapps/webview
- WebView JS bridge risk: https://developer.android.com/privacy-and-security/risks/insecure-webview-native-bridges

## Security/consent conditions
No unrestricted JS bridge on arbitrary provider origins; isolate any trusted VIVIM-native messaging endpoint; use scoped origin checks and typed commands. Never treat Accessibility as a credential/database access replacement. Prove permission revocation, OS kill, sensitive-screen restrictions and no duplicate sends.

## Unverified
Physical-phone support, provider webapp mobile layouts and anti-automation behavior, Firefox/Chromium extension API compatibility, Android SDK/target-specific edge cases, local persistent service survival and support across OEMs have not been run in this research pass.
