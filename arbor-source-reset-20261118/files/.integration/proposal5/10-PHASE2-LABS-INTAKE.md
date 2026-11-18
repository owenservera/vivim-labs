# Proposal 5 — Phase 2: Labs Intake Without Delaying Beta

**Status:** source-informed planning and runnable ChatGPT GitHub prompts, **not execution evidence**.  
**Reviewed:** 2026-11-12. Labs main `022608f5797e7b4a14ccc4c393bd3c5a4704a4be`; `lab/user-io-rust-core-20261112` `61d9fc15c55144c4f0b79afa248981c2b0ca9bd6`; `lab/nlp-visual-command-ui-20261111` `56bb145ea1eb479e4d8ec629163a84322797367a`. Refresh on execution.

## Objective

Evaluate and prepare **all worthwhile Labs contributions** for ARBOR's first self-contained beta, with a hard separation between **easy migration**, **core development/migration**, and **high-value pending development**. Run preflight/intake **concurrently with current W1 lanes A–F**, without touching any working branch or overloading integration G. Only production-worthy, already-source-backed code should enter the actual beta runtime. Proposed features stay as acceptance contracts and a post-beta backlog.

**No outside wiring:** after graduation selected code/tests are physically in ARBOR. No import from a Labs checkout, no Labs runtime process or service.

## Three categories and preliminary source assessment

### E — Easy migration / low-risk source preservation

| ID | Labs source evidence | What is genuinely present | First action / proposed owner | First-beta policy |
| --- | --- | --- | --- | --- |
| E01 | `capability-observatory/lane-d/`: `src/observatory.mjs`, `scripts/refresh.mjs`, `tests/observatory.test.mjs` | Standalone read-only observatory, content-addressed snapshots, synthetic seed and test suite; not authoritative | F-owned QA/capability projection; **reuse test/schema/report logic**, avoid second live server | Pure observability package/test, optional local inspector |
| E02 | `vivim/extension-points/`, `vivim/PLUGIN-SYSTEM-ARCHITECTURE.md` | Candidate hook maps plus verified-hook investigation; broad candidates are **not installed plugins** | F-owned evidence/contract crosswalk | Import only verified existing hook metadata; no plugin manager |
| E03 | `nlp-command-ui-lab/src/semantic-lens.mjs`, `bridge.mjs`, `affordances.mjs`, R005 | JS prototype for source-derived semantics, negation, quote handling and inert commands; synthetic tests reported, no device proof | E/F review and **tested pure functions/fixtures** only | Optional read-only projection, no new command parser |
| E04 | `chronos-toolkit/wave2` TS projection + pure JS `chronos-conversation-projection.mjs` + tests | Staged pure read projection; not integrated; `chronos.rs` belongs to native/core lane | E/C/D receive evidence, **stage tests only** unless nonconflicting owner consent | No second scheduler, timer, or Host effect |
| E05 | `Architecture/10-THREE-WAVE-RUST-LIVE-GRAPH/wave-1/lane-d/` | Source-grounded wiki/provenance/privacy tests and read-only view code | D/F tests and wiki quality fixtures | Can be reused only if compatible with internal HQ and redacted |
| E06 | `intelligence/algorithms/`, capability-observatory source taxonomy | Research briefs / algorithms; not verified performance | F benchmark/test registration only | Preserve benchmark ideas, do not automatically ship algorithm |

### C — Core development / migration (requires real integration owner)

| ID | Labs source | Actual starting point | Required owner and work | Gate |
| --- | --- | --- | --- | --- |
| C01 | `personal-knowledge-lifecycle/runtime/` (ZIP reader, streaming JSON, ChatGPT normalization, t0-core, ledger/graph ports) | Source implementation, synthetic tests reported; **no production Host/Rust cutover** | B Host + D internal HQ + A graph; stage on separate Phase 2 branch, not edit W1 donor import | Explicit consent, checkpoint/idempotency, exact receipts, no real data in Git |
| C02 | `user-io-rust-core/node-0-prime/`, R001 PRDs and Harness draft Rust `user_io.rs` | Architecture plus uncompiled/untested native R001 draft; thread-per-edit and whole-core mutex not acceptable | A Node 0, F conformance, E display adapter | Bounded actor-authenticated read-only stream, CAS, stale/privacy tests |
| C03 | `chronos-toolkit/wave2/repos-hq/research/rust-knowledge-graph/src/chronos.rs` and `hqkg-time.rs` | Staged source; README says partially tested and **not activated** | A graph owner + D HQ source owner; source-preserving selective import after Rust graph lands | No second clock scheduler/effect service; time zone/DST and native test |
| C04 | `chronos-toolkit/wave2/vivim-web-harness/studio/...chronos-conversation-projection.mjs` | Read-only staged Studio projection and tests | C Studio owner | No new provider send; existing conversation owner interface |
| C05 | `capability-observatory` + source-implemented host/HQ source registries | Mature evidence vocabulary, but production integration incomplete | F generated capability status + G final consistency | No duplicate Host catalog, clear DESIGNED vs AVAILABLE |
| C06 | Labs `vivim/voice-to-tasks/README.md` | **Research plan, not implementation**. Notes existing `PocketRecorder` / `PocketStore` donor source requiring caller audit | B/E/A after current donor source inventory | Only when existing recorder→task draft→Host commit is real and safe; otherwise pending |

