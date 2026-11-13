# Install and validate the ZCode Workflow Specialist

This folder in VIVIM Labs is a **distribution/research package**. ZCode does **not** discover custom agents recursively from `agents/zcode-workflow-specialist/`. You must copy the native Markdown profile into the target **ZCode workspace** agent root.

## Installation — Windows PowerShell
From a checkout of `vivim-labs`, copy:
`agents/zcode-workflow-specialist/install/project/.zcode/agents/zcode-workflow-specialist.md`

Into the **target project's** root:
`<TARGET_PROJECT>/.zcode/agents/zcode-workflow-specialist.md`

Example (replace both paths; inspect destination before overwriting):
```powershell
$source = "C:\path\to\vivim-labs\agents\zcode-workflow-specialist\install\project\.zcode\agents\zcode-workflow-specialist.md"
$target = "C:\path\to\my-project\.zcode\agents"
New-Item -ItemType Directory -Force $target | Out-Null
Copy-Item -Path $source -Destination (Join-Path $target "zcode-workflow-specialist.md")
```

Or place the same file in the user data agents root (`<storage.dir>/agents/`, normally `~/.zcode/agents/`) to expose it across projects. Preserve user config and existing profile if already present; follow installed version's precedence. The file should be directly under `agents`, not nested inside another folder.

## Distinguish the two instruction contexts
- **Custom subagent:** project `.zcode/agents/zcode-workflow-specialist.md`. It can analyze and propose. The bundled default restricts it to `Read`, `Grep`, `Glob`. It does not call CreateWorkflow.
- **Main/parent agent:** guidance in `MAIN-SESSION.md`. The parent loads `dynamic-workflows` and invokes workflow execution tools. The subagent skill load does not open the parent's skill gate. Do not overwrite root AGENTS.md; merge a short reference or paste the parent guidance into a live main session.

## Actual product enablement
- In standalone ZCode CLI/TUI/headless, inspect `zcode --help`; the documented flag is `--workflow-mode onDemand` or `--workflow-mode alwaysOn`, and absent flag defaults to `disabled`. Do not use old removed `--enable-workflow`.
- In Host/desktop, the effective `dynamicWorkflow.mode` can come from server offer, local override or user's Settings choice. `--workflow-mode` does not control app-server/agent-server.
- In `onDemand`, initial CreateWorkflow may be absent from tool list; start with the documented `/workflow <instructions>` activation.
- The custom agent's Markdown `model` / `thoughtLevel` can be configured explicitly when the model is known, but this supplied profile leaves them absent and inherits the parent defaults. Do not hardcode a provider that may not exist.
- Do not add `permissionMode` expecting it to enforce a custom profile's behavior: current parser tests show that field may be ignored for project profile. Actual mutation approval must be checked against the installed runtime.

## Low-risk interactive smoke check
1. Open a disposable scratch project as the ZCode workspace; avoid live VIVIM source for first tests.
2. Inspect available subagents: verify the profile is listed with name `zcode-workflow-specialist`. If absent, confirm path and restart/reload the session as required by that version.
3. Ask main agent to invoke specialist **for design only**, and obtain a parent-ready TypeScript body.
4. Explicitly request “use a dynamic workflow” in parent session and load `Skill` for `dynamic-workflows`.
5. Check mode, create a small workflow that uses only `files.glob` and a couple of read-only analysis asks; inspect compilation and confirmation.
6. Observe actual run ID and notifications, verify `GetWorkflowRun` events and published report/artifact.
7. Test cancellation + resume in a scratch workspace. Create a changed-script run via `AmendWorkflow` and compare reused work, if actually supported.
8. Mark every line of the matrix in `tests/ACCEPTANCE.md` PASS/FAIL/UNRUN with evidence.

## Safety limits
- An ask saying “read-only” is **not** a hard tool-level policy. Isolate scratch workspace; require appropriate permissions and inspect any tool requested by the subagent before approving.
- `EvalWorkflowSnippet` can run `world.run`; commands can write even though other `files`/`git` facade members are read-only. Do not test writes against the live repository.
- Headless/TUI dynamic workflow confirmation may be bypassed by design. Never assume a GUI confirmation protects these modes.
- Do not submit credentials or private material to provider logs; do not save incidental workflows without user permission.

## Expected result
Verified native agent discovery + verified parent workflow tool availability + typechecked read-only script + actual background run status + final artifact + exact tests/events. This remains **UNRUN** until the local installed ZCode environment is exercised.
