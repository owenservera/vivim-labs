# ARBOR — Proposal 5 — PREG-E Final Freeze Handoff

**Lane:** E — existing Host-served mobile PWA and internal TierFlow view/runtime
**Recorded:** 2026-11-12
**Frozen worker branch:** `work/p5-w01-e-ui`
**Exact verified pre-report HEAD:** `4ed530f1702e9ce3f592e2ae1a7080fa36e5f93d` (tree `2c5dec3f74a77b0c657f653001ce04f99f14ff1c`)
**Final frozen HEAD:** The GitHub tip of `work/p5-w01-e-ui` **after this report-only fast-forward commit**. The exact resulting commit SHA is recorded in the committing worker's final FREEZE_READY response; this file intentionally does not attempt the impossible SHA self-reference.
**Status:** **FREEZE_READY for G source input**; not integrated and not accepted for runtime/security/device release.
**Classifications:** `SOURCE_IMPORTED` (W01 selected donor source), `SOURCE_PATCHED` (isolated W02 browser compatibility source), `BUILD_UNRUN`, `DEVICE_UNRUN`. `HOST_WIRED` = NO; `SECURITY_ACCEPTED` = NO.

## 1. Source and branch authority: exact provenance

- Destination repository: `owenservera/ARBOR`. Original common ancestor: `dd245d78ff3c110ba8630cad557ea0479c51a635` on `architecture/proposal5-beta-first-centralization`, **not** a W0-G integration baseline. The architecture branch subsequently advanced independently.
- Pinned donor 1: `owenservera/vivim-web-harness` main `cae97afe848d79957c9d6c698a90996a758bfd56`; `apps/mobile-relay/` source tree `2d78344af74f17ce8a3f16da7c5352112fee7356`.
- Pinned donor 2: `owenservera/tierflow` main `50822f11ee63c3d80f631f8fea7dcb9b19bffc38`; selected subtree SHA pins: `src/ 701f299cddb358f4083e203820c5bb64395fcbd9`, `tests/ a219a6c20c6b88db2351949e8af63c17c955f10f`, `web/ 9652f0752ce9db04890a17231df043c93135724c`, `runtime/ 503176cb13d994a16bcf2c4243521d2b2c361497`.
- Original physical import **source-only** commit: `9a80e2d0dd09822f274a7b040b173e75b3eb4045`; W01 report commit/HEAD: `1d2fb4fc55784ca74dfeae416129b8b76d5c8216`. Original migration comprised **26 sequential commits** after the common ancestor, including the final W01 report. The original W01 report provides all source-selection decisions: `integration/worker-reports/W01-E.md`.
- Follow-up prior to this final report: `4ed530f1702e9ce3f592e2ae1a7080fa36e5f93d`; **15 sequential focused follow-up commits**, no merge/rebase, root edits or foreign owner modifications. Detailed compatibility work: `integration/worker-reports/W02-E-PREINTEGRATION.md`.
- Synchronization gate: `integrate/proposal5-beta-r001` remains **404 Not Found**, checked again at freeze. No `work/p5-w02-e-ui` branch was created. Master G alone owns creation and convergence of the integrated branch.

## 2. Source inventory — Git tree/blob evidence, not inferred from capped compare

