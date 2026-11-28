# Goal: determine and achieve completeness

You have the supplied objective and starting files. Build an executable system that fulfills that objective. A second, equally important objective is to determine what it means for this system to be complete.

Before broad implementation, inspect the actual source and formulate a finite, versioned completion contract. Derive required observable behaviors from the objective, distinguish mandatory behaviors from optional enhancements, specify a bounded workload and operating conditions, identify failure cases and what would falsify each claim, and state what is explicitly outside the release boundary. Include independently executable acceptance checks and evidence requirements. Define how all essential checks jointly imply the objective, rather than merely listing features. Record the contract and its rationale in your own workspace. Challenge it for omitted essential behavior, circular tests, vacuous success, and unverifiable claims. Freeze the initial release contract before claiming implementation progress against it.

Persist a goal, remaining obligations, verified results, failed attempts, source revision or content digests, and resumable next actions. Prioritize the largest unresolved mandatory dependency. Reuse verified deterministic computation rather than spending reasoning capacity again. Use model capacity only when needed. Do not assume that an available budget must be spent. On budget exhaustion, interruption or uncertain execution, checkpoint and yield; never declare completion or discard outstanding obligations.

A contract may be revised only with a dated reason, impact on prior evidence, and explicit handling of any removed requirement. Do not narrow the original objective to make tests pass. Material scope reductions require controller approval; newly discovered mandatory gaps reopen affected claims. Optional enhancements go into a separate future backlog rather than endlessly expanding the current release.

A candidate is complete only when the frozen mandatory contract is satisfied by executable evidence on the same candidate source, relevant regressions and failure cases have been checked, and no unresolved mandatory failure or unknown effect remains. Submit the contract, test commands, evidence, source digest, known limitations, and a reproducible run procedure. An external evaluator will independently verify the claim. Until then, report CANDIDATE_COMPLETE, not COMPLETE_VERIFIED. Exhausted resources mean WAITING_BUDGET, not success. A failure that cannot be repaired within the allowed effort is an explicit bounded failure.

Work only within your assigned workspace. Do not access unrelated files, services or other conversations. No deployment, purchases, network or privileged actions unless separately authorized by the controller.

## Common system goals — R013

These are mandatory outcomes, not architecture prescriptions. The external controller owns actual admission and independent verification; you must implement the capabilities in your own system. G01 is the first explicit goal. Prioritize dependencies as needed. Optimality is conditional on a declared workload, evidence and constraints.

### G01 — Optimal model router
Select the least sufficient authorized intelligence for each task, including no inference when deterministic execution suffices; adapt model, reasoning effort and escalation to actual task needs.

Demonstrate: Executable routing on a bounded varied workload; compare against fixed-model and fixed-effort baselines with verified outcomes, resource use and failure rates. Report uncertainty, tradeoffs and cases where routing is worse. Optimal means best demonstrated under declared workload and constraints, not universal optimality.

### G02 — Intelligent resource allocation
Convert live subscription limits, user-defined budgets, resource availability and priorities into safe allocations while preserving work when capacity runs out.

Demonstrate: Execute controlled budget and contention traces, including stale observations, exhaustion, failure, reset, unknown costs and concurrent requests. Never silently overspend, borrow reserved allocations or purchase capacity; show measured progress per resource unit.

### G03 — Autonomous goal management
Maintain persistent objectives, discover missing obligations, organize dependencies, choose next actions and preserve outstanding work independently of execution resources.

Demonstrate: Track a bounded novel objective through decomposition, dependencies, interruptions, changed conditions and reprioritization; keep budget suspension distinct from success.

### G04 — Persistent execution
Preserve goals, knowledge, work, identity and outstanding obligations independently of workers, provider conversations and controller lifetimes.

Demonstrate: Interrupt and replace executing components at multiple lifecycle points, recover from persisted state and continue without duplicate effects, lost verified work or reliance on the original conversation.

### G05 — Dynamic orchestration
Allocate, coordinate, communicate with and retire execution resources according to dependencies, capability, expected value and availability.

Demonstrate: Run bounded parallel/dependent work, prove correct admission, cancellation, replacement, communication and convergence with failures and constrained resource slots.

### G06 — Independent verification
Establish reliable evidence for correctness, provenance, permitted effects and completion; never substitute a worker assertion for verification.

Demonstrate: Use independent executable oracles, adversarial false claims, regression and failure tests on a common candidate version; reject invalid or stale evidence and reconcile uncertain effects.

### G07 — Computational assimilation
Turn repeated verified discoveries into reprogrammable reusable computation, reducing future reasoning demand without losing correctness.

Demonstrate: Solve related fresh problems using previously acquired capability, show a verified reduction in inference demand versus an appropriate baseline, reject incorrect generalizations and support invalidation or replacement.

### G08 — Self-optimization
Learn from verified outcomes, resource consumption and failures to improve the system's own decisions without undermining safety, correctness or reproducibility.

Demonstrate: On held-out workloads compare a versioned improved policy to its baseline, bound exploration, account for its cost, retain regressions and permit rollback when evidence does not support improvement.

### G09 — Autonomous sufficiency and evolution
Determine what is enough for a specific purpose, operating envelope and evidence standard. Separate essential unmet obligations from optional improvements; distinguish verified functional sufficiency, economically justified optimization, resource suspension and invalidated sufficiency.

Demonstrate: For at least two materially different bounded objectives, derive and defend a finite release contract before implementation. Demonstrate rejection of unsupported completion and disguised scope reduction, acceptance of an adequate release with a nonempty optional backlog, a justified decision to continue or defer optional work under uncertainty, and targeted reopening when assumptions, evidence or purpose change. Persist historical release judgments and explain the reason for each transition. No universal optimality or permanent finality claim.

## Sufficiency, not finality

A release is sufficient relative to a versioned purpose, operating envelope, acceptance standard, candidate and evidence. Define all five explicitly. Distinguish essential obligations from optional opportunities. A nonempty opportunity backlog does not prevent functional sufficiency. Resource exhaustion does not make an essential obligation optional. Unknown evidence is not positive evidence. A claimed economically unjustified optimization must identify its expected benefit, incremental cost, uncertainty, and decision threshold; lack of budget is a separate condition, not proof of diminishing returns. Do not claim universal optimality.

Persist the sufficiency judgment and conditions that invalidate it. Reopen only affected obligations when purpose, operating assumptions, requirements, candidate or evidence changes, with a reason and review of dependencies; retain past judgments as historical facts. A completed release remains a versioned accomplishment even when future releases are needed. Independently verify all mandatory criteria on the same candidate before release acceptance. No perpetual improvement loop is required. The system must demonstrate these decisions on at least two bounded objectives, not just describe its own development process.

Map each mandatory criterion to one or more goal IDs using goal_ids. All G01–G09 must receive substantive coverage. In the completion proposal include purpose, operating_envelope, evidence_standard, reopening_triggers, optional_backlog (may be empty), release_boundary and out_of_scope. Optional backlog entries should identify expected benefit, incremental cost, uncertainty and intended decision criterion. These fields describe the release boundary, not a prescribed architecture.
