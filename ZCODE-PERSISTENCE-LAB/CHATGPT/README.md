# Persistent intelligence laboratory — R002 (canonical)
Status: DESIGN / NOT RUN. Supersedes R001's four different fault lanes; R001 remains in Git history (30a971d711091f6e5c9f9ad1d10629d33312f98c). Owner approved the independent conceptual design and requested experimentation around it. No external research used for this architecture.

## ERA I — Fundamental architecture
Build around persistent objectives, evolving knowledge and disposable intelligence, not agents. An agent is a temporary allocation of reasoning capacity; it does not own goals, knowledge or committed effects.

Seven persistent entities: Goal (what must be accomplished), Task (decomposable work), Class (reprogrammable behavioral contract), Agent (logical allocation of intelligence), Context (selected relevant knowledge), Artifact (materialized work/discovery), Evidence (proof of occurrence/correctness). Identity, version, dependencies and provenance connect entities. The system should survive simultaneous loss of every active AI agent without losing verified intelligence or the ability to continue.

## ERA II — Agent civilization
One general-purpose base agent class, composed from operators: Observe, Discover, Reason, Decompose, Plan, Construct, Execute, Experiment, Challenge, Verify, Repair, Synthesize, Optimize, Delegate, Learn. Compose with objective, context, tools, constraints and acceptance. Examples: researcher = Discover + Reason + Experiment; reviewer = Challenge + Reason + Verify; engineer = Plan + Construct + Execute + Verify; coordinator = Observe + Decompose + Delegate + Repair. Operators are behavior contracts, not claims of deterministic implementation. Successful specializations may become versioned classes after verification.

An agent has immutable logical identity, lineage, class version, objective, capabilities, constraints, context, state, mailbox, workspace, discoveries, checkpoints, evidence and incarnations. An incarnation is one model/process/session; it can be replaced without changing logical identity. New agents get new isolated workspaces; new incarnations of the same agent retain identity and use a new attempt directory. Filesystem is a projection of canonical state, not a shared mutable database.

Lifecycle: Necessity → Genesis → Activation → Operation → Evaluation → Evolution → Assimilation → Retirement. Evolution can resume, repair, replace or reproduce; assimilation turns verified work into reusable knowledge. Dormant, dependency-waiting, message-waiting and archived agents need no active inference; deterministic execution should not require a model. Active reasoning and recovery allocate inference only when necessary. Task graph replaces permanent AI orchestrator; runtime owns scheduling, dependencies, messages, leases, checkpointing, fault detection, effect reconciliation/deduplication, verification, authorization, budgets and cancellation. AI handles uncertain decisions. Scheduler should prioritize critical dependencies, expected information gain, uncertainty, cost and evidence of progress; test rather than assume benefit.

## ERA III — Accumulating intelligence
Unknown problem → AI reasoning → verified solution → generalized algorithm → reprogrammable computational capability → deterministic reuse. Convert successful command sequences into procedures, reasoning patterns into operators, failure resolutions into recovery capabilities, mathematical discoveries into verified engines, successful behavior into classes, coordination into algorithms, relevant information into indexed knowledge, context selection into generation strategies. Promote only after independent verification, scope checks, regression and versioning; permit rollback. Familiar problems should consume progressively less inference without sacrificing correctness; this is a hypothesis, not a measured result.

## Four architectures, identical trials
| Lane | Persistence authority | Recovery boundary |
|---|---|---|
| 01 | Session | Retain/reattach native ZCode session; no external goal/agent reconstruction |
| 02 | Agent | Replace session/incarnation, preserve logical agent and its checkpoint |
| 03 | Goal | Replace entire agents and coordinator; durable goal, task DAG and verified evidence |
| 04 | Intelligence | Everything in 03 plus reusable verified classes, operators, repairs and deterministic capabilities |
All use same objectives, seeds, provider/model where feasible, concurrency and resource ceilings, fault schedule and independent oracle. Lane 04 may spend budget on assimilation; count that cost. No cross-lane knowledge leakage; external observer logs cannot become recovery inputs except through each architecture's admitted substrate. Measure cold start, recurrent tasks and novel variants. The prior 24 faults are now common treatments, not lane definitions. Four initially blank workspaces and separate controller metadata; top-level sessions launch in parallel. Each lane receives the same subagent opportunity, bounded to one concurrent child per lane initially (eight total); larger equal limits in subsequent trials. See PROTOCOL.md and SUBAGENT-LIFECYCLE.md.

## Core hypothesis
A computational system should improve as replaceable agents leave verified reusable knowledge behind. 04 is expected to outperform on repeated and related workloads, but 01–03 are legitimate controls and may outperform it on short tasks. No claim until measured. Endurance qualification is programmable per run; optional elapsed-time requirements must be real wall time, not simulation. No live launch, native CLI/ACP verification, implementation or qualification yet. Do not overwrite the public repo with private transcripts. Git cannot track empty directories; initialize them locally. Only lab-owned processes/files, simulated dependency faults, explicit finite budgets, respect stop and provider limits. ASDF grant/profile discrepancy must be reconciled before launch.

## R003 executable substrate
The standalone Rust implementation is now available in [runtime/](runtime/README.md). Native Windows validation: [8/8 integration tests passed](runtime/VALIDATION-R003.md), including deterministic cross-goal capability reuse. R002 remains the canonical experimental architecture; the four comparative lanes and endurance trials remain unexecuted. No AI wiring.
