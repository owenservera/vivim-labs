# FA-NATIVE-001 — Actual Native Reprogrammability: ABI, Engine Slots, Typed Effects Boundary

**Status:** proposed buildable contract / implementation instructions, NOT a live Rust implementation on this branch.
**Evidence:** A `core/programmability/src/lib.rs` (blob `fe42cb6bf535c91d3ba05911977999a9d8f8c254`); `native/hqkg/src/engine.rs` (`fbb1100999fd229fb5a4352a78bbb0d77b643988`); `typed_dag.rs` (`24d4ef22ab76f1c8832f5c426b29860c74fac2a5`); `scoped_graph.rs` (`586843b56eedd4a9926bb443cbbeda80d841bda4`); `durable_dag.rs` (`140161b4f34874dee9c8540dfcd4733ed97f7cd4`). Owner branch: `work/p5-w01-a-native`; source is isolated, no merged G.

## 1 — Why this is feasible without a rewrite

A already has the important primitive: a **generational `EngineRegistry`** whose leases pin old implementations after replacement, coupled to a bounded `Scheduler` and a typed DAG with scoped source revisions. Core programmability is deliberately pure and source-ref-validated. This is stronger than adding a generic plugin loader or rewriting into a new Rust daemon.

The missing element is not an algorithm; it is the **admission lifecycle and serializable cross-runtime contract** between source evidence, pure engine generations and Host effects.

### Three independent things named "program"

| Program class | Existing source authority | Allowed capability | NOT permitted |
|---|---|---|---|
| **Pure TransformProgram** | A `core/programmability` + HQKG `EngineRegistry` | Source/grammar/graph transforms, data selection, typed DAG, no side effects | Write files, send browser messages, grant permissions, mutate Host World |
| **Wiki Program** | D `hq/lib/wiki-program.mjs` + `comment-wiki-bridge.mjs` | Owner-confirmed configuration for *existing* wiki read/render | Arbitrary path, user data export, Host control/effect |
| **Host CommandProgram** | B `apps/vivim-host/control/command-program.mjs` | Typed inert list/DAG of existing Host actions, later owner admission | Direct dispatch, approval minting, unchecked model action |
| **TierFlow Visual Program** | E `packages/tierflow/src/visual-runtime/command/compile.ts` | Versioned inert UI command proposal for existing UI World publisher | Host effect, own secondary UI publisher, browser send |

A common manifest may **link** these by stable source ref, but cannot pretend they share one runtime evaluator or authority.

## 2 — Stable data-only ABI proposal

The following is a proposed schema shape for post-G A/F owners to ratify. It is NOT already implemented by the imported A code:

```ts
type CapabilityManifestV0 = {
  schema: 'arbor.capability-manifest/0';
  capabilityId: string;           // registered HQ/Host/domain owner identity
  manifestRevision: number;
  source: { repo: 'ARBOR'; commit: string; path: string; digest: string;
            ownerDomain: 'native'|'host'|'hq'|'studio'|'tierflow' };
  interface: { inputSchemaRef: string; outputSchemaRef: string; abiVersion: number };
  mode: 'pure-transform'|'inert-view'|'owner-action-declaration';
  engine?: { slotId: string; implementationDigest: string; budgetClass: string };
  targetOwner?: { owner: 'host'|'tierflow'; catalogId: string; catalogRevision: number };
  semantic?: { wikiIrId: string; grammarPackId?: string };
  safety: { risk: 'R0'|'R1'|'R2'|'R3'; allowedSourceClasses: string[];
            maxInputs: number; maxOutputBytes: number; maxComputeBudgetMs: number };
  tests: { digest: string; sourceRef: string };
  presentation: { iconRole?: string; visualRoleRefs: string[] };
};
type ActivationDecisionV0 = {
  manifestDigest: string; expectedRegistryRevision: number;
  admissionReceiptRef: string | null; sourceVerified: boolean;
  decision: 'proposed'|'rejected'|'admitted'|'revoked'|'rolled-back';
  engineGeneration?: number;
};
```

