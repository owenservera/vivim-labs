# STATIC-REVIEW-001 — canonical implementation and launch readiness
Reviewed source: 02d3e8f737cfb750b164b1cd6a3ac10e44760b53. Targeted static review of tracker, transition, persistence/index, model, composition, authority, custody, evolution, intelligence, behavior, supervisor, routing, planning, allocation, adapter, driver, policy evolution, CLI and manifest. No compiler, build, test, real/synthetic provider call or local sync. Not an exhaustive compiler or invariant audit.

## Decision: NOT READY for autonomous Genesis

Original tracker before review: 40 implemented/unverified, 6 partial, 4 deferred. After targeted correction: {"SOURCE_IMPLEMENTED_UNVERIFIED":38,"PARTIAL_SOURCE":8,"DEFERRED":4}. Routing before: 7 implemented/unverified, 3 partial. After: {"SOURCE_IMPLEMENTED_UNVERIFIED":4,"PARTIAL_SOURCE":6}. Historical R003 verified 28/100 and R004 verified 0/100 are unchanged. Original 50 criteria/weights and supplemental 10 criteria preserved. These are source labels, not verified functionality.

## Source findings and remediation

SR01 (E03): transition.rs settle() computes active work from parent-local tasks and can make a parent terminal while open children continue. Explicit Cancel traverses descendants, but automatic failure does not. Define hierarchy-wide terminal semantics, fence descendants or retain parent until all resolve, and add negative fixtures.

SR02 (R03/R07): routing_adapter.rs finish_handoff settles a completed receipt and submits output; transition.rs Cancel removes proposal and lease; routing.rs cancel_goal changes only Reserved/Dispatched. routing_evolution.rs outstanding() treats Completed without verification as outstanding. Cancellation between receipt and verification can therefore prevent closure and future trial startup indefinitely. Add explicit charged-but-not-evaluated terminal outcome that never awards success and preserves actual usage.

SR03 (R06): allocation.rs routing_cycle_needed requires a feasible assignment, while transition.rs ready() does not consider routing deadline, exhaustion, or missing adapter. If no model is eligible, tasks may remain pending and supervisor can stop idle without a durable reason or wake time. Add explicit waiting-for-quota/dependency/configuration and terminal inference-exhausted outcomes; distinguish recoverable wait from permanent failure.

SR04 (E33): index.rs MAPS excludes nested router state, so growing routing reservations, decisions, receipts, observations and trials are stored in a single scalar object; put() rejects objects above 64 MiB. A sufficiently large routing history will stop new commits. Shard nested routing maps, define migration and bounded retention preserving audit evidence, and verify restart compatibility. Full World load/clone still remains.

SR05 (R07): routing.rs independent_review filters providers against the history of the same demand; a fresh review demand has no history of the original task. Explicit excluded_providers can be configured by host but source does not automatically bind review to origin evidence/provider. Add reviewed-artifact/attempt lineage and enforced origin exclusion or independent deterministic verification.

SR06 (R08/R09): real adapter execution absent; backend trait has synthetic local implementation only. No actual model catalog, live quotas, authenticated provider receipt, provider reset semantics or capability negotiation. Configuration examples disabled. A mismatch of actual model/thinking identity rejects receipt while retaining hold; explicit charge-only reconciliation is required. No guarantee of external budget compliance until transport applies ceilings and reports real usage.

SR07 (R09): sequential policy trials compare declared demand histograms, not matched workload/oracle difficulty. Model performance estimates and provider conditions change between arms; adapter bindings can also change during trial. No randomized/repeated paired cohorts, uncertainty intervals or regression watchdog. Promotion is opt-in heuristic, not causal validation. Freeze relevant bindings and add repeated comparisons before real automatic promotion.

SR08 (E31): raw trusted local apply path and host-only CLI are not an OS isolation boundary; signed ingress protects cooperating callers only. Real launcher/credentials/filesystem scope and authenticated adapter transport required before giving autonomous agents workstation or repo effects.

SR09 (E41/E42): semantic procedure, repair, class/coordination/context discovery beyond declared typed patterns and bounded Expr synthesis remains incomplete. Resource accounting does not cover all CPU/memory/I/O, search and regression costs. True goal portfolio management and autonomous completeness/replanning remain future mechanisms.

SR10 (Genesis launch): main.rs bootstrap registers native operator contracts only. It does not materialize a Genesis objective, acceptance-driven task graph, class/context bindings, actual model registry or real coding-agent transport. No launch-ready Genesis package exists in this reviewed CLI path. Need executable bootstrap manifest and first bounded objective with explicit success oracle.

## Remaining prelaunch sequence
1. Source repairs SR01–SR05, explicit terminal/blocked states, charge-only reconciliation and source-schema compatibility.
2. Author deterministic Genesis manifest, native first task and independent oracle, bounded budgets and scope; no AI needed for first smoke.
3. Only after owner permits builds, compile and repair, execute native qualification and routing synthetic fixtures, record pinned receipts.
4. Configure one real adapter and real model/quotas, with least required scope and one active intelligence slot. Verify one complete request/receipt, failure/unknown, recovery, and stop behavior before autonomous coding.
5. Complete remaining original gates (E31/E34/E41/E42/E44/E47 plus E03/E33) and comparative/endurance gates E45/E46/E48/E49 when authorized.

Review findings are source-derived, not observed runtime failures. No fixes executed in this review; no claim of operational readiness.
