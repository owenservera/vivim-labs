# WNI05 — native-app-semantics

Priority: P0. Integration owner: dedicated Studio/Harness convergence. Source readiness: donor extraction recipe only; R033 hold on integration. All public action candidates remain proposed until wired and independently verified.

## Extraction
Port semantic selector, element identity, wait-for assertion and cooperative desktop-turn arbitration into existing Studio/Harness. Rust windows COM UIA, selected app only; mask password controls, no arbitrary desktop recording; actions must be registered and authorized. Bind window identity + runtime ID + semantic fingerprint + observed revision. Reject stale selectors, serialize focus ownership and read back after action. No WinApp UI runtime or browser controller; library donor is .NET, so use patterns only unless approved otherwise.

## Shared capability actions
- `windows.application.inspect` / `vivim_windows_application_inspect` — R0; fields: app_id. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.application.element.query` / `vivim_windows_application_element_query` — R0; fields: app_id, query, limit. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.application.action.invoke` / `vivim_windows_application_action_invoke` — R2; fields: app_id, element_ref, action, revision. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.

## Acceptance
Notepad fixture unique selector, ambiguous selector refusal, state change invalidates stale revision, selected-app readback, no competing renderer or focus contention.

## Source references
- https://github.com/microsoft/winappCli/blob/c9e6a9bdf010add8edaddb790689e3cbaae33cd5/docs/ui-automation.md
- https://github.com/microsoft/winappCli/blob/c9e6a9bdf010add8edaddb790689e3cbaae33cd5/src/winapp-CLI/WinApp.UIAutomation/PACKAGE.md
- https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-eventsoverview

## Integration handoff
Review existing code and claims before edits. Stage through existing source-pinned build, no new daemon, no automatic effects or grants. Register candidate IDs in existing Host catalog; only mark source_wired after actual route/adapter exists. MCP and GPUI must use same dispatcher and live authorization; readback, cancellation and unsupported outcomes are explicit. Store approved observations in existing semantic fabric, never a competing database. Native collector and app acceptance are UNRUN unless documented separately.
