# WNI01 — filesystem-delta

Priority: P0. Integration owner: native Host + HQ. Source readiness: src/change.rs (parser and revision reducer; OS reader pending). All public action candidates remain proposed until wired and independently verified.

## Extraction
Use ReadDirectoryChangesW on explicitly approved directories as the default non-elevated watcher. Open directory handle with FILE_LIST_DIRECTORY and shared read/write/delete; cancel with CancelIoEx or worker shutdown. Treat zero bytes, ERROR_NOTIFY_ENUM_DIR and bounded queue overflow as a gap. USN_QUERY/READ are optional privileged NTFS capabilities; no auto-elevation, volume-wide filename ingestion or journal creation. Persist journal ID plus cursor only after authorized batch commit. Re-scan approved scope on journal rotation or gap. Do not reconstruct global private file paths from raw volume records.

## Shared capability actions
- `windows.filesystem.changes.status` / `vivim_windows_filesystem_changes_status` — R0; fields: scope_id. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.filesystem.changes.read` / `vivim_windows_filesystem_changes_read` — R0; fields: scope_id, cursor, limit. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.filesystem.changes.configure` / `vivim_windows_filesystem_changes_configure` — R2; fields: scope_id, enabled. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.

## Acceptance
Real temp directory create/rename/delete; restart cursor; force gap, rebaseline; non-admin USN refusal; verify no events from unapproved roots.

## Source references
- https://learn.microsoft.com/en-us/windows/win32/fileio/using-the-change-journal-identifier
- https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-readdirectorychangesw

## Integration handoff
Review existing code and claims before edits. Stage through existing source-pinned build, no new daemon, no automatic effects or grants. Register candidate IDs in existing Host catalog; only mark source_wired after actual route/adapter exists. MCP and GPUI must use same dispatcher and live authorization; readback, cancellation and unsupported outcomes are explicit. Store approved observations in existing semantic fabric, never a competing database. Native collector and app acceptance are UNRUN unless documented separately.
