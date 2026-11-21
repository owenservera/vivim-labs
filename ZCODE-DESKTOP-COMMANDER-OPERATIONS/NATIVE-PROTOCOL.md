# Native ZCode Protocol — Practical Operator Reference R002

## Verified endpoint
- Installed launcher: `C:\Users\VIVIM.inc\.local\bin\zcode.cmd` delegates to `node "C:\Program Files\ZCode\resources\glm\zcode.cjs"`.
- Run `zcode app-server` for stdio NDJSON. stdout is protocol frames; stderr is diagnostics. Prefer direct `node.exe zcode.cjs app-server` for a programmatic child process on Windows.
- Our `scripts/probe-protocol.ts` starts one child, sends read-only requests, never streams raw session titles or provider credentials, then closes it. Execute with Node 24 native TypeScript stripping: `node scripts/probe-protocol.ts`. Bun's ordinary PATH shim was broken during R002; its separate `~/.bun/bin/bun.exe` exists.

## Two protocol layers — never confuse them
1. Legacy compatibility reads: `{ "id":"s1", "method":"session/list", "params": {"limit": 5} }`, with replies `{id,result}` or `{id,error}`. There is NO `jsonrpc` key required in these native messages.
2. V4 control/streaming: `v4/connection/flow`, `v4/controller/subscribe`, `v4/conversation/subscribe`, `v4/conversation/rowsRange`, `v4/conversation/workflowRuns`, `v4/commands/query`, `v4/command` and `v4/conversation/frame` notifications. V4 command envelopes carry revisions/client identity and must validate state.
3. ACP: external standardized editor-agent protocol. Optional future adapter; not the native ZCode protocol.

## Tested read queries
```json
{"id":"r_sessions","method":"session/list","params":{"limit":5}}
{"id":"r_plugins","method":"plugins/list","params":{"workspace":{"workspacePath":"C:\\0-BlackBoxProject-0\\ARBOR","workspaceKey":"C:\\0-BlackBoxProject-0\\ARBOR"}}}
{"id":"r_workflows","method":"workflows/list","params":{"workspace":{"workspacePath":"C:\\0-BlackBoxProject-0\\ARBOR","workspaceKey":"C:\\0-BlackBoxProject-0\\ARBOR"}}}
{"id":"r_subagents","method":"session/subagents","params":{"sessionId":"<real-session-id>","endedLimit":5}}
```
Workspace schema requires `workspacePath`, `workspaceKey`, optional `workspaceIdentity`, `remoteSessionId`. `session/list` permits `limit`, `includeArchived`, `sessionIds` and workspace filtering. `session/subagents` returns `revision`, `childSessionIds`, `running`, `ended`.

## Live results R002
- `session/list`: PASS; 5 sessions requested/returned.
- `plugins/list`: PASS; 67 in this workspace context (CLI globally reported 68). Different inventories are not an inconsistency to silently hide.
- `workflows/list`: PASS; 15 saved ARBOR workflows.
- `session/subagents`: PASS; sampled recent session returned 0 children. Other historical parent sessions contain children per DB.
- Full protocol probe: four requests, four successes, zero errors, exit 0; total ~11.69s.
- `plugins/list` without workspace failed `-32602` in initial probe; fixed by reading the schema.

## Safe operations we can implement next
- Session discovery, explicit resume and read-only transcript retrieval, no prompt sending by default.
- V4 controller/session subscription and state revision snapshots; determine `clientHello` + resume/delta contract before any V4 writes.
- `v4/commands/query` for available commands. Eventually use `v4/command` with command IDs, baseRevision/logEpoch and request receipts.
- Model discovery and selection from authoritative process registry. Do not assume `--model` exists: installed CLI help only advertises `/model` in-session.
- Cancellation and idempotency: successful remote process exit is not a model receipt; correlate requests by ID and session revision.

## Dangerous operations — no automatic invocation
`session/setModel`, `session/send`, `v4/command`, `workflows/updateMeta`, `workflows/delete`, plugin enable/install, model connectivity tests, file edits, and tool grants can alter live state or consume quota. Require a specific user task, explicit scope, and verification.

## Relevant local schema paths
`packages/shared/src/zcode-protocol/index.ts` (request schemas/method constants);
`packages/shared/src/zcode-protocol-v4/transport.ts`;
`packages/shared/src/zcode-protocol-v4/command.ts`;
`apps/zcode-cli/packages/bootstrap/src/zcode-protocol/server.ts`.

## Verified V4 sessions-index snapshot subscription (R002)
A new read-only probe `scripts/probe-v4-subscribe.ts` starts ZCode app-server, calls `session/list`, extracts a grounded workspace key, and sends:
```json
{"id":"sub","method":"v4/conversation/subscribe","params":{"topic":"sessions-index/<workspaceId>","visibility":"background","connectionId":"dc-probe-<pid>","clientMode":"desktop-continuous"}}
```
`connectionId` and `clientMode` are required by `v4ConversationSubscribeParamsSchema` in `packages/shared/src/zcode-protocol-v4/transport.ts`. The first attempt without them failed with `-32603`; the corrected source-grounded request PASSED. Result: `V4_ACK=PASS mode=snapshot`, `V4_FRAME_RECEIVED=YES`, `V4_PROBE_FINAL=RESPONDED ack=true frames=1 notifications=5 errors=0`, exit code 0, runtime 10.84 s. The script closes its child process. Read-only initial snapshot verified; multi-frame deltas, reconnection, state revision handling and subscription cancellation remain UNRUN.
