# Execution Ledger
Date: 2026-11-21. This is a factual status record, not a claim of production readiness.

| Check | Outcome | Evidence |
|---|---|---|
| Desktop Commander connected | PASS | DESKTOP-9KB03LU online; remote shell and file operations |
| ZCode CLI | PASS | `zcode --help` → 0.18.0 |
| ZCode doctor | PASS | Windows x64; Node v24.22.1; exit 0 |
| Single-prompt bridge | PASS | `zcode --prompt ... --mode plan --disallowed-tools Bash Edit` → `ZCODE_BRIDGE_OK`, exit 0, 46.12 s |
| Skills enumeration | PASS | `zcode skills list` → 562 available |
| Plugin enumeration | PASS | `zcode plugins list` → 68 |
| Commands enumeration | PASS | `zcode commands list` → 19 |
| Agent definition files | PASS | 93 active .md excluding .bak in `~/.zcode/agents` |
| Comm bus file discovery | PASS | `~/.zcode/comms` has bus, receipts, delivered, projects |
| Provider config metadata | PASS | 21 provider rules, 218 model rules, four desired providers mapped |
| Live provider auth/model availability | UNRUN | Config membership alone is insufficient |
| ZCode app-server handshake | UNRUN | CLI only |
| Multi-agent spawn | UNRUN | Need permitted test |
| Dynamic workflow | UNRUN | Need permitted test |
| Session resume | UNRUN | Need session-ID access |
| Cross-chat bus delivery | UNRUN | Need protocol contract |
| Desktop graphical automation | UNRUN | Connector has no native visual controls |
| R001 SITREP script | PASS | `powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts\\sitrep.ps1` → `SITREP_COMPLETE`, exit 0, 9.28 s |
| Current ZCode process | OBSERVED | SITREP found zero matching ZCode processes; this does not rule out desktop components |
| Bus freshness | OBSERVED | Last file writes 2026-10-07, so live delivery is not established |

## Round 2 priorities
1. Read ZCode guide plugin skill for app-server and workflow contracts.
2. Inspect safe SQLite schema read-only, not full DB; confirm session/project identifiers.
3. Identify bus format from supported documentation, not by writing guessed messages.
4. Validate session listing and model selection without switching providers or triggering spending.
5. Two read-only lanes with explicit budget and stop/cancel, only after checks.

## Round R002 — verified 2026-11-21

| Check | Outcome | Exact evidence |
|---|---|---|
| Context7 ZCode source resolution | PASS | `/zai-org/zcode`; V4 transport, commands, workflow runtime; local checkout HEAD 29628c9 |
| Context7 ACP comparison | PASS | `/websites/agentclientprotocol` (separate from native ZCode) |
| Native stdio app-server: session/list | PASS | `node scripts/probe-protocol.ts` → 5 sessions |
| Native stdio app-server: plugins/list | PASS | same → 67 workspace plugins |
| Native stdio app-server: workflows/list | PASS | same → 15 saved workflows |
| Native stdio app-server: session/subagents | PASS | same → sampled parent 0 children |
| Protocol request success/close | PASS | 4 responses, 0 errors, exit 0, ~11.69s |
| SQLite session graph read-only | PASS | 3348 total; 2719 child; 1495 task links; 0 workflow_run/definition |
| ARBOR saved workflow files | PASS | 15 `.dwf.ts` under `ARBOR/.zcode/workflows` |
| HQ team read-only JSON | PASS | Bun direct `hq.mjs team --json` exit 0; 93 agents, 4 seats, 1 task |
| R002 deep SITREP | PASS | `powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts/sitrep-r002.ps1 -Deep` → `SITREP_R002_COMPLETE`, exit 0, 18.39s |
| Bun PATH shim | FAIL | `bun run` remap error; direct `~/.bun/bin/bun.exe` works; Node24 TypeScript probe works |
| Live V4 streaming subscription | PASS | `node scripts/probe-v4-subscribe.ts`: `V4_ACK=PASS mode=snapshot`, `V4_FRAME_RECEIVED=YES`, frames=1, errors=0, exit 0 (10.84s); no state mutations |
| Actual concurrent subagent spawn | UNRUN | Source and historical graph only |
| Dynamic workflow run/cancel/resume | UNRUN | 15 definitions, 0 journal runs |
| Cross-ChatGPT thread message delivery | UNRUN | Needs explicit authorized bus/wake/receipt test |

All R002 data and source paths: `evidence/R002-OBSERVED.md`. Next test gates: `R003-TEST-MISSIONS.md`.

| Combined V4 + HQ one-command SITREP | PASS | `powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts/sitrep-r002.ps1 -V4 -Team` → V4 ack snapshot + frame, HQ 93 agents / 4 seats / 1 task, `SITREP_R002_COMPLETE`, exit 0 (17.54s) |
| Session count drift | OBSERVED | R002 earlier 3348 sessions; later SITREP 3349. Could be concurrent/user runtime activity; do not infer that probe caused it. |

## R003 — actual Desktop Commander execution (2026-11-21 UTC)

| Gate | Outcome | Actual evidence |
|---|---|---|
| Deep + V4 + Team SITREP | PASS | PID 10844, exact `sitrep-r002.ps1 -Deep -V4 -Team`, `SITREP_R002_COMPLETE`, exit 0, 27.33s |
| Rust native read-only observer | PASS | `native/zcode-observer` — native `v4/commands/query` unknown nonce, sessions, plugins, 15 workflows, one V4 complete snapshot with owner matching/cursor metadata; 0 errors, exit 0 |
| Rust quality gates | PASS | `cargo fmt --all` exit 0; `cargo clippy --offline --all-targets -- -D warnings` exit 0; `cargo test --offline` 3/3 passed |
| Historical parent/child native projection | PASS | `session/subagents` on grounded SQLite parent; children=18, running=0, ended_total=18, ended_page=18; wrapper PID 4080, exit 0 |
| Concurrent headless ZCode research workers | PARTIAL | Two local processes launched (16128,13664), both encountered free OpenRouter model upstream 429; first ended without Agent child, second terminated; no paid reroute |
| Genuine two-child Agent tool fanout | BLOCKED | Current read-only session did not expose Agent grant; free model 429; no distinct newly spawned child IDs |
| V4 delta/resync, HQ wake, workflow run/cancel, model comparisons | UNRUN | Outside successful read-only gates; no synthetic success claim |

Further source paths, command constraints and transcripts of sanitized status: `evidence/R003-OBSERVED.md`. ARBOR local edits and worktrees preserved.
