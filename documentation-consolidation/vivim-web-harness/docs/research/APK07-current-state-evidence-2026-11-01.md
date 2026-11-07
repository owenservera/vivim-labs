# APK07 current-state research and evidence register

**Snapshot:** 2026-11-01 research; remote GitHub `owenservera/vivim-web-harness`, `main` SHA `761266c1437fca9b93fcb0fd800d9a7cdd85c54b` (last repository commit observed 2026-10-31 22:22:38 UTC). **Evidence boundary:** remote source and repository documentation + dated uploaded APK research + Android/Tailscale official public documentation. This is not a local process, Windows desktop, Android device, build, or end-to-end acceptance inspection. Local checkout may be newer; the local agent MUST supersede this snapshot with verifiable local evidence.

## Evidence classes

- `S`: current tracked source inspected on remote SHA (code behavior, not live success).
- `T`: checked-in unit tests or recorded test receipts (do not claim tests rerun here).
- `L`: dated *prior* live receipt reported in repo docs, limited to the explicitly observed direction/device.
- `D`: design/PRD/roadmap claim, not implementation proof.
- `X`: current official external platform documentation.
- `U`: not verified or missing critical evidence. Every APK07 P0 item requires current local verification and a machine-readable receipt.

## Source snapshot

Repository metadata: private; default branch `main`; recursive tree read returned 1,162 tracked blobs, `truncated=false`. Selected categories: `studio/android/` 9 blobs, `apps/vivim-host/` 43 blobs, `apps/mobile-relay/` 44 blobs, `docs/` 95 blobs. Repo size and file count change; do not pin as future truth.