**Constraints:** no raw JS/Rust source, executable path, function pointer, URL-to-code, authority token, shell script, arbitrary handler name, model prompt or private corpus in the manifest. The payload is **a typed description of already reviewed local source**, not a code download/install instruction.

A runtime admission path must reject an unknown schema, duplicate capability owner, absent source/review receipt, invalid risk assignment, or a manifest whose dependency graph exceeds bounded resources. For an effectful Host action, an activation entry is only a **declaration**; Host's own catalog and dispatcher remain the live effect authority.

### Source and runtime identity

The runtime source digest must be produced from the **trusted in-ARBOR source walker / compiled bundle**, not accepted from a caller's JSON alone. A local file path/ownerRef is not itself proof of authorization. Cross-runtime `SourceRef` in F's contract mirror has an ID/domain/owner/revision but needs full source-attestation and actor context before a live effect path can use it.

Version vector:
`{bundleSha,capabilityRegistryRevision,semanticWikiIrRevision,grammarRevision,engineGeneration,hostCatalogVersion,hostWorldRevision,policyRevision,sourceEpoch,uiRevision}`.

Only the fields relevant to a specific read/plan must be transmitted, but a consumer must not accept an older or ambiguous revision as "close enough." Every stale result is `SKIPPED_STALE`, `UNAVAILABLE` or `CLARIFICATION_REQUIRED` rather than successfully executed.

## 3 — Pure Rust engine generation proof to build in W2-F1

**Target real Rust code locations** (do not create parallel stores/daemons):
- `core/programmability/src/lib.rs`: versioned manifest validation, typed serialization, pure transform admission.
- `native/hqkg/src/engine.rs`: generational `EngineRegistry` and `ComputeEngine` lease reuse, per-task budget/cancel.
- `native/hqkg/src/typed_dag.rs`: source-revisioned DAG and existing bounded cache.
- `native/hqkg/src/scoped_graph.rs`: scoped namespace checks, never trusted actor authentication.
- `native/hqkg/src/durable_dag.rs`: *descriptions* of pure replay, not effect receipts.
- `contracts/interop/v1/`: TS/Rust/JSON ABI mirrors; F owns compatibility tests.

**Concrete expected demonstration** with *real Cargo tests*:
1. Instantiate v1 read-only semantic engine in a registered slot `semantic.grammar`; compute input `"show status"` and capture typed result + exact generation and source digest.
2. Hold a v1 `EngineLease`, install reviewed v2 in same slot, call v1 lease again and prove original output remains stable while newly pinned lease gets v2.
3. Run a two-node `TransformPlan` under policy byte/task/time budgets. Cancellation halts future pure work and cannot leave partial side-effect state.
4. Change sourceEpoch/worldRevision/policyRevision while one producer computes; reducer rejects stale result.
5. Try to activate `effectClass=provider-send`, `shell`, missing owner/catalog ID, forged source hash, untrusted policy/actor or unknown schema as a pure package: all refuse.
6. Roll back to v1 via owner-reviewed generation compare-and-swap and show old/new versions remain distinguishable in receipts and visual truth. No Host receipt is claimed by pure engine lease.
7. Validate the same serialized manifest and source-ref against real Rust/TS/JS tests. `cargo test` and `node --test` actual exit codes are mandatory; V8 evaluation is not sufficient.

**Proof required to call "reprogrammable" implemented:** versioned manifest installed from in-ARBOR trusted source, pure engine swap works, rollback works, source/policy changes invalidate dependent outputs, and a user-observable view updates—all without a new effect owner.

## 4 — Parallel event fabric: one input, many bounded intelligence producers

