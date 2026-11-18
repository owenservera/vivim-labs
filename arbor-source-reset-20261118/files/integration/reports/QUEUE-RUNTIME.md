# QUEUE-RUNTIME — PWA host iteration loop, measured

**Base SHA:** `452c82e` (main, upstream `origin/main`) · **Date:** 2026-11-13 · **Total wall time:** ~42 min
**Scope:** read-only; one written file (this report). No git mutation.

## Commands run (exact, with exit codes)

| # | command | exit | wall | result |
|---|---|---|---|---|
| C1 | `cargo check --workspace --locked` | 101 | 87s | `error: cannot create the lock file C:\0-BlackBoxProject-0\ARBOR\Cargo.lock because --locked was passed` (no root lockfile at this SHA) |
| C2 | `CARGO_TARGET_DIR=%TEMP%\arbor-cargo-target cargo check --workspace` | 101 | 350s | `error: failed to run custom build command for vivim-host` → `resource path resources\runtime doesn't exist` |
| C3 | `bun scripts/test-gate.mjs --all --no-cache` | 1 | 665s | 271 selected, **2377 pass, 39 fail** |
| C4 | `node apps/vivim-host/server.mjs` (`VIVIM_HOST_PORT=43999`, `VIVIM_HOST_DATA_DIR=%TEMP%\arbor-host-probe`) | running | — | boot OK, ready in **3595 ms**, stderr empty |
| C5 | same + `VIVIM_DIRECT_ORIGIN=https://pc.example.ts.net`, `VIVIM_DIRECT_USER=owner@example.com`, port 44001 | running | — | `/mobile` served only with simulated Tailscale headers |
| C6 | `node --test apps/vivim-host/test/mobile-first-entry.test.mjs` | 1 | 2s | **2 pass 0 fail→0 pass 2 fail**; `/` returned legacy `host.js` |
| C7 | `bun test packages/tierflow/tests/bus.test.ts` | 0 | 1s | **9 pass 0 fail** (125 ms) — gate recorded 0/0 |
| C8 | `node --test apps/vivim-host/test/data-intelligence.test.mjs` | aborted | >240s | hung standalone; gate measured **317385 ms / 4 fail** |
| C9 | `node hq/bin/arbor-hq.mjs status --allow <repo>` | 0 | 2s | `arbor-self` 2197 files, 52756 symbols |

Processes started: PIDs 16048 and 20252 — both killed by PID. No other process touched.

---

# 1. PWA HOST ITERATION LOOP — current actual state

**Boot: YES. PWA serve: ONLY behind a simulated Tailscale proxy. Render: NO (framing refused).**

`node apps/vivim-host/server.mjs` boots from a clean tree in **3595 ms**, prints `VIVIM Host preview: http://127.0.0.1:43999`, empty stderr, no build step, no network. Default port **43187** (`apps/vivim-host/server.mjs:832`); data root env `VIVIM_HOST_DATA_DIR` (`server.mjs:72-80`).

### Routes (measured; `HEAD`/`GET` against a live boot)

| route | plain `http://127.0.0.1:44001` | with `Host: pc.example.ts.net` + `x-forwarded-proto: https` + `tailscale-user-login` |
|---|---|---|
| `/` | **200** `text/html` (legacy desktop `index.html`) | 403 |
| `/onboarding` | 200 `text/html` | — |
| `/host.css`, `/control-panel.js` | 200 | — |
| `/manifest.webmanifest` | **404** | — |
| `/sw.js` | **404** | — |
| `/mobile` | **403** `application/json` | 200 `text/html` |
| `/mobile/` | **403** | 200 `text/html` |
| `/mobile/app.js` | **403** | 200 `text/javascript` |
| `/mobile/manifest.webmanifest` | **403** | 200 `application/manifest+json` |
| `/mobile/sw.js` | **403** | 200 `text/javascript` |
| `/mobile/?host-preview=1` | 200, but `x-frame-options: DENY`, `frame-ancestors 'none'` | same |
| `/mobile-first.html`, `/mobile-first.js`, `/mobile-first.css` | **404** | **404** |

Cause of each: `/mobile` gated by `direct-phone.mjs:152` (`remote` loopback **and** `req.headers.host !== url.host` **and** `x-forwarded-proto !== 'https'` **and** `tailscale-user-login !== user`); origin must be exact HTTPS (`direct-phone.mjs:82-83`). `/mobile-first.*` never registered in the two allowlists. `/manifest.webmanifest` + `/sw.js` exist only under `/mobile/` (`direct-phone.mjs:20-70`), not at host root (`server.mjs:44-66`).

