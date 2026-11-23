# P0-AI-CHAT — No-effect Studio readiness handoff R003

**Observed source:** ARBOR `dfb5480b17f5daa8861352e0cb81dd97e01e9f9d`; **physical service observation:** 2026-11-23 Europe/Madrid, Desktop Commander connected to the owner PC. **Status:** Studio readiness seen, but provider turn/Host receipt/phone render still **NOT VERIFIED**. This document is a read-only evidence addendum to the existing `P0-AI-CHAT` package; it is not permission to send a message, reopen a session, start a worker or transfer its claim.

## Actual read-only observation

The incumbent local `studio/src/studio/relay-chat-transport-readiness.mjs` probe was run through the existing local `.private/relay-readiness-live.mjs` helper. It returned:

```json
{"ready":true,"reason":"STUDIO_OWNER_IDLE","supportedProviders":["chatgpt"]}
```

This **only** means the Studio status GET reported recorder idle, its sole browser connection available/verified, a ChatGPT conversation target registered, and all returned conversations were passive with `writer.active === false` according to that read snapshot. It is **not** verified ChatGPT account sign-in, correct current tab/composer identity, sent turn, settled reply, retained history or mobile visibility. A later snapshot can change. The helper's process-level nonzero exit convention must not override the explicit readiness fields or be called a full PASS.

The local Host root reported HTTP 308; Studio root HTTP 200. Neither HTTP root response is protected Host/Studio provider acceptance.

## Source and safety gates (exact existing implementation)

- `studio/src/studio/relay-chat-transport-readiness.mjs`: loopback-only GET to `/api/status`, `/api/v1/conversation-targets`, `/api/v1/conversations`; fails closed on active/missing writer, unknown state, browser unverified or unavailable API. **Do not expand allowed sites from ChatGPT based on configured adapters alone.**
- `studio/src/studio/relay-chat-startup-fence.mjs`: immutable read of full append-only bridge queue and boot watermark; torn tail, race, corrupt IDs fail closed. Existing historical command IDs are excluded from new worker claims.
- `studio/bin/relay-chat-mvp.mjs` and `studio/bin/lib/relay-chat-studio-channel.mjs`: incumbent worker/transport only; source HEAD and worker PID/heartbeat must be verified again before any new turn.
- `apps/vivim-host/conversations/studio-journal-evidence.mjs`: Host source-linked journal evidence projection (send/observed/reply/uncertain) requires matching exact real source; no provider “sent” inferred from queue entry or HTTP 202.
- `apps/mobile-relay/app.js` + incumbent provider handoff: mobile target/real conversation UI is separate from status-only Studio card and may be stale relative to backend.
- **CRITICAL local-only trap:** the helper named `.private/reconcile-studio-sessions.mjs` includes POST `/api/v1/conversations/{id}/reconcile` mutations. **It is NOT a read-only probe; DO NOT execute automatically during SITREP or unattended ZCode verification.** The helper's private IDs and contents must not be copied into GitHub, Labs or issue comments.

## Next ready-to-run, no-effect operations — one owner, no provider sends

1. Refresh latest ARBOR main and `#32` + current `TASK-CLAIMS.md`. No active other agent's Studio source collision, no duplicate browser owner.
2. Re-run only the **existing read-only** Studio readiness probe; capture fields `ready`/`reason`, source SHA, timestamp and server status, never raw prompt/conversation text or account tokens.
3. Read existing bridge queue metadata/startup fence and the uncertain ChatGPT request identifier **without resending**. Old `CREATION_UNPINNED/TYPING_UNCONFIRMED` remains quarantined. Do not run POST reconcile on busy/unknown conversations as a “health check”.
4. Confirm a **single** guarded worker and its exact executable/source, boot fence and no old command claim. If browser, account or worker identity changed, return BLOCKED and do not start a competitor.
5. Verify actual current selected provider tab and correct composer/thread identity using Studio-owned read-only inspection. A global `STUDIO_OWNER_IDLE` result is **insufficient** for step 5.
6. After (1–5) and **only** for a fresh user-submitted prompt, permit exactly one new key and one Studio-owned send; keep the old key permanently fenced. On uncertain result do not retry.
7. Observe settled reply and exact thread continuation; then Host relay receipt with source revision; then physical GPUI and Android render/ACK. Record each independent state, don't promote earlier stages.

## Concrete test and acceptance matrix

| Test or event | Current status | PASS requirement |
| --- | --- | --- |
| Read-only Studio global readiness | **OBSERVED** `ready:true / STUDIO_OWNER_IDLE`, one snapshot | Repeat at actual admission time, same browser owner/source identity |
| Old uncertain key quarantined | Historical source/fence exists; current ledger not enumerated here | No replay, one fenced old ID, no hidden queue claim |
| Current worker PID + startup fence | **UNVERIFIED** in this handoff | One worker, exact source, fresh heartbeat and no old queued claim |
| ChatGPT selected tab/composer verified | **UNRUN** | Exact tab+thread+composer readback via incumbent Studio |
| Actual signed-in ChatGPT account state | **UNRUN** | Studio owner demonstrates correct signed-in target, not just target catalog |
| Fresh owner-submitted prompt | **NOT SUBMITTED** | One new approved request with new idempotency key |
| One provider send and settled reply | **UNRUN** | Provider-side thread plus one send + completed response, no duplicate |
| Host relay readback | **UNRUN** | Exact matching turn/thread, Host event/receipt |
| Native GPUI conversation render | **UNRUN** | Physical native UI receipt, no second UI |
| Android PWA render/ACK | **UNRUN** | Same source/epoch, actual on-device acknowledgement |

