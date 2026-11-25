# WNI04 — indexed-search

Priority: P0. Integration owner: HQ + existing file port. Source readiness: adapter recipe only. All public action candidates remain proposed until wired and independently verified.

## Extraction
Use Windows Search OLE DB SystemIndex with fixed SELECT fields, bounded results and properly escaped literal input; never accept caller-supplied SQL. Resolve scope from Host grant, filter returned paths against approved canonical roots and recheck access before opening; index hits are candidate evidence, not proof of current contents. Windows Property System IPropertyStore for allowlisted file metadata. Fall back to existing HQ index, never silently index the whole machine. COM initialization and cleanup on dedicated worker.

## Shared capability actions
- `windows.knowledge.search.status` / `vivim_windows_knowledge_search_status` — R0; fields: none. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.knowledge.search.query` / `vivim_windows_knowledge_search_query` — R0; fields: scope_id, query, limit. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.knowledge.file.properties` / `vivim_windows_knowledge_file_properties` — R0; fields: scope_id, file_id. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.

## Acceptance
Known authorized fixture indexed, query matches it, out-of-scope hit withheld, stale/missing file clearly marked, service disabled fallback; query-injection test.

## Source references
- https://learn.microsoft.com/en-us/windows/win32/search/using-sql-and-aqs-to-query-the-index
- https://learn.microsoft.com/en-us/windows/win32/properties/props-system-itempathdisplay

## Integration handoff
Review existing code and claims before edits. Stage through existing source-pinned build, no new daemon, no automatic effects or grants. Register candidate IDs in existing Host catalog; only mark source_wired after actual route/adapter exists. MCP and GPUI must use same dispatcher and live authorization; readback, cancellation and unsupported outcomes are explicit. Store approved observations in existing semantic fabric, never a competing database. Native collector and app acceptance are UNRUN unless documented separately.
