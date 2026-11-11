# Implementation roadmap — one existing infrastructure, staged integration

## Foundation checks (blocking)
F0. Pin exact HEADs for repos-hq, vivim-web-harness, tierflow and vivim-labs; compare canonical Rust kernel ownership and branch integration. Do not equate source presence with runtime acceptance.
F1. Trace actual Host World/evidence writers, browser captures, knowledge reads, DCB composer, TierFlow viewers. Reconcile with KB-consolidation taxonomy to prevent duplicated work.
F2. Lock source grant, data-retention and user-external-disclosure rules; ensure DATA-007 remains a spec until enforcement is actually implemented.
F3. Prove shared Rust DAG linked to Host use path, domain separation and durable commit; preserve Host authority.

## Vertical slices
S1. **Local import MVP:** user selects one JSON/ZIP conversation export and one document folder; signed source grants, parse and retain evidence, content hash idempotency, checkpoint/restart. No remote provider API.
S2. **Knowledge extraction:** deterministic structure plus bounded AI-candidate extraction, source links, time dimensions, decisions/proposals/outcomes and conflicting claims.
S3. **Living graph:** canonical Personal scoped graph updates, retries/CAS, locally rebuilt indexes, source-level invalidation, query.
S4. **Usable onboarding:** TierFlow screens for source choosing, progress, coverage, unresolved identity matches, source provenance and review. No final-device certification on fixture alone.
S5. **Memory off continuity:** task-scoped personal DCB from authorized graph, preview and destination-specific Host-verified send receipt via existing signed-in browser.
S6. **Incremental learning:** authorized new conversations and local file deltas; event coalescing, work budgets, active task prioritization.
S7. **Right to leave:** pause, revoke, disconnect, delete-derived-content, restore-blocking, portable owner export and audit receipts.

## Team and repository ownership
- repos-hq: Rust graph/DAG algorithms, deterministic typed transformations, development domain parity and documentation of common contracts.
- vivim-web-harness: source onboarding UI authority, local discovery, ingestion workers, personal evidence/World durable state, permissions, browser and DCB delivery.
- tierflow: accessible read-only projections, onboarding and graph explorer for Windows and mobile.
- vivim-labs: provider research, synthetic fixtures, differential fuzz tests and observability evidence. Only design and test assets here.

## Release gates
G1 First useful search and DCB with one selected source, nonblocking bootstrap.
G2 Source-hash duplicate import yields zero duplicate graph claims.
G3 Evidence drill-down shows original supported offsets and correct timestamps.
G4 Conflicting claim remains disputed; user proposal never auto-promoted to implementation.
G5 Permission revoked during batch stops import; cache/derived-content deletion verified.
G6 No sensitive content leaves Host without explicit destination-specific permission.
G7 Crash/restart resumes exact checkpoint without partial graph mutation.
G8 Performance limits at large archives, low disk, small RAM, offline mode.
G9 Browser receipt binds exact authorized thread, not a guessed provider response.
G10 Cross-surface desktop/phone parity and real-device acceptance explicitly recorded, otherwise UNRUN.

## Suggested next source audit output
One `WIRING-MATRIX.md` with source path, ownership, authority, Rust/JS boundary, current tests and classification. One `PROVIDER-ADAPTER-REGISTRY.json` with evidence-based maturity, separate from this broad candidate atlas.
