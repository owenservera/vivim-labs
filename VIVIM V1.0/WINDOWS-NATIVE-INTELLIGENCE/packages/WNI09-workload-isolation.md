# WNI09 — workload-isolation

Priority: P1. Integration owner: native workstation + Host. Source readiness: optional backend recipe only. All public action candidates remain proposed until wired and independently verified.

## Extraction
Probe Microsoft Execution Containers supported backends before dependency adoption; optional mxc-sdk feature, explicit policy and trust model, no host prep by default. Existing Job Object process containment is lifecycle control, not sandbox. Use isolation for approved delegated workload only; never silently downgrade a requested isolation guarantee.

## Shared capability actions
- `windows.workload.isolation.status` / `vivim_windows_workload_isolation_status` — R0; fields: none. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.workload.isolation.plan` / `vivim_windows_workload_isolation_plan` — R0; fields: profile_id. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.workload.isolation.run` / `vivim_windows_workload_isolation_run` — R2; fields: plan_id, revision. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.

## Acceptance
Supported backend starts fixture with specified resources, unauthorized resource denied, stop cleans workload, unsupported backend fails closed; record memory/disk overhead.

## Source references
- https://learn.microsoft.com/en-us/windows/apps/develop/mxc/
- https://learn.microsoft.com/en-us/windows/apps/develop/mxc/releases

## Integration handoff
Review existing code and claims before edits. Stage through existing source-pinned build, no new daemon, no automatic effects or grants. Register candidate IDs in existing Host catalog; only mark source_wired after actual route/adapter exists. MCP and GPUI must use same dispatcher and live authorization; readback, cancellation and unsupported outcomes are explicit. Store approved observations in existing semantic fabric, never a competing database. Native collector and app acceptance are UNRUN unless documented separately.
