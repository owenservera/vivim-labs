# ARBOR Proposal 5 — FA-PG00 / Source Truth and Duplicate-Authority Census

**EXECUTE NOW IN AN ISOLATED WORKER CONVERSATION using connected GitHub.**
**First read:** [shared execution contract](00-SHARED-PRE-G-CONTRACT.md) (same `prompts/` directory: `00-SHARED-PRE-G-CONTRACT.md`), `../SOURCE-ATLAS.v1.json`, `../GAP-REGISTER.v1.json`, `../WAVE-MANIFEST.v1.json` from the foundation branch. In case of any broken relative link, read by the exact paths below.

- **Repository:** `owenservera/ARBOR`
- **Create branch:** `work/p5-fa-pg00-source-truth-r001` from current **pinned** `work/p5-foundational-architecture-prep-r001` HEAD (write exact full commit into ledger).
- **Only write path:** `.integration/proposal5/foundations/proofs/PG00-source-truth/`
- **Status on completion:** `PREPARED_NOT_PROMOTED`; no source integration, code changes, G handoff merge, or promotion.
- **Mapped tasks:** `FA-PG00-01..FA-PG00-08` from wave manifest.
- **No time gate:** independent source audit; must not delay original Lane G.

## FIRST: verify branch + actual source (not recap)

Recheck current HEADs, then read actual code and final `integration/worker-reports/PREG-A..F.md` on:

| Owner | Frozen worker ref | Must inspect real code, not only worker report |
|---|---|---|
| A | `work/p5-w01-a-native@dda546a11294920d6521a0f64ab7ee62ec993c70` | `core/programmability/src/lib.rs`, `native/hqkg/src/engine.rs`, `typed_dag.rs`, `scoped_graph.rs`, `durable_dag.rs`, `src-tauri/tauri.conf.json` |
| B | `work/p5-w01-b-host@78a5105a9c1f1880d658f350e130b1e2632d3532` | `apps/vivim-host/control/action-catalog.v1.json`, `intent-planner.mjs`, `command-program.mjs`, `dispatcher.mjs`, `journal.mjs`, `direct-phone.mjs` |
| C | `work/p5-w01-c-studio@f950b91685927f37b48d020433aa8d2d3c271419` | `studio/src/plugins/registry.mjs`, `studio/src/recipes/catalog.mjs`, `service.mjs`, `studio/src/agents/dcb-definition-store.mjs`, `studio/src/agents/personal-agent-service.mjs` |
| D | `work/p5-w01-d-hq@ad55fe28c2c331841675f308f9199ee4c2add86a` | `hq/lib/annotation-v2.mjs`, `semantic-registry.mjs`, `action-registry.mjs`, `action-router.mjs`, `comment-wiki-bridge.mjs`, `wiki-program.mjs`, `arbor-service.mjs` |
| E | `work/p5-w01-e-ui@1a5cf26ce14ebce4e31d9a4373060ace61852645` | `apps/mobile-relay/control-language.mjs`, `browser-device-key.mjs`, `sw.js`, `generative-ui/compiler.mjs`, `packages/tierflow/src/visual-runtime/command/{ir,compile}.ts` |
| F | `work/p5-w01-f-verify@be458f3c4cdb7eb38d47b1de2a00d441b7cdc704` | `contracts/interop/v1/{types.rs,types.ts,protocol.schema.json,validate.mjs}`, actual tests/fixtures and reports |

**G check:** re-read `.integration/proposal5/prompts/W1/G.md` from `work/p5-pre-g-convergence-r001` and check `integrate/proposal5-beta-r001` branch. At prompt authoring G was absent. **Do not create or modify it.**

**Existing preparations to compare, not redo:** 
- `work/p5-p2-preg-tests-r001`: `.integration/proposal5/phase2/pre-g/T-tests/{source-delta.json,synthetic-contracts.test.mjs,owner-handoff.md}`
- `work/p5-p2-highvalue-labs`: `.integration/proposal5/phase2/highvalue/{PRIORITIES.md,cards.json,DEPENDENCIES.md}`
- `work/p5-preg-{semantic-wiki,visual-language,autowiki,n0-io}-r001`: respective S/V/W/N0 source mapping, actual pure adapters/fixtures and run ledgers.
- `owenservera/vivim-labs` **main**: `capability-observatory/lane-d/src/observatory.mjs`, `vivim/extension-points/VERIFIED-HOOKS.md`, `personal-knowledge-lifecycle/runtime/src/t0-core.mjs`, `vivim/voice-to-tasks/workbench/core.mjs`, and its five algorithm research tracks. Labs is a **donor evidence source**, not operational ARBOR runtime.

