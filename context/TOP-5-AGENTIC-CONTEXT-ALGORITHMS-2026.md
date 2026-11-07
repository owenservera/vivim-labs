# Top Five Algorithms for Agentic Context Management

**VIVIM Labs / context research**  
**Research date:** 2026-11-07  
**Status:** Research and design recommendations only; no implementation or benchmark run is claimed.  
**Scope:** Dynamic context bundles, Oracle's task-time context, Rust live graph / KB, tool results, long conversations and multi-agent work.

## Executive conclusion

The strongest design is a **composition** rather than a winner-take-all context algorithm:

1. **Hybrid retrieval + Reciprocal Rank Fusion (RRF)** finds candidate evidence through lexical, semantic, and graph-assisted routes.
2. **Maximal Marginal Relevance (MMR)** removes near-duplicates while preserving topic breadth.
3. **Personalized PageRank (PPR)** adds evidence that is causally, semantically, or structurally related across the knowledge graph.
4. **Budgeted submodular selection (facility location / CELF, PACMS-style)** chooses which heterogeneous context items actually enter the limited prompt.
5. **Query-aware compression (LongLLMLingua / LLMLingua-2 family)** shrinks selected, non-critical material when necessary without changing authoritative instructions or executable contracts.

**Ranking criterion:** practical value and applicability to VIVIM's architecture, not an auditable industry adoption league table. No reliable cross-vendor deployment census was found. These five represent algorithmic families with primary research and/or established ecosystem implementations, but differ in maturity. Hybrid RRF and MMR are well-established retrieval approaches; graph-based PPR retrieval and compression have empirical research; agent-level submodular assembly is promising and still emerging.

**Important:** Dynamic context management is not long-term memory storage, personalized preference learning, or model training. It is a deterministic and inspectable compilation step that decides what evidence is visible to an agent **now**. The original source remains available for drill-down, correction, and recovery.

## 1. Reciprocal Rank Fusion (RRF) — retrieve complementary evidence

**Problem:** lexical BM25 finds exact IDs/terms, dense vector retrieval finds paraphrases and semantic neighbors, and graph/local retrieval can find relationships. The raw scores are not comparable.

**Algorithm:** for candidate document `d` from retrieval lists `L`:

`RRF(d) = Σ[L containing d] 1 / (k + rank_L(d))`, commonly beginning with `k = 60` (tune with judgments).

**Why agents use it:** combine ranked results without score normalization; resistant to differences between embedding distances and BM25 values. Both Elasticsearch and OpenSearch document RRF, and Google Cloud's hybrid search uses it.

**VIVIM design:** gather candidates from lexical index, optional local embedding index, wiki/entity graph and session state. Resolve semantic identities before fusion; fuse at stable `evidence_id`. Preserve per-source ranks in the receipt, and apply authorization filtering *before* retrieval and again at context assembly.

**Failure modes:** duplicate IDs across sources, graph over-representation, stale indexes, exact-code snippets drowned out by semantic matches. Benchmark keyword-only, semantic-only and fused pipelines.

**Implement:** deterministic Rust rank-fusion node; minimal dependencies; top-N cap and bounded time. Initial `k=60` is a test baseline, not a universal optimum.

