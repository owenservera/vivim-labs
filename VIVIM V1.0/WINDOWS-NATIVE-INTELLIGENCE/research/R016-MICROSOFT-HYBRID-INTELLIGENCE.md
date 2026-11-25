# R016 — Microsoft next-generation Windows intelligence research
Research date: 2026-11-25. Owner requested pause on implementation and a fresh deep research pass. No production modifications, dependency installs or live capability registration. R015 remains the previous tested integration candidate.

## Decision proposal
Highest new donor: Microsoft Research Hybrid Computer Use (HCU), paper submitted 2026-10-30. Use HCU's interaction-state contract and UFO's app-specialized hybrid capability architecture to strengthen incumbent Studio/Harness and Host. Do not run a second browser surface, agent OS, MCP authority or renderer. GPUI/PWA remain one surface; native Rust default. These are research proposals, not accepted integration decisions.

## Source pins verified through GitHub
- microsoft/Hybrid-Computer-Use: 8ce82b310b014ab1133b28febc2eca81527e43e6 (MIT license inspected). Paper https://arxiv.org/abs/2610.41432 ; https://www.microsoft.com/en-us/research/publication/hybrid-computer-use-a-unified-programmatic-semantic-and-visual-interface-for-general-agents/
- microsoft/UFO: eae3c0a8d168c47cb10df0652e293e092eda1078. https://github.com/microsoft/UFO ; UFO2 paper published May 2026 https://www.microsoft.com/en-us/research/publication/ufo2-the-desktop-agentos/
- microsoft/winappCli: c9e6a9bdf010add8edaddb790689e3cbaae33cd5. November 9 2026 CLI 0.8 public preview: experimental WinUI DevTools, local ETW profiling, C++ project workflows. https://learn.microsoft.com/en-us/windows/apps/whats-new/whats-new-for-developers
- microsoft/mxc: af0d9e9e108f64aeb87b65e9f7253c740e1c47ea. MXC Windows GA announced October 7; further preview process containment features announced November 17. https://blogs.windows.com/windowsexperience/2026/10/07/building-windows-for-hybrid-intelligence/ ; https://blogs.windows.com/windowsexperience/2026/11/17/microsoft-ignite-2026-building-windows-for-the-next-era-of-work/

## HCU source inspection
At pinned HCU commit inspected docs/CODE.md (blob e581cba2e7028cba3b36648136ef3bb818c3936c), docs/PLATFORMS.md (e7dc40a87e8f07e25b72ad304755fafe3c364ed1), computer-use/internal/core/ax_ref_registry.go (36182c2f3a151fb272643ec50a4113750fefb245), request_replay.go (fdf75d95fe0d59c12c5e4d6d180cff2375961af9), ax_snapshot_store.go (a9f4534317caf4b94d64e7b8e2e854d15f9c93f0). Reviewed docs and source declarations/comments, not full repository audit or runtime testing. Core Go; Windows native C#/.NET 8 FlaUI UIA3; interactive session and compatible integrity needed. Do not deploy whole server. Extract:
1. Shared semantic target references with lifecycle, application/window ownership, generation and revalidation.
2. Per-consumer delivered snapshot baselines, bounded deltas and loss-triggered full snapshots.
3. Ordered typed action batches with explicit conditional waits, step-level effects/readback, halt on uncertainty; use Host existing approval and replay contracts, not HCU store as separate authority. In-memory replay does not prove durable exactly-once across crashes.
4. Response budget, targeted inspect/read/search and explicit ambiguity; no passwords, raw sensitive value or unrestricted visual capture.
5. Frame coordinates/revision fencing and unified semantic/visual evidence; no arbitrary blind fallback after an uncertain input effect.
6. Schema parity: one canonical Host registry projected to MCP, GPUI and VLANG. HCU's shared core is evidence of direction, not code replacement.
Candidate mapping: WNI05, WNI07, WNI11, existing Studio/Harness and Host. R033 browser convergence hold remains in effect; handoff to dedicated owner, no independent browser automation.