### Edit → visible change (measured)

| change | steps | measured |
|---|---|---|
| edit existing `apps/mobile-relay/**` or `apps/vivim-host/public/**` | edit, browser reload | **9–45 ms** warm serve, 562 ms first request — files re-read per request (`direct-phone.mjs:392`, `server.mjs:295`) |
| new file under `apps/mobile-relay/` | edit `assets` Map (`direct-phone.mjs:20-70`) then restart | else **404** (`direct-phone.mjs:391`); restart **3.6 s** |
| new file under `apps/vivim-host/public/` | edit `MIME` (`server.mjs:44`) + `ASSETS` (`server.mjs:57`) then restart | restart 3.6 s |
| any route/server logic (`server.mjs`, `direct-phone.mjs`) | restart | 3.6 s; no `--watch` and no `dev` script in `package.json` |
| build | **none** — no vite/esbuild/rollup/webpack config anywhere; `node_modules` present, no `bun install` needed | 0 |
| service worker | network-first, cache bump only on name change (`apps/mobile-relay/sw.js:8,161,171`) | no purge required |

**Verdict:** the build is not the bottleneck (0 s). Access is: a plain browser cannot open `/mobile/` at all (403), and `/` serves the legacy desktop page, so a UI edit is only observable via `curl` with spoofed proxy headers or a hand-built TLS proxy.

---

# 2. OPTION PACKAGES (ranked)

### O1 — Loopback read-only PWA preview mode  ★ PICK FIRST
- **Outcome:** open `http://127.0.0.1:43187/mobile/` in Chrome, edit a PWA file, reload, see it. No env gymnastics, no phone, no proxy.
- **Why first:** every other UI iteration is currently invisible; this is the single gate on the loop.
- **Work:** `direct-phone.mjs:152` guard + `:153` CSP → accept loopback + `VIVIM_HOST_PREVIEW=1` with `Host` = `127.0.0.1|localhost`, set `x-frame-options: SAMEORIGIN`, `frame-ancestors 'self'`; `server.mjs:229` dispatch, `:851-852` `directPhone` construction. Device-grant routes stay 401/403.
- **Not already done:** measured 403 on `/mobile` and `/mobile/` without proxy headers; `direct-phone.mjs:82-83` rejects non-HTTPS origin outright.
- **Effort:** S–M (<1–3 h). **Risk:** med — a preview must never mint a device session or read owner history.
- **Acceptance:** `VIVIM_HOST_PREVIEW=1 node apps/vivim-host/server.mjs` then `curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:43187/mobile/` → `200`, header `x-frame-options: SAMEORIGIN`; and `GET /mobile/api/memory/recent` → 401/403.
- **Excludes:** TLS, Tailscale, real pairing, send paths.

### O2 — Wire `/` to the PWA-first entry that already exists on disk
- **Outcome:** the documented product surface (phone PWA first, Host utilities on demand) is real at `/`.
- **Work:** `server.mjs:44-66` (`ASSETS['/']` → `mobile-first.html`, register `mobile-first.{js,css}`), `:290-295`; frame allowance per O1. `public/mobile-first.html` + `.js` already present.
- **Not done:** C6 fails 2/2 — `/` returned `host.js`+`control-panel.js`; `/mobile-first.js` 404; framed `/mobile/` sends `XFO: DENY`.
- **Effort:** S. **Risk:** low if legacy stays at `/host/control`.
- **Acceptance:** `node --test apps/vivim-host/test/mobile-first-entry.test.mjs` → 2 pass.
- **Excludes:** Tauri launch-code exchange, packaging.

### O3 — Make the loop visible to the gate
- **Outcome:** a PWA-entry regression fails the canonical gate in seconds instead of shipping silently.
- **Work:** add `apps/vivim-host/test/mobile-first-entry.test.mjs` + a boot/serve smoke to `scripts/test-manifest.json` under a new `pwa-loop` gate; fix `parseBunTotals` ANSI mismatch (`scripts/test-gate.mjs:~188-196`); quarantine the 317 s `data-intelligence` test.
- **Not done:** zero manifest entries match `mobile-first` (271 scanned); 31 bun-runtime files record `0 pass 0 fail` while C7 runs 9 tests.
- **Effort:** S. **Risk:** low–med (receipt shape changes).
- **Acceptance:** `bun scripts/test-gate.mjs --gate pwa-loop` → exit 0, non-zero pass counts, ≤10 s.
- **Excludes:** re-triaging the other 270 tests.

