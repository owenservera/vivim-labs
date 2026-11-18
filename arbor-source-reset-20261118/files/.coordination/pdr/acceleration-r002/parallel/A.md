# ARBOR Acceleration R002 — Parallel W1 Lane A

**Assignment:** Host-owned journal authority, replay and revision safety
**Branch:** `work/accel-r002-w1-a-host-authority`
**Maps to:** A02 hardening; unblocks independent A02 acceptance
**Priority:** Immediate critical-path source fix
**Parent contract:** [parallel/README.md](README.md) — read it first. Do not touch `main`, do not merge, and do not change the canonical roadmap.

## Exclusive write scope

- `apps/vivim-host/server.mjs`
- `apps/vivim-host/chat/product-journal.mjs`
- `apps/vivim-host/test/product-journal.test.mjs`
- `apps/vivim-host/test/product-routes.test.mjs`
- `docs/acceleration/w1/A.md`

Only these paths are writable. All other repository content is read-only even if a nearby fix seems convenient. Produce your final report at `docs/acceleration/w1/A.md` within the branch; source path ownership is exclusive.

## Mandatory additional source inspection

- `apps/vivim-host/control/browser-session.mjs`
- `apps/vivim-host/control/dispatcher.mjs`
- `apps/vivim-host/control/http-status.mjs`
- `apps/vivim-host/public/pages/wiki.js (read-only: Lane C owns)`
- `apps/vivim-host/public/pages/chat.js (read-only)`

Read actual current contents and verify branch base; do not substitute the work-order prose for source inspection.

## Execution steps

1. Inspect the actual session and owner-authentication routes used by the canonical Host. The current Wiki write fence is Origin + a custom header and a string actor, not a verified owner grant. Bind the workbench Wiki mutation to an existing owner authority, or if not possible in the scoped source, refuse privileged mutation fail-closed and provide a precise cross-lane UI requirement. Never invent an auth token or turn the browser actor string into permission.
2. Retain one dataRoot-private append-only mutation journal, with no secondary store or publisher. Validate typed envelopes before entry, bound payload sizes, actor scope, revision/CAS checks, and forbidding path/file/code effects.
3. Stress simultaneous edits (same revision), restart replay, torn-tail recovery, duplicate/invalid entries, write failure behavior and dataRoot isolation. Do not silently swallow an unsafe corrupt middle segment or claim multi-process single writer if none exists; handle the actual process model or provide a blocked evidence record.
4. Keep /api/state, /api/wiki, /api/journal and /demo/journal.jsonl observationally consistent; maintain canonical dispatcher separation and the test isolation contract. Do not touch frontend owned by Lane C.

## Run/prove (actual exits or UNRUN)

- `node --test apps/vivim-host/test/product-journal.test.mjs apps/vivim-host/test/product-routes.test.mjs`
- `node --test apps/vivim-host/test/chat-box-baseline.test.mjs (read-only; report failures not fixes)`

## Acceptance criteria

- ${s}
- ${s}
- ${s}
- ${s}

## Required final handoff

One exact branch head SHA, base/main pin, per-file change list and why, actual test commands with literal exit codes versus UNRUN, observed runtime/browser/CI environment, security/ownership checks, unresolved blockers, dependency acceptance state, cross-lane integration request(s), and the next ready frontier as read-only observation. Report a measured subsequent-work acceleration only with real evidence; otherwise `NOT_YET_MEASURED`. Do not claim production/owner acceptance based on source-only checks. Preserve all original historical receipts.
