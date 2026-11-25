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
