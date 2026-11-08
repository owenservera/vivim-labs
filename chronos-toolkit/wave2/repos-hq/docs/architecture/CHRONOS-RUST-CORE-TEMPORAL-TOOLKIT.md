# VIVIM Chronos — canonical temporal service utility (v0.1)

**Status:** core source change proposal; production wiring NOT accepted. **Owner:** canonical Repos HQ Rust core and existing Host/supervisor effect authority. **Scope:** desktop, Android, natural-language commands, user-facing tasks and conversations, internal fleet coordination, automation, graph/context, browser companion, telemetry.

## Era 1 — Contract and source integration

VIVIM has **one temporal vocabulary**, not one hardware clock. Each device has an OS wall clock and a process/boot-local monotonic clock. Chronos exposes injectable readings, temporal calculations, immutable occurrence IDs, bitemporal events, lifecycle projections and read-only compute operations. Existing Host remains the effect owner; existing supervisor owns durable job admission and idempotency claims; existing Rust HQKG graph owns source-grounded temporal relations. There must be no second universal scheduler, agent bus, publisher, graph, browser executor, policy engine, automation controller or credentials store.

Source home is **existing** `research/rust-knowledge-graph/` in Repos HQ, exported through `hqkg::chronos`, installed optionally as a `ComputeEngine` in the **existing** `EngineRegistry`. Default Rust build stays std-only, with no web provider APIs or network code. This crate is still isolated research, not already-installed VIVIM service. Deployment needs product owner approval, Host integration, rollback, test receipts and physical device acceptance.

### Temporal identity layers

| Layer | Value | Semantics |
|---|---|---|
| Physical wall | `utc_ms` (`i64`) | UTC epoch milliseconds; discontinuities may occur after OS correction |
| Monotonic | `monotonic_ms` (`u64`) + `boot_id` + `source` | Elapsed within same source/boot; never compare across restarts or devices |
| Calendar | `tzid` (IANA), local civil fields, offset, TZDB revision | **Future adapter**; not implemented in std-only Chronos v0.1; must resolve DST folds/gaps explicitly |
| Logical | `seq`, `revision`, durable state version | Causal/order evidence; never infer chronology from wall timestamps alone |
| Bitemporal | `occurred_at_ms` and `recorded_at_ms` | When an event happened/was valid vs. when VIVIM learned/recorded it |
| Schedule | `(owner_scope, schedule_id, revision, scheduled_at_ms)` | Stable occurrence/claim identity; not itself an authority grant |

Clock quality should be represented explicitly. OS-wall source without sync/uncertainty evidence reports uncertainty **unknown**, not zero. Reject extrapolation across boot boundary. A single central time authority means common semantics + authenticated source policy, **not** a false claim that a phone and PC share an atomic clock.

### Six production-grade temporal operations

1. `now` — time sample with source/boot identity; separate timezone presentation adapter.
2. `duration`, `elapsed`, `window`, `deadline` — bounded, deterministic calculation and comparison.
3. `next` — next due UTC occurrence, in a pure rule evaluator; optional `end_ms`, `max_runs` and policy.
4. `due_candidates` — bounded proposals only; Host authorizes and supervisor claims before any effect.
5. `project_timeline` — replayed lifecycle: start, pause, resume, stop, complete, fail, cancel, observed, due.
6. `capabilities` — versioned discovery for CLI/agents/Host/mobile; unknown operations fail closed.

Source also provides `TimeWindow` half-open semantics `[start,end)`, deadline grace and explicitly bounded catch-up. No private data is read or captured by these computations.

### Safe CLI

```text
cargo run --manifest-path research/rust-knowledge-graph/Cargo.toml --bin hqkg-time -- now
cargo run --manifest-path research/rust-knowledge-graph/Cargo.toml --bin hqkg-time -- duration 1000 2500
cargo run --manifest-path research/rust-knowledge-graph/Cargo.toml --bin hqkg-time -- deadline 1794300000000 30000
cargo run --manifest-path research/rust-knowledge-graph/Cargo.toml --bin hqkg-time -- next 10000 60000 70001
cargo run --manifest-path research/rust-knowledge-graph/Cargo.toml --bin hqkg-time -- capabilities
```

Every result is a bounded `vivim.chronos.cli-result/1` JSON object. This CLI deliberately cannot register automations, send AI messages, grant permission, or start local agents. That integration belongs in the existing Host typed-command gateway once authorization and persistence are verified. Core `EngineRequest` operations are stable (`now`, `duration`, `deadline`, `next`, `window`, `elapsed`), data-only, and exposed as `utility:chronos` / `vivim.chronos/1`.

## Era 2 — Full-spectrum consumer integration matrix

