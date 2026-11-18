# QUEUE-CONTRACT — PWA HOST TRUST BOUNDARY

Priority frame: **the Host-served PWA is the product surface; fast safe local iteration of it outranks everything else.**
Base SHA `452c82eb48db6fa5f95b47a778d664723359bb63` (main). 2026-11-13. Read-only lane; one write = this file. No git mutation.
Evidence commands: `Select-String`/`read` over `apps/vivim-host/{server.mjs,direct-phone.mjs,control/,public/}`, `apps/mobile-relay/`; `node --test apps/vivim-host/test/control-http.test.mjs` → **1/1 pass, 4157 ms**. Nothing else executed.

## 1. PWA HOST TRUST BOUNDARY — current state

**Verdict: local PWA iteration is impossible today — VIOLATED at the front door, by design, not by bug. The PWA also cannot be reached by an owner without an enrolled device, and every read route proves a device, not a person.**

Three independent blockers, all needed to open `/mobile/`:

1. Mount is env-gated: handler exists only when `VIVIM_DIRECT_ORIGIN` and `VIVIM_DIRECT_USER` are both set (`server.mjs:851-852`), attached at `server.mjs:229` (`direct && rawPath.startsWith('/mobile/')`).
2. Construction requires an **exact `https:`** origin: `if (url.protocol !== 'https:' || url.origin !== origin …) throw` (`direct-phone.mjs:82-83`). A loopback `http://127.0.0.1:43187` throws before the route table exists.
3. Per-request gate requires all four of loopback source, `host === url.host`, `x-forwarded-proto: https`, `tailscale-user-login === user` (`direct-phone.mjs:151-152`) — i.e. a Tailscale Serve proxy.

The client half is already local-ready: `transport.mjs:12` accepts `loopback && (http:||https:)`, `:22` sets direct mode from `directOrigin(location.origin) && pathname.startsWith('/mobile/')`. Only the server refuses. There is no `VIVIM_DIRECT_LOCAL`, no dev branch, no loopback exception anywhere in `direct-phone.mjs`.

The working local owner surface is the **Host Control UI, not the PWA**: `control/cli.mjs:31-37` → `http://127.0.0.1:43187/#control-session=<64hex>` (single-use, 60 s) → `public/control-panel.js:103-107` → `POST /api/host/control/ui-exchange` (`server.mjs:571-584`) → HttpOnly `SameSite=Strict` cookie + CSRF; mutations additionally need `x-vivim-ui-csrf` (`control/browser-session.mjs:52-57`). Nothing in it serves PWA code.

**Minimum authority that would be needed if the mount were opened:** one active **device grant + device session** carrying `control:read`. `authorizeDevice` (`direct-phone.mjs:141-148`) 401s without `x-vivim-grant-id` + `x-vivim-device-session` (`:144`), 403s on scope (`:146`). Reads are not public; there is no read-only unauthenticated floor. `transport.mjs:61-65` mirrors that: any request without a live session throws 401 client-side.

### Route table — every route the PWA can call (`direct-phone.mjs`)

Global check `:151-152` applies to all rows and is omitted from the column.

