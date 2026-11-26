# Virtual Semantic Surface — VIVIM research prototype

Read-only native Windows semantic desktop observer. No screenshots, graphics capture, virtual display driver, keyboard hooks, input injection, network listener, installation or auto-start. Run explicitly in your signed-in desktop session. The console reports that observation is active. Stop with Ctrl+C. Output is ephemeral NDJSON unless the operator explicitly redirects it. Default output omits accessible names; `--labels` explicitly includes non-password accessible names, which can contain private data. No Value/Text pattern reads.

## Build / run

Windows with Rust MSVC toolchain and linker installed:

```
cargo build --release
cargo test
cargo run --release -- --once
cargo run --release -- --interval-ms 500
```

The executable emits full snapshots when changed and a heartbeat at least every five seconds **when providers respond**. Desktop and window rectangles are physical screen pixels; right/bottom are exclusive. Negative coordinates are valid. Window identity is ephemeral HWND plus PID, element indices are only valid inside the current snapshot. DWM bounds do not imply actual visibility or occlusion. The foreground UIA control tree is bounded to 1500 elements / depth 24; cursor hit-testing uses UIA directly. Window inventory includes visible-style noncloaked windows, including minimized windows flagged as such. Not a complete visual or semantic reconstruction.

## Windows platform research

- UIA desktop tree / hit testing: https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-obtainingelements
- Physical coordinates: https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-screenscaling
- Cache requests: https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-cachingforclients
- DWM window geometry: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect
- Composition boundaries: https://learn.microsoft.com/en-us/windows/win32/api/dcomp/nf-dcomp-idcompositiondesktopdevice-createtargetforhwnd
- Security boundaries: https://learn.microsoft.com/en-us/dotnet/framework/ui-automation/ui-automation-security-overview
- Rust bindings: https://github.com/microsoft/windows-rs

## Next design stages

1. Split geometry worker from UIA worker; add bounded provider timeouts, event subscriptions and coalescing, cached property batches, periodic reconciliation, explicit staleness and provider failure status. UIA provider calls may stall this initial single-thread prototype. Tree traversal failures can yield incomplete results even when the size/depth cap is not reached.
2. Add per-monitor topology and DPI transforms, window z-order and explicit uncertainty for occlusion; UIA runtime IDs plus generation tracking; spatial index; live arbitrary-point query with coordinate/revision validation. Current code only does native hit-testing at the current cursor.
3. Add authorized app-specific adapters (own GPUI semantic graph, cooperative browser accessibility tree, custom renderers). Do not circumvent privilege, secure desktop or protected-content boundaries. Expose local authenticated IPC only after a deliberate integration decision.
4. Interaction layer is separate: resolve target, revalidate against current revision, authorize action, execute through Host, verify result. No interaction execution is implemented here.

## Verification status

Source prototype only. Windows compilation and live interactive desktop acceptance have not been performed. Test cases are included but not executed. Initial implementation uses periodic polling, not the planned event-driven pipeline.
