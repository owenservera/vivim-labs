# R004 — source implementation checkpoint 001

**Source pin:** 4c7fb9dc73fe91c1f58f5c0f21ba85842063dacd  
**Qualification:** NOT COMPILED / NOT BUILT / NOT TESTED.  
**Owner instruction:** continue the canonical CHATGPT build in large coding chunks; GitHub plugin only; do not run builds or tests until implementation is finished. The four experimental folders are the separate conversation's responsibility.

## Implemented source mechanisms

- `model.rs`: separate versioned goals, tasks, agents, contexts, artifacts, evidence and classes; explicit incarnation, lease, checkpoint, knowledge, effect and learning records.
- `composition.rs`: typed multi-source graph wiring, deterministic evaluation, predecessor/context/knowledge input bindings, validation of graph dependencies, resumable pure-computation prefixes.
- `behavior.rs`: fifteen bounded deterministic behavioral operators. These are computational primitives, not human-level reasoning capabilities.
- `transition.rs`: atomic world transitions, goal/task outcomes, hierarchy, graph refinement, actor roles, generation fencing, cancellation, mailbox wakeups, evidence, policy and learning commands.
- `persistence.rs`: immutable command records and commit markers, periodic snapshots, bounded journal tail, lock-based writer serialization, request idempotence, explicit external anchor comparison and workspace projections.
- `supervisor.rs`: bounded native process execution, explicit incarnation bindings, global task slots, automatic worker generation, host clock/lease supervision, progress checkpoint admission, replacement and resumption, automatic learning dispatch and execution cost receipts.
- `evolution.rs`: bounded deterministic expression synthesis, disjoint held-out evaluation, regression checks, class/capability admission and lineage.
- `intelligence.rs`: durable learning plans/attempts, evidence-backed knowledge records, automatic matching support, critical-path scheduling and observed cost feedback.
- `effects.rs`: effect-intent identity, uncertain-outcome fencing and host reconciliation receipts. The runtime does not execute external effects.
- `migration.rs`: explicit import of a verified R003 state into a fresh R004 store, with the original state preserved as an archive. Old leases are fenced; unsupported historical state is archived, not silently translated.

R003 remains available for reproducibility. R004 is connected through the new `kernel` CLI and uses a separate store. No AI or provider adapter is included.

## CLI contract (source only)

```text
persistent-intelligence-runtime kernel <store> inspect
persistent-intelligence-runtime kernel <store> ready
persistent-intelligence-runtime kernel <store> bootstrap
persistent-intelligence-runtime kernel <store> apply <request-file-or->
persistent-intelligence-runtime kernel <store> discover <learning-spec-file>
persistent-intelligence-runtime kernel <store> assimilate <request-id> <name> <version> <discovery-file>
persistent-intelligence-runtime kernel <store> run <workspace> <run-config-file>
persistent-intelligence-runtime kernel <store> project <agent-id> <projection-root>
persistent-intelligence-runtime kernel <store> import-r003 <old-store> <request-id>
persistent-intelligence-runtime kernel <store> anchor
persistent-intelligence-runtime kernel <store> verify-anchor <anchor-file>
```

The run configuration includes explicit time, lease, poll, job and learning-plan budgets. The optional stop-when-idle policy permits a bounded idle supervisor. Worker subprocesses receive only the pure work package; the host owns state and output oracles. The local trusted host CLI is not an authenticated security boundary. Future adapters must preserve actor authority and knowledge-seed isolation.

## Acceptance discipline

The canonical acceptance criteria, IDs, weights and prior evidence are unchanged. R003's historical 28/100 is not an R004 verification score. The candidate has 31 source-implemented/unverified gates, 15 partial source gates, and 4 deferred gates. All 50 must eventually be independently qualified at a pinned source version. See the per-gate `r004` fields in `acceptance-tracker.json`.

## Next coding work before qualification

1. Strengthen disk-indexed entity/payload persistence, independent authority boundaries and projection completeness; retain bounded recovery and verified provenance.
2. Complete autonomous event delivery, recovery policies, hierarchical terminal propagation, and source-level invariant review across the new kernel.
3. Expand deterministic knowledge synthesis, applicability, rollback, repair/context/procedure reuse, and full failed-attempt/learning cost accounting.
4. Prepare qualification fixtures without executing them while the no-build/no-test instruction remains active; integrate actual process crash, resource-capacity, persistence-policy and endurance evidence only when authorized.

Remaining limitations: the storage engine still loads a complete snapshot into memory; receipts and canonical entities grow over time. Snapshot integrity is not protection against a malicious trusted host or a rewritten entire store. External effect execution and authenticated actor transport are absent. Learning is bounded by the implemented expression grammar; generalized procedure and context discovery are incomplete. Wall-clock restart and multi-process crash safety have not been empirically demonstrated. No 36h or 72h claim is made.

No compilation, build, native test, or experimental agent launch was performed during this source iteration.
