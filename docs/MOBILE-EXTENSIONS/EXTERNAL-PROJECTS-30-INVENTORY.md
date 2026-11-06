# 30 external Android/browser automation projects — evidence-triaged intake

**Date:** 2026-11-06. Source: owner's 30-item shortlist, independently checked against project repositories, official add-on listings, and Appium documentation. Canonical working repo: **vivim-labs** only. No third-party source installed or executed; no physical Android evidence produced.

## Evidence semantics and scope
- SOURCE = a primary project repository, official publisher, or official add-on listing supporting the core project description (not an independent security audit, phone test, or compatibility guarantee).
- PARTIAL = relevant project found but user-supplied specific name/API/variant not independently confirmed; must verify before integration.
- P0 = architecture-critical reference; P1 = useful candidate or evidence tool; P2 = peripheral/testing/diagnostics; P3 = out-of-scope or prohibited path.
- Project statements remain **maintainer claims** until replicated on user-owned physical Android with exact package/build/version.
- **No Playwright or Puppeteer** in VIVIM implementation or tests per VIVIM standing rule. No provider APIs, no browser credentials or session export, no unapproved sends. Additional restrictions still apply to developer-facing debug tools.

## Per-project evaluation

| ID | Project | Architectural function | Priority | Evidence | Grounding and crucial caveat |
| --- | --- | --- | --- | --- | --- |
| 1 | [mobilebridge](https://github.com/VulpineOS/mobilebridge) | ADB CDP proxy | P1 | SOURCE | Real Chrome via authorized ADB; single client per page; NOT on-device consumer standalone. |
| 2 | [Mobilerun (CDP mode)](https://github.com/droidrun/mobilerun) | Mobile agent / possible CDP | P1 | PARTIAL | Mobilerun mobile device control exists; distinct claimed generic per-device CDP reverse proxy not confirmed in first-party repo. |
| 3 | [bwb-browser-termux / bwb-browser](https://github.com/krshforever/bwb-browser) | Raw-CDP Termux MCP | P1 | SOURCE | Verified project name is bwb-browser; static-first + CDP Termux; does not grant access to arbitrary installed Chrome. |
| 4 | [DevDroid](https://github.com/DraxonV1/devdroid) | Browser extension + HTTP polling/CDP | P1 | SOURCE | Published PyPI links repo. Extension chrome.debugger API on Android requires browser-by-browser proof; full HTTP event capture is sensitive. |
| 5 | [chrome-protocol-proxy](https://github.com/wendigo/chrome-protocol-proxy) | CDP diagnostic proxy | P2 | SOURCE | Protocol inspection/replay; debug-only because it may expose credentials / inject CDP commands. |
| 6 | [j2v8-debugger](https://github.com/eclipsesource/J2V8) | Embedded V8 debugger | P3 | PARTIAL | J2V8 V8DebugServer code exists; distinct named project and modern Chrome CDP equivalence NOT verified; not browser WebView control. |
| 7 | [Termux Browser Pilot](https://github.com/salviz/termux-browser-pilot) | On-phone X11 browser daemon | P1 | SOURCE | Uses Xvfb, Chromium CDP or Firefox xdotool/clipboard JS, not simply controls Android Chrome; compatibility research only. |
| 8 | [AMV-Playwright](https://github.com/uno-km/amv-playwright) | Termux bundled Chromium automation | P3 | SOURCE | Playwright-based; VIVIM prohibits Playwright/Puppeteer. Study OS constraints only. |
| 9 | [playwright-termux (Jobians)](https://github.com/Jobians/playwright-termux) | Termux Playwright/Chromium | P3 | SOURCE | Playwright-based X11 Chromium; not physical Android Chrome sessions; no direct implementation. |
| 10 | [So7o Android Webview Shell](https://github.com/jan5o7o/android-webview-cdp) | OWN WebView + CDP in-app relay | P0 | SOURCE | Strong build/architecture analog: aapt2->javac->d8, WebView, own loopback CDP relay after initial ADB. Debugging endpoint exposes powerful control, NOT production safe unmodified. |
| 11 | [Damru](https://github.com/akwin1234/damru) | Redroid container and Playwright | P3 | SOURCE | Redroid Docker not physical phone; Playwright and noncommercial license; exclude implementation. |
| 12 | [dbgidchromium](https://github.com/dbgid/dbgidchromium) | Specialized browser toolkit | P3 | SOURCE | Requires separate DBG-ID-Browser; spoofing and challenge automation irrelevant to trusted VIVIM signed-in provider path. |
| 13 | [MCP Browser Bridge (Firefox Android)](https://github.com/wujie272/mcp-browser-bridge) | Android Firefox WebExtension + Termux WS | P0 | SOURCE | Direct Android Firefox+Termux bridge; content script, local WS, background, MCP tools. Inspect connection/auth/restart before copying pattern. |
| 14 | [Claude Code Browser Bridge](https://addons.mozilla.org/en-US/android/addon/claude-code-browser-bridge/) | Firefox Android add-on + local MCP | P0 | SOURCE | Firefox AMO Android listing verifies supported platform and loopback server, 19 advertised tools; maintenance/privacy/bridge installation need review. |
| 15 | [mcp-page-bridge](https://github.com/rytsh/mcp-page-bridge) | Browser extension + local Go MCP server | P1 | SOURCE | Firefox Android extension listing exists; page-exposed MCP tools are not same as authorized arbitrary third-party provider control. |
| 16 | [claude-chrome-android](https://github.com/tribixbite/termux-tools) | Edge Android extension + Termux WS | P0 | SOURCE | Edge Add-ons listing describes Termux WS bridge; verify compatibility with Vivaldi/other Chromium Android before adaptation. |
| 17 | [ANOTHERpass Browser Extension](https://anotherpass.jepfa.de/extension/) | Browser extension ↔ Android app LAN | P1 | SOURCE | Actual example is extension on OTHER device connecting to Android app via LAN. Useful pairing/transport analogy; credential vault not a model for VIVIM. |
| 18 | [claude-in-mobile / mcp-devices](https://github.com/AlexGladkov/claude-in-mobile) | ADB/native device test toolkit | P1 | SOURCE | Renamed modular mcp-devices 4.x; Android plugin requires ADB; useful physical-device test rig, not ordinary APK capability. |
| 19 | [mobilerun-mcp](https://github.com/droidrun/mobilerun-mcp) | Mobilerun MCP | P1 | PARTIAL | Official droidrun repo exists plus separate Hi-im-Connect/mobilerun-mcp CDP project; distinguish identities/features, avoid merging tools. |
| 20 | [droidrun / Mobilerun](https://github.com/droidrun/mobilerun) | Android control via Portal | P1 | SOURCE | Current naming Mobilerun with Portal Android app; device UI/accessibility integration; not a browser DOM replacement. |
| 21 | [browxai](https://browxai.com/) | MCP browser orchestrator | P2 | SOURCE | Vendor claims Android engine support; physical Android and no-provider-API compatibility not independently demonstrated. |
| 22 | [@mobileaidev/ai-app-bridge](https://github.com/mobileAiDev/ai-app-bridge) | Native/WebView testing MCP | P1 | SOURCE | Useful debug-only inspection/test workflows. Maintainer says debug builds only: no production bridge without security review. |
| 23 | [Appium](https://appium.io/docs/en/latest/ecosystem/drivers/) | Mobile app testing framework | P1 | SOURCE | Native/hybrid/web official drivers. W4 physical test harness; not packaged into customer APK. |
| 24 | [uiautomator2](https://github.com/openatx/uiautomator2) | Android UI test library | P1 | SOURCE | ADB-driven device UI server; useful test rig; UI tree cannot replace provider DOM/CDP. |
| 25 | [u2-webview](https://github.com/YuYoungG/uiautomator2-webview) | uiautomator2 + DrissionPage + CDP | P1 | SOURCE | WebView debug + ADB; useful cross-boundary test seam; not consumer release with debugging enabled. |
| 26 | [appium-devtools-plugin](https://github.com/mykola-mokhnach/appium-devtools-plugin) | Appium CDP proxy | P1 | SOURCE | Appium 3 plugin exposes CDP from mobile browser/WebView; debug test reference, not runtime adapter. |
| 27 | [appium-cdp-driver](https://github.com/AppiumTestDistribution/appium-cdp-driver) | Android Chromium W3C WebDriver | P2 | SOURCE | Project exists; evidence source for test adapters, not independent android app permission grants. |
| 28 | [qa-browser-agent](https://github.com/OxideDall/qa-browser-agent) | FSM-based test harness | P2 | SOURCE | Android uiautomator2 driver vs Playwright browser driver; reuse spec/finite-state testing ideas only, not Playwright. |
| 29 | [OpenBrowser](https://github.com/ntegrals/openbrowser) | General Playwright agent | P3 | SOURCE | Playwright-driven browser agent; outside no-Playwright production/test constraint. |
| 30 | [Autobahn-Java](https://github.com/crossbario/autobahn-java) | WebSocket/WAMP transport SDK | P2 | SOURCE | Android WebSocket primitives; not browser automation. Evaluate against Java platform support/size/cryptographic transport policy. |

## Four corrections to the incoming descriptions
1. **So7o** is not only an ADB-to-WebView proxy. Its README describes *in-app* loopback CDP relay after an initial developer setup, explicitly requiring debugging and warning that **other phone apps with INTERNET can access an unprotected loopback endpoint**. Treat as security-critical debug research, not VIVIM product design without authenticated endpoint/role separation.
2. **Termux Browser Pilot** is not simply an automation interface to the user's already logged-in Android Chrome; it runs its own Xvfb/X11 browser process (Firefox or Chromium). Mobile provider login/profile parity is distinct.
3. **ANOTHERpass** demonstrates an extension on another browser/device talking to a credential-holding Android app on a LAN. This is useful transport prior art but **not proof of Android-browser-extension ↔ same-phone APK compatibility**; do not reuse credential flow.
4. **Mobilerun** now appears under multiple related and independent projects. First-party Portal/MCP capabilities do **not** establish the specific per-device Chrome CDP reverse-proxy claim without exact source path. Keep item 2 partial.

## Compatibility and architecture extraction
**Reusable seam:**
`browser content script (read or synthetic events) → extension worker → outbound authenticated local WebSocket/HTTP → Android APK-owned capability broker (validated typed envelope) → owner approval/Host grant and journal → verified status/receipt`

The **extension must never become a new Host effect owner**. A local MCP server can be a developer diagnostic or intent client, not a trust/authority boundary. Session cookies, private URLs and raw page bodies must not be transmitted by default. The existing passive VIVIM Browser Companion is a separate read-only observation producer.

**Critical independent alternatives:** Vivaldi 8.3 + custom MV3 extension (official unpacked install); Firefox Android + a local-WS extension; app-owned GeckoView built-in WebExtension; existing APK/WebView + strictly isolated CDP-style debug experiment. No selection until browser and native lifecycle proof.

## Next work (M1–M4, no ninth TierFlow lane)
- M1 (Host primitives): current page methods, CDP send-once preflight and provider effect receipt contracts; set strict required feature list.
- M2 (Browser bridge): inspect #13, #14, #16, #4 and Vivaldi 8.3 extension compatibility; make a **synthetic only** origin-scope/permission/worker-loss fixture.
- M3 (Native broker): inspect #10, #22 and native APK constraints; explicitly disallow unauthenticated CDP debug listener and production WebView debugging.
- M4 (Physical verification): use #23–26/#18/#20 in controlled test environment where permitted; verify Playwright-free paths, OEM battery kill, pairing, revoked permissions, uncertain send, and background tabs.

**Wave 4 production source HOLD until published I3 GO and exact owners' grants.** This is actionable research preparation only.
