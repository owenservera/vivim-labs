# ARBOR VLANG upgrade — launch index
2026-11-18. Research archive in parent folder. Pinned inspected ARBOR commit 8b3ffe9e2ab90272ea84c17a9f2c10f185ad46ef; subsequent e67c4f81c30e native HQKG sync commit observed (not re-audited). Current main must be rechecked by each worker.

## Launch
Start U1 (HQ indexing), U2 (evidence), U4 (Host approvals), U5 (learning), U6 (Wiki projection), U7 (draft slots) in parallel after checking overlapping callers. U3 (context) can start on its existing composer but finish its dependency wiring against U1/U2. U4 owns dispatcher changes; U5 uses its public planner path, never a second execution path. U2/U6 coordinate wiki-semantics.mjs; U2 owns trust metadata, U6 consumes it. U4/U7 coordinate control-chat.mjs and desktop chat: U4 final approval renderer, U7 draft slots. U1/U2 coordinate existing schema migration; no independently competing migrations. Finish by exercising actual cross-lane callers on main; no separate orchestration project.

## Candidate source
semantic-primitives.mjs contains pure candidate helpers with no state, authorization, parser or effect engine. semantic-primitives.test.mjs passed 7/7 tests in the local container with node --test; Git blobs verified against tested bytes. Tests use tiny fixture adapters, not the ARBOR runtime. SHA-256 candidate domains are not a ratified cross-language VSL/BLAKE3 wire protocol; do not change incumbent digests gratuitously. Integrate only compatible primitives, adapt to actual parsers and data contracts, and add actual product-path tests. ARBOR product, Cargo, device and end-to-end integration checks UNRUN. No changes to ARBOR by this research task.

## Research decisions
- Adopt T2.5 canonicalization principles and T4 split fingerprints only with actual parser evidence.
- T3.5 distinguishes derived rebuildable state from durable learning. Never drop durable user data.
- T5 occurrence is not semantic proof; origin, assurance, freshness and authorization are independent.
- T5 mandatory context is not truncatable; quarantine dependency prose.
- Final T7 binds a round-tripped canonical sentence and exact effect manifest, with version pins, to the incumbent Host plan digest. Host authorization is unchanged.
- T7 reified commands and rule feedback cannot authorize effects; owner acceptance, current binding, and fresh gating are mandatory.
- T6 writable slice is distinct from extracted/derived evidence. Source writeback only via an existing authorized source mutation route; otherwise read-only.
- Defer new SQLite control plane, separate compiler process, generic WASM runtime, Git dependency, API-based provider adapters, MCP/A2A exposure and broad self-modification. They are not prerequisites.
- The raw archives contain conversation metadata and should remain in the private repository; no public publication was performed.
