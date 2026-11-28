# Gold seed and four dogfood experiments — R004

Owner decision (2026-11-28): retain one primary build and start four isolated ZCode dogfood experiments from the gold seed. Native runtime remains provider-neutral. ZCode is an execution adapter, never the owner of the canonical runtime, goal or verification authority.

## Source and isolation
Gold seed: native-qualified R003 source pin f837042b76e2bd438f107a6b93360306da6d98ef, eight native tests passed; acceptance baseline 28/100. Gold means frozen reproducible reference, not completed architecture. Primary build continues on main under runtime/acceptance-tracker.json. Do not fork four independently evolving production codebases. Each dogfood instance uses the same pinned executable/source and adapter version, with private workspace, store, session IDs, logs, process ownership and budgets. Experiment metadata lives outside the four initially blank workspaces. No shared writable runtime state or cross-lane learning. Shared read-only seed and provider configuration permitted; no credential copying. Four experiments are operational instances, not Git branches.

## Four controlled arms
01 persistent session: native ZCode session resume only; observer evidence never used to reconstruct lost goals.
02 persistent agent: durable logical agent and checkpoint survive session replacement.
03 persistent goal: durable goal/task graph and evidence survive agent/coordinator replacement.
04 persistent intelligence: 03 plus verified reusable class/operator/program/repair accumulation, automatic reuse where implemented.
Unimplemented authority boundaries must be marked unsupported; no claim of comparative qualification until enforced and tested. All four have the same objective, input, oracle, model when feasible, fault schedule and resource ceilings. Start with four top-level sessions, one per lane; optional equal child slots later. Use finite initial smoke budgets; do not launch unattended endurance until stop and recovery are proven.

## Adapter contract
Probe actual installed ZCode CLI/ACP, explicit cwd, session creation, prompt delivery, streaming events, cancellation and resume. Use supported interfaces only. Bind run/lane/goal/task/agent/incarnation/provider session and owned process IDs in durable receipt. Adapter may request claim, read bounded context, submit result/checkpoint and send messages; Host verifies and admits effects. Native core remains runnable with no ZCode. Separate provider errors, transport loss, idle, actual completion and unknown effect. Reconcile before retry; never auto-relaunch after cancellation, authorization failure or exhausted budget.

## Dogfood progression
D0: pin seed and verify native baseline. D1: launch four isolated sessions; require unique IDs, cwd and overlapping activity, one deterministic challenge and independently checked artifact each. D2: test resume/replacement and controlled fault injection on owned processes only. D3: add capabilities one gate at a time to primary build, verify native, pin new version, redeploy all four or explicitly record a new experimental cohort. D4: compare recurrent and novel workloads; only after real stop/recovery tests run 36h and 72h qualification with bounded budgets. No real provider outages or changes to shared configuration.

## Promotion loop
Dogfood failure -> receipt -> reproducible minimal native test -> gate ID -> primary implementation -> regression test -> acceptance evidence -> new pinned seed -> all four arms. Do not merge generated agent code or claims without review. Preserve original seed for reproducibility; never overwrite run data. No score increase without qualifying evidence. Current implementation has no ZCode adapter and four-arm authority enforcement remains open (E45–E49). Smoke tests do not imply those gates pass.
