# R004 architecture acceptance ledger — 100 points
Canonical machine-readable tracker: [acceptance-tracker.json](acceptance-tracker.json). Architecture: [OWNER-DESIGN-R002.md](../OWNER-DESIGN-R002.md). Audit: [ARCHITECTURE-AUDIT-R003.md](ARCHITECTURE-AUDIT-R003.md).

**Verified baseline: 28/100 (14/50 PASS, 3 PARTIAL, 33 OPEN).** R003 source pin f837042b76e2bd438f107a6b93360306da6d98ef; [native receipt](VALIDATION-R003.md). Prior ledger reported 30/100; E27 reduced to PARTIAL because explicit stale checkpoint write rejection was not tested. No new runtime test was run. E05 criterion unchanged; dependency gating alone is insufficient. Qualitative architecture estimates are not evidence scores.

Each gate is worth 2 points; only PASS counts. Fixed criteria and IDs; each PASS requires exact source pin, actual native test/experiment receipt, and coverage of the entire criterion. PARTIAL, OPEN, FAIL, BLOCKED earn zero. Reopen affected gates after regression or material unverified changes. No AI wiring for native qualification. Endurance criteria are programmable; when elapsed time is requested it must be observed real time.

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
| E45 | Four isolated architecture policies enforce allowed persistence | OPEN | No qualifying evidence |
| E46 | Common fault schedule and repeatable oracle across four arms | OPEN | No qualifying evidence |
| E47 | Real simultaneous worker/controller death and autonomous recovery | OPEN | No qualifying evidence |
| E48 | Configurable continuous lifecycle qualification | OPEN | No qualifying evidence |
| E49 | Configurable repeated-fault endurance qualification | OPEN | No qualifying evidence |
| E50 | No AI/provider execution dependency in native lifecycle | PASS | native 8 integration tests; no AI adapter in source |
## Iteration plan
1. P0 integrity and lifecycle terminal states; immutable history and validated checkpoints; indexed storage with migration and actor admission.
2. P1 executable behavior: typed graph/dataflow, operator contracts, class composition and evidence; context binding.
3. P2 runtime lifecycle: real native processes, leases/clock, durable activation, resource slots, replacement, mailbox and projections.
4. P3 evolution: bounded synthesis, disjoint holdouts, typed matching, provenance, disable/regression, cost feedback.
5. P4 experiment: policy-isolated session/agent/goal/intelligence arms, same workload and fault schedule, actual crash and fan-out, then policy-defined lifecycle and repeated-fault qualification.
Record source pin, tests, execution receipts and remaining limitations for each revision. Never mark an unrun or simulated gate PASS. No external effects or provider calls until the native architecture is qualified.

## R004 source checkpoint (unqualified)

Candidate source pin: 4c7fb9dc73fe91c1f58f5c0f21ba85842063dacd. Historical R003 evidence remains 28/100; the new R004 kernel has no native qualification yet. The existing status column refers to that historical baseline, not a claim about R004. Per-gate candidate state is recorded in the canonical JSON tracker: 31 source-implemented/unverified, 15 partial source, 4 deferred. Builds, compilation and tests are deliberately deferred by owner instruction. No experimental folder changes.

See [IMPLEMENTATION-R004.md](IMPLEMENTATION-R004.md) for mechanisms, limitations and next implementation gates.

## R004 source checkpoint 002 (unqualified)

Source pin: 4da8909bec4dea47f5863a6f7e2c8a6f4c28f93e. 38 source-implemented/unverified, 8 partial source, 4 deferred. Historical R003 28/100; R004 candidate verified 0/100. See [IMPLEMENTATION-R004-002.md](IMPLEMENTATION-R004-002.md). All original IDs, criteria, weights and evidence are preserved. No compilation, build, test, agent launch or endurance execution occurred.

## R004 source checkpoint 003 (unqualified)

Source pin: 3496f0b829b652488cdfdc2ed39da996c2c6e425. 40 source-implemented/unverified, 6 partial source, 4 deferred. Historical R003 28/100; R004 verified 0/100. See [IMPLEMENTATION-R004-003.md](IMPLEMENTATION-R004-003.md). Native qualification source was authored but never executed. Original criteria, weights, priorities and historical evidence unchanged.

## Supplemental model routing R001

See [MODEL-ROUTING-R001.md](MODEL-ROUTING-R001.md) and [routing.example.json](routing.example.json). Source pin f732d7648c729910a1dfe5ad412e409cb6cc3d58. Seven of ten supplemental routing gates have source implementation; three deferred for live provider adapters, automation and qualification. The original 50-gate source count (40 implemented, 6 partial, 4 deferred) and verification scores remain unchanged. No model calls, builds, tests or experimental folder changes.

## Supplemental model routing R002

Source pin 0f05cb749b5b42986988cbba32d1af1f3b97b740. See [MODEL-ROUTING-R002.md](MODEL-ROUTING-R002.md). Seven implemented/unverified and three partial source; five authored but unexecuted qualification fixtures. Original 50-gate tracker and verified scores unchanged. No real provider adapter, calls, builds or tests.

## Supplemental model routing R003

