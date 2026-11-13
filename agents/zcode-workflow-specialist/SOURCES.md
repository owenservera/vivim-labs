# Source map, provenance, gaps

Reference corpus: `https://github.com/zai-org/zcode` (main), Context7 ID `/zai-org/zcode`, indexed at 25,931 examples when resolved 2026-11-13.

## Canonical entrypoints
- Dynamic feature overview: https://github.com/zai-org/zcode/blob/main/docs/dynamic-workflow.md
- Authoring/skill gate/facade: https://github.com/zai-org/zcode/blob/main/docs/dynamic-workflow/authoring.md
- Launch/saved/restart/amend: https://github.com/zai-org/zcode/blob/main/docs/dynamic-workflow/launch.md
- Static analysis: https://github.com/zai-org/zcode/blob/main/apps/zcode-cli/packages/dynamic-workflow/docs/analysis.md
- Engine and journal: https://github.com/zai-org/zcode/blob/main/apps/zcode-cli/packages/dynamic-workflow/docs/execution-engine.md
- Concurrency: https://github.com/zai-org/zcode/blob/main/docs/dynamic-workflow/concurrency.md
- Presentation: https://github.com/zai-org/zcode/blob/main/docs/dynamic-workflow/presentation.md
- Notifications: https://github.com/zai-org/zcode/blob/main/docs/dynamic-workflow/transcript-and-notifications.md
- Sandbox runtime: https://github.com/zai-org/zcode/blob/main/apps/zcode-cli/packages/dynamic-workflow-runtime/README.md
- Legacy Workflow tool: https://github.com/zai-org/zcode/blob/main/apps/zcode-cli/docs/design/v2/tool/15-workflow.md
- Expert workflows: https://github.com/zai-org/zcode/blob/main/apps/zcode-cli/docs/design/v2/expert-workflow.md
- Expert runtime split: https://github.com/zai-org/zcode/blob/main/apps/zcode-cli/docs/design/v2/workflow-expert-runtime-split.md
- Workflow debugger caveat: https://github.com/zai-org/zcode/blob/main/docs/workflow-debugger.md
- Client feature modes: https://github.com/zai-org/zcode/blob/main/packages/shared/src/dynamic-workflow-feature.ts

## Verification ledger
SOURCE_VERIFIED: modern tools, TS facade, types/schemas/graphs, site/instance model, journal, saved scope, dynamic confirmation, bounded asks and governor, `/expert` as separate runtime, legacy Workflow as separate tool.
DOCUMENTED_ONLY: actual UI availability in any user's currently installed release, current product flag, exact tool schema and allowed mode, active runtime integration, successful execution.
CONFLICT/CAUTION: modern `/workflow <instructions>` authoring route versus legacy `/workflow run` removal (different systems); desktop workflow debugger adapter stubbed in documented branch; local ZCode settings might differ from public main.
UNRUN: live ZCode smoke test, script compiler probe, saved workflow round trip, cancellation/resume/replay, permission and provider rate-limit testing.

## Recommended next verification
Open installed ZCode, capture version/commit, list available tools and dynamic workflow mode, load `dynamic-workflows` skill, run one read-only typed workflow, inspect events and validate persistence. Only then mark installed functionality LIVE_VERIFIED.
