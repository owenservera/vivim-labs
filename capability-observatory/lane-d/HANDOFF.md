# VIVIM Wave C4 - Lane D Handoff

Owner: Labs Observatory only. Write path: `capability-observatory/lane-d/**`. All other paths read-only. This delivery does not assert four-lane integration.

## Evidence pinned before edits

- Labs main `80319c441be4ffbd8ca0bd9b2cb5af2224a92910`, seed 10 capabilities, dev 10, prod 0, sources 5 (1 unlocated).
- Repos HQ main `6679752362a5ef58aa66e2af5463f9d81077001c`.
- TierFlow main `cb16d2a85314879409a5304dc97267bf36d52b71`; final master integration Gate 5 BLOCKED, Gate 6 BLOCKED; A-H composed with fixture evidence, native/device/Host acceptance UNRUN.
- Harness main `09382c61f74a304f120b05445f6dfa900b58494b`.
- No Labs AGENTS.md present at root or capability-observatory (full tree checked); owning AGENTS in HQ, TierFlow, Harness reviewed read-only.

## Implemented

- Strict versioned HQ export adapter; provenance-preserving many-to-many crosswalk; tombstone, confidence, conflicts, duplicate/orphan findings and missing-source states.
- Fail-closed production certification; separate dev/target/fixture/live evidence horizons.
- Privacy-redacted, content-addressed and atomically refreshed local snapshots, optional watch mode, measured/unmeasured counts and source age.
- Standalone interactive, read-only dashboard; seven local model tests; loopback-only file server.

## Explicit unknowns / next integration

1. HQ Lane C must publish the negotiated export envelope, not a second knowledge store. Current adapter schema is a **proposed profile**, not evidence that HQ already produces it.
2. Lane A must supply measured source catalogue counts; B must supply authoritative dedup/orphan clusters and review decisions. Until then, duplicate and orphan counts are UNKNOWN, not zero.
3. Integration owner validates export shape, path and SHA pinning, replays `refresh.mjs --hq ...`, runs tests and verifies removal/missing-source behavior. Avoid editing this lane or Lab seed in parallel without a fresh-head CAS.
4. Physical Windows/Android, Host, consent, deployment and user-reachability receipts remain absent. Production ledger stays zero, no activation attempted.

## Tests

`node --test capability-observatory/lane-d/tests/observatory.test.mjs` - **7/7 pass** (fixture-based, local Node). Includes missing HQ, provenance, source counts/age, private redaction, tombstones, schema fail-closed, ageing/replay, receipt gating.

Integration verdict for Lane D alone: **IMPLEMENTED / LOCAL TEST PASS / HQ CONNECTION BLOCKED until compatible pinned export**. Overall C4 verdict must be supplied by final integrator, not by this lane.