| Route (under `/mobile/`) | Method | Class | Admission check | Verdict |
|---|---|---|---|---|
| `api/live/ack` `:158-186` | POST | **effect** (records phone delivery receipt) | `origin===origin` + `x-vivim-ack:1` `:162` + `authorizeDevice('relay:write')` `:166` | HOLDS (journal/inbox receipt, no provider effect) |
| `api/memory/recent` `:187-201` | GET | read | `authorizeDevice('control:read')` `:192` | HOLDS |
| `api/media/library` `:202-217` | GET | read | `authorizeDevice('control:read')` `:207`; query string rejected `:208` | HOLDS |
| `api/control/{catalog,world,language,lexicon,recent,plans/:id}` `:223-236` | GET | read | `authorizeDevice('control:read')` `:227` | HOLDS |
| `api/control/plan` `:237-249` | POST | privileged (allocates plan digest) | `control:read` `:227` + `origin===origin` + `x-vivim-control:1` `:239` | HOLDS — but plan is admitted at `control:read`, same scope as reads |
| `api/control/interpret` `:243-246` | POST | privileged (LLM/parse) | same as `plan` | HOLDS, same scope asymmetry |
| `api/control/execute` `:240,250-251` | POST | **effect** (World mutation) | `authorizeDevice('control:execute')` `:240` + origin/CSRF `:239` + dispatcher `c.execute({approval:{channel:'direct-phone', actor:device.deviceId}})` `:251` | HOLDS — single journaled path |
| `api/relay?path=…` GET `:257-288` | GET | read | `authorizeDevice(relay:read)` `:262` + `RELAY_PATH` allowlist `:265` | HOLDS |
| `api/relay?path=…` POST `:257-288` | POST | **effect** (creates bridge commands) | `authorizeDevice(relay:write)` `:262` + `origin===origin` `:266` + `RELAY_PATH` `:265` | HOLDS — writes only the bridge queue |
| `api/enrollment/start|prove|session` `:293-314` | POST | privileged (mints a grant/session) | **none** — network gate only; proof-of-possession is the P-256 transcript | HOLDS by design (enrollment must precede authority) |
| `api/enrollment/status|grant-status` `:301-310` | GET | read | **none** — needs a challenge/grant id as capability | HOLDS |
| `api/enrollment/grants` `:315-319` | GET | read (sensitive) | `authorizeDevice('devices:read')` `:316` | HOLDS |
| `api/pair` `:327-344` | POST | privileged (consumes QR token, starts challenge) | **none** — the 64-hex single-use token is the capability (`:334`) | HOLDS |
| `api/apk/releases`, `api/apk/debug`, `connection.json` `:349-353,377,380-388` | GET | read | **none** — network gate only, redacted (`:384-385`) | HOLDS (metadata only) |
| `api/apk/download` `:354-376` | GET/HEAD | read | **none** — releaseId only; streams a signed APK | HOLDS (artifact, not authority) |
| `<asset>` `:390-393` | GET | static | closed `Map` `:20-62`; miss → 404 `:391` | HOLDS |

**Can the PWA reach an effect route?** Yes — with a grant: `execute` (`control:execute`), `live/ack` and relay POST (`relay:write`). All three terminate in existing authority (`control/dispatcher.mjs` journal; bridge store), so the PWA adds no second writer. The PWA holds **no capability it can self-issue**: every grant is minted by `enrollment.startChallenge/prove` (`apk/enrollment.mjs`) and approved owner-side.

**The real effect hole is next door, on the same origin.** `server.mjs` serves, unauthenticated, `POST /api/onboarding/setup` (`:681-697`, spawns `studio setup` with caller-supplied `workspaceRoot`/`chromeExe`/`chromeProfile` via `:688-694`), `/start` (`:699-711`, runs `powershell.exe … VIVIM.ps1` `:708`), `/doctor` (`:713-721`), `/apk/register` (`:723-735`, resolves an arbitrary caller path) and `/pair/register` (`:797-801`, mints pairing tokens). No owner token, no origin check — the file says so at `:723-726`. Only the `guided` block checks origin (`:816-819`), and only those routes are covered by `test/onboarding.test.mjs:183-210`. Today this is unreachable *from the PWA* only because the PWA is never served on that origin. The moment a local `/mobile/` is mounted on the same loopback origin, the PWA's own JS (or any injected string) can POST these. That is the P0 that must ship with the loop.

**Service worker.** Network-first, cache-as-fallback only: `sw.js:161-196`. Single cache name `var CACHE = 'vivim-relay-shell-v4'` (`:8`); precache of a fixed `SHELL_PATHS` list (`:12-50`) with `cache:'reload'` (`:132`) and unconditional `skipWaiting` (`:142,145`); `activate` deletes only caches with prefix `vivim-relay-shell-` and name `!== CACHE` (`:149-158`). On every successful shell fetch it re-`cache.put`s (`:177`). **Exact invalidation trigger: editing the string at `sw.js:8`.** Nothing else rotates the cache.

Stale-UI risk is therefore narrow but real: **online, a code edit is picked up immediately** — the network wins and refreshes the cache. **The edit is invisible only when the fetch fails**, e.g. Host restart mid-iteration, killed process, or a module 404 (`:184-192` silently resolves the cached `INDEX_PATH`). Mixed versions are possible: `index.html` from cache v4 plus a newly-edited module that did load. `handles()` (`:106-118`) excludes `/api/`, `?token=`, auth headers (`authorization`, `:115`) and `range`, so no privileged traffic is ever replayed. Note `cache-control: no-store` is set on every Host response including `/mobile/` (`server.mjs:37-38` applied at `:227` before the direct dispatch `:229`), so the HTTP cache is not the vector — the SW cache is.

