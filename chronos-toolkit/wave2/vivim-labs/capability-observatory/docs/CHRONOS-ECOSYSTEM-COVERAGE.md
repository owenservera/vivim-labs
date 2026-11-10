# Chronos customer + internal timing coverage — read-only integration wave

Status: staged source handoff; no Host/Android or live effects acceptance. Chronos is one **temporal interface**, not a network master clock or a second scheduler.

| Domain | Currently staged adapter | Effect owner | Outstanding acceptance |
|---|---|---|---|
| Rust core | `repos-hq/research/rust-knowledge-graph/src/chronos.rs`, `hqkg-time` | None, read-only | cargo default + importer tests on Rust runner |
| Conversations | `vivim-web-harness/studio/src/studio/chronos-conversation-projection.mjs`, GET `/api/v1/conversations/{id}/timeline` | Existing conversation runtime | authenticated Host-level end-to-end GET/POST regression |
| Internal agents | `repos-hq/hq/lib/chronos/supervisor-timeline.mjs` | Existing supervisor | real journal replay, restart/recovery, readonly inspection |
| TierFlow desktop/phone | `tierflow/src/visual-runtime/host/chronos-read-projection.ts` | Host read authority | UI rendering parity, TalkBack/keyboard, real devices |
| Cross-repo toolchain | `SOFTWARE_HQ/scripts/chronos-verify.mjs` | None | local verified exit-code evidence |
| Calendar + timezone | Deliberately NOT activated | Host/timezone adapter | IANA tzdb, DST gap/fold, weekday/cron semantics |
| Automation execution | Deliberately NOT activated | Existing Host + supervisor | scope, durable cursor/claim, revocation, uncertain-effect replay |
| Voice-to-task | Proposed consumer of typed Chronos schedule | Existing voice/approval/task stack | phone transcription->review->reminder->receipt |
| Graph/context | Rust scheduling, TTL/freshness primitives exist; no authoritative graph mutation | Existing HQKG writer | durable as-of retrieval, tombstones/revision validation |
| Security, expiry | `freshness`, `heartbeat`, `deadline` pure Rust | Existing authorization/revocation | fail-closed clock drift, stale and replay tests |

## Producer vs consumer truth

- `occurredAt` is the source's reported occurrence. `recordedAt` is when the owner journal observed it. Neither is inferred from a display clock.
- Conversation `createdAt`/`updatedAt` and turn timing are snapshot metadata, **not** verified send/reply time. The adapter exposes `responseLatencyMs: null` rather than pretending a snapshot update is a provider response.
- A supervisor replayed `EXITED` event is evidence of one recorded attempt, not a verified OS process-liveness check. All wall-duration numbers are estimates and never enforce a lease.
- Mobile/TierFlow receives only read-only typed data. No UI receipt or prediction may be promoted to an owner-granted operation.
- `due_preview` carries capped occurrences, a resumable cursor suggestion, and `effect_allowed=false`; only the Host's authoritative claim transaction may commit a cursor.

## Production blocker checklist

1. Rust build/test and CLI in actual Cargo environment (ChatGPT container did not have cargo).
2. Main writer reviews latest checkout and target-path leases before applying; no force push.
3. Host endpoint scope and broker `send-once` regression tests remain mandatory.
4. Persistent schedule table/cancellation/claim are **not** part of this wave; no background timers.
5. Native Android alarm notification and DST/timezone fixtures unrun.
6. Cross-repo schema parity + full green TypeScript/Bun test suite needed before integration GO.
