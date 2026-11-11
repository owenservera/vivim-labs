# ChatGPT-first onboarding and continuous personal-knowledge workflow
Date: 2026-11-11
Status: DESIGN CONTRACT — not production implementation or verified end-to-end wiring.
Owner: Host for authority and durable personal data; Studio for signed-in browser effects; shared Rust kernel for computation; TierFlow for projections; Labs for fixtures.

## Outcome
A user signs into ChatGPT once, receives a useful provisional personal profile without waiting for an archive, optionally authorizes selected conversations, and later supplies an official export ZIP. VIVIM produces source-backed Personal World facts and task-specific Dynamic Context Bundles (DCBs), updates them without duplicates, and supports corrections and revocations. Memory-off AI sessions are assumed.

## First-run state machine
States: NOT_STARTED -> ENV_READY -> PROVIDER_CONNECTED -> GRANTS_REVIEWED -> QUICK_BOOTSTRAP_PENDING -> QUICK_BOOTSTRAP_RECONCILED -> FIRST_KNOWLEDGE_READY -> FIRST_DCB_READY -> ACTIVE. Optional branches: EXPORT_REQUESTED -> ARCHIVE_SELECTED -> ARCHIVE_VALIDATED -> BACKFILL_RUNNING -> BACKFILL_RECONCILED. Any stage may become PAUSED, ACTION_REQUIRED, FAILED_RECOVERABLE, or REVOKED. UI should never represent EXPORT_REQUESTED as EXPORT_READY; neither imply sidebar discoveries comprise all account history.

Transitions and receipts:
1. ENV_READY: Host confirms local World writable, browser profile identity, storage reserve; proof: preflight receipt.
2. PROVIDER_CONNECTED: Studio confirms exact signed-in ChatGPT target and qualified account identity; user handles login/MFA; proof: account binding receipt, no credentials copied.
3. GRANTS_REVIEWED: Host records separate scopes for bootstrap prompts, visible thread metadata, selected thread content, local ZIP backfill, ongoing capture, outbound DCB transmission and retention. Sign-in does not grant capture. Each grant is versioned and revocable.
4. QUICK_BOOTSTRAP_PENDING: Host shows exact F02/F01 prompts and their destination and requests explicit approval. Scheduler reserves one Studio-owned effect; uncertain sends must reconcile rather than replay.
5. QUICK_BOOTSTRAP_RECONCILED: Studio response and Host journal reconcile against exact prompt digest/thread, with provisional candidate extraction; incomplete response remains partial, not success.
6. FIRST_KNOWLEDGE_READY: user sees evidence-linked profile and can accept, edit, dispute, suppress, or delete candidate assertions.
7. FIRST_DCB_READY: compose minimal purpose-specific bundle from permissioned accepted evidence; preview audience/omissions/source/freshness; Host checks the exact destination at send time; Studio sends and binds receipt.
8. ACTIVE: onboarding finishes regardless of archival status. Backfill jobs never block using VIVIM.

## UX surfaces and actions
Welcome: Begin / Connect ChatGPT / Continue without account.
Connect: provider account, dedicated Chrome identity and connection status.
Source Permissions: independent toggles for bootstrap, discover index, select conversations, ZIP import, future approved captures, AI outbound delivery; retention controls.
Learning Now: provisional profile, why each assertion was inferred, review and correction; don't present percentages as empirical completeness.
Bring History: Request export instructions / Choose ZIP / Select conversation(s) / Later.
Import Center: item counts by stage, selected source, byte size, time and resource estimates explicitly labeled estimates, pause/resume/retry, duplicate/skipped, coverage limits.
Knowledge Explorer: person/projects/decisions/preferences/timelines with evidence drilldown, contradiction states, temporal validity, sensitivity.
Context Preview: task, intended provider/account/thread, included facts, excluded facts, effective policy, source freshness and estimated token budget.
Control Center: grants, revoked sources, stop capture, disconnect keep history, delete all derived, owner export and replay/recovery receipts.

## Import worker (local ZIP, no provider API)
Host receives user-selected archive handle (never uploads archive to Labs or provider). Preflight: file signature/type and path safety, archive bomb limits, enough disk, encrypted/local retention policy. Create job ID and content fingerprint; parse a manifest and incremental conversation objects, preserving native IDs, source account, timestamps, message roles, parent/branch relationships and attachment omissions. Do not use World transfer's 200-message limit as an archive pipeline.

Bounded batches: validate -> stage raw evidence -> normalize content units -> deterministic entity/reference extraction -> optional AI-derived candidate claims (explicit grants) -> resolve identities/conflicts -> compute Personal graph scoped changes via shared Rust DAG -> Host CAS/persist -> refresh read indexes -> receipt/cursor. The batch receipt must be durable before cursor advance. Crash/restart should be idempotent. Stop before next batch on revoke, low disk, account mismatch or unexpected schema.