| Consumer | Core timestamps/evidence | Temporal behavior | Writer/authorization owner |
|---|---|---|---|
| Personal task / reminder | created, revised, due, snoozed, completed, cancelled | reminders, deadline status, recurrence, bounded misfires | Host task store/approval, phone notification adapter |
| Voice note → task | captured, transcribed, interpreted, reviewed, approved | extract requested dates with explicit timezone/ambiguity review | existing voice/task pipeline; no auto-execution |
| AI conversation | thread opened, first/user turn, send claim, response start/end, idle, closed | response latency, elapsed talk time, follow-up at user request | Host broker/thread provenance and send-once boundary |
| Browser/provider actions | grant issued/expires, action claimed, timeout, uncertain send | expiry and reconciliation triggers | existing Host/browser adapter |
| User sessions | connected, foreground/background, paused, resumed, disconnected | foreground duration, return intervals, inactivity timeout | authenticated session owner |
| Internal agents | launched, heartbeat, checkpoint, lost, recovered, completed | lease health, TTL, retries, cooldown and budget tracking | existing supervisor and lane leases |
| Multiwave workflows | admitted, dependency complete, waiting, blocked, released | critical path, wait time, deadline and SLA | existing supervisor DAG/admission |
| Automation rules | rule created/paused, occurrence due, claimed, executed/skipped | interval, calendar, recurrence, bounded missed runs | existing automation scheduler and Host effect owner |
| Files / source data | observed, valid, indexed, invalidated, deleted | freshness/retention, data TTL, graph as-of query | canonical Rust HQKG and policy |
| Context, memory, learning | source time, effective time, last read, last refresh | TTL, temporal relevance, expiration, revision invalidation | existing graph/context pipeline |
| Studio / TierFlow | clock state, active span, upcoming due, progress | projections for calendar/timeline/stopwatch/timer UI | UI adapter, **never** authoritative clock writer |
| Paired Android phone | phone monotonic span, notification due, sync observed | local countdown while offline; online reconciliation | phone OS alarms + Host policy and effect authority |
| Diagnostics / KPIs | span start/end, latency, throughput, error time | monotonic durations, percentiles, scheduled audits | current telemetry pipeline |
| External calendar/email | received time, meeting start/end, RSVP | projection and explicit owner-approved edits | connected app/provider auth via Host; no direct core APIs |
| Plans and projects | milestone start/end, dependencies, blocked/unblocked | forecasts as **estimates**, critical-path windows, review cadence | project/workflow owners |
| Access, approvals, grants | issued, expires, revoked, renewal due | expiry checks and fail-closed clock uncertainty | existing security/Host policy |

### Required cross-surface host contract (proposed, not yet wired)

```json
{
  "schema": "vivim.chronos.request/1",
  "requestId": "stable-unique-id",
  "ownerScope": "authenticated-owner-scope",
  "operation": "schedule.preview",
  "subject": {"kind": "task", "id": "task-42"},
  "temporal": {"atUtcMs": 1794300000000, "tzid": "Europe/Madrid"},
  "policyRevision": 12,
  "expectedRevision": 7,
  "dryRun": true
}
```

For **mutation**, the Host must check caller scope at actual write/send boundary, durable schedule revision, idempotency, fresh permissions, revocation, expiry and replay before registering or dispatching. Dispatch outcomes must produce evidence records distinguishing `DUE`, `CLAIMED`, `ADMITTED`, `APPROVED`, `EXECUTED`, `REFUSED`, `SKIPPED`, `UNCERTAIN`; a due event does not imply any of those later states. Schedule identity and records must not carry raw prompts, private document text, tokens or authentication secrets.

### Lifecycle event envelope (proposed adapter)

```json
{
  "schema": "vivim.chronos.event/1",
  "eventId": "uuid-or-stable-event-key",
  "subjectKind": "conversation",
  "subjectId": "thread-42",
  "kind": "started",
  "seq": 1,
  "occurredAtUtcMs": 1794156800000,
  "recordedAtUtcMs": 1794156800014,
  "sourceRef": "host:builtin-session-ledger",
  "sourceClock": {"bootId": "host-boot-42", "monotonicMs": 60000, "wallUncertaintyMs": null},
  "policyRevision": 12,
  "provenanceRef": "existing-host-receipt"
}
```

As-of projections filter events not known at the query time and must signal missing coverage, out-of-order delivery, gaps, invalid transitions and clock uncertainty. Do not infer a conversation was read or a message was sent from elapsed wall time alone. Retention and deletion rules must propagate to event projections and caches.

### Schedule and restart semantics

- Persist rule, owner scope, schedule revision, timezone intent, next cursor, policy, cancellation/revocation, event receipts and successful claim outcome **in the existing authorized durable store**.
- Use unique `(owner_scope, schedule_id, revision, at_ms)` occurrence keys and a transactional claim fence. Two Host workers cannot acquire the same live claim.
- Treat a crash after external effects but before receipt as `UNCERTAIN`, not auto-retry without reconciliation. Non-idempotent sends never blindly repeat.
- `Skip`: no backlog, allow an explicit 1000ms on-time detection tolerance in the v0.1 UTC rule utility.
- `CoalesceOne`: at most one latest due candidate for a missed window.
- `CatchUpBounded(n)`: emit oldest n candidates and retain a precise cursor so remaining occurrences are not silently lost.
- Device off/offline: phone may manage **phone-owned** OS reminders. It cannot assume authority for Host-owned action effects; sync produces a reconciliation receipt before future execution.
- Device clock anomalies: wall time jump, monotonic discontinuity, changed timezone, missing TZDB evidence, revoked grant, stale policy revision => explicit degraded/refusal state; not silent `GO`.

