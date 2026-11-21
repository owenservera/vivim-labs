# Owner override — ONE VIVIM UI (2026-11-21)

## Product decision (supersedes earlier same-day GPUI-only desktop UI directive)
There is ONE user-facing VIVIM product interface: the **existing First Contact text-box-first PWA**. The owner personally rejected the native GPUI diagnostic workspace with Overview/Wiki/Build/Graph/Decisions tabs and asked to restore the previously working Edge app: one composer, Wiki projected in response to typing, with additional functionality integrated into that same surface. Do not launch, develop, recommend or present alternate user-facing dashboards as the product. No parallel browser vs native desktop presentation competition.

## Canonical existing code and URL
- Source: ARBOR `apps/mobile-relay/index.html` and `apps/mobile-relay/app.js`, with `FIRST_CONTACT` and existing `first-wiki` and composer integrations. Preserve source-backed Wiki, host-owned effects, permissions, existing Rust/WASM, journal, grants and native HQKG. Do not replace with a fresh UI/backend.
- Currently verified loopback Host URL: `http://127.0.0.1:43187/mobile/index.html?first=1` (HTTP 200 on PC on 2026-11-21). Existing Edge `--app` window title `VIVIM Direct` observed.
- PC shortcut: `C:\\Users\\VIVIM.inc\\OneDrive\\Desktop\\VIVIM.lnk`, launching Edge app to the above local URL. Earlier `VIVIM GPUI.lnk` archived at `VIVIM-Archived-Launchers-20261121`. Local shortcut change verified by terminal output; visual live Wiki acceptance remains UNRUN.

## Interface inventory / disposition
| User-visible surface | Disposition |
|---|---|
| First Contact composer + AutoWiki | **CANONICAL**; converge all user-facing work here |
| Old full Direct phone shell with tabs/drawer | Legacy feature source only; no competing product entry point; archive/deprecate after dependency audit |
| Native GPUI diagnostic workspace | **RETIRED AS USER-FACING UI**; preserve reusable Rust/Host/projection code; do not destroy dependencies or remove history |
| Tauri/WebView/native fallback/other launchers | No user-facing launcher; archival/deprecation audit, no destructive blind deletion |
| Developer-only Studio/graph/diagnostic tools | Internal implementation features only; don't surface as a second VIVIM product UI |

## Required source cleanup — not yet falsely marked complete
1. Check out task atomically in shared TASK-CLAIMS and coordinate prior Dev owner of `apps/mobile-relay/app.js`; read live main source head. Do not race concurrent source writers.
2. Make First Contact the **single default app entrance** on Windows and phone without needing `?first=1`; retain a safe explicitly restricted internal route for diagnostics, not a user alternate. Preserve pairing/approvals, privacy and Host effects. No client-side bypass of auth conditions.
3. Audit all desktop shortcuts, packaging scripts, app-menu launch paths, start-menu entries, documentation, remote URLs and historical browser/GPUI modes. Archive alternative launchers with hashes and references, disable their product entrypoints only after regression tests.
4. No new UI framework, no second composer, no alternative Wiki engine. One shared input and one revision-fenced AutoWiki projection, including user and authorized agent typing.
5. Test with the real Windows app and phone PWA, typed input, Wiki changes, approval refusal, refresh/reconnect and screenshots. Evidence must separate SOURCE / TEST / SERVE / VISUAL. Do not claim UI consolidation complete from a shortcut or HTTP 200.

## Status at writing
**PARTIAL LOCAL CUTOVER, SOURCE CONSOLIDATION PENDING.** The old GPUI source and full Direct shell still exist. The one-UI rule is authoritative for future development, but the repository has not yet been surgically migrated and visually verified. Earlier GPUI-only launch directives are superseded, not deleted from history.


## Verified single-UI implementation and Windows acceptance — 2026-11-21 (supersedes the initial status snapshot above)

- **SOURCE:** ARBOR `main` commit `cf2e5c8da0d5f46f17a5de0cc24ab838778e60c0` (preceded by `802e352`); `FIRST_CONTACT = transport.direct && !HOST_PREVIEW` now defaults the existing Direct Host PWA to the single text-box-first interface. `?first=1` no longer required; Host preview retains separate read-only boundary.
- **BUILD/TEST:** immutable archive `C:\\0-BlackBoxProject-0\\.arbor-pwa-releases\\cf2e5c8da0d5` produced BUILD_PASS; Rust composer 4/4, Rust activity 20/20, PWA browser composer 11/11, Host bridge 5/5, Host HTTP 1/1, default First Contact 9/9 (50 tests across the focused suites), Rust binary, WASM and JS syntax all PASS. A test regex defect found in the first build was fixed in the cloud and re-tested; this final SHA was used for deployment.
- **LIVE HOST:** owner-approved, rollback-aware local execution through the archived-release script `OWNER-WORK/scripts/deploy-one-ui-r001.ps1` reported `CUTOVER_PASS`; local Host `127.0.0.1:43187`, PID 17380, LIVE_SERVED_SHA pinned to `cf2e5c8`. The dirty ARBOR and Labs checkout were not modified.
- **AUTH ROUTING:** local loopback `/mobile/api/first-contact/wiki` yielded HTTP 404 under the existing Host boundary; **do not bypass this**. The exact Tailscale owner HTTPS URL `https://desktop-9kb03lu.taild7271a.ts.net/mobile/index.html` and owner-authenticated Wiki path returned HTTP 200, TLS verify 0. Canonical PC shortcut now points to this HTTPS origin; mobile remains the same Tailnet Host.
- **WINDOWS REAL UI:** native GPUI process cleanly closed. Edge app `VIVIM Direct` shows only the blank `Ask anything…` text box and a dynamically mounting Wiki panel. Actual Windows keystrokes in the existing field: `Roadmap` → **4 Wiki matches, revision 1**, `Main` → **1 Wiki match, revision 1**, source screenshots visually inspected; clearing input hides Wiki. Screenshot paths under local release root: `one-ui-https-empty.png`, `one-ui-https-roadmap.png`, `one-ui-https-main.png`, `one-ui-final-blank.png`. No Send action.
- **DESKTOP ENTRY POINTS:** Desktop `VIVIM.lnk` and Start Menu `vivim.lnk` both target Edge `--app=https://desktop-9kb03lu.taild7271a.ts.net/mobile/index.html`; previous GPUI desktop shortcut and previous native Start Menu shortcut archived under `VIVIM-Archived-Launchers-20261121`. Underlying native GPUI source/binary intentionally kept for recoverability, but not an accepted product UI.
- **ZCODE:** CLI 0.18.0 and headless plan route verified. Independent read-only ZCode audit failed upstream OpenRouter free model 429, so no agent work claimed, no paid fallback; ChatGPT authored GitHub source and used scoped local Desktop Commander for build and real Windows testing.
- **NOT YET PROVEN:** real Android screen typing after deployment, authorized agent-originated progressive physical-screen typing, cross-device parity, full dependency-by-dependency removal of dormant historical UI source/launch packaging. Do not mark these PASS, and do not delete shared Rust/Host/Studio dependencies blindly.
