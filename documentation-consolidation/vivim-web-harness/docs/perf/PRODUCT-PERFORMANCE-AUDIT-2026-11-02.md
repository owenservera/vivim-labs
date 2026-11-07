# Product performance audit — reaction speed on every surface

Status: measured audit with landed fixes. Informative numbers, not gates. Every number below came
from a command in this repository on this machine; nothing is estimated unless labelled.

- Date: 2026-11-02
- Revision: `83c3fca` plus the working-tree changes listed in the fix ledger (the tree was already
  dirty when the audit started; unrelated dirty files were left untouched)
- Host: Windows, Node 24.22.1
- Tool: `tools/perf/bench-http.mjs` — starts the real Studio on an ephemeral loopback port with an
  injected stub conversation runtime (no Chrome, no CDP, no provider send, no owner data), then issues
  raw `node:http` requests that negotiate `gzip, br` and count the bytes that actually cross the wire.

## Headline results

| # | What was slow | Measured before | Measured after |
|---|---|---|---|
| 1 | World first paint over the wire | 625,875 B, identity, no cache, disk read per request | 166,697 B br (−73.4%); repeat load 304 / 0 B / 5 ms |
| 2 | Phone page (Studio-served) | 94,867 B | 26,518 B (−72.1%) |
| 3 | Library list with 200 sessions | `/api/sessions` 771.7 ms p50 / 919.9 ms p95 | 69.6 / 239.4 ms |
| 4 | World conversations with 200 sessions | 463.9 / 796.4 ms | 52.0 / 101.1 ms |
| 5 | Launcher port discovery (5 closed ports) | 11.09-12.07 s inside a dry run | 0.69-1.86 s |
| 6 | Relay PWA repeat load | no service worker; 244,368 B shell re-downloaded | versioned network-first shell cache (202,002 B cached once) |
| 7 | Phone→PC queue at 5,000 commands | `list queued` 166.6 ms, `claim` 355.4 ms | 2.4 ms / 13.6 ms (20k: 10.1 ms / 16.8 ms) |

Still open, with evidence and reasons in the body: the relay/mobile polling cadence, the host's full-file
`commands.jsonl` and live-projection scans, the harness's fixed 1.5 s post-open sleep and 15 s picker wait,
the 3.38 s CLI module-load cost, and the 2 s reply-stability window (deliberate, not a defect).


## 1. What "reaction speed" means here

Six user-visible waits, each with a measured or code-traced owner:

| Wait | Surface | Owner |
|---|---|---|
| Open World and see it usable | Studio web | static asset transport, first-paint API calls |
| Open the phone page and get a usable screen | Studio mobile / relay PWA | payload, service worker, fonts |
| Switch a conversation or list the library | Studio web | `/api/sessions`, `/api/v1/world/*` |
| Send and see the prompt leave | Harness → signed-in webapp | CDP attach, picker wait, draft clear |
| See a reply marked complete | Studio capture | reply stability rule |
| Double-click VIVIM and get the World | Windows launcher | port discovery, Node startup |

Surface coverage of this pass:

| Surface | Status in this audit |
|---|---|
| Studio web (World, chat, drive, platform) | Measured over HTTP, transport fixed |
| Studio API (`/api/sessions`, `/api/v1/world/*`, `/api/status`) | Measured, session-list scan fixed |
| Studio-served phone page | Measured, transport fixed |
| Relay PWA (phone) | Static size measured, repeat-load cache fixed; real-device acceptance open |
| Host command/state path | Code-traced, not changed |
| Harness pre-send path | Code-traced, not changed (needs live browser) |
| Windows launcher | Measured, fixed, unit-tested |
| Desktop Tauri shell (`src-tauri`) | Not covered in this pass |
| Browser companion extension | Passive observation only; not on a user-wait path |
| Test execution | Runner optimized; `docs/perf/TEST-TIME-OPTIMIZATION.md` |


## 2. Baseline (before any change)

`bun tools/perf/bench-http.mjs --label baseline --samples 25` at revision `83c3fca`:

| Route | p50 ms | p95 ms | Wire bytes | Encoding | Cache |
|---|---|---|---|---|---|
| `/` (World shell) | 13.1 | 54.2 | 145,281 | identity | no-store |
| `/chat` | 5.1 | 14.9 | 69,648 | identity | no-store |
| `/ui/chat.mjs` | 6.6 | 12.2 | 160,050 | identity | no-store |
| `/ui/world-view.mjs` | 6.2 | 27.9 | 250,429 | identity | no-store |
| `/mobile` | 2.6 | 8.0 | 6,486 | identity | no-store |
| `/ui/mobile.mjs` | 5.4 | 11.6 | 84,320 | identity | no-store |
| `/api/status` | 9.8 | 15.7 | 465 | identity | no-store |

