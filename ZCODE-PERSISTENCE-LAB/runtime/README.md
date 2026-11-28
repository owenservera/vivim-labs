# Persistent Intelligence Runtime — R003

Standalone Rust library and CLI implementing the R002 persistent-intelligence design. No AI provider, ZCode, MCP, external effect adapter, or runtime network access. JSON commands are the boundary for an arbitrary future executor; a deterministic pure expression interpreter is the first native executor.

## Build and execute

From this directory, with Rust 1.99.1 (the native test toolchain):

    cargo test --offline --locked
    cargo build --release --offline --locked

Offline builds require the locked dependencies already cached. On a fresh machine, first obtain the locked dependencies through Cargo's normal trusted package mechanism. Run the binary as:

    persistent-intelligence-runtime <store-directory> inspect
    persistent-intelligence-runtime <store-directory> ready
    persistent-intelligence-runtime <store-directory> apply <request-id>
    persistent-intelligence-runtime <store-directory> apply-file <json-file> <request-id>
    persistent-intelligence-runtime <store-directory> drain <goal-id> <max-steps>
    persistent-intelligence-runtime hash

The apply/hash commands consume JSON from standard input; apply-file reads a JSON file. Each mutating request can carry an optional stable request ID; identical replay returns the original receipt without another revision, whereas conflicting reuse is rejected. Nonzero exit indicates a rejected command or store failure. Treat an uncertain outcome as unknown and repeat the same request ID, not a new one. A store can be reopened by a completely different executable process.

PowerShell example (after building):

    $exe = '.\target\release\persistent-intelligence-runtime.exe'
    $store = Join-Path $env:TEMP ('persistent-intelligence-' + [guid]::NewGuid().ToString('N'))
    '{"cmd":"goal","id":"demo","objective":"independent objective","claim_budget":10}' | & $exe $store apply req-1
    & $exe $store inspect

For an end-to-end automated example, see the test autonomous_scheduler_reuses_capability_across_goals in tests/runtime.rs.

## Generalized entities and commands

- Goal: goal, seal, cancel. Goal completion is derived only from verified committed tasks and explicit sealing. A goal has a bounded claim budget.
- Task: task, claim, checkpoint, renew, fail, submit, verify. New tasks reference existing tasks within the same goal, preventing cycles. Ready work depends on committed predecessors, remaining attempts and budget.
- Class: class. Immutable ID/version and composition of 15 behavioral operators; capabilities explicitly associated with the class.
- Agent: agent, activate, retire. Immutable identity, parent lineage, goal/class/context references, replaceable incarnation, active or dormant state. One running lease per agent. Retired agents can be explicitly activated; only never-activated dormant agents auto-activate through the scheduler.
- Context: context. Structured JSON and verified artifact references.
- Artifact and Evidence: produced only by verification. Every submission is checked against a separately supplied expected output digest. A failed verification records evidence and retries only within bounds. One committed artifact per task. Evidence includes the task and lease generation.
- Mailbox: send, ack. Durable, deduplicated message ID and recipient acknowledgement.
- Capability: promote, disable, execute. A versioned bounded pure expression is admitted only after it reproduces at least two distinct verified examples. A new task must still pass its own oracle; example success is not a mathematical proof of general correctness. Disabling prevents new uses.
- Runtime: tick, step, drain. Explicit monotonic logical clock advances lease expiration; it does not equate to wall time. A deterministic step verifies submitted work first, then selects runnable capability-backed tasks using pending-dependent count and stable IDs, checks class/agent eligibility and claims, executes and verifies in one transaction. Drain performs at most the caller's step limit and stops at idle. Tasks without a deterministic capability wait for an external executor; none is wired here.

Expression instructions: input, const, field, array, add, multiply, concat, equal, sha256, if. Operations are fuel/depth/result limited and integer arithmetic is checked. No shell, filesystem, network, ambient clock, or arbitrary code execution is accessible from an expression.

## Durability and trust boundary

The journal stores immutable sequential revision envelopes with SHA-256 hash linkage, commands, results and snapshots. An OS file lock serializes cooperating readers/writers. Write a temporary file, sync it, then rename it to the next revision. On reopen, replay every command and compare each state snapshot/result; reject broken chains or mismatches. Uncommitted temporary files are ignored. This is process-restart persistence, not a claim of verified power-loss durability, protection against malicious rewriters of the entire store, or detection of truncation of its final suffix without an external anchor. No authentication layer: only trusted callers should have write access to the store and submit oracle definitions/commands. A task's expected digest must come from an independent source for meaningful verification.

Lease generations, incarnation IDs, monotonic expiration, and explicit cancellation reject stale operations. Requests and output admission prevent duplicated internal committed effects. There are no external side effects; exactly-once external delivery is deliberately not claimed.

This initial backend replays full history and writes full snapshots, so storage and replay scale poorly. It is a small correctness-oriented lab implementation, not a production database. Multi-host coordination, storage compaction, cryptographic external anchoring, wall-clock watchdogs, richer oracles, arbitrary general-purpose execution, automated discovery/class generation, independent holdout promotion policies, real process-kill fault injection and the four architecture comparison are future lab work. R003 does not claim 36h/72h endurance. No AI wiring or AI-generated work is required for execution.

## Acceptance

Run tests/runtime.rs through Cargo. Tests cover process reopening, stale lease fencing, checkpoint retention, dependency release, verification rejection, capability promotion and cross-goal deterministic reuse, mailbox replay, cancellation, corruption rejection, isolated temporary-file recovery, bounded retries and one running lease per logical agent. See VALIDATION-R003.md for native evidence once recorded.