**Secrets.**

| Secret | Lives | Committed? | Logged? | Exposed to PWA / store / curl? |
|---|---|---|---|---|
| Owner bearer | env `VIVIM_BOOTSTRAP_TOKEN` or `<dataRoot>/bootstrap/owner-token` via `bootstrap/owner-auth.mjs loadOwnerToken`; read at `server.mjs:845`, `src-tauri/src/main.rs:97` | No (runtime path only) | Only the **source path**, `server.mjs:847`; never the value (`control/cli.mjs:3-4,19`) | Not sent to the browser: `ui-exchange` trades a 60 s single-use code for an HttpOnly cookie (`server.mjs:571-584`); no `VIVIM_BOOTSTRAP_TOKEN`/`owner-token`/`authorization` reference anywhere in `apps/mobile-relay/` except SW's header exclusion `sw.js:115` |
| Device key | non-extractable P-256 `CryptoKey` in IndexedDB, `browser-device-key.mjs:1-4,34-48`; SW never opens that store (`:4-5`) | No | No | Never leaves the browser; same-origin script could still call `sign` (documented `:4`) |
| Pairing code | URL fragment, `pairing-link.mjs:2-10` (64 hex ⇔ device token, `:7`); redeemed once `POST /mobile/api/pair` `:14-17`; consumed `direct-phone.mjs:334` | No | No | Fragment never reaches a server or the SW; it is however **single-use only, not bound to a user agent** — screen/QR shoulder-surfing is the residual risk |
| Device session | in-memory only, `transport.mjs:29-36` (`never persist … in localStorage`); headers `:70` | No | No | Replayed per request as `x-vivim-grant-id`/`x-vivim-device-session` until `expiresAt`; static within window, no per-request nonce (UNPROVEN runtime half) |

**Static allowlist — closed.** Host: 12 entries in `ASSETS` (`server.mjs:57-69`), lookup `let asset = ASSETS[pathname]` (`:290`), miss → 404 (`:292`), then `readFile(path.join(staticRoot, asset))` (`:295`) with `staticRoot` fixed to `public/` (`:164`) and `pathname` taken from a parsed URL (`:232`). No directory listing, no `..` reachable, no dev route. `/` swaps to `onboarding.html` when no manifest (`:291`) — still an allowlisted entry. Direct/PWA: closed `assets` Map with 31 explicit entries (`direct-phone.mjs:20-62`), served from `readFile(path.join(root, asset[0]))` where `root` is the relay dir (`:19,392`); `route` is the raw suffix after `/mobile/` (`:157`) with no normalisation, but only exact Map keys resolve (`:390`). Traversal is not reachable (`assets.get('../x')` is undefined).
Note: `public/mobile-first.html`, `mobile-first.css`, `mobile-first.js` exist on disk but are **absent from `ASSETS`** → permanently 404. Dead surface, not a leak; also an iteration trap, since a PWA import missing from `direct-phone.mjs:20-62` fails only in the browser.

## 2. OPTIONS — enable fast local iteration without weakening authority

Ranked by (loop unblocked ÷ risk added). All five are one coding pass each.

