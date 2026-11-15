# ZCode Dynamic Workflows × OpenRouter — Autonomous Closed-Loop Development

Research and execution kit • 2026-11-15 • intended upstream home: `vivim-labs/agents/zcode-dynamic-closed-loop/`.

## Status

**Source-grounded research + runnable local utilities + ZCode authoring template.** The JavaScript utilities have local unit tests. The dynamic-workflow script requires a live ZCode installation with the bundled `dynamic-workflows` skill and registered OpenRouter provider and has **not** been compiled or executed in ZCode here. No local machine, secret, workflow run, or remote provider was accessed by this package.

This is an extension of `agents/zcode-workflow-specialist/`, **not** a competing engine. ZCode creates, journals, resumes, and amends workflows; this kit supplies an agent-driven development topology, policy, runtime catalog preflight, evidence contract and model bindings.

## Files

- `docs/SOURCE-AUDIT.md` — verified ZCode API facts, capability boundaries, links and research date.
- `docs/ARCHITECTURE.md` — state machine, authority, feedback loop, fleet, error recovery, milestones, acceptance.
- `docs/MODEL-ROUTING.md` — OpenRouter free-first routing policy, current seed roster and quotas.
- `workflows/closed-loop-development.workflow.ts.txt` — ZCode TypeScript workflow *body* template with four explicit model placeholders; not a standalone Node/TS program.
- `scripts/model-router.mjs` — pure catalog filtering/ranking; no direct model calls.
- `scripts/discover-models.mjs` — read-only OpenRouter `/api/v1/models` catalog refresh; no credential required.
- `scripts/render-workflow.mjs` — strict catalog + provider-id preflight then template renderer; never installs providers automatically.
- `tests/model-router.test.mjs` — offline pure tests and negative cases.
- `prompts/LAUNCH-IN-ZCODE.md` — complete parent-session command for ZCode, requires approved model and workflow confirmation.
- `prompts/NEXT-ROUND.md` — rollout and future dynamic experiment queue.

## Fast local usage (PowerShell 7, Node 24+)

```powershell
Set-Location 'C:\0-BlackBoxProject-0\vivim-labs\agents\zcode-dynamic-closed-loop'
node --test tests/model-router.test.mjs
node scripts/discover-models.mjs --out .local-models.json
node scripts/render-workflow.mjs --catalog .local-models.json --provider-id openrouter --out .local-closed-loop.dwf.ts
```

**Important:** `openrouter` is a suggested **ZCode custom provider identifier**, not a proven installed ID. Configure the provider in ZCode and check its exact canonical ID/model strings before submitting. `.local-*` files are operator-local and should not be committed. If ZCode resolves custom model IDs differently, change the provider flag and retry compilation. Do **not** substitute a paid model without an explicit opt-in.

Open `prompts/LAUNCH-IN-ZCODE.md` and instruct the **main ZCode session** to load the built-in `dynamic-workflows` skill and use `CreateWorkflow` with `path` pointing to the generated workflow body; use `AmendWorkflow` for revisions and `ResumeWorkflowRun` for stop/retry of the identical script. The `zcode-workflow-specialist` subagent is read/search guidance and **cannot** itself create nested workflows.

## Non-negotiable boundaries

- Free-first, default **zero paid inference** and explicit model-resolution preflight. Free does not mean unlimited; a call that fails on quota is not silently retried at another cost tier.
- Separate LLM proposals from independent tests and approval. No model's statement of "done" is acceptance.
- No unattended mutation of production/main, GitHub secrets, signed-in browser accounts, data deletion or external release.
- File scope, exact source SHA, actual test exit, and provenance in every keeper handoff.
- Max repair iterations, per-run work budget, bounded concurrency, cancellation, resumability and stop conditions.

See the `docs/` files for detailed operating policy.