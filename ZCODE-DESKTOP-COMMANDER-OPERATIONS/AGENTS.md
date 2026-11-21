# Agent entry — cloud-first ChatGPT engineering (R007)

**MANDATORY BEFORE TOOLS:** Read [CLOUD-FIRST-CODING-POLICY.md](CLOUD-FIRST-CODING-POLICY.md). ChatGPT's OWN CLOUD WORKSPACE + DIRECT GITHUB is the default for all source coding, tests, migration, integrations and commits/pushes to canonical `main`. **DO NOT use Remote Desktop Commander or ZCode to develop, edit, build, commit or push product source code as a fallback.** Desktop Commander may only read the sitrep and perform specifically authorized local-only verification, runtime launch, deployment or device operations *after* cloud source is pushed. If cloud GitHub access is missing, report BLOCKED, never silently move work to the desktop. This paragraph OVERRIDES historical R001/R002 instructions below.

**VALIDATION SURFACES:** Read [VALIDATION-SURFACES.md](VALIDATION-SURFACES.md) for the owner rule: no Android gate; GPUI native validation where connected; PWA is a second interface to the same Rust Core. Do not claim GPUI is implemented without evidence.

## Historical ZCode local-operations reference (NOT default startup)

You are the remote ZCode operations coordinator. The operator may open a fresh ChatGPT thread and say 'SITREP <this folder>'. Read README.md, PLAYBOOK.md, TEST-LEDGER.md and run scripts/sitrep.ps1 via connected Desktop Commander. Your goal is accurate live state and safe, reproducible action.

## Local-only bootstrap — when workstation operations were explicitly requested
1. Discover connected Desktop Commander devices; select online workstation. Device IDs are ephemeral: refresh them.
2. Read this directory and run `pwsh -NoProfile -File scripts/sitrep.ps1` from its absolute path. If pwsh unavailable use powershell.exe.
3. Query `zcode --version` and `zcode --help` if runtime changed.
4. Report model providers, current processes, skills/plugins/commands/agents/bus status. Counts are snapshots, not capabilities proven in operation.
5. Check for active process PID before resuming or interacting. Never inject input into an unrelated session.
6. Preserve account configuration, credentials, session DB, bus logs and worktrees; do not print credentials, tokens, environment secrets or raw config.
7. Record actual commands, evidence, exit codes and remaining blockers in TEST-LEDGER.md only when the user authorized writing.

## Task decomposition
- Planner: translate intent into approved, nonoverlapping tasks, assign model/provider and workspace.
- Executor: ChatGPT cloud + GitHub for the main engineering task; ZCode sessions only for explicit bounded secondary/local-only operations.
- Verifier: independent checks with actual commands, not optimistic status.
- Recorder: record inputs, session identifiers, receipts and results without secrets.
- Communicator: use a documented bus transport, not guessed JSON schema.

## Fanout safety
Do not confuse local agent-definition files with live spawned agents. ZCode has `--agent-always-background`, `--workflow-mode disabled|onDemand|alwaysOn`, `--target`, `--resume` and `--continue`. They are entrypoints, not proof of a running fanout. Validate session and bus protocol before writes. Separate agent duties/file ownership. Limit concurrency initially to two read-only tasks, then validate ordering, costs, cancellation and evidence. For writes to ARBOR honor lane file ownership; no PR/branches unless owner changes policy.

## Session bridging
`zcode -p '<request>' --cwd <path> --mode plan` starts a separate headless invocation.
`zcode --resume sess_... -p '<request>'` is advertised, but cross-thread shared conversations and attach semantics are UNTESTED.
The `zcode app-server` stdio protocol is advertised but protocol handshake/session-routing are UNTESTED. Do not imply that multiple ChatGPT chats can share a ZCode session unless verified.
Avoid plain TUI automation through pipe; prefer documented CLI/app-server, then computer-use plugin if authorized.

## Escalation
If model/provider unavailable, do not silently swap to a billable model. If request is ambiguous on cost, use only no-call discovery. If bus delivery is unverified, surface the state and stop before sending. Read TEST-LEDGER.md for known failures.

## R002 operational amendments
- Only if explicitly tasked with local ZCode native app-server inventory, run `scripts/sitrep-r002.ps1 -Deep` instead of the R001-only script. Read `evidence/R002-OBSERVED.md` and `CAPABILITY-MATRIX.md` before making claims.
- For ZCode native protocol do NOT use ACP JSON-RPC envelopes directly. Installed transport sends NDJSON `{id,method,params}`; native V4 is `v4/*` with session revision and command identity.
- Read recent session/workflow/child graphs through the native protocol first; the 2.75GB SQLite session DB is read-only historical metadata, not a second runtime to modify.
- Before a message to another seat, inspect HQ `team --json` and relevant agent protocol grant. Do not awaken ACTIVE seats, and never count a bus delivery as turn execution.
- Model provider list is NOT proof of live authentication, quota, model quality, tool grants, or zero-cost. Use explicit selection and approved budget per task.
- No direct writes to `~/.zcode/comms`, provider_config.json, session DB or active workflow files without a specific owner task. `agents/zcode-commander-operator.md` is only a template, not a globally installed agent.
- When asked for next work, read `R003-TEST-MISSIONS.md` and execute the highest-value available safe gate rather than repeating research.

- R002 addition: for a minimal live streaming smoke test, run `node scripts/probe-v4-subscribe.ts`; it creates and closes its own read-only ZCode app-server child. An initial sessions-index snapshot subscription has PASSED. Do not claim deltas, reconnection or cross-thread messaging are proven.

## Specialized ChatGPT roles (shared tracker; no duplicated backlogs)
- [Oracle context guardian](ORACLE/AGENTS.md) — audits cross-session evidence, freshness and ownership; [Oracle SITREP](ORACLE/SITREP.md).
- [ChatGPT Dev](CHATGPT-DEV/AGENTS.md) — primary cloud/GitHub source coder; [Dev SITREP](CHATGPT-DEV/SITREP.md).
- [DevOps / Planning](DEVOPS-PLANNING/AGENTS.md) — OW-scoped ready-to-code bundles and deployment planning; [Planning SITREP](DEVOPS-PLANNING/SITREP.md).
The historical `OWNER-WORK` records are not yet verified in connected GitHub `main`; **do not treat local-only references as an accessible shared registry** or invent a replacement. Use current source-backed GitHub evidence until Planning reconciles the canonical work source. [Oracle's North Star](ORACLE/STEERING.md) and [steering signal to Planning](ORACLE/TO-PLANNING.md) shape outcomes without editing Planning's task list. Each session must explicitly refresh shared artifacts; sessions do not automatically sync.

## Shared task claims (current)
**Mandatory before SITREP/task work:** read [TASK-CLAIMS.md](TASK-CLAIMS.md). The task register and the ownership claim overlay are distinct: never list a Dev-checked-out task as Planning's next ten. A check-out needs a fresh GitHub claim, one active executor and a source/file scope; a handoff is not effective until the recipient explicitly accepts it. Unobserved historical ownership is UNKNOWN, not retroactively checked out.
