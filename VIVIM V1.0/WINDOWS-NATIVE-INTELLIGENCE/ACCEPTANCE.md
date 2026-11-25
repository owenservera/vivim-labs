# MCP-first acceptance and release evidence

Every candidate has a public MCP and app mapping. This is design coverage, not live availability. Before enabling any action: actual Host catalog entry, dispatcher binding, native adapter, schema match, grant decision, MCP invocation, GPUI action and evidence from same Host; PWA parity where applicable. Fail closed for unsupported, missing or revoked grant. Compare action ID and schema, not volatile metric values. Test argument validation, stale revisions, reconnect, cancellation, unsupported hardware, unauthorized scope and error redaction.

| Package | Real acceptance | Current |
|---|---|---|
| WNI01 | Real temp directory create/rename/delete; restart cursor; force gap, rebaseline; non-admin USN refusal; verify no events from unapproved roots. | UNRUN |
| WNI02 | Compare memory/capacity to native OS APIs, verify nonzero CPU delta and invalid sample refusal; emit one owned event, verify correlation, stop and verify no remaining trace session. | UNRUN |
| WNI03 | Real ConPTY echo/resize/interrupt, no stuck handles, child cancellation, two owned ACP sessions with no output mix-up, settled receipt and no unexpected provider charges. | UNRUN |
| WNI04 | Known authorized fixture indexed, query matches it, out-of-scope hit withheld, stale/missing file clearly marked, service disabled fallback; query-injection test. | UNRUN |
| WNI05 | Notepad fixture unique selector, ambiguous selector refusal, state change invalidates stale revision, selected-app readback, no competing renderer or focus contention. | UNRUN |
| WNI06 | On supported Windows, discover documented fixture provider, invoke harmless typed action after Host approval, verify output; unavailable platform returns explicit unsupported; reject arbitrary action ID. | UNRUN |
| WNI07 | Owner selects test window; verify scope, stop/revoke, minimized/occluded support reported accurately, phone and GPUI state consistent; refuse nonselected target. | UNRUN |
| WNI08 | Read installed versions, plan known harmless fixture, demonstrate dry-run leaves state unchanged; approved apply with readback and explicit rollback capability. | UNRUN |
| WNI09 | Supported backend starts fixture with specified resources, unauthorized resource denied, stop cleans workload, unsupported backend fails closed; record memory/disk overhead. | UNRUN |
| WNI10 | Known permitted local fixture, compare latency/RAM/accuracy vs incumbent deterministic method; unsupported GPU/NPU path marked unavailable; no network fallback. | UNRUN |
| WNI11 | Cargo tree before/after, no new mandatory service or runtime, supported Windows tests, cold/warm build time and disk delta. | UNRUN |

Proposed engineering budgets (not benchmarks): no new mandatory process/service, no model downloads, no privileged collector by default; bounded 200 rows/page, 64 KiB signal batch, 16K queued signals maximum with loss marker, idle CPU target <1%, measure actual CPU/RAM/disk on hardware. Resource snapshot request target <250 ms excluding optional sample interval. Baseline existing build and cargo tree; report exact delta. For user-facing events compare baseline repeated scan vs scoped incremental updates using authorized fixtures. Logs and raw capture never published in Labs. No tenfold claim until benchmarked.

MCP-first receipt: source SHA, deployed SHA, action ID, authenticated grant scope, actual input redacted, returned status, evidence/receipt ref, duration, native output schema, app readback revision, negative result, cleanup. Current VIVIM tunnel maintenance/drained; actual Windows native build and MCP/GPUI acceptance blocked. Do not bypass or clear maintenance.
