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

## R015 sprint update

| Item | Latest evidence | Remaining |
|---|---|---|
| WNI01 HQ observation transaction | 4/4 actual SQLite fixture tests; exact indexer transaction hook | Continuous collector coverage and actual Wiki readback |
| WNI02 resource scheduler | Deterministic resource admission and budget tests | Host atomic reservation and public projection |
| WNI02 ETW | Closed numeric decoder, generation/source correlation and actual Windows EVENT_RECORD struct fixture | Native trace session controller and real consumer |
| WNI04 file metadata | Native owned-file handle metadata/identity/size test | Full Property System and Windows Search execution |
| WNI11 combined overlay | R015: 12 exact Git blobs, 7 additions, 7/7 Node tests, normal exit | Reconcile current source, full crate/GPUI and live MCP |

Native starter 45/45 test output (includes prior 35); both R013 and R014 enclosing jobs needed owned-job cancellation after output closed. No production acceptance claim. No new mandatory dependency or ARBOR writes. Evidence: [R015](EVIDENCE-R015-WINDOWS.json).

## R017/R018 — HCU and UFO extraction

WNI05: scoped semantic reference lifecycle, complete snapshot delta and acknowledgment, incumbent receipt recovery projection, bounded condition predicates and hybrid route selection authored. No Windows UIA collector, app integration or public MCP action yet. WNI07 and WNI06 remain research-only. R015 combined overlay remains the tested integration candidate; new modules are independently staged. ARBOR unchanged by this workstream.

## R019 verification

56/56 native test output (11 new HCU-inspired tests over R015), actual Windows via VIVIM. Owned test jobs required cleanup after output closed. [Evidence](EVIDENCE-R019-WINDOWS.json); [incumbent source handoff](research/R019-STUDIO-HARNESS-HANDOFF.md). WNI05 state/ref/diff/recovery/route primitives verified; actual native UIA and production integration pending. All 33 original public candidates still not live. No ARBOR writes.

## R021 — Windows Search and property bridge

WNI04: real selected-file Shell property size verification, seven scoped adapter checks and five new Rust result-validation/fallback tests. SystemIndex provider connects but execution returns 0x80040154; index unavailable, not empty/complete. Keep existing HQ fallback; no index/service changes. Native Rust OLE DB implementation and live Host/MCP/GPUI integration pending. Latest native suite 61/61, normal exit 0. [Evidence](EVIDENCE-R021-WINDOWS.json).
