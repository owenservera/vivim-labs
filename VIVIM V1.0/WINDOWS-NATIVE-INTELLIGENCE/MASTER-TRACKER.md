# Windows Native Intelligence — master implementation queue R012

Scope: Labs-only implementation, no ARBOR source modifications or deployment. Owner decides incorporation. Shared Host catalog is eventual authority; all 33 public candidate descriptors have MCP/app projections, 0 new live acceptance. Existing permissions preserved.

| ID | Workstream | Current implementation | Next blocker |
|---|---|---|---|
| WNI01 | Filesystem intelligence | Windows watcher verified; USN V2/V3 and bounded journal tests pass | continuous supervised collector, authorized identity, HQ transaction |
| WNI02 | System diagnostics | native resource probe/Host fixture verified; hardware/disk tests pass; ETW producer tested | ETW consumer, disk/hardware projection, full Host/GPUI integration |
| WNI03 | Terminal and agents | native ConPTY attach fixture passes; ACP framing/router/transport tests pass | standard handles/input, active cancellation, OwnedProcess and actual agent transport |
| WNI04 | Search | typed search query tests pass | native Windows Search execution, property store |
| WNI05 | App semantics | pinned donor research | Studio/Harness owner integration, native UIA scope |
| WNI06 | App Actions | research, contract | platform probe and adapter |
| WNI07 | Capture | donor research | selected-window adapter in Studio/Harness |
| WNI08 | Configuration | research, contract | WinGet/DSC plan/readback adapter |
| WNI09 | Isolation | research, contract | optional backend support and scoped test |
| WNI10 | Local inference | research, contract | hardware-adaptive runtime probe |
| WNI11 | Capability compiler | 33 candidate mappings; 27 real schema checks | install actual Host/MCP/GPUI routes per adapter |

## Active engineering list

| Item | State |
|---|---|
| 01 ConPTY child attachment | explicit console fixture verified; normal stdio unresolved |
| 02 ACP lossless transport | verified |
| 03 ACP actual agent integration | pending |
| 04 ETW producer | verified emission only |
| 05 ETW consumer/correlation | pending |
| 06 disk/hardware native probes | tests pass; no public projection |
| 07 Windows Search execution | pending |
| 08 Property System | pending |
| 09 filesystem to AutoWiki | pending |
| 10 shared schemas | verified candidate compiler |
| 11 native resource Host port | real native fixture observed |
| 12 GPUI full compile/interaction | pending |
| 13 native app semantic discovery | donor research only |
| 14 Windows App Actions | pending |
| 15 graphics capture | pending |
| 16 configuration reconciliation | pending |
| 17 execution container support | pending |
| 18 local inference support | pending |

Evidence R008–R012. Latest native starter: 35 tests pass, outer job cancelled after output; worker: 6 tests pass, normal exit. 27 schema checks and 3 Host/MCP fixture tests, 2 extra resource probe tests previously observed. Do not sum overlapping tests as unique. No real production new capability invocation.
