# W1 — launch six parallel lanes, then G

**Wave objective:** CANONICAL BACKEND SPINE.

Start six ChatGPT/GitHub conversations in parallel using the exact linked worker prompts, all from accepted W0 gate head.

- [A — Schemas and validators](./A.md): Create versioned descriptor/account/stream/content/action/receipt projection schemas, runtime validators, sample golden fixtures and strict unknown-value refusals.
- [B — Hub registry and service](./B.md): Build one injected-port ProviderHub supporting list providers/streams/read thread/search/propose action/status, capability gating and bounded paging.
- [C — Fake future World port](./C.md): Implement typed fake World grants/evidence/commit receipts and an interface ready for later existing Host World; never create durable competing World.
- [D — Fake Host action boundary](./D.md): Implement inert action-plan/readback/approval and mock receipt adapter. No direct send; later live actions must use existing Host dispatcher.
- [E — Mock Rust graph interface](./E.md): Build source/ref graph proposal adapter and deterministic fake that obeys personal namespace, CAS revisions, provenance; no graph store.
- [F — Seeded source simulators](./F.md): Create deterministic fake StudioSourcePort and seeded 10-provider-safe synthetic data, fault scripts, clock, receipt markers and no live network.

After all six commit and supply reports, run [G serial integration](./G.md). Do not assume worker completion or tool success; use actual branch heads.
