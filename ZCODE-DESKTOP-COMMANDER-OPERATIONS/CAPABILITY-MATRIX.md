# ChatGPT ↔ ZCode Capability Matrix — R002

Legend: **PASS** actual local execution with output; **SOURCE** local/upstream contract verified but operation untested; **UNRUN** not attempted; **BLOCKED** additional authority required.

| Capability | Interface | Status / evidence |
|---|---|---|
| Discover Desktop Commander machine | Remote MCP list_devices | PASS online |
| Read/list files and project folders | Remote MCP file API | PASS |
| Run PowerShell and interactive shell | Remote start_process/read_output | PASS |
| Start and read ZCode headless prompt | CLI `zcode -p` | PASS echo with exit 0 |
| ZCode self diagnostics | CLI doctor | PASS |
| List skills | CLI skills list | PASS 562 |
| List installed plugins | CLI plugins list | PASS 68 |
| List custom commands | CLI commands list | PASS 19 |
| Enumerate custom subagent definitions | `~/.zcode/agents` | PASS 93 non-backup |
| Enumerate model rules/provider metadata | JSON projection, redact secrets | PASS 21 provider, 218 model rules |
| Verify provider authentication/quota | Registry/runtime + short model test | UNRUN |
| Select explicit per-session model | `session/setModel` / V4 switch | SOURCE, changing state UNRUN |
| Native app-server stdio NDJSON | Node child process | PASS 4 queries |
| List native sessions | `session/list` | PASS five returned |
| List parent subagent status | `session/subagents` | PASS sampled parent zero |
| Read persisted child graph | SQLite read-only `session`, `session_task_link` | PASS 2,719 child sessions; 1,495 links |
| Resume existing session for a prompt | CLI `--resume sess_...` | SOURCE, UNRUN |
| Fork/rewind session | CLI slash / V4 commands | SOURCE, UNRUN |
| Stream V4 conversation frames | `v4/conversation/subscribe` | PASS: sessions-index subscription ACK (snapshot) and one frame, exit 0; `scripts/probe-v4-subscribe.ts` |
| List saved workflows | `workflows/list` | PASS 15 |
| Query workflow run journal | SQLite read-only | PASS 0 records |
| Start dynamic workflow | ZCode CreateWorkflow/runtime | SOURCE, UNRUN |
| Spawn two new child agents | Agent tool | SOURCE, UNRUN |
| Cancel/resume child agents | TaskOutput/Agent and V4 | SOURCE, UNRUN |
| List workspace plugins via protocol | `plugins/list` | PASS 67 |
| Read plugin skills/docs | Remote filesystem | PASS sample agent/plugin docs |
| Read HQ team roster | `hq.mjs team --json` | PASS 93 agents, 4 seats, 1 task |
| Read legacy comms bus file metadata | `~/.zcode/comms` | PASS stale Oct 7 |
| Deliver/wake another ZCode seat | HQ bus + `SendMessage` | SOURCE, UNRUN |
| Communicate across ChatGPT threads | Shared authorized bridge | DESIGN, UNRUN |
| Create durable send-once receipts | Studio/HQ discipline | SOURCE, UNRUN |
| GUI screen/click/type through Desktop Commander | Connector alone | NOT NATIVE |
| GUI through ZCode computer-use plugin | plugin + permission | UNRUN |
| Rust ARBOR native bridge | ADR / design only | NOT IMPLEMENTED |

## Highest-leverage next implementation
Start with read-only V4 controller and session subscriptions, then wrapper for explicit model selection and two-agent read-only orchestration. Use HQ for team wake/delivery only after verifying a currently active seat and ledger authority. Avoid duplicating the ZCode session database or inventing an alternate agent runtime.
