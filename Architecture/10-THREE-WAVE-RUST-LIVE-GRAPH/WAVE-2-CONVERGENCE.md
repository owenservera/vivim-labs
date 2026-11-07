# Wave 2 — CONVERGENCE: Make the graph explain the whole system

**Input:** Accepted Wave 1 live Rust journal, source-pinned adapters, ID crosswalk and actual/target baseline. Preserve one logical graph, one native Rust EngineRegistry and one journal writer.

| Lane | Ownership | Build / change | Acceptance | Accelerates |
|---|---|---|---|---|
| **2A — Incremental ingestion & provenance** | Live HQ→Rust projection adapter | Detect source/KB/action/wiki differences; emit versioned source-owned updates, tombstone only with independent evidence, cursor and stale coverage; use safe import path | One real code edit changes Rust result; unchanged re-run no-op; partial source scan does not erase entities | 3A refresh/monitor |
| **2B — Knowledge/identity convergence** | KB and identity graph mapping | Connect facts, decisions, terms, documents, sessions (privacy-scoped), source hashes, source owners, aliases and revisions to code/engine/command records; preserve observed vs accepted | Query answers `why do we believe this?` and `what version?` without false authority promotions | 3B semantic parity, Oracle |
| **2C — Command/NL/CLI graph** | Semantic command/op read model + engine slot links | One ID per existing operation; map plain-English router, registered action, actual CLI syntax, handler, argument schemas, side effects, permission owner, wired/declared status, versions and transports | Ask `how do I do X?`: exact CLI/who executes/effect status, or NO_MATCH; no simulated handler treated as wired | 3C operational UX |
| **2D — Generated/programmable wiki ↔ Rust** | Wiki generated pages/menus/projection; no new config writer | Pages link to graph entities and command IDs, explain input/output/dependencies, status, missing edges; wiki program preview/activation stays with existing HQ owner | Same object visible via Rust CLI and wiki with matching IDs/revisions; safe preview, no OS/provider execution | 3D live UI/wiki |
| **2E — Engine/dataflow/actual-target topology** | Rust topology query and Labs report | Add typed external engine/dataset/DB/transport/schema/flow descriptors and real cross-repo calls, consumes/produces/writes_to/depends_on/provides edges; distinguish source import from runtime proof | For representative capability show inbound/outbound dependency chain and read/write owner, status + required target delta | 3E real observatory |
| **2F — Cross-surface conformance & audit** | Independent integration test + source coverage audit | Verify Rust importer, HQ/CLI/wiki semantic parity, negative/boundary cases, replay/restart, source conflict, secrets and privilege boundaries; measure excluded/unknown categories | 2–3 real end-to-end examples through code+KB+commands+wiki, evidence and refusal tests; no false integrated claims | 3F release readiness |

### Integration-2 gate
- Graph schema represents code units, semantic facts, actual commands, generated wiki, engine slots, implementations (only if real), databases, dataflows, integrations and evidence.
- Query APIs/CLI answer: where, what, caller/callee, source input/output, writes, authority, implementation vs intention, staleness.
- Reverse dependency traversal identifies impact of a code/schema/command change.
- Generated wiki links the same semantic identity and graph revision; command/NL router uses same typed operation ID and returns refusal on ambiguity.
- Tests prove no duplicate KB or journal writers, no hidden UI/Host effects, no deletion on missing source, and restart/replay.
- All six handoffs with source SHAs, test receipts and actual/target delta.
- Keep real scope coverage percentage separately from link/implementation evidence; 100% of indexed subset is not 100% of entire platform.

### Ownership cautions
External Host/TierFlow/Visual runtime effects remain native; map via read-only typed observations until authorized integration. Avoid automatically promoting source-level call references to proven runtime communication.
