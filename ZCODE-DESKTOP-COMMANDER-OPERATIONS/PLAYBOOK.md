# ZCode Operations Playbook — R001

## 1. Access surfaces
- Remote Desktop Commander: remote filesystem, process list, shell, interactive terminal input/output. No native visual screenshot/click/keyboard tools in this connector; platform UI automation requires an adapter.
- ZCode CLI 0.18.0: `--prompt`, `--cwd`, `--mode build|edit|plan|yolo|guarded`, `--resume sess_...`, `--continue`, `--target`, `--workflow-mode disabled|onDemand|alwaysOn`, `--agent-always-background on|off`, `--surface terminal|desktop`, `app-server`, `plugins`, `skills`, `commands`, `doctor`.
- Built-in slash commands include `/expert`, `/dwf`, `/mcp`, `/resume`, `/fork`, `/mode`, `/model`, `/goal` and `/team` via custom plugin. Slash command support in headless invocations must be tested.

## 2. Discovery commands (safe)
```powershell
zcode --help
zcode doctor
zcode skills list
zcode plugins list
zcode commands list
Get-ChildItem "$HOME\.zcode\agents" -File -Filter '*.md' | Where-Object Name -NotMatch '\.bak'
Get-ChildItem "$HOME\.zcode\comms" -File | Select-Object Name,Length,LastWriteTime
```
Avoid outputting raw `provider_config.json`: it can contain sensitive fields. Parse and print only provider ID/name/enabled and model IDs.

## 3. Models
Known local registrations:
- `opencode-zen-chat-2`: 16 model rules; enabled.
- `new-provider` (NVIDIA NIM): 73; enabled.
- `minimax-m-plan`: 1 model rule, six model-order entries; enabled.
- `account:zai-start-plan`: two model rules and three display/order variants; account-backed, not same enabled schema.
Model listing ≠ authenticated live availability. Current default seen: `openrouter` / `nvidia/nemotron-3.5-super-vl:free`. No changes made. First verify protocol model selection and quota/error surfaces. Obtain approval before any billable provider test. Do not fabricate `--model` option: CLI --help does not advertise it; `/model` is a session slash command.

## 4. Skills, plugins, commands
At R001 inventory: skills 562; plugins 68; custom commands 19. Plugin list includes official `computer-use`, `browser-use`, `zcode-guide`, `github`, plus custom `vivim-team` providing `/team`. Plugin existence does not prove activation in an agent runtime. Use `zcode plugins list`, `zcode skills list`, `zcode commands list`. For newly needed capability read its SKILL.md and plugin metadata. Do not bulk enable/install without approval.

## 5. Workflows, subagents, concurrent lanes
Workflow entrypoint: `zcode --prompt '<task>' --cwd '<repo>' --mode plan --workflow-mode onDemand` (execution UNTESTED). Dynamic workflow slash command: `/dwf list|cancel|resume` (in-session; UNTESTED). Expert workflow: `/expert status|resume|stop|<task>` (UNTESTED). Background subagents: `--agent-always-background on`, but actual spawn requires a compatible task and runtime (UNTESTED).
Proposed run pattern: coordinator generates a work item ID; one isolated ZCode session per lane, nonoverlapping owned files, approvals/restrictions fixed; track PID/session ID; gather responses/receipts; verification lane checks results; coordinator reconciles conflicts. Limit initial fanout to 2 read-only lanes. Never imply ZCode's internal agents, separate terminal sessions and ChatGPT threads are the same thing.

## 6. Bus / cross-thread messaging
Observed `%USERPROFILE%\.zcode\comms\bus.jsonl`, `delivered.jsonl`, `receipts.jsonl`, `projects.json`. These are local records, not proof of reliable cross-thread transport. First inspect sanitized schema and `/team` plugin docs. Define a versioned envelope with `message_id`, `sender`, `recipient`, `correlation_id`, `project`, `created_at`, `kind`, `payload_ref`, `status`; this is a PROPOSAL only. Do not write into existing bus until protocol/readers/permissions are verified. ChatGPT conversation-to-conversation signaling requires an external shared bridge with explicit ownership and polling; not a native shared memory claim.

## 7. App-server bridge
`zcode app-server` is a stdio protocol endpoint. Target R002: inspect supported initialize, authentication, session creation/list, model enumeration/selection, tool permissions, notifications and cancellation from source or docs. Implement narrow Rust-side client later, with bounded messages, correlation IDs and receipts. Do not send malformed protocol guesses to live state.

## 8. Test stages
R001: CLI existence, doctor, inventory, directory / bus presence, safe chat echo.
R002: app-server handshake and read-only sessions/model lists; discover provider rates without spend.
R003: two read-only agent sessions in parallel; verify independent traces and cancellation.
R004: documented session resume and inter-session bus delivery; replay/dedupe.
R005: authorized file edits against disposable fixture, permission gates, rollback.
R006: ARBOR Host-controlled orchestration integration, Rust transport, audits, device UI.

## R002: verified native control and research addendum
For current implementation use `NATIVE-PROTOCOL.md` and `scripts/probe-protocol.ts`, not earlier speculative descriptions. The native app-server returned `session/list`, `plugins/list`, `workflows/list`, `session/subagents` successfully after correcting the required workspace object. Read-only `session/subagents` works, but the sampled parent had no children. See `WORKFLOWS-AGENTS.md` for persisted child/session graph and `COMMS-AND-THREADS.md` for the HQ seat vs ZCode child distinction. `scripts/sitrep-r002.ps1 -Deep` produces the complete repeatable local discovery result.