### O4 — One-command dev loop with restart
- **Outcome:** `bun run dev:host` boots + `--watch` restart on `server.mjs`/`direct-phone.mjs` edits; no manual kill.
- **Work:** `package.json` script + `node --watch` (or `bun --watch`) wrapper, port/data-root env defaults.
- **Not done:** `package.json` has only `test:*` scripts; no watch anywhere.
- **Effort:** S. **Risk:** med — restart drops in-memory bootstrap worker/lease; must not double-write the ledger.
- **Acceptance:** edit `direct-phone.mjs`, server restarts unattended, `curl` 200 within 2 s, single ledger writer.

### O5 — Single asset allowlist (kill the duplicate maps)
- **Outcome:** dropping a file into `public/` or `mobile-relay/` serves it; no double bookkeeping.
- **Work:** one route table shared by `server.mjs:44-66` and `direct-phone.mjs:20-70`, deny-by-default with extension/dir allowlist.
- **Not done:** two independent hardcoded maps today; `mobile-first.*` proves they drift.
- **Effort:** M. **Risk:** med — must not expose `.env`, `node_modules`, or drafts.
- **Acceptance:** new file serves with no map edit; `/.env`, `/../` , `/node_modules/x` still 404.

**Rank:** O1 > O2 > O3 > O4 > O5. **Pick O1** — without it the loop has no visible output at all; O2/O3 land naturally after it.

---

# 3. EVIDENCE APPENDIX (queue items)

| ID | Sev | Defect | Evidence | Lane | Acceptance |
|---|---|---|---|---|---|
| RT-01 | **P0** | Windows app cannot build: Tauri bundle references a missing resource dir | C2: `resource path resources\runtime doesn't exist`; `src-tauri/tauri.conf.json:30-31` `"resources/runtime/": "runtime/"`; `src-tauri/resources/` holds only `vivim-release.manifest.json` | rust-native | `cargo check --workspace` exit 0 |
| RT-02 | **P1** | PWA-first entry absent: `/` still serves legacy desktop page, `mobile-first.*` 404 | C6 2 fail (`/` body had `host.js`/`control-panel.js`); route table rows 1, 12; `server.mjs:58` `'/': 'index.html'` | host-js | mobile-first-entry test 2 pass |
| RT-03 | **P1** | `/mobile/` unreachable from a local browser (403 unless Tailscale headers + HTTPS origin spoofed) | route table; `direct-phone.mjs:152`, `:82-83` | host-js | O1 acceptance |
| RT-04 | **P1** | Gate is blind to the PWA entry: `mobile-first-entry.test.mjs` absent from manifest | python scan of `scripts/test-manifest.json` for `mobile-first` → `[]`; 271 entries | host-js | O3 acceptance |
| RT-05 | **P1** | `data-intelligence.test.mjs` takes **317 s** and fails 4 in host-suite (dominates the gate) | C3 line `FAIL ... data-intelligence.test.mjs ... 0 pass 4 fail 317385ms`; C8 hung >240 s standalone | host-js | test < 10 s, exit 0 |
| RT-06 | **P1** | Corpus intake + phone-auth security paths fail (19 asserts) | C3: `bootstrap/web-memory-phone-auth.test.mjs 0 pass 1 fail`; `corpus/intake/host-intake.test.mjs 11 pass 18 fail` | host-js | both exit 0 |
| RT-07 | **P1** | HQ claims broken at this SHA | C3: `git-main-sync 0/1`, `live-graph-wave1-master 0/1`, `live-graph-wave1-a-r2-preflight 0/1`, `comment-program-integration 0/3`, `comment-wiki-bridge 0/1`, `day-one-context`, `intelligence-contracts` | hq-js | those files exit 0 |
| RT-08 | P2 | Gate records `0 pass 0 fail` for 31 bun-runtime files (ANSI codes defeat the totals regex) — counts and receipts are wrong | C3 vs C7 (`bun test bus.test.ts` → 9 pass); `scripts/test-gate.mjs:~188-196` | host-js | gate reports >0 for bun files |
| RT-09 | P2 | Static allowlist "proved" by source-text regex only — cannot detect a 404 | `apps/mobile-relay/test/host-static-allowlist-contract.test.mjs:17-30` (`assert.match(source, /'browser-device-key\.mjs'.../)`) | pwa-tierflow | runtime fetch test |
| RT-10 | P2 | Two duplicated hardcoded asset maps drift (`mobile-first.*` already unregistered in both) | `server.mjs:44-66` vs `direct-phone.mjs:20-70` | host-js | O5 acceptance |
| RT-11 | P2 | Host origin serves no manifest/service worker → not installable from `127.0.0.1` | route table `/manifest.webmanifest` 404, `/sw.js` 404 | host-js | both 200 on host origin |
| RT-12 | P2 | 33 manifest files assert nothing (0/0), incl. domain-separated `hq-contract.test.mjs` skip | C3 parse: 33 files `0 pass 0 fail`; `apps/vivim-host/corpus/transport/hq-contract.test.mjs:17` skip unless sibling `repos-hq` | host-js | 0/0 files are skipped-with-reason or removed |
| RT-13 | P2 | No-donor closure proof scans 3 files only | `hq/test/in-repo-closure.test.mjs:8` (`['lib/arbor-service.mjs','bin/arbor-hq.mjs','package.json']`); `tests/privacy/w01-f-source-lint.test.mjs:5` scans 3 dirs | hq-js | tree-wide scan, 0 findings |
| RT-14 | P2 | Assorted host/compat failures | C3: `control-language 43/1`, `device-ui-wiring 1/1`, `kpi-utc 0/1`, `tests/compatibility/wave1f 5/1`, harness 3 files 5 fails, `packages/tierflow/tests/e2e.test.ts` FAIL 0/0 | host-js | those files exit 0 |

