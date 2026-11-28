# R003 native validation receipt — 2026-11-28

Status: NATIVE_PASS (standalone runtime only). Four-architecture comparison and endurance runs NOT RUN. No AI integration.

## Reproducible source and execution

Source pin: f837042b76e2bd438f107a6b93360306da6d98ef
Runtime: ZCODE-PERSISTENCE-LAB/runtime
Windows x64 Rust 1.99.1, Cargo 1.99.1, PowerShell 7.6.3. The approved ASDF terminal ran the pinned files in an isolated temporary source directory. Cargo used the workstation's already configured target directory. Command: cargo test --offline --locked --manifest-path <temporary-source>/Cargo.toml. No AI/provider process was launched. Dependency cache already present.

| File | SHA256 |
|---|---|
| Cargo.toml | 9903af15553d553f2b43043dbc9acc000354bf82c0140b0d11ce01c2dededbb1 |
| Cargo.lock | 9dbf02be0f220d088cd2686c4a4e7d6f9c6485177b5b0bf26ef1bd1ba57d790a |
| src/lib.rs | b1a2ef0f96620cee6f75c0deB77a0485002cf422c610105cd6076d013fcc18f1 |
| src/main.rs | 10a4f11ec76e4dabc2b85a1f8593a8ee5005f8755cb6816e5692f2c2125c0441 |
| tests/runtime.rs | 01728032fc1d3747c385b14b346267263c69784c1b0247a15436e7c159f52d33 |

ASDF session ctl-8578a66af9da7776, exit 0, elapsed 20,030 ms; compilation 12.47 s; eight integration tests 2.43 s. All eight passed, zero failed or ignored.

- actual_separate_processes_recover: same request replay across fresh processes, monotonic revisions, state reopened.
- restart_fences_old_worker_and_keeps_checkpoint: expiry and new generation, persisted checkpoint, stale submission rejection, dependency release, rejected proposal retry, goal completion, duplicate commit rejection.
- promote_then_reuse_and_disable: reject wrong program, admit sample-verified program, enforce class capability binding, verify novel input, disable.
- autonomous_scheduler_reuses_capability_across_goals: fresh process drains new goal; two deterministic computations, independent oracle checks, dormant worker activation and completion.
- incarnation_mailbox_request_replay_cancel: identity preserved across incarnations, acknowledgement and idempotency, renew lease, cancellation prevents late submission.
- busy_agent_cannot_hold_multiple_running_leases: one running lease per worker; another worker may claim independent task.
- budget_exhaustion_and_explicit_failure: bounded retries, exhausted task not ready.
- corruption_fails_closed_and_uncommitted_temp_is_ignored: invalid journal rejected, orphan temp ignored.

Earlier source pin 1e6261b91ee87e2fdd8313730b8908d4551f740d: initial six tests passed in session ctl-04ecb1718667a08f. Scheduler revision a796ff56e0da58f71a940c906506b4dc680c69f9 failed compile with E0308 in ctl-d85cf4106a1350b6; explicit return fixed in f837042. Rebuilt with lockfile and reran full suite above.

Limits: logical clock is caller-driven; no wall-clock watchdog, no real process-kill during a write, no production power-loss/multi-host qualification, no AI wiring, no arbitrary external side effects, no 36/72h endurance claim. Sample promotion is not universal proof. Full-history replay/snapshots are a prototype tradeoff. No four-lane comparative results. Native test pass demonstrates provider-independent executable semantics, not the full future civilization.
