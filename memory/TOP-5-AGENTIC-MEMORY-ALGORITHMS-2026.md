# Agentic memory: five high-value algorithms and their VIVIM fit

**Research snapshot:** 2026-11-07  
**Status:** research/design recommendation only — no runtime implementation claimed  
**Target:** VIVIM Labs / memory  
**Scope:** persistent personal/agent memory, temporal history, retrieval, associative discovery, and active-context management.

## Executive decision

The best starting point for VIVIM is **not** to pick a single memory product. Combine five complementary mechanisms on top of the existing Rust live graph and keep algorithms swappable:

1. **Mem0 (2026)** — append-only salient fact extraction plus hybrid, entity-aware, time-aware retrieval.
2. **Graphiti / Zep** — bitemporal fact and relationship invalidation, provenance, historical truth.
3. **HippoRAG 2** — query-seeded graph association using Personalized PageRank (PPR).
4. **A-MEM** — Zettelkasten-style semantic note creation, linking, and evolution.
5. **Letta / MemFS** — hierarchical memory paging, Git-backed versioning, selective context loading.

These are five **research-backed, implemented approaches**, not the five statistically most-used algorithms. There is no reliable, public cross-industry usage census. Numbers published by vendors are not directly comparable across different evaluation setups. A product may package multiple algorithms. The ranking here is by likely architectural utility to VIVIM, not claimed market share.

## Comparison matrix

| Priority | Algorithmic mechanism | Reference | Read/write role | Main upside | Risk / trade-off |
| --- | --- | --- | --- | --- | --- |
| 1 | Salience extraction + append-only memory + multi-signal fusion | Mem0 (2026) | Write + read | Preserves events; compact persistent facts | Duplicates, growth, retrieval complexity |
| 2 | Temporal fact graph, contradiction resolution, valid/recorded time | Graphiti | Write + read | “What is true now?” and “what was true then?” | Expensive entity/edge resolution, temporal bugs |
| 3 | Query-seeded graph propagation / Personalized PageRank | HippoRAG 2 | Read | Multi-hop associative recall | Graph quality, high-degree noise |
| 4 | Dynamic semantic linking and memory-note evolution | A-MEM | Write/consolidation | Organic memory relationships and reflection | Link drift, unnecessary writes, hallucinated edges |
| 5 | Hierarchical paging, index files, selective context, version control | Letta MemFS | Context assembly | Bounded prompts, audit/history | Search/index integration and stale context |

### 1. Mem0: extract salient memories and retrieve by multiple signals

**Underlying algorithm:** detect durable facts/decisions/preferences/action outcomes from a stream; append compact memories; index semantic meaning, terms, entities, and time; fuse multiple retrieval signals.

**Important version distinction.** The 2025 Mem0 paper describes extraction followed by an LLM selecting ADD, UPDATE, DELETE, or NOOP after searching semantically similar memories. In its **April 2026 managed-platform algorithm**, Mem0 documents single-pass **ADD-only** extraction with no automatic overwrite, entity linking, parallel semantic/BM25/entity retrieval, and temporal ranking. Those are not the same design. The advanced managed implementation includes proprietary optimizations and is not identical to the OSS SDK.

**VIVIM adoption:** use the ingestion pattern, not necessarily its SDK or external API. Extract facts/events locally through the Host-approved model path; append immutable evidence events; deduplicate logically in a materialized view; preserve user explicit deletes and security-policy erasure separately. Rank with lexical, dense, entity, and temporal signals. In the first pass prefer deterministic policy gating and explicit provenance over fully automatic memory edits.

**Inputs:** conversation turns, user declarations, agent-verified actions, observations, documents.  
**Outputs:** compact memory candidates, evidence references, ownership/sensitivity metadata.  
**Initial validation:** repeated restatements do not overwhelm results; corrections do not surface stale current facts; deleted memories never reappear.