Source-safe deterministic checks can test the existing startup fence and readiness parser independently but must not call browser mutation, provider send, full historical session reconcile, or ZCode session wake. No second worker. No paid-model fallback.

## Agent assignment and stopping conditions

**ChatGPT Dev owns P0-AI-CHAT** and any `studio/` or Host source changes; Desktop Commander performs owner-authorized Windows read-only verification; Oracle audits receipts; ZCode may run only deterministic, no-effect checks. This packet offers a **readiness observation**, not an accepted source-writing task claim.

**STOP immediately** on writer.busy/unknown, wrong browser controller, possible old-key replay, ambiguous composer/thread, changed credentials, stale source, unapproved provider spend, uncertain prior new turn, or duplicate worker. Do not force-stop a busy browser or alter sessions to make readiness green.

Receipt: `ARBOR_SHA | STUDIO_SOURCE_ID | STUDIO_OWNER_READY | BROWSER_VERIFIED | WORKER_PINNED | OLD_KEYS_FENCED | TARGET_THREAD_PINNED | MESSAGE_SUBMITTED | REPLY_SETTLED | HOST_RECEIPT | GPUI_RENDERED | PHONE_RENDERED | BLOCKER`. Mark non-observed fields **UNRUN**.

## R004 verified guarded worker cutover — 2026-11-23 19:52 UTC

This R004 observation **supersedes the older “worker PID unverified” row in the R003 snapshot above**, without changing any provider/Host/phone acceptance claims.

**Source code:** [ARBOR `516a23e8`](https://github.com/owenservera/ARBOR/commit/516a23e826e812bea7e86c1bad64683d0c495923) added the **existing** `relay-chat-mvp.mjs` worker's read-only Chrome channel preflight and [`relay-chat-channel-preflight.mjs`](https://github.com/owenservera/ARBOR/blob/main/studio/src/studio/relay-chat-channel-preflight.mjs), alongside [five new negative tests](https://github.com/owenservera/ARBOR/blob/main/studio/test/unit/relay-chat-channel-preflight.test.mjs). The worker requires BOTH existing Studio owner readiness and the configured `127.0.0.1:38702/json/version` browser endpoint, with `Chrome/*` identity and matching loopback `ws://.../devtools/browser/*`, **before** publishing `ok` heartbeat or claiming a bridge command. No second controller, user account credentials or external provider API. HTTP metadata is liveness, **not** a genuine send guarantee.

**Actual Windows tests:** `node --test --test-concurrency=1 studio/test/unit/{relay-chat-channel-preflight,relay-chat-transport-readiness,relay-chat-startup-fence}.test.mjs` **17/17 PASS**, zero skips/fails at source `516a23e8`, repeated **17/17 PASS** against fast-forwarded ARBOR `f44ae9b56d49616260dc7252bb7a3b8a8a27ac37`. Exact tested worker blob `7d377a0a2b192f863a3f386acac03343bba91771` and guard blob `ad1d7f247a612db8d704b316199f07962599e8df`. Separate source lanes had concurrent HEAD changes; rollout required matching tested blobs, clean tracked checkout, and no existing source collision.

**Real running state:** At preflight Studio `recorder=idle`, `browserConnection.available/verified=true`, zero active conversation writers; existing Chrome endpoint metadata matched `127.0.0.1:38702`. Existing single relay worker PID `6968`, nine historical bridge commands (5 done, 2 failed, 2 queued, **none claimed**). A first worker-only deployment attempt using the repository root as CWD **failed safely** with `MODULE_NOT_FOUND` (the launcher expects `ARBOR/studio` as working directory); no provider claim occurred. Corrected startup used the original command arguments with the **existing** Studio subdirectory as CWD, source `f44ae9b5`; NEW PID **`17188`**, live `chatWorker` heartbeat `outcome=ok` rechecked at `2026-11-23T19:52:20.527Z`. Startup stderr explicitly reports **9 historical command IDs ignored**, 2 queued remain unclaimed, no new bridge commands or history mutations during rollout. No Host, Studio browser or phone URL restart/change. Exact private runtime receipt: `C:\\VIVIM\\bridge\\health\\chatWorker-cutover-20261123.json`; logs stay local and are not public.

**Evidence ladder:** `SOURCE_PUSHED=PASS`, `SOURCE_TESTED=17/17 PASS`, `STUDIO_OWNER_READINESS=OBSERVED`, `GUARDED_WORKER_LIVE=PASS_AT_OBSERVATION`, `OLD_QUEUE_FENCED=9/9`; `PROVIDER_MESSAGE_SUBMITTED=UNRUN`, `SETTLED_PROVIDER_REPLY=UNRUN`, `HOST_RECEIPT=UNRUN`, `NATIVE_GPUI_CHAT_RENDER=UNRUN`, `ANDROID_PHONE_RENDER=UNRUN`. No historical uncertain turn was replayed. The user still needs a fresh, explicitly submitted prompt for actual one-send/settled-reply validation; a healthy worker cannot certify that outcome.

**Reusable launch invariant for Dev/Windows operator:** For the *existing* `studio/bin/relay-chat-mvp.mjs`, working directory must be `<ARBOR>/studio` when invoking relative `bin/relay-chat-mvp.mjs`. Do not confuse repository root with Studio root. Before any restart, require source blob pin, clean tracked checkout, exactly one known worker, no claimed commands, Studio idle/writer false, loopback CDP identity, and old-command boot fence. Preserve the original data root `C:\\VIVIM`, `--studio-url http://127.0.0.1:38730 --transport direct --studio-channel --cdp-port 38702`, and current paired phone URL. **Never start a second worker while one is active.**
