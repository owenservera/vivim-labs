# WNI02 — system-diagnostics

Priority: P0. Integration owner: native Host + workstation. Source readiness: src/resources.rs (native aggregate CPU/memory and selected fixed-drive capacity); ETW adapter pending. All public action candidates remain proposed until wired and independently verified.

## Extraction
Aggregate memory and system CPU deltas through Win32; report unavailable for unsupported processor group coverage rather than imply whole-machine certainty. Drive capacity only for an approved fixed drive. ETW start with VIVIM-owned provider and owned task correlation; StartTrace/EnableTraceEx2/OpenTrace/ProcessTrace with cancellation and bounded event queue; TDH decode allowlisted fields. No command lines, unrelated process payloads, credentials, network payloads or persistent system-wide tracing by default. Preserve dropped-event count; correlate owned process creation time + session ID, not PID alone.

## Shared capability actions
- `windows.system.resources.read` / `vivim_windows_system_resources_read` — R0; fields: none. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.system.volume.capacity` / `vivim_windows_system_volume_capacity` — R0; fields: drive. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.system.diagnostics.read` / `vivim_windows_system_diagnostics_read` — R0; fields: session_id, cursor, limit. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.system.diagnostics.configure` / `vivim_windows_system_diagnostics_configure` — R2; fields: scope_id, enabled. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.

## Acceptance
Compare memory/capacity to native OS APIs, verify nonzero CPU delta and invalid sample refusal; emit one owned event, verify correlation, stop and verify no remaining trace session.

## Source references
- https://learn.microsoft.com/en-us/windows/win32/etw/about-event-tracing
- https://learn.microsoft.com/en-us/windows/win32/etw/consuming-events
- https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-globalmemorystatusex
- https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getsystemtimes

## Integration handoff
Review existing code and claims before edits. Stage through existing source-pinned build, no new daemon, no automatic effects or grants. Register candidate IDs in existing Host catalog; only mark source_wired after actual route/adapter exists. MCP and GPUI must use same dispatcher and live authorization; readback, cancellation and unsupported outcomes are explicit. Store approved observations in existing semantic fabric, never a competing database. Native collector and app acceptance are UNRUN unless documented separately.
