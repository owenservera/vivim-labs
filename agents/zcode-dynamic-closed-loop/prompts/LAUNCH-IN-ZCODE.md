# Full main-session ZCode launch order — paste into the *main* ZCode agent

You are controlling ZCode's **native dynamic workflow system**, not a new shell orchestrator. Use the installed ZCode `Skill` tool to load **`dynamic-workflows`** in this **main session** before authoring. Do not delegate CreateWorkflow to a subagent. Read the authoritative built-in skill/API and compare with this package's `docs/SOURCE-AUDIT.md` and existing `vivim-labs/agents/zcode-workflow-specialist` package.

Workspace package: `C:\0-BlackBoxProject-0\vivim-labs\agents\zcode-dynamic-closed-loop` (update only if the local checkout differs). **This package is research/tooling only**. You are not authorized to interrupt the currently running ARBOR R004 sprint.

**ERA 1 — capability and model preflight**

1. Identify installed ZCode version, dynamic-workflow mode, tool availability and exact provider IDs; check that `CreateWorkflow`, `AmendWorkflow`, `ResumeWorkflowRun`, `ListWorkflowRuns`, `GetWorkflowRun` and authoring skill actually exist in this client. If not, return a blocker; do not invent tools.
2. Confirm an `openrouter` custom provider or report the exact installed provider ID; allow only OpenRouter strict-free `:free` models, zero request/response pricing, text and required tool support. Ask for missing provider connection/key through the normal ZCode settings; do not print API keys.
3. Run `node --test tests/model-router.test.mjs` and record actual exit. Run `node scripts/discover-models.mjs --out .local-models.json` for current public catalog; select roles through the package. Render `node scripts/render-workflow.mjs --catalog .local-models.json --provider-id <actual-zcode-provider-id> --out .local-closed-loop.dwf.ts`. The generated script is not yet compiled or accepted; preserve errors.
4. Read local generated script and installed ZCode workflow facade. Fix compatibility only within this package; rerun tests; do not silently change models to paid endpoints or `openrouter/free` roulette.

**ERA 2 — native dynamic workflow launch**

1. Use a **scratch repository or dedicated clean feature worktree**, not the active ARBOR branch/main. Supply explicit `args: { mission, allowedPaths, baseSha, testCommand }` in the `CreateWorkflow` call using its `path` source. The rendered `.dwf.ts` file includes the verified `/* zcode-workflow` YAML metadata block, which is required for path + args. Inline `script` cannot carry top-level args. Never manufacture a successful compile.
2. Prefer `max_concurrency: 3` or 4 if the user/owner authorizes that cap; the provider shared governor is separate. Request permission for every external provider/fee transition and repository mutation as mandated by the host.
3. Have the **main agent** call `CreateWorkflow` with the rendered, reviewed script/path and approved run settings. Observe the confirmation window if present. Record exact returned run ID (typically `dwfrun-...`). Never say the run started without actual confirmation/response.
4. Inspect progress with `ListWorkflowRuns` and `GetWorkflowRun`, and respond to `ResolveWorkflowQuestion` escalations. For source/script revisions use `AmendWorkflow` so named subagent work can be reused; for identical scripts on stopped runs use `ResumeWorkflowRun`. Do not start duplicate runs in parallel by accident.
5. Require independent executed tests and review; approve neither release nor a main merge within this pilot. Save workflow for reusable future use via `SaveWorkflow` **only because this user explicitly asked to save the workflow package**; follow ZCode's own save confirmation and choose project scope by default.

**ERA 3 — close loop and improve**

Return exact source SHA, generated script digest, four chosen model IDs, actual ZCode run ID, workflow state, per-phase accomplishments, real test exits, changed paths, verified versus unrun acceptance, quota pressure, model retries, cost if observable, failed cases, bounded repair results and next candidate tasks. Save evidence to scoped project-local artifacts after the host confirms paths. Compare success against a frozen scratch-project baseline; propose routing changes only after objective results. Maintain source/effect authority and stop if authorization, provider cost, or reality of execution becomes uncertain.