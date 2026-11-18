# HOST-SUITE-TRIAGE

Workspace: `C:\0-BlackBoxProject-0\ARBOR` main@452c82e. Read-only triage (this report only).

## Manifest verdict
- `apps/vivim-host/test/mobile-first-entry.test.mjs`: **ABSENT** from `scripts/test-manifest.json` (0 matches; manifest 271 entries).

## Runs (node --test)
- `apps/vivim-host/test/mobile-first-entry.test.mjs`: exit 1, 0 pass 2 fail (1564ms). First: `✖ Host default loads only the actual mobile PWA, not legacy desktop modules` at `apps/vivim-host/test/mobile-first-entry.test.mjs:19` `AssertionError ... /id="phone-surface"/` actual `/` served legacy `host.js`/`control-panel.js`.
- `apps/vivim-host/test/data-intelligence.test.mjs`: exit 1, timeout 90s (duration ~89158ms). First: `✖ versioned ingress registry...` at `apps/vivim-host/test/data-intelligence.test.mjs:16` via `apps/vivim-host/data-intelligence/catalog.mjs:10` `Error: ENOENT ... open '...\docs\data-intelligence\ingress-registry.v1.json'`. All 4 tests failed on missing docs (`docs/data-intelligence/schemas/*` also missing) but process hung to timeout after failures (likely unclosed server/runtime handles after uncaught throw in request path, see server route).
- `apps/vivim-host/bootstrap/web-memory-phone-auth.test.mjs` (requested path under test/ not found; actual per manifest): exit 1, 0 pass 1 fail. First: `apps/vivim-host/bootstrap/web-memory-phone-auth.test.mjs:35` `AssertionError 403 !== 401` "Direct phone web memory never exposes owner history without a device grant".
- `apps/vivim-host/corpus/intake/host-intake.test.mjs` (requested path under test/ not found; actual per manifest): exit 1, 11 pass 18 fail. First: `apps/vivim-host/corpus/intake/host-intake.test.mjs:49` -> assert at `:46` `503 !== 201` (many failures return `STORAGE_UNAVAILABLE`/503 instead of expected domain codes/statuses).

QUEUE-RUNTIME other host-suite failures named: `control-language`, `device-ui-wiring`, `kpi-utc`, `tests/compatibility/wave1f` (RT-14) — not individually rerun here (time/cost; gate evidence in QUEUE-RUNTIME).

| ID | file | first failure | class | minimal fix | lane |
|---|---|---|---|---|---|
| T1 | `apps/vivim-host/test/mobile-first-entry.test.mjs` | `apps/vivim-host/test/mobile-first-entry.test.mjs:19` assert `/id="phone-surface"/` on `/` (legacy `index.html`/`host.js` served) | (a) missing mobile-first preview wiring on main | Serve mobile-first entry at `/` and register `mobile-first.*`; add preview handling for `/mobile/?host-preview=1` headers/status per test (`apps/vivim-host/server.mjs` routes/assets, e.g. around `:44-66,229,290-295,851-852`) | host-js |
| T2 | `apps/vivim-host/test/mobile-first-entry.test.mjs` | `apps/vivim-host/test/mobile-first-entry.test.mjs:45` assert `/HOST_PREVIEW/` absent in `apps/mobile-relay/app.js` (+ transport/wrapper expectations) | (a) missing mobile-first preview wiring on main | Implement `HOST_PREVIEW` gating/flow in `apps/mobile-relay/app.js` and `apps/vivim-host/public/mobile-first.js` to satisfy test assertions | host-js/pwa |
| T3 | `apps/vivim-host/test/data-intelligence.test.mjs` | `apps/vivim-host/test/data-intelligence.test.mjs:16` ENOENT `docs/data-intelligence/ingress-registry.v1.json` via `apps/vivim-host/data-intelligence/catalog.mjs:10` | (b) pre-existing independent (missing artifacts on main; not mobile-first wiring) | Restore/add `docs/data-intelligence/ingress-registry.v1.json` + `docs/data-intelligence/schemas/*.json` or gate/skip when registry absent (`catalog.mjs:10`, test schema reads) | host-js/docs |
| T4 | `apps/vivim-host/bootstrap/web-memory-phone-auth.test.mjs` | `apps/vivim-host/bootstrap/web-memory-phone-auth.test.mjs:35` `403 !== 401` unauth device grant path | (b) pre-existing independent (auth/status contract mismatch) | Align direct-phone/web-memory auth denial to expected 401 vs 403 in handler (investigate `apps/vivim-host/*direct-phone*`/web-memory auth path) | host-js |
| T5 | `apps/vivim-host/corpus/intake/host-intake.test.mjs` | `apps/vivim-host/corpus/intake/host-intake.test.mjs:49` (`:46`) `503 !== 201`; widespread `STORAGE_UNAVAILABLE` | (c) flaky/env (storage/writer availability in this env) / possibly (b) if deps missing — **env-sensitive** | Ensure intake storage/writer paths writable and required runtime deps present; then reclassify specific assertion mismatches (`apps/vivim-host/corpus/intake/host-intake.mjs` + storage setup) | host-js |

## UNVERIFIED
- Exact root cause of `data-intelligence.test.mjs` 89s hang after all 4 ENOENT failures (hypothesis: unclosed server/runtime on thrown request handling); not traced with debugger.
- Whether T4/T5 are deterministic on clean CI/main env vs this machine state (no full gate rerun).
- RT-14 cheap host-suite failures (`control-language`, `device-ui-wiring`, `kpi-utc`, `tests/compatibility/wave1f`) not rerun individually; evidence only from `integration/reports/QUEUE-RUNTIME.md`.
- Real Tailscale/preview behavior beyond HTTP/unit tests (per QUEUE-RUNTIME).
