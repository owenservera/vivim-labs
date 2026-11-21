# The execution problem: does the actual built program behave as specified?
## Three distinct things often conflated
1. **Source program:** pinned Git bytes, typechecking, source AST, dependencies and contracts.
2. **Compiled/bundled artifact:** actual JS module graph, Rust binary, Tauri executable, PWA asset manifest with artifact SHA, lockfile/toolchain versions and build flags.
3. **Running outcome:** an observed invocation with input/environment/world revision, visible output/error, allowable effects, exact actor/grants and receipt. A source pin alone does not prove the executable deployed.

## Verification loop for a function
1. Read pinned reviewed `ClaimSpec`, including preconditions and explicit effect policy. Refuse an arbitrary JS/shell assertion in a comment.
2. Resolve actual implementation/symbol owner from pinned build/source map; ensure source pin and compiled artifact link in a trusted build provenance receipt.
3. Produce independent **oracle inputs**: hand-authored examples, boundary/negative cases, generated property tests, mutation/killed-mutant probes, metamorphic relations (e.g. canonical order invariance), model transitions where bounded.
4. Run approved read-only/pure implementation in isolated resource-bounded worker. Capture inputs, exact output/error, code path/coverage, resource limits and structured event trace. No ambient network, secrets, provider Chrome or personal World.
5. Check postconditions against the oracle, not the compiler's paraphrase. Counterexample automatically refutes that scoped claim and provides minimal replay.
6. For Host control, Studio sends and UI publication, **do not re-execute real-world side effects just to verify**. Instead verify existing domain-owned receipts, idempotency/nonce safety, closed authorized observation ports and actor/grant provenance. External delivery acknowledgment is different from local send journal.
7. Produce a per-property receipt with `observed`, `not_observed`, `incomplete` and declared assumptions. Never conclude `no effect` from logs that might miss an effect path.

## Observation contracts needed at boundaries
- **Function pure read:** input/output/errors plus stable state snapshot before/after if purity is claimed; static closure/FFI coverage required for stronger statement.
- **Rust native HQKG:** engine version, input DAG, pure result digest, cancellation/resource ledger, no Host effect; bounded model proof where possible.
- **Host:** accepted action ID, actor/session, grant/catalog/world revision, preview/approval, claim journal event, effect result/uncertain outcome, receipt ID. Verifier only reads already-owned journal.
- **Studio/provider:** sender-owned operation ID, attempted send, provider-observed response/ack, uncertain/unknown; do not infer recipient/delivery from local ACK.
- **TierFlow:** typed UiStore revision, input command, accepted publisher revision/rollback receipt; display projection is not Host effect.
- **PWA:** signed device/grant scope, source revision, request nonce/replay status and Host response; a source-only module route does not prove device security.
- **Wiki/generated explanation:** source/IR/spec digests, template ID, claim IDs and factual clauses; human prose is a projection, not runtime witness.

## Limits and counterexamples
A test may prove **a particular execution** behaved as specified. A finite test suite supports but does not universally prove an unrestricted implementation. A model checker can prove a bounded finite transition system *within its model*; it does not establish that the actual unmodeled native binary is equivalent. Static effect analysis can conservatively flag potential writes but cannot generally exclude reflection, native extensions or external processes. Instrumentation completeness must be explicit and independently audited.
