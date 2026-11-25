# WNI07 — windows-capture

Priority: P1. Integration owner: dedicated Studio/Harness + GPUI. Source readiness: donor extraction recipe only. All public action candidates remain proposed until wired and independently verified.

## Extraction
Use Windows Graphics Capture picker or supported owner-consented selected-window API; keep session visible and revocable. Capture limited to selected app, no covert desktop surveillance, credential windows excluded, no frames in public evidence. Use GPUI existing render/capture paths; no second renderer or browser surface. Return authorized artifact ref and revision, not raw frames in routine MCP tool responses.

## Shared capability actions
- `windows.application.capture.status` / `vivim_windows_application_capture_status` — R0; fields: none. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.application.capture.request` / `vivim_windows_application_capture_request` — R2; fields: app_id, purpose. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.application.capture.stop` / `vivim_windows_application_capture_stop` — R2; fields: capture_id. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.

## Acceptance
Owner selects test window; verify scope, stop/revoke, minimized/occluded support reported accurately, phone and GPUI state consistent; refuse nonselected target.

## Source references
- https://learn.microsoft.com/en-us/windows/uwp/audio-video-camera/screen-capture
- https://github.com/microsoft/winappCli/blob/c9e6a9bdf010add8edaddb790689e3cbaae33cd5/src/winapp-CLI/WinApp.UIAutomation/PACKAGE.md

## Integration handoff
Review existing code and claims before edits. Stage through existing source-pinned build, no new daemon, no automatic effects or grants. Register candidate IDs in existing Host catalog; only mark source_wired after actual route/adapter exists. MCP and GPUI must use same dispatcher and live authorization; readback, cancellation and unsupported outcomes are explicit. Store approved observations in existing semantic fabric, never a competing database. Native collector and app acceptance are UNRUN unless documented separately.
