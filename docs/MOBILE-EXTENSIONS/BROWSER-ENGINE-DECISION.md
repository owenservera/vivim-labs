# Mobile browser engine decision — staged evidence, not a selected product runtime

Research 2026-11-06. The problem is method-level equivalence to existing Browser/Page and send-once contracts, not maximal extension store count. No real phone or provider-site execution performed.

## Options
| Candidate | Main opportunity | Fundamental cost/constraint | Research verdict |
| --- | --- | --- | --- |
| Existing Android WebView + origin-scoped JS | Lowest change to current inc.vivim.app APK, same Pocket/Direct shell, controlled trusted pages | NOT a generic Chromium extension platform; injecting arbitrary third-party JS carries security and origin/behavior limits; not desktop CDP | Keep baseline/control; test only trusted synthetic pages first |
| Embedded GeckoView + built-in WebExtension | App-owned Gecko browser engine, explicit extension/native messaging supported in official API; bundled content scripts with controlled messaging | Different engine from Chromium, potentially different provider experiences; additional dependency/packaging/release implications; cannot assume fits plain javac/aapt2 build | **Strongest app-owned extension candidate**, NOT approved engine replacement |
| External Firefox Android extension | Official user-installed Android add-ons, independent Gecko compatibility reference | App cannot assume arbitrary extension install/native bridge/profile control; inter-app link/localhost transport needs independent proof | P0 independent test comparator, not automatic production foundation |
| External Quetta Android + Chrome extension | Vendor officially advertises Chrome Web Store/Edge add-on install; closest extension format to existing MV3 Companion | Chrome desktop APIs/worker/permissions not guaranteed; trust, service lifecycle and app integration depend on vendor | P0 Chromium compatibility comparator, not trusted effect owner |
| Edge Android stable | Curated supported extensions | Custom VIVIM extension not generally installable via stable path | Lower priority for custom deployment; Canary/Beta are developer experiments only |
| Developer ADB/debugging | Can inspect controlled test phone and sometimes drive browser remotely | No consumer-default installation or always-on privilege, consent/security risk | Diagnostics only, never production architecture |
| Existing paired PC Host fallback | Desktop Chrome/CDP and journal retained with current authority | PC must be available for provider sends/canonical World writes | Required reliable baseline during Wave 4/5, preserve until explicit architecture decision |

## Official API evidence
- Mozilla GeckoView built-in WebExtension registration and native messaging: https://mozilla.github.io/geckoview/consumer/docs/web-extensions
- GeckoView WebExtension API installBuiltIn/ensureBuiltIn: https://mozilla.github.io/geckoview/javadoc/mozilla-central/org/mozilla/geckoview/WebExtensionController.html
- Mozilla Firefox for Android add-ons: https://support.mozilla.org/en-US/kb/find-and-install-add-ons-firefox-android
- Quetta own documentation: https://www.quetta.net/
- Edge Android custom extension caveat (third party, test rather than assume): https://github.com/microsoft/amplifier-browser-bridge/blob/main/docs/ANDROID.md
- Android foreground service API34 rules: https://developer.android.com/develop/background-work/services/fgs/changes
- Android WebView security: https://developer.android.com/privacy-and-security/risks/insecure-webview-native-bridges

## Documentation resolution
Context7 queried 2026-11-06: /mozilla/geckoview (version not supplied by Context7; current doc Javadoc observed), and /websites/developer_android (platform API34 target in existing APK, docs not version-pinned). Official GeckoView docs confirm special geckoViewAddons / nativeMessagingFromContent permissions for bundled privileged extension/native messaging. Do not equate this with normal Firefox Android external extension privileges.

## Proposed first decision
Compare three different architectures, in parallel *as research fixtures only*: (1) current WebView trusted-origin observation, (2) embedded GeckoView built-in WebExtension/natively mediated synthetic content, (3) external Firefox + Quetta extension synthetic fixtures. Select no browser until signed-in provider compatibility, update/maintenance, app packaging, auth, background and receipt semantics are measured.

## Nonnegotiable test questions
1. Can synthetic content scripts observe exact target identity, DOM/readiness and focus? What is blocked by cross-origin/frame permissions?
2. Can the browser preserve multiple signed-in sessions across restart and controlled API permission revocation? No credential export allowed.
3. Can Android run in foreground/background/screen-off and return honest stop/uncertain state without duplicate external effects?
4. Can observations reach existing authenticated Host transport without exporting local bearer/CDP or granting browser effect authority?
5. What does add-on installation/update/distribution look like for a customer, not a developer? Is browser vendor support dependable?
6. Would a GeckoView dependency require changing the current plain aapt2/javac/d8 APK build? If yes, gate it behind owner-approved build work.

Current architecture proposal: ONE existing APK, ONE phone UI, independent capability adapter under Host-governed contracts. GeckoView is a candidate for app-owned browser control, Quetta/Firefox independent evidence comparators. No runtime migration approved.
