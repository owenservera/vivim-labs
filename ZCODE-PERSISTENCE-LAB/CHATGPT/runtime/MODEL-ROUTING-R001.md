# Provider-neutral intelligence allocation and frontier economy

Source-only checkpoint. No AI/provider adapters, actual model list, account quota readings, or pricing data were supplied. All example profiles are disabled and their values are fictional configuration examples, not provider facts. No builds or tests.

## Configure once

Copy routing.example.json; assign stable profile IDs per provider/account/model/thinking level. Attach all applicable quota pools to each profile: account-wide subscription, model family, exact thinking level, shared monetary cost, token or credit caps. Add a per-goal or experiment pool in a demand's extra_pools. A pool may be fixed with explicit reset anchor/period, rolling, or lifetime. Use provider-observed reset epochs rather than assume UTC or local midnight; fixed periods do not implement daylight-saving calendars. A provider with shared quota must use the same pool ID across profiles. Currency is explicit, no exchange-rate conversion; absolute price weight requires a common currency. A tariff can price requests, input, output, and separately billed reasoning units. Configure non-overlapping token counts according to provider accounting. A zero marginal tariff does not remove subscription usage constraints. Profile quality and latency priors are user estimates, revised through verified evidence.

## Frontier allocator

For example only, a 20-request daily pool with 25% emergency reserve protects five calls from ordinary tasks; 20% opening release of the remaining 15 permits three at reset and releases the remainder gradually. Per-goal share and minimum expected uplift further gate ordinary frontier use. Critical priorities may bypass pacing, reserve and per-goal restrictions, never absolute pool limits, stale required quota, context limits or authorization. Ready demands are ranked by criticality, priority, uplift, novelty, uncertainty and urgency; each allocation recalculates feasible choices. This is a deterministic heuristic, not a proven optimal future-demand forecast. Model quality is smoothed from family-specific verified outcomes and availability failures. Fallbacks consider other allowed profiles/providers, capacity, prior rejections, cooldowns and deadline; do not retry uncertain external effects or unverified completed requests.

## Durable lifecycle

QueueInference records demand linked to open pending task. Native capability is preferred by default; explicit inference/review can override. AllocateInference selects and reserves across every attached pool in one canonical transaction. Allocation is not provider execution. A Host adapter must claim the task, bind a matching routed:<adapter> incarnation, and commit DispatchInference before performing a provider call. Every dispatch is tied to task generation. SettleInference records actual reported usage and provider receipt; overruns are recorded and cooldown applied rather than discarded. Unknown charges retain holds across resets; only an independently reconciled settlement can resolve them. Reserved-but-unsent calls expire safely. VerifyInference links an independent task evidence record to the exact task generation and updates model performance; an accepted result stops further allocation. Cancelled goals release unsent holds and mark dispatched work unknown.

ObserveQuota supplies trusted timestamped remaining quota; if require_observation is true, missing/stale readings fail closed. Observations are conservatively combined with local reservations; the adapter must specify accurate units and supply authoritative receipts. Provider budget limits cannot be guaranteed externally without correct provider usage bounds and observations. No live provider monitoring is wired. The broker must protect Host-only configuration and evidence admission.

## Source interfaces

kernel <store> routing-configure <json-file> <request-id>
kernel <store> routing-budget
kernel <store> routing-plan
kernel <store> routing-recommend <demand-id>
kernel <store> routing-allocate <limit> <request-id>

Other durable actions via kernel apply: ObserveQuota, QueueInference, DispatchInference, InferenceUnknown, ReleaseInference, SettleInference, VerifyInference. Clock advances through trusted Tick. Adapter and supervisor integration for live inference, automatic polling/dispatch/reconciliation and actual model usage measurement is explicitly pending; the native supervisor does not call AI. The future adapter must invoke the allocator on readiness/settlement/reset events and report actual model, thinking level, context and usage. Routing does not create a new verified acceptance score or alter existing gate criteria.

## Implementation files

src/kernel/routing.rs — profiles, pools, tariff, windows, health, verified-performance estimates, durable reservations and receipts.
src/kernel/frontier.rs — daily pacing and ranked frontier demand.
src/kernel/allocation.rs — task scheduling, native preference, atomic reservation batch, incarnation binding and independent feedback.
src/kernel/model.rs, transition.rs, supervisor.rs, mod.rs, src/main.rs — durable integration.

## Open qualification

Verify overlapping caps, observation staleness, rolling/fixed resets, concurrent reservations, unknown outcomes, frontier emergency reserve, mixed currencies, exact thinking-level charging, deterministic fallbacks and evidence-based performance. The configuration must be validated with actual provider quotas, tariffs and model availability before enabling any profile. No claim that all source compiles or runs.
