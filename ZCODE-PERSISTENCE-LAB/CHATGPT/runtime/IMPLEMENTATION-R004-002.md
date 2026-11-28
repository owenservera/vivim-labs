# R004 — implementation checkpoint 002

Source pin: 4da8909bec4dea47f5863a6f7e2c8a6f4c28f93e. All changes on main inside CHATGPT/runtime, authored through GitHub connector. No compilation, builds, tests, live processes, or experimental folder changes. This checkpoint extends [001](IMPLEMENTATION-R004.md), not a new architecture or replacement tracker.

## Implemented in this chunk

- E33/E34: index.rs provides content-addressed immutable disk pages, sharded by table and key digest. Every new commit binds the index root to the record and head. Existing unchanged pages are reused. Point lookup reads manifest and page rather than replaying commands. Bounded retained journal, checkpoint record anchor, reachability garbage collection, legacy import. Canonical transaction execution still loads the entire world, and receipt and canonical entity counts remain unbounded; sharding is not yet a fully incremental in-memory transition engine.
- E21/E31: authority.rs provides HMAC-SHA256 signed request envelopes, audience, expiry, principal role and goal scope, worker incarnation binding and independent verifier identity. Admission is serialized with state mutation. This is a trusted-host ingress library/CLI; the caller chooses the authority file. OS permissions and a trusted adapter must prevent untrusted clients selecting their own authority or calling the raw local host API. No claim of a remote security boundary or external effect execution.
- E23: lifecycle.rs provides ordered recipient-scoped durable delivery, lease-based redelivery, incarnation-bound acknowledgments and mailbox wakeups. Native generic message-processing classes still need explicit adapter integration.
- E15: projection.rs provides 14 separately addressed agent projection components (identity, lineage, class, objective, capabilities, constraints, context, state, mailbox, workspace, discoveries, checkpoints, evidence, incarnations), with an immutable sequence manifest. Full projection is host-owned, separate from the worker's work package. OS-level isolation remains future work.
- E26/E47: exclusive controller ownership, pipe-bound native worker lifetime, restart fencing, existing generation fences and graph checkpoint recovery. No actual controller death or worker termination was performed. A separate service manager to restart the whole host is not implemented.
- E36: rollback of reproducibly rejected learned classes, versioned receipts, compatible previously admitted fallback, pending task rebinding and running-attempt fencing. No oracle weakened. Submitted candidates retain independent verification.
- E37/E38: bounded multi-level expression synthesis, observational equivalence pruning, nested field projection, affine inference, arithmetic, hash, arrays, conditionals and concatenation. Search receives training only; holdouts evaluate one selected candidate; durable rejection and consumed holdouts. Manual trusted-host admission remains an alternative path requiring stronger holdout custody enforcement.
- E41: knowledge.rs automatically assimilates recurring independently verified typed artifacts via registered family rules, with distinct-input evidence, provenance, publication caps, automatic selection and ambiguity rejection. Rules may represent repair/procedure/context/coordination/fact/proof artifacts. Arbitrary procedure execution and autonomous invention of semantic rules are not implemented.
- E42/E43/E44: durable attempt start/end records, measured duration, failed search accounting, yield, empirical uncertainty and novelty heuristics, weighted critical path and population audit. Resource measurements and search overhead accounting incomplete; no 60-agent run performed.

## Additional CLI source interfaces

Under the existing `kernel <store>` prefix:
```text
lookup <table> <entity-id>
collect
population
project-host <agent-id>
apply-signed <trusted-host-authority-file> <signed-request-file-or->
```
New durable actions: Receive, RollbackCapability, RecoverNative, RegisterKnowledgeRule, AssimilateRecurring, plus verifier actor. These are source interfaces only; execution has not been attempted.

## Tracker

The original acceptance-tracker.json remains canonical. 38 gates source-implemented/unverified, 8 partial source, 4 deferred; R003 historical 28/100; R004 verified 0/100. Do not confuse code coverage labels with proof. E26 E31 E34 E38 E41 E42 E44 E47 retain partial status; E45 E46 E48 E49 deferred. All gates require native acceptance at a pinned revision.

## Next chunk

Finish true scoped host authority and independent holdout admission; incremental entity transactions and receipts retention policy; durable outer service restart and message-driven execution; complete evidence-based class/repair/context/procedure specialization; complete compute/resource accounting and qualification fixture source. Comparative execution and actual 36/72-hour qualification require explicit later execution authorization.

Known limitations and choices: no new crate dependencies; hand-built SHA256 HMAC adapter requires verification against standard vectors; filesystem sync on Windows lacks Unix directory-sync equivalent in this source; the immutable index is not malicious-host tamper proof; no code was compiled or executed so source defects remain possible. No local checkout synchronization was attempted because this iteration used GitHub only.