### OPT-1 — Local-owner read ceiling for `/mobile/` **(S)** ← first pick
**Outcome:** the PWA is served and hot-iterable at `http://127.0.0.1:43187/mobile/` with no Tailscale, no proxy header, no phone and no enrollment; every effect route answers 403 `DEVICE_SCOPE_DENIED`.
**Repairs:** the front-door violation only. Authority gets *narrower* in this mode, never wider — production path untouched when the flag is unset.
**Work:** `direct-phone.mjs:80-83` accept `http:` only when `origin` is loopback and `opts.local === true`, keeping the exact-origin check; `:150-152` branch `if (local)` → loopback `remoteAddress` + `host === url.host` only, no `x-forwarded-proto`, no `tailscale-user-login`; `:141-148` in local mode resolve a synthetic device grant whose scope list is exactly `['control:read','relay:read','devices:read']` so `relay:write` (`:166,262`) and `control:execute` (`:240`) throw 403; `server.mjs:224` pass `local: process.env.VIVIM_DIRECT_LOCAL === '1'`; `server.mjs:851-852` default `origin: 'http://127.0.0.1:'+port` when local and `VIVIM_DIRECT_ORIGIN` unset; one loud startup line naming the mode.
**Evidence not done:** no `local`/`VIVIM_DIRECT_LOCAL`/loopback-http branch exists (`direct-phone.mjs:82-83,151-152`); client side already supports it (`transport.mjs:12,22`), so this is purely the server.
**Risk if skipped:** iteration stays at zero and the inevitable shortcut is deleting the gate or pasting the owner token into the browser — strictly worse than a scoped flag.
**Accept:** `VIVIM_DIRECT_LOCAL=1 node apps/vivim-host/server.mjs` then `curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:43187/mobile/` → `200`; `curl -s -o /dev/null -w "%{http_code}" -X POST -H "content-type: application/json" -H "origin: http://127.0.0.1:43187" -H "x-vivim-control: 1" -d '{"planId":"x","planDigest":"y"}' http://127.0.0.1:43187/mobile/api/control/execute` → `403`; `node --test apps/vivim-host/test/direct-local-mode.test.mjs` (new) + existing `node --test apps/vivim-host/test/control-http.test.mjs`.
**Excludes:** no new scope, no owner token in the browser, no change to the Tailscale/Direct production gate, no local-mode write path of any kind.

### OPT-2 — Authenticate the onboarding/APK POST surface **(S/M)** — P0, ships with OPT-1
**Outcome:** nothing on the loopback origin can spawn a process, run PowerShell, register an APK path or mint a pairing token without owner proof. This is the gate that makes OPT-1 safe.
**Repairs:** unauthenticated privileged surface (`server.mjs:681-763,797-801`), today reachable by any same-origin page.
**Work:** `server.mjs handleOnboarding` — one shared guard before `setup`/`start`/`doctor`/`apk/register`/`pair/register`: `verifyOwnerToken` (reuse `ownerAuthorized` `:546`) + `origin === 'http://'+host` + `x-vivim-onboarding:1`, following the pattern already in `handleBootstrap:419,423`; constrain `apkPath` (`:733`) to an allowlisted roots list; keep `state`/`pair-status`/guided reads loopback-only.
**Evidence not done:** the file states it explicitly — `:723-726` "Unauthenticated DEV onboarding route"; only `guided` checks origin (`:818`), and `test/onboarding.test.mjs:183-210` exercises only those.
**Risk if skipped:** on the same origin as the PWA, arbitrary `chromeExe`/`workspaceRoot` reaches `process.execPath` (`:688-694`) and `powershell.exe` (`:708`). No token needed.
**Accept:** `node --test apps/vivim-host/test/onboarding.test.mjs` with new asserts — `POST /api/onboarding/setup|start|doctor|apk/register|pair/register` → `401` without `authorization`, → `403` with a foreign `origin`.
**Excludes:** Direct/Tailscale phone paths (`direct-phone.mjs:152` already covers them); no change to guided flow semantics.

### OPT-3 — Kill SW staleness in the loop **(S)**
**Outcome:** a local edit can never be hidden; cache invalidation is automatic per boot instead of a human string edit.
**Repairs:** stale-UI risk (`sw.js:8` manual `CACHE`, `:184-192` silent shell fallback).
**Work:** in local mode serve `sw.js` with a generated name — `direct-phone.mjs:39` asset entry returns an in-memory rewrite of `var CACHE = 'vivim-relay-shell-dev-' + bootId`; or simplest, skip registration in local mode (`app.js:815` already conditional on `transport.direct`) and set `VIVIM_LOCAL` on the transport; additionally surface cache state in the build console.
**Evidence not done:** `CACHE` is a literal at `:8`; `activate` only prunes prefixed names (`:153`) — no build-time or per-boot input exists.
**Risk if skipped:** an edit is invisible exactly when the Host restarts while you are editing — the case a fast loop hits most — and mixed old-index/new-module is the confusing variant.
**Accept:** `node --test apps/mobile-relay/test/service-worker.test.mjs`; new assert that local mode either emits a per-boot cache name or emits no registration, and that `handles()` still excludes `/api/` and auth headers (`:106-118`).
**Excludes:** the production network-first strategy — it is correct; do not add stale-while-revalidate.

