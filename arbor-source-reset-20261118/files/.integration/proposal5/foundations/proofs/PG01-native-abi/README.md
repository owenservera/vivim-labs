# FA-PG01 — Native programmability / engine ABI source proof

**Disposition: PREPARED_NOT_PROMOTED.** Isolated proof only. No product edits, feature install, G integration, release or native Host execution.

## Verified source coordinates
- Foundation parent HEAD: **80c29fbc0d45afbce0fadde5388a8f912cdd2762**; created worker branch **work/p5-fa-pg01-native-abi-r001** from that immutable SHA.
- Frozen A/B/C/D/E/F and N0/P2-C and Labs Node0 current HEADs are pinned in ENGINE-ABI-CROSSWALK.json. A–F ref heads matched shared-contract authoring pins when inspected; all six PREG reports contain FREEZE_READY.
- G branch integrate/proposal5-beta-r001 was **absent (404)** at inspection. This does not prove future integration status.
- File-level immutable source blobs: see SOURCE-DELTA.json and ENGINE-ABI-CROSSWALK.json. The Rust code in proof/rust/ uses names inspected in A's actual engine.rs, typed_dag.rs and core/programmability/src/lib.rs, not a fabricated plugin runtime.

## Measured actual source semantics
1. Registry install **requires previous generation equal expected** (None only when empty), checks nonempty slot/interface/implementation/capabilities, refuses interface change and capability removal, and increments generation using checked_add. EngineLease holds Arc<Generation>; existing pinned leases remain bound to old engine even after a new install. SwapReceipt only carries slot/previous/generation.
2. **There is no source admission or attestation** in EngineRegistry::install. The original implementation is not a cryptographic digest. Any trusted Rust caller can supply a ComputeEngine; pure interface name is a social/integration promise, **not a sandbox**. Never load arbitrary unreviewed code or claim manifest-granted capabilities.
3. Scheduler checks task, elapsed and output limits, but synchronous execute cannot be interrupted. Cancellation is checked at boundaries and after execution. A hung or malicious engine can stall; runtime isolation/resource fencing is an unresolved owner decision. The engine's execute method itself has no input byte bound.
4. Typed DAG validates namespaces, step dependencies, source revision/digest shape and cache task/output budgets. It uses in-memory, deterministic bounded eviction and staged all-or-nothing cache commit on Complete. SourceBinding.digest/provenance and Graph provenance are **caller assertions**, not verified source bytes or Host read permission. A mismatched graph revision exits through Err before a RunState report. After initial lease pin, DagExecutor dispatches through Scheduler, which pins anew; generation races are caught as StaleGeneration, not guaranteed old-lease execution.
5. The DAG carries raw ephemeral MaterializedOutput.data/input_stamp and provenance in memory. durable_dag emits **redacted computation descriptions** in the existing HQD1 journal, **not** Host approval/effect receipts. DagReport.effect_authorized is false.
6. Programmability Program/PolicyLimits validate inert DAGs, risk, taint and destinations; ValidatedProgram.execution_authorized and effect_performed remain false. EffectCandidate is never an action grant.
7. F's Rust/TS structural SourceRef and N0's input source vocabulary are not fully equivalent to A Namespace/RevisionToken (Development/Personal vs four F/N0 domains; engineVersion string vs generation u64). N0 and Labs describe fan-out but are **not** a running native producer system.

## Artifact index
- SOURCE-DELTA.json: reviewed source/contract provenance with owner classification.
- ENGINE-ABI-CROSSWALK.json: actual -> proposed wire ABI, trust gap and fan-out data flow.
- rust/pg01_engine_abi.rs: additive HQKG Cargo integration tests for install, lease pinning, CAS/ABI refusal, reviewed rollback, DAG budgets, cancellation/namespace/source revision and a clearly fixture-only stale preview gate.
- rust/pg01_program_abi.rs: pure programmability crate tests for effect candidate nonauthority, stale policy and invalid digest.
- PROOF-PLAN.md: exact commands in an **isolated temporary A checkout**, expected outcomes and follow-on G staging.
- NEGATIVE-ACCEPTANCE.md: failures and unimplemented security gates.
- OWNER-HANDOFF.md: five path-bounded owner patches after G acceptance.
- RUN-LEDGER.json: observed GitHub operations and explicit UNRUN toolchains.

## Execution truth
GitHub source read + branch creation: **OBSERVED**. Cargo/rustc are **not available** in the current container; **all cargo tests, cargo check, rustfmt, cross-language parity, Windows/PWA/device acceptance are UNRUN**. The two Rust test files are candidate source-compatible harnesses, **not compile-verified**. No privileged runtime, credentials, exports or user-private data were touched. This is a proof of interface *feasibility* and precise missing contract, not a safe installed plugin or admitted feature.
