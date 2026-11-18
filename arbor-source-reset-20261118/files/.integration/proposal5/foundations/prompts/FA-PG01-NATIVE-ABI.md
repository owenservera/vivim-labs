# ARBOR Proposal 5 — FA-PG01 / Native Programmability and Event-Fabric Proof

**EXECUTE NOW with connected GitHub and a real Rust runtime IF available.** First read `.integration/proposal5/foundations/prompts/00-SHARED-PRE-G-CONTRACT.md`, `NATIVE-MACHINERY-AND-CONTRACTS.md`, `SOURCE-ATLAS.v1.json`, `WAVE-MANIFEST.v1.json` from the foundation branch.

**Repository:** `owenservera/ARBOR`
**Create:** `work/p5-fa-pg01-native-abi-r001` from the **current pinned** `work/p5-foundational-architecture-prep-r001` HEAD; record its full SHA.
**Exclusive writes:** `.integration/proposal5/foundations/proofs/PG01-native-abi/` ONLY. No frozen A source edits, no Cargo root changes, no G code, no runtime model loading or shell execution through VIVIM.
**Task coverage:** `FA-PG01-01..FA-PG01-08`. **Disposition:** `PREPARED_NOT_PROMOTED`.

## FIRST read actual Rust code, not architecture titles

**Immutable source branches at authoring (verify current first):**
- ARBOR A `work/p5-w01-a-native@dda546a11294920d6521a0f64ab7ee62ec993c70`
- ARBOR F `work/p5-w01-f-verify@be458f3c4cdb7eb38d47b1de2a00d441b7cdc704`
- B `work/p5-w01-b-host@78a5105a9c1f1880d658f350e130b1e2632d3532`
- Pre-G native I/O `work/p5-preg-n0-io-r001@cde3b6e5521d5b17215ab10088cb8090f8b87e6f`
- Pre-G interop `work/p5-p2-preg-interop-r001@73b1ea769de6023dfff857d7e9c4003731f3b112`
- Labs `lab/user-io-rust-core-20261112` from `owenservera/vivim-labs`, refresh current head rather than assuming it is immutable.

**Exact source modules and blobs previously observed:**
- `native/hqkg/src/engine.rs` blob `fbb1100999fd229fb5a4352a78bbb0d77b643988`: `EngineRegistry::install(..., expected)`, `pin(slot)`, `EngineLease`, `ComputeEngine`, `CancellationToken`, `Scheduler`.
- `native/hqkg/src/typed_dag.rs` blob `24d4ef22ab76f1c8832f5c426b29860c74fac2a5`: `Namespace`, `RevisionToken`, `SourceBinding`, `TransformPlan`, `DagExecutor`, `IncrementalCache`.
- `native/hqkg/src/scoped_graph.rs` blob `586843b56eedd4a9926bb443cbbeda80d841bda4`: structurally scoped graph. **Caller-provided provenance not authenticated.**
- `native/hqkg/src/durable_dag.rs` blob `140161b4f34874dee9c8540dfcd4733ed97f7cd4`: durable pure computation *descriptions*, not Host effect journal.
- `core/programmability/src/lib.rs` blob `fe42cb6bf535c91d3ba05911977999a9d8f8c254`: `Program`, `PolicyLimits`, `ValidatedProgram`, `validate_program`, no effect permission.
- `native/hqkg/Cargo.toml` blob `b90c06fa3035ad7b8b7cdd2e97593c3e4fd2b19a`: package `hq-knowledge-graph`, lib crate `hqkg`. `core/programmability/Cargo.toml` blob `9c3a8e997897a8aa4079ecbef420adc1af41c090`: lib crate `vivim_programmability_core`.
- `contracts/interop/v1/{types.rs,types.ts,protocol.schema.json,validate.mjs}` on F.
- `.integration/proposal5/phase2/pre-g/N0-io/{IO-CONTRACT.json,reference.types.rs,reference.types.ts,synthetic-proof.mjs,W2-NATIVE-HANDOFF.md}` on N0.
- `.integration/proposal5/phase2/pre-g/C-interop/{interop-read.mjs,interop.types.ts,acceptance.md,source-delta.json}` on P2-C.

## Mission: prove the *existing* native machinery can underpin a reprogrammable product