**Gate/build table (C3, exit 1, 665 s):**

| gate | files | pass | fail | unrun |
|---|---|---|---|---|
| uncategorized | 126 | 1233 | **24** | 0 |
| host-suite | 50 | 430 | **7** | 0 |
| hq-suite | 47 | 315 | **7** | 0 |
| interop | 28 | 193 | 0 | 0 |
| hvi-g1-g2 | 6 | 73 | 0 | 0 |
| hvi-modules | 6 | 81 | 0 | 0 |
| hvi-hq-boundary | 3 | 9 | 0 | 0 |
| hvi-host-read-port | 2 | 17 | 0 | 0 |
| hvi-discovery | 1 | 4 | 0 | 0 |
| hvi-tierflow-inspector | 1 | 17 | 0 | 0 |
| compatibility | 1 | 5 | **1** | 0 |

18 failing files / 39 failing assertions. Rust: `cargo test` never reached — `cargo check` fails first (RT-01). No Rust test count executed.

**Classification:** *(a) executed-proven* — Host boot + loopback `/`, `/onboarding`, static assets, `/mobile/*` under simulated proxy, 2377 gate assertions, tierflow visual-runtime (~800 asserts). *(b) unproven* — PWA-first `/`, local preview, PWA install (manifest/SW at host origin), corpus intake, HQ git/live-graph/sync, Windows build, all 226 Rust tests. *(c) absent* — `resources/runtime/` bundle dir, dev/watch script, single asset allowlist, tree-wide donor scan.

**Artifacts my runs left in-tree (untracked, not product source):** `Cargo.lock` (created by C2), 11 `*-receipt-452c82e….json` + `run-receipt-20261113T020943Z-452c82eb48db.json` under `.integration/proposal5/hq-version-intelligence/proofs/G2/` (created by C3). `src-tauri/gen/schemas/*.json` show `M` with **0 line diff** (`git diff --numstat` empty) — touched by the tauri build script in C2.

---

# 4. UNVERIFIED

- Whether a real Tailscale Serve proxy actually reaches `/mobile/` on this machine — no proxy running, no phone; the 200s above used spoofed request headers only.
- `/mobile/?host-preview=1` was intended (per `MOBILE_FIRST.md` and test line 23) as an unpaired read-only preview; no preview branch exists in `server.mjs`/`direct-phone.mjs`, but I did not diff against the HVI/wave branches that may have held it.
- Root cause of the 317 s `data-intelligence` run and its 4 failures — timed out standalone twice, no per-assert detail captured.
- `cargo build`/`cargo test` never ran (C2 fails in the `vivim-host` build script), so Rust compile status beyond `hq-knowledge-graph` + `vivim-programmability-core` check is unknown.
- Visual rendering of the PWA at any viewport — no browser driven; all route evidence is HTTP-level.
- `harness/test/**` and `hq/version-intelligence/**` failures were counted from gate output only, not root-caused.