**Sources:**  
- https://github.com/mem0ai/mem0 (April 2026 algorithm notes and product/OSS distinction)  
- https://github.com/mem0ai/mem0/blob/main/docs/core-concepts/memory-operations/add.mdx  
- https://arxiv.org/abs/2504.19413 (2025 original algorithm)

### 2. Graphiti: bitemporal knowledge graph and evolving truth

**Underlying algorithm:** entities + relationships + source episodes, with **two clocks** for each fact: its real-world validity and its recorded/learned history. As new evidence contradicts older evidence, close the old fact's validity window without pretending it never existed.

A commonly used fact/edge vocabulary is:
- `valid_at`: when the assertion began applying in the world;
- `invalid_at`: when it stopped applying in the world;
- `created_at`: when the system recorded it;
- `expired_at`: when the system recognized/superseded it.

**VIVIM adoption:** make bitemporal semantics a native Rust-graph contract. Keep source/episode IDs, user/project scopes and truth status on graph assertions. Offer current-as-of-world-time and known-as-of-system-time queries. Use explicit evidence and confidence when two claims cannot safely be reconciled. Prefer materialized latest-state views over erasing history. Handle prospective end dates correctly.

**Inputs:** facts with source and date; corrections, new observations.  
**Outputs:** temporal assertions with provenance and current/historical views.  
**Validation:** late-arriving corrections; future-dated end times; overlapping intervals; contradictory sources; time-travel queries.

**Implementation caution:** an October 2026 Graphiti issue describes future `invalid_at` being incorrectly treated as immediately expired in an affected version. Treat prospective timestamps as a mandatory regression test, not a design assumption.

**Sources:**  
- https://arxiv.org/abs/2501.13956  
- https://github.com/getzep/graphiti  
- https://help.getzep.com/facts  
- https://github.com/getzep/graphiti/issues/2012

### 3. HippoRAG 2: associative graph recall with Personalized PageRank

**Underlying algorithm:** connect entities/phrases and source passages into a knowledge graph; seed the graph with entities associated with a question; propagate importance through graph links; retrieve high-scoring evidence for multi-hop reasoning. HippoRAG introduced the PPR mechanism; HippoRAG 2 extends it as a non-parametric continual-memory framework.

**Core recurrence (conceptual):** p_next = (1 - d) * seed + d * transition_transpose * p.  
Here d is a damping factor and seed is a query-specific probability distribution over relevant graph nodes. Apply convergence/iteration limits, bounded neighborhoods, and robust score normalization.

**VIVIM adoption:** expose PPR as a plug-in Rust read-side graph algorithm, with entity-seeded retrieval and source-path explanations. Use it when ordinary dense/BM25 retrieval misses a relationship distributed across conversations or files. Cache **results**, not durable truth, under cache keys incorporating graph revision, scope, permissions, and query signature.

**Inputs:** query entity candidates; permitted typed graph neighborhood.  
**Outputs:** associated nodes/edges, retrieved episodes, explanatory paths.  
**Validation:** two- and three-hop questions; distracting hub nodes; incremental graph updates; scope leakage.

**Sources:**  
- https://proceedings.neurips.cc/paper_files/paper/2024/hash/6ddc001d07ca4f319af96a3024f6dbd1-Abstract-Conference.html  
- https://github.com/OSU-NLP-Group/HippoRAG  
- https://arxiv.org/abs/2502.14802

### 4. A-MEM: self-organizing semantic memory notes

**Underlying algorithm:** convert a memory event into a note with a summary/context/keywords/tags; search candidate related notes; create meaningful links; revise note metadata or interpretations in light of newly added memories. Derived from the Zettelkasten approach.

**VIVIM adoption:** make note-generation and cross-linking an optional **learning/consolidation worker**, not an authoritative rewrite of raw evidence. Link evidence-backed topics, tasks, agent procedures, preferences, and discovered relationships. Store computed annotations separately from the original immutable event. Human/user corrections remain authoritative. Guard against semantic drift and endless relinking cycles.

