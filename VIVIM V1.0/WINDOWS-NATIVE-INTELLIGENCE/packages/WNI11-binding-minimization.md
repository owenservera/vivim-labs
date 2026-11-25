# WNI11 — binding-minimization

Priority: P0. Integration owner: native engineering. Source readiness: Cargo.toml feature list + dependency budget. All public action candidates remain proposed until wired and independently verified.

## Extraction
Use incumbent windows-sys 0.61.2 and narrowly scoped Win32 features for raw APIs. Use windows only where COM/WinRT requires it and verify incremental build/disk footprint. No wholesale framework or global package installs. Generate specialized bindings only if measurements justify; pin SDK and license provenance. Capability support means runtime probe, not compile success.

## Shared capability actions
- `windows.system.native.support` / `vivim_windows_system_native_support` — R0; fields: none. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.

## Acceptance
Cargo tree before/after, no new mandatory service or runtime, supported Windows tests, cold/warm build time and disk delta.

## Source references
- https://github.com/microsoft/windows-rs
- https://docs.rs/windows-sys/0.61.2/windows_sys/

## Integration handoff
Review existing code and claims before edits. Stage through existing source-pinned build, no new daemon, no automatic effects or grants. Register candidate IDs in existing Host catalog; only mark source_wired after actual route/adapter exists. MCP and GPUI must use same dispatcher and live authorization; readback, cancellation and unsupported outcomes are explicit. Store approved observations in existing semantic fabric, never a competing database. Native collector and app acceptance are UNRUN unless documented separately.
