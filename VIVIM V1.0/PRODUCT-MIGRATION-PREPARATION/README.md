# Product migration preparation — P0 launchpad R001

Owner direction, 26 November 2026: SDK first. Six independent preparation sessions, all isolated in Labs. No actual Product donor integration until the self-executable SDK and external adoption acceptance (intel-200, intel-210) are independently demonstrated and the owner authorizes the scope. The staged 201–210 catalogue delta is not active yet; do not modify its active catalogue or the Product SDK.

## Start here

| Lane | Owner scope | Route |
|---|---|---|
| [P01-NATIVE-CORE](P01-NATIVE-CORE/MANIFEST.json) | Native core | Direct Product; no duplicate M01 |
| [P02-HQKG-GRAPH](P02-HQKG-GRAPH/MANIFEST.json) | HQKG/graph | Direct Product pure closure; operational adapters separate |
| [P03-VWIN-FILESYSTEM](P03-VWIN-FILESYSTEM/MANIFEST.json) | VWin filesystem intelligence | Direct Product for R031/R032/R044; ARBOR only for missing live OS/Host acceptance |
| [P04-VWIN-EXECUTION](P04-VWIN-EXECUTION/MANIFEST.json) | VWin resource execution | Direct Product for R039 SDK policy/domain; ARBOR for missing effect adapter proof |
| [P05-HOST-RECEIPTS](P05-HOST-RECEIPTS/MANIFEST.json) | Host/receipt recovery | ARBOR operational proof, then Product |
| [P06-MCP-GRAPH](P06-MCP-GRAPH/MANIFEST.json) | MCP graph integration | ARBOR live transport, Product pure SDK projection |

71 initial pinned entries (including repeated cross-lane references), all paths fetched and Git blob IDs verified by GitHub readback; not a dependency closure or a claim that sources compile together. Each manifest contains exact repository commit, path and blob. VWin baseline is R044, with prior evidence preserved. Re-pin at actual integration time, not by silently changing donor history.

## Common execution contract

Read the lane MANIFEST.json and ARBOR AGENTS.md; discover current VIVIM MCP grants and use it for authorized local testing. ChatGPT cloud owns source engineering; no local coding agent or Desktop Commander by default. Write only to your assigned lane in Labs, direct main commits with concurrent change preservation; leave ARBOR, Product, original VWin folder and other lanes unchanged. No synthetic live claims. Existing test fixtures may establish fixture behavior, never real runtime acceptance.

Materialize real donor bytes in a **new** scratch directory using [materialize.mjs](materialize.mjs), with a local JSON mapping named ARBOR, Labs, Product to exact local Git repo roots. Run from a shell with Node:
```
node materialize.mjs P01-NATIVE-CORE/MANIFEST.json NEW_SCRATCH_DIRECTORY REPOS.json
```
Repeat for the assigned lane only. The parent scratch directory must exist, outside all three repositories; no existing destination. Pin missing locally? Fetch exact Git objects through an authorized mechanism without checking out/resetting a shared worktree. If unavailable, report missing pin. The materializer verifies blob identity and bytes and produces MATERIALIZATION.json. It does not download, build, execute donor code, grant access, or make a production edit. It is preparation tooling using the incumbent Labs Node/Git convention, not a Product runtime dependency.

Each lane must return real working isolated packages, exact source/test dependency closure, source provenance, licensing disposition, real build/test evidence and an adoption plan. Add actual portable source/patches/tests only inside the lane if required for isolated experiments; reference unchanged donor code by exact pin rather than duplicating it. Do not copy private data or credentials into Git.

Required outputs:
- closure.json: exact source + original tests + dependency graph + destination conflicts + license status + known unavailable dependencies.
- adaptation.json: per-file deterministic transform candidates, semantic obligations for LLM, unsupported cases, SDK task references, destination map; no invented semantic claims.
- EVIDENCE.json: commands, exact pins/toolchain, exit status, result, receipt references, limitations; distinguish prior evidence and newly executed proof.
- HANDOFF.md: actual working entrypoint, current disposition, route, what remains before intel-210 and first migration package.
- Any isolated Rust implementation, test or exact-overlay specification needed to demonstrate readiness (not Product or ARBOR commits).