An uncapped, **non-truncated recursive Git tree** of the W02 pre-report HEAD was compared directly against the exact donor subtree trees. That check returned **91** files under `apps/mobile-relay/` and **183** under `packages/tierflow/` (**274** selected source/package files currently on E's isolated branch).

| Source selection | Donor count | W01 imported | Matching original donor blobs at W02 tip | Deliberate exclusion/adaptation |
| --- | ---: | ---: | ---: | --- |
| Harness `apps/mobile-relay/` -> same path | 87 | **87/87** | **83/87** | Four original files adapted by W02: `app.js`, `device-enrollment.mjs`, `sw.js`, `test/service-worker.test.mjs`. All 87 donor paths remain present. |
| TierFlow `src/` -> `packages/tierflow/src/` | 86 | **85/86** | **85/85 selected** | Excluded `src/adapters/hq/sync-bus.ts` (donor checkout, external HQ subprocess/root override). |
| TierFlow `tests/` -> `packages/tierflow/tests/` | 80 | **80/80** | **80/80** | All imported unchanged, including 52 visual runtime tests. |
| TierFlow `web/` -> `packages/tierflow/web/` | 12 | **12/12** | **12/12** | Retained renderer/browser assets, *not* a second deployed app. |
| TierFlow `runtime/` -> `packages/tierflow/runtime/` | 4 | **2 inert JSON defaults** | **2/2 selected** | Excluded executable `runtime/programs/build.program.ts`, `health.program.ts`. |
| TierFlow root/package/tooling | selected files | 4 files | 2 exact source blobs | `README.md` and `tsconfig.json` exact; `package.json` and `scripts/generate-host-tokens.mjs` internally adapted. |

**W01 imported selection: 270 files = 87 mobile + 183 TierFlow; at W01, 268 were exact donor blobs and two TierFlow package/tool files intentionally adapted.** W02 adds four new mobile files (below), retaining the 270 existing paths. Present W02 donor-identity count = 264 original unchanged blobs (83 mobile + 85 source + 80 tests + 12 web + 2 safe runtime + 2 TierFlow root files), with six adapted selected files (4 mobile + TierFlow package and generator).

**Four NEW W02 mobile files:** `apps/mobile-relay/browser-device-key.mjs`, `apps/mobile-relay/test/browser-device-key.test.mjs`, `apps/mobile-relay/test/browser-enrollment-flow.test.mjs`, `apps/mobile-relay/test/host-static-allowlist-contract.test.mjs`. The latter intentionally skips its Host assertion on isolated E when the B Host subtree does not exist; after G merges B, it must stop skipping and verify static readiness.

**Explicit other exclusions:** TierFlow native Rust `core/` is A-owned; stand-alone server `src/server.ts` is source-preserved but not launched; executable programs are excluded; donor-dependent `test/generate-host-tokens.test.mjs` excluded; unrelated Labs/campaign docs, benchmarks, archives, native app and outside-repo HQ dependencies not imported. Three original WOFF2 font binary blobs, PWA shell/manifest/service worker, NLCL, Mobile Control/Composer, Today/World/Me, media-facing source and revision guards remain present. No browser profile, credential, user data store, build cache or other runtime secrets imported.

**Internal packaging decisions already present:** TierFlow package is private/non-launching (no `start`, `dev`, `repl`, independent server or native launcher). Host CSS token generator defaults to in-ARBOR `apps/vivim-host/public/vivim-tokens.css` rather than a sibling donor repo. G owns root Bun/Cargo workspace, dependency/lock and Windows packaging; E did not touch them.

## 3. Complete W02 focused follow-up commit ledger (15, chronological)

All commits are on this **same** E branch and are ancestors of the verified pre-report HEAD:

| # | Commit SHA | Actual change |
| ---: | --- | --- |
| 01 | `9b9a0450f5a36aa1f7dcbe1df75a07ae24cb6237` | New nonextractable browser P-256 signer |
| 02 | `c43bbb746721b1e9ce97606eefc2dbbe0f7ba993` | Direct enrollment device-signer selection; native preserved |
| 03 | `50ef727263d219f7b21f522475d54b57e058f3f2` | Existing PWA QR startup browser preflight |
| 04 | `4dea0b5ddd56ffa18a6b0dc18f848a5ade05bae7` | Nonsecret service worker asset list/version and test |
| 05 | `d5768821713abc0bbf7745e3639f50b2c8aba0e6` | Browser crypto/replay/negative fixtures |
| 06 | `006104d931a241e515c81380636a70cf27b0de8f` | Align ECDSA transcript delimiter with existing Host literal backslash-n bytes |
| 07 | `09fd544418609c6c969dbb4f72e48657efae8feb` | Browser grant/session/mock Host lifecycle tests |
| 08 | `0b05b643d08a9bd844cb22b9a541c850142291e7` | Missing saved browser key must not silently regenerate identity |
| 09 | `d9e4d332ed7545d6038b620dd251e0b9aaeaccb7` | Lost-key resume negative test |
| 10 | `0140e101353a889f15b157a4887614aa9267c401` | Guarded in-ARBOR Host static allowlist contract test |
| 11 | `d3cb8f12240615dcb1fe7be3cdbccb27715a86d1` | B-owned Host one-line static patch evidence |
| 12 | `3f573ed4a488c94664f57acb08b26f45d0f3e5e2` | Correct patch hunk for B handoff |
| 13 | `c917d77f16d26121acc81a7018a8bd17fd49c1f2` | Browser same-origin CryptoKey threat boundary documentation |
| 14 | `3b15bc3043d250e51385b696ab2884b1f6d64769` | Full W02 compatibility/security/TierFlow handoff report |
| 15 | `4ed530f1702e9ce3f592e2ae1a7080fa36e5f93d` | Report formatting; source-free |

**Changed W02 paths, exactly 10 (from GitHub compare):** `apps/mobile-relay/app.js`, `browser-device-key.mjs`, `device-enrollment.mjs`, `sw.js`, `test/browser-device-key.test.mjs`, `test/browser-enrollment-flow.test.mjs`, `test/host-static-allowlist-contract.test.mjs`, `test/service-worker.test.mjs`, `integration/worker-reports/W02-E-B-HOST-STATIC.patch`, `integration/worker-reports/W02-E-PREINTEGRATION.md`. This final commit adds only `integration/worker-reports/PREG-E.md`.

## 4. Host security/compatibility contract and ownership

**E source implementation, not live device acceptance:** ordinary HTTPS Direct browser PWA selects WebCrypto P-256 ECDSA and persists a private `CryptoKey` with `extractable=false` in IndexedDB. Original Android `VivimDeviceKey` WebMessagePort path remains selected when available; broken/partial native bridges never downgrade to browser signing. Capability and storage preflight occurs before spending the one-use QR token. Missing existing browser private key causes a hard refusal, not an anonymous key rotation. `publicKeyPem()` emits SPKI; browser raw P1363 signatures are DER encoded for Host Node `crypto.verify('sha256',...)`. The signer, enrollment client and donor Host transcript source were examined: `fields.join('\\n')` is the **literal backslash+n** separator used by the existing Host, rather than a newline character. Client persists only grant ID/origin/user/signer kind; session bearer is in memory.

**Host B alone owns grant authority:** `POST /mobile/api/pair` one-use challenge; `POST /mobile/api/enrollment/prove` canonical `vivim-enrollment-v1` signature creates *pending* grant; PC-only explicit `approveGrant` narrows approved scopes; `GET /mobile/api/enrollment/grant-status?id=` tracks activation/revocation; `POST /mobile/api/enrollment/session` binds `vivim-session-v1` signature to grant/nonce/timestamp/origin/user and issues short-lived session (current Host max four hours); `validateGrant` authorizes scoped `x-vivim-grant-id` + `x-vivim-device-session`. E did not alter B's signer verifier, pending grant store, revoke/approve state, or session authority.

**Verified static Host integration gap:** in B's latest inspected `work/p5-w01-b-host` branch (observed HEAD `78a5105a9c1f1880d658f350e130b1e2632d3532`) `apps/vivim-host/direct-phone.mjs` still lacks the `'browser-device-key.mjs': ['browser-device-key.mjs', 'text/javascript; charset=utf-8']` asset allowlist entry. E's exact one-line patch is **not applied**; it is committed as `integration/worker-reports/W02-E-B-HOST-STATIC.patch`. B/G must apply to in-repo Host before the module can load at `GET /mobile/browser-device-key.mjs`. Auditing all 45 service worker shell entries against the donor Host asset list found this to be the only newly missing shell path. Live HTTP validation remains UNRUN.

**B/G/F P0 session replay blocker:** current Host checks bearer ID equality, active grant, revoke/expiry and scope; it does not use a fresh per-request private-key signature to bind every privileged HTTP request. Theft of **both active grant and session headers** can replay until current session expiry, replacement or revoke. Stealing only a grant ID is insufficient to mint a new signed session. B must own a reviewed request-bound proof (nonce/method/path/body digest or equivalent) in its existing validator; E must later match the agreed client contract. No second device authority or speculative patch is authorized. Shorter session TTL alone is not a proof-of-possession fix. **Classification: SECURITY_BLOCKER — unresolved, not accepted.**

**PWA Direct/cloud boundary:** Existing Host `/mobile/connection.json` source claims `direct=true, directOnly=true, cloudEnabled=false`, and Direct transport uses authenticated same-origin `/mobile/api/relay?path=...`. The installed beta must not advertise or fallback to retained legacy Vercel/cloud relay. PWA service worker caches only approved shell GET files, not device grants, API responses, session bearer, signed proofs, mutations or cross-origin traffic. HTTPS/Tailscale secure origin and authenticated server remain required; a nonextractable browser CryptoKey is not guaranteed hardware-backed and same-origin script compromise remains a threat.

## 5. Visual runtime / host-owned effects

TierFlow's imported visual compiler, typed document schema `vivim.ui-document/1`, document/source/revision contracts, `renderWave3Semantics`, `createWave4HostStatusContract`, `projectWave4DesktopAdapter` and `projectWave4PhoneAdapter` remain source-only, inert and not live-mounted. Existing mobile `generative-ui/host-revision-guard.mjs` refuses rollback and conflicting same-revision fingerprints. TierFlow adapter must receive authenticated Host actor/grant/policy/document observations; untrusted UI state cannot synthesize authorization, World truth, plan receipts or grant state. Existing Host owns the *only* publisher, effect journal and action dispatcher; TierFlow does not execute actions, publish independently, or start a second server. No R007/new app builder is activated. **Classification: SOURCE_IMPORTED / HOST_ADAPTER_UNWIRED**.

Host Control continues to own `/mobile/api/control/{catalog,language,world,lexicon,recent,plan,execute,plans/ctl-*}` and `vivim.control-receipt/1` `applied|failed|uncertain` receipts, plan digest and no automatic retry on uncertain effects. A TierFlow visual control is inert or an existing Host-routed action candidate, never its own authority.

## 6. Validation and honest execution ledger

**Directly observed in this finalization through connected GitHub:**

| Verification | Actual command/tool | Result | Exit code |
| --- | --- | --- | --- |
| W01+W02 HEAD and exact commit ancestry | GitHub `GET /branches/work/p5-w01-e-ui`, `GET /compare/dd245d...1d2fb4f` and `GET /compare/1d2fb4f...4ed530f` | **PASS**: 26 original commits, 15 follow-up commits; branch HEAD verified | N/A — API reads |
| Uncapped donor vs imported source identity | GitHub `GET /git/trees/<pin>?recursive=1` for all five source subtrees plus destination; path-to-Git-blob SHA equality | **PASS**, exact counts and four mobile source adaptations above; GitHub returned `truncated=false` | N/A — API reads |
| Browser signer + original Host protocol source comparison | GitHub `fetch_file` on browser/device-enrollment and B Host; inspect DER conversion, `\\n` transcript, grant and session routes | **PASS source inspection only**; NOT a cryptographic runtime acceptance claim | N/A — source inspection |
| B Host static asset allowlist | GitHub `fetch_file` on B's `apps/vivim-host/direct-phone.mjs` | **BLOCKER CONFIRMED**: signer module missing from allowlist | N/A — source inspection |
| Shared G integration branch | GitHub `GET /branches/integrate/proposal5-beta-r001` | **ABSENT (HTTP 404)** | HTTP 404, not a test process exit |
| CI status read | GitHub `get_commit_combined_status` for pre-report HEAD | **INACCESSIBLE (HTTP 403)**; no CI result asserted | HTTP 403, not a test process exit |

**Prior W02 worker report — self-reported, raw exit codes not present in this freeze evidence:** `node --check /mnt/data/arbor_e_exact/apps/mobile-relay/browser-device-key.mjs` and `node --check /mnt/data/arbor_e_patch/device-enrollment.mjs` described as PASS; `node --test /mnt/data/arbor_e_exact/apps/mobile-relay/test/browser-device-key.test.mjs /mnt/data/arbor_e_exact/apps/mobile-relay/test/browser-enrollment-flow.test.mjs` described as **8 passing / 0 failing** on Node 22.16.0. The report stated the two executed runtime implementations matched Git blobs, while the temporary tests had locally adjusted relative imports. At freeze, the report itself is present, but **the raw Node process transcript, per-command exit codes, and GitHub CI logs are not attached/independently observed here**; record as `SELF_REPORTED_LOCAL_PASS / EXIT_CODE_NOT_RECORDED`, **not an independently verified ARBOR checkout run**. No test command was newly executed in this final freeze; do not upgrade gate status from those assertions.

**Explicit UNRUN / not accepted**:
- `node --test apps/mobile-relay/test/*.test.mjs` against the complete combined ARBOR repo.
- `bun test packages/tierflow/tests/visual-runtime`; TierFlow legacy tests and `bun run --cwd packages/tierflow typecheck`.
- G root Bun/TS/Cargo build/lock validation, Windows/Tauri packaging and no-external-donor smoke.
- Live B Host route `GET /mobile/browser-device-key.mjs`, actual Host proof/approve/grant-status/session/revoke/control receipts.
- Real browser HTTPS WebCrypto/IndexedDB and key persistence; copied active-session bearer replay negative test.
- Physical phone/PWA pairing/revocation/reconnect, mobile navigation/Composer/NLCL/Control/Mission/media/offline shell; TierFlow on desktop/phone.
- Single-publisher live runtime assurance and complete S2 security/device acceptance.

**Do not claim** `BUILD_PASS`, `HOST_WIRED`, `SECURITY_PASS`, `DEVICE_ACCEPTED`, `G08/G09/B04/B09 PASS` or completed release.

## 7. Exact frozen branch handoff to sole G integrator

1. Treat E worker source as frozen at the **post-PREG-E commit branch tip**; use the SHA supplied with this freeze receipt. Do not ask E to continue, cherry-pick a stale intermediate head or reimport donor trees. Original W01 source-only snapshot is `9a80e2d...`; original W01 handoff is `1d2fb4f...`; W02 follow-up changes are `1d2fb4f..4ed530f`; this document-only final commit comes after them.
2. G alone creates `integrate/proposal5-beta-r001` from the latest approved architecture source and merges all **six frozen worker** snapshots. All worker branches share ancestor `dd245d78...`; G must manage root manifests, Bun/Cargo locks, release scripts, CSS token generator and import paths. In particular, keep `apps/mobile-relay/` with its three font blobs, WebCrypto module, PWA manifest, `sw.js` and tests; `packages/tierflow/` private/internal, with no standalone effect writer or UI publisher.
3. G/B apply or route the **exact B-owned** static allowlist patch before claiming Host-served browser PWA runtime readiness. B must decide request-bound active bearer replay protection and E/B must agree matching client request-proof shape, with F negative tests. Browser PWA source presence is not browser device acceptance.
4. B integrates one Host Direct API authority for pairing, grants, 4-hour bounded session, Control plan/receipt, media/status and source-scoped World reads. No cloud fallback, provider APIs, donor Repo services, external HQ subprocess or independent TierFlow server.
5. G coordinates typed Host-to-TierFlow **read-only** adapter, canonical visual document identity/revision crosswalk, existing Host publisher singleton and responsive projection without changing UI shape. Desktop/mobile interaction stays existing; controls remain inert/Host-routed.
6. F runs Node, Bun, typecheck, Host HTTP negative cases, no-donor source check and physical phone/browser/Windows acceptance. S2 remains a release blocker until signed proof and stolen-session behavior are exercised against B's actual Host. A/B native launch S1 is a separate blocker owned outside E.
7. No Phase 2 Labs runtime sources are promoted from this E branch; freeze report is evidence-only. Subsequent implementation belongs to G-authorized post-merge W2 owners.

**Release classifications at freeze:** `SOURCE_IMPORTED` = YES, `SOURCE_PATCHED` = YES, `BUILD_UNRUN` = YES, `DEVICE_UNRUN` = YES, `HOST_WIRED` = NO, `SECURITY_ACCEPTED` = NO. **FREEZE_READY** means E's isolated input is stable for the independent PRE-G referee and G, not that the product beta has passed.
