# WNI08 — configuration-reconciliation

Priority: P1. Integration owner: Settings + Host. Source readiness: adapter recipe only. All public action candidates remain proposed until wired and independently verified.

## Extraction
Read-only inventory first; existing Settings stores desired state. WinGet/DSC optional supported commands, version probe, pinned trusted manifests. Plan diff before mutation; exact Host approval; only supported owner-selected packages, no silent elevation, install scripts or broad security setting changes. Track source digest and readback; rollback only where supported. Do not make DSC or WinGet a mandatory service.

## Shared capability actions
- `windows.system.configuration.inspect` / `vivim_windows_system_configuration_inspect` — R0; fields: profile_id. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.system.configuration.plan` / `vivim_windows_system_configuration_plan` — R0; fields: profile_id. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.system.configuration.apply` / `vivim_windows_system_configuration_apply` — R2; fields: plan_id, revision. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.

## Acceptance
Read installed versions, plan known harmless fixture, demonstrate dry-run leaves state unchanged; approved apply with readback and explicit rollback capability.

## Source references
- https://learn.microsoft.com/en-us/windows/package-manager/configuration/
- https://learn.microsoft.com/en-us/powershell/dsc/overview

## Integration handoff
Review existing code and claims before edits. Stage through existing source-pinned build, no new daemon, no automatic effects or grants. Register candidate IDs in existing Host catalog; only mark source_wired after actual route/adapter exists. MCP and GPUI must use same dispatcher and live authorization; readback, cancellation and unsupported outcomes are explicit. Store approved observations in existing semantic fabric, never a competing database. Native collector and app acceptance are UNRUN unless documented separately.