**Inputs:** committed memory records and candidate graph neighbors.  
**Outputs:** derived memory-note index, suggested links, confidence-scored metadata revisions.  
**Validation:** meaningful-link precision, link stability after ingestion, reversibility, evidence-backed attributes.

**Sources:**  
- https://papers.nips.cc/paper_files/paper/2025/hash/19909c36f51abc4856b4560aff3d36d6-Abstract-Conference.html  
- https://github.com/WujiangXu/A-mem  
- https://github.com/agiresearch/A-mem

### 5. Letta / MemFS: hierarchical memory paging into active context

**Underlying algorithmic pattern:** treat the context window as scarce working memory. Keep core memory and a concise navigation index readily available; place deeper facts/projects in files or blocks loaded on demand. Commit edits as versioned state. This is a memory-management architecture, not a single new mathematical ranking algorithm.

**2026 implementation:** Letta's MemFS represents agent memories as Git-backed Markdown files. Root memory files are loaded into prompt context; nested directories with `MEMORY.md` indexes can remain out of context until requested. Semantic search is **not included by default** in MemFS; an optional search mod adds lexical/semantic/hybrid modes subject to dependencies. This differs from describing Letta only as the original 2023 MemGPT tiered core/archival model.

**VIVIM adoption:** integrate this *pattern* with the dynamic context-bundle compiler: minimal core identity/operating policy always present; retrieved memory facts and project knowledge on demand. Build context deterministically from scope + budget + provenance. Git-style version history can be applied to editable memory documents without treating Git as the hot query engine.

**Inputs:** bounded query/request context, eligible memory candidates, budget, permission state.  
**Outputs:** assembled context bundle plus source IDs, token cost, omission reasons.  
**Validation:** token cap, accurate citation, repeatable compilation, restart persistence, context invalidation on revocation.

**Sources:**  
- https://github.com/letta-ai/letta-docs-md/blob/main/concepts/memfs/index.md  
- https://github.com/letta-ai/letta-code/blob/main/src/agent/prompts/letta_root_memfs.md  
- https://github.com/letta-ai/lettabot/blob/main/docs/configuration.md

## Combined VIVIM design

Design intent only; **not** a claim that these flows are wired in existing repositories.

    Signed-in provider web UI / Host-approved local action / user edit
       -> event + provenance + scope + consent
       -> extraction candidate (Mem0-inspired)
       -> durable evidence/event log (versioned, revocable)
       -> bitemporal graph projection (Graphiti-inspired)
       -> semantic note/link worker (A-MEM-inspired, non-authoritative)
       -> lexical + dense + entity + temporal indexes
       -> query retrieval: hybrid + graph PPR (HippoRAG-inspired)
       -> relevance/diversity/evidence reranking
       -> scope and token-budget context compiler (Letta-inspired)
       -> Oracle / dynamic context bundle / accountable action

The **Rust graph is the shared identity/relationship computation layer**; SQLite or other pluggable persistence provides durable records and indexes. Do not introduce an independent conflicting 'personal-memory graph' unless there is a proven isolation requirement. Algorithms are hotswappable nodes/adapters.

### Minimal canonical event/fact contract (proposed)

- `memory_id`, `event_id`, `source_ref`, `source_type`, `source_revision`, `content_hash`
- `owner_id`, `namespace`, `project_id`, `agent_id`, `scope`, `visibility`, `consent_policy_id`
- `subject_id`, `predicate_id`, `object_id_or_value`, `fact_type`, `evidence_span`
- `observed_at`, `valid_at`, `invalid_at`, `created_at`, `expired_at`
- `confidence`, `importance`, `sensitivity`, `retention_policy`, `supersedes`, `revocation_id`
- `embedding_model_version`, `extractor_version`, `graph_revision`, `schema_version`

