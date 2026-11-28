# Model routing R004 — bounded policy evolution

Source pin: 8fa44485e12c178655cca3c5c7e092526137017b. Canonical CHATGPT/runtime on main. GitHub connector only. No compile, build, test, synthetic execution, model request or experiment-folder modification. Atomic non-forced tree commits preserve concurrent changes on main.

## Mechanisms

- routing_evolution.rs generates four bounded candidate policy configurations (broader search, completion emphasis, priority emphasis, cheap greedy approximation). Candidate generation is proposal-only and does not authorize experiments or extra usage.
- BeginRoutingTrial requires explicit matched baseline and challenger demand sets, disjoint fresh task IDs, declared equal demand distributions, ready compatible adapters, a minimum sample count, positive success floor, resource-saving target, equal duration, and per-arm cap for every configured pool. Neither arm can override existing hard limits or provider observations. Config, batch policy and externally supplied forecast settings cannot be modified while the trial is active.
- Trials run sequentially, exclude unrelated routing demands and record reservations against the active arm. No native-first demands are eligible. Actual usage is charged, including failed attempts. Unattempted enrolled tasks count against completion. Successful feedback must be linked to the task's independent evidence through the normal world transition; raw Router unit fixtures use synthetic evidence labels only.
- Each reservation records its batch policy revision and verification time. Reports preserve arm membership, reservation IDs, policy, elapsed-window boundaries, starting available quota, actual usage by pool, verified successes, and evidence IDs. Unknown charges and completed-but-unverified results block arm closure. Late verification is not credited as on-time success.
- Between arms the runtime waits for comparable starting available quota (capped by the respective equal arm caps). Per-arm caps persist across global quota windows. No automatic increase in hard budgets.
- Recommendation requires no cap overrun, baseline success floor, challenger verified yield no lower than baseline, target reduction in objective pool and no increase in any other pool. Distinct resource units are never summed. This conservative guard can reject useful tradeoffs; owner can run a new authorized comparison with a different objective rather than bypass it.
- Auto-advance and auto-promote are opt-in per trial. Supervisor arm advancement counts against existing learning-plan budget; supervisor duration/job limits remain authoritative. Operator may manually advance/promote or abort. Abort releases undispatched holds, marks dispatched attempts unknown and preserves their charges. Rollback restores previous weights under a new increasing installed revision; requires drained routing work.
- No empirical causal proof is implied. Matched declared demand characteristics do not ensure identical difficulty, goal grouping, time-of-day quota pacing, provider health or learned performance across sequential arms. Small samples can be misleading. No automated randomized allocation, repeated paired trials, confidence intervals or post-promotion regression watchdog yet. Default should be manual promotion for real providers until qualified.

## Interfaces

Under kernel <store>: routing-policy-candidates; routing-trials.
Host actions through kernel apply: begin_routing_trial {spec}, close_routing_arm, start_routing_candidate, abort_routing_trial {reason}, promote_routing_trial {trial}, rollback_routing_trial {trial}. A spec includes id, candidate BatchPolicy, baseline_demands, candidate_demands, duration_ms, caps, objective_pool, min_samples, min_success_bps, min_savings_bps, auto_promote and optional auto_advance. Explicit owner-supplied cohorts and budgets are mandatory; no trial starts automatically from candidate generation. Use dedicated lab stores/cohorts for experiments, especially because unrelated routing is paused while a trial is active. The synthetic fixture is not evidence of live provider savings.

## Qualification status

tests/routing_evolution.rs adds three unexecuted guard fixtures: promotion and rollback after synthetic matched usage receipts; unresolved charge blocks closure and survives abort; incomplete challenger work prevents promotion. Ten total routing fixtures authored across R002–R004. No runtime result exists for these fixtures.

R01–R07 remain source-implemented/unverified; R08–R10 partial. Original 50 gates, weights and R003 historical 28/100 and R004 verified 0/100 unchanged. Remaining: real adapter execution, live usage observations, empirical repeated/randomized comparisons and regression watch, source-level consistency review and later authorized qualification. Source may contain compiler or logic errors. Full-state scans/clones remain a scalability constraint. No local sync performed.
