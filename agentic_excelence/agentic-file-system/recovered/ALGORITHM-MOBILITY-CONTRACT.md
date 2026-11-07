# Deterministic movable-algorithm graph — protocol v0.1

## Three eras of an algorithm

**Definition:** immutable identity and version of a pure function or an explicit runtime-owned action adapter. Stored once with input/output schemas, determinism promise, owner, implementation hash and test evidence.

**Placement:** graph edge describing where the algorithm *could be useful*. A placement binds an algorithm version to a target section/task/subgraph, scoped task/context, runtime owner, policy, token/time budget, deterministic rank, and expiry.

**Activation:** a lease created only after deterministic gate validation and the responsible runtime's authorization. HQ itself only computes/records a proposal or validation; it does **not** send messages, approve, manipulate browser sessions, or execute provider actions.

### Directed transitions

`proposed -> validated -> active -> expired`, with `revoked` or `rejected` as terminal safety transitions. `replaced` records a successor placement. Every transition is an append-only event, never a silent rewrite. A moved algorithm is **new placement + supersession**; the stable Algorithm definition remains in place.

### Selection function

1. Normalize task and node identifiers with a versioned grammar (existing HQ A01/A02 + NLCL owner, no new parser).
2. Candidate retrieval by exact identifiers, aliases, FTS5/BM25 and graph neighborhood. Optional embeddings may nominate candidates but never grant authority.
3. Hard constraints: compatible input/output schema, authorization boundary, source policy, freshness, required data, token/time budgets and non-conflicting lease.
4. Stable relevance score: `score = sum(weight_i * measured_feature_i)`; absent measurements remain unknown, not zero-quality evidence. Resolve ties by algorithm ID + version.
5. Choose a bounded set using coverage-gain-per-cost (existing HQ `packContext`) and bounded task assignment (`allocateTasks`). Dijkstra/A* only for explicitly weighted nonnegative traversal, not arbitrary semantic resemblance.
6. Emit a dry-run placement plan with source hashes and receipts. The runtime owner validates authorization independently before any effect.
7. After execution, compare expected vs observed via `detectDrift`; preserve contradictory evidence with `reconcileClaims`, revise scores, and expire the lease.

### Stable graph node/edge vocabulary

Nodes: Repository, File, FileType, SectionType, SectionVariant, SchemaAsset, TaxonomyTerm, Concept, Capability, Algorithm, AlgorithmPlacement, Agent, Task, RuntimeOwner, Policy, Evidence, Receipt, Test, Version.

Edges: CONTAINS_FILE, HAS_SECTION, INSTANCE_OF, CLASSIFIED_AS, CITES, IMPLEMENTED_AT, TESTED_BY, REQUIRES, REFINES, CONTRADICTS, SUPERSEDES, APPLIES_TO, PROPOSED_FOR, PLACED_AT, APPROVED_BY, OBSERVED_IN, DERIVED_FROM.

Only evidence-backed edges may move from proposed to validated. Preserve source and scope for every relationship.

### Scalability and determinism

SQLite + indexed adjacency tables + optional FTS5 first; typed edge grammar and bounded recursive traversal. Existing HQ graph/indexer remain canonical; this SQLite DB is an isolated projection for contract testing. Donor content cannot become operational agent instructions by being inserted into the graph.

### Verification gates

- All 217 file IDs unique and source hashes preserved.
- No dangling edges, no source-kind/graph-type collisions, and no unexplained missing section types.
- Every scored feature retains its scorer version; provisional filename-only metrics must never drive autonomous deletion.
- No placement reaches active without schema validation, authorization receipt and correct runtime ownership.
- Deterministic reruns yield equivalent sorted node/edge outputs and stable content digests.
- Provenance, conflicts, and supersession decisions remain navigable.

## External reference basis

[Official Agent Skills specification](https://agentskills.io/specification), [SQLite FTS5](https://www.sqlite.org/fts5.html), [AGENTS.md convention](https://agents.md/), [existing HQ optimization algorithms](../../docs/research/semantic-language-kernel/OPTIMIZATION-ALGORITHMS-V1.md). These references are design guidance, not an external permission to change live runtime behavior.
