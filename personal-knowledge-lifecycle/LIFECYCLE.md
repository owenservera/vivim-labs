# End-to-end lifecycle: first boot to long-lived personal intelligence

## State machine
`uninitialized -> local_identity_ready -> source_discovered -> consented -> bootstrap -> initial_value -> backfilling -> steady_state -> [paused | degraded | recovery] -> steady_state -> [disconnected | exporting | deletion]`

A source can be in a different state from the personal World. Account connection, data import and permission are independently versioned; import does not block first useful task.

### Era A — installation, trust and initial bootstrap
0. **Preflight:** confirm device/storage, encrypted user-owned data location, disk headroom, migration strategy, clock/timezone and crash-safe journal; no secret in logs.
1. **Identity:** create local owner identity and World namespace; obtain explicit source-specific grants with read range, data class and allowed retention. Separate optional model-disclosure grants.
2. **Discovery:** enumerate *metadata of user-selected* local folders, browser profiles and supported signed-in providers without silently scanning content. Surface account identities carefully; do not merge two accounts on display name alone.
3. **Value-first seed:** collect explicit user intentions, current task and selected small data sample; make first search and context result work before bulk import.
4. **Bootstrap:** import recent/high-value sources first (e.g., selected conversations, active documents, current projects), preserve immutable originals or stable source references plus digest.
5. **Identity and semantic reconciliation:** cross-source entity candidates, timelines, duplicate detection, owner review for ambiguous merges; classify sensitive and third-party data.
6. **Coverage report:** what sources/date ranges were indexed, missing files and unsupported adapters, verified vs inferred knowledge, next user action.
7. **Nonblocking background batches:** user-visible bandwidth/CPU/storage budget, rate limits, pause/resume, per-source checkpoint and processing receipts.

### Era B — history/backfill and convergence
8. Use checkpoints by source, export revision, content digest and cursor; ingest bounded atomic units.
9. Parse native structural objects (threads, messages, attachments, folder trees, commits, meetings) before chunking text. Keep a lossless parent/child map and offsets.
10. Deduplicate exact bytes plus provider object IDs; separately detect semantically similar evidence. Never merge source identities just because summaries overlap.
11. Extract claims, entities, relationships, preferences, decisions, goals and outcomes; AI extraction yields *proposals* validated against anchors.
12. Reconcile temporal state (occurred, observed, recorded, accepted, superseded); preserve contradictions, dissent and uncertainty.
13. Build scoped graph change sets using Rust DAG; provenance, revision CAS, transaction/journal and invalidation are mandatory.
14. Maintain full-text/semantic indexes as projections with ability to rebuild from permitted evidence; content-sensitive embeddings stay local by default.
15. Offer owner review for significant identity decisions, inferred preferences and risky/private claims. Do not gate harmless ingestion on enormous manual queues.
16. Produce completeness/quality diagnostics and a graceful stop when disk, RAM, battery, permissions or external sessions are constrained.

### Era C — continuous capture, learning and retention
17. **Observe** approved source changes: local watch events, export refreshes, authorized app sessions, accepted user notes and Host action receipts. Do not assume every provider offers webhooks.
18. **Coalesce and prioritize** active task impact, freshness, source reliability and user expectation; never repeatedly reprocess unchanged input.
19. **Incrementally update** only affected graph nodes, dependency descendants, summaries and DCB caches.
20. **Cross-AI continuity:** compose local, purpose-bound DCBs assuming receiving AI memory is empty; Host performs destination permission checks and send-once receipt binding to actual browser thread.
21. **Feedback:** incorporate explicit corrections, verified action outcomes and rejection/confirmation events. Avoid converting transient chat language into stable preferences without evidence.
22. **Maintenance:** periodic health, source drift, backfill gaps, dedupe audits, stale claims, consent expiry and replay verification. User-visible quiet state, not compulsive notifications.
23. **Account changes:** handle provider rename, sign-out, permission changes and duplicate accounts while retaining namespace-safe lineage.
24. **Reunion:** after months or device restart, reconstruct minimal working context from trusted local state without requiring cloud-native model memory.

### Era D — withdrawal, portability and eventual retirement
25. **Pause:** stop new capture; preserve resumable source cursor as permitted.
26. **Disconnect:** revoke new reads and active tasks, then apply chosen data-retention policy to historical evidence.
27. **Delete source:** tombstone content and every derived reference in graph, search, cache, summaries, DCBs, exports and restore/replay eligibility; prevent resurrection through stale snapshots. Audit with non-content receipts only.
28. **Owner export:** loss-minimizing portable source manifest, graph edges, evidence pointers, provenance, accepted corrections, decisions, policies and schema version; include explicit exclusions for sensitive/unexportable content.
29. **Migrate/recover:** schema migration with deterministic replay and rollback; no silent re-authorization.
30. **Delete World / uninstall:** explicit clear-choice flow; remove local encryption keys/data according to product policy and report what cannot be revoked in already-sent provider messages or external backups.

## Important UX principle
Initial value is an independent release gate from import completeness. Display *coverage* rather than vague percentage; e.g., 18/20 selected conversations parsed, 2 unsupported attachments, 3 tentative identity merges. Never silently declare import complete.
