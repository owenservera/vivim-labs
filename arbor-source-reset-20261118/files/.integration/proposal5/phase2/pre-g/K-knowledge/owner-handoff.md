# P2-K — owner handoff (max 5 selective tasks, no G dependency)

**Overall:** PREPARED_NOT_PROMOTED. **READY_FOR_W2_OWNER** means ready to delegate the following small, gated tasks only; **production import DEFERRED**. Origin: Labs main 022608f5797e7b4a14ccc4c393bd3c5a4704a4be. Observed owner branches: A dda546a11294920d6521a0f64ab7ee62ec993c70; B 78a5105a9c1f1880d658f350e130b1e2632d3532; D ad55fe28c2c331841675f308f9199ee4c2add86a. G and all Wave 1 owners may proceed without this lane.

## Delegation 1 — G/F: source pin + fixture selection (0.5 day)

**Dependency:** G completes its A–F source merge and reports exact integrated HEAD and any collision map. Verify file blob identity versus source-delta.json; assign only selected Labs synthetic ZIP/parser/lifecycle tests to F. Do not import Labs package.json, docs corpus, real exports or the whole personal-knowledge directory. Acceptance: same source pins/relevant SHA recorded; selected tests appear under existing ARBOR test authority; no product path edits from this P2-K branch. **No test result is asserted here.**

## Delegation 2 — B: explicit T=0 grant and bounded-disk preflight (0.5–1 engineering day)

**Dependency:** G and B current onboarding/World identity and Host ledger. Adapt existing Host owner approval to an explicit, time/operation/account/source scoped archive import grant; require rights/third-party/retention review and fresh account binding. Bind storage quota/reserve with Rock Core protected minimum inside current Host storage. No authority from signed-in ChatGPT, existing full-bootstrap grant, archive filename, or model text. Enforce false/expired/revoked, account swap, low disk and path mismatch with synthetic negatives. Source entrypoints: B apps/vivim-host/onboarding/service.mjs and bootstrap/ledger.mjs; Labs t0-core.mjs and zip-reader.mjs. No new DB.

## Delegation 3 — B: source-bound ZIP worker + signed publication receipt adapter (1–2 engineering days)

**Dependency:** Delegation 2 and current Host evidence storage owner. Reuse Labs zip-reader.mjs, json-array-stream.mjs, chatgpt-normalize.mjs, chatgpt-import.mjs only after assessing 8 MiB normalized staging ceiling and crash-safe invisible temporary custody. Wire a real B-owned inspectEvidenceBatch(key,digest,scope), verified receipt and checkpoint transaction. Fix Labs fallback inspection dropping digest; no optimistic commitment when inspection returns uncertain. Do not equate Host corpus manual upload grant with ZIP import grant. Test corrupt CRC after early entries, ZIP traversal, over-expansion, duplicate archive, crash after effect before checkpoint, mismatched digest, revoked grant during import, source changed after fingerprint, and low-space rollback.

## Delegation 4 — A/B: Rust graph marker and erasure bridge (1–2 engineering days)

**Dependency:** Delegation 3 evidence admission and exact current A HQKG CLI. Host authenticates principal and source provenance, calls A hqkg-personal with scope+opId+payload digest+expected epoch/bytes, validates marker then creates a durable join receipt. Stale CAS and a response-loss state must HOLD/reconcile. Add B-owned erasure fence before reimport, A erase-effects or scope erase as appropriate, and B raw/derived/index/context invalidation with minimum noncontent fence. Verify no resurrection after restart; do not assert atomic evidence+graph cross-store transaction without tests. No new graph service, policy authority or scheduler.

## Delegation 5 — D/E/F: scoped read projection and independently observed acceptance (0.5–1 engineering day plus device acceptance)

**Dependency:** Delegations 2–4; D never reads raw personal chats through developer session-index paths. Use D personal-corpus-contract for scoped read-only evidence envelope shape and F tests; E optionally displays only Host-supplied progress/source coverage/omissions after real receipt publication. Test account isolation, contested/model-authored claims remaining candidates, prompt injection as source text, revoke/erasure search and DCB invalidation, disk exhaustion/compaction, and proof of no outbound provider send. No claim of phone/Windows acceptance until F records observed runs.

## Final owner gates

**W2 task-readiness:** READY_FOR_W2_OWNER after G source merge, with owners to approve selection. **Operational activation:** DEFER until B signed evidence journal and grant checks, A graph CAS/erase, F security tests, real locally consented archive schema and Windows/device validation are actually observed. **Integration G:** NOT_WAITING, do not merge this branch, and do not cherry-pick any runtime module from here (none exists). Do not collect user data to complete this handoff.
