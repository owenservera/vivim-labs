# 04 — Cadence, time and behavior-based workload frequency
Based on the earlier Proposal 5 rescore taxonomy, but more explicit about idle energy, time budgets and user-controlled triggers. **Cadence in the atlas is hypothesized**, except timer mechanisms cited in code. A module containing a timer does not prove the service is currently running.

| Class | Meaning | Example / measurement |
|---|---|---|
| ALWAYS_ON | Can operate continuously while a designated service is alive | observed timers: HQ watcher, Studio flush, TierFlow frame loop; measure actual uptime, wakes/min, duty cycle |
| HIGH_FREQUENCY | Per frame, token/chunk or event during active sessions | decoder, delta, hotpath; measure max burst and steady-state |
| FREQUENT | Repeated event-triggered per receipt/journal/ingest | web memory, DAG runs; measure events/min and burst length |
| USER_DEFINED | User behavior determines activation, even when idle=0 | H active editing/chat/search; M moderate recurring use; L rare explicit runs |
| MEDIUM_FREQUENCY | incremental refresh or batching | journals/compaction when triggered; verify caller cadence |
| LOW_FREQUENCY | rare but potentially huge spike | cold compaction, integrity scan, recovery |
| DEV_ADMIN | code indexing, builds, diagnostics, admin | H active dev loops; M occasional; L maintenance |

**User H/M/L and Dev/admin H/M/L are independent attributes**. A dash denotes “not the primary use class,” not impossible. Frequencies must not be invented as requests/second.

## Two competing cost models
- **Lifetime cost:** `events_per_hour × CPU_ms_per_event`, `idle_hours × wakeups_per_hour`, sustained DB writes and energy.
- **Peak cost:** worst-case p95/p99 latency, max RSS, temporary disk, backlog, cancellation recovery. A very rare 8 GiB spike can be catastrophic on a small workstation.

## Binding time truth
Do not compare wall timestamps across devices as though they were monotonic durations. Clock policy needs:
- monotonic duration for deadlines/performance;
- UTC instant for persistent event chronology;
- source-reported clock plus offset/uncertainty for browser/device events;
- timezone/DST only for human calendar recurrence;
- producer sequence/epoch for ordering and gap detection;
- idempotency key + durable ledger to resolve possible retries;
- explicit Host clock authority rather than a new scheduler.

## Candidate distribution (NOT observed activity)
- `ALWAYS_ON`: 3
- `HIGH_FREQUENCY`: 6
- `FREQUENT`: 7
- `USER_DEFINED`: 24
- `MEDIUM_FREQUENCY`: 1
- `LOW_FREQUENCY`: 3
- `DEV_ADMIN`: 15
## Scenarios to profile
Idle desktop 1h; idle paired PWA 1h; 30min active chat/capture; high-volume synthetic stream; source refresh burst; 100k small index objects; 1M synthetic graph edges when resources permit; bad disk/ENOSPC; restart after partial journal; offline PWA with stale source. Record real machine and trace, never pretend a synthetic benchmark is a production workload.
