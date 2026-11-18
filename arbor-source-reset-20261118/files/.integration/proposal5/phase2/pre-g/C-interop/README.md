# ARBOR Proposal 5 — P2-C Native / Chronos / Visual Read Interoperability

**Disposition: PREPARED_NOT_PROMOTED.** Source-grounded preparation only, outside the W1 A–F workers and G's integration. This directory contains no active runtime, user data, executor, publisher, graph database, schedule daemon, action catalog, stored token, or grants.

## Exact observed pins (GitHub ref reads, 2026-11-12)

| Source | Immutable commit SHA | Material read |
| --- | --- | --- |
| ARBOR pre-G base | `8b23c73bf450738a76b256bf0fa17ecbf86db591` | P2-C work order / Phase2 readiness |
| ARBOR A | `dda546a11294920d6521a0f64ab7ee62ec993c70` | `native/hqkg/src/{engine,typed_dag,scoped_graph}.rs` and PREG-A |
| ARBOR D | `ad55fe28c2c331841675f308f9199ee4c2add86a` | `hq/lib/arbor-service.mjs`, PREG-D |
| ARBOR E | `1a5cf26ce14ebce4e31d9a4373060ace61852645` | `packages/tierflow/src/visual-runtime/final-wave/integration/world-visual-bridge.ts`, PREG-E |
| ARBOR F | `be458f3c4cdb7eb38d47b1de2a00d441b7cdc704` | `contracts/interop/v1/{types.ts,types.rs,validate.mjs}`, `contracts/chronos/staged-contract.v1.json`, PREG-F |
| vivim-labs main | `022608f5797e7b4a14ccc4c393bd3c5a4704a4be` | `chronos-toolkit/wave2/` Rust, JS, TS and read-model tests |
| vivim-labs Node0 | `61d9fc15c55144c4f0b79afa248981c2b0ca9bd6` | `user-io-rust-core/` R001 PRDs / Node0 security |
| vivim-labs R005 | `67e76c5b730dcc2cd3ab3919c3ba273a6c260db8` | `nlp-command-ui-lab/src/{semantic-lens,bridge}.mjs`, R005 tests/doc |
| vivim-labs Chronos branch | `b87ea71c653c3833451bd0af19d9ef1c4327728e` | separate Chronos branch ref; detailed staged files inspected on Labs main |
| Harness R001 spike | `fa6dea9576c83496d95562b98b7a815fda4aec88` | `core/programmability/src/user_io.rs` |

The earlier readiness ledger used R005 `56bb145e...`; the R005 branch has advanced to `67e76c5b...`. Audit reflects the live read ref. The readiness ledgers are supporting plans, not compile or device proof.

## Observed source vs proposed seam

