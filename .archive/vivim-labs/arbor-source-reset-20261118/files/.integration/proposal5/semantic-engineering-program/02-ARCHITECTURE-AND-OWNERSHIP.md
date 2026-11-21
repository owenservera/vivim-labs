# Architecture and authority
```text
Agent/IDE source edits → untrusted @hq: declarations → HQ v1/v2 + proposed v3 parser
  → existing HQ Registry + semantic compiler → typed SC-ABI source facts
                                              ├→ existing index/HQKG/wiki
                                              └→ Independent SV verifier
             owner-reviewed specs/oracles ─────┘     |
             safe bounded test runners/artifact pin──┤
             read-only Host/Studio/TierFlow receipts─┘
                                      ↓
                          ProofReceipts and verdicts
                                      ↓
             checked verbal explanation → Wiki / HVI / agent / SDK / UI
```
Separate trust boundaries: coding agent/source comment is **untrusted hypothesis**; specification/oracle acceptance is independently reviewed; compiler may not certify its own parse; verifier source attachment rederived independently; tests run in bounded isolated context; actual effect evidence comes from existing Host/Studio/UI owner, not newly executed side effects; explanations bind each factual clause to a checked typed claim and exact code/build revision.

**Invariants:** HQ semantic registry/index/wiki only one owner; Rust HQKG pure compute one owner; Host effect/World one owner; Studio sender one owner; TierFlow publisher one owner; existing agent capability registry reused. No second scheduler, graph/KB/DB, source identity registry, action dispatcher or privileges from source metadata. No proof of "never writes" from partial telemetry, "always correct" from a finite test suite or provider delivery from local ACK.

Trust tiers: source SHA/build/actor/spec identity → compiler-derived facts → independent claim/runner proofs → domain-origin observed receipts → disclosed checked explanations. Revision invalidation affects every dependent proof/passport; stale receipts never silently promoted.
