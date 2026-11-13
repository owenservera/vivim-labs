# Dynamic Workflow API — source-checked operational card

Sources: official `apps/zcode-cli/packages/bundled-skills/skills/dynamic-workflows/SKILL.md` §16, `docs/dynamic-workflow/launch.md` and contracts. Verified in source on 2026-11-13, not tested against local installed ZCode.

## Modern tools and their responsibilities
| Tool | Correct use | Critical distinction |
|---|---|---|
| `Skill` | load `dynamic-workflows` in current main session | authoring gate; reload after compaction |
| `CreateWorkflow` | compiler/analyzer + approval + new background run | exactly one of `script`, `path`, `saved` |
| `EvalWorkflowSnippet` | test fixed logic with no subagents or persistent run | lacks `agent`, `phase`, `report`, `artifact`, `hole` |
| `AmendWorkflow` | edit predecessor's script/settings; reuse matching work | do not call CreateWorkflow again just to revise |
| `FillWorkflowHole` | fill typed hole in an already-waiting run | needs exact run_id and hole_id |
| `ResumeWorkflowRun` | resume stopped run's identical effective script | amendment is for changed script |
| `SaveWorkflow` | save a user-approved reusable script/argument declaration | never unsolicited; project/global explicit |
| `ListSavedWorkflows` | discover reusable definitions | project names override global names |
| `ListWorkflowRuns`, `GetWorkflowRun` | inspect actual state and events | backgrounded is not completed |
| `ResolveWorkflowQuestion` | answer a subagent's escalation | child cannot ask user directly |

## CreateWorkflow request
- `name`: short label; always include for inline script.
- Exactly one of `script` (whole TypeScript body), `path` (file), or `saved: {name, args?, scope?}`.
- `args` at top level belongs to `path` only and must match the metadata block; `saved.args` belongs to `saved`.
- `max_concurrency`: optional user-selected limit for asks in flight; default is runtime-calculated; do not set merely to work around provider refusal.
- `subagent_model`: optional user-requested model; main model does not change. Provider crossing must be explicitly authorized.
- `CreateWorkflow` returns a background run ID only once accepted; compiler errors yield diagnostics and launch nothing.

## Facade quick signatures
```ts
interface Node<T> extends PromiseLike<T> {}
interface AgentPersona { system?: string; model?: string | ModelRef; budget?: { webSearch?: number } }
interface Agent { ask<T = string>(instructions: string): Node<T> }
function agent(name?: string, persona?: string | AgentPersona): Agent;
function model(id: string): ModelRef;
function phase(name: string): void;
function log(message: string): void;
function report(item: unknown, artifactId?: string): void;
const args: Readonly<Record<string, unknown>>;
const run: { readonly id: string; readonly resumedFrom?: string };
const files: {
  glob(pattern: string): Promise<string[]>;
  read(path: string): Promise<string>;
  grep(pattern: string, glob?: string): Promise<GrepMatch[]>;
};
const git: {
  changedFiles(base?: string): Promise<string[]>;
  diff(base?: string, path?: string): Promise<string>;
  status(): Promise<GitStatus>;
  log(count?: number): Promise<GitCommit[]>;
};
const world: { run(cmd: string, args?: string[], opts?: { timeoutMs?: number; output?: "full" | "discard" | { tail: number } }): Promise<WorldRunResult> };
```
The declarations above are **reference notation**, not code to paste into a workflow body; the full authoritative ambient declarations are in built-in `SKILL.md` §16.2. A script uses normal `interface`, `type`, `await`, loops, maps and `return`. No `declare`, `import`, `export`, Node/DOM objects or escaping facade calls.

## Artifact signatures (verified)
```ts
artifact.file(id, path, { title?, description?, primary?, contentType? })
artifact.markdown(id, content, { title?, description?, primary? })
artifact.chart(id, { title?, description?, primary?, type?, x, y, scale?, baseline? })
artifact.table(id, { title?, description?, primary?, columns, key? })
artifact.metrics(id, { title?, description?, primary?, metrics })
artifact.board(id, { title?, description?, primary?, key, status, columns, cardTitle?, detail? })
```
Content artifacts are async: `await artifact.markdown(...)`; presets are sync declarations fed with `report(item, "<artifact-id>")`. Reports persist as useful partial output across failure. Only one primary artifact ID per run.

## Runtime mechanics
- `agent()` creates a new context; `actor.ask<T>()` is a typed and schema-validated result.
- One actor queues concurrent asks FIFO; distinct actors can run in parallel.
- Named actor identities must be unique across the run; names enable cross-amendment cache matching.
- `phase("...")` is required and must be a standalone statement with a literal name; every phase contains at least one ask or world.run.
- Files and git reads are journaled; `world.run` command must be a literal, takes argv (not shell), returns nonzero exits as normal resolved results, and can still have real side effects. Explicitly check `exitCode` before describing success.
- `world.run` fixed command and argv are shown for approval; no shell pipes/redirection/expansion.
- `channel<T>()` supports async producer/consumer streams and must be closed to avoid deadlock; `future` and `hole` are advanced extensions.
- Reports have caps (65,536 per run, 1 MiB each, 1 GiB total). File artifact cap 20 MiB; markdown cap 256 KiB; maximum 32 IDs and 16 versions/ID.
- `files.glob` cap 2000 results, `files.grep` cap 2000 lines/256 KiB, `git.diff` cap 512 KiB. Oversize rejects rather than truncates.

## Modes
`disabled`: no authoring tools/skill/resume entrance, existing run records remain readable.
`onDemand`: /workflow and skill discoverable; tools register after session activation.
`alwaysOn`: tools are registered from first model turn.
Standalone CLI `--workflow-mode disabled|onDemand|alwaysOn`; default disabled. This flag is **not** for Host-controlled app-server/agent-server.

## Failure taxonomy
Authoring error 428 = missing loaded built-in skill. Missing source = malformed tool JSON or absent source; inline escaping errors often cause it. Compiler diagnostic = script not launched. Duplicate agent identity = compile/runtime failure. `ChannelDeadlock` = open stream that cannot make progress. `WorldReadCapExceeded` / `ReportCapExceeded` = fail loudly. Unresolved hole = wait for `FillWorkflowHole`. Provider transient handling is not evidence that logical tests passed. Exact error names can differ by installed release.
