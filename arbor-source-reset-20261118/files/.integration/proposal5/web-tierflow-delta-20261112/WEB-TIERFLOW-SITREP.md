# ARBOR Proposal 5 — Web UI, TierFlow and Cross-Surface Migration Delta

**Audit date:** 2026-11-12. **Snapshot:** ARBOR `main@f9823a5131e73e7562db5a4ec14e1ec481557d69`.  
**Disposition:** **SOURCE_CONSOLIDATED / RUNTIME_UNVERIFIED / SECURITY_BLOCKED / DEVICE_UNRUN / BETA_NO_GO**.  
**Scope:** Web UI, existing Host UI, mobile PWA, Studio browser surfaces, internal TierFlow visual runtime, their source-to-consumer boundaries and original-vs-ARBOR source deltas. This is a **read-only source audit committed as documentation**, not an implementation, test execution, merge, release or device certification.

## 0. Executive answer

ARBOR now contains substantial, source-preserved original UI and TierFlow code on canonical `main`. It does **not** yet establish that the UI is one coherent, physically accepted runtime. The decisive unfinished job is to connect the existing **Host publisher / personal World / typed action authority** to **a single TierFlow inert visual projection** and **one Host-served phone PWA**, without introducing a second publisher, Store, graph, action catalog, provider sender, scheduler or executor. The original TierFlow independent server/assets were intentionally imported as dormant source/fixtures; those are not the customer UI. The native desktop and phone packaging/security prerequisites still block release.

**Highest-impact new delta identified by this audit:** all **269/269 donor files in `studio/src/`** are present byte-identically, but **none of the 305 original `studio/test/` files** are in ARBOR main. Both `studio/package.json` and root `package.json` still point at a Studio test suite under `test/unit/**`. This is an executable-evidence/source-closure problem, not a claim that 579 Studio runtime modules are missing.

**Do not infer these from source:** that the current Tauri executable launches, that any TierFlow DOM is mounted into the running Host, that a signed-in Chrome provider receives one exact message, that phone enrollment/revocation is secure, that a physical PWA renders the same World/revision, or that a Windows installer exists. The committed beta ledger still says `BLOCKED_NOT_BETA`.

## 1. Source pins, original contract and method