Prioritization: recently active threads and user-selected high-value threads first, but label ranking as heuristic. Imported original remains distinct from candidate model interpretation. Duplicate detection uses provider account + stable native ID + message revision/content digest; same title is insufficient. Two sources making the same claim remain separate evidence attestations.

## Knowledge semantics
Content observation != user assertion != accepted personal fact != verified external outcome. Preserve valid time, observation time, ingestion time, extractor version, confidence and evidence anchors. A later contradiction does not silently overwrite a previous claim. Decisions require positive acceptance; implementation requires observed operational proof. For sensitive data and third parties, default to narrower disclosure.
Dynamic Context Bundles are derived read-only views, not a second permanent knowledge store. Each bundle includes owner, purpose, audience, destination, source evidence refs, inclusion/omission manifest, freshness, scope and digest; invalidate on revisions, corrections and revocation.

## Ongoing lifecycle
Event triggers: a newly authorized Studio capture, a user-selected folder change, another local archive import, user correction, expired assumption or pending conflict review.
Scheduler: coalesce events; inspect power/disk and priority; classify work as immediate or deferred; run bounded deterministic parsing and only permitted enrichment; never retry browser effects blindly. Owner sees clear 'updated from...' receipts and a digest of changed facts instead of ongoing prompts.
A recurring context request starts naked: task classified -> relevant graph/evidence retrieval -> permission filter -> minimal DCB assembly -> optional user review -> exact destination authorization -> Studio browser send -> receipt -> selectively ingest new response as provisional candidate evidence, avoiding circular self-confirmation.
A revoked source must stop ingestion and forbid future inclusion immediately; deletion then removes raw/derived/index/cache material according to selected action, leaving minimally necessary audit tombstones where legally/technically appropriate. Already-sent external messages cannot be withdrawn.

## Interface boundaries and no-duplicate rules
Host onboarding/service.mjs: coordinator/state/grants/scheduler; do not fork a second onboarding service.
Host bootstrap/service.mjs and scheduler.mjs: approved F02/F01 execution and reconciliation only, never implicit full-history grants.
Studio bootstrap-owner-adapter.mjs and conversation-browser-adapter.mjs: exact signed-in browser target and authorized send/capture; do not introduce scraping through undocumented private endpoints.
Studio chatgpt-chat-index-reader.mjs: partial visible metadata; no claims of archive completeness.
Studio world-service.mjs/world-bootstrap.mjs: Personal World reads and bounded existing data paths; identify actual durable owner before inserting worker.
Shared Rust typed_dag.rs/scoped_graph.rs: typed transforms and scoped revisions, no second personal graph engine or foreign-namespace writes.
TierFlow: UI projection and corrections dispatched to Host authority, never an independent personal writer.
Labs: synthetic anonymized fixtures and test evidence only.

## End-to-end acceptance script
New user -> local preflight -> ChatGPT sign-in -> approve F02/F01 exact prompts -> provisional knowledge review -> first evidence-bounded DCB -> approve exact ChatGPT send -> request export -> continue using VIVIM -> choose ZIP -> interrupt after batch 2 -> restart -> complete import -> reimport same ZIP with zero duplicate identities -> import a later conflicting assertion -> preserve both and review -> revoke ZIP source during a batch -> prove query/cache/send exclude revoked content -> delete derived knowledge -> inspect audit receipts.
Required test status per segment: DESIGN_ONLY / SOURCE_IMPLEMENTED / INTEGRATED / AUTOMATED_TESTED / REAL_ACCOUNT_VERIFIED / DEVICE_VERIFIED. Never imply a stronger status.

## Prioritized implementation work packets
P0 Source-to-writer wiring matrix at exact SHAs, account identity and grant schema; prove single Host durable writer.
P1 Local ZIP parser with fixtures, manifest and security checks; dry-run preview.
P2 Batch checkpoint/replay receipt and idempotent normalization into existing Personal World evidence.
P3 Shared Rust scoped Personal DAG apply plus read projection; deterministic graph diff.
P4 Candidate extraction, chronology, contradiction and acceptance UX.
P5 TierFlow source chooser/import/progress/coverage/evidence UI.
P6 Scoped DCB composer connection to Host send-time authorization and Studio effect receipt.
P7 Continuous captured-conversation delta ingestion, schedule, revocation and deletion cascades.
P8 End-to-end real-account, crash, disk, privacy and device acceptance.

## Release invariant
First use does not depend on historical export arrival. Every retrieved personal knowledge assertion has provenance and permission. Every external disclosure is destination-scoped and verified at send time. Every job is resumable and duplicate-safe. UI never claims that source-implemented functionality is product-verified.