### OPT-4 — Allowlist parity guard **(S)**
**Outcome:** adding a PWA module cannot silently 404 in the browser, and the dead `mobile-first.*` surface is either served or deleted.
**Repairs:** iteration trap in `direct-phone.mjs:20-62`; dead files `public/mobile-first.{html,css,js}` absent from `server.mjs:57-69`.
**Work:** new `apps/vivim-host/test/static-allowlist.test.mjs` that walks `apps/mobile-relay` and every `ASSETS` entry and fails on (a) a served path missing on disk, (b) a file imported by `app.js`/`index.html` that is not in the direct Map; then either add `mobile-first.*` to `server.mjs:57-69` or delete the three files.
**Evidence not done:** no such test exists; `mobile-first.*` is on disk and unserved (`Select-String mobile-first server.mjs` → no hit).
**Risk if skipped:** low security risk; high iteration tax — the failure appears as a blank PWA on a phone or a 404 in devtools.
**Accept:** `node --test apps/vivim-host/test/static-allowlist.test.mjs` exit 0, and it fails when a line is added to a module without a Map entry (self-test).
**Excludes:** no change to the closed-by-default design; no directory serving.

### OPT-5 — Admission contract test for the whole PWA route surface **(M)**
**Outcome:** the route table above becomes executable: every route has a classification and a negative test, so a future edit cannot add an effect route or drop a scope check silently.
**Repairs:** the read/privileged/effect boundary is currently prose + scattered inline checks (`:162,166,239,240,262,266`).
**Work:** new `apps/vivim-host/test/direct-phone-admission.test.mjs` driving `createDirectPhoneHandler` with a stub enrollment: assert 401 without device headers on each scope-gated route, 403 on `control:execute` with only `control:read`, 403 on `relay:write` with `relay:read`, 403 `execute` without `x-vivim-control`, 403 `relay` POST with foreign origin (`:266`), and that `enrollment.start|prove|session` remain unauthenticated deliberately.
**Evidence not done:** no test file references `createDirectPhoneHandler` admission at all.
**Risk if skipped:** the ceiling in OPT-1 protects today's routes; the next route added by anyone is unprotected by construction.
**Accept:** `node --test apps/vivim-host/test/direct-phone-admission.test.mjs` exit 0; `node scripts/test-gate.mjs --gate host-suite`.
**Excludes:** runtime device/Tailscale realism — that stays a device gate, not this test.

**First pick: OPT-1, with OPT-2 as the same-pass P0.** Reason: the loop is currently zero — not slow, zero — because `direct-phone.mjs:82-83` throws on any non-https origin before a single route exists, so every PWA edit today is unverifiable except through the Host Control UI, which does not run PWA code. OPT-1 is one construction check plus one gate branch plus a scope ceiling, and it strictly *reduces* authority in that mode. OPT-2 is not optional: the moment the PWA is served from the loopback origin it shares that origin with unauthenticated `process.execPath`/`powershell.exe` triggers (`server.mjs:681-735`), so shipping OPT-1 alone would hand an unenrolled UI a local RCE-adjacent surface. Do OPT-3 in the same pass; it is a three-line change and removes the most common "my edit did nothing" confusion.

## 3. DEFERRED-SAFE (wait; none make the loop dishonest, none grant effect authority, none leak a secret, none fake readiness)

1. **Tauri owner-token handshake (`main.rs:103-130`)** — installer path, not the PWA loop; the loop uses the CLI-minted UI session.
2. **Tauri version bump / capability wildcard (`Cargo.toml:7`, `capabilities/default.json:7-9`)** — native shell only; PWA served by Node.
3. **Device per-request nonce signing (`transport.mjs:70`)** — sessions are grant-bound, ephemeral and 403-scoped; local mode has no write scope at all.
4. **R2 fresh-approval bypass via UI cookie (`server.mjs:598-599,662`)** — pre-existing Host Control issue, unchanged by the loop.
5. **Root lockfiles untracked (`Cargo.lock`, `bun.lock`)** — build reproducibility, not authority; `release-manifest.json:23` already records `rootLockfiles:"ABSENT"`.
6. **Status-authority contradiction (`beta-gates.json:3,8,123,130` vs `release-manifest.json:4,14-15`)** — release truth, orthogonal to local iteration; must be fixed before any go/no-go.
7. **Donor wiring in `hq/` (`channel-broker.mjs:854-883`)** — dev intelligence, not on the `/mobile/` path.
8. **`mobile-first.*` dead surface** — fold into OPT-4, not urgent alone.
9. **Second World writer via bootstrap mutations (`server.mjs:305-317,416-431`)** — owner-token-authenticated (`:419,423`) and journal-less; affects effect truth, not the read loop.