| Domain | Source anchors (GitHub prefix `https://github.com/owenservera/vivim-web-harness/blob/main/`) | Finding | Evidence / residual risk |
|---|---|---|---|
| Android build | `studio/android/{AndroidManifest.xml,build.ps1,build.sh,README.md}` | SDK 34, min 26, target 34; Gradle-free aapt2→javac→d8→package→zipalign→apksigner. Both builds default to local auto-generated debug `.keystore`, hardcoded alias/password and output path. Manifest versionCode=1. | S; 2026-10-29 recorded 20.6 KB signature-verified build (T), not installed. No release factory, version monotonicity or reproducible signing verified. |
| Android WebView | `studio/android/src/inc/vivim/app/MainActivity.java` | ~670 lines. Pairing URL persisted in SharedPreferences; confirmation required on incoming link; text/plain share supported, prefilled via URL fragment; WebView same-origin navigation policy; script enabled, file/content access disabled, mixed content denied; debug inspection only if app debuggable; connection timeout and renderer recovery. | S; no device acceptance. No bundled offline UI, device credential enrollment or persistent synced private store demonstrated. |
| Android security configuration | `studio/android/AndroidManifest.xml` | `usesCleartextTraffic=true` app-wide; `allowBackup=false`; INTERNET and ACCESS_NETWORK_STATE; exported launcher, `vivim://pair` and text share filters. | S; migrate to HTTPS-first and narrow exceptions. Review intent spoofing, source validation, deep-link secrets and minimum required permissions. |
| Android onboarding UX | `studio/android/README.md`, `studio/docs/architecture/ANDROID-APP.md`, MainActivity | README updated with real recovery design; older architecture doc contradicts it about debugging/auto intent handling. Pair screen asks for `http://192.168...` LAN URL and same-WiFi, whereas Direct uses `.ts.net` HTTPS; `looksLikeHomeNetwork()` warns about `.ts.net`. | S; high-priority UX/runtime reconciliation, not necessarily functional connectivity blocker. |
| APK QR distribution | `apps/vivim-host/direct-phone.mjs`, `studio/src/studio/lan-pair.mjs` | Direct handler has closed allowed asset list, and no APK download route; QR module is for LAN URL pairing, not a user APK factory/installer page. | S+U; design work required. QR encodes URL, not APK. Original research's QR version 1–2 estimate for 60–90 bytes was inaccurate; actual version depends on exact length and ECC, verify with encoder. |
| Tailscale Direct | `apps/vivim-host/{server,direct-phone}.mjs`, `apps/mobile-relay/transport.mjs` | `/mobile` routed to Direct handler before local Host guard; requires loopback client, exact Host, forwarded HTTPS and exact `tailscale-user-login`. CSP and closed asset allowlist. Direct front-end intentionally prevents cloud fallback. | S. Identity header identifies Tailscale *user* not exact device; no APK-specific approval, device key binding, revocation or session enrollment. |
| Direct live chat | `docs/ARCHITECTURE-CURRENT-AND-TARGET.md`, `docs/agents/HANDOFF-2026-10-31-DIRECT-PHONE.md` | Prior PC-originated Tailscale HTTPS chat requests were recorded as completed on exact ChatGPT thread; first blocked request correctly protected an unsent draft. | L (historical). **Does not prove** Android paint, direct APK install, current browser session or general provider/model support. |
| Android vs Direct handoff | `apps/mobile-relay/transport.mjs`, MainActivity | WebView can load http(s) URL; Direct PWA origin validator accepts `.ts.net` and loopback. Native pairing still accepts broad http(s) addresses, not app-specific trusted `ts.net` requirement. | S; actual first-launch/transport pairing with installed APK unverified. |
| Phone mobile UI | `apps/mobile-relay/{app.js,index.html,bootstrap-phone.mjs,visual-bootstrap.mjs,control-phone.mjs}` | Talk UI, Build mount, provisional visual bootstrap. `app.js` has `FALLBACK` provider list with `signedIn:true`; localStorage stores some talk state. `control-phone.mjs` is **read/plan-only** and explicitly refuses effects. | S; never advertise false provider readiness; mobile text actions are not yet end-to-end execution. LocalStorage not a canonical offline World store. |
| Host typed actions | `apps/vivim-host/control/{action-catalog.v1.json,intent-planner.mjs}`, `docs/design/UNIVERSAL-TEXT-CONTROL-V1.md` | Shared command invariant is decided. Current five-action catalog is aspirational, mostly `adapter_pending`/unavailable; planner returns preview with `executionPerformed:false`; no Host dispatcher or mobile approvals proven. | S+D; **critical** APK07 integration blocker. |
| Host bootstrap | `apps/vivim-host/bootstrap/{ledger,service,runtime,command-worker,owner-auth}.mjs`, `apps/vivim-host/server.mjs` | Trusted owner-only bootstrap functions, SQLite-based revisions, UI CAS and undo, local/Direct separation. Worker/process supervision exists in source. | S; actual active device, Host and worker process topology must be re-inspected locally. |
| GenUI | `apps/mobile-relay/generative-ui/{catalog,compiler,renderer,bootstrap-adapters,semantic-projection,phone-surface}.mjs` | Strict shared declarative UI, bootstrap adapters, revision/proposal flows. `docs/superpowers/specs/2026-10-31-gui07-android-acceptance-spec.md` explicitly requires a real paired Android and signed-in AI provider; repo says 105 related tests, but real GUI07 not accepted. | S+T+D; do not create duplicate renderer/activation authority. Extend GUI07 acceptance with native APK07 gates. |
| Personal World | `studio/src/studio/{world-service,world-transfer,world-search}.mjs`, `docs/architecture/vaws/*`, `dev/prd/06-personal-world-memory-and-context.md` | Studio World source and import/export tests exist, VAWS registry evolution is designed; on-phone full offline replica/sync/permissions unproven. | S+D+U; investigate canonical writer, privacy projection, asset lifetimes and migration before choosing Android DB schema. |
| Context bundles | `docs/architecture/DYNAMIC-CONTEXT-BUNDLES-REVIEW-2026-10-31.md` | Ten logical layers, profiles and disclosure rules designed; new DCB runtime not accepted. | D; APK must use authorized Host-constructed context, not leak local World. |
| Mobile visual behavior | `docs/ux/MOBILE-INTERACTION-SCHEMA-MAPPING-2026-10-30.md`, `docs/ux/schema/*` | 28 proposed semantic interactions, accessibility and touch requirements; proposed does not imply app UI built. | D; attach real phone screenshots/interaction receipts. |
| Control/roadmap authority | `docs/INDEX.md`, `docs/roadmap.json`, `docs/capabilities/{atomic-registry,source-symbol-index}.json` | One canonical roadmap and atomic feature inventory. Source has many items marked `done` while dated docs explicitly limit native acceptance. Avoid a competing roadmap. | S+D; local agent must map APK07 tasks into existing ledger and update evidence qualifiers rather than overwrite. |
| Existing GUI07 | `docs/superpowers/specs/2026-10-31-gui07-android-acceptance-spec.md` | Ten acceptance steps including paired Direct, verified provider, BF00, F02/F01, D03–D07, selected Build edit, CAS+Undo, reconnect and Talk draft regression; explicitly deferred pending real-device proof. | D; extend, do not replace, and avoid confusing GUI07 with broader APK07 product target. |
| Cross-repo possible reuse | `docs/research/GUI02-PRE-PRD-CANVAS-SEMANTIC-LANGUAGE-2026-10-31.md`, references to `vivim-final` and `VOMEGA` | Legacy InfiniteCanvas, mutation schema, NLCL 17-symbol language and semantic correlation are candidates for reuse. | D/source-archeology; local agent must inspect current sibling checkout, dependency/license and maintain single authority. |
| Historical roadmap | `docs/roadmap.json` | `P01` in progress, `DIR01` source pending live, `PACK01` planned, `GUI07` queued, `UTC01`/`UTC02` in progress, `UTC03`–`UTC07` planned. `START01`–`START03` listed done despite acceptance caveats; inspect latest receipts. | D; statuses are not real-device proof. |

