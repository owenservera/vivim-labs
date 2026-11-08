# Day-0 voice-to-tasks — ChatGPT-owned multi-lane execution ledger

**Date:** 2026-11-08. **Authority:** Product requirement [Repos HQ #11](https://github.com/owenservera/repos-hq/issues/11). **Local production-closeout #10 continues unchanged.** **Repository written:** PUBLIC `owenservera/vivim-labs`, isolated under `vivim/voice-to-tasks/`. No private source, original recordings, transcripts, credentials, tokens or owner identity were copied here.

## Concurrency and implementation policy
- Five logical independent lanes. They are *not* five spawned agents, ongoing background jobs or concurrently running systems. ChatGPT performed the read-only inspection and test-workbench implementation during this session. No local agent requested/started/stopped/preempted.
- One GitHub writer performed sequential additive commits on Labs main, without changing files of active automation/agentic Labs PRs; no branch or PR created. No production repo write lease, Host/Chrome/phone access or task-effect grant was asserted.
- The workbench **is a reference/evaluation program** only: zero model weights, zero external dependencies, no app runtime, no Host writes, no provider APIs and no network calls.
- Any future runtime promotion goes via the existing single owner/integrator after explicit file leases and independent device tests; this document is not authorization.

## Workstream board

| Lane | Bounded ownership | Executed deliverable | Status now | Real next gate |
|---|---|---|---|---|
| A — capture + ASR contract | metadata envelope, source hashes and consent | [`workbench/core.mjs`](workbench/core.mjs) `checkCapture`, `checkTranscript`; negative fixtures | **TESTED SYNTHETIC** | Confirm Android raw M4A path and actual authorized Host corpus upload; real PCM decode + ASR output test |
| B — task intelligence | multi-action extraction, source grounding, timezone | [`core.mjs`](workbench/core.mjs) baseline, `interpretDuePhrase`, `checkProposals`; multilingual/DST/negation tests | **TESTED SYNTHETIC** | Model-backed en/es/ca transcript samples with human labels, date ambiguity F1 and critical-token error |
| C — owner authority | review intent, source/ref binding, idempotency key proposals | [`core.mjs`](workbench/core.mjs) `prepareOwnerReview`; unknown/duplicate/unresolved proposal refusal | **INERT CONTRACT TESTED** | Identify existing canonical task store / typed Host mutation. Authenticate grant at ACTUAL Host effect boundary; cross-process replay tests |
| D — mobile/desktop | verify expected state transitions and Host receipt provenance | [`workbench/trace-check.mjs`](workbench/trace-check.mjs) + negative tests | **OFFLINE TRACE CONTRACT TESTED** | Real Android APK + desktop UI, paired transfer, approve/reopen/restart, keyboard/TalkBack, denied mic and dropped connection |
| E — evaluator | deterministic golden fixture tests, CLI and reproducibility | [`workbench/evaluate.mjs`](workbench/evaluate.mjs) + [10 fixture cases](workbench/fixtures.json) + [10 test groups](workbench/workbench.test.mjs). CLI red/green repaired | **TESTED SYNTHETIC** | Real 100+ utterance/language benchmark, WER/date negations/task F1, p50/p95, RSS, battery, install delta and licenses |

## Source-grounded reuse map (read-only inspection)
- `vivim-web-harness/studio/android/src/inc/vivim/app/PocketRecorder.java`: Android foreground-only M4A, recoverable pending original, 16 kHz AAC / 64 kb/s, duration/storage bounds and permission gate. Already code-present, **not** a speech-to-task feature.
- `vivim-web-harness/studio/android/src/inc/vivim/app/PocketStore.java`: device-local source capture and evidence storage; explicitly *no automatic network or Host effects*. Need verify actual record-to-Host intent, grants and lifecycle before reuse.
- `vivim-web-harness/apps/vivim-host/corpus/host-integration.mjs`: already existing Host corpus bridge with device, scope, owner approval, per-event rights and signed intake receipts. Reuse only after review; transfer must not be inferred from a recorded file.
- `vivim-web-harness/apps/vivim-host/control/dispatcher.mjs` and `control/journal.mjs`: existing plan/approval/claim/effect/receipt owner boundary. The current `control/action-catalog.v1.json` inspected does **not** contain a personal task creation action. **Task write authority is not found in that action catalog**; investigate whole app before treating this as missing globally.
- `vivim-web-harness/schemas/chat-task.v1.json`: ChatGPT broker-task envelope, **not** a personal to-do schema; do not reuse its name as proof of personal task storage.
- `vivim-web-harness/studio/src/studio/world-note-units.mjs`: unit/source projection for World notes, not itself an authorized to-do mutation engine.

**Attribution:** those are source-inspected paths on remote repository main during this session. No remote GitHub read proves local binaries, process leases, APK build/device behavior, signed-in account, model download or runtime capability. The private code itself was not mirrored into Labs.

## Baseline red / green: executable evidence
- **Red:** initial compact fixture CLI invocation of `evaluate.mjs` raised `CAPTURE_INVALID` because raw JSON cases were not expanded to synthetic envelopes.
- **Fix:** CLI-only `expandSyntheticFixture`, with deliberate `NOT-AUDIO:` synthetic hash prefix, no actual raw audio; added executable spawned-CLI regression test.
- **Green:** `node --test /mnt/data/vivim-voice-workbench/workbench.test.mjs` — **10/10 subtests pass, exit 0** on Linux Node.js 22.16.0 in the isolated ChatGPT container; no external dependency.
- **Green:** `node /mnt/data/vivim-voice-workbench/evaluate.mjs /mnt/data/vivim-voice-workbench/fixtures.json` — compact fixture CLI exits **0**. 10 synthetic text-only cases, 12 explicit gold action spans matched, zero false positives *on this tiny hand-curated corpus*. **These are not ASR, generalization, model F1, performance or product acceptance scores.**
- Published GitHub blobs matched independently tested local files by **Git blob object SHA** (comparison of actual content bytes):
  - `core.mjs` `24120e5214833e4365ef9ad76ec7fe40f6339ef2`
  - `evaluate.mjs` `c9b965a2f2a8d0d71bb6c54d61502a1cfa8483b5`
  - `fixtures.json` `49c0a1fab872353aa24c5861f5dccf71bbefe32d`
  - `trace-check.mjs` `af25ab69cae6169c27246414b7ab52c60d2b5c78`
  - `workbench.test.mjs` `4b9e4ee123f98e3face0ce853b4ca41570e91fb6`

For an independent clean checkout of Labs:
```sh
node --test vivim/voice-to-tasks/workbench/workbench.test.mjs
node vivim/voice-to-tasks/workbench/evaluate.mjs vivim/voice-to-tasks/workbench/fixtures.json
```
Node.js 22+ recommended; no `npm install`.

## Implementable contract restrictions and omissions
1. `checkCapture` rejects unknown fields/raw audio embedded in JSON, malformed metadata, invalid mime/size/hash/timezone and non-boolean consent flag. It is **not** a cryptographic authorization to upload.
2. Baseline uses narrow explicit verb recognition, punctuation segmentation and en/es/ca tokens. It **will not reliably capture** indirect/free-form tasks, quoted speech, complex conjunctions, names, code-switching, or all negations. Candidate proposals are always review-only, never authoritative.
3. Calendar-only exact today/tomorrow/day-after tokens resolve from capture timezone and timestamp; unrecognized phrases are clarification-required, not guessed timestamps. No automatic reminder time or external calendar effect.
4. `prepareOwnerReview` emits an inert `host_authorization_required` plan and stable candidate idempotency keys; it does NOT validate a real Host grant, persist anything, or prevent cross-process effects on its own.
5. `verifyObservedTrace` checks a simplified idealized sequence; it does **not** prove actual C43 phone transport, Host receipts, Android UI or crash/replay.
6. A 10-case synthetic 100% score is a smoke fixture check only. Production correctness and speed remain **UNRUN**; no false release GO.

## Next independent, lease-safe ChatGPT rounds
**Round 2 (research + fixtures, no other team interference):** extend hard-negative transcript cases, model-output proposal conformance fixtures, measurable corpus protocol; verify ASR checkpoint licenses and quantization assets. Use the existing evaluator rather than another engine.

**Round 3 (read-only source reconciliation):** find actual task mutation/storage and note identity integration; map the smallest Web Harness/Repos HQ diff, with exact touched paths, risk and rollback. Stage patches for the sole integrator, do not write shared runtime paths without owner clearance.

**Round 4 (hardware-dependent blocked gates):** native APK M4A → ASR + editable review; authenticated Host-only task commit; phone↔desktop reopen; physical Android permission/airplane mode/TalkBack; independent p95/RSS/battery and deletion/revocation. Gate stays **BLOCKED/UNRUN** until real observations.

## Release status
This work **reduces design/contract uncertainty and produces tested reusable pure functions**, not an end-to-end shipped feature. Personal task authority, full ASR, authenticated phone path, real Android and full release are **BLOCKED/UNRUN**. Continue production closeout exactly as planned, with no duplicate Host or browser writer.

## Local implementation round (2026-11-08, branch `voice-workbench-impl`)

Executed by the local ZCode seat (zcode-main) under owner goal "full implementation"; supplements, does not replace, the ChatGPT rounds above. Scope held to ONE code file plus this ledger; `core.mjs`, `fixtures.json`, `evaluate.mjs`, `trace-check.mjs` untouched.

1. **Windows portability fix** in `workbench.test.mjs`: `spawnSync` paths now built with `fileURLToPath(new URL(...))` (was `URL.pathname`, which yields `C:\C:\...` on win32). Suite on win32 node v24.22.1: was 10 tests / 9 pass / 1 fail / exit 1 — now **14 tests / 14 pass / 0 fail / exit 0**.
2. **Round-2-shaped hard-negative test group added (4 tests):** false-positive traps drawn from this ledger's own omissions list (quoted speech, past regret, questions, indirect forms, conditionals, third-person schedules, cross-locale verbs), explicit negation refusals including the documented conservative `don't forget X` false-negative — pinned so any future widening of the verb/negation logic is a conscious change, never an accident — plus locale-mismatch refusal, span grounding and idempotency determinism on a mixed transcript.
3. **Acceptance on this branch:** `node --test vivim/voice-to-tasks/workbench/workbench.test.mjs` → exit 0 (14/14); `node vivim/voice-to-tasks/workbench/evaluate.mjs vivim/voice-to-tasks/workbench/fixtures.json` → exit 0, tp=12 fp=0 fn=0 f1=1 (unchanged from the published baseline).
4. **Still BLOCKED/UNRUN, unchanged and owner/hardware-gated:** model-backed ASR samples, real 100+ utterance multilingual benchmark, authenticated Host task authority (no task-creation action exists in `action-catalog.v1.json` — verified locally), Android APK + device tests, phone↔desktop transport, p95/RSS/battery.
