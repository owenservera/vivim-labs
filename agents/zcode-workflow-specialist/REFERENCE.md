# ZCode workflow system — consolidated technical reference

As researched 2026-11-13 via Context7 `/zai-org/zcode` and official repository. This is a **substantive technical digest and link-index**, not a verbatim copy of upstream documentation. Upstream remains the authoritative full documentation dump.

## 1. Three systems under the name “workflow”
| System | Entry points | Script form | Durable state | Caution |
|---|---|---|---|---|
| Dynamic workflow | `CreateWorkflow`, `AmendWorkflow`, `SaveWorkflow`, run tools | typed TS body, saved `.dwf.ts` | journal: run, actor, node, event; resume/replay and amendment cache | Requires explicit workflow request and skill gate |
| Legacy script workflow | `Workflow` | `.workflow.js`, older metadata/DSL | `wf_...` runs, deterministic prefix | Old docs explicitly describe removed `/workflow run` and `/workflows` |
| Expert phased DAG | `/expert`, workflow runtime/app-server | `WorkflowDefinition` JSON, phase/node artifacts | snapshots, graph/events/artifacts | Different lifecycle and potential default yolo behavior |

## 2. Dynamic authoring facade
Compiler uses a single `FACADE_DTS`; TypeScript strict, ES2022 only, no Node/DOM imports, exports or declares; script body can use `await` and `return`.
- `agent(name?, persona?)` gives a persistent subagent, optional system instructions, model and search budget. `.ask<T>(instructions)` returns `Node<T>`, a thenable. Same actor executes sequential FIFO; independent actors/tasks can run concurrently.
- `model(id)` resolves per-actor model where supported. No implicit provider switch.
- `phase(name)` marks stages. `report` emits structured output; `artifact` publishes file/markdown/structured deliverables; `log` narrates.
- `files.*`, `git.*` and `world.run` are journaled world reads; `EvalWorkflowSnippet` can dry-run supported snippets with no subagents.
- `args` carries inputs, `run` runtime identity; newer facade also includes `channel`, `future` and `hole`.
- Direct facade calls required to retain stable site identity. An analyzer derives site, causality, control-flow, and handoff graphs. Site = static call; instance = site plus ordinal execution.
- Typed `ask<T>` results produce JSON schemas and are validated. Name each actor uniquely; reusing named actors across iterations is an error. Keeping a handle outside a retry loop preserves memory.

## 3. Launch/control
`CreateWorkflow` takes **exactly one** of inline `script`, `saved` reference, or on-disk `path`; `args` accompanies a path, while saved values live in `saved.args`. Other inputs may include `name`, user-specified `max_concurrency` and `subagent_model`. The engine compiles and analyzes before display. The normal GUI confirms every new script, including in broad-permission modes. The docs make TUI/headless bypass exceptions, so operational checks matter.
`AmendWorkflow` revises a run, imports finished matching work and may stop a live predecessor. If only max concurrency changes on a live run, it may retune in place. `ResumeWorkflowRun` replays the same byte-identical script against the journal; `SaveWorkflow` is user-opt-in only. `ListWorkflowRuns` and `GetWorkflowRun` inspect; `ResolveWorkflowQuestion` answers subagent escalations; `FillWorkflowHole` handles explicitly deferred code extension when present.
Saved location: `<cwd>/.zcode/workflows/<name>.dwf.ts` or `~/.zcode/workflows/<name>.dwf.ts`, project copy shadows global names. Metadata includes `/* zcode-workflow */`. Draft script work copies may appear in `<cwd>/.zcode/workflow-drafts/`.

## 4. Lifecycle and journal
User opts in → skill loaded → compile/analyze → confirm → background run → dispatch per-actor asks → journal nodes/events → reports/artifacts → completed / errored / stopped. Runner uses a sandboxed child VM with NDJSON bridge to host; journaled world calls and state transitions are durable. Model sees notifications, including completion and unanswered questions. Resume re-executes identical program and reuses journaled work; amend can reuse predecessor results for matching named identities.

## 5. Concurrency
Per-run `max_concurrency` bounds in-flight asks; separate provider-key governor adapts model-request capacity to rate limits. Default parallelism documented as `max(4, min(16, availableParallelism() - 2))`. A user-specified run limit can exceed default but effective throughput remains governor-bound. Live retuning drains/parks excess work between turn steps rather than necessarily cancelling started actors. Never confuse requests, asks, and actors.

## 6. Fault behavior and risk
Compile-time type failures prevent launch. Actor ask errors propagate to the script; logic retries must be explicit and bounded. Provider transients may be internally retried; deterministic provider errors stop according to engine policy. Cancellation signals abort in-flight work. Workflow run status must be read from actual journal/notification, not inferred from launcher `backgrounded`.
Legacy `Workflow` tool is marked high risk/workspace mutating and child agents may be yolo; expert `/expert` docs also describe default yolo. Do not generalize dynamic approval behavior to legacy systems.

## 7. Expert scheduler reference
Built-in `/expert` is orchestration around child `AgentRuntime` sessions, not a separate thinking loop. `WorkflowDefinition` contains ID/version/kind/title/phase order/strategy and phases with behaviors `agent`, `scheduled_graph`, `critic`, `complete`. Generic `WorkflowGraphScheduler` advances ready DAG nodes, records dependencies and activity→child-session links. Storage is generally `~/.zcode/cli/workflows/definitions/` and `runs/<id>/` with `run.json`, `events.jsonl`, `graph.jsonl`, artifacts and reports. App-server methods documented: `zcode.dev/workflow/start|list|get|cancel` and event notifications. Graph inspection uses derived `scheduler` plus snapshot.

## 8. UI caveat
The official `docs/workflow-debugger.md` reports a sidepane UI and shared types exist, **but its task-service adapter in that branch returns empty state and mutations unsupported**. Don't equate the UI's presence with live workflow APIs, and don't claim an installed desktop reconnect is verified.

## 9. Research limits
This package checked official docs and Context7 output, not a live local Windows ZCode installation, UI screens, actual tool schemas on that installation, or end-to-end tests. Docs were authored across several development dates and may conflict; classify an exact installed release before giving definitive operational promises.