## UFO and WinApp research via Context7
Context7 /microsoft/ufo returned hybrid control detection (UIA + OmniParser merge), application-specific native COM and UI MCP roles, config/ufo/mcp.yaml. Extract provider capability profiles, semantic-first detection and app-specific API adapters. Use app-scoped observation and approval, no unconditional COM or GUI fallback after unknown effect. Do not adopt Python/Conda/LLM runtime or Galaxy service. UFO cross-device task DAG useful as design input to existing Work; no competing scheduler. 
Context7 /microsoft/winappcli returned inspect/search/wait-for, and value-setting fallback ValuePattern, RangeValuePattern, LegacyIAccessible for compatible controls; unsupported controls fail clearly. Extract selector semantics, bounded waits, test assertions and API discovery. Use existing GPUI renderer; WinUI DevTools itself targets WinUI, not GPUI. Optional CLI test donor, not mandatory runtime. HCU not indexed by Context7 under exact name; inspected primary pinned GitHub source instead. No unrelated library substituted.

## Other current Windows developments
- Windows AI APIs and Windows ML: shared ONNX runtime and dynamic execution providers, hardware eligibility per API. https://learn.microsoft.com/en-us/windows/ai/apis ; https://learn.microsoft.com/en-us/windows/ai/new-windows-ml/overview ; https://learn.microsoft.com/en-us/windows/ai/apis/troubleshooting . WNI10 capability discovery only, then optional suitable execution path. Existing machine is hardware constrained; do not assume Copilot+ or compatible GPU. No compulsory model downloads. Foundry Local uses OpenAI-compatible API, so would need explicit owner exception to current web-app provider rules if used for inference; no activation.
- HydraFusion: October 7 announcement of hybrid local/cloud routing; no reusable standalone SDK verified in this pass. https://commandline.microsoft.com/local-models-sandboxed-tools-github-windows/ . Borrow architectural idea, not product dependency. Existing deterministic task execution and approved provider registry remain canonical.
- App Actions, Agent Launchers and Windows MCP registration: https://learn.microsoft.com/en-us/windows/ai/app-actions/ ; https://learn.microsoft.com/en-us/windows/ai/agent-launchers/agents-get-started ; https://learn.microsoft.com/en-us/windows/ai/mcp/overview . WNI06 optional OS projections; discover availability, package/signing/preview constraints, and always use Host authorization. Do not require Windows preview features for V1.
- MXC: optional scoped untrusted workload execution backend, complements existing Job Object lifecycle but does not replace it. Do not weaken isolation or auto-grant.
- OmniParser: https://github.com/microsoft/OmniParser . Useful visual fallback research; model weights/runtime overhead and component licenses require review. No default install.
- Intelligent Terminal 0.2.2572: current Microsoft developer roundup reports performance and reliability improvements; existing WNI03 ACP/ConPTY remains preferred extraction, no duplicate terminal. https://learn.microsoft.com/en-us/windows/apps/whats-new/whats-new-for-developers
- Windows Search redesign announced, rollout pending: retain established SystemIndex APIs, do not build on unreleased OS search UI.

## Proposed ordered research-to-integration queue
1. HCU semantic reference lifecycle and revision fence (Rust pure core; fixture).
2. HCU differential observation and bounded delivery (reuse current live state source).
3. HCU typed batches/waits and uncertain-effect contract (reuse existing Host receipts).
4. UFO app profile and native action discovery (schema, current grant, selected app).
5. UIA event/cache primitive on approved local fixture; no browser integration in this workstream.
6. WinApp selector and semantic readback tests; optional developer tooling.
7. Optional MXC support/readback probe; no automatic containment changes.
8. Windows ML hardware/runtime readiness probe; no model downloads.
9. Windows App Actions/OS MCP registration mapping; optional.
10. Performance benchmark: native semantic operations vs repeated visual inference, context size, latency, correctness and recovery. No 10x claim without measurement.

Acceptance for each candidate: source pin/license, minimal dependencies, exact existing code mapping, actual isolated fixture, default-denied scope, effect receipt and unknown-outcome recovery, source owner integration review. Production new capability count remains zero. No change to ARBOR.
