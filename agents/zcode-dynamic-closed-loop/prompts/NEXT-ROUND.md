# Next experimental rounds (only after a real pilot)

## R0 — Compatibility gate
- Confirm installed workflow mode, dynamic authoring skill, model registration, ZCode parser, script argument metadata and `world.run` approval surface.
- Compile generated template in main session. Fix diagnostics in package; record exact ZCode commit/version and real run ID if launched.

## R1 — Scratch repository pilot
- One tiny real bug with a known failing test and pinned branch. Execute source lock, small/full routing, two explorers, implementer, independent verifier, audit, at most one repair, keeper handoff.
- Check actual paths, exits and no cross-scope changes. Fail if the agent claims green without a process exit.

## R2 — Model bake-off
- Freeze 30 tests across 10 bugfixes, 10 features, 10 reviews. Compare ERNIE, Nemotron, Particle and North Mini with matched tools/context. Track verified pass rate, time to accepted change, token usage, failed tool calls, provider 429, rework.
- Keep model routing deterministic and compiled statically; publish scorecard, confidence intervals and the chosen fallback thresholds.

## R3 — ARBOR dev worktree integration
- Only after permission and R004 sprint convergence, run on a new ARBOR worktree with disjoint scope and real Node24 gates. Main remains keeper-owned. Disallow direct code mutators for read-only reviewer where host supports permission profiles; audit tool permissions.

## R4 — Resumability / self-healing
- Test cancellation, stale-source detection, provider saturation, transient failure, amended script with stable names, cold replay, duplicated run avoidance and run-pane reconciliation. No automatic self-launch recursion.

## R5 — Verified acceleration economics
- Compare fixed-model baseline vs dynamic selection with exact task-matched resource measurements. Optimize accepted tasks per request, not raw subagent count. Keep budgets, stop states and owner approval first-class.