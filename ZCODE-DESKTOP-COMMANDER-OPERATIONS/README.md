# ZCode × Desktop Commander — ChatGPT Operator (R001)
Status: R001 bootstrap, 2026-11-21. Workspace: `C:\0-BlackBoxProject-0\vivim-labs\ZCODE-DESKTOP-COMMANDER-OPERATIONS`.

## One-command SITREP
From any fresh ChatGPT conversation with Desktop Commander connected, say:
**"SITREP C:\0-BlackBoxProject-0\vivim-labs\ZCODE-DESKTOP-COMMANDER-OPERATIONS"**
Then read `AGENTS.md`, `PLAYBOOK.md`, `TEST-LEDGER.md`; execute `scripts\sitrep.ps1` on the connected desktop. Do not print secrets or raw provider configs.

## What is real today
- Desktop Commander connects to DESKTOP-9KB03LU; remote shell, file operations and process interaction work.
- ZCode CLI 0.18.0 supports headless prompt, session resume, protocol app-server, custom commands, skills, plugins and workflow modes. A single headless prompt returned ZCODE_BRIDGE_OK, exit 0.
- Inventory as observed: 562 available skills; 68 plugins; 19 custom commands; 93 user agent definitions. Local `.zcode\comms\bus.jsonl`, `delivered.jsonl`, `receipts.jsonl`, `projects.json` exist.
- Provider config: OpenCode Zen Chat 2 (16 model rules), NVIDIA NIM (73), MiniMax M Plan International (1 rule, 6 model-order entries), StartPlan (2 rules, account-backed). Availability and credit use unverified.
- The local 2.75GB ZCode SQLite database is NOT safe to scan casually. Read-only introspection only, under user approval if substantial.

## Commands
```powershell
pwsh -NoProfile -File .\scripts\sitrep.ps1
zcode --help
zcode doctor
zcode skills list
zcode plugins list
zcode commands list
zcode -p 'Reply with exactly PING_OK' --mode plan --disallowed-tools Bash Edit
```
## Operating doctrine
Rust-first when engineering; TypeScript 7+ only if sensible. Never assume CLI options work without `zcode --help`. No external writes, installations, spending, provider change, git commits or agent fanout without clear task authorization. No destructive Git operations. Existing ARBOR rules: Host is effect authority; local-first; PWA-first; work directly on main only when requested; preserve state. Status ledger distinguishes PASS, FAIL, UNRUN, UNKNOWN. No secrets in reports.

## R002 upgrade — Start here in fresh ChatGPT sessions
Run the full read-only status:
```powershell
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "C:\0-BlackBoxProject-0\vivim-labs\ZCODE-DESKTOP-COMMANDER-OPERATIONS\scripts\sitrep-r002.ps1" -Deep
```
Read: `CONTEXT7-SOURCE-ATLAS.md`, `NATIVE-PROTOCOL.md`, `WORKFLOWS-AGENTS.md`, `COMMS-AND-THREADS.md`, `MODELS-AND-PROVIDERS.md`, `CAPABILITY-MATRIX.md`, `RUST-BRIDGE-ARCHITECTURE.md`, `evidence/R002-OBSERVED.md`, `R003-TEST-MISSIONS.md`. All are source-backed, with live tests distinguished from designs. A candidate ZCode subagent definition is in `agents/zcode-commander-operator.md`; it has NOT been globally installed.

Native app-server read-only probes are now PASS: session/list (5), plugins/list (67), workflows/list (15), sampled session/subagents (0). V4 read-only sessions-index subscription also PASS: snapshot ACK + one frame. Local DB has 3348 sessions / 2719 child sessions / 1495 task links. HQ team JSON is accessible. See latest ledger; do not confuse historical child sessions with active agent workers.

For an all-in-one comprehensive read-only status on demand, use `powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts/sitrep-r002.ps1 -Deep -V4 -Team` with the full absolute script path or from its folder. The `-V4` sessions-index subscription and `-Team` HQ JSON overview also passed together: `SITREP_R002_COMPLETE`, exit 0. This starts and closes a separate ZCode app-server child; it does not write messages to the user-agent bus. Session counts are live, not immutable (later SITREP observed 3349).

## R003 verified Rust execution (2026-11-21)
The native read-only Rust observer is implemented at `native/zcode-observer/`. Repeat its live G1/G2 checks with:
```powershell
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "C:\0-BlackBoxProject-0\vivim-labs\ZCODE-DESKTOP-COMMANDER-OPERATIONS\scripts\rust-observe-r003.ps1" -WithHistoricalParent
```
Observed: `R003_RUST_OBSERVER_COMPLETE`, exit 0; 3/3 Rust tests, Clippy clean, read-only commands/query and V4 snapshot with epoch/sequence, historical parent native projection 18 child IDs / 18 ended / 0 running. Full facts: `evidence/R003-OBSERVED.md` and `TEST-LEDGER.md`. Concurrent ZCode headless workers hit upstream OpenRouter free-model 429; new Agent child fanout is still blocked/unverified. Do not treat local source definitions as active agents or silently switch billable providers.
