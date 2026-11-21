# Mapping the 20 capability use cases to Craft files

Every row is a conceptual feature target, not a claim that ARBOR has this capability implemented or the source file has been audited end to end.

| # | Capability | Craft primary source candidates | ARBOR owner | Sequence |
|---|---|---|---|---|
| 1 | Universal application understanding | website_tools.rs, js_app.rs, html.rs | Studio/Atlas | W0 then W2 |
| 2 | Teach-by-demonstration workflows | browser_tools.rs, script_check.rs | Studio/Host | W0; no auto execution |
| 3 | Dynamic UI state intelligence | website_tools.rs, web_common.rs, html.rs | Studio probes | W0/W2 |
| 4 | Automatic capability discovery | js_trace.rs, js_app.rs, api_ops.rs | VLANG | W2 |
| 5 | Evidence-backed investigation | evidence lib.rs, canonical.rs, js_ledger.rs | HQKG/Studio | W1/W2 |
| 6 | Visual design-system extraction | website_tools.rs, html.rs | VEC/TierFlow | W0/W2 |
| 7 | Visual and behavioral comparison | compare_tools.rs, browser_tools.rs, js_compare.rs | Studio/VEC | W0/W2 |
| 8 | Automatically generated application wiki | investigation_tools.rs, report_tools.rs, unknown.rs | AutoWiki/HQKG | W1/W2 |
| 9 | Data-surface discovery | js_app.rs, web_har.rs, api_ops.rs | Studio/HQKG | W2/W3 |
| 10 | Integration blueprint generation | js_trace.rs, model.rs, tool.rs | Studio factory/VLANG | W2 |
| 11 | Cross-application semantic equivalence | js_compare.rs, fingerprint.rs, compare_tools.rs | VLANG | W2 |
| 12 | Software-change intelligence | js_compare.rs, fingerprint.rs, js_ledger.rs | Studio revisions | W2 |
| 13 | Adaptive workflow repair | js_compare.rs, fingerprint.rs, browser_tools.rs | Studio improvement | W2/W3 |
| 14 | Desktop and mobile application understanding | js_inventory.rs, js_projection.rs, asar.rs | Dev platform tools | W3 |
| 15 | File-format and schema intelligence | js_inventory.rs, asar.rs, sourcemap.rs, sqlite.rs | HQKG | W3 |
| 16 | Protocol behavior mapping | web_har.rs, api_ops.rs, web_runtime_tools.rs | Dev research | W3 |
| 17 | Local behavior stand-ins | standin.rs, standin_tools.rs | Dev testing | W3 |
| 18 | Accessibility and interaction intelligence | website_tools.rs, html.rs, window_tools.rs | Studio/TierFlow | W0/W2 |
| 19 | Investigation-to-skill compilation | evidence lib.rs, js_ledger.rs, mcp lib.rs | VLANG/Rust | W2 |
| 20 | Portable learned-capability packages | evidence lib.rs, registry.rs, mcp lib.rs | HQKG/Host | W2/W3 |

## Source-to-use-case principles
- One Craft source can support multiple cases; do not fork the same algorithm per feature.
- Missing evidence remains unknown, not absent. Observe the whole interface with explicitly bounded scope.
- Application data surfaces and protocol captures may be assessed only for permitted sources; not an invitation to extract third-party private APIs.
- Host alone owns effects/grants/receipts; Studio keeps proposals read-only; HQKG remains canonical graph.