Source pin 1b3af7066b4b5cb2a10408031fb0163e379ffb8f. [MODEL-ROUTING-R003.md](MODEL-ROUTING-R003.md). Seven source-implemented/unverified, three partial source, seven authored and unexecuted qualification fixtures. Bounded joint assignment, shadow comparison, and an explicitly synthetic-only backend/driver are integrated. Original 50-gate criteria, weights and verified scores unchanged; no live provider calls or native execution.

## Supplemental model routing R004

Source pin 8fa44485e12c178655cca3c5c7e092526137017b. [MODEL-ROUTING-R004.md](MODEL-ROUTING-R004.md). Seven source-implemented/unverified and three partial source gates; ten authored, unexecuted fixtures. Explicitly authorized policy trials, bounded candidate generation, separate resource caps, comparable starting quota, evidence-linked observed yield, optional automatic promotion and versioned rollback. Original 50 gates and verified scores unchanged. No builds, compilation, tests or provider requests.

## Static review 001

Reviewed 02d3e8f737cfb750b164b1cd6a3ac10e44760b53. See [STATIC-REVIEW-001.md](STATIC-REVIEW-001.md). E03/E33 and R03/R06/R07 downgraded to partial source based on specific source gaps. Original and routing verified scores unchanged; no builds/tests. Genesis launch not ready.

## Truth tracker 001 — unified inventory and closure order

Tracker input commit: d05e332f884a11ef3bcbc69658a4de308adc812a. Last reviewed runtime source: 02d3e8f737cfb750b164b1cd6a3ac10e44760b53. No runtime source changed in this update.

| Register | Total | Source implemented, unverified | Partial | Deferred | Current verified |
|---|---:|---:|---:|---:|---:|
| Original architecture | 50 | 38 | 8 | 4 | 0 |
| Supplemental routing | 10 | 4 | 6 | 0 | 0 |
| Combined inventory (not weighted acceptance) | 60 | 42 | 14 | 4 | 0 |

Original source coverage 76%; routing source coverage 40%; combined inventory 42/60 (70%). All figures are provisional source classifications. Historical R003 verified 28/100; current R004 verified 0/100. No new qualification evidence, no reclassification in this tracker-only update. The prior 80% source claim was corrected to 76% by static review of E03 and E33. Routing was corrected from 70% to 40% by review of R03, R06, R07.

### Original phase breakdown

| Phase | Source implemented / total | Partial | Deferred |
|---|---:|---:|---:|
| P0 | 11/15 | 4 | 0 |
| P1 | 10/10 | 0 | 0 |
| P2 | 11/12 | 1 | 0 |
| P3 | 6/8 | 2 | 0 |
| P4 | 0/5 | 1 | 4 |

### Closure order

1. **Core correctness and persistence** (E03, E33, R03, R06, R07): Repair hierarchy-wide terminal propagation; charged-but-unverified cancellation; durable blocked/reset/exhausted routing states; nested router storage growth; review origin-provider lineage. Author negative fixtures, then qualify at pinned source.
2. **Genesis preparation** (E13, E26, E31, R06, R08): Author executable bounded Genesis manifest, native first objective and independent completion oracle. Explicit model list and scope; no live provider activation before qualification. Genesis readiness is a separate launch condition, not another scored gate.
3. **Remaining original source implementation** (E31, E41, E42): Protected host admission/effect boundary, generalized evidence-backed learning, complete resource/cost feedback. Do not expand scope until these original requirements are addressed.
4. **Native build and qualification** (E34, E44, E47, R10): After explicit execution authorization, compile, repair defects, run pinned native and synthetic routing qualification including actual concurrency and process destruction; verify every other source-marked gate against its original criterion.
5. **One live adapter and routing qualification** (R08, R09): Real model registry and authorized budgets, verified live quota/usage receipts, bounded dispatch and recovery, controlled repeated policy comparisons; no provider calls before authorization.
6. **Comparative and endurance qualification** (E45, E46, E48, E49): Execute authorized isolated comparative fault schedule and actual policy-defined endurance qualification. No simulated duration accepted.

Genesis is not launch-ready. No new gates or weights added; no builds, tests, provider calls or source repairs in this update. See completion_dashboard in acceptance-tracker.json and STATIC-REVIEW-001.md.

## Closure 002

Source pin 0e36d605c6297d53e0c79634b8ed73ff890cc881. [CLOSURE-002.md](CLOSURE-002.md). Original 39/50 source-implemented/unverified, 7 partial, 4 deferred; routing 6/10 source-implemented/unverified, 4 partial. Combined 45/60 provisional source coverage. Genesis native initialization source present, not qualified. No runtime execution or new verified credit.

## Source closure 005 (unqualified)
Native programmable qualification runner and matched four-arm planning source authored. E45/E46/E48/E49 remain PARTIAL_SOURCE. The historical PASS column is unchanged. 39/50 provisional source-complete, 11 partial, 0 deferred; no verified R004 acceptance credit. See SOURCE-CLOSURE-005.md.

## Source closure 006 (unqualified)
Host-only static declaration batches, bounded pinned index scans and a separate manifest-bound comparative persistence vault authored. E33/E45 remain partial. Original gate weights and verified scores unchanged. See STORAGE-BATCH-SCAN-001.md and EXPERIMENT-VAULT-001.md.
