# Execution and acceptance recipes — no effectful provider tests by default

## 0. Safety and source gates
- Refresh ARBOR `main` HEAD via GitHub; compare to any isolated local test checkout and remote tracking branch. The user's local ARBOR HEAD was `5b129485783c` while online main was `84ca3c36` when this packet was made. Do not reset it, force-push, or let an old local checkout author source.
- Inspect active `TASK-CLAIMS.md` claims and Dev-owned paths; use exact GitHub file SHA. Keep code work in ChatGPT cloud/GitHub; DC cannot silently edit product source.
- Confirm all tests are no-effect/no-provider-send before launch. Source may change at any point.

## 1. Deterministic reproducible verification — Z01/Z02/Z07
Existing local source-pinned runner (already installed, not a new system):

```powershell
$ops='C:\0-BlackBoxProject-0\vivim-labs\ZCODE-DESKTOP-COMMANDER-OPERATIONS'
powershell -NoProfile -ExecutionPolicy Bypass -File "$ops\scripts\vlang-schedule.ps1" -Action status
Get-Content "$ops\local-logs\native-verifier\latest.json" -Raw
Get-Content "$ops\local-logs\vlang-iterations\latest.json" -Raw
powershell -NoProfile -ExecutionPolicy Bypass -File "$ops\scripts\vlang-zcode-pilot.ps1" -Action status
```

Expected: `sourceSha == actual current origin/main`, hash-bound Rust binary and test runner, exactly four PASS lanes, 215 baseline test count **only when observed**, zero source writes/effects/provider sends, `releasePromoted=false`. A Windows Task Scheduler **Running** state is not PASS. Earlier receipt at `7b9e7dbd` is historical. The existing hourly observer can be separately triggered through `vlang-schedule.ps1 -Action run`, without launching model jobs.

## 2. ChatGPT provider bridge — A01–A08 / #32
- Read-only classify historical uncertain key and busy Studio owner; no replay and no restart. Confirm single guarded worker from exact pinned source and identical Studio controller/browser ownership; test correct composer type/readback and thread identity.
- **Only upon a fresh owner-submitted authorized prompt**, use a fresh idempotency key, verify one send, capture provider settled response, Host timeline/relay receipt, source revision, and actual phone rendered/ACK receipt. A 200/202, worker heartbeat, Studio sign-in, synthetic DOM fixture or duplicate request does not pass.
- Separately check failure/uncertainty/cancellation; avoid sending more than one prompt to resolve a missing receipt.

## 3. Native GPUI/PWA shared surface — #31, S02–S14, U01–U09
- Freeze package from one source with `tools/packaging/prepare-host-runtime.mjs` using same-SHA Rust target(s), PWA and Studio artifacts; lock manifests, SHA256 manifest and rollback. No mixed native/Host SHA.
- Launch a new isolated candidate only after verifying no conflicting live Host owner; retain existing paired phone URL and shortcut until promotion proof. Test real native GPUI typing/IME/focus/menu and Wiki-as-you-type source revision.
- Observe PC composer revision appearing in Android; then phone-origin revision appearing in GPUI; verify equal Host source/view/epoch revision and explicit ACK, including concurrent edit/stale readback and refresh.
- Screen snapshots must be actual target-window composited/native capture; Win32 menu handles, screenshot source, local HTTP root or mock DOM fixtures are weaker evidence than human interaction/phone render.

## 4. VLANG/AutoWiki/Oracle — U05–U09, O01–O06
- No LLM needed: existing Host NLCL grammar classifies `create wiki page notes` as draft candidate, `edit wiki section notes/tasks saying item` as candidate, `stop all traffic` as clarification/denial; verify current source before expecting exact responses.
- Ambiguous, negated, unsupported, R2/R3 or unsafe commands must not execute from ambient typing. Accepted plans go through existing typed Host approval → effect → receipt only after owner approval.
- Wiki must derive from actual permitted sources and show source revisions; deny revoked/grant-incomplete/cross-user neighbors; stale responses may not overwrite newer state. Personal corpus requires explicit authorization, provenance and correction/deletion tests.
- Shared VEC/LiveSurface projection required; no parallel TierFlow front end, provider API, parser, world database, planner, or second Host authority.

## 5. CI + operations — S12 / R04
- Investigate `startup_failure` and zero allocated GitHub Actions jobs as infrastructure failures until a real runner executes. Do not mark product source tests failed from lack of runner; report actual run/job IDs when available.
- Report code, test, package, served/runtime and device/phone status separately; close #33 checkbox only with exact current source receipt and the required device proof.