## Mission

Produce a **code-based cross-source factual map** so future A/B/C/D/E/F waves know one canonical owner and do not import overlapping stacks. The 43-entry existing SOURCE-ATLAS is a *starting set*, not permission to treat it as complete.

1. Build a **module/code-entry census** for launch entrypoints, action catalog/handlers and risk, source/index/wiki writers, Rust graph/engine slots, typed command/semantic interpreters, PWA route/permission, TierFlow publisher, Studio Chrome send owner, DCB/memory and durable receipts. Traverse directories and exact imports; if GitHub list truncation prevents exhaustive listing, declare scope as partial and enumerate unscanned areas. Do NOT invent source files.
2. Classify every observed module with an explicit lifecycle: `CANONICAL | ADAPTER | PROJECTION | DUPLICATE_REVIEW | CONSOLIDATE_PENDING | HISTORICAL | RETIRE_CANDIDATE`, plus `SOURCE_PRESENT | PREPARED | DESIGN_ONLY | TEST_EXECUTED | BLOCKED`. Annotate exact owning source domain, source Git blob SHA, and why other similar modules are not the owner.
3. Build **two distinct crosswalks**: (a) action/capability inventory against Host catalog v6 and TierFlow type IDs; (b) source/wiki/semantic comment/typed grammar systems. Make it impossible for a wiki action declaration to imply a Host executable action. Only actual handler source and verified runtime claims count as effect-capable.
4. Verify current A–F `FREEZE_READY` reports, git refs, root Cargo/Bun import constraints and G's sole landing authority. Flag source drift as a blocker, not auto-branch rewrite.
5. Compare Labs source-only work against imported ARBOR code and Phase 2 P2-T to identify **the smallest actual missing source/contract**, not an entire replacement toolkit. Inspect actual source/test files and explicitly mark Lab designs `DESIGN_ONLY`.
6. Make a dependency/duplicate diagram that demonstrates there is **one** graph, Host effect writer, Studio provider sender, UI publisher, semantic registry, source wiki writer and end-user scheduled-effect owner.
7. Recommend at most five post-G code-owner handoffs with precise path/module and *real* test command that would prove each one, including P0 native/Host/PWA and source-read trust. All security tests not executed remain UNRUN.

## Required files in exclusive write path

- `README.md`: source snapshot, precisely which branches/files traversed, test truth, bounded scope.
- `SOURCE-DELTA.json`: **source-by-source** names/refs/owners/status and current branch/head/actual blob; include `coverage.complete=false` when incomplete.
- `OWNER-CANONICALITY.json`: registry/publisher/dispatcher/storage/scheduler/browser/phone owner graph with competing implementations classified.
- `CAPABILITY-CROSSWALK.json`: explicitly source-only vs actual runtime-known actions; no permission grants.
- `PROOF-PLAN.md`: tests for source closure, import paths, ABI drift, orphaned handles, no-donor build; include exact target locations and owner names.
- `NEGATIVE-ACCEPTANCE.md`: duplication/unknown action/handler drift/source spoof/false test evidence negatives.
- `OWNER-HANDOFF.md`: maximum five exact tasks after G.
- `RUN-LEDGER.json`: true pass/fail/UNRUN and `PREPARED_NOT_PROMOTED`.

Optional pure fixture/test source is allowed only inside the lane prefix; **no product rewrites**.

## Acceptance and stop

- Truthful source census with explicit scanned coverage, exact HEAD/blobs, duplicate classification and G blockers.
- 0 changed paths outside `.integration/proposal5/foundations/proofs/PG00-source-truth/`.
- No new action owner, graph or alternative source index.
- If GitHub connector cannot run actual `cargo test`/Node/Bun: mark **UNRUN**; citation/source code does not imply PASS.
- Commit final artifacts, provide exact HEAD and a 5-item max cross-owner handoff, report **PREPARED_NOT_PROMOTED** and STOP.