## 4. EVIDENCE APPENDIX (tightened, PWA surface only)

- **CT-01 / P0 / local PWA cannot be served** — `direct-phone.mjs:82-83` (exact https throw) + `:151-152` (Tailscale headers) + `server.mjs:851-852` (env-gated mount) + `:229`. Client already ready: `transport.mjs:12,22`. → OPT-1.
- **CT-02 / P0 / unauthenticated process spawn on the same origin** — `server.mjs:681-697` (`process.execPath` with caller `chromeExe/workspaceRoot` `:688-694`), `:699-711` (`powershell.exe` `:708`), `:723-735` (arbitrary `apkPath`), `:797-801` (mints pair tokens); explicitly acknowledged `:723-726`; only `guided` checks origin `:818`; coverage limited to guided `test/onboarding.test.mjs:183-210`. → OPT-2, must ship with OPT-1.
- **CT-03 / P1 / scope asymmetry: planning under a read scope** — `direct-phone.mjs:227` admits every `api/control/*` route at `control:read`, including `plan` `:249` and `interpret` `:246`; only `execute` raises to `control:execute` `:240`. → OPT-5 assertion.
- **CT-04 / P1 / SW invalidation is a hand-edited literal** — `sw.js:8`; prune `:153`; silent shell fallback `:184-192`; network-first `:171-183`. → OPT-3.
- **CT-05 / P2 / PWA allowlist maintenance trap** — `direct-phone.mjs:20-62` vs `app.js` imports; `public/mobile-first.*` absent from `server.mjs:57-69`. → OPT-4.
- **CT-06 / P2 / static session bearer within its window** — `transport.mjs:70`, `direct-phone.mjs:144`; no nonce binding. Deferred-safe #3.
- **Duplicate prose omitted:** host static allowlist enforcement `server.mjs:290-295`; SECURITY headers `:37-43` applied at `:227` before direct dispatch; direct CSP override `direct-phone.mjs:153`.

## 5. Outside PWA host priority (one line each, no analysis)

Tauri bootstrap/token handshake; Tauri version and capability scope; onboarding-first-run product flow; HQ donor wiring and `hq/` boundary tests; HVI G1/G2 capability discovery; beta-gate and release-manifest reconciliation; Rust `Cargo.lock`/`bun.lock` tracking; APK release pipeline and signing; VEC compiler and `packages/vec-ui-engines/`; Studio conversation journal; device per-request nonce protocol; CI `startup_failure` and test-gate wiring.

## 6. UNVERIFIED

- No browser was launched; the `200`/`403` acceptance checks in OPT-1/OPT-2 are proposed commands, **not executed results**.
- No phone, Tailscale proxy, signed-in Chrome or Android WebView exercised. All device-scope verdicts rest on source reading; `transport.mjs`/`browser-device-key.mjs` runtime behaviour is unproven here.
- Only `apps/vivim-host/test/control-http.test.mjs` ran (1/1, 4157 ms). `apps/mobile-relay/test/service-worker.test.mjs`, `test/onboarding.test.mjs` and `node scripts/test-gate.mjs --all` were **not** run in this lane.
- `direct-phone.mjs` route table was read in full (`:150-395`), but no request was issued against it; admission semantics are inferred from the guards, not observed.
- Service-worker staleness is a read of `sw.js:106-196`; no cache state was inspected in a real browser.
- Secret handling is source-read plus a bounded regex scan of tracked text files (0 real credentials, 6 detection-regex hits). Absence of hits is not proof of absence.
- Whether `beta-gates.json` `ACCEPTED` reflects an uncaptured owner decision is an owner statement this lane cannot adjudicate; the two files still disagree, which is the defect.
