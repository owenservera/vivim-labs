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

## R022–R023 — native UIA and optional platform probes

Selected hidden-window UIA root and process identity verified; zero child results, native runtime IDs and action readback pending. Graphics Capture IsSupported true; App Actions and legacy ML type metadata present; new ML catalog unknown. Both isolated native tests passed, normal exit. WNI08/09, ETW consumption, full interactive ConPTY, ACP worker and continuous collector remain pending. No ARBOR writes or deployment. [Evidence](EVIDENCE-R023-WINDOWS.json).

## R031–R032 — Priority 1 isolated integration acceptance

**Incremental filesystem and compiler intelligence: 100% of the bounded isolated integration acceptance.** [Exact overlay](integration-packs/WNI01-R031-PRODUCT-LIVE/PATCH-SPEC.json), [execution evidence](EVIDENCE-R031-PRODUCT.json). Real Windows Product/HQ/native SDK and graph binaries, 24 combined tests, automatic source edit refresh, source digest and graph epoch advance, Wiki readback, source focus proof, unchanged refresh retaining epoch 9, deletion advancing epoch to 12, exact missing-source contract, deleted Wiki article absent, surviving sibling query verified. Extended live fixture exited 0 in session ctl-e3daa64b0f8b9af0. Priority 1 acceptance does not mean production WNI01 or global watcher/USN coverage: full HQ refresh retained, notifications are hints, exporter projects the full universe, incumbent importer reconciles. No Product/ARBOR production modifications or deployment. Other workstream acceptance unchanged.

## R034 — SDK resource-aware campaign admission

11/11 native Rust tests passed, exact source blobs confirmed; five new resource campaign tests including two owned parallel Rust threads, admission ordering, lease expiry/cancel fencing, stale snapshot and large bounded queue. [Evidence](EVIDENCE-R034-SDK.json). Existing Product campaign exact overlay and Labs scheduler reused. No production changes; current Host process receipt integration, live native sampling, cross-campaign budget and actual agent workloads remain open. R035 native sampling/real compiler fixture authored. Original 14-item order unchanged.

## R035 — actual native samples and compiler workload

16/16 Windows tests passed, exit 0, eight materialized source blobs verified. Existing Win32 resource and topology probes drive the same R034 admission transition; two real rustc metadata jobs admitted and launched before waiting, both exit successfully and release all reservations. [Evidence](EVIDENCE-R035-SDK.json). Product main campaign blob unchanged at b3debb7. Single-group CPU scope, conservative configured resource estimates, no production Host receipt/cross-campaign binding or live policy activation yet. Next: reprogrammable SDK admission policy bounded by fixed resource constraints, then incumbent owned-process receipts. No tracker reordering or production writes.

## SDK-intensive R036–R038 — typed policy and native workload feedback

45/45 exact-pin native tests passed for live Meta-VLANG policy and bounded workload profile; [evidence](EVIDENCE-R036-R037-SDK.json). Reused the concurrently landed owned-admission package at 1ba61b4, applied exact-source overlay rather than duplicate process owner. R038 combined 58/58 Windows tests, normal exit 0, whole-job lifecycle and actual compiler feedback: 64 MiB initial reservation, measured 115101696-byte peak job commit, second job admitted at 143877120 bytes. [Evidence](EVIDENCE-R038-SDK.json). Unknown measurements retain prior known samples. Production Host/cross-campaign authority and actual agent protocol still open; original priority list unchanged.

## R036–R039 — Priority 2 bounded isolated integration acceptance

**Resource-aware parallel execution: 100% of the bounded single-campaign isolated acceptance.** Reused exact Product campaign/resident Meta-VLANG modules and incumbent ARBOR Windows OwnedProcess/Job Objects. [Combined package](integration-packs/WNI02-R036-OWNED-ADMISSION/PIN.json), [R036 ownership evidence](EVIDENCE-R036-OWNED-ADMISSION.json), [R038 live policy evidence](EVIDENCE-R038-OWNED-POLICY.json), [R039 final evidence](EVIDENCE-R039-OWNED-ADMISSION.json). R036 28/28, R038 58/58, R039 59/59 actual Windows tests, all exit 0. Two real rustc metadata children executed through resident policy and owned-job adapter, verified artifacts and zero reservations after exit. Separate live two-job fixture verifies third deferral, exact lease cancellation and no premature release; live policy tightens admission without dropping holds and rejects widening. 13 reused source blobs verified in R038. Concurrent R038-SDK evidence separately covers native workload feedback; do not conflate packages or add overlapping test counts. Cross-campaign durable Host receipts and quota enforcement remain distinct production integration work. No ARBOR/Product writes or deployment by this workstream. Original priority list unchanged; next Priority 3 ACP actual owned agent integration.

## R039 SDK shared admission domain — authored

[Exact source candidate](integration-packs/WNI02-R034-SDK-ADMISSION/resource-domain.rs) extends existing resource admission and owned campaign wrapper with global held-resource accounting, resident policy inputs, worker uniqueness, sample fencing and domain-bound lease identity. Three native tests authored. [Verification status](EVIDENCE-R039-SDK.json): workstation launch receipt unavailable; scratch readback did not show R039 materialization; no new-ID retry of the uncertain launch. The verified combined baseline remains R038 58/58. No production writes or acceptance claim.

## R039 shared domain + R040 ACP

R039 shared resource domain verified: 61/61 Windows Rust tests, 19 exact source blobs, normal exit 0; global cap, actual owned jobs, global worker uniqueness, sample fencing, foreign lease denial and uncertain-launch retention across campaigns. [Evidence](EVIDENCE-R039-SDK-DOMAIN.json). R040 existing ACP transport connected to incumbent OwnedProcess; deterministic owned stdio session/prompt/update/permission denial/EOF passed. First actual-agent test failed due orphaned Node launcher, alternate native OpenCode 2.2.7 initialize verified with clean job cancellation. [Evidence](EVIDENCE-R040-ACP.json). Actual agent session/new extension authored; actual provider prompt, production binding, and full ACP acceptance pending. No ARBOR/Product changes.