### Calendar/DST and timezone gate

The std-only v0.1 Rust code intentionally accepts **UTC milliseconds and fixed-duration interval rules only**. It does not implement IANA local recurrence, cron, weekday rules, leap-second policy, DST folds/gaps or OS alarm persistence. Before user-facing `every day at 09:00` can activate, introduce a bounded, verified timezone adapter (e.g. an opt-in established Rust timezone library) with explicit ambiguity policy and versioned TZDB data. A 24-hour interval is **not** the same as 09:00 local daily across DST. Keep `America/New_York`, `Europe/Madrid` and other IANA names as validated calendar inputs in that adapter; reject ambiguous time without user policy.

## Era 3 — Production-closeout implementation waves

### Wave 0 — Source integration (this change)

- New `src/chronos.rs`, `src/bin/hqkg-time.rs`, `tests/chronos.rs` in the **existing** Rust crate.
- Add `pub mod chronos;` to `src/lib.rs` and a bin target to the existing Cargo manifest.
- Keep graph/engine/supervisor/broker behavior unchanged; no auto-install or new daemon.
- Run `cargo test --manifest-path research/rust-knowledge-graph/Cargo.toml` and `cargo test --manifest-path ... --features importer` on a Rust-equipped runner.

### Wave 1 — Adapter and durable state, existing owners only

- Map existing internal task, supervisor, conversation, browser/voice, Host receipt and phone schedules to the event envelope.
- Add Host-owned persisted registration/pause/resume/cancel APIs, CAS revisions and exact idempotency claim keys. **Do not** copy authorization or implement parallel job runner in Rust.
- Add schema conformance, `--json` command dispatch, read-after-write, restart and kill/replay tests. Preserve pure Rust compute and source-of-truth ownership.

### Wave 2 — Calendar, mobile, user journey

- IANA timezone/weekday/cron adapter behind a feature gate; DST fold/gap and TZDB update fixtures.
- Android OS notification/AlarmManager acceptance with revoked permission, Doze, reboot, clock skew and offline Host reconciliation, physical device required.
- Natural-language temporal parsing into validated typed Host commands with explicit approval; voice-to-task Day 0 acceptance; Studio/TierFlow projection only.
- End-to-end journey: owner speaks "remind me tomorrow at nine about the sales call"; transcript -> reviewed task/timezone -> durable schedule -> phone reminder -> review/complete -> paired desktop timeline with receipts.

### Independent release gates

- No duplicate Host executor or Rust graph, no unexpected default feature/dependency.
- Bounded deterministic clock injection; no monotonic comparison between boots.
- No data-only time query can cause writes, sends, browser control or permissions changes.
- Event identity replay, as-of history and source provenance reject invalid and duplicate input.
- DST, timezone, missed-event, cancellation, revocation, clock jump and offline tests explicitly passed before activating corresponding features.
- Supervisor dependency admission, send-once, receipt recovery, rollback and real Windows + Android physical acceptance independently verified.
- No feature marked production `GO` based only on a GitHub source commit or local fixture result.

**Current acceptance:** source proposal only, not production-enabled. If GitHub integration is unavailable, keep source as a separate patch artifact and do not claim it is on `main`.

## 2026-11-08 — Ecosystem integration wave 2 (staged)

This follow-up adds read-only temporal integration to four existing consumer paths, without activating a new effect executor:

- Rust pure utilities: `freshness` (including clock-skew states), `heartbeat`, `retry` bounded exponential delay, `due_preview` returning a cap-resumable proposed cursor. All leave effect authority to existing Host and supervisor.
- Existing supervisor ledger: `hq/lib/chronos/supervisor-timeline.mjs` uses `replayEvents` as the canonical validator and offers only wall-clock-estimate spans with explicit drift diagnostics. `hq/bin/chronos-inspect.mjs` reads an explicitly supplied bounded events file; no writes or sends.
- Existing Conversation Runtime: additive source-proven `createdAt`, `updatedAt` and per-turn stamps, plus GET `/api/v1/conversations/{id}/timeline` using the already existing conversation API gate and pure `projectConversationChronos`. No follow-up send and no fabricated response latency.
- Existing TierFlow Host read layer: `projectChronosConversationRead` and `projectChronosSupervisorRead` strictly whitelist read-only metadata, refuse forged effect permissions, and produce inert UI projection.
- Existing SOFTWARE_HQ: `scripts/chronos-verify.mjs` performs explicit cross-repository test commands and reports PASS/FAIL/UNRUN with raw exit-code evidence, without CI or background work.

**Not delivered:** durable schedule claims/registration, actual service activation, calendar/DST/IANA resolution, OS alarms or persistent Android notifications, autonomous reminder delivery, shared cross-device authority, physical-device acceptance. The route's complete authenticated Host integration and all Cargo tests still require an actual local checkout/toolchain. Never equate a candidate occurrence or metadata timeline with an executed action or delivered reminder.