Source code first, documentation minimal. Existing M01 and VWin evidence must not be repeated unnecessarily. Match the SDK's current architecture and prefer existing implementations. No direct activation of capability bindings. Report missing authorization or unavailable toolchain; never bypass.

## Cross-lane interfaces

P01 owns pure core semantics; P02 owns graph identity and queries. P03 uses P02 graph contracts; P04 owns resource policy, P05 owns effect/owned process authority; P06 consumes P02/P05 interfaces and owns MCP transport/projection. Work independently on pinned snapshots; unresolved interface differences become explicit handoffs, never cross-lane writes. Shared master index is master-owned.

## VWin route decision

Direct Product candidates after SDK readiness: R031/R032 actual Product source/compiler/Wiki overlay (check current equivalent behavior first); R039 resource-aware SDK campaign/shared domain; R044 native file identity/HQ join where it uses Product semantics; pure bounded policy, graph, provenance and diagnostic primitives. Existing isolated evidence is already Product-targeted; an intermediate ARBOR implementation would impose redundant adaptation. No blind old overlay application.

ARBOR-first where live authority is still missing: operational Windows UIA/Craft, native window/session control, effectful terminal/ConPTY, installed MCP/Host projection and service lifecycle. Reuse existing Host grants/receipts and actual Windows tests; promote portable contracts only after operational proof. Do not reroute independently accepted portable primitives merely to satisfy repository order.

Hybrid: ETW R041 actual owned compiler trace may directly serve SDK measurements; Host-wide diagnostics need ARBOR operational integration. ACP R040 actual owned transport may be reused as source; live agent lifecycle through installed Host/MCP requires actual acceptance. R042 capability parity is an inert projection, not registration. R044 Windows Search still unavailable; native handle/digest proof is positive, Shell property on joined source unavailable. Do not advertise those as positive indexed search. Source acquisition/authorization and licensing remain pending lane obligations.

## Launch prompts

### P01-NATIVE-CORE
You are VIVIM Product migration preparation agent P01-NATIVE-CORE. Read https://github.com/owenservera/vivim-labs/blob/main/VIVIM%20V1.0/PRODUCT-MIGRATION-PREPARATION/P01-NATIVE-CORE/MANIFEST.json and https://github.com/owenservera/vivim-labs/blob/main/VIVIM%20V1.0/PRODUCT-MIGRATION-PREPARATION/README.md. Take ownership of Native core. Execute the manifest's isolated preparation work against actual pinned code using VIVIM MCP for authorized Windows verification. Build and test in isolation, preserve existing evidence, write only your lane in vivim-labs main, and return exact source/test pins, adaptation obligations, evidence and the minimal candidate package. Do not modify Product, ARBOR, original VWin, active SDK tasks or another lane. No migration or deployment before intel-200 and intel-210 are accepted and scope authorized. Begin immediately.

### P02-HQKG-GRAPH
You are VIVIM Product migration preparation agent P02-HQKG-GRAPH. Read https://github.com/owenservera/vivim-labs/blob/main/VIVIM%20V1.0/PRODUCT-MIGRATION-PREPARATION/P02-HQKG-GRAPH/MANIFEST.json and https://github.com/owenservera/vivim-labs/blob/main/VIVIM%20V1.0/PRODUCT-MIGRATION-PREPARATION/README.md. Take ownership of HQKG/graph. Execute the manifest's isolated preparation work against actual pinned code using VIVIM MCP for authorized Windows verification. Build and test in isolation, preserve existing evidence, write only your lane in vivim-labs main, and return exact source/test pins, adaptation obligations, evidence and the minimal candidate package. Do not modify Product, ARBOR, original VWin, active SDK tasks or another lane. No migration or deployment before intel-200 and intel-210 are accepted and scope authorized. Begin immediately.

