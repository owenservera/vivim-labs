# Research audit — ZCode dynamic workflows and model providers

Checked on 2026-11-15. All technical claims below are from upstream ZCode docs/Context7, existing `vivim-labs/agents/zcode-workflow-specialist`, or OpenRouter public docs. Source implementation **does not prove** identical support in the user's installed ZCode build.

## Confirmed upstream ZCode contracts

1. `docs/dynamic-workflow/authoring.md` describes an **implemented TypeScript script body** environment, not YAML DAG definitions. Its facade has `agent`, `model`, `args`, `run`, `phase`, `log`, `report`, `artifact`, `files`, `git`, `world.run`, `channel`, `future`, `hole`. It typechecks `strict` ES2022. **Do not use** Node globals, `fetch`, `process`, `import`, `export`, or `declare` in a workflow body. `world.run` bridges bounded approved subprocesses.
2. `agent(name, {system, model})` creates a persistent subagent. `.ask<T>` produces an asynchronous thenable, and `Promise.all()` joins independent asks. An agent is serial within its own context. Distinct agent names must remain unique. Naming enables reuse across amendments.
3. The model choices must form a statically enumerable **closed set**: literal strings or `model("literal")`; runtime arbitrary strings are rejected. This package's render step fixes all model names before compilation.
4. `phase("literal")` demarcates work; each phase must contain an ask or `world.run`. `report` and `artifact` persist incremental evidence without marking acceptance.
5. In current tool flow, the **main ZCode session must first call `Skill` for `dynamic-workflows`**. The authoring tools `CreateWorkflow`, `AmendWorkflow`, `SaveWorkflow`, `EvalWorkflowSnippet` reject script authoring when the skill is not loaded (error 428). Some non-authoring variants are exempt. A skill loaded in a subagent does not unlock parent authoring. Reload after session compaction if needed.
6. `CreateWorkflow` accepts one of `{script, path, saved}`; `path` supports `args` only if the first nonblank line is a `/* zcode-workflow` YAML metadata block in a `.dwf.ts` file (see launch.md Saved workflows / path source); it is confirmation-gated in supported clients, with explicit headless/TUI approval exceptions. User consent is not inferred merely from a task being described.
7. `AmendWorkflow` can preserve predecessor script when only settings change, can reuse cached subagent work through named actor identity; `ResumeWorkflowRun` restores stopped run with unchanged script. Inspect `ListWorkflowRuns`/`GetWorkflowRun` to know actual state; a launched/background run is not a completed run.
8. ZCode enforces both per-run `max_concurrency` and a shared provider-key governor. The user-set bound isn't automatically rewritten due to rate limits; the governor manages provider pressure. Do not spin up 40 workers on a 50/day free quota.
9. Runtime package `@zcode/dynamic-workflow-runtime` exposes `runWorkflowScript({scriptText,caps,askSpecs,validate,makeDriver,signal,timeoutMs})`, returning completed/failed/cancelled. **This is internal host integration**, not a standalone supported CLI call to launch scripts outside ZCode.
10. Provider schema `zcode.model-providers.v1` accepts OpenAI-compatible endpoints and model catalogs. Custom provider IDs and exact full model strings must be verified within installed ZCode; this kit does not assume the provider has already been configured.

## Source links

- https://github.com/zai-org/ZCode/blob/main/docs/dynamic-workflow/authoring.md
- https://github.com/zai-org/ZCode/blob/main/docs/dynamic-workflow/launch.md
- https://github.com/zai-org/ZCode/blob/main/docs/dynamic-workflow/concurrency.md
- https://github.com/zai-org/ZCode/blob/main/docs/dynamic-workflow/presentation.md
- https://github.com/zai-org/ZCode/blob/main/docs/dynamic-workflow/transcript-and-notifications.md
- https://github.com/zai-org/ZCode/blob/main/apps/zcode-cli/packages/dynamic-workflow-runtime/README.md
- https://github.com/zai-org/ZCode/blob/main/docs/model-provider-json-schema.md
- https://github.com/owenservera/vivim-labs/tree/main/agents/zcode-workflow-specialist
- https://openrouter.ai/collections/free-models
- https://openrouter.ai/models?variant=free
- https://openrouter.ai/docs/api-reference/list-available-models

## Not yet verified here

- Installed ZCode build/version and exact enabled workflow mode (`disabled`/`onDemand`/`alwaysOn`).
- Whether OpenRouter is registered in the local ZCode provider catalog and what its actual provider ID is.
- Actual compile acceptance of the supplied script against the local bundled skill facade.
- End-to-end workflow launch, model call, tool calling, worktree creation, browser UI and Windows Host evidence.
- Free-model endpoint availability/quality/latency for the user's account.

## Revalidation gate

Before launch, the main session must read the installed `dynamic-workflows` skill and API shapes, check ZCode version, compare this script with the installed facade, and use `EvalWorkflowSnippet` only for facade-compatible pure checks. Do not declare the script compiled or launched without a returned `dwfrun-...` and actual run events.