The Labs [Native I/O Fabric](https://github.com/owenservera/vivim-labs/blob/lab/user-io-rust-core-20261112/user-io-rust-core/PARALLEL-EVENT-FABRIC-ARCHITECTURE.md) is **design-only**. ARBOR pre-G N0-io has `arbor.n0-io/1` candidate contract; V-visual has `arbor.p5.realtime-semantic-projection/1`. They do **not** share one accepted ABI yet.

The implementation must not build a sequential "super-router." A verified Host/Tauri ingress creates one immutable bounded event, then independent pure Rust subscribers compute in parallel or appropriately scheduled bounded queues. No subscriber owns UI/Host effects. A single version-fenced reducer maps their outputs into an inert graph of understood meanings.

```text
local edit (same text visible immediately) / PWA draft input
      │
Host/native trusted session/auth source → immutable InputSnapshot
      ├── lexical/negation producer (hot)
      ├── context/knowledge read producer (authorized source + budget)
      ├── semantic WikiIR/grammar producer (hot Rock pack)
      ├── project/DCB resolver (owner-scoped only)
      ├── time/Chronos producer (pure due/relative time)
      ├── UI preview/TierFlow roles producer
      └── optional model candidate (no approval)
      │
revision+scope+source+policy gate → one InterpretationTrace
      │                              │
TierFlow/Host/PWA renders             Host original-text plan when Submit
      │                              └→ approval / effect journal / receipt
      └ ← read-only result + status
```

**Hard bounds:** max draft bytes, active producer count, per-event derived fanout, graph traversals, source refs, CPU/time, output bytes, queue size, TTL source epochs and materialized cache bytes. With insufficient budget, producer status becomes `SKIPPED_BUDGET`/partial, never silent omission. Competing simultaneous editing sessions must be actor/world-scoped and cannot share a global "current" suggestion buffer.

**Mobile policy:** the PWA is an authenticated remote input/output client of the Host. It may perform only data-only local hints using the same portable syntax, with clear `offline_unverified` state. Real Host action admissions require reconnect, current challenge and current World/policy version. No local PWA effect queue may replay after reconnect.

## 5 — Rust/Host separation that must survive future refactors

| Concern | Owner | Actual interface invariant |
|---|---|---|
| Engine algorithms and hot swap | A HQKG `EngineRegistry` | Pure, bounded, source-pinned generations |
| Graph node, source/namespace relations | A HQKG, D HQ source provenance | Graph data isn't a permission |
| Source-comment semantic registry / wiki | D HQ existing registry/writer | No new action handler from a comment |
| Original user text interpretation | B Host accepted NLCL | Exactly source/current policy checked |
| Host action execution/journal/receipt | B Host dispatcher | Single effect writer, approval/uncertain-send rules |
| Studio web provider side effect | C Studio through B Host approval | Exact thread, one browser sender, no external API |
| Typed UI World publish/rollback | E TierFlow / Host delegated owner | Existing publisher and revision, not semantic highlight |
| Personal DCB/memory and export | B Host with C read/definition support | Consent/erasure, private owner scope, no HQ public leak |
| Portable contract compliance | F tests/schema mirrors | Test truth only, not owner grants |
| Product merge/build/locks | G | Sole serial integrator |

### Required negative tests before any effect or private source is exposed

- Actor/owner mismatch; source spoof; current source revoked while output in flight.
- Invalid code comment tries `@hq:executor` or raw handler path; no effect.
- Two semantic aliases resolve to different high-risk effects; ambiguous and refused.
- Wrong catalogVersion or World revision; no dispatch.
- Live text edit invalidates prior Tab slot; no stale selection application.
- Crashed/uncertain provider send; no retry even when engine v2 installs.
- Plugin package attempts arbitrary Node import or Rust dynamic library; refuse.
- Installed source becomes unavailable at low disk; degrade read, never bypass disk reserve.
- PWA untrusted actor/grant, browser same-origin compromise and replay; no elevated Host effect.
- Stale/revoked private DCB source does not leak existence in VIEW ALL or AutoWiki.

## 6 — First post-G source patch discipline

**Do not implement until original G records an exact accepted combined source HEAD.**

A owns Rust crate files, F owns typed parity/schema/test files, D owns HQ semantic read adapter, B owns Host original-text admission adapter, E owns visual projection adapter. Each uses the existing module/owner instead of a new native bus service, graph database, action router, plugin daemon, scheduler or publisher.

**Implementation exit is NOT document approval:** only a real cross-language build, a real native generation/hot-swap test, negative actor/Host policy tests, and one actual source-grounded user-facing read path can graduate this to `RUNTIME_VERIFIED`.
