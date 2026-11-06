# Android browser extensions: broader VIVIM harness survey (2026-11-06)

**Research status:** browser vendor docs and maintained project documentation checked on 2026-11-06. No device installs/extension API experiments done; no provider sends authorized. This is LAB research feeding existing Wave 4 B/C/D/G/H, not another wave or product app.

## Major new finding: Vivaldi 8.3 is now P0

Vivaldi **8.2 (2026-09-10)** added Android extension support. Stable **8.3 (2026-11-05)** is current at review. Vivaldi's Android-specific help explicitly supports:
- Chrome Web Store, requiring *Desktop site* for installation.
- **Developer Mode → Load unpacked** from Android device storage.
- Local **CRX drag/install**, enable/disable, permission/site access and extension menu/pinning.
- Ordinary browser UI also includes tab stacks and browser features, but those are not evidence of programmable automation APIs.

Sources: https://vivaldi.com/blog/vivaldi-on-mobile-8-2/ ; https://vivaldi.com/blog/android/releases/ ; https://help.vivaldi.com/android/android-tools/extensions-on-android/

### 8.3 extension risk register (changelog issues, not proof of still-active bugs)
- VAB-13594: runtime host permission request not shown for an MV3 extension.
- VAB-13705: extension-opened background tab reloads when first shown with Desktop Site.
- VAB-13534: manually installed extension re-enabled after browser restart.
- VAB-13671: browser permission popup will not open.
- VAB-13693/VAB-13557: some install failures; VAB-13580: keyboard opening in extension UI.
- VAB-13742/VAB-13631: private-mode extensions.
Source: https://vivaldi.com/changelog-vivaldi-browser-8-3-on-android/
**Re-test these on a real installed 8.3 device**, because changelog inclusion may indicate a fix rather than an active defect.

## Candidate matrix — primary-source anchored

| Candidate | Install VIVIM-built extension | Evidence of mobile extension path | Main VIVIM open question | Priority |
|---|---|---|---|---|
| **Vivaldi 8.3 Android** | **Yes, documented unpacked/CRX** | Official Android help + 8.2/8.3 release notes | MV3 background, tabs, injection, permission/revocation fidelity; native APK communication | **P0 #1 Chromium** |
| **Yandex Android** | **Yes, official unpacked developer upload** | Official extension developer/USB remote-debugging guide | Real provider login/DOM fidelity, local transport, privacy | **P0 #2 Chromium** |
| **Titanium Android (formerly Helium)** | **Yes, project documents unpacked/CRX/ZIP** | Open-source maintainer repository, Chromium/Vanadium-derived | Maintainer/update/supply-chain verification and Android browser API semantics | **P0/#3 Chromium research** |
| **Quetta Android** | Store installs documented; unpacked workflow not confirmed | Official Chrome Web Store + Edge Add-ons announcement | Development/distribution of our own extension | **P0 comparison** |
| **Firefox Android** | Mozilla add-ons documented; own distribution path needs validation | Official Firefox mobile add-on support | Compatibility with Chromium MV3 manifest and provider DOM/transport | **P0 Gecko comparator** |
| **GeckoView embedded** | Built-in privileged extension via app-owned runtime (not browser app) | Mozilla GeckoView docs | Android APK packaging and provider website compatibility | **P0 app-owned architecture experiment** |
| **Edge Android Stable** | General arbitrary local unpacked not verified | Curated extensions; Canary has developer install-by-ID documented by Kiwi maintainer | Version-specific install API, stability | **P1 / Canary experiment only** |
| **Lemur** | Site advertises Chrome/Edge extensions | Browser vendor site | Custom unpacked, updates, extension API coverage | **P1/P2** |
| **Mises** | Play listing advertises extensions | Android Google Play listing and user reports | Installation/API evidence, security and maintenance | **P1/P2** |
| **Samsung Internet** | Different Galaxy Store Android addon program, not generic MV3 | Samsung developer docs | Restricted developer program and different addon architecture | **P2** |
| **Cromite** | Experimental *disabled-by-default* patch; FAQ says no supported extensions | Maintainer FAQ and development patch | Whether experimental path becomes maintained | **P2 research only** |
| **Chrome Android** | No general desktop extension support | Chrome mobile baseline | Negative control | Negative |
| **Brave Android** | No broad desktop-extension mechanism established | No primary evidence sufficient for custom MV3 parity | Negative control | Negative |
| **Kiwi** | Historical yes; archived as of January 2025 | Kiwi maintainer README | No active maintainer / security release path | Exclude production |

## Important direct sources
- Yandex custom unpacked developer + remote debugging: https://yandex.com/support/browser-mobile-android-phone/en/personal-settings/extensions
- Titanium project README: https://github.com/jqssun/android-titanium-browser
- Quetta official: https://www.quetta.net/blog/quetta-unveils-browser-extension-support-for-android-devices
- Firefox mobile addons: https://support.mozilla.org/en-US/kb/find-and-install-add-ons-firefox-android
- GeckoView web extensions: https://firefox-source-docs.mozilla.org/mobile/android/geckoview/consumer/web-extensions.html
- Lemur vendor: https://www.lemurbrowser.com/app/en.html
- Mises Play listing: https://play.google.com/store/apps/details?id=site.mises.browser_stable
- Samsung developer: https://developer.samsung.com/browser/android/extension-guide.html
- Cromite maintainer FAQ: https://github.com/uazo/cromite/blob/master/docs/FAQ.md
- Kiwi maintainer: https://github.com/kiwibrowser/src.next

## Not all extension support is equal: decisive VIVIM acceptance

Desktop harness **actual required methods** per Harness source: browser targets/openPage/attach, evaluate DOM/query and origin binding, focus, insertText, click/Enter, preflight auth-wall/draft/thread checks, positive outgoing/reply evidence, send-once journal, uncertain/recovery. Browser Companion today is **passive** MV3 tabs/webNavigation metadata; not an actuator.

Test all 4 P0 external browsers with the **same harmless synthetic extension fixture**:
1. Install/unpack and update extension; check extension ID / permissions.
2. `tabs`, `webNavigation`, `storage`, `alarms`, `scripting`, `runtime` methods one at a time; unsupported APIs explicitly noted.
3. Content-script host isolation, frame scope, site permission denied/revoked, SPA navigation; never inspect real secrets.
4. Foreground DOM focus/insert, simulated click and attributable reply on local synthetic provider page **without external sends**.
5. Background app, screen off, OEM battery restriction, browser kill/restart; distinguish lost vs uncertain action.
6. External extension to existing APK/Host authenticated transport; **never inherit Chrome desktop `127.0.0.1` Studio ingress or extension origin without platform design**.
7. Multiple logged-in provider webapps only with user consent, read-only, after synthetic pass; actual provider sends require separate Host approval.
8. Install/update and long-term browser security/maintenance story.

## Decision recommendation now
**Vivaldi 8.3 first**, **Yandex second**, **Titanium third**, **Quetta fourth** for Chromium probes; **Firefox Android** independent Gecko reference; **GeckoView** app-owned engine research track; **existing APK WebView + paired PC Host** compatibility fallback throughout Wave 4.

This is a prioritization for experimentation, not a product dependency selection. A browser's feature-rich UI cannot grant Android app sandbox privileges, background survivability or authority to send AI provider prompts.
