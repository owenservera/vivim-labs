# WNI06 — app-actions

Priority: P0. Integration owner: Studio/Harness + VLANG registry. Source readiness: adapter recipe only; SDK availability probe required. All public action candidates remain proposed until wired and independently verified.

## Extraction
Microsoft.AI.Actions consumer SDK is documented; verify package identity, OS version, supported provider registration and Rust/WinRT accessibility before choosing a minimal adapter. Import action metadata as proposed semantic IDs; never auto-promote discovery to execution rights. Bind typed arguments, target identity and risk through Host. Optional adapter, no Copilot dependency, no blanket claim every app supports actions.

## Shared capability actions
- `windows.application.actions.discover` / `vivim_windows_application_actions_discover` — R0; fields: app_id. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.application.action.describe` / `vivim_windows_application_action_describe` — R0; fields: action_id. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.application.action.execute` / `vivim_windows_application_action_execute` — R2; fields: action_id, input_ref, revision. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.

## Acceptance
On supported Windows, discover documented fixture provider, invoke harmless typed action after Host approval, verify output; unavailable platform returns explicit unsupported; reject arbitrary action ID.

## Source references
- https://learn.microsoft.com/en-us/windows/ai/app-actions/
- https://learn.microsoft.com/en-us/windows/ai/app-actions/actions-consume

## Integration handoff
Review existing code and claims before edits. Stage through existing source-pinned build, no new daemon, no automatic effects or grants. Register candidate IDs in existing Host catalog; only mark source_wired after actual route/adapter exists. MCP and GPUI must use same dispatcher and live authorization; readback, cancellation and unsupported outcomes are explicit. Store approved observations in existing semantic fabric, never a competing database. Native collector and app acceptance are UNRUN unless documented separately.
