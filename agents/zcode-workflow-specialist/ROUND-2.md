# Research round 2 — what changed and why

Date: 2026-11-13. Official repo: `zai-org/zcode`; Context7 ID `/zai-org/zcode`. GitHub search indexed the observed source at `fec975905a2abd4c07e35c40d51fa097a2258ec9` (use live main/installed release as final authority).

## Findings materially affecting the agent
1. ZCode's actual custom-agent loader reads direct `<workspace>/.zcode/agents/*.md` Markdown files with frontmatter. A Labs folder alone is not an installed agent. Added a native distribution profile.
2. A custom agent loaded through the Agent tool is a **subagent**, not the parent turn. Dynamic scripts prohibit nested CreateWorkflow inside subagents. Split the profile into a design-only specialist plus explicit parent execution contract.
3. Official `dynamic-workflows` skill contains ~92KB and defines the exact facade in §16.2; its `patterns.md` lists 16 shapes and `examples.md` contains seven end-to-end scripts. The prior package's prose examples are secondary, not compiler authority.
4. Source verified: `report(item, artifactId?)` is synchronous, `artifact.markdown` is async, `phase` is required with at least one ask or world.run, snippets cannot host agent/phase/report/artifact.
5. Standalone CLI defaults to disabled; `--workflow-mode onDemand|alwaysOn` enables it. Desktop uses Host/server/user choice. On-demand tool absence before activation does not itself prove the feature is unavailable.
6. The parser test shows `permissionMode` frontmatter may be ignored; cannot trust a Markdown permission field to enforce no writes. Shipping profile restricts its tools to Read/Grep/Glob, and its instructions require parent-side permission checks.
7. The older workflow and `/expert` are distinct from the modern DWF. Do not infer old command removals mean the dynamic authoring command is absent.
8. Existing desktop debugger documentation describes a stubbed service adapter. End-to-end UI correctness remains an installed-client verification item.

## Package additions
- Native installable custom subagent profile under `install/project/.zcode/agents/`.
- `MAIN-SESSION.md` parent/child authority split.
- `API-REFERENCE.md` checked tool/TS facade contract and caps.
- `INSTALL-AND-VERIFY.md` Windows setup and smoke checks.
- `examples/read-only-audit.ts` source-reviewed script (unexecuted).
- `tests/ACCEPTANCE.md` reproducible readiness suite.
- This delta report.

## Known limitations
- No installed Windows ZCode desktop or CLI was available in this remote GitHub/Context7 session for execution.
- The custom agent has not been automatically deployed into any VIVIM/ARBOR project root; copying it requires a deliberate target.
- No workflow was run, approved or saved through the installed model tools; do not call the package fully runtime-accepted yet.
- This package is not a mirror of the full third-party skill body. Official upstream source links and the installed built-in Skill are authoritative.
