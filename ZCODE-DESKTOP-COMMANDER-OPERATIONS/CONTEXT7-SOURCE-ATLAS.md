# Context7 Source Atlas — R002 (2026-11-21)

## Pulls executed
Resolved official ZCode at `/zai-org/zcode` (high reputation; 35,554 indexed snippets) and supporting Agent Client Protocol at `/websites/agentclientprotocol`. Also resolved `/anomalyco/opencode` as a separate integration reference, but DID NOT query its docs in this round. Context7 calls:
1. ZCode app-server stdio, v4 commands, transport and sessions.
2. ZCode subagents, dynamic workflows, agent model overrides, CLI routing.
3. ACP session lifecycle/permissions/config options as a comparison protocol.

## Source of truth hierarchy
1. Installed CLI `zcode --version`, `--help`, live protocol response and local SQLite metadata.
2. Local ZCode source checkout `C:\0-BlackBoxProject-0\ZCode`, inspected at HEAD `29628c9`.
3. Official upstream ZCode source/documents and Context7 excerpts; version drift possible.
4. Public ACP specification, a SEPARATE protocol, not the ZCode app-server contract.
5. Third-party bridge proposals and issue comments (not authoritative).

## Exact source anchors
- Native V4 transport: https://github.com/zai-org/zcode/blob/main/packages/shared/src/zcode-protocol-v4/transport.ts
- Native V4 commands: https://github.com/zai-org/zcode/blob/main/packages/shared/src/zcode-protocol-v4/command.ts
- Legacy read-only request schemas: https://github.com/zai-org/zcode/blob/main/packages/shared/src/zcode-protocol/index.ts
- Native bootstrap server: https://github.com/zai-org/zcode/blob/main/apps/zcode-cli/packages/bootstrap/src/zcode-protocol/server.ts
- stdio lifecycle: https://github.com/zai-org/zcode/blob/main/apps/zcode-cli/packages/cli/src/protocol-lifecycle.ts
- Sandbox workflow runtime: https://github.com/zai-org/zcode/blob/main/apps/zcode-cli/packages/dynamic-workflow-runtime/README.md
- Background subagent implementation design (historical; verify runtime): https://github.com/zai-org/zcode/blob/main/apps/zcode-cli/docs/design/v2/tool/07-subagent-background-plan.md
- User-facing subagents guide: https://zcode.z.ai/en/docs/subagents
- App-server protocol overview: https://github.com/zai-org/ZCode/blob/main/docs/zcode-protocol.md
- ACP separate specification: https://agentclientprotocol.com/protocol/v2/session-config-options

## Key findings and traps
- **Wire distinction**: ZCode app-server accepts NDJSON frames of shape `{id,method,params}`, not necessarily ACP `{jsonrpc:"2.0",...}`. The live probe received responses from `session/list`, `plugins/list`, `workflows/list` and `session/subagents`. V4 uses the same carrier but versioned methods, subscriptions and command envelopes.
- **Versioned UI control**: `v4/command` takes `{commandId,clientId,sessionId,baseRevision?,type,payload,issuedAt}`; do NOT send without current state, capability handshake and approval.
- **Workflow isolation**: `@zcode/dynamic-workflow-runtime` uses subprocess + NDJSON host bridge, sandbox, cancellation and journal. Its internal child protocol is NOT the public app-server RPC.
- **Agent model**: Child sessions have independent histories; `Agent` tool launches them. Foreground batches and background execution are documented publicly. Presence of definitions does not imply currently running agents.
- **Provider config**: Process registry is execution authority, not stale workspace snapshots (per local `zcode-protocol-entrypoint.ts`). User-level config reading requires secret-safe projection.
- **ACP integration**: An external `zcode-acp` bridge exists, but its compatibility and Windows support are unverified; prefer native ZCode initially. Never send ACP-specific `session/new` to ZCode native interface without an adapter.

## Evidence classification
Documented: upstream source claims. Confirmed: local source or actual runtime response. Unrun: tests requiring live model turns or state mutations. Treat published docs as guidance only if compatible with installed 0.18.0.

## Source-to-runtime validation result
V4 `sessions-index/<workspaceId>` via `v4/conversation/subscribe` now passed a live app-server subscription after conforming to the local V4 schema. Required parameters `connectionId` and `clientMode` were cross-checked in `packages/shared/src/zcode-protocol-v4/transport.ts`; the server returned a snapshot ACK then one `v4/conversation/frame` notification. This confirms an externally invoked read-only streaming entrypoint on the installed runtime.