### H — High-value pending development, do **not** silently include as beta implementation

| ID | Product direction | Why wait | Phase 2 preparation |
| --- | --- | --- | --- |
| H01 | R007 total VIEW ALL interpretation hypergraph | Explicitly DESIGN, 43 interpretation facets/31 source-observed producer census are not live subscriptions | Stable trace schema and fixture acceptance only; E UI optional |
| H02 | R006 100 predictive real-time composer capabilities | Design concepts, no full native benchmark or actor security proof | Rank post-beta experiments after verified R001/Node0 |
| H03 | Dynamic, screen-mirroring AutoWiki with deep temporal source links | HQ deterministic **dev wiki exists**, user-personal screen mirror not proven | D source/view proof, F source coverage/retention contracts |
| H04 | Configurable Rock Core/hot-warm-cold compaction engine | Storage model not accepted as a production data migration or benchmark | G/F quota tests, no protected World/effect deletion |
| H05 | General app builder and user-program runtime | Requires supported typed ports, grants, sandbox and visual authoring; current source evidence mixed | Register capability-space and disabled preview only |
| H06 | Automation Queen/orchestration and generalized extension/plugin loader | Labs status is discovery/copy-only; dangerous second executor/grant owner | Preserve taxonomy, explicit Host admission boundaries, no runtime |
| H07 | Advanced personal DCB/Oracle/Zero-Intent learning and graph intelligence | Mixed research, source read/consent and personal graph data ownership not yet tested | Source/privacy/benchmark contracts; avoid owner-memory auto-import |
| H08 | Local voice ASR + cross-device task creation | Day-0 strategic goal but Labs readme labels design, not product launch | Audit current recorder/import ports; promote only proven narrow flow |
| H09 | Algorithm priority ranking, semantic resolution, context selection, incremental reconciliation, outcome certainty | Research tracks, not verified integrated engines | Design input/output benchmarks against native baseline; don't replace working source |

## Critical truth categories

- **Existing in Labs** ≠ compiled in ARBOR.
- **Synthetic unit tests** ≠ Windows/phone acceptance.
- **Owner-selected direction** ≠ ratified implementation.
- **A proposal or manifest** ≠ operative source/authority.
- **Imported source** ≠ connected or runtime available.

Every item records: `lab_ref, branchSha, sourcePath, originalOwner, currentCodeStatus, testStatus, consent/privacyClass, plannedArborPath, integrationOwner, requiresWave, collisionRisk, recommendation, disposition, regressionProof`.

## Recommended parallel workflow

**NOW, concurrently with A–F:** launch P2-E1 (easy migrations, tests and manifests), P2-C1 (core integration *preparation and isolated prototype/test only*), P2-H1 (high-value technical backlog and source-delta audit). Their branches are *separate* from W1 A–F, and their write scope is `.integration/proposal5/phase2/` plus `tests/labs-intake/` on their own branch only when it does not overlap any W1-F source path. Default: documentation/evidence only.

**AFTER W1-G:** only source-owned G may promote selected Stage E changes into the current integration branch after comparing with F/E code and checking donor-source import completeness.

**AFTER W2-G:** C changes may be assigned to actual A/B/C/D/E owners on fresh branches from exact integration baseline, limited to already-required beta capabilities. Avoid creating a free-floating "Labs runtime" package.

**AFTER BETA:** H work becomes scored development rounds, not release blockers or unverified feature claims. Exception: a high-value item may enter beta if its implementation is low cost, source-backed, passes existing gates, and G explicitly records that it does not extend critical path.

## No-collision safety contract

The Phase 2 Chats MUST NOT:
- edit `src-tauri/`, `native/`, `core/programmability/`, `apps/vivim-host/`, `studio/`, `harness/`, `hq/`, `apps/mobile-relay/`, `packages/tierflow/`, `contracts/`, `capability-spaces/`, root workspace manifests or release code while W1 A–F work;
- merge, rebase, cherry-pick into `integrate/proposal5-beta-r001` or `main`;
- assert automatic synchronization with parallel ChatGPT conversations;
- use external Labs runtime dependencies in a candidate ARBOR release;
- import private archives, credentials, user data or raw chats into Git.

Any Lab source requiring one of those owners is prepared as an **exact source-adapter patch recommendation** for that owner/G, not committed into their worktree.

## Deliverable

Three branch-separated ChatGPT work orders in `phase2/prompts/`. Each returns source and target SHA, exact source files, tested/unrun evidence, collisions and a G-ready promotion recommendation. A separate integration handoff prompt decides what can be safely landed **after** W1-G.

Phase 2 is **not permission to extend beta scope**. It is a way to stop valuable Labs work from disappearing in the centralization, with a clean lane for future development.
