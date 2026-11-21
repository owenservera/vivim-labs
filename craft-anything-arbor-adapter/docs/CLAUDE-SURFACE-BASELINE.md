# Claude.ai surface coverage: pre-capture baseline, evidence not inferred

**Status: SOURCE-COMPARED, NOT LIVE-VERIFIED.** The purpose of this file is to guide the *next authorized investigation*, not pretend Craft has already scanned Claude.ai. To obtain a full provider-specific interactive atlas, require a permitted observation path and explicit coverage statements. Start the actual automated study with first-party VIVIM MVP surfaces.

## What ARBOR already represents (source evidence)

| Area | Evidence location | Current statement |
|---|---|---|
| Composer hydration | `studio/src/profiles/sites/claude.mjs` | Measured ProseMirror contenteditable; generic `textarea` deliberately omitted due hydration race recorded on 2026-11-04. |
| Harness composer | `harness/src/providers/claude.mjs` | Includes `textarea` fallback; profile difference is worth testing but no runtime failure proven. |
| Send and streaming | Both Claude profiles and `studio/src/studio/channels/claude.mjs` | Existing send/stop selectors, chat transport with fail-closed uncertainty semantics and approval boundary. |
| Transcript roles/identity | Studio Claude profile and `studio/src/capture/conversation-thread.mjs` | Typed user/assistant node selectors, `data-turn-key` identity, `/chat/<uuid>` thread recognition. |
| Model/effort UI | Harness Claude profile | Selector and tier labels represented in profile source; refresh against permitted current UI before treating as valid. |
| Auth wall | Studio and Harness Claude profiles | Login-wall preflight selectors; not proof of sign-in state at runtime. |
| Site capture | `studio/profiles/claude.json` | Scope and capture family configuration only; notes explicitly say live family mapping needs discovery/confirmation. |

## Missing coverage to investigate (NOT claims of absent implementation)

| Surface family | Investigation question | Artifact expected |
|---|---|---|
| Global navigation | Which route and state transitions expose workspaces, chats, projects? | Screen/route atlas + unknowns |
| Conversation list/search | How is pagination, filtering, current-thread identity and state restoration displayed? | Read-only transcript/route evidence |
| Project lifecycle | What pages/dialogs/controls expose project create/edit/archive/navigation? | Screen + state model; no effect activation |
| Project knowledge | What file/text contexts are visible, and how are they labeled? | Metadata-only inventory; no personal content |
| Attachments | What supported controls and progress/error states exist? | Input/state/feedback atlas |
| Artifacts | What creation, preview, version, copy, download and editing states are visible? | Read-only feature map + evidence |
| Models/effort | Which dropdowns, options, disabled states and billing gates exist? | Menu-state comparison |
| Search/research | Which commands and outputs have UI discoverability and reliable status? | State atlas + unknowns |
| Settings/connectors | How are available settings and installed connectors represented? | Public docs inventory / permissioned observation |
| Error/recovery | What are quota, auth expiry, stalled streaming, refresh and rehydration states? | Bounded failure-state evidence |
| Mobile/responsive | Which controls relocate/hide between desktop and phone? | 1440/768/390 capture diffs |
| Accessibility | Accessible role/name, focus, keyboard and dialogs | A11y-derived structure and state evidence |

## Coverage ledger policy

Rows are not booleans. Each feature should carry independent statuses: `documented-only`, `source-present`, `captured`, `mapped-candidate`, `held-out-verified`, `Host-wired`, `live-effect-verified`, `unknown`, `contradictory`. A `source-present` chat selector does not mean complete application coverage; an observed button does not mean the action works or is permitted. Provide exact capture IDs and permission scope for every finding.

## Next useful comparison

Given a genuine Craft bundle from a VIVIM-owned webapp and later permitted provider observations, identify: (a) present site feature with no Studio model; (b) existing Studio feature with no verified evidence; (c) semantic mapping gaps; (d) differences between Studio and Harness profiles; (e) changes in control/route state across site revisions. Output first as non-executable coverage report and only then propose qualified adapters.