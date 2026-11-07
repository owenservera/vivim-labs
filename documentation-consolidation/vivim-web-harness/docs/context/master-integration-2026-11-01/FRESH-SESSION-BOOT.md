# Fresh agent session — "hi" to useful next action

**Acceptance target:** A new user-created OpenCode/Claude/Codex session in C:\0-BlackBoxProject-0\vivim-web-harness receives first prompt "hi" and gives current project-specific choices or safely resumes a previously identified task. Never start an AI provider send or other external effect from a greeting alone.

## 1. Observed problem

Attached local-session reconnaissance: global OpenCode AGENTS.md + repo AGENTS.md + context.md are orientation text (~13.5 KB combined); after following docs/index and roadmap, a fresh agent can consume >120 KB without an actionable first command. It found no repo-root .opencode config or first-action greeting recipe and reports that docs/session/current.md is absent. **These observations are from the attachment** and need local re-check; current GitHub source separately confirms AGENTS.md, context.md and stale START-HERE.

## 2. Minimal entrypoint design

Root AGENTS.md should contain a compact first-turn policy, not a second context library:

> For a greeting or ambiguous first request, run read-only bounded session brief (once), check actual Git HEAD/branch/dirty and authoritative roadmap, then present 2–3 evidence-linked next actions. Use only existing owner-approved scope; never infer send/release permissions from "hi". If a specific task is supplied, build that task's targeted DCB and act only within its authority.

**Do not** include full context material or hard-code "today's top task" into AGENTS.md. Prefer a one-line command pointing to a verified local helper (e.g. node tools/context/lake.mjs brief --max-bytes 6000 --json). The command **must exist and pass tests before adding the pointer**.

## 3. Session brief output candidate

```json
{
  "schema": "vivim.session-brief/1",
  "project": "vivim-web-harness",
  "pathValidated": true,
  "git": {"head": "observed-sha", "branch": "main", "dirtyCount": 0},
  "activePhaseRef": "docs/roadmap.json#activePhase",
  "checks": [{"id":"source_index","status":"fresh|stale|unknown","evidenceRef":"..."}],
  "options": [
    {"id":"continue_current","title":"Continue highest evidence-backed active task","roadmapId":"...", "sourceRefs":["..."],"requiresApproval":false},
    {"id":"fix_drift","title":"Reconcile context/index drift","sourceRefs":["..."],"requiresApproval":false},
    {"id":"check_release","title":"Inspect local readiness without provider sends","sourceRefs":["..."],"requiresApproval":false}
  ],
  "uncertainties": [],
  "metadataOnly": true
}
```

Titles and IDs above are **examples**, not actual current task status. Emit only grounded options with exact source status; if unknown, show inspect-before-act.

## 4. OpenCode-specific technology

Current OpenCode official documentation describes automatically loaded root AGENTS.md and configurable additional instructions. Plugin events include session.created, session.idle, and other lifecycle events. An event hook is optional: it can cache/read a brief, but does not imply any mechanism to inject new user messages or authorize actions. Test semantics for the installed OpenCode version before using plugins. Avoid writing a plugin that blindly sends a prompt whenever session.created fires. Repository AGENTS instructions remain the portable baseline.

Official references:
- https://dev.opencode.ai/v2/docs/instructions/
- https://docs.opencode.ai/docs/plugins/

The local session mentioned an ultracave skill and session-objectives skill with contradictory/manual triggers. Treat global skill behavior as user-specific; verify installed package and do not silently modify global rules.

## 5. Read-only probe sequence for local agent

PowerShell:

```powershell
Set-Location 'C:\0-BlackBoxProject-0\vivim-web-harness'
git status --short
git rev-parse HEAD
git branch --show-current
node --version
node tools/docs/query.mjs --search "context" --json
```

Then inspect existing tools/context or its proposed local draft, docs/roadmap.json activePhase, root AGENTS.md/context.md, local global OpenCode instructions/config, relevant skill directories and docs/session/current.md. Do not assume absence from the GitHub snapshot means absence locally. Use an executable tested script for the future bounded brief.

## 6. Live acceptance

- A fresh OpenCode session with only "hi" invokes a local read-only brief once and responds within one interaction with actual branch/HEAD, authority constraints, a meaningful next task/choices, source pointers and unknowns.
- Fresh "hi" with clean repo, dirty worktree, stale indexes, context-feed absent, and no active roadmap task each have defined safe fallback.
- No duplicate workers, network/provider sends, browser tabs modified, worktree mutations, false test receipts or private data in briefing output.
- Specific task request bypasses generic greeting chooser and loads only its scoped DCB.
- Check Claude/Codex equivalent on the installed versions; use AGENTS portably where supported and tool-specific config only if tested.
