# R004 architecture acceptance ledger — 96 points
Canonical machine-readable tracker: [acceptance-tracker.json](acceptance-tracker.json). Architecture: [OWNER-DESIGN-R002.md](ARCHITECTURE.md). Audit: [ARCHITECTURE-AUDIT-R003.md](ARCHITECTURE-AUDIT-R003.md).

**Verified baseline: 28/96 (14/48 PASS, 3 PARTIAL, 31 OPEN).** R003 source pin f837042b76e2bd438f107a6b93360306da6d98ef; [native receipt](VALIDATION-R003.md). Prior ledger reported 30 points; E27 reduced to PARTIAL because explicit stale checkpoint write rejection was not tested. No new runtime test was run. E05 criterion unchanged; dependency gating alone is insufficient. Qualitative architecture estimates are not evidence scores.

Each gate is worth 2 points; only PASS counts. Fixed criteria and IDs; each PASS requires exact source pin, actual native test/experiment receipt, and coverage of the entire criterion. PARTIAL, OPEN, FAIL, BLOCKED earn zero. Reopen affected gates after regression or material unverified changes. No AI wiring; 36h and 72h must be actual elapsed time.

| ID | Acceptance criterion | Status | Evidence / outstanding |
|---|---|---|---|
| E01 | Goal identity survives fresh process | PASS | native actual_separate_processes_recover |
| E02 | Goal completes only when sealed and all tasks verified | PASS | native restart_fences_old_worker_and_keeps_checkpoint |
| E03 | Explicit terminal blocked/exhausted outcomes and dependency propagation | OPEN | No qualifying evidence |
| E04 | Goal hierarchy and versioned success predicates | OPEN | No qualifying evidence |
| E05 | Task DAG dependency gating and cycle prevention | PARTIAL | Dependency gating tested; cycle rejection needs explicit native negative test. Criterion unchanged. |
| E06 | Typed predecessor output binding and dynamic graph refinement | OPEN | No qualifying evidence |
| E07 | Versioned class and capability association | PASS | native promote_then_reuse_and_disable |
| E08 | All 15 behavioral operators have executable contracts and semantics | OPEN | No qualifying evidence |
| E09 | Class composition validation and execution graph | OPEN | No qualifying evidence |
| E10 | Class generation, admission and migration from evidence | OPEN | No qualifying evidence |
| E11 | Logical agent identity survives reincarnation | PASS | native incarnation_mailbox_request_replay_cancel |
| E12 | Complete incarnation records with executor binding and lineage | OPEN | No qualifying evidence |
| E13 | Autonomous genesis and replacement after failure | OPEN | No qualifying evidence |
| E14 | Agent states dormant/waiting/running/recovering/archived | OPEN | No qualifying evidence |
| E15 | Per-agent workspace and durable projections | OPEN | No qualifying evidence |
| E16 | Context references validated and independently persisted | OPEN | No qualifying evidence |
| E17 | Context assembly, typed input binding and budgeted selection | OPEN | No qualifying evidence |
| E18 | Versioned artifact payloads and provenance | OPEN | No qualifying evidence |
| E19 | Verified artifact persists and duplicate admission rejected | PASS | native restart_fences_old_worker_and_keeps_checkpoint |
| E20 | Independent deterministic output oracle rejects wrong result | PASS | native restart_fences_old_worker_and_keeps_checkpoint |
| E21 | Composable evidence, independent verifier authority and predicate oracles | OPEN | No qualifying evidence |
| E22 | Durable mailbox deduplication and recipient acknowledgment | PASS | native incarnation_mailbox_request_replay_cancel |
| E23 | Mailbox wakeups and durable event delivery | OPEN | No qualifying evidence |
| E24 | One active lease per worker and generation fencing | PASS | native busy_agent_cannot_hold_multiple_running_leases; restart_fences_old_worker_and_keeps_checkpoint |
| E25 | Global resource slots enforced across independent processes | OPEN | No qualifying evidence |
| E26 | Host monotonic/wall-clock supervision and recovery | OPEN | No qualifying evidence |
| E27 | Durable checkpoint with stale-attempt rejection | PARTIAL | Checkpoint retention and stale submission rejection tested; explicit stale checkpoint write rejection not tested. |
| E28 | Versioned checkpoint validation and fallback | OPEN | No qualifying evidence |
| E29 | Bounded retry and cancellation | PASS | native budget_exhaustion_and_explicit_failure; incarnation_mailbox_request_replay_cancel |
| E30 | Idempotent request replay across process boundaries | PASS | native actual_separate_processes_recover |
| E31 | Trusted host admission with actor authorization and effect reconciliation | OPEN | No qualifying evidence |
| E32 | Journal corruption detected and uncommitted temp ignored | PASS | native corruption_fails_closed_and_uncommitted_temp_is_ignored |
| E33 | Indexed incremental persistence, bounded journal growth and migration | OPEN | No qualifying evidence |
| E34 | Concurrent writers, crash-during-write and tail-integrity qualification | OPEN | No qualifying evidence |
| E35 | Bounded deterministic program evaluation and reusable capability admission | PASS | native promote_then_reuse_and_disable |
| E36 | Versioned capability disable and regression/rollback | PARTIAL | native disable only; full gate open |
| E37 | Automatic candidate synthesis from verified examples | OPEN | No qualifying evidence |
| E38 | Disjoint held-out validation and applicability constraints | OPEN | No qualifying evidence |
| E39 | Automatic capability matching and reuse without manual per-task assignment | OPEN | No qualifying evidence |
| E40 | Cross-goal deterministic reuse in fresh executable | PASS | native autonomous_scheduler_reuses_capability_across_goals |
| E41 | Learned procedure/repair/class/coordination/context artifacts with provenance | OPEN | No qualifying evidence |
| E42 | Cost and verified yield measurement and policy feedback | OPEN | No qualifying evidence |
| E43 | Critical-path, uncertainty, information gain and cost scheduling | OPEN | No qualifying evidence |
| E44 | 60 logical agents with four active slots and dormant workers without inference | OPEN | No qualifying evidence |
| E47 | Real simultaneous worker/controller death and autonomous recovery | OPEN | No qualifying evidence |
| E48 | 36h continuous real lifecycle qualification | OPEN | No qualifying evidence |
| E49 | 72h repeated fault endurance qualification | OPEN | No qualifying evidence |
| E50 | No AI/provider execution dependency in native lifecycle | PASS | native 8 integration tests; no AI adapter in source |