## Important conflicts requiring local adjudication

1. **Distribution policy**: old APK research recommends simple debug APK + QR first. New user decision requires *full custom APK07 on Day One*, including release-safe signing, update continuity and independent local capability. Supersede old sequencing while reusing existing assets.
2. **Global vs user-owned signer**: user-specific package names and keys maximize ownership, but create developer-verification and update burdens. Need dual profile from same codebase; do not silently treat every user-generated key as Android-verified.
3. **Offline truth**: current WebView-only APK with no bundled web assets cannot satisfy a meaningful offline Personal World, even if the PWA caches pages. Audit current service workers and storage, then design explicit local snapshot/version/migration semantics.
4. **Trusted device identity**: `Tailscale-User-Login` plus same-tailnet access is not equivalent to per-device enrollment/approval. Never encode permanent credentials in public QR or compilation variables.
5. **HTTP pairing**: app-wide cleartext and LAN URLs are transitional; Direct-first should use strict HTTPS origins and intentionally isolated LAN compatibility with visible warning.
6. **Browser owner**: do not let custom APK builds, Android UI or independent phone agents instantiate a second browser driver; Studio existing owner remains exclusive.
7. **`done` vs acceptance**: source presence, tests, CI, PC-originated HTTP, actual Android UI and complete user journey are separate evidence levels.
8. **Product full-day-one vs code implementation order**: all APK07 features must pass one release gate; internal dependency steps may still be sequenced. Reuse GUI07, UTC, BF and World tasks instead of duplicating effects.

## External verification (official sources, checked 2026-11-01)

- Android developer verification rollout: https://developer.android.com/developer-verification/guides and https://developer.android.com/developer-verification/guides/faq . Selected participating stores/regions started September 30, 2026; direct sideloading was not universally blocked on that date; broader 2027 rollout requires reassessment. Advanced sideload path and optional limited-distribution account have different requirements.
- Android app signing and update identity: https://developer.android.com/studio/publish/app-signing . Upgrade generally needs compatible package/signing lineage; key loss/rotation and Play distribution paths differ from direct APKs.
- Tailscale Serve identity headers and trusted loopback proxy: https://tailscale.com/docs/features/tailscale-serve . Headers refer to a user, may be absent for tagged devices, and are distinct from VIVIM device keys/grants; Serve is not Funnel.
- Android WebView unsafe file inclusion: https://developer.android.com/privacy-and-security/risks/webview-unsafe-file-inclusion . Restrict JavaScript bridge, file URLs, origin and access; locally packaged pages should use a same-origin-safe HTTPS asset-loader strategy, not `file://` shortcuts.
- Offline packaged web content: https://developer.android.com/develop/ui/views/layout/webapps/load-local-content . Bundled UI needs versioning independent of the Host website.
- Android notification permission: https://developer.android.com/develop/ui/compose/notifications/notification-permission . API 33+ non-exempt notifications need runtime permission.
- Android Storage Access Framework: https://developer.android.com/training/data-storage/shared/documents-files . User-visible archive import/export can use system-owned document pickers.
- Android 16 KB page size: https://developer.android.com/guide/practices/page-sizes . Relevant if APK adds native `.so` dependencies; current Java-only shell may not be affected.

## High-priority source revalidation commands for the local agent

```powershell
Set-Location 'C:\0-BlackBoxProject-0\vivim-web-harness'
git status --short
git rev-parse HEAD
git log -8 --oneline
rg -n "APK|Android|GUI07|UTC0|P01|DIR01|PACK01" docs/roadmap.json docs/INDEX.md docs -g '*.md'
rg -n "WebView|SharedPreferences|usesCleartextTraffic|versionCode|apksigner|keystore" studio/android
rg -n "tailscale-user-login|/mobile|api/control|connection.json" apps/vivim-host apps/mobile-relay
rg -n "signedIn: true|FALLBACK|executionPerformed|apply-proposal|uncertain" apps/mobile-relay apps/vivim-host
```

Read `docs/INDEX.md` and `AGENTS.md` first. Do not execute a live browser send to probe capabilities; prefer existing receipts and safe read-only inspection. Do not print QR secrets, owner tokens, raw messages or personal data into logs.
