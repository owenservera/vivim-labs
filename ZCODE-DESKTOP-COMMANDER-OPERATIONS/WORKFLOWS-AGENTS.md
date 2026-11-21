# Workflows, Subagents, Fans and Recovery — R002

## Local reality
`zcode skills list` -> 562 skills; `zcode plugins list` -> 68 plugin entries; `zcode commands list` -> 19 custom commands. `~/.zcode/agents` has 93 non-backup Markdown definitions. These are descriptors, NOT 93 running agents.
Built-in general-purpose and Explore roles exist in ZCode docs; user-level custom agents have YAML frontmatter fields like name, description, model, tools, skills, maxTurns, requiredMcpServers.
Full ZCode native protocol reports 67 plugins for the specific ARBOR workspace. Scope filtering explains why this can differ from global CLI count.

## ARBOR workflow corpus — verified files
`C:\0-BlackBoxProject-0\ARBOR\.zcode\workflows` holds **15 .dwf.ts definitions**, including:
`arbor-area-a`, `arbor-area-b`, `arbor-area-c`, `arbor-area-d`, `arbor-area-e`, `arbor-batch-engine`, `arbor-batch1`, `arbor-build`, `arbor-chatgpt-bridge`, `arbor-demo`, `arbor-integrate`, `arbor-integration`, `arbor-life`, `arbor-product`, `arbor-roadmap`.
The app-server returned 15 saved workflows. The separate SQLite `workflow_run` and `workflow_definition` tables each currently contain zero records. Do not interpret that as no workflow capability or no workflow files; no live workflow run is verified.

## Supported entrypoints vs proof
- CLI flags: `--workflow-mode disabled|onDemand|alwaysOn`, `--agent-always-background on|off`, `--target`, `--resume sess_...`, `--continue`. Help-verified only, except `--prompt` echo PASS.
- Session slash commands: `/dwf list|cancel|resume`, `/expert status|resume|stop`, `/fork`, `/resume`, `/model` (not yet interaction-tested).
- Agent tool: spawn specialized child, foreground parallel or background, parent receives notification. Local parent-child persistence observed.
- Existing agent manifests reference `Agent`, `TaskOutput`, `SendMessage`, `CreateWorkflow`, `ListSavedWorkflows`, `ListWorkflowRuns`, `GetWorkflowRun`, `ResumeWorkflowRun`, `AmendWorkflow`, `FillWorkflowHole`, `ResolveWorkflowQuestion`. Actual per-session tool grants must be checked.
- Workflow runtime child uses a separate internal NDJSON `create-actor` / `request` / `event` / `complete` carrier. Do not send these directly to app-server.

## Fanout procedure (to verify next round)
1. Input is a user-authorized goal, repo, permission mode, model/provider budget, max parallelism and time bounds.
2. Coordinator builds DAG: independent read-only research first; dependent write tasks serialized or isolated by file owner.
3. Start 2 read-only workers initially. One ZCode parent session per independent workstream, with child agents for nonoverlapping queries. Use project-specific permissions and NoBash/NoEdit for exploration.
4. Capture stable root sessionId, child sessionIds, parentId, task links, workflow run ID (if any), session state revision, model identity, start timestamp and request correlation ID.
5. Query `session/subagents` and `workflows/runs`/V4 queries. Distinguish accepted, running, blocked, terminal and stale.
6. Wait only on real terminal signals, inspect outputs and tool receipts, verify at least one side effect or independent test when claiming PASS.
7. Reconcile disagreements; allow at most 3 specialists in contested decisions. Use explicit authority ordering and preserve inputs.
8. Cancel by documented API, then verify worker ended. Never kill unrelated desktop processes.
9. Persist small receipt/journal in canonical operator surface, not model-generated optimistic prose.

## Existing valuable local roles
`orchestrator.md` has a DAG/delegate/reconcile contract and recommends max ~6 simultaneous delegates; `arbor-devops-coordinator.md` operates saved ARBOR workflows; `chatgpt-keeper.md` documents browser-mediated ChatGPT communication and receipts; `workflow-keeper.md` exists. These should be read on a relevant task, never copied into system grants wholesale.

## Stop conditions
No provable model availability, active seat conflict, revocation, unexpected write permission, stale state revision, permission escalation, uncertain send receipt, device offline, repeated invalid responses, or unbounded fanout. No auto provider swapping to a paid route.
