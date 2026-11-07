# REPOS-HQ — Deep Read-Only Source Audit and SITREP

**Audit date:** 2026-11-04 (Europe/Madrid)  
**Repository:** [owenservera/repos-hq](https://github.com/owenservera/repos-hq), private, default branch `main`  
**Observed head:** [`776a02a00f973fca749135a36cdd28a68222a481`](https://github.com/owenservera/repos-hq/commit/776a02a00f973fca749135a36cdd28a68222a481), 2026-11-04 01:36:35 UTC  
**Method:** authenticated GitHub metadata and commit history; targeted full-file source retrieval and archived contract inspection. **Not** a full recursive tree enumeration, local Windows checkout audit or runtime test execution.  
**Changes during the audit:** none. **This report's publication is a separately authorized documentation commit.**

> **Evidence language:** SOURCE-CONFIRMED = actual file/code inspected; RECORDED = commit/document/test source says so, not freshly rerun; PLANNED = specified but unimplemented; UNKNOWN = not established by available evidence. A working communication session reported by the owner proves a useful exercised path; it does not imply that all providers, retries, attachments, approvals and long-running sessions have passed acceptance.

## ERA 1 — Strategic architecture and current position

### Executive assessment

Repos HQ is a **working local intelligence/tooling foundation with significant integration work left**, not merely an aspirational architecture. Its existing parts are real: CLI, SQLite+FTS5 indexing, code dependency graph, deterministic context bundles, capability inventory, knowledge-package intake, agent communication bus, channel seat transport, durable kernel-store module, local MCP, release tooling, and tests. They are **not** yet proven as one coherent production-grade product. The greatest risk is building parallel mechanisms without joining them to a single, provenance-rich, end-to-end event and data flow.

The current priority in [`AGENTS.md`](../../AGENTS.md) is **DATA VOLUME**: increase the depth and breadth of real feeds first, verify their provenance and redaction next, turn them into useful results third, and avoid features without grounded data. Telemetry is supporting work, not a blocker.

### Scope and authority

| Repository / component | Role and intended authority | Status |
|---|---|---|
| `repos-hq` | Canonical shared KB, semantic registry, context, reusable agent coordination and architecture | Source-confirmed components; integration incomplete |
| `vivim-web-harness` | Product/Host runtime, Studio's signed-in browser boundary, mobile and device experience, effect receipts | External sibling; **not source-audited** here |
| `hq` | Earlier code and research, migration input | External sibling; no fresh runtime verification |
| `tierflow` / `vivim-ui` | Universal execution infrastructure and experimental live UI/state patterns | Integration planned; no current E2E audit |
| SOFTWARE_HQ / ZCode / OpenCode | Current developer-agent tooling and seats | Useful present development adapter; not desired final product dependency |

The kernel is **knowledge and semantic control**, not a second browser owner, OS, provider sender or universal execution-permission issuer. Native source authorities retain their own observations and effect boundaries. The Host owns user/device effects; Studio owns the signed-in browser/send journal; the KB writer owns curated knowledge revisions. An incoming document never grants authority. See [Shared KB architecture](../SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md).

### Verified architecture shape

- Entry points: [`hq/bin/hq.mjs`](../../hq/bin/hq.mjs), [`hq/lib/mcp.mjs`](../../hq/lib/mcp.mjs), ZCode plugin manifest under [`plugins/vivim-dev-tools/`](../../plugins/vivim-dev-tools/).
- Rebuildable intelligence projection: [`hq/lib/schema.mjs`](../../hq/lib/schema.mjs), [`indexer.mjs`](../../hq/lib/indexer.mjs), [`search.mjs`](../../hq/lib/search.mjs), [`graph.mjs`](../../hq/lib/graph.mjs), [`context.mjs`](../../hq/lib/context.mjs).
- Durable authored domain: [`hq/lib/kernel/store.mjs`](../../hq/lib/kernel/store.mjs), with append-only revisions, locks, provenance, receipts and SQLite-native backup/restore design.
- Data entry: [`hq/lib/kb-intake.mjs`](../../hq/lib/kb-intake.mjs), [`hq/lib/capabilities.mjs`](../../hq/lib/capabilities.mjs), session tools and repository index.
- Coordination and evidence: [`scripts/sync.mjs`](../../scripts/sync.mjs), [`hq/lib/channel-seats.mjs`](../../hq/lib/channel-seats.mjs), [`docs/AGENT-BUS.md`](../AGENT-BUS.md), plus channel evidence tests.
- Quality and release: [`scripts/quality-gate.mjs`](../../scripts/quality-gate.mjs), [`hq/lib/release-status.mjs`](../../hq/lib/release-status.mjs), build verification and plugin commands.

**Distinction:** `hq/lib/graph.mjs` is the indexed source-import/dependency graph; it is not itself a fully general Rust knowledge/computation graph. The Rust research is separately authorized, while replacing the Bun runtime remains gated by G1–G6 in [`docs/RUST-WORKSTREAM.md`](../RUST-WORKSTREAM.md).

## ERA 2 — Actual source and delivery register

### GitHub commits observed

| Commit | Recorded change | What it establishes |
|---|---|---|
| [`531f669`](https://github.com/owenservera/repos-hq/commit/531f669a176c81ec8acc0b56c0ffa4263e5cc80a) | SQLite topology and gaps | Architecture documentation |
| [`583f049`](https://github.com/owenservera/repos-hq/commit/583f04985371f5e5051da5ca72ee6c45c6d041bb) | Quality gate, release automation, scoped checks | Code commit, not current pass |
| [`f53be2f`](https://github.com/owenservera/repos-hq/commit/f53be2f9511243221cad13e86afe0bd7b63a3cf7) | F0 integration scaffold frozen | Recorded historical contract baseline |
| [`e8962df`](https://github.com/owenservera/repos-hq/commit/e8962dff7204231d6ba632e94a9ca309e78ac441) | Durable kernel store; D1 green | Recorded acceptance, not fresh rerun |
| [`2503c4d`](https://github.com/owenservera/repos-hq/commit/2503c4d55a15111337d9eaa8f8e54f787ad14342) | Wave 1: D1 passed, P1 owner-pending, U1 open | Historical verdict |
| [`1409729`](https://github.com/owenservera/repos-hq/commit/140972912d98d75f44b647f687b54321471259d7) | Seat wake, transport and evidence tests | Code/test additions |
| [`7b02f27`](https://github.com/owenservera/repos-hq/commit/7b02f27a11d566833595c131403415079daa684b) | Release plan/status read model and MCP exposure | Code additions |
| [`fa1f191`](https://github.com/owenservera/repos-hq/commit/fa1f1911b0136384291cc438e78ec4944db84fed) | Channel/strategy registers; archives F0 | Historical scaffold relocated |
| [`9485848`](https://github.com/owenservera/repos-hq/commit/9485848cf94696075889a492cb7be83e599d3055) | VIVIM UI registry and code-ratio guard | Integration metadata |
| [`c75ce93`](https://github.com/owenservera/repos-hq/commit/c75ce934f9c7a03212f5c5cadf80d7f607949130) | 62 bus records; digest-chain verification | Recorded ledger/code change |
| [`3ccbf3c`](https://github.com/owenservera/repos-hq/commit/3ccbf3c92d07331414d8f90dbf4bbd758d28f953) | HQ knowledge-graph import and release transcript | Migration/research artifact |
| [`776a02a`](https://github.com/owenservera/repos-hq/commit/776a02a00f973fca749135a36cdd28a68222a481) | `vivim-dev-tools` plugin | Plugin committed; live loading unverified |

### File-level readiness matrix

| File or surface | Source observation | Verdict / missing evidence |
|---|---|---|
| [`hq/bin/hq.mjs`](../../hq/bin/hq.mjs) | Extensive CLI: status, doctor, index, context, KB, memory, sessions, seat, capabilities | SOURCE-CONFIRMED; command E2E untested |
| [`hq/lib/schema.mjs`](../../hq/lib/schema.mjs) | SQLite+FTS5 schema and transaction helpers; disposable `.hq` index | SOURCE-CONFIRMED |
| [`hq/lib/indexer.mjs`](../../hq/lib/indexer.mjs) | Incremental project indexing, import resolution | SOURCE-CONFIRMED |
| [`hq/lib/graph.mjs`](../../hq/lib/graph.mjs) | Resolved-import dependency/impact/project graph | SOURCE-CONFIRMED; not general knowledge graph |
| [`hq/lib/context.mjs`](../../hq/lib/context.mjs) | Deterministic bounded composer and bundle verifier | SOURCE-CONFIRMED; retrieval accuracy unevaluated |
| [`hq/lib/kb-intake.mjs`](../../hq/lib/kb-intake.mjs) | Package classification, provenance, refusal, transactional intake | SOURCE-CONFIRMED; nine-section model not implemented |
| [`hq/lib/semantic-registry.mjs`](../../hq/lib/semantic-registry.mjs) | Versioned semantic IDs, aliases, lifecycle, source anchors | SOURCE-CONFIRMED; durable survival incomplete/unproven |
| [`hq/lib/capabilities.mjs`](../../hq/lib/capabilities.mjs) | Static dependency scan, taxonomies, mappings, exports | SOURCE-CONFIRMED; promotion to safe effects unproven |
| [`hq/lib/kernel/store.mjs`](../../hq/lib/kernel/store.mjs) | Durable revisions outside rebuildable `.hq`, single-writer design | SOURCE-CONFIRMED; D1 acceptance RECORDED |
| [`hq/lib/mcp.mjs`](../../hq/lib/mcp.mjs) | Read-only JSON-RPC stdio server, versioned protocol | SOURCE-CONFIRMED; current clients untested |
| [`scripts/sync.mjs`](../../scripts/sync.mjs) | `hq.sync-bus/2`, append-only ledger, cursor/ack, claims, receipts, authority limits | SOURCE-CONFIRMED; concurrent live tests needed |
| [`hq/lib/channel-seats.mjs`](../../hq/lib/channel-seats.mjs) | Seat roster, guards, wake intent before effect, result journal | SOURCE-CONFIRMED; long-wait reconciliation defect recorded |
| [`scripts/git-main-sync.mjs`](../../scripts/git-main-sync.mjs) | Main-only sync and edit-age blocking; `git add -A` at line 212 | SOURCE-CONFIRMED; competing-lane commit hazard |
| [`scripts/quality-gate.mjs`](../../scripts/quality-gate.mjs) | Quick/local/CI/release profiles: runtime, workflows, types, tests, scoped indexing/lint, bundle, clean-tree | SOURCE-CONFIRMED; no fresh run |
| [`hq/lib/release-status.mjs`](../../hq/lib/release-status.mjs) | Read-only release and gate status | SOURCE-CONFIRMED |
| [`hq/test/kernel-store.test.mjs`](../../hq/test/kernel-store.test.mjs) | Durable-store test source including reset survival | TEST PRESENT; not rerun |
| [`hq/test/kb-intake.test.mjs`](../../hq/test/kb-intake.test.mjs) | Intake fixtures and schema contract tests | TEST PRESENT; not rerun |
| [`hq/test/sync-bus.test.mjs`](../../hq/test/sync-bus.test.mjs) | Legacy bus behavior tests | TEST PRESENT; not rerun |
| [`hq/test/channel-evidence.test.mjs`](../../hq/test/channel-evidence.test.mjs) | Synthetic and live evidence audit | TEST PRESENT; not rerun |
| [`plugins/vivim-dev-tools/.zcode-plugin/plugin.json`](../../plugins/vivim-dev-tools/.zcode-plugin/plugin.json) | `/bootstrap`, `/gate`, `/release` local operator tools | COMMITTED; bootstrap path inconsistent |
| `hq/lib/model-intake.mjs` and `hq/test/model-intake.test.mjs` | Target of archived L2 plan | NOT FOUND at queried paths |
| `hq/lib/connectors/**` / Host connector | Target of archived L4 plan | NOT VERIFIED |
| `docs/VIVIM-INTEGRATION-ROADMAP.md` / `docs/integration/lanes.v1.json` / `scripts/integration/check.mjs` | Active plugin still refers to original locations | NOT FOUND at queried paths |
| [`docs/integration/archive/VIVIM-INTEGRATION-ROADMAP.md`](../integration/archive/VIVIM-INTEGRATION-ROADMAP.md) / [`archive/lanes.v1.json`](../integration/archive/lanes.v1.json) / [`archive/check.mjs`](../integration/archive/check.mjs) | Historical F0 plan, lanes and checker | PRESENT, explicitly archived; not automatically the current execution plan |

### Archived L0–L7 lane register (historical, **not live**)

| Lane | Recorded status | Responsibility and acceptance |
|---|---|---|
| L0 | done | Freeze scaffold/integration contract |
| L1 | done | Durable kernel store, D1 reset-survival acceptance recorded |
| L2 | blocked | Nine-section model intake; missing model fixture and implementation |
| L3 | blocked | Corpus parity and migration receipts |
| L4 | blocked | Read-only Host connector |
| L5 | awaiting_owner | Prototype acceptance receipts |
| L6 | done | Experimental UI timing assessment |
| L7 | blocked | Conditional UI-core adoption; dependent on L2/L3/L4/L6 |

Reference: [archived lane register](../integration/archive/lanes.v1.json). Since F0 was deliberately archived, these are **not current operational assignments** to reanimate automatically.

### Channel capabilities — scope of the number

[`docs/integration/CHANNEL-CAPABILITIES.md`](../integration/CHANNEL-CAPABILITIES.md) records 100 channel capabilities in C1-frame form, and a last-written status of **78 N, 22 P, 0 Y**. This is the **channel inventory** only, not a whole-repository completion percentage. Its states are author-asserted today; a future V2 should **derive** them from valid receipts and tested predicates.

## ERA 3 — Risk register, gaps, verification and next action

### Source-confirmed or recorded concerns (ranked)

| Rank | Finding | Source / rationale | Correct next action |
|---|---|---|---|
| P0 | **Broken/stale bootstrap reference** | F0 checker moved into `docs/integration/archive/check.mjs` in `fa1f191`, while newer plugin requests `scripts/integration/check.mjs` | Choose active checker ownership and route deliberately; do not silently revive archived plan |
| P0 | **Model ingestion false success** | [`docs/VIVIM-KB-INTAKE.md`](../VIVIM-KB-INTAKE.md) specifies nine model sections; historic GAP-004/005 records missing model branch and ordinary-doc ingestion fallback | Fail closed on unsupported `model/`; preserve provenance; test true nine-section ingestion |
| P0 | **Whole-tree auto-commit with concurrent lanes** | [`scripts/git-main-sync.mjs`](../../scripts/git-main-sync.mjs), `git add -A` line 212, 60-second default edit-age window | Prove path-scoped staging under simultaneous writers; checkpoint != acceptance |
| P0 | **Authored semantic state may be deleted by reset** | [`hq/lib/schema.mjs`](../../hq/lib/schema.mjs) stores semantic tables in rebuildable index; kernel migration remains a separate task | Migrate authored IDs/aliases/programs to durable home, preserve hashes and re-test reset |
| P1 | **Wake-result incompleteness** | [`docs/AGENT-BUS.md`](../AGENT-BUS.md) records `WAKE_PENDING` with no later backfill; stopped subagent queues; ACP resume failure | Stable wake ID, completion scanner, terminal receipt, time-bounded retry only where safe |
| P1 | **KB source/target parity unknown** | Historical GAP-008 reports 17-vs-25 corpus counts without item-level comparison | Compare canonical item IDs/hashes, not counts |
| P1 | **UI revision authority conflict** | Historical GAP-007 notes Host CAS revisions vs TierFlow SQLite revision table | Select sole authority per state domain and validate stale-write refusal |
| P1 | **Host read-only connector absent** | Historical L4 and strategic initiative SI-04 | Read exact ledger through read-only connector; test refusal of all writes/effects |
| P1 | **Context selection unmeasured** | [`context.mjs`](../../hq/lib/context.mjs) exists; strategic SI-05 notes no retrieval eval | Gold-set recall, grounding, omission, latency and cost comparisons |
| P2 | **Rust authority change not accepted** | [`docs/RUST-WORKSTREAM.md`](../RUST-WORKSTREAM.md) keeps G1–G6 before replacement | Permit isolated research/adapters only; demand parity, rollback and measured benefit |
| P2 | **C1 capability truth not derived** | [`CHANNEL-CAPABILITIES.frames`](../integration/CHANNEL-CAPABILITIES.frames) | Each state recomputed from verified receipts rather than copied claims |

### Quality and operational tests: **not executed here**

The source defines `bun scripts/quality-gate.mjs --profile local --json` with required runtime/workflows/typecheck/index-build/tests/lint/bundle steps and advisory wiki freshness. A fresh pass is necessary before any release claim. The source of `hq/test/kernel-store.test.mjs` includes a reset-survival case that explicitly touches its default kernel directory before cleanup: run tests only in an isolated/disposable checkout, not casually in a running workspace. The plugin's suggested `--no-write` flags and archived F0 checker path require confirmation against current implementations before use.

Suggested verification ladder in a disposable checkout:
1. Inventory exact repository tree and compare with all linked paths (GitHub source search and root-listing were not available in the audit).
2. Repair the bootstrap-path mismatch by a separately approved source change; ensure archived F0 remains historical.
3. Run scoped quality gate and record actual step-level results, no proxy-green claims.
4. Run durable knowledge/reset tests, including semantic IDs, aliases and wiki program versions.
5. Test two concurrent agent lanes: path-scoped staging and no half-finished unaccepted tree committed.
6. Implement/test model-intake and corpus-parity contracts; refuse unrecognized model data.
7. Implement a read-only Host connector plus negative authorization tests.
8. Prove one human prompt → journal → provider result → normalized event → kernel revision → context/UI projection → receipt, with restart and replay.
9. Test agent wake pending→terminal reconciliation and confirm delivery vs wake vs provider send are distinct.
10. Benchmark real retrieval and UI loops before introducing a replacement engine or Rust runtime.

### Evidence limitations and outstanding uncertainties

- No full recursive GitHub tree, total file/line count, complete coverage matrix, fresh local build, fresh Bun test output, local Windows working-tree status, production telemetry or release artifact verification.
- No source audit of sibling repos in this read-only pass; cross-repository statements are architecture direction or historical registered findings.
- Archived lane states and historical source anchors may be stale; do not treat them as current red/green.
- A real user-observed ZCode↔ChatGPT/file communication path is a **major product demonstration**, but must retain separate receipt proof for each turn, file, attachment, provider, approval and replay guarantee.
- The new forward architecture is deliberately separated into [**Compounding VIVIM Architecture**](../architecture/2026-11-04-COMPOUNDING-VIVIM-ARCHITECTURE.md), which describes how every development activity can become product intelligence and progressively replace the current developer-tool dependency.

### Verdict

**Infrastructure: real; unified intelligence product: not yet verified.** The next value-producing slice is **user intent → send-once channel receipt → normalized provenance-bearing observation → durable knowledge revision → deterministic context retrieval → approved change in a real VIVIM-owned app → observable UI and rollback.** This sequence produces data and user value while building the future product itself.

**No original source was modified to conduct the audit.** Publishing this document is an explicit, additive report save.
