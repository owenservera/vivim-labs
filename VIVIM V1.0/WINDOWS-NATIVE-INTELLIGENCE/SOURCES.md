# Source extraction references

Retrieved 2026-11-25; upstream commit pins above. Context7 selected /microsoft/windows-rs. Its returned current-source snippets used newer feature grouping, so the starter deliberately targets ARBOR's existing windows-sys 0.61.2; compile verification pending. Microsoft documents administrator privileges for USN journal operations; do not elevate automatically. ReadDirectoryChangesW supports scoped changes but can lose records. App Actions consumption uses a documented NuGet package; Rust direct interoperability is not yet established. MXC Rust SDK exists but requires supported backend and potentially host preparation. Windows AI APIs are hardware/OS-dependent. No performance or hardware support claims until measured.

## WNI01
- https://learn.microsoft.com/en-us/windows/win32/fileio/using-the-change-journal-identifier
- https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-readdirectorychangesw

## WNI02
- https://learn.microsoft.com/en-us/windows/win32/etw/about-event-tracing
- https://learn.microsoft.com/en-us/windows/win32/etw/consuming-events
- https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-globalmemorystatusex
- https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getsystemtimes

## WNI03
- https://github.com/microsoft/intelligent-terminal/blob/7ef2e637b810d58f20ef53f61275a0ec849cf26f/tools/wta/OVERVIEW.md
- https://github.com/microsoft/intelligent-terminal/blob/7ef2e637b810d58f20ef53f61275a0ec849cf26f/doc/agent-profiles.md
- https://github.com/microsoft/intelligent-terminal/blob/7ef2e637b810d58f20ef53f61275a0ec849cf26f/doc/security-model.md
- https://learn.microsoft.com/en-us/windows/console/createpseudoconsole

## WNI04
- https://learn.microsoft.com/en-us/windows/win32/search/using-sql-and-aqs-to-query-the-index
- https://learn.microsoft.com/en-us/windows/win32/properties/props-system-itempathdisplay

## WNI05
- https://github.com/microsoft/winappCli/blob/c9e6a9bdf010add8edaddb790689e3cbaae33cd5/docs/ui-automation.md
- https://github.com/microsoft/winappCli/blob/c9e6a9bdf010add8edaddb790689e3cbaae33cd5/src/winapp-CLI/WinApp.UIAutomation/PACKAGE.md
- https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-eventsoverview

## WNI06
- https://learn.microsoft.com/en-us/windows/ai/app-actions/
- https://learn.microsoft.com/en-us/windows/ai/app-actions/actions-consume

## WNI07
- https://learn.microsoft.com/en-us/windows/uwp/audio-video-camera/screen-capture
- https://github.com/microsoft/winappCli/blob/c9e6a9bdf010add8edaddb790689e3cbaae33cd5/src/winapp-CLI/WinApp.UIAutomation/PACKAGE.md

## WNI08
- https://learn.microsoft.com/en-us/windows/package-manager/configuration/
- https://learn.microsoft.com/en-us/powershell/dsc/overview

## WNI09
- https://learn.microsoft.com/en-us/windows/apps/develop/mxc/
- https://learn.microsoft.com/en-us/windows/apps/develop/mxc/releases

## WNI10
- https://learn.microsoft.com/en-us/windows/ai/overview
- https://learn.microsoft.com/en-us/windows/ai/apis/

## WNI11
- https://github.com/microsoft/windows-rs
- https://docs.rs/windows-sys/0.61.2/windows_sys/

ARBOR incumbent anchors:
- https://github.com/owenservera/ARBOR/blob/972dbccafea7b9fde47913207c2fefcf02ac8365/native/host-supervisor/Cargo.toml
- https://github.com/owenservera/ARBOR/blob/972dbccafea7b9fde47913207c2fefcf02ac8365/native/host-supervisor/src/workstation_process.rs
- https://github.com/owenservera/ARBOR/blob/972dbccafea7b9fde47913207c2fefcf02ac8365/native/host-supervisor/src/workstation_windows_job.rs
- https://github.com/owenservera/ARBOR/blob/972dbccafea7b9fde47913207c2fefcf02ac8365/apps/vivim-host/control/action-catalog.v1.json
- https://github.com/owenservera/ARBOR/blob/972dbccafea7b9fde47913207c2fefcf02ac8365/apps/vivim-host/control/dispatcher.mjs
- https://github.com/owenservera/ARBOR/blob/972dbccafea7b9fde47913207c2fefcf02ac8365/tools/mcp/vivim-arbor/server/workstation-tunnel.cjs
- https://github.com/owenservera/ARBOR/blob/972dbccafea7b9fde47913207c2fefcf02ac8365/hq/lib/semantic-registry.mjs
- https://github.com/owenservera/ARBOR/blob/972dbccafea7b9fde47913207c2fefcf02ac8365/docs/product/ONE-CANONICAL-LIVE-SURFACE.md