1. **A native:** `native/hqkg/src/engine.rs` already defines `EngineRegistry`, `Scheduler`, budgets and cooperative cancellation; `typed_dag.rs` provides versioned pure transforms and `scoped_graph.rs` restricts graph identities. No second graph store, scheduler, user-IO loop, or state authority is justified. These files are present on **A's worker**, not merged to G.
2. **D development:** `hq/lib/arbor-service.mjs` exposes `createDevIntelligence`, `sourceEvidence` and `indexWatermarks` behind required Host-scoped actor/session/approvedRoots/authorize parameters. The development index is rebuildable and separate from Personal World. Index HEAD equality is not file freshness.
3. **E visual:** `world-visual-bridge.ts` projects safe static subsets from immutable World revisions and checks scope/digest/CAS; TierFlow owns display but cannot invoke Host effects or publish a new World from arbitrary visual content.
4. **F contracts:** `contracts/interop/v1/types.ts` already provides `SourceRef` (id/domain/ownerRef/revision), `RevisionEvent`, `InertViewProjection` and Host journal state vocabulary; `validate.mjs` rejects owner/revision mismatch. `contracts/chronos/staged-contract.v1.json` is byte-identical to Labs contract (Git blob `a74a612458e1854a95bc8331e6ed14747fec2742`), declaring `staged_not_activated`. Reuse F; P2-C is **not** a replacement contract owner.
5. **Labs R001:** Node0 proposes actor-authenticated revisions, independent pure evidence and bounded projections. The separate Harness `user_io.rs` draft uses per-event `thread::scope`; Tauri wraps `UserIoCore` in a mutex. It is an inert demo with default empty trusted sources, not an approved per-keystroke runtime. The draft's own tests and the Host/native security link are unverified.
6. **Labs Chronos:** Rust `src/chronos.rs` exports `ClockSource`, injected `FixedClock`, bounded time windows/freshness/retry and a `ChronosCompute` using existing `ComputeEngine` slot `utility:chronos`. Time calculations grant no effect. `supervisor-timeline.mjs` imports `../../supervisor/recovery.mjs` (missing from the inspected ARBOR D destination), so importing that JS file as-is is **BLOCKED**. Studio conversation and TierFlow visual adapters are source-present but staged. IANA timezone/DST and production durable CAS/claims are not implemented by the read-only contract.
7. **Labs R005:** `semantic-lens.mjs` uses bounded lexical spans, conservatively detects negation and labels project mentions `lexical_hint`. Its live bridge reads separate checkouts through environment paths and dynamically imports HQ; that is **not** an installable ARBOR-local bridge. R005 recorded 28/28 Node and 100/100 synthetic probes in Labs; these are historical source reports, **not tests run in this P2-C execution**.

## Minimum interoperable boundary (proposal)

`Host/B authenticated actor + session + owner scope` → immutable `SourceRef` from A/D/B → revisioned **read-only** `ReadObservation` → F-compatible inert `InertReadEnvelope` → E visual render. The only admission of effect, approval and receipt remains at B Host. No input text, model guess, semantic highlight, Chronos due, UI gesture, HQ index or R001 suggestion can mint a Host action ID or permission.

`interop.types.ts` stages **structural** actor/source/revision/event/projection types. `interop-read.mjs` is a **test-only pure gate** with a sanitized low-cardinality cue output, not a trust/authentication verifier; its `expected` parameter MUST be supplied by a trusted Host owner. It deliberately rejects active-effect claims, scope/actor substitution, duplicate/stale event sequence, raw/unknown visual fields and stale/ambiguous status. `acceptChronosRead` exposes only an estimate classification/count, never a due-run, live heartbeat or receipt claim. Do not mount either file as a product endpoint.

## Migration sequence (no beta critical-path dependency)

1. **G first** combines already-frozen A–F at their verified tips and validates root manifests/lockfiles and no donor runtime dependency. P2-C has no cherry-pick permission into G during this step.
2. **F+G** compare P2-C staged types with existing `contracts/interop/v1`; choose minimal tests, *not* a duplicate protocol. Run pure negative fixtures against the real F contract.
3. **A+B+D** join trusted source identity, revision and fresh read proofs; then *selectively* compile pure Chronos Rust using existing HQKG registry only. Defer native R001 thread-per-edit implementation and any scheduler.
4. **C+E+F** bind Studio conversation/HQ supervisor **existing-source** reads and TierFlow inert visual projections; preserve provenance/revision/actor scope. Resolve missing supervisor replay dependency before D Chronos import.
5. **B+F** independently accept security/regression gates, uncertain outcome no-retry, revocation and Windows/PWA/device tests before calling the feature available.

## Evidence limits

GitHub source reads and exact branch pointers were observed. **22/22 standalone V8 synthetic function assertions passed** during preparation. The committed `interop-read.test.mjs` suite has **NOT** been run by Node; TypeScript, Cargo, integrated Windows/Host/phone, full regression and physical-device acceptance are **UNRUN**. No private data was ingested. No active product source or worker branch was modified.

See `source-delta.json`, `acceptance.md`, `owner-handoff.md`.
