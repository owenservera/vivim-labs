# 07 — Compilation, encoding, decoding and verification laboratory
## A unified efficiency question
**Can we reduce representation changes without weakening meaning, verification or security?**

```text
User NL / source bytes
   ↓ Parse / Decode (bounded, untrusted)
Typed intermediate meaning / event records
   ↓ Normalize / Optimize (pure, versioned, reversible)
Verified IR / Source provenance
   ↓ Validate (risk, schema, identity, source, actor)
Host-approved request or inert UI projection
   ↓ Render / Persist / Replay with receipt
```

## Grounded surfaces
| Function | Source | Existing guarantee | Optimization hypothesis |
|---|---|---|---|
| Studio protobuf decoder | `studio/src/decode/protobuf.mjs` | schema-less framing, strict UTF8, ambiguity and bounds | avoid temporary byte copies; vectorized bounded parsing only if parity preserved |
| Studio delta assembler | `studio/src/decode/delta.mjs` | multiple JSON patch dialects, replay and snapshots | incremental structural sharing, adaptive snapshot interval |
| TierFlow command compiler | `packages/tierflow/src/visual-runtime/command/compile.ts` | bounded input, closed descriptors/IR | cache descriptor resolution by immutable revision; reject stale cache |
| TierFlow document validator | `packages/tierflow/src/visual-runtime/document/validate.ts` | bounded structure, 5000 visits/24 depth | reject early without changing canonical refusal or unsafe-input detection |
| Host NLCL | `apps/vivim-host/control/command-intent.mjs` | typed intent path with Host ownership | grammar-index and parse reuse while minimizing false-positive intents |
| Rust pure programmability | `core/programmability/src/lib.rs` | pure bounded transform candidates, no effects | ABI-friendly typed data and fewer clones, no authority changes |
| Rust typed DAG | `native/hqkg/src/typed_dag.rs` | deterministic computation, namespace cache and cancellation | cache/invalidation, fewer allocs, explicit versioned digest |
| TierFlow hotpath | `packages/tierflow/src/hotpath.ts` | NDJSON events aggregated via sidecar/fallback | choose framing only after bytes/cpu benchmark |

## The compiler/verifier optimization law
Any rewrite needs the **same semantic outputs, same refusal classes, same source pins, same actor/grant boundary, and same executable negative-test behavior**. Faster results that accept more untrusted inputs are regressions. Variable-length/UTF8 malformed bytes, Unicode normalization, cycle/sparse arrays, unsafe JS getters, replay, partial graphs and revoked grants are first-class adversarial fixtures.

## Ideas for future experiments
- IR interning and structural sharing for repeated validated subtrees.
- Parse once, derive several typed projections from the same frozen verified representation.
- Zero-copy byte views for safe codec spans with lifetime and ownership constraints.
- Cache compiled pure representations keyed by **schema version + source digest + namespace + policy revision**.
- Canonical hash serialization cost vs content-addressed stable spans.
- Early structural refusal before expensive semantic verification, with full negative parity.
- Streaming incremental verification that stops on precise limit, never outputs partial accepted authority.
- Optimized Rust pure transforms only if FFI/IPC cost, build size and rollback are favorable.

## Verifier is not a bottleneck by definition
Start with a per-stage flame graph: decode, parse, normalize, optimize, verify, authorize, serialize, send and render. If verification is 2% of latency, rewriting it is almost surely wrong focus. If it is 60%, profile inputs and repeated work before changing invariants.

## Proof artifacts needed before any promotion
Synthetic corpus and adversarial cases; golden IR and refusal snapshots; exact runner and exit; mutation/fuzz coverage; maximum bytes/nodes/time; verified hash and version; baseline and variant latency, allocation and RSS; ABI/serializer parity; rollback plan. A test file in Git isn't an executed proof.
