# ChatGPT-first onboarding: Harness / Studio investigation and timeline
Date: 2026-11-11. Source-level findings; no live browser, signed-in account or device test performed.

## Investigation source map
- `apps/vivim-host/onboarding/service.mjs`: single guided onboarding coordinator, providers list, ChatGPT primary, approval review and scheduling, studio readiness, timed pump (~4000ms), F02/F01 passes. Existing approvals are **bootstrap prompts**, not permission to ingest full account history.
- `apps/vivim-host/onboarding/environment.mjs`: OS/Chrome/Edge/Node preflight and dedicated browser profile defaults, CDP port candidate.
- `apps/vivim-host/bootstrap/service.mjs`: trusted local personalization writer with bounded grants, 20-send maximum and up to 24-hour expiry for *new* grants; legacy unbounded grants remain a migration/privacy audit item.
- `apps/vivim-host/bootstrap/scheduler.mjs`: ChatGPT verified readiness, exact prompt digest approvals, one Studio-owned effect, journal reconciliation; uncertain effects must not be resent.
- `studio/src/studio/bootstrap-owner-adapter.mjs`: exact-once reservation, fresh Studio conversation, submit, inspect, durable binding; no blind replay.
- `studio/src/capture/chatgpt-chat-index-reader.mjs`: read-only single exact-target CDP session; rendered sidebar index is partial and not verified account history; no scroll, navigation, private HTTP, capture, or session creation.
- `studio/src/studio/conversation-api.mjs`: `POST /api/v1/chat-index`, conversation create/connect/turn/read/finish and event surfaces. Browser adapter remains owned by Studio.
- `studio/src/capture/conversation-browser-adapter.mjs`: exact-target/URL binding, idle mode until explicit send, capture only when armed and protections for unsent drafts.
- `studio/src/studio/world-service.mjs`: saved sessions and imported-snapshot World conversations, metadata-only when evidence unread, qualified provider identity; not an archive importer.
- `studio/src/studio/world-transfer.mjs`: bounded World import/export; `IMPORT_CONTENT_LIMITS.maxMessages=200`, so do **not** feed multi-year ChatGPT ZIP directly to it.
- `studio/src/studio/world-bootstrap.mjs`: read-only view of Host bootstrap SQLite preferences/claims/passes; inferred claims not activated.
- `studio/src/agents/personal-agent-continuity.mjs`: locally assembled continuity only, CANON/Immediate/Curated and Work; not a remote ChatGPT history sync and not itself transmitted.

## Proposed two-clock user journey
Timings below are **design targets only**, not measured product performance or promises. Manual login, MFA, hardware, connectivity, and provider changes vary.

| Relative time from first launch | User experience | Owner | Acceptance evidence |
|---|---|---|---|
| T+0–1 min (target) | Host preflight, local World, data directory and browser readiness | Host | explicit readiness codes |
| T+1–4 min (target) | Dedicated Chrome sign-in to ChatGPT, user-led MFA | Studio | verified signed-in observation, no credential collection |
| T+4–5 min (target) | Choose permissions: first-run personalization, optional visible-chat discovery, historical ZIP import, ongoing capture, separate AI disclosure | Host | independently scoped grants |
| T+5–8 min (target) | Preview and approve exact F02/F01 prompts, run one at a time and reconcile response | Host scheduler + Studio | exact prompt digest + send and ingest receipts |
| T+8–10 min (target) | Display first useful profile, selected sources, coverage and one demonstrated DCB | Host + TierFlow | source-backed World read |
| Any point after sign-in | Offer official ChatGPT export request; do not block app use | User / ChatGPT | user-entered export-request status, not guessed provider completion |
| When export arrives: up to 7 days | Select ZIP from disk; download link valid 24h after notification | Host local import worker | archive validation, content manifest |
| Import after ZIP selection | Progressive batches; recent/active conversation priority, bounded CPU/disk/memory; nonblocking | Host + Rust | checkpoint/cursor/count/coverage |
| Continuous thereafter | Only approved live conversations and explicit future exports captured or reconciled | Studio + Host | source delta and allowed-grant journal |

## Three ingestion routes
A. **First-session bootstrap**: F02/F01 prompt outputs from ChatGPT, candidate personalization through existing Host validation. Immediate value; NOT account-history import.
B. **Visible conversation discovery/capture**: read-only sidebar candidate index plus explicit authorized per-thread browser capture. Partial coverage, and no implicit mass historical extraction or private endpoint scraping.
C. **Official local export**: ChatGPT ZIP via Settings > Data controls > Export; user downloads and selects file; separate archive parser preserving graph structure, message role, branch, timestamps, attachments and omissions; no web-provider API used.

Priority: A now -> B as optional bridge -> C asynchronously. Link overlapping observations and exports by verified provider account/native thread and content digest, never title alone.

## Missing or unverified
No verified ChatGPT ZIP-to-World-to-Rust full-history path; no end-to-end real account or device acceptance; existing F01/F02 personalized results are not graph evidence of the user's entire life; chat index cannot claim completeness; cross-store deletion propagation and joined source permissions not proven.
Do not expand Studio browser-driving authority to support historical extraction. Build local archive parser on Host, use the existing Rust kernel and add TierFlow read-only progress surfaces.

## Official export rules verified 2026-11-11
https://help.openai.com/en/articles/7260999-exporting-your-chatgpt-history-and-data
Eligible ChatGPT Free/Go/Plus/Pro and some Edu. Not self-serve for Business/Enterprise/Healthcare. Settings > Data controls > Export. ZIP may arrive within 7 days; link expires in 24 hours. Modern export includes local viewer/index.html and JSON data; older exports may include chat.html. Do not assume all assets or deleted conversations recoverable.