| Reference | Exact immutable commit |
|---|---|
| [ARBOR main](https://github.com/owenservera/ARBOR/tree/f9823a5131e73e7562db5a4ec14e1ec481557d69) | `f9823a5131e73e7562db5a4ec14e1ec481557d69` |
| [Harness donor main](https://github.com/owenservera/vivim-web-harness/tree/cae97afe848d79957c9d6c698a90996a758bfd56) | `cae97afe848d79957c9d6c698a90996a758bfd56` |
| [TierFlow donor main](https://github.com/owenservera/tierflow/tree/50822f11ee63c3d80f631f8fea7dcb9b19bffc38) | `50822f11ee63c3d80f631f8fea7dcb9b19bffc38` |
| [Labs main](https://github.com/owenservera/vivim-labs/tree/022608f5797e7b4a14ccc4c393bd3c5a4704a4be) | `022608f5797e7b4a14ccc4c393bd3c5a4704a4be` |

The Harness/TierFlow donor pins match those in [W01-E](https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/integration/worker-reports/W01-E.md); this makes the comparison unusually strong: it compares against the **actual frozen import inputs**, not merely a vaguely “current” original.

Primary original requirements are [Proposal 5 README](../README.md), [03 capability placement](../03-CAPABILITY-SPACES-AND-MIGRATION.md), [05 beta journeys](../05-FULL-BETA-RELEASE-CONTRACT.md), [06 cutover](../06-EXECUTION-AND-CUTOVER.md), [09 execution gates](../09-EXECUTION-PLAYBOOK.md), the original [Harness roadmap](https://github.com/owenservera/vivim-web-harness/blob/cae97afe848d79957c9d6c698a90996a758bfd56/docs/ROADMAP.md) and [TierFlow ownership audit](https://github.com/owenservera/tierflow/blob/50822f11ee63c3d80f631f8fea7dcb9b19bffc38/VIVIM-VISUAL-RUNTIME/MISSION-D-TIERFLOW-CONVERGENCE-AUDIT-2026-11-10.md).

**Method:** GitHub-connected source inspection; recursive Git trees of three repos without truncation; all corresponding original/destination blob SHA matches; package and selected runtime files; original worker reports and source-lock, beta-gate, release-manifest evidence; targeted code-search for TierFlow package consumers. Full 1,522-path reproducible table is [SOURCE-BLOBS.v1.json](SOURCE-BLOBS.v1.json); the five new ARBOR-only mapped paths are recorded separately in [ARBOR-ADDITIONS.v1.json](ARBOR-ADDITIONS.v1.json); semantic capability gap registry is [DELTA-ATLAS.v1.json](DELTA-ATLAS.v1.json). Git object identity is proof of identical bytes, **not proof of live wiring, installability, API behavior or UI quality**.

## 2. Quantitative original → ARBOR source delta

These are donor **file counts**, not features or release tests. `NOT_IMPORTED` can be deliberately out of scope.

| Original donor mapped subtree | Original files | Identical blobs | Adapted present | Not imported |
|---|---:|---:|---:|---:|
| Harness `apps/mobile-relay/` | 87 | 83 | 4 | 0 |
| Harness `apps/vivim-host/` | 137 | 134 | 1 | 2 |
| Harness `studio/` | 958 | 379 | 0 | 579 |
| Harness `harness/` | 132 | 122 | 0 | 10 |
| Harness `src-tauri/` | 14 | 12 | 1 | 1 |
| Harness `core/programmability/` | 7 | 7 | 0 | 0 |
| TierFlow `src/` | 86 | 85 | 0 | 1 |
| TierFlow `tests/` | 80 | 80 | 0 | 0 |
| TierFlow `web/` | 12 | 12 | 0 | 0 |
| TierFlow `runtime/` | 4 | 2 | 0 | 2 |
| TierFlow selected package/tool files | 5 | 2 | 2 | 1 |
| **Total mapped donor files** | **1,522** | **918** | **8** | **596** |

ARBOR also includes **new** files in selected source directories (for example browser PWA signing source/tests); the above table is deliberately directional *donor → destination* and does not count additions as missing donor files. Exact path, source/destination SHA and category for every entry are in the JSON inventory. These totals are **not** an assertion that ARBOR is missing 596 product source files.

### 2.1 What those 579 omitted Studio files actually mean

All `studio/src/` source files: **269/269 imported, 269 byte-identical**. All `studio/scripts/`: **41/41 imported**. The 579 omitted paths include **305 under `studio/test/`**, **166 under `studio/docs/`**, **87 under `studio/android/`**, and **11 under `studio/.opencode/`**, plus other non-core items. The entire original `studio/test` suite is thus absent even though the imported `studio/package.json` and ARBOR root test script still reference it.

**Decision:** preserve the full operational Studio implementation but restore a reviewed *nonsecret, non-device-destructive* test dependency closure. Avoid importing historical Android APK code merely to satisfy parity; current policy is PWA first. Use the [machine blob ledger](SOURCE-BLOBS.v1.json) to identify each omitted test and distinguish reviewable unit fixtures from device E2E, logs or unsafe artifacts.

### 2.2 Intentional omissions and source changes

- **PWA:** four adapted donor files (`app.js`, `device-enrollment.mjs`, `sw.js`, `test/service-worker.test.mjs`). Four additional browser-key/contract tests were added in ARBOR, as reported by W02-E. Nonextractable WebCrypto signer in `browser-device-key.mjs` is source-present.
- **Host:** `direct-phone.mjs` adapted to serve the browser signer. Two missing donor `.tap` outputs are historical test receipts, not runtime source.
- **TierFlow:** original `src/adapters/hq/sync-bus.ts` intentionally removed because it could spawn a **separate repos-hq checkout** from hardcoded / overridden paths; runtime executable programs intentionally withheld to avoid a second effect executor. All 80 selected original tests and 12 browser assets retained. `package.json` adapted to remove `start/dev/repl` product launchers; `generate-host-tokens.mjs` adapted to write within ARBOR.
- **Tauri:** mandatory APK staging removed from `tauri.conf.json`; donor `vivim.apk` not imported. This matches PWA-first policy, **not** proof that MSI packaging works.
- **Harness docs:** ten selected documentation/example paths not imported; code/test tree equality is much stronger than those doc omissions.
- **New ARBOR status documents:** root workspaces, security reports and several Phase 2/HVI proposal folders were authored after donor import; they are planning/evidence, not a hidden source implementation.

## 3. Real ownership map: four UIs, one product

| Layer | Present in ARBOR | Actual role and unresolved question |
|---|---|---|
| Windows Host desktop | `apps/vivim-host/public/index.html`, `host.js`, `control-panel.js`, `src-tauri/` | Owner authority, control and status UI exists. Is **packaged** launch authenticated to the actual Host child? No accepted proof; documented native fixed-port/owner-bearer defect. |
| Phone PWA | `apps/mobile-relay/index.html`, `app.js`, `ux/model.mjs`, `control-chat.mjs`, `generative-ui/*` | Source for the existing six tabs and Host-backed Build/World. Is real Android Chrome paired, revoke-safe, nonce-safe and parity-accepted? No accepted proof. |
| Studio / signed-in webapp UI | `studio/src/studio/ui/*`, `studio/src/studio/chat-channel.mjs`, `harness/` | Provider UI/capture and signed-in user Chrome channel. No live authenticated exact-thread two-turn acceptance; all original Studio test suite paths omitted. |
| TierFlow internal visual library | `packages/tierflow/src/visual-runtime/*`, `src/ui/*`, `web/*` | Strong retained typed visual, render, revision, command, World bridge and dry-run logic. Pure/inert library **not verified mounted** in desktop/phone product. Its independent `src/server.ts` source must not become a second VIVIM process. |

**Canonical rules unchanged:** Host alone owns action catalog, grants, durable effect claims, receipts and World authority. Studio controls the single signed-in browser sender. TierFlow maps/proposes/renders, never authorizes effect or creates an unreviewed second UI publisher. User browser, phone and external signed-in websites are legitimate external clients; *donor project checkouts or services are not*.

## 4. UI feature-by-feature delta against original intent

### Desktop, phone and semantic control

1. **Today/Talk/Build/World/Me/Control:** `apps/mobile-relay/ux/model.mjs` defines exactly those six tabs; `app.js` imports Host-derived models and command chat. **SOURCE_PRESENT**; physical navigation, keyboard, thumb reach, IME, focus, screen reader, refresh and same-revision rendering **UNRUN**.
2. **Text as universal controller:** Host command form, approval UI and receipts source are present in `apps/vivim-host/public/index.html` and `control-panel.js`; phone has `control-chat.mjs` and Host dispatcher. **SOURCE_PRESENT**, but there is no accepted cross-surface single-control contract test run against integrated ARBOR.
3. **Build and generative UI:** original phone has `generative-ui/{compiler,design-composer,renderer,phone-surface}.mjs` and `build-canvas.mjs`; source `phone-surface.mjs` guards Host-committed revisions. **SOURCE_PRESENT**. This is **not** evidence of a complete general full-stack app generator, nor proof that TierFlow is now the rendering implementation for the Build tab.
4. **Studio web UI:** original `studio/src/studio/ui` source landed, including platform/chat/mobile views. No claim of a single unified desktop UI navigation/publisher or independently accepted live provider send is supported; Studio test closure is a P0 verification repair.
5. **Visual consistency:** tokens and fonts exist across Host/mobile/TierFlow; `generate-host-tokens.mjs` now targets ARBOR. Need actual rendered screenshots/DOM/a11y, revision and color-theme consistency from the shipped app, not merely CSS equality.

### TierFlow source versus *operational* TierFlow

6. **Document grammar:** `visual-runtime/document/*` canonical schemas, validators and legacy importer preserved. Good seam; **not** equivalent to Host personal World/Build shell schema.
7. **Rendering:** `render/dom.ts`, `render/wave3-semantics.ts` and components exist. Outputs are semantic/pure render plans in source/fixtures. **No confirmed shipping DOM mount** through Host or PWA.
8. **Visual commands:** `command/{ir,compile,crosswalk,...}` exists as inert candidate compilation. The action vocabulary/risk must be bound to **Host catalog IDs** at runtime; typed interpretation itself grants zero effects.
9. **World→visual bridge:** `final-wave/integration/world-visual-bridge.ts` fails closed on unsafe shapes, source digest/scope mismatch and stale revisions. It does not open Host storage, publish, own data or execute. A **trusted Host read snapshot adapter** and real revision/identity propagation are still needed.
10. **Device parity:** `wave4-desktop-adapter.ts`, `wave4-phone-adapter.ts`, `integration-4/compose.ts` and `final-wave/integration/master.ts` retain actual composition and refusal source; their own comments explicitly disclaim device acceptance. Prior independent TierFlow audit recorded a renderer/root guard mismatch; rerun exact in-ARBOR tests before attributing repair.
11. **Cutover:** `final-wave/lane-g/cutover-dry-run.ts` is source for rehearsals, not a performed single-publisher production cutover. Keep Host publisher until authenticated TierFlow shadow parity, rollback and owner-approved fence can be measured.
12. **Standalone TierFlow:** old `src/server.ts`/`web/` kept for source fidelity and tests, but adapted internal `package.json` has no `start` script. This is a deliberate **non-import of product process behavior**, not a lost beta feature. External HQ subprocess adapter and Tier-2 job programs are intentional exclusions.

### Security and release barriers which directly affect UI acceptance

13. **PWA browser signer:** `browser-device-key.mjs`, device enrollment and static Host route patch are on main. Signed grant/session headers alone do **not** yet prove request-body binding / fresh nonce / replay rejection. The P0 B+E+F request-proof repair remains.
14. **Tauri→Host bootstrap:** `src-tauri/src/main.rs` still performs fixed loopback health check and transmits durable owner token to `/api/host/control/ui-session`. Without spawned-child attestation, local listener impersonation can compromise control. Treat the UI as **not safe for production owner data** until A+B+F repair.
15. **Direct-only phone mode:** existing `transport.mjs` retains a legacy Cloud Vercel path for other modes. On a Host-served `/mobile/` origin it selects Direct and refuses cloud switching, but **packaged no-donor Direct-only routing** is unverified. Do not promote the Cloud worker to a required ARBOR runtime.
16. **PWA-only Android:** removing the APK resource does not prove physical browser enrollment, WebCrypto/IndexedDB durability, HTTPS Tailscale Serve, service-worker updates, offline-auth refusal or device revocation.
17. **Build/installer:** root `Cargo.lock` and `bun.lock` absent at this pinned main; released Node24 runtime, Windows MSI and no-donor isolation are not evidenced. Recorded CI run `43233248631` had `startup_failure` and zero jobs; this audit **did not retrieve a newer successful CI run**.

## 5. What has *not* been promoted from research lanes

ARBOR main currently has HVI Proposal 5 **documents**, not `hq/version-intelligence/*` production source or a mounted TierFlow version inspector. Isolated branches for HVI S0 and A–F exist, but do not equate existence of those refs with G1 acceptance or main promotion. Likewise PG00–PG03/PGZ, semantic wiki, visual language, N0 I/O, AutoWiki and Phase 2 high-value research have separate preparation paths, not a verified product feature on this pinned main.

**R005 semantic lens:** verified Labs pure semantics are possible candidates for a narrow read-only adapter; not a second Host parser. **R006 predictive UI / R007 VIEW ALL:** higher-level designs / research; do not describe them as operational. **Screen-state AutoWiki / Rock Core / general app builder:** preserve contracts, source/proof lineage and privacy gates, not runtime availability. **PWA-first policy:** no blanket native APK migration required to satisfy this report.

## 6. Delta register and recommended serial repair order

This ordering is a proposed technical triage, **not authorization to overwrite another owner lane**.

| Order | Owner seam | Exact repair / proof | Blocked by |
|---:|---|---|---|
| 0 | **G/F build proof** | Resolve genuine `Cargo.lock` and `bun.lock`, Node24 staged Windows runtime, Rust/Bun/TS tests, no-donor source closure | Current missing locks, CI receipts |
| 1 | **A+B+F native auth** | Attest spawned Host before any durable capability leaves Tauri; strict origins and compatible patched Tauri | Live security P0 |
| 2 | **B+E+F phone auth** | Bind every privileged request to device key, actor/scope, fresh nonce, body digest, session, expiration and replay ledger; verify revoke | Live security P0 |
| 3 | **C+G+F Studio closure** | Restore/curate original 305 missing `studio/test/` files; reconcile root test script; run synthetic and exact signed-in provider tests | Test suite absent |
| 4 | **B+E+F visual read port** | Host-authenticated source snapshot + principal + revision/fingerprint; map Host World/build to TierFlow safe subset | No accepted runtime mount |
| 5 | **E/G/F one publisher** | Inert TierFlow projection mounted in existing desktop and PWA, common IDs/receipts, source freshness, rollback and stale-screen refusals | Cross-schema and prior root/guard mismatch |
| 6 | **B+C+E+G acceptance** | Windows MSI, real browser PWA, Studio provider, refresh/reconnect/uncertain/revoke/ENOSPC/device tests | Earlier repairs |
| 7 | **D/HVI + future UI** | Gated HQ source version inspector, R005 experiments, AutoWiki, advanced builder without authority proliferation | Current beta proof, G1/G2 |

No independent UI publisher, app/phone duplicate, alternate personal database, TierFlow standalone server, provider API or generic plugin executor should be introduced as a shortcut.

## 7. Test and release truth at snapshot

| Check | Evidence | Outcome |
|---|---|---|
| GitHub commit + source tree read / no truncation | Connected GitHub API, anchored refs | **PASS (API, no shell exit)** |
| All mapped donor blobs and destination paths | 1,522 explicit crosswalk entries | **PASS (source-level API comparison)** |
| Current product TierFlow mount reachability | Targeted GitHub code search plus source | **NO VERIFIED PRODUCT CONSUMER** (not exhaustive runtime proof) |
| Studio source-vs-test closure | Git tree and package scripts | **269/269 src identical; 0/305 tests imported** |
| Cargo/Bun install/build/test | No checked-out integrated runner invoked | **UNRUN / no exit** |
| Tauri security and child attestation | Source + W01-G security ledger | **BLOCKED** |
| PWA signer / replay / revocation on real Android | Source + W02-E security ledger | **UNRUN / SECURITY_BLOCKED** |
| Host/TierFlow actual DOM mount and visual parity | No device/browser test | **UNRUN** |
| Signed-in Chrome real provider exact send/reply | No real account test | **UNRUN** |
| Windows MSI, offline/no-donor and rollback | No installed binaries | **UNRUN** |
| All B01–B12 beta journeys | `integration/beta-gates.json` | **NO_GO** |

**No local toolchain tests were executed in this audit.** GitHub reads, source diff and new commits do not have OS test exit codes. See [ACCEPTANCE-PLAN.md](ACCEPTANCE-PLAN.md) for executable gates and negative paths.

## 8. Decisions to preserve and acceptance definition

1. **Do not rewrite or replace UI prematurely.** Keep the existing six-tab PWA, Host owner console and Studio source, while making TierFlow their internal inert visual layer.
2. **Do not confuse preservation with activation.** TierFlow `web` and `src/server.ts` being inside ARBOR does not mean deployed, and keeping TierFlow dormant avoids a second app.
3. **Treat omitted tests as actionable.** Studio is the largest source-vs-verification delta. Recover safe tests without bringing back the native APK or donor runtime dependencies.
4. **One receipt/state language:** every desktop/phone surface must show the same Host action ID, plan digest, command state, source revision and uncertainty. No UI-specific “done” shortcut.
5. **Migration exits:** a real Windows installed Host and authenticated ordinary-browser PWA must work after renaming/removing all donor checkouts/processes, with physical security, schema parity, rollback and measured performance; **until then status remains blocked**.

This audit is intentionally **documentation only**. It does not revise `integration/source-lock.json`, `integration/beta-gates.json`, `integration/release-manifest.json`, frozen worker source, main, or any live release decision. Its machine ledger is a review input for the appropriate sole integrator, **not a competing capability authority**.