**Evidence:** [Elasticsearch RRF documentation](https://www.elastic.co/docs/reference/elasticsearch/rest-apis/reciprocal-rank-fusion); [OpenSearch RRF documentation](https://docs.opensearch.org/latest/vector-search/ai-search/hybrid-search/rrf/); [Google Cloud hybrid search](https://docs.cloud.google.com/gemini-enterprise-agent-platform/build/vector-search/about-hybrid-search).

## 2. Maximal Marginal Relevance (MMR) — balance relevance and diversity

**Problem:** top-K retrieval often returns several near-identical passages, starving the context of different relevant concepts, failures, decisions and files.

**Algorithm:** iteratively pick candidate `d` not already selected `S` maximizing:

`MMR(d) = λ · sim(query,d) − (1−λ) · max[s in S] sim(d,s)`.

`λ` here is defined as the relevance weight: 1 = relevance only, 0 = diversity only. **Warning:** some SDKs invert the λ convention; map by semantic meaning, not by parameter name.

**VIVIM design:** apply MMR to the fused candidate pool using embeddings or another meaningful similarity function. Partition or constrain by source/task type when diversity could exclude legally or operationally mandatory evidence. Initial `λ=0.7` is a proposed experiment, not a validated production setting.

**Why it matters:** agents need multiple kinds of supporting material: code, architecture decisions, current task state, negative evidence, and tool outcomes.

**Failure modes:** irrelevant but novel documents can displace relevant ones; MMR can over-diversify at very tight budgets. PACMS's 2026 evaluation explicitly notes poorer MMR recall at 20% budgets in its chosen setup. Compare MMR against plain top-K at identical token budgets.

**Evidence:** [Google Cloud: MMR in LangChain / Bigtable](https://docs.cloud.google.com/bigtable/docs/mmr-vector-search); [OpenSearch: vector search with MMR](https://docs.opensearch.org/latest/vector-search/specialized-operations/vector-search-mmr/); [PACMS evaluation](https://arxiv.org/abs/2606.20047).

## 3. Personalized PageRank (PPR) — follow knowledge relationships

**Problem:** vector similarity alone can miss evidence requiring multiple hops across entities, dependencies, people, decisions, commits, agents and documents.

**Algorithm:** run a random walk with restart over an evidence graph, seeded by entities/passages relevant to the current query:

`r_(t+1) = α·p + (1−α)·Pᵀ·r_t`

where `p` is the query-specific restart distribution and `P` is a normalized graph transition matrix. Iterate until convergence or a hard time/iteration limit; bound traversed graph and return passages with provenance paths.

**Evidence and maturity:** HippoRAG (NeurIPS 2024) combines knowledge graphs with PPR; HippoRAG 2 (ICML 2025) augments graph retrieval with deeper passage integration and LLM-assisted filtering. In the paper's evaluation HippoRAG 2 reports a 7% improvement in associative memory over the specified embedding baseline; this is not a guarantee for VIVIM.

**VIVIM design:** use canonical identities and typed, dated, scoped edges from the Rust live graph. Support seed nodes from current task, referenced repo, active file, owner request, latest tool receipt, or specific prior decision. Supply graph traversal explanations `seed → edge → evidence`, not just an opaque score. Recheck every returned node/edge against access permissions.

**Failure modes:** incorrect alias merges, stale edges, hub nodes and path explosion. Compare against direct RRF-only retrieval on multi-hop queries, contradictory decisions and renamed files.

**Implementation note:** do not equate PPR traversal with every part of Microsoft's GraphRAG: GraphRAG also uses Leiden community detection, hierarchical summaries and global/local query modes, which solve related but distinct problems.

**Evidence:** [HippoRAG NeurIPS 2024](https://papers.nips.cc/paper/2024/hash/6ddc001d07ca4f319af96a3024f6dbd1-Abstract-Conference.html); [HippoRAG 2 ICML 2025](https://proceedings.mlr.press/v267/gutierrez25a.html); [HippoRAG source](https://github.com/OSU-NLP-Group/HippoRAG); [Microsoft GraphRAG](https://microsoft.github.io/graphrag/).

## 4. Budgeted submodular coverage (facility location + CELF) — decide what enters the prompt

**Problem:** every agent step combines recent conversation, older memory, tool outputs, file snippets, active plans and persistent instructions under one finite token budget. Last-K truncation throws away old but currently relevant information.

**Algorithm family:** maximize relevance-weighted coverage with diminishing returns, subject to token budget and a mandatory-item set. PACMS's formulation is:

`maximize F(S) = Σ[i in candidates] max[j in S] w(i,j)`

subject to `Σ[j in S] tokens(j) ≤ B` and `mandatory_items ⊆ S`.

`w(i,j)` weights how well selected item `j` represents query-relevant candidate `i`. CELF lazy-greedy is a practical optimization technique. Preserve mandatory system/user/approval state outside the discretionary candidate budget, or model as immutable mandatory items with explicit reserved tokens.

**Why this is not simply MMR:** MMR scores individual selections using a selected-set similarity penalty; facility-location coverage rewards how much of the *whole relevant candidate pool* a chosen item represents, accounting for heterogenous sizes and diminishing returns.

**2026 evidence:** the PACMS research prototype integrates a pluggable context engine and evaluates 100 LongMemEval questions. At 45% budget and the authors' chosen redundancy setting, it reports QA accuracy of 52% vs 50% top-K and 44% MMR under GPT-5-mini, and 68% vs 64% vs 56% under GPT-5.4-mini. These are limited, model- and setup-specific comparisons—not general production results. At 20% budget, top-K had better evidence recall in several settings. The October 2026 SCOPE paper studies coordinating memory/tool/prompt budget via a contextual bandit; its reported 96.8% decision fidelity at 4.46× compression is from that paper's evaluation, not from VIVIM.

**VIVIM design:** create a context assembly policy node with reserved bands for (a) untouchable instructions, (b) current owner request/approval state, (c) live task and action contracts, (d) candidate evidence, and (e) output headroom. Compare `topk`, `lastk`, `mmr`, `coverage_celf` at the same budget. Use exact tokens and deterministic tie-breaking. Record *excluded* items and reasons, not merely included items.

**Failure modes:** embedding coverage poorly approximates task success; a coverage-optimal set can omit one critical proof fragment; operation ordering and source authority may matter more than semantic spread. Treat safety / execution preconditions as hard constraints, not soft semantic scores.

**Evidence:** [PACMS June 2026](https://arxiv.org/abs/2606.20047); [SCOPE EMNLP 2026 industry track](https://aclanthology.org/2026.emnlp-industry.129/); [Anthropic context engineering overview](https://www.anthropic.com/research/effective-context-engineering-for-ai-agents).

## 5. Query-aware compression (LongLLMLingua family) — spend fewer tokens

**Problem:** even a good selected set may include long tool traces, logs, repeated explanations and files. Text compression can preserve useful meaning while reducing tokens and latency.

**Algorithm family:** LongLLMLingua (ACL 2024) uses question-aware coarse-to-fine compression, document reordering, dynamic compression ratios and information recovery. LLMLingua-2 uses a learned token-classification encoder for task-agnostic extractive compression. For current-task specificity, prefer question-aware compression or a constrained extractive alternative over a generic summary.

**Evidence:** the LongLLMLingua paper reports, on a particular NaturalQuestions configuration, up to a 21.4% improvement with around four times fewer input tokens using GPT-3.5-Turbo. LLMLingua-2 reports faster compression than earlier methods under its evaluations. These numbers are benchmark-dependent, not guarantees.

**VIVIM design:** preserve raw evidence by reference. Compress optional passages and verbose tool outputs only, keep identifiers, line numbers, URLs, hashes, ordered steps and decision boundaries, and attach a `compression_method`, `source_evidence_id`, `source_hash` and `risk`. Allow owner/Oracle to request original uncompressed evidence at any time.

**NEVER compress:** system/owner policy, approval requests and receipts, capability/permission contracts, cryptographic material, executable command arguments, exact code/diffs and other data where a single altered character changes semantics. For these, retain exact originals or exclude safely.

**Failure modes:** erased negations, stale summaries, altered numbers, missing constraints, inverted instructions. Test semantic and exact-field preservation, not just shorter text.

**Evidence:** [LongLLMLingua ACL 2024](https://aclanthology.org/2024.acl-long.91/); [LLMLingua-2 paper](https://arxiv.org/abs/2403.12968); [Microsoft code](https://github.com/microsoft/LLMLingua); [EMNLP 2026 survey on compression failure modes](https://aclanthology.org/2026.emnlp-main.2396/).

---

## Proposed VIVIM dynamic context pipeline

```text
OWNER REQUEST / AGENT TASK
          │
          ▼
Task interpreter + immutable owner/host security context
          │
          ▼
Candidate acquisition: active turn + notes + KB + code + wiki + tool receipts
          │
          ├─ BM25 / exact term / file and symbol search
          ├─ Dense semantic retrieval (optional local model / adapter)
          └─ Rust live graph PPR expansion (bounded)
          │
          ▼
Identity reconciliation + provenance + ACL/filter + freshness
          │
          ▼
RRF fusion → MMR diversification (compare to no-MMR ablation)
          │
          ▼
Budgeted facility-location / CELF selection
          │
          ▼
Optional safe compression of non-authoritative evidence only
          │
          ▼
Dynamic Context Bundle v1: ordered evidence + source refs + token ledger
          │
          ▼
Model through approved, signed-in web surface (no provider APIs)
          │
          ▼
Read-only context receipt, outcomes, feedback, cache updates
```

**Alternative paths:** run PPR after RRF seed generation if the graph is too large; run facility-location without MMR as an ablation because they partly overlap. These are swappable deterministic engines, not five non-negotiable serial transforms. The agent may choose to retrieve a further evidence slice just-in-time; it must not gain privileges from retrieved text.

### Proposed portable contracts

```ts
type ContextCandidate = {
  evidence_id: string;
  canonical_entity_ids: string[];
  source_uri: string;
  revision_or_hash: string;
  source_kind: "chat"|"file"|"tool"|"graph"|"memory"|"wiki"|"policy";
  body_ref: string;           // immutable backing data, not necessarily embedded text
  created_at: string;
  observed_at: string;
  token_estimate: number;
  authority_class: string;    // immutable policy, required task state, optional evidence...
  scope_id: string;
  allowed_for_agent: boolean;
  lexical_rank?: number;
  semantic_rank?: number;
  graph_rank?: number;
  retrieval_path?: string[];
};
type DynamicContextBundle = {
  bundle_id: string;
  task_id: string;
  policy_version: string;
  model_profile: string;
  budget: { total: number; reserved: number; evidence: number; response: number };
  included: { evidence_id: string; source_hash: string; tokens: number; reason: string }[];
  excluded: { evidence_id: string; reason: string }[];
  algorithms: { name: string; version: string; parameters: Record<string, unknown> }[];
  compile_receipt: string;   // audit hash / replay reference
};
```

**Boundary with the other Labs workspaces:**
- `context/` owns retrieval, assembly, current working state, context budgets, freshness and source receipts.
- `memory/` owns durable remember/forget lifecycle and retrieval-ready memory artifacts; it does not own each prompt's final assembly.
- `learning/` may learn improved routing/weights from consented outcome feedback after reproducible deterministic baselines exist.
- `personalization/` remains **undefined** until an explicit product contract exists; do not silently reinterpret profile facts as permanent preferences or execution authority.
- Oracle may request, rank and explain evidence, but permission checks and execution approvals stay enforceable Host-side. Agent output never bypasses owner approval.

### LRU cache: performance layer, not relevance policy

Use a bounded LRU for parsed chunks, embeddings and query results, keyed by `evidence_id + revision + model_version + scope_id + index_version`. Apply invalidation upon file revision, changed permissions, deletion, namespace moves and graph edits. A cache hit does **not** mean an item should be included in context. Do not evict durable KB records just because an LRU entry expires. Never cache private content across authorization scopes.

## Evaluation program — pre-Wave-2 baseline

**Fixtures:** repo/symbol navigation, renamed files, duplicate docs, multi-hop dependency questions, past decisions that conflict with newer decisions, large tool logs, user correction, revoked permissions, multi-agent handoffs, partial reads and source deletion.

**Ablations:**
- A: top-K BM25 / dense separately
- B: BM25+dense RRF
- C: B + MMR
- D: B + graph PPR
- E: B/D + budgeted facility-location (with and without MMR)
- F: E + safe selective compression

**Measure at identical budgets:** evidence recall@budget; answer support and groundedness; cross-hop success; correct citation/line precision; token cost; context compile latency p50/p95; prompt overflow rate; dropped-required-information incidents; security/ACL violations (must be zero); deterministic replay and restart behavior. Use objective evaluation independent of answer fluency.

**Budget sweeps:** e.g., 4K / 8K / 16K evidence budgets plus 20%, 45% and 70% of candidate pool size to compare with recent research; test at cold and warm cache.

**Recommended order of engineering:**
1. Freeze evidence identity, source provenance, ACL, exact token ledger and deterministic bundle receipts.
2. Build RRF baseline and MMR comparison.
3. Attach bounded PPR traversal to existing Rust graph without duplicating the graph engine.
4. Add an opt-in facility-location selector, comparing it to top-K and MMR at equal budgets.
5. Add strict, selective compression with exact-data exclusions and reversible references.
6. Tune only with recorded benchmark outcomes; add learned policy routing later.

## Critical context research and trade-offs

- [Anthropic, Effective context engineering for AI agents (2025)](https://www.anthropic.com/research/effective-context-engineering-for-ai-agents): just-in-time retrieval, compaction, structured note-taking, multi-agent context isolation.
- [Anthropic Cookbook, Context engineering: memory, compaction and tool clearing (2026)](https://platform.claude.com/cookbook/tool-use-context-engineering-context-engineering-tools): practical operational distinctions.
- [Liu et al., Lost in the Middle (TACL 2024)](https://aclanthology.org/2024.tacl-1.9/): relevance can be obscured by context position and length; inclusion alone does not guarantee model use.
- [Wang et al., Context Compression for LLM Agents (EMNLP 2026)](https://aclanthology.org/2026.emnlp-main.2396/): failure phases—pre-compression choice error, in-compression information loss, post-compression access failure.
- [Ghosh et al., SCOPE (EMNLP 2026)](https://aclanthology.org/2026.emnlp-industry.129/): allocation across memory, tools and instructions via an adaptive planner, still research rather than assumed default.
- [Ghulyani et al., PACMS (2026 preprint)](https://arxiv.org/abs/2606.20047): implemented submodular context packer; limited published 100-question comparison and caveats.

## Research claims vs. unproven implementation

**Verified in cited external material:** each algorithm and its described research use; documented search-engine support for RRF/MMR; peer-reviewed research on PPR retrieval and compression; early agent-specific budgeted selection results.

**Not verified / not claimed:** these five are the globally most deployed techniques; VIVIM currently runs any of them; VIVIM has passed any benchmark; the suggested parameter values are optimal; or the proposed TypeScript-like schema matches the existing canonical Rust contracts.

**Next code investigation before implementation:** inspect canonical Repos HQ retrieval/graph contracts and the Labs dynamic context bundle research; align rather than invent duplicate stores or agentic file types. Preserve every existing Host authorization invariant.
