# ZCode Workflow Specialist — VIVIM Labs

Research snapshot: 2026-11-13. Source: official `zai-org/zcode` repository, resolved in Context7 as `/zai-org/zcode`. This is a **research and agent package**, not proof that any particular installed ZCode build exposes every feature.

## Purpose
Specialist agent for discovering, designing, authoring, validating, operating and debugging ZCode workflows. Prefer the modern **Dynamic Workflow** API (`CreateWorkflow`, `AmendWorkflow`, `SaveWorkflow`, `ListSavedWorkflows`, `ListWorkflowRuns`, `GetWorkflowRun`, `ResumeWorkflowRun`, `EvalWorkflowSnippet` and `ResolveWorkflowQuestion` when exposed), while detecting legacy `Workflow` and `/expert` systems without mixing their contracts.

## Contents
- `AGENT.md`: copyable agent system prompt and execution contract.
- `REFERENCE.md`: verified architecture and implementation-oriented documentation dump/index.
- `PLAYBOOK.md`: workflow topologies, examples, operational recipes, debugging and acceptance tests.
- `SOURCES.md`: source map and certainty ledger.

## How to use
1. In ZCode, load `AGENT.md` as your custom-agent instructions, or ask the active agent to read it and its references. **Do not assume** this folder alone auto-registers an agent in ZCode.
2. Confirm the installed ZCode version, available workflow tools and mode. Load the built-in `dynamic-workflows` skill before submitting scripts.
3. Ask for an explicit workflow, e.g. “Use a dynamic workflow to inventory the project, fan out analysis and independently verify findings.”
4. Use the actual tool contract exposed by the installed instance; syntax or availability may differ.
5. Store reusable scripts in the ZCode-recognized project `.zcode/workflows/<name>.dwf.ts` or global workflow directory after verification.

## VIVIM context
Labs owns research. The VIVIM Host and Rust core own product authority and runtime decisions. This agent is a ZCode workflow specialist, **not** a new VIVIM orchestration runtime. It must not introduce external provider APIs or bypass approvals when operating on VIVIM code. Favor single-repository execution where sufficient; use broader fan-out only when work is independent.

## Evidence level
Official docs label dynamic workflow “implemented,” but a separate desktop workflow-debugger document states its adapter is currently stubbed and workflow controls unsupported in that branch. Therefore distinguish **documented implementation** from **proven installed functionality**. No local ZCode runtime was exercised in this research.

## Round 2 — native installation and runtime contract (2026-11-13)
**This repository folder is not auto-discovered by ZCode.** To install the project subagent, copy [`install/project/.zcode/agents/zcode-workflow-specialist.md`](install/project/.zcode/agents/zcode-workflow-specialist.md) into the chosen ZCode workspace's root `.zcode/agents/` directory. See [`INSTALL-AND-VERIFY.md`](INSTALL-AND-VERIFY.md) for a Windows PowerShell command and safe smoke tests.

**The installed custom agent is a design/review subagent, not a nested workflow launcher.** The root/main ZCode session must load the bundled `dynamic-workflows` Skill and call `CreateWorkflow` or `AmendWorkflow`. See [`MAIN-SESSION.md`](MAIN-SESSION.md).

New reference material:
- [`API-REFERENCE.md`](API-REFERENCE.md): source-checked modern DWF facade and tool signatures.
- [`examples/read-only-audit.ts`](examples/read-only-audit.ts): complete illustrative TypeScript script, source-reviewed but runtime UNRUN.
- [`tests/ACCEPTANCE.md`](tests/ACCEPTANCE.md): machine-verifiable acceptance targets and evidence template.
- [`ROUND-2.md`](ROUND-2.md): discoveries, corrections, outstanding gaps.
- [`catalog/patterns.json`](catalog/patterns.json): machine-readable routing catalog for 16 documented patterns.
- [`catalog/examples.json`](catalog/examples.json): indexed seven official example scripts.

ZCode standalone CLI defaults its modern workflow tools to disabled. `--workflow-mode onDemand` and `--workflow-mode alwaysOn` are documented for TUI/headless; Host desktop has a separate Settings/config mode. The skill gate belongs to the **parent session**, not the specialist subagent.
