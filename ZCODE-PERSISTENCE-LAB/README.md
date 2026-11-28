# ZCode persistence laboratory — R001

Status: DESIGN / NOT RUN. Owner direction: start with four blank subfolders and four independent parallel ZCode instances; expose different operational failures repeatedly until a persistent lifecycle is demonstrated. Design before execution.

## ERA I — The laboratory

A new run contains four completely blank agent working directories 01, 02, 03, 04. No pre-populated project instructions, shared agent state, or preinstalled recovery solutions. Initial objectives are delivered via sessions. Controller metadata, logs, prompts and receipts live outside the four directories. Initially the controller observes and injects failures, not silently solving every problem. Baseline native ZCode behavior before adding recovery mechanisms. Four distinct instance/session identities; run concurrently, with isolated working directories. Shared provider credentials remain under their existing configuration; never copy or log them.

## ERA II — Four lanes

| Round | 01 Session immortality | 02 Operational resilience | 03 Context immortality | 04 Orchestration immortality |
|---|---|---|---|---|
| 1 | Premature voluntary finish | Invalid command | Long conversation | Spawn four subagents |
| 2 | Interrupted inference | Command timeout | Context compaction | One subagent fails |
| 3 | Worker process terminated | Missing dependency | Fresh conversation | Unresponsive subagent |
| 4 | ACP disconnected | Unavailable tool | Working memory loss | Conflicting results |
| 5 | Parent session terminated | Simulated write failure | Partial checkpoint corruption | Duplicate task execution |
| 6 | Controller restarted | Simulated provider outage | Replacement model | Coordinator terminated |

01: recover original objective and last verified checkpoint, not restart work. 02: recognize failure, find alternative, verify repair and retain reusable knowledge. 03: preserve goals, discoveries, decisions, pending work and operational knowledge without replaying all history. 04: retain task graph, recover incomplete assignments and avoid duplicate effects.

Common task: maintain a monotonically advancing, deterministic sequence of numbered artifacts and verifiable receipts, with a finite target per round and a longer endurance campaign. Use equivalent workloads to permit comparison. Lane 04 partitions the workload into independent assignments and reconciles their results. Each round records an unmodified baseline attempt, then one minimal repair and a replay of the same fault; failed repairs are retained as evidence.

## ERA III — Evolution

L0 native ZCode, L1 session recovery, L2 durable goals, L3 operational recovery, L4 replaceable orchestration, L5 persistent lifecycle independent of process, context and model. Inject → observe → repair → repeat. Do not add persistence layers before the baseline fails. Process disposable; agent recoverable and replaceable; goal persistent until verified completion or explicit cancellation.

Acceptance gates:
- Four independent parallel instances operational.
- Automatic recovery following worker termination.
- Goal survives complete context replacement.
- Operational failures automatically remediated.
- Zero duplicate committed effects.
- Coordinator successfully replaced.
- 36 hours continuous goal execution.
- 72 hours with repeated fault injection.

Targets: zero goal loss, lost committed work, duplicate committed effects or human recovery interventions; 100% injected failure recognition; recovery under 60 seconds where feasible; progressively declining context and recovery token cost. Report actual distributions and exceptions, not invented measurements. A 36h/72h run is wall-clock measured; do not substitute simulated time. Provider outages may exceed the recovery target; record detection and wait/recovery separately.

Final test: terminate all four owned ZCode workers and their coordinator, restart controller and demonstrate continuation of all four goals from verified durable state. Explicit cancellation must be respected; never defeat a user stop, provider policy, access boundary or budget. All injected faults restricted to lab-owned processes, files and simulated dependencies. No global network changes, machine-wide process kills, credential changes or interference with other sessions. A watchdog must not be mistaken for proof of native ZCode recovery.

## Execution and evidence

See experiment.json and PROTOCOL.md. Initialize a new local run using Initialize-Lab.ps1; the four workspaces are empty at creation. Do not launch from this repository or store raw private transcripts in this public repository. Commit only reviewed, sanitized results. No real experiments executed at R001. ASDF transport status and grant reporting were inconsistent at design time; reconcile before launch. The actual installed ZCode CLI/ACP flags, session persistence and launch/reconnect behavior must be verified in a harmless preflight, not assumed. Git cannot track truly empty directories: create them at runtime instead of .gitkeep.

The eventual product handoff is experimentally validated primitives for a native, provider-independent Rust Harness, not a dependency on an immortal AI conversation.
