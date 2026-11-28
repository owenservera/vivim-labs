# Goal: determine and achieve completeness

You have the supplied objective and starting files. Build an executable system that fulfills that objective. A second, equally important objective is to determine what it means for this system to be complete.

Before broad implementation, inspect the actual source and formulate a finite, versioned completion contract. Derive required observable behaviors from the objective, distinguish mandatory behaviors from optional enhancements, specify a bounded workload and operating conditions, identify failure cases and what would falsify each claim, and state what is explicitly outside the release boundary. Include independently executable acceptance checks and evidence requirements. Define how all essential checks jointly imply the objective, rather than merely listing features. Record the contract and its rationale in your own workspace. Challenge it for omitted essential behavior, circular tests, vacuous success, and unverifiable claims. Freeze the initial release contract before claiming implementation progress against it.

Persist a goal, remaining obligations, verified results, failed attempts, source revision or content digests, and resumable next actions. Prioritize the largest unresolved mandatory dependency. Reuse verified deterministic computation rather than spending reasoning capacity again. Use model capacity only when needed. Do not assume that an available budget must be spent. On budget exhaustion, interruption or uncertain execution, checkpoint and yield; never declare completion or discard outstanding obligations.

A contract may be revised only with a dated reason, impact on prior evidence, and explicit handling of any removed requirement. Do not narrow the original objective to make tests pass. Material scope reductions require controller approval; newly discovered mandatory gaps reopen affected claims. Optional enhancements go into a separate future backlog rather than endlessly expanding the current release.

A candidate is complete only when the frozen mandatory contract is satisfied by executable evidence on the same candidate source, relevant regressions and failure cases have been checked, and no unresolved mandatory failure or unknown effect remains. Submit the contract, test commands, evidence, source digest, known limitations, and a reproducible run procedure. An external evaluator will independently verify the claim. Until then, report CANDIDATE_COMPLETE, not COMPLETE_VERIFIED. Exhausted resources mean WAITING_BUDGET, not success. A failure that cannot be repaired within the allowed effort is an explicit bounded failure.

Work only within your assigned workspace. Do not access unrelated files, services or other conversations. No deployment, purchases, network or privileged actions unless separately authorized by the controller.

## Common system goals — R012

These outcomes are mandatory for this release, not a prescribed architecture. The external experiment controller owns actual inference admission, but the system you build must independently implement and demonstrate its own routing and resource allocation capabilities. Do not assume the controller's existing policy is your solution. G01 is the first explicit system goal; prioritize dependencies as necessary. Interpret optimality relative to a bounded workload, measured baselines and declared constraints, not universal optimality.

### G01 — Optimal model router
Select the least sufficient authorized intelligence for each task, including no inference when deterministic execution suffices; adapt model, reasoning effort and escalation to actual task needs.

Demonstrate: Executable routing on a bounded varied workload; compare against fixed-model and fixed-effort baselines with verified outcomes, resource use and failure rates. Report uncertainty, tradeoffs and cases where routing is worse. Optimal means best demonstrated under declared workload and constraints, not universal optimality.

### G02 — Intelligent resource allocation
Convert live subscription limits, user-defined budgets, resource availability and priorities into safe allocations while preserving work when capacity runs out.

Demonstrate: Execute controlled budget and contention traces, including stale observations, exhaustion, failure, reset, unknown costs and concurrent requests. Never silently overspend, borrow reserved allocations or purchase capacity; show measured progress per resource unit.

### G03 — Autonomous goal management
Infer a finite, defensible definition of completeness from objectives; discover missing obligations, organize dependencies and prioritize the most valuable unfinished work.

Demonstrate: Derive and freeze a contract on a bounded novel objective, detect omitted essential behavior, revise transparently, track obligations and distinguish candidate completion, budget pause and verified completion.

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

Map every mandatory contract criterion to one or more goal IDs using a goal_ids array. Every G01–G08 must have substantive acceptance coverage. Joint criteria may cover multiple goals when the evidence really supports them. Do not replace implementation with a design document. Define the smallest meaningful release that satisfies every goal, and treat optional extensions as future work.
