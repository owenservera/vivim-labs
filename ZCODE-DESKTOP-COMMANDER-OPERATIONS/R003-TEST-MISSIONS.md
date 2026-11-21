# Round R003 — Engineering Test Missions

## Prioritized gates
**G1 Native streaming control**: INITIAL SUBSCRIBE PASS in R002: `v4/conversation/subscribe` with connectionId and clientMode yielded snapshot ACK + frame. REMAINING: decode revision/logEpoch, validate resync/reconnect/deltas and `v4/commands/query`, implement persistent read-only subscriber. NO writes to sessions.

**G2 Subagents**: select persisted parent with known child IDs using read-only SQLite; query `session/subagents` to validate running/ended projection and pagination. Next, owner-authorized two-read-only-agent spawn with exact model/provider and bounded budget. PASS = 2 distinct child session IDs, separate transcripts, parent notifications, receipts, no writes.

**G3 Workflow execution**: read 15 saved ARBOR workflows and identify exact `ListSavedWorkflows` / `ListWorkflowRuns` / `CreateWorkflow` shapes from local source. First isolate a no-effect workflow in a disposable directory; test validation/run/cancel/restart/resume. Existing live ARBOR workflows must NOT be launched as a general probe.

**G4 Messaging**: inspect HQ team status via direct Bun executable; establish current broker truth vs stale `~/.zcode/comms`; test receipt and idempotent wake using disposable seats only. Never wake a live occupied seat.

**G5 Provider selection**: read live process provider registry, evaluate explicit model IDs; perform approved low-token trials across the four priority providers. Compare error classes, latency, prompt cost, and tool capabilities. No silent paid fallback.

**G6 Headless UI**: ZCode `computer-use` plugin exists and is enabled in at least one plugin inventory. Test its discovery/permission grant separately; Desktop Commander itself lacks native screenshot/mouse primitives. Never assume desktop UI control works merely because a plugin name exists.

## Acceptance ledger conventions
PASS = command + actual output + exit 0. FAIL = error with evidence. UNRUN = not attempted. BLOCKED = prerequisites/cost/permission missing. PARTIAL = data returned but semantics unresolved.
Every new test records UTC timestamp, host version, process PID/session ID if relevant, workspace, exact command, result, counts, errors, artifacts. Keep sensitive data local; no model prompt contents.

## Concrete next actions
1. Run `node scripts/probe-protocol.ts` from this folder to re-establish native connection.
2. Use `powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts/sitrep.ps1` for fast inventory.
3. Read `WORKFLOWS-AGENTS.md`, `NATIVE-PROTOCOL.md`, `COMMS-AND-THREADS.md`.
4. Inspect relevant local ZCode source at `C:\0-BlackBoxProject-0\ZCode` and official Context7 ZCode ID `/zai-org/zcode`.
5. Do not create fake workflow run results; do not alter current ARBOR workflows or active sessions without explicit task authorization.
