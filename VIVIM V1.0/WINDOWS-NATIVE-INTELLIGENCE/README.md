# Windows Native Intelligence — extraction pack R001

Owner direction: Windows-first, native Rust, minimum dependencies, one existing Host capability layer with full MCP coverage and equal native GPUI/mobile interaction. No new mandatory services, no second renderer, no second browser controller, no duplicate registry or graph. This Labs pack is public-safe candidate source and integration evidence, not a live product or competing roadmap.

Source baseline: [ARBOR 972dbcc](https://github.com/owenservera/ARBOR/commit/972dbccafea7b9fde47913207c2fefcf02ac8365). Donor source pins: [Intelligent Terminal 7ef2e63](https://github.com/microsoft/intelligent-terminal/commit/7ef2e637b810d58f20ef53f61275a0ec849cf26f), [WinApp CLI c9e6a9b](https://github.com/microsoft/winappCli/commit/c9e6a9bdf010add8edaddb790689e3cbaae33cd5). Microsoft's source and docs inform independent implementation; no upstream code vendored or upstream build executed. Verify source license before copying donor code.

- [Capability candidates](CAPABILITY-CANDIDATES.json): 33 proposed public actions; every action has a unique MCP name and all user surfaces. Proposals are not grants.
- [Integration contract](INTEGRATION.md): exact incumbent files, code placement and MCP-first real acceptance.
- [Rust starter](rust-starter/Cargo.toml): aggregate Windows resource APIs, bounded directory-change record decoding, receipt-fenced agent session state. Native watcher, ETW, ACP, UIA and other OS adapters still need implementation. No build claim.
- [Coverage prototype](coverage.mjs), [tests](coverage.test.mjs): derive MCP/GPUI/VLANG candidate metadata from one descriptor set; fail closed on missing wiring, authorization and evidence.
- [Acceptance matrix](ACCEPTANCE.md): actual MCP then user app interaction, negative cases, dependency/performance budgets.
- [Sources](SOURCES.md): pinned research and runtime constraints.

| Package | Priority | Deliverable | Status |
|---|---|---|---|
| [WNI01 filesystem-delta](packages/WNI01-filesystem-delta.md) | P0 | src/change.rs (parser and revision reducer; OS reader pending) | Candidate |
| [WNI02 system-diagnostics](packages/WNI02-system-diagnostics.md) | P0 | src/resources.rs (native aggregate CPU/memory and selected fixed-drive capacity); ETW adapter pending | Candidate |
| [WNI03 native-terminal-agents](packages/WNI03-native-terminal-agents.md) | P0 | src/session.rs (receipt-fenced lifecycle reducer); ConPTY and ACP transport pending | Candidate |
| [WNI04 indexed-search](packages/WNI04-indexed-search.md) | P0 | adapter recipe only | Candidate |
| [WNI05 native-app-semantics](packages/WNI05-native-app-semantics.md) | P0 | donor extraction recipe only; R033 hold on integration | Candidate |
| [WNI06 app-actions](packages/WNI06-app-actions.md) | P0 | adapter recipe only; SDK availability probe required | Candidate |
| [WNI07 windows-capture](packages/WNI07-windows-capture.md) | P1 | donor extraction recipe only | Candidate |
| [WNI08 configuration-reconciliation](packages/WNI08-configuration-reconciliation.md) | P1 | adapter recipe only | Candidate |
| [WNI09 workload-isolation](packages/WNI09-workload-isolation.md) | P1 | optional backend recipe only | Candidate |
| [WNI10 hardware-adaptive-ai](packages/WNI10-hardware-adaptive-ai.md) | P2 | adapter recipe only | Candidate |
| [WNI11 binding-minimization](packages/WNI11-binding-minimization.md) | P0 | Cargo.toml feature list + dependency budget | Candidate |

First slice: WNI02 aggregate resources through existing Host to MCP and GPUI; WNI01 approved directory deltas into existing HQ/AutoWiki; WNI03 existing worker/ACP lifecycle. All other recipes remain explicitly proposed. Studio/Harness owns WNI05/06/07 integration; no duplicated UI stack. Machine-wide terminal authorization is separate and is not changed by this pack. Existing Settings project is the config owner. Live tunnel reported maintenance/drained, so Windows build, real MCP invocation and app interaction are UNRUN.

## R002 — exact-source integration overlay

[WNI02 native resources vertical slice](integration-packs/WNI02-R002/README.md): pinned ARBOR 67859e96c12a55d4341d4008cf8a5b1528d18eea, 9 exact-blob edits and 4 added source/test files; reuses native probe, Host, MCP, staging and GPUI control client. Isolated overlay only. 11/11 cloud checks passed; Windows compile, Node fixture tests, actual app control and live MCP remain unverified. Workstream remains Labs-only until owner decides integration.

## R003 — real native watcher, bounded observation journal and app control

[WNI01 native watcher and journal](packages/WNI01-R003-NATIVE-WATCHER.md) adds executable Rust source and tests; production handle-binding/continuous-stream adapter pending. [WNI02 R003 exact-source overlay](integration-packs/WNI02-R003/README.md) supersedes R002 and adds a native GPUI Developer control. Native compile and live acceptance unrun. No ARBOR edits.

## R004 — ACP connection core and observation correctness

[WNI03 ACP routing implementation](packages/WNI03-R004-ACP-ROUTER.md) adds bounded protocol framing, v1 initialization, session ownership/routing, prompt correlation, cancellation and default-denied delegated permission handling. Watcher handle validation and journal gap fencing strengthened. Rust compile/OS tests remain unrun; no ARBOR modification or provider invocation.

## R005 — USN record decoding and exact overlay validation

[USN decoder](packages/WNI01-R005-USN-DECODER.md) handles V2/V3 records without collecting filenames or opening a volume. [Overlay verifier](overlay-verifier.mjs) exercised all 14 WNI02-R003 source outputs against pinned GitHub base blobs; see [evidence](EVIDENCE-R005.json) for ten cloud checks. Source replay is not compile or live acceptance. ARBOR unchanged.

## R006 — ConPTY, stream accounting, ETW and indexed search source

[ConPTY and stream transport](packages/WNI03-R006-CONPTY-STREAM.md), [ETW producer](packages/WNI02-R006-ETW-PRODUCER.md), [typed indexed-search query](packages/WNI04-R006-SEARCH-QUERY.md). Four new Rust modules and fixture tests; no process attach, ETW consumer or OLE DB execution yet. Compiler absent from current cloud runtime; tests unrun. No ARBOR writes.

## R007 — shared capability compiler

[Compiler package](packages/WNI11-R007-CAPABILITY-COMPILER.md) generates 33 current-Host-compatible candidate schemas and MCP/app projections with 19/19 cloud checks passed. All candidates remain non-executable pending real adapters and grants; no ARBOR edits.

## R008/R009 — first Windows-native verification and owned ACP transport

[Windows test evidence](EVIDENCE-R008-WINDOWS.json): 28 native Rust tests passed on the actual Windows workstation via existing VIVIM MCP; 27 actual Node checks passed in the preceding test. ETW type corrected to REGHANDLE. [ACP transport integration](packages/WNI03-R009-ACP-TRANSPORT.md) combines bounded framing, router and stream accounting. New R009 tests pending. No ARBOR or production changes.

### R009 acceptance update

[Windows evidence](EVIDENCE-R009-WINDOWS.json): 33/33 native Rust tests, 3/3 actual overlay Node fixture tests; earlier 27/27 actual schema tests. [Exact Git blob materializer](materialize-base.mjs) preserves strict source pinning when archive/checkout changes text bytes. No deployment or ARBOR changes.

## R010 — real native probe to Host adapter

[Actual Windows adapter evidence](EVIDENCE-R010-NATIVE-ADAPTER.json): exact overlay native module compiled in scratch, 2/2 native probe tests and valid real Host-port observation. The containing test job required owned-job cancellation after output closed; no overall successful job claim. Full application/live new MCP acceptance pending. ARBOR unchanged.
