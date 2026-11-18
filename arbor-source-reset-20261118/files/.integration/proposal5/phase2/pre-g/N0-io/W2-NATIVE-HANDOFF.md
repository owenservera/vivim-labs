# W2 NATIVE HANDOFF — smallest post-G bounded first hop

**G handoff only.** No changes to A–F, main, G, product runtime, root manifests, source packages or other Phase 2 lanes were made here. Source gates are not release gates. P2-C owns general Chronos, Node0 and visual interop; N0-IO limits itself to native bidirectional user I/O.

## Integration DAG and minimal changes

| Gate / cost (relative dependency count, not an elapsed-time promise) | Owner and exact destination paths | Narrow acceptance |
|---|---|---|
| 0. Integrate A/B/E/F source first — HIGH, 4 owners | G serially integrates work/p5-w01-{a-native,b-host,e-ui,f-verify}, root Cargo workspace and lock under G only | No duplicate launchable Host/phone/native runtime, workspace imports and file paths resolved; failing security gates remain marked BLOCKED |
| 1. Host/Tauri verified actor and native bounded IPC — HIGH, 2 owners A+B | A src-tauri/src/main.rs and capabilities; B apps/vivim-host/direct-phone.mjs, apps/vivim-host/control/dispatcher.mjs, Host launch contract in A src-tauri/NATIVE-SECURITY-W02-PREP.md | Untrusted webview or spoofed localhost refused; no durable bearer passed to renderer; PC native and phone session bind actor, audience, source policy and revocation; no request-supplied grant accepted |
| 2. Pure first-hop input and source snapshot — MEDIUM, 2 owners A+B | A core/programmability/src/lib.rs and **new reviewed submodule** in same crate, A native/hqkg/src/typed_dag.rs; B apps/vivim-host/control/action-catalog.v1.json and existing read adapter | Validate <=4096-byte input, sequence/CAS, separate Submit identity, source policy/version and composition. No persistence/effects. Snapshot sourced from authoritative read port with no private corpus broadcast |
| 3. Independent derivations via *existing* graph — MEDIUM, 1 core owner A + F test review | A native/hqkg/src/engine.rs, typed_dag.rs, invalidate.rs and scoped_graph.rs; staging reference.types.rs only for review | Declarative interests, pure reactor leases, bounded budget and deterministic revision-check joins; no new graph, semantic router, scheduler service, or thread-per-edit. Current Scheduler is synchronous: true parallel execution is a separate measured engineering gate inside **this** engine |
| 4. Audience-filtered output and cancellation — MEDIUM, 2 owners A+E | A Rust first-hop projection, E apps/mobile-relay/control-client.mjs, packages/tierflow/src/visual-runtime/command/compile.ts and renderer | Partial source-labelled projections, monotone projectionVersion, current inputRevision/graphEpoch CAS and source status; screen reader equivalent; UI local edit/IME always responsive. Never expose source bodies beyond grants |
| 5. Host admission and receipt reference — HIGH, 2 owners B+A | B apps/vivim-host/control/{intent-planner,dispatcher,journal}.mjs, A native bridge only | User Submit produces inert typed request; any actual effect must pass Host plan digest, current approval, exclusive claim and durable receipt. Lost or claimed-unreceipted send = UNCERTAIN, no auto retry |
| 6. Differential/device proof — HIGH, 3 owners F+E+A, G release decision | F contracts/interop/v1 and tests, A unit tests/Windows, E PWA fixtures/accessibility | Cross-check Host v6 action catalog/NLCL, R001 demo, TierFlow visual IR, Unicode/IME, phone revocation, CLI, voice partial/final, queue/backpressure; preserve all UNRUN labels until observed |

## First viable Rust hop, without competing runtime

1. A registers an input module in the **existing** core/programmability crate after G; it accepts no actor in JSON. The verifier injects an opaque ActorContext *out of band*, tied to the authenticated transport and current Host permissions.
2. A emits one immutable, bounded DraftChanged or VoicePartial revision from an authorized snapshot. The editor paints synchronously before and independently of this call. Source adapters disclose only permitted references/body slices; development HQ must never receive private draft text by default.
3. A maps independent subscriber descriptors to current hqkg EngineRegistry leases / typed DAG prerequisites; compute stays pure and ephemeral with budgets/cancellation. The current synchronous Scheduler is not itself proof of parallel reactor delivery; P1 acceptance requires bounded concurrency design **inside A's existing engine** and independent progress tests, not the R001 per-edit scoped-thread strategy.
4. A reduces out-of-order results using (actor scope, session, inputRevision, graphEpoch, policyVersion, engine generation, source refs). Stale completions drop. Conflicts remain explicitly ambiguous. Audience is checked at both source read and output projection.
5. E renders optional partial output and full authorized snapshot/resync; accessibility/text geometry/IME remain in the platform UI. A/broker never paints DOM or uses candidate label as grant.
6. A Submit has a new committed idempotency identity + exact content digest. B Host alone checks live plan/catalog/grant/expiry/approval and writes attempt/receipt. Rust may project a **Host-verified receipt reference**, never itself write Host journal state.

## Minimal acceptance corpus

Run the committed staging synthetic proof (source-only unit contract; no actual C++/Rust/Host auth):
    
    node .integration/proposal5/phase2/pre-g/N0-io/synthetic-proof.mjs

After G merges, from ARBOR repo root (G controls workspace; do not use these commands as claimed results):

    cargo test --manifest-path core/programmability/Cargo.toml --locked
    cargo test --manifest-path native/hqkg/Cargo.toml --locked
    cargo test --manifest-path native/hqkg/Cargo.toml --features importer --locked
    cargo check --manifest-path src-tauri/Cargo.toml --locked
    cargo metadata --locked --format-version 1
    node --test apps/vivim-host/test/*.test.mjs
    node --test apps/mobile-relay/test/*.test.mjs

Windows MSVC + Tauri package, owner-token/loopback/port collision, PWA HTTPS enrollment/typed IME/screen-reader, source privacy checks, external provider send-once and revocation, full integrated CI and device gates are **UNRUN** here. Execute using project-approved environment, do not mislabel source tests as acceptance.

## Blockers and stop condition

P0: no verified Host/Tauri actor or child attestation; A remote IPC wildcard and Tauri dependency review; B device active bearer per-request replay concern; no native secure PWA bridge. P1: A's Rust engine is not packaged as the product I/O graph; Scheduler synchronous, source-bound permission adapter and bounded fan-out unspecified; no true native partial output stream. P2: no real native voice/accessibility/CLI/automation adapters or approved full-project-update action. All require owner resolution after G.

The exit criterion for N0-IO is **PREPARED_NOT_PROMOTED** with exact source evidence, staged contract and synthetic tests. P2-C retains its Chronos/interop scope; avoid duplicate scheduler/graph/catalog/host journal and coordinate only through this handoff.