### P03-VWIN-FILESYSTEM
You are VIVIM Product migration preparation agent P03-VWIN-FILESYSTEM. Read https://github.com/owenservera/vivim-labs/blob/main/VIVIM%20V1.0/PRODUCT-MIGRATION-PREPARATION/P03-VWIN-FILESYSTEM/MANIFEST.json and https://github.com/owenservera/vivim-labs/blob/main/VIVIM%20V1.0/PRODUCT-MIGRATION-PREPARATION/README.md. Take ownership of VWin filesystem intelligence. Execute the manifest's isolated preparation work against actual pinned code using VIVIM MCP for authorized Windows verification. Build and test in isolation, preserve existing evidence, write only your lane in vivim-labs main, and return exact source/test pins, adaptation obligations, evidence and the minimal candidate package. Do not modify Product, ARBOR, original VWin, active SDK tasks or another lane. No migration or deployment before intel-200 and intel-210 are accepted and scope authorized. Begin immediately.

### P04-VWIN-EXECUTION
You are VIVIM Product migration preparation agent P04-VWIN-EXECUTION. Read https://github.com/owenservera/vivim-labs/blob/main/VIVIM%20V1.0/PRODUCT-MIGRATION-PREPARATION/P04-VWIN-EXECUTION/MANIFEST.json and https://github.com/owenservera/vivim-labs/blob/main/VIVIM%20V1.0/PRODUCT-MIGRATION-PREPARATION/README.md. Take ownership of VWin resource execution. Execute the manifest's isolated preparation work against actual pinned code using VIVIM MCP for authorized Windows verification. Build and test in isolation, preserve existing evidence, write only your lane in vivim-labs main, and return exact source/test pins, adaptation obligations, evidence and the minimal candidate package. Do not modify Product, ARBOR, original VWin, active SDK tasks or another lane. No migration or deployment before intel-200 and intel-210 are accepted and scope authorized. Begin immediately.

### P05-HOST-RECEIPTS
You are VIVIM Product migration preparation agent P05-HOST-RECEIPTS. Read https://github.com/owenservera/vivim-labs/blob/main/VIVIM%20V1.0/PRODUCT-MIGRATION-PREPARATION/P05-HOST-RECEIPTS/MANIFEST.json and https://github.com/owenservera/vivim-labs/blob/main/VIVIM%20V1.0/PRODUCT-MIGRATION-PREPARATION/README.md. Take ownership of Host/receipt recovery. Execute the manifest's isolated preparation work against actual pinned code using VIVIM MCP for authorized Windows verification. Build and test in isolation, preserve existing evidence, write only your lane in vivim-labs main, and return exact source/test pins, adaptation obligations, evidence and the minimal candidate package. Do not modify Product, ARBOR, original VWin, active SDK tasks or another lane. No migration or deployment before intel-200 and intel-210 are accepted and scope authorized. Begin immediately.

### P06-MCP-GRAPH
You are VIVIM Product migration preparation agent P06-MCP-GRAPH. Read https://github.com/owenservera/vivim-labs/blob/main/VIVIM%20V1.0/PRODUCT-MIGRATION-PREPARATION/P06-MCP-GRAPH/MANIFEST.json and https://github.com/owenservera/vivim-labs/blob/main/VIVIM%20V1.0/PRODUCT-MIGRATION-PREPARATION/README.md. Take ownership of MCP graph integration. Execute the manifest's isolated preparation work against actual pinned code using VIVIM MCP for authorized Windows verification. Build and test in isolation, preserve existing evidence, write only your lane in vivim-labs main, and return exact source/test pins, adaptation obligations, evidence and the minimal candidate package. Do not modify Product, ARBOR, original VWin, active SDK tasks or another lane. No migration or deployment before intel-200 and intel-210 are accepted and scope authorized. Begin immediately.