Keep assertions, raw evidence, derived links, and compilable memory summaries as different object types. Do not equate a retrieved claim with authorized action. The owner controls memory scope and deletion; append-only system design must **not** obstruct complete user-directed erasure.

### Retrieval scoring (starting hypothesis, not benchmarked weights)

Use separately calibrated retrieval channels:

- Lexical BM25 for exact names, IDs, commands and rare technical tokens.
- Dense semantic similarity for paraphrases and concept matches.
- Entity / relationship matches for subjects, objects and projects.
- Temporal validity and request-time relevance.
- PPR/graph score for multi-hop associative evidence.
- Provenance, permission and source confidence as hard gates or subsequent rerank features.

A robust first fusion baseline is Reciprocal Rank Fusion: `RRF(candidate) = sum_channel 1/(k + rank_channel(candidate))` with a configurable k. Then apply an optional Maximal Marginal Relevance diversity pass to avoid near-identical memories occupying the whole context. Add source-citation and token-cost checks **before** assembling the dynamic bundle. Never blend scores from unrelated distributions without calibration.

**LRU** belongs to *ephemeral retrieval, embedding, and compiled-context caches*. It is **not** a forgetting or importance algorithm and must never silently delete durable memories. Cache key: owner/scope + permission revision + index/graph revision + query signature + model/embedding version. Invalidate on writes, revocations, retention policy changes, and graph updates.

### Folder ownership boundaries

| Labs folder | Owns | Explicitly does not own |
| --- | --- | --- |
| `memory/` | durable event/fact memory, indexes, retrieval, temporal histories, memory evaluation | active prompt assembly |
| `context/` | dynamic context bundles, token budgeting, retrieval-to-prompt compilation, short-term caches | canonical long-term truth |
| `learning/` | memory consolidation, reflection, link proposals, derived procedures, correction feedback | permission to rewrite raw evidence |
| `personalization/` | owner-approved preferences, identities, adaptation policies and profiles | unbounded inference about the owner |

**Oracle** orchestrates across all four but remains subject to the owner's permissions, revocations, and Host-owned action contracts. The memory system is passive for a user request unless an authorized route explicitly asks it to act.

## Experimental plan — compare alternatives, not vendor marketing

### Control conditions

A. Full recent transcript / fixed window (when feasible).  
B. Vector-only semantic retrieval.  
C. Lexical + dense hybrid retrieval.  
D. C + temporal graph.  
E. D + query-seeded PPR.  
F. E + A-MEM link evolution.  
G. F + hierarchical dynamic context compilation and revision-keyed LRU caching.

Run the same event history and model stack across every condition. Fix random seeds, retrieval budgets, owner scope, source-citation rules, and model versions. Report generation correctness **separately** from memory retrieval recall; do not hide write-side cost or inference traffic.

### Scenario suites

1. **User preference revision:** preference A -> preference B; ask what is true now and what was true before.  
2. **Across-project multi-hop:** answer requires linking at least two episodes and one graph relationship; confirm evidence path.  
3. **Agent workflow experience:** remember an environment-specific pitfall, previous failed action, and verified repair steps.  
4. **Contradiction + out-of-order evidence:** old observation arrives late; add prospective temporal end date.  
5. **Security + memory erasure:** revoke a provider/session/memory and ensure no exposed bundles, caches, indexes, search results, or generated summaries retain access.  
6. **Scale and context pressure:** 1k, 10k, 100k fact/evidence items; monitor latency and answer quality as memory grows.  
7. **Persistence/restart + graph revisions:** repeat queries before/after restart, re-index, model change, and incremental updates.

### Metrics and gates

