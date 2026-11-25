# WNI10 — hardware-adaptive-ai

Priority: P2. Integration owner: native intelligence + model registry. Source readiness: adapter recipe only. All public action candidates remain proposed until wired and independently verified.

## Extraction
Probe supported Windows ML/Foundry execution providers and OS/hardware requirements. Use existing model registry and explicit resource budgets; no automatic model downloads, paid fallback or assumption of NPU. Small CPU task benchmarks first. Keep local inference optional; feed bounded typed outputs to existing verifier/semantic engine. Preserve model license and data provenance.

## Shared capability actions
- `windows.intelligence.local.status` / `vivim_windows_intelligence_local_status` — R0; fields: none. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.intelligence.local.evaluate` / `vivim_windows_intelligence_local_evaluate` — R2; fields: model_id, fixture_ref, budget_id. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.

## Acceptance
Known permitted local fixture, compare latency/RAM/accuracy vs incumbent deterministic method; unsupported GPU/NPU path marked unavailable; no network fallback.

## Source references
- https://learn.microsoft.com/en-us/windows/ai/overview
- https://learn.microsoft.com/en-us/windows/ai/apis/

## Integration handoff
Review existing code and claims before edits. Stage through existing source-pinned build, no new daemon, no automatic effects or grants. Register candidate IDs in existing Host catalog; only mark source_wired after actual route/adapter exists. MCP and GPUI must use same dispatcher and live authorization; readback, cancellation and unsupported outcomes are explicit. Store approved observations in existing semantic fabric, never a competing database. Native collector and app acceptance are UNRUN unless documented separately.
