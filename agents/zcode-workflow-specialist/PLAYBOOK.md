# Practical ZCode workflow playbook

## Decision table
| Goal | Pattern | Agents | Control |
|---|---|---|---|
| Single deterministic analysis | serial | one | await |
| Many independent files | map/fan-out | separate unique actors | Promise.all, bounded cap |
| Deep claim verification | maker/checker | 2 distinct actors | bounded loop with verdict |
| Multi-stage study | phase sequence | specialist per phase | typed outputs, phase() |
| Conditionally escalate | conditional | targeted actor | if/else, escalate |
| Continuous dependent processing | stream | producer/consumer | channel/future if installed |
| Large autonomous implementation | expert DAG | phase/node child sessions | /expert, graph scheduler |

## Dynamic script example: read-only inventory and independent verification
*Illustrative TypeScript body; check installed facade and compile before run. Subagents must obey the read-only task instruction.*

```ts
interface Finding { area: string; evidence: string[]; risk: string }
interface Verdict { verified: boolean; gaps: string[] }
const analyst = agent("Analyst", { system: "Read-only code analyst. Quote precise repo paths and evidence. Never edit." });
const reviewer = agent("Reviewer", { system: "Independent read-only reviewer. Reject unsupported claims. Never edit." });
phase("Inspect");
const findings = await analyst.ask<Finding[]>("Inspect workflow-related code and return grounded findings. Do not modify anything.");
phase("Verify");
const verdict = await reviewer.ask<Verdict>("Check whether these findings are supported: " + JSON.stringify(findings));
report({ findings, verdict });
return { findings, verdict };
```

**Caution:** The exact `report()` overload must be checked in installed facade; replace its example call with the actual declared signature if compilation rejects it. If the actor tool policy cannot enforce read-only, use a no-mutation approval mode or refrain from launching.

## Example: bounded implement-check loop
```ts
interface PatchResult { summary: string; files: string[] }
interface CheckResult { pass: boolean; evidence: string[]; remaining: string[] }
const builder = agent("Builder", "Make minimal changes; preserve repository ownership; return concrete file paths.");
const checker = agent("Checker", "Independently run the narrowest relevant checks; do not approve untested claims.");
phase("Build");
let patch = await builder.ask<PatchResult>("Implement the approved task; report actual changed paths.");
phase("Check");
let result = await checker.ask<CheckResult>("Verify changed paths: " + JSON.stringify(patch.files));
let attempts = 1;
while (!result.pass && attempts < 3) {
  attempts++;
  patch = await builder.ask<PatchResult>("Fix these verified gaps only: " + JSON.stringify(result.remaining));
  result = await checker.ask<CheckResult>("Re-run tests and verify: " + JSON.stringify(patch.files));
}
return { patch, result, attempts };
```

## Before launch
1. User expressly requested a workflow; skill loaded.
2. Installed tool supports the specific dynamic engine; permission is known.
3. Every actor name unique; schemas typed, errors anticipated.
4. Fan-out has a meaningful cap; work ownership/disjoint paths established.
5. Script valid under strict ES2022-only TypeScript; no Node/DOM.
6. Actual compiler/analyzer produces a clean graph and expected mutations.
7. User consent obtained at relevant host boundary.
8. Output paths, verification procedure, and run tracking defined.

## During/after launch
Capture run ID; don't conflate `backgrounded` with completion. Observe notifications and journal-backed status. On stopped process, resume byte-identical script. For code changes or changed dependencies, amend instead of misusing resume. If workflow errors after partial writes, inspect actual workspace state before retrying (journals make actor asks replayable, not arbitrary filesystem side effects transactionally reversible).

## Acceptance suite (execute only on suitable authorized instance)
- T01 skill gate: authoring without skill fails; loading skill enables same input.
- T02 typed result: malformed subagent answer rejected.
- T03 fan-out: unique actors run in parallel within selected cap.
- T04 duplicate actor: compiler/runtime refuses repeated names.
- T05 static graph: phase ordering, branch, joins and loop shown.
- T06 journal: run, actors, nodes and events persisted before output.
- T07 resume: interrupted run reuses completed matching work.
- T08 amend: changed script produces successor and correctly reuses compatible work.
- T09 approval: confirmation/denial observed in applicable GUI; TUI/headless caveat recorded.
- T10 saved workflows: project overrides global same-name entry.
- T11 provider throttling: governor/backoff works with observed evidence.
- T12 error states: cancelled, errored and stopped distinguished.
- T13 escalate: parent question visible, resolved with correct tool.
- T14 desktop debugging: real adapter returns live graph or mark unsupported.
- T15 provenance: final artifacts include source, check result and exact run IDs.

## Installation integration
This folder is library-side intelligence. Install/use via ZCode's **actual documented custom-agent loading mechanism** once verified on the target build; do not assume that copying `AGENT.md` into a generic directory auto-registers it. On VIVIM use the specialist as a prompt/agent profile, with Host and Rust remaining authoritative.

## Round 2 implementation corrections
The sample `report({...})` calls are valid against official §16.2 `report(item, artifactId?)`. Async content artifact publication must be `await artifact.markdown(id, content, options)` or `await artifact.file(...)`. This file contains teaching patterns; the source-reviewed full example is in `examples/read-only-audit.ts`, and the official bundled skill and examples are the compiler contract. `EvalWorkflowSnippet` cannot be used to validate `agent`, `phase`, `report` or `artifact`: only its supported subset.

Crucially, a project `.zcode/agents/*.md` custom specialist is a *subagent*. Have it return a workflow to the parent for submission, never direct it to call nested `CreateWorkflow`. Standalone CLI mode is disabled by default and must be activated in the correct client. In a live workspace, read-only prompts are not a security boundary.
