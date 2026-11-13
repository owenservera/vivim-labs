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
