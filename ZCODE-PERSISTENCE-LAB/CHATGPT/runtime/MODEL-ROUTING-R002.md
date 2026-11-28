# MODEL ROUTING R002 — autonomous preparation and frontier planning

Source pin: 0f05cb749b5b42986988cbba32d1af1f3b97b740. Scope CHATGPT/runtime on main; GitHub connector only. No compile, builds, tests, provider requests, or experiment modifications.

## Added source mechanisms

- routing_planning.rs: versioned forecast guard; explicit confidence-weighted future critical demand; ready and pending critical task estimates; empirical upper-quartile same-time-of-window future usage across up to seven prior observed windows (minimum three); protection is the maximum of those estimates capped by configured pool fraction, not a sum. Fixed emergency reserve and pacing remain authoritative. Historical data are settlement-time observations, not a counterfactual forecast of success. Opportunity penalties affect model ranking.
- routing.rs: adaptive reasoning ceilings per model/thinking profile with explicit minimum and maximum; profile quota membership immutable; fresh optional observations also checked at dispatch; provider/account cooldown key corrected; future protection rechecked before dispatch; quota refresh plan and evidence-backed scoped observation receipt. Existing shared daily, rolling and lifetime pools, hard reservations and provider usage reconciliation remain.
- routing_adapter.rs: versioned enabled adapter binding to class/context; atomic task lease and immutable work envelope; one-shot committed ClaimHandoff before any external send; exact actual provider/account/model/thinking identity in receipt; usage charged even if output stale or unusable; independent verification remains separate. Raw managed handoff dispatch/settlement and routed worker submissions are denied. Replay of the original claim request returns reconciliation-only, never send authority. A trusted adapter must honor that response and reconcile unknown transmissions; no guarantee of exactly-once provider effects.
- allocation.rs: bound-profile compatibility checks, atomic ranked RoutingCycle, generation fencing, prepared-handoff expiry/revocation handling, native-first eligibility. Supervisor automatically prepares work within its existing max_jobs budget if a compatible enabled binding is present. Independent verification automatically credits the corresponding reservation. No network dispatch or real adapter installed.
- tests/routing_r002.rs: four authored fixtures covering protected frontier reserve, unknown usage across reset, stale observations/unsent expiry, adaptive thinking and forecast guard.
- tests/routing_handoff.rs: one authored integration fixture using synthetic receipt, store reopen and duplicate send-authority denial, independent verification and performance feedback. None executed.

## New interfaces

kernel <store> routing-cycle <limit> <request-id>
kernel <store> routing-adapter-queue <adapter-id>
kernel <store> routing-refresh-plan

Host actions: SetFrontierGuard, SetFrontierForecast, BindRoutingAdapter, ObserveAdapterQuota, PrepareHandoff, ClaimHandoff, FinishHandoff, RoutingCycle. ConfigureRouter and QueueInference retain R001 semantics. A demand may enable adaptive_thinking and specify minimum_reasoning_tokens; reasoning_tokens is then a ceiling rather than a fixed request. Actual provider accounting must distinguish input, output and separately charged reasoning.

Illustrative guard: {"version":1,"max_forecast_bps":5000,"opportunity_weight":10,"protect_pending_critical":true}. Configure by trusted Host. No guard is enabled implicitly. No real model profiles enabled by this revision.

## Limits and next steps

Seven routing gates remain source-implemented/unverified; R08/R09/R10 are partial source, not passes. Actual authenticated provider adapters, live quota polling and exact model/usage observation, provider idempotency and reconciliation, real budget calibration and empirical policy optimization remain open. A mismatched identity receipt is rejected and its hold remains outstanding pending trusted reconciliation; no silent reassignment. Provider-observation provenance is a trusted host contract, not proof of provider authenticity. A prepared handoff consumes a task attempt and slot until expiry/revocation or completion. The history-based forecast is heuristic, not a globally optimal allocation. Current implementation scans and clones the full world/router, and unbounded historical records require later indexing/retention. No live authority or OS isolation has been provisioned. No tests or builds were attempted, so compile and logic defects remain possible.

Original 50-gate criteria and scores unchanged. Historical R003 28/100; R004 verified 0/100. The supplemental routing workstream is separately tracked in acceptance-tracker.json.