Do not invent an independent plugin VM, new graph, daemon, persistent scheduler or action router. Identify the minimum **existing Rust API** and versioned contract additions required for an owner-admitted pure capability to be swappable and observable.

### Build a source-true lifecycle and ABI map

1. Trace actual `EngineRegistry::install` CAS semantics, interface/capability subset guard, lease pinning, generation retention, source provenance assumptions and cancellation limits. **Important existing limitation:** cooperative `CancellationToken` cannot interrupt a blocked `ComputeEngine::execute`. Record this and design resource/deadline fencing accordingly; do not promise preemption.
2. Inspect `DagExecutor`, `IncrementalCache` and `SourceBinding` for bounded cache bytes, namespace owner, epoch, source revisions and redacted receipt semantics. Record actual runtime data class and actor authentication separation.
3. Compare proposed `contracts/capability-manifest.v0.schema.json` on foundation branch with actual A Rust `Program` and F `SourceRef` schema: precise missing serialization, owner admission, input/output types, manifest/provenance digest, compatibility, user-visible result and resource bounds. Mark unimplemented fields clearly.
4. Compare N0 native input/event ABI with V-visual and E browser/TierFlow types. Design minimal **parallel bounded pure producer fan-out** driven by one trusted input snapshot; do not create a sequential "super-router" or duplicate Host planner.
5. Distinguish a **pure engine generation receipt** from real Host approval/effect receipt. Show which fields are source-verified versus merely caller-supplied.
6. Define exact post-G patch owners and path set: A Rust crates, B native↔Host actor ingress, F contracts/test parity, E render subscription; owner changes must be separate worker lanes.

### Actual source-based proof (required code if feasible)

Inside your owned staging prefix, prepare a minimal **Rust test plan and source-compatible proof** that uses **the existing** `hqkg::engine` public interface on a read-only checkout of A at its full pinned SHA. Verify actual API names before writing.

Required real tests where a local Cargo runner is available:
- v1 engine `ComputeEngine` installs at `semantic.grammar`, `pin` returns generation 1 and deterministic output.
- Replace v1 with v2 via `install(v2, Some(1))`; old `EngineLease` still responds v1 and new lease responds v2. Check `SwapReceipt`.
- Wrong expected generation, different interface, removed capability or empty descriptor refuses, with no modified active generation.
- Bounded `TransformPlan` / cache, source epoch change, namespace owner mismatch and cancellation are identified by actual `RunState` rather than side effects.
- Simulate old result arriving after a later input/source revision: projection reducer refuses it, no private source ref leak.
- Pure engine cannot execute a Host action; malicious manifest `owner=host`, unknown effect or caller-asserted `granted=true` has no execution path.
- Reinstall old **reviewed implementation** as newer generation for rollback (not rewriting history or rewinding generation). Verify old pinned generation remains stable.

If your GitHub-only tool cannot execute Cargo, **commit compilable focused Rust test/harness code and mark every Cargo check `UNRUN`**, including that syntactic/typing compatibility has NOT been verified. Do not copy the whole A native tree into ARBOR's proof directory or install new crates in product.

## Required evidence products

- `README.md` — source pins/head snapshots, measured code semantics, what cannot be verified.
- `SOURCE-DELTA.json` — A/F/N0/P2-C type/contract/function comparison and owner-specific missing ports.
- `ENGINE-ABI-CROSSWALK.json` — precise `EngineDescriptor/EngineRequest/EngineOutput`, Program, Namespace, RevisionToken → proposed capability manifest/activation/event trace mapping.
- `PROOF-PLAN.md` — actual Cargo test/commands and exact expected assertions; optional `rust/` or `test/` code under assigned subtree.
- `NEGATIVE-ACCEPTANCE.md` — forged actor, stale generation, cancellation, source revocation, old pack mismatch, loss of provenance, attempt to load effectful code.
- `OWNER-HANDOFF.md` — ≤5 patches after G, no product code change now.
- `RUN-LEDGER.json` — exact base/final SHA, actual/UNRUN results, `PREPARED_NOT_PROMOTED`.

## Stop conditions

No changed paths outside `.integration/proposal5/foundations/proofs/PG01-native-abi/`; verify GitHub compare. Do not merge/admit a capability, touch native runtime, Host effects, root Cargo lock or `main`. Do not confuse a Rust pure engine swap with a trusted end-user plugin installation or background native service. Commit handoff, report exact HEAD, STOP.
