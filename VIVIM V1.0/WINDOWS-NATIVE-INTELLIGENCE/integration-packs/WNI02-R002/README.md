# WNI02 R002 — Native resource vertical slice (exact-source integration overlay)

Pinned ARBOR base: [67859e96c12a55d4341d4008cf8a5b1528d18eea](https://github.com/owenservera/ARBOR/commit/67859e96c12a55d4341d4008cf8a5b1528d18eea). No ARBOR source changed. This is a separately reviewed, pull-in candidate, not a deployed feature. The overlay extends existing probe binary (no new executable), windows-sys feature SystemInformation, existing Host dispatcher/catalog, private grant/root config, existing MCP projection, release seal, Windows build/staging, and native GPUI control client. Existing opt-in scope and expiry are retained. No new authority or remote self-grant.

## Pull-in

From this pack, run `node prepare-overlay.mjs <ARBOR checkout>` for read-only exact-blob preflight; optional second argument is a NEW output directory outside ARBOR containing only replacement/addition files. Copy those files into a reviewed integration checkout only after inspecting diff and reconciling other owners. The tool refuses drift rather than guessing. It does not commit, install, modify the source checkout or activate. All 9 existing files are pinned by Git blob SHA, with 4 new files. Do not apply to newer head without rebase/retest. EnableNativeResources is a separate optional build/stage flag; default disabled. Use incumbent supervised release lifecycle.

The public action `windows.system.resources.read` has zero arguments, existing typed desktop owner admission, R0 and bounded aggregate output. MCP name `vivim_windows_system_resources_read` calls same Host plan. GPUI read_native_resources() uses existing owner-authenticated request path; wire its typed result into existing Tools/Developer action and current GPUI result renderer (not a second UI). This is a source-level GPUI client, NOT a connected menu/button or physical app acceptance. Current dispatcher intentionally restricts workstation actions to authenticated typed desktop owner, so phone execution remains unavailable; do not weaken the device grant. Mobile parity requires the existing device-scope owner to integrate. Natural-language discovery may propose the typed action but does not authorize it.

## Tests

Cloud V8 executed 11/11 checks: exact replay, catalog ID, MCP/Host/GPUI identity, CJS syntax and pure native-result validation. Node fixture tests and native Rust build/tests supplied, UNRUN: cloud has no Rust compiler and live VIVIM was under maintenance in R001. Execute on supported Windows after source review: existing Build-WorkstationRelease tests (new port and tunnel suites included), native probe --native-resources, stage with -EnableNativeResources, verify seal, live MCP call and GPUI action, default-denied and revoked/expired grant tests. Confirm GPUI render and mobile disposition separately. No claims of compiled or live integration.

## Research

Context7 /microsoft/windows-rs/0.61.0 (feature-gated Win32 raw bindings), incumbent windows-sys 0.61.2. Microsoft's GetSystemTimes documentation explains that on >64 logical processors the returned counters cover the calling processor group. Result explicitly labels CPU scope instead of claiming whole-machine utilization. GlobalMemoryStatusEx requires dwLength initialized. No extra crate or service beyond existing dependency. Sources: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getsystemtimes and https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-globalmemorystatusex. Exact version build still required. ETW, volume capacity, directory watchers and ACP are not implemented in this slice. No raw trace or private data collection.

## Known remaining work

Catalog native argument types only string/enum; R001 limit integer and enabled flag descriptors must be translated to bounded strings/enums at Host/MCP boundary, with native checked parsing. Probe failure is error, unsupported is explicit, partial CPU is not zero. R001 proposed 33 actions remain proposed; one exact-source overlay prepared. Build verification may reveal additional installer/discovery interactions. No live permission change.