Milestones from the same run:

- World first paint (the audit's modelled set: shell + chat modules + status + sessions): **625,875 bytes**
  over 6 requests, all identity, every one `no-store`.
- Phone first paint (Studio-served): **94,867 bytes**.

The same tool with `--seed-sessions 200` (synthetic session metadata only) isolated the library surface:

| Route | p50 ms | p95 ms |
|---|---|---|
| `/api/sessions` | 771.7 | 919.9 |
| `/api/v1/world/conversations?limit=50` | 463.9 | 796.4 |
| `/api/v1/world/projects` | 18.9 | 23.5 |
| `/api/status` | 34.4 | 46.6 |

Launcher baseline, measured with a stopwatch on this host:

- `vivim start -DryRun` (public CLI): **18.92 s** (target 5 s in `docs/design/DEV-TIMING-TARGETS.md`).
- `scripts/windows/VIVIM.ps1 -DryRun` alone: **18.46 s**.
- `node studio/bin/studio.mjs --version`: **3.38 s**; `info --json`: **5.38 s**.

## 3. Fix ledger

### FIX-1 — Static assets: one encoder, compression, ETag revalidation

Files: `studio/src/studio/http-encode.mjs` (new), `studio/src/studio/http.mjs`, `studio/src/studio/server.mjs`.
Tests: `studio/test/unit/http-encode.test.mjs` (10 tests).

- Negotiates brotli then gzip for bodies at or above 1024 bytes; identity is the fallback.
- Stable weak ETag over the identity body; `if-none-match` answers 304 with no body.
- Correct `content-length` for the bytes actually written; HEAD supported.
- Static files are read once per process and cached by `mtimeMs`+`size`, including their compressed
  variants, so a repeat request does no compression work.
- Static UI is now `cache-control: no-cache` (store, revalidate) instead of `no-store`. Every API/JSON
  response stays `no-store`. Bodies are static UI, never user data.

After: `bun tools/perf/bench-http.mjs --label after-compression --samples 25`

| Route | Before bytes | After bytes | Change | Repeat load |
|---|---|---|---|---|
| `/` | 145,281 | 40,419 (br) | −72.2% | 304, 0 B, 5 ms |
| `/ui/chat.mjs` | 160,050 | 47,496 (br) | −70.3% | 304, 0 B, 2 ms |
| `/ui/world-view.mjs` | 250,429 | 60,790 (br) | −75.7% | 304, 0 B, 2.2 ms |
| `/ui/mobile.mjs` | 84,320 | 22,326 (br) | −73.5% | 304, 0 B, 10.4 ms |
| `/mobile` | 6,486 | 2,425 (br) | −62.6% | 304, 0 B, 6.2 ms |

- World first paint: **625,875 → 166,697 bytes (−73.4%)**.
- Phone first paint (Studio-served): **94,867 → 26,518 bytes (−72.1%)**.
- `/` p50 13.1 → 5.4 ms; `/ui/world-view.mjs` p95 27.9 → 10.3 ms (cached file, no per-request disk read).

### FIX-2 — Session metadata: stat-keyed parse cache

Files: `studio/src/store/session.mjs`, `studio/test/unit/session-list-cache.test.mjs` (9 tests).

`listSessions()` re-ran `readdirSync` + `readFileSync` + `JSON.parse` for every `session.json` on every
request, synchronously. It now caches parsed metadata per file keyed by `mtimeMs`+`size`, returns fresh
shallow copies, and keeps the cache bounded at 2000 entries. Raw-byte totals are still recomputed per
call (correctness over speed there).

`bun tools/perf/bench-http.mjs --seed-sessions 200 --samples 12` after:

| Route | Before p50 / p95 | After p50 / p95 |
|---|---|---|
| `/api/sessions` | 771.7 / 919.9 ms | 69.6 / 239.4 ms |
| `/api/v1/world/conversations` | 463.9 / 796.4 ms | 52.0 / 101.1 ms |

The remaining ~0.3 ms per session is one `statSync` (Windows) and is the honest floor for detecting
external writers without cross-process invalidation. Under a burst of 8 concurrent clients at 200
sessions the single event loop still serializes those scans: `sessions` p95 929.5 ms, `world.conversations`
p95 881.4 ms (`concurrent-8-200.json`). A short-TTL list snapshot would collapse a burst into one scan, but
it can serve stale metadata for the TTL, which conflicts with create-then-list flows; that trade was not
taken without an invalidation contract.

### FIX-3 — Windows launcher port discovery

See section 6. Files: `studio/scripts/windows/VIVIM.ps1`, `studio/test/unit/windows-launcher-probe.test.mjs`.

### FIX-4 — Test execution time

Full detail and receipts: `docs/perf/TEST-TIME-OPTIMIZATION.md`. Headline: 323 files run cold in **373.6 s**
(Bun entry, concurrency 8); a warm run skips 312 cached files and costs **183.0 s**, of which 180 s is one
known-failing test hitting its timeout. Failure set is identical to standalone `node --test` runs.

The important correctness finding: Bun's native runner (`bun test`) produced **36 failures** on files that
pass under `node:test` (chat-ui alone: 34/6 under `bun test`, 40/0 under `node --test`). Bun is therefore the
entry point (`bun run node --test`) and Node executes the tests. All `test*` package scripts now start with
Bun; `bun test/selftest.mjs` covers the runner itself (5/5). Final run: `bun test/run.mjs --all` →
323 selected, 313 cached, 10 ran, 0 passed, 10 failed (all pre-existing), 182.0 s.

### FIX-5 — Relay PWA repeat loads

Files: `apps/mobile-relay/sw.js`, `apps/mobile-relay/index.html`,
`apps/mobile-relay/test/service-worker.test.mjs` (9 tests).

The relay PWA's worker was a 173-byte no-op. It is now `vivim-relay-shell-v1`: install precaches 15
scope-relative shell paths plus the three fonts with `cache: 'reload'`; activate prunes older
`vivim-relay-shell-*` caches; fetch is network-first and intercepts only GET + same-origin + (navigation or
an exact shell path). `/api/`, pairing-token URLs, `Authorization`, `Range`, non-GET and cross-origin never
reach `respondWith`, and only `ok` + `basic` + non-redirected responses are stored. Scope is derived from
`registration.scope`, so the same file works at `/sw.js` and at a Direct Host mount under `/mobile/`.

Two font preloads were added for the faces the first screen actually shows (22,288 B + 29,400 B); the third
face (40,404 B) stays lazy. `font-display: swap` is unchanged.

Repeat-load shell chain that is now served from Cache Storage when the network is down or slow:
`index.html` 25,797 + `app.js` 39,309 + `visual-bootstrap.mjs` 9,270 + `control-phone.mjs` 8,733 +
`build-canvas.mjs` 8,813 + transport/welcome/pairing/enrollment 17,317 + 3 fonts 92,092 + generative-ui
43,037 = **244,368 B** of shell per load. Cache body: 202,002 B unique assets (the document is stored under
both `/` and `/index.html`).

Verification: `cd apps/mobile-relay; bun run node --test test/service-worker.test.mjs` → 9 pass, 0 fail.
Whole directory: 117 tests, 113 pass, 4 fail — all four are pre-existing `transport.test.mjs` failures,
proven by stashing only the two changed files and re-running (same 4), then restoring. Real service-worker
registration, offline relaunch and the font-preload effect remain owner device acceptance.

### FIX-6 — Bridge command queue: incremental reads

Files: `studio/src/studio/bridge-commands.mjs`, `studio/test/unit/bridge-commands.test.mjs` (9 tests),
`tools/perf/bench-bridge.mjs` (new benchmark).

The queue is append-only JSONL and every `list`/`get`/`claim`/`histories` re-read and re-parsed the whole
history. The store now keeps a per-file parsed Map with `{size, mtimeMs, tail}`:

- unchanged file → cache hit, no read;
- grown file → read only `[oldSize, newSize)`, after verifying the 64 bytes before the old end still match
  the cached fingerprint;
- shrunk file, changed fingerprint, or same size with a different mtime → full reload;
- a final line without a newline is left unconsumed until a later append completes it.

`bridgeStoreCacheStats()` exposes `{entries, hits, misses, incremental}` for the tests and this audit.

`bun tools/perf/bench-bridge.mjs --records 5000 --samples 6` (identical workload before/after):

| Operation | Before p50 | After p50 |
|---|---|---|
| `list queued (limit 200)` | 166.6 ms | 2.4 ms |
| `list all (limit 50)` | 131.7 ms | 1.2 ms |
| `histories (200 ids)` | 131.6 ms | 37.4 ms |
| `claim one` | 355.4 ms | 13.6 ms |

At 20,000 records (after): `list queued` 10.1 ms, `list all` 6.4 ms, `claim one` 16.8 ms. `histories`
is still 203.3 ms because it deliberately builds per-id revision history by scanning the file; it is a
local inspection API, not the relay/worker hot path, and was left alone.

The relay sync cycle and the 3 s bootstrap tick now pay milliseconds instead of hundreds of milliseconds
that grew with command history. Existing claim/reply semantics and the on-disk format are unchanged.

## 4. Surface notes and findings not changed

### 4.1 Studio web — World page

- The World shell `index.html` is a single 145 KB document with all CSS and ~1600 lines of JS inline and
  parser-blocking (`studio/src/studio/ui/index.html:364`). Compression fixes the transfer; the parse cost
  remains. Moving the non-critical render logic into a deferred module is the next win, but it touches the
  most-shared UI file and needs a real-browser pass, so it was not changed here.
- First paint issues several uncached list reads (`choose()` in `world-view.mjs`): `/api/v1/world/...`,
  `/api/status`, `/api/sessions`. With FIX-2 the library scan is no longer the dominant cost; coalescing
  them into one initial payload is a design change, not transport.
- `/api/status` p50 rose 9.8 → 19.8 ms in the post-fix run at 0 sessions. The post-fix run also computed
  ETags for JSON; this is inside run-to-run variance on this host and is not a regression claim either way.
  Repeat measurement is the honest state.

### 4.2 Studio web — capture view

- `setInterval(() => loadStreams(), 2000)` in `index.html:1956` polls `/api/sessions/{id}/streams` only
  while recording and on the Streams view. Raising it to 5 s and gating on `document.hidden` is a safe
  follow-up; not changed because it alters live-capture feedback timing and needs a live check.
- `connectLive()` opens `EventSource('/api/live')` at load. It is one persistent connection, not a poll;
  moving it behind first interaction would delay live capture state. Not changed.

### 4.3 Phone

Measured payload inventory (bytes, from the files themselves):

| Surface | Assets | Total |
|---|---|---|
| Studio mobile | `mobile.html` 6,486 + `mobile.mjs` 84,320 + manifest 1,142 + SW 2,919 | 94,867 |
| Relay PWA shell | `index.html` 25,797 + `app.js` 39,309 + `transport.mjs` 6,989 + `visual-bootstrap.mjs` 9,270 + `control-phone.mjs` 8,733 + `build-canvas.mjs` 8,813 + `welcome.mjs` 5,404 + `pairing-link.mjs` 1,464 + `device-enrollment.mjs` 3,460 + manifest 234 + icon 221 + 3 fonts 92,092 | 201,786 |
| Relay PWA generative chain | `generative-ui/*` (phone-surface, renderer, compiler, catalog, semantic-projection, design-composer, bootstrap-adapters, portable-digest) | 43,037 |

- Studio mobile now transfers 26,518 bytes (FIX-1) and its service worker already precaches the shell
  only, never API responses (`studio/src/studio/ui/mobile-sw.js`).
- The relay PWA's `apps/mobile-relay/sw.js` was a no-op: `install` → `skipWaiting`, empty `fetch`. Every
  reload re-downloaded the shell, fonts and modules. FIX-5 replaces it with a versioned network-first shell
  cache; the 244,368 B shell chain is now served from disk when the network is slow or down.
- Three `@font-face` files totalling 92,092 bytes are declared with `font-display: swap`
  (`apps/mobile-relay/index.html:13-15`). FIX-5 preloads the two faces the first screen shows and leaves the
  third lazy.
- Interaction honesty is already good: send, tab switch, model pick and theme changes set local state and
  render before awaiting the network (audited in `mobile.mjs` and `app.js`). No tap handler awaits the
  network before showing feedback.

### 4.4 Reply completion

`studio/src/capture/conversation-browser-adapter.mjs:711-745` polls every 250 ms and declares a reply
`ready` only when the content digest is unchanged for `replySettleMs` = 2000 ms, across at least 2 stable
polls, with provider controls idle. That is a deliberate ~2.25 s floor after the last text change so a
late code block or table body is never cut off (comment at lines 715-716). It was **not** shortened: the
reaction-speed target (U7 ≤ 5 s after the last change) is already met, and reducing the window would trade
a correctness guarantee for ~1.5 s. If it is ever revisited, the safe lever is `replyPollMs` (250 → 150),
not the quiet window.

### 4.5 Harness pre-send path

Traced, not changed (needs the owner's signed-in browser for acceptance):

- `harness/src/core/cdp.mjs:31` opens a fresh tab, polls `document.readyState` up to 40 × 500 ms, then
  sleeps a **fixed 1500 ms** even when the page is already complete. Removing the fixed sleep when
  readiness was observed is the single clearest pre-send win for a new-chat send.
- `harness/src/core/models.mjs:155-198` waits up to 15 s (250 ms polls) for the model picker, then opens,
  reads and closes it per send. A warm `models.json` still pays the discovery round-trips because the
  current selection has to be read from the page.
- `harness/src/core/cdp.mjs:28` waits up to 3 s for the send button to become enabled, but exits on the
  first ready poll.
- Provider selection itself is not a delay: `listProviders()` is in-memory and `decide()` is filesystem +
  pure scoring, no network.
- `studio/src/studio/provider-performance.mjs` is read-only over saved jobs and is the offline baseline for
  provider reply latency (median/p90); it cannot prove current sign-in state.

### 4.6 Host command and state path

Traced, not changed:

- `studio/src/studio/bridge-commands.mjs` now reads only the appended tail (FIX-6). Measured at 5,000
  records: `list queued` 166.6 → 2.4 ms p50, `claim one` 355.4 → 13.6 ms; at 20,000 records `list queued`
  is 10.1 ms. `histories` remains a deliberate full scan (203 ms at 20k) because it builds per-id revision
  history and is a local inspection API, not the relay/worker hot path.
- `apps/vivim-host/conversations/live-projection.mjs:87-101` re-reads and re-parses every event file on
  every snapshot/changes request. Same class of fix, same reason for deferral.
- `studio/src/studio/conversation-events.mjs:65,76,106` recomputes a SHA-256 of the reply text (head-capped
  at 16,000 chars) for every watched conversation every 200 ms. Bounded, but it is host idle CPU that
  scales with subscribers.

### 4.7 Startup

`studio/bin/studio.mjs` imports every command module before parsing the first argument:
`node studio/bin/studio.mjs --version` takes **3.38 s** and `info --json` **5.38 s**, of which
`collectorSourceSha()` alone is **433 ms** (recursive read of `studio/src`). Lazy command registration would
cut the fixed CLI cost; it is a larger refactor of the registry and was not attempted in this pass.

## 5. Reproduce

```powershell
# baseline / after, 0 sessions
bun tools/perf/bench-http.mjs --label baseline --samples 25
bun tools/perf/bench-http.mjs --label after-compression --samples 25

# library scaling
bun tools/perf/bench-http.mjs --label scaling-200 --samples 8 --seed-sessions 200

# encoder unit tests
cd studio; bun run node --test test/unit/http-encode.test.mjs
# session cache tests
cd studio; bun run node --test test/unit/session-list-cache.test.mjs
# World UI route still serves the exact bytes
cd studio; bun run node --test test/unit/chat-ui.test.mjs
```

Raw receipts: `tools/perf/results/baseline.json`, `after-compression.json`, `scaling-0.json`,
`scaling-200.json`, `session-cache-200.json`, `concurrent-8-200.json`, `final.json`. Each record carries
revision, platform, sample count and per-route p50/p95/bytes/encoding/revalidation.

Independent verification run by the audit (not the implementing lane):

| Command | Observed |
|---|---|
| `cd studio; bun run node --test test/unit/http-encode.test.mjs` | 10 tests, 10 pass, 0 fail |
| `cd studio; bun run node --test test/unit/chat-ui.test.mjs` | 40 tests, 40 pass, 0 fail |
| `cd studio; bun run node --test test/unit/session-list-cache.test.mjs test/unit/conversation-api.test.mjs` | 20 tests, 20 pass, 0 fail |
| `cd studio; bun run node --test test/unit/bridge-commands.test.mjs` | 9 tests, 9 pass, 0 fail |
| `cd apps/mobile-relay; bun run node --test test/service-worker.test.mjs` | 9 tests, 9 pass, 0 fail |
| `cd studio; bun run node --test test/unit/windows-launcher-probe.test.mjs` | 1 test, 1 pass (`dry run finished in 3906 ms`) |
| `bun tools/perf/bench-http.mjs --label after-compression --samples 25` | table in FIX-1 |
| `bun tools/perf/bench-http.mjs --label concurrent-8-200 --samples 24 --concurrency 8 --seed-sessions 200` | concurrency note in FIX-2 |

## 6. Launcher startup (Windows)

Measured defect: `VIVIM.ps1 -DryRun` took **18.46 s** wall on this host when the audit started (the lane
later measured 24.79 s on the same loaded host). The cost was port discovery, not the launcher logic. A TCP
connect to a closed loopback port costs ~2 s on Windows (`Test-Port` measured 2016-2436 ms per closed port),
and `Get-Health` made an `HttpWebRequest` with a 2500 ms timeout *before* checking the port, then called
`Test-Port` again on failure. `Find-Studio` probes five candidate ports, so the dry run paid roughly 11-12 s
of pure waiting before doing anything useful. A second, independent cold-start cost lived in the same
function: `New-Object Collections.Generic.List[int]` measured 1045-6091 ms per cold PowerShell process,
versus 4-67 ms for the `::new()` type literal.

Fix (`studio/scripts/windows/VIVIM.ps1`): `Get-Health` consults the OS listener table first and never builds
an HTTP request for a closed port. The table is one `GetActiveTcpListeners()` call cached for 500 ms (the
start loop polls a port that only begins listening during the loop, so a stale snapshot is never trusted).
Only wildcard and 127.0.0.0/8 listeners count, because those are the ones a `127.0.0.1` connect reaches.
`Find-Studio` also uses `[Collections.Generic.List[int]]::new()` instead of `New-Object`. Timeouts, 401 →
`locked`, non-1 `apiVersion` → `other`, ownership checks and every DryRun preview line are unchanged.

The old TCP probe had a correctness defect of its own: connecting from an ephemeral port equal to the
target produced a self-connect (`netstat` showed `127.0.0.1:38730 -> 127.0.0.1:38730` in `TIME_WAIT`), so
a closed port could be reported open. A listener-table lookup cannot do that. The lane also re-checked the
four state mappings end-to-end with throwaway local servers: listening VIVIM + token → "already running"
exit 0; 200 non-VIVIM → `other` → blocked error exit 1; 401 without token → `locked` → blocked exit 1;
closed → start preview exit 0.

A/B on the same loaded host, alternating old/new, internal markers (excludes PowerShell process startup):

| Variant | `Find-Studio` (5 closed ports) | internal total to exit |
|---|---|---|
| old (TCP connect + HTTP probe) | 11.09 s | 15.07 s |
| new (listener table) | 0.69 s | 3.92 s |
| old | 12.07 s | 19.95 s |
| new | 1.86 s | 9.63 s |

Port discovery is **6-17x faster** and no longer scales with the number of dead candidate ports. The
dedicated test `studio/test/unit/windows-launcher-probe.test.mjs` runs the real launcher with two
OS-chosen closed ports and reported `dry run finished in 3906 ms` (1/1 pass); `platform-boundary.test.mjs`
stays 3/3. The test's wall clock is diagnostic and only enforced when `VIVIM_PROBE_CEILING_MS` is set, so
shared-host load cannot produce a false failure.

Remaining fixed costs, measured: PowerShell process startup (1-6 s under load), `Find-Node` (`Get-Command`)
0.3-2.9 s, `node --version` 0.5-1.7 s, and `vivim start` additionally runs
`resolve-start-config.mjs` (0.6 s) and a second Node process. After the fix the lane observed 13.4-15.0 s
wall on a 100%-loaded host (`n=3`), and one instrumented run in a brief lull: **5.95 s wall with 2.44 s
in-script**. A final quiet-host stopwatch is still pending because the test-suite lane saturated this
machine during the audit; the internal A/B table above is the load-independent result.

## 7. Limits of this audit

- No browser render timing was taken. The audit measures bytes, server latency, process time and code
  paths; it does not claim a Lighthouse score or a phone frame rate. A real-browser and real-phone pass
  remains the owner's live acceptance.
- No provider send was performed. Harness and capture findings are code-traced with file:line and need a
  live signed-in run to become accepted numbers.
- Synthetic sessions contain `session.json` only. Real sessions add raw segments; `includeRawBytes` was
  deliberately left uncached, so a real library will be slower than the seeded benchmark on that one route.
- This host was already running other work while measuring. p50/p95 comparisons use the same tool, route,
  seed and revision; treat small deltas (< 10 ms) as noise.
