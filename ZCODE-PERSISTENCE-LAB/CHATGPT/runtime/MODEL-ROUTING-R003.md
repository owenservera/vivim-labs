# Model routing R003 — joint allocation and offline execution

Source pin: 1b3af7066b4b5cb2a10408031fb0163e379ffb8f. CHATGPT/runtime only, main, GitHub connector. No build, compile, test, synthetic execution, real provider call, or experiment modification. Initial concurrent branch-head conflict was resolved by fresh-head, non-forced atomic tree/commit updates. No branches created.

## Joint allocation

routing_optimizer.rs uses a bounded beam search across ready demand/profile assignments, including the option to wait. Each hypothetical assignment goes through the same reservation feasibility checks as the real router. Each committed profile selection is revalidated. This can preserve a scarce model for a task that has no alternative while assigning a flexible task elsewhere. Search limits: beam 1–16, demands 1–64, profiles per demand 1–8, expanded nodes 1–512, batch limit 1–128. Default version zero preserves existing greedy supervisor behavior; explicitly enable with SetRoutingBatchPolicy. Policy weights are configurable. Simulated pending-critical forecasts are conservatively retained across candidate branches and refreshed in the real world on commit. The controller recomputes after each real reservation. This is bounded heuristic search, not guaranteed global optimum; a negative-utility demand may wait indefinitely unless policy or deadlines change.

## Shadow comparison

The read-only comparison accepts up to eight newer policy configurations against the same ready task snapshot. Outputs estimated success sum, per-pool held resources, utility, selected profile IDs, node count and truncation. Scores under different weights are not directly comparable; use estimated yield and resource use to inspect tradeoffs. Shadow results are not empirical counterfactual outcomes and cannot automatically promote a policy.

## Backend driver

routing_driver.rs defines supports/send/reconcile. A cooperating driver holds an adapter-specific lock, reconciles prior claimed/unknown work before prepared work, advances the host clock before and after backend work, sends only after a new committed one-shot claim and settles using the normal adapter receipt path. Unknown outcomes remain reserved; no automatic resend. Backend implementation must enforce actual bounded execution and transport idempotency; the trait cannot guarantee either. A synthetic backend uses a deterministic Expr and configured synthetic usage, persists integrity-checked receipts in a separate local directory and can recover a receipt after replacement. Only synthetic-prefixed provider and adapter identities are accepted; the bounded synthetic cycle rejects mixed enabled provider profiles. Use a separate ledger per synthetic store/experiment. Missing receipts remain unknown rather than proving absence. No real credentials, connections or model calls.

The synthetic cycle prepares, drives and independently verifies up to configured per-round operation limits over at most 128 rounds. It stops on lack of progress. This is an explicit offline invocation, not an unattended service. The existing supervisor prepares work; no real backend process is automatically launched.

## CLI / configuration

Under kernel <store>:
- routing-joint-plan <limit>
- routing-shadow-compare <limit> <policy-array-json-file>
- routing-synthetic-step <synthetic-config-file> <ledger-directory> <operation-limit>
- routing-synthetic-cycle <synthetic-config-file> <ledger-directory> <max-rounds> <operations-per-round>

Host action set_routing_batch_policy with policy:
```json
{"version":1,"beam_width":8,"demand_limit":32,"profiles_per_demand":4,"node_budget":256,"completion_weight":100,"priority_weight":100}
```

Synthetic configuration is not a real provider profile. A corresponding explicitly enabled synthetic provider profile, adapter binding, compatible class/context, task and demand must be configured by the host. No example profiles were enabled by this revision.

## Qualification source

New tests/routing_optimizer.rs demonstrates the intended flexible-task/scarce-model assignment and read-only shadow comparison. New tests/routing_synthetic.rs demonstrates the intended local receipt recovery and rejection of a foreign provider identity. Seven routing fixtures total with prior R002 sources. All are uncompiled and unexecuted.

## Limits and tracker

R01–R07 source-implemented/unverified; R08–R10 partial. Real providers, quota polling, actual usage verification, timeout-aware backend transport, cross-provider reconciliation, empirical policy optimization and qualification remain outstanding. Search clones the router, so large-history memory/runtime require profiling and indexing. Synthetic usage is configured, not measured provider performance. Source may contain compile or logic errors. No local sync attempted. Original 50 acceptance gates and historical R003 28/100, R004 verified 0/100 remain unchanged.