- Retrieval Recall@k, MRR/nDCG, source-evidence precision and multi-hop path correctness.
- Final-answer factual accuracy; temporal correctness; update correctness; abstention when evidence is missing.
- Scope isolation and revoked-memory leakage: **zero permitted**.
- P50/P95 ingest, search and end-to-end context compile latency.
- Storage growth, duplicate ratio, graph-edge/link precision, index freshness and cache hit rate.
- Tokens per answer and cost of extraction/consolidation/retrieval; number of tool/model calls per operation.
- Reproducibility after restart and number of stale results after an index/permission revision.

**Primary evaluation references:**
- LongMemEval (ICLR 2025): extraction, multi-session reasoning, knowledge updates, temporal reasoning, abstention. https://proceedings.iclr.cc/paper_files/paper/2025/hash/d813d324dbf0598bbdc9c8e79740ed01-Abstract-Conference.html
- LongMemEval-V2 (May 2026): 451 questions targeting agent environment state, workflows, gotchas and premise awareness. https://arxiv.org/abs/2605.12493 and https://github.com/xiaowu0162/LongMemEval-V2
- LoCoMo long-conversation memory is another useful benchmark; match the published setup before making numerical comparisons.

## Engineering rollout: three eras

**Era 1 — deterministic baseline:** define immutable evidence/provenance and bitemporal schema; extract with authorization; BM25+dense retrieval, valid-time filtering, revision-aware LRU, contextual source receipts. Compare to vector-only and transcript baselines.

**Era 2 — graph intelligence:** entity resolution and query-seeded PPR from existing Rust graph; explanation paths, confidence, relation-aware reranking; test contradictory/late evidence.

**Era 3 — memory evolution and Oracle:** optional A-MEM reflection/links; hierarchical context compilation; owner-governed long-term preference handling; benchmark multi-agent sharing, learning, revocation, restart, and scale.

## What not to assume

- The five examples are not a universal 'top 5 by usage'. They demonstrate five useful, different algorithms/patterns with available research and implementations.
- The Mem0 managed platform's April 2026 benchmark figures are vendor-reported and may rely on unavailable proprietary optimizations; they are **not** VIVIM predictions.
- A-MEM and HippoRAG paper gains apply to their evaluated tasks, not automatically to personal-agent memory.
- A Git-backed memory folder is not automatically a low-latency semantic retrieval engine.
- Temporal invalidation is not equivalent to owner-requested hard deletion or access revocation.
- The presence of Rust, SQLite, a graph or a memory directory does not prove that these five algorithms are already integrated.

## Source index and provenance

Snapshot: verified public primary research / first-party code and docs as of 2026-11-07. Public sources cited in this note:
1. Mem0 repo: https://github.com/mem0ai/mem0
2. Mem0 2025 paper: https://arxiv.org/abs/2504.19413
3. Graphiti paper: https://arxiv.org/abs/2501.13956
4. Graphiti code: https://github.com/getzep/graphiti
5. Graphiti temporal docs: https://help.getzep.com/facts
6. HippoRAG NeurIPS 2024: https://proceedings.neurips.cc/paper_files/paper/2024/hash/6ddc001d07ca4f319af96a3024f6dbd1-Abstract-Conference.html
7. HippoRAG 2 code: https://github.com/OSU-NLP-Group/HippoRAG
8. A-MEM NeurIPS 2025: https://papers.nips.cc/paper_files/paper/2025/hash/19909c36f51abc4856b4560aff3d36d6-Abstract-Conference.html
9. A-MEM code: https://github.com/WujiangXu/A-mem
10. Letta MemFS: https://github.com/letta-ai/letta-docs-md/blob/main/concepts/memfs/index.md
11. LongMemEval: https://proceedings.iclr.cc/paper_files/paper/2025/hash/d813d324dbf0598bbdc9c8e79740ed01-Abstract-Conference.html
12. LongMemEval-V2: https://github.com/xiaowu0162/LongMemEval-V2

**Next implementation decision:** create an experiment harness that reads the **existing** VIVIM canonical graph/event contracts first. Do not duplicate the architecture before baseline gaps and permission semantics have been audited.
