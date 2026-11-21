# R002 Observed Results — 2026-11-21 (local)

Environment: DESKTOP-9KB03LU / Windows x64 / ZCode CLI 0.18.0 / source checkout `C:\0-BlackBoxProject-0\ZCode` HEAD `29628c9` / Labs before edits `7e03426`.

## Research pulls
Context7 `/zai-org/zcode` on V4 transport and dynamic workflows; `/websites/agentclientprotocol` ACP comparison. Upstream and locally checked source locations in `CONTEXT7-SOURCE-ATLAS.md`.

## Protocol observations
Command: `node scripts/probe-protocol.ts`
Output (sanitized):
```text
PROBE_SENT=3_READ_ONLY_NATIVE_PROTOCOL_REQUESTS
NOTIFICATION=startup/storageState (4+)
QUERY_sessions=PASS count=5 result_keys=sessions
PROBE_SENT_SUBAGENTS=READ_ONLY
QUERY_workflows=PASS count=15 result_keys=workflows,invalid,dir
QUERY_subagents=PASS count=0 result_keys=revision,childSessionIds,running,ended
QUERY_plugins=PASS count=67 result_keys=plugins,diagnostics
PROBE_FINAL=RESPONDED failed_requests=0
```
Exit code 0 in 11.69s. Earlier initial probe: session/list succeeded, plugins/list failed -32602 due missing workspace; then corrected from source schema and all four passed. Protocol child closed; no model calls sent.

## Store read-only counts
Command: `sqlite3.exe -readonly ~/.zcode/cli/db/db.sqlite SELECT ...`
```text
stored_sessions=3348
child_sessions=2719
task_links=1495
workflow_runs=0
workflow_definitions=0
```
The 15 saved workflow files are in `ARBOR/.zcode/workflows`, independent of runtime journal.
Agent definition files: 93 non-backup. Skills CLI 562; plugins CLI 68; commands CLI 19.

## HQ team
Command: direct Bun executable `C:\Users\VIVIM.inc\.bun\bin\bun.exe` with `repos_hq/hq/bin/hq.mjs team --json`.
```text
HQ_TEAM_EXIT=0
HQ_TEAM_JSON_KEYS=agents,defects,generatedAt,liveSessions,nextMoves,providerHealth,seats,tasks
agents=93; seats=4; tasks=1; defects=157; nextMoves=131
```
This is a health/report snapshot; not evidence that four seats are actively executing or that all 157 defects are product regressions.

## Toolchain note
`bun run scripts/probe-protocol.ts` via PATH-selected Bun shim failed with remap error. `node scripts/probe-protocol.ts` passed on Node 24. Direct Bun executable at `~/.bun/bin/bun.exe` returned version 1.4.2 and ran HQ team JSON successfully.

## Remaining unverified
Native V4 subscriptions and live steering, provider auth/model calls, actual spawned fanout, dynamic workflow run, cross-session message delivery and receipt, graphical desktop controls. Do not mark these PASS.

## V4 live subscription experiment
Command: `node scripts/probe-v4-subscribe.ts`, ran from Desktop Commander. Initial request omitted `connectionId` and failed `-32603`. Local source contract revealed required `connectionId` + `clientMode`; corrected request:
```text
V4_WORKSPACE_KEYS=workspaceKey,workspacePath
V4_SUBSCRIBE_SENT=READ_ONLY
V4_ACK=PASS mode=snapshot
V4_FRAME_RECEIVED=YES
V4_PROBE_FINAL=RESPONDED ack=true frames=1 notifications=5 errors=0
```
Exit 0, 10.84 seconds. This is a confirmed initial V4 snapshot subscription, NOT proof of state-changing controls, delta replay or persistence.

## Combined SITREP regression
`powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts/sitrep-r002.ps1 -V4 -Team` returned full base counts, `V4_ACK=PASS`, `V4_FRAME_RECEIVED=YES`, `v4_probe_exit_code=0`, `HQ_TEAM_SITREP_COMPLETE`, `SITREP_R002_COMPLETE`, and exit 0 in 17.54 seconds. Session count was 3349 at this later snapshot (one more than initial 3348), origin of difference unverified. No prompt sends or protocol mutation requests were issued.
