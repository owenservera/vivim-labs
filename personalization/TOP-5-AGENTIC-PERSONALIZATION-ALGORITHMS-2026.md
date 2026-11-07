# Top 5 algorithms for agentic personalization — VIVIM / Oracle research

**Research date:** 2026-11-07  
**Status:** research and candidate architecture; **not** an implementation report  
**Owner:** `personalization/` in `owenservera/vivim-labs`  
**Scope:** A continuously adapting *individual* agent that personalizes interaction, planning, retrieval, model/tool selection, and recommendations. Explicit owner permissions and safety limits always override inferred preferences.

## Executive conclusion

There is no audited industry-wide deployment census identifying the five numerically "most used" algorithms in agentic personalization. This is a **research-backed, opinionated top five for VIVIM** ranked by complementary architectural value, current evidence, implementation practicality, and fit with Oracle's local-first / user-controlled design. They are **algorithm families** with concrete candidate algorithms, not interchangeable products. The papers below demonstrate research use and benchmark performance, **not** verified adoption rates in commercial agent stacks.

| Rank | Algorithm family / recommended instance | What it solves | VIVIM locus | First implementation |
| --- | --- | --- | --- | --- |
| 1 | **Online latent preference tracing** — sequential Bayesian update / sequential Monte Carlo (SMC), with a simpler decay baseline | What the user presently wants, with uncertainty and changing preferences | Personalization state estimator | Temporal facts + confidence + scoped evidence; add SMC when warranted |
| 2 | **Personalized graph retrieval** — query-conditioned weighted Personalized PageRank (PPR) + hybrid lexical/semantic ranking | Which user-specific facts, constraints, relationships and exceptions are relevant *now* | Rust knowledge graph / dynamic context bridge | Filtered graph expansion, hybrid rank, cited provenance |
| 3 | **Pairwise preference learning** — per-user Bradley–Terry (BT) scoring; DPO only if actual weight training is justified | Relative tastes among responses, actions, UI modes, models, suggestions | Preference scorer / policy ranker | Lightweight per-user BT or logistic pairwise model |
| 4 | **Contextual bandits** — LinUCB or (linear) Thompson sampling | Which *permitted, low-risk* option to try, balancing usefulness and uncertainty | Action/model/recommendation selector | Conservative LinUCB with logged propensities; optional Thompson |
| 5 | **Active preference elicitation** — expected information gain / value-of-information decision rule, followed by feedback updates | Whether to ask a question before acting; how to learn from corrections afterward | Oracle clarification + feedback gate | Cost-sensitive ask/act rule, bounded clarification, explicit corrections |

**Recommendation:** implement `1 + 2 + 5` as the *minimum safe personalization loop*, then `3` as a user-specific ranking layer, then `4` for reversible choices. None of these should become a separate system of record or bypass Host action contracts.

---

## 1. Online latent preference tracing — temporal Bayes / SMC

**Mechanism:** maintain multiple plausible hypotheses about the user's preferences and short-term intent, rather than stamping every inference as an enduring fact. On observation `o_t`, perform a state transition and update posterior belief `p(z_t | o_1..o_t)`. Sequential Monte Carlo approximates that belief with weighted hypotheses / particles. A lightweight baseline uses confidence-weighted updates and time decay, plus direct override semantics.

**Best for:** contradictions, implicit preferences, transient context versus stable preferences, changing interests, and uncertainty. Distinguish `always prefers short answers` from `wants a short answer for this task`.

**Candidate routine:**
1. Receive permissioned evidence from recent user messages and explicit feedback.
2. Classify evidence as **explicit**, **inferred**, or **observed outcome**; attach source, scope, timestamp, version and confidence.
3. Generate or retain hypotheses for `(subject, preference dimension, scope)`; update likelihoods with new evidence.
4. Apply deterministic precedence (current explicit instruction > scoped explicit preference > inference) and appropriate decay only to *non-permanent* evidence.
5. Retain competing hypotheses when evidence is weak; expose uncertainty to the clarification gate.
6. On contradiction, record supersession and effective interval; do not silently erase provenance.

**Research:** HyperTrace (EMNLP 2026) updates interpretable short-term/long-term hypotheses via SMC-style reweighting without model weight updates. Vague Preference Policy Learning (ACM TOIS 2025) uses ambiguity-aware soft estimates and time-aware decay. These are examples, not proof that SMC is universally optimal.

**Costs / failure modes:** too many particles, incorrect observation likelihood, treating sarcasm or one-off requests as durable identity, preference hysteresis, volatile memory. Start with a deterministic baseline and establish incremental lift before deploying a full particle filter.

**References:** [HyperTrace, EMNLP 2026](https://aclanthology.org/2026.findings-emnlp.1038/); [Vague Preference Policy Learning, ACM TOIS 2025](https://doi.org/10.1145/3717831); [PersonaMem-v2, 2025](https://arxiv.org/abs/2512.06688).

## 2. Personalized graph retrieval — weighted PPR + hybrid reranking

**Mechanism:** retrieve a relevant, **permission-filtered** subset of the user knowledge graph by seeding the current task, user, workspace, domains and recent relevant evidence. Traverse scoped relations, perform bounded weighted random-walk/PPR or constrained neighborhood expansion, then fuse semantic and lexical relevance with freshness, explicitness and provenance.

A candidate (not calibrated) reranking expression:

`rank(e,q) = w_sem * semantic(e,q) + w_lex * lexical(e,q) + w_graph * graph_relevance(e,q) + w_rec * recency(e) + w_ev * evidence_quality(e)`

**Best for:** cross-session personal context, user/project/entity connections, scoped exceptions, contextualization across tools and projects, retrieval without huge prompt dumps.

**Implementation boundary:** integrate with the existing Rust knowledge graph/indexer and dynamic-context bundle; do **not** maintain a second independent graph that disagrees with canonical semantic identity. The personalization layer owns retrieval *policy* and relevance weights, not canonical storage. Emit traceable `source_ids`, effective dates, and why a memory was selected.

**Key constraints:** ACL/privacy filtering **before** scoring and graph expansion, explicit scope, maximal hops and top-k, score calibration, invalidation on preference change/deletion, dynamic context token budget, deterministic fallback (recency+lexical) if graph is unavailable. PPR does not confer access permission.

**Research:** PGraphRAG uses user-centric knowledge graphs to improve personalized generation; MemORAI (2026 preprint) explicitly uses **query-adaptive Dynamic Weighted PageRank** and provenance-enriched graph relationships. EMG-RAG (EMNLP 2024) explores editable memory graphs for personalized agents on mobile memories.

**Costs / failure modes:** dense/high-degree nodes dominate, stale or incorrectly linked identities, graph expansion leaking unrelated facts, too much graph machinery for trivial tasks. Compare with lexical and vector retrieval at equal context budgets.

**References:** [PGraphRAG, 2025](https://arxiv.org/abs/2501.02157); [MemORAI, 2026](https://arxiv.org/abs/2605.01386); [Editable Memory Graph RAG, EMNLP 2024](https://aclanthology.org/2024.emnlp-main.281/); [LongMemEval, ICLR 2025](https://proceedings.iclr.cc/paper_files/paper/2025/hash/d813d324dbf0598bbdc9c8e79740ed01-Abstract-Conference.html).

## 3. Pairwise preference learning — per-user Bradley–Terry; conditional DPO

**Mechanism:** the user chooses or corrects the agent's offered variants. For alternatives A and B, a Bradley–Terry-style model estimates `P(A > B | user, context) = sigmoid(r(user, context, A) - r(user, context, B))`. Learn small per-user/context score weights from verified pairwise preferences rather than assuming a universal population preference.

**Best for:** answer verbosity, visual style, candidate model selection, recommended action ordering, notification cadence and other **soft** choices. This is a scoring preference, not an authorization.

**Implementation:** collect explicit A/B wins and optionally low-confidence, opt-in implicit signals; score only within comparable tasks and contexts; store the pair and outcome with provenance; regularize aggressively for small datasets; let explicit, current instructions supersede learned ranking. Do not equate clicks, dwell time or silence with consent.

**DPO boundary:** Direct Preference Optimization is a training-time algorithm for tuning model weights from preference pairs; it is **not** inherently required for lightweight per-user inference. With web-only third-party models, local BT reranking is immediately compatible; per-user DPO is only a later experiment if VIVIM controls the trainable model and has sufficient consented data.

**Research:** personalized user rankings from ACL 2026 show large divergence from aggregate rankings; PLUS (ICLR 2026) adds user-summary-conditioned reward modeling. DPO's original NeurIPS 2023 paper establishes the separate model-training technique.

**Costs / failure modes:** confounding by task difficulty, sparse labels, preference cycles, nonstationarity, preference features leaking sensitive traits, optimization for clicks at expense of owner goals. Pairwise rank is a soft heuristic; never use it to expand action authority.

**References:** [Personalized Benchmarking, ACL 2026](https://aclanthology.org/2026.findings-acl.31/); [PLUS, Microsoft Research / ICLR 2026](https://www.microsoft.com/en-us/research/publication/learning-to-summarize-user-information-for-personalized-reinforcement-learning-from-human-feedback/); [DPO, NeurIPS 2023](https://papers.neurips.cc/paper_files/paper/2023/hash/a85b405ed65c6477a4fe8302b5e06ce7-Abstract-Conference.html).

## 4. Contextual bandits — LinUCB / Thompson sampling

**Mechanism:** for an allowed candidate arm (e.g., answer presentation mode, useful suggestion, low-risk recommendation), model expected user utility conditioned on task context and current preferences. LinUCB selects high predicted utility plus bounded uncertainty bonus; Thompson sampling selects according to a sampled utility posterior. Feedback updates the model online.

**Best for:** adaptive presentation, notification scheduling, ranking several already-permitted tools/modes, low-stakes suggestions, and cold-start learning. Use deterministic choices when an owner preference has already been made clear.

**Implementation:** construct a stable context feature schema; enumerate **only** reversible/authorized candidate arms; cap exploration; record arm, context, confidence, reward, and action propensity; discount outdated evidence; evaluate offline before any online use; minimize prompts/annoyance. Reward should reflect user-confirmed utility, not purely engagement.

**Do not use exploration** for financial transfers, data deletion, messaging others, privacy boundary changes, irreversible actions, or elevated permissions. The Host approval contract applies whether an arm was selected deterministically or probabilistically.

**Research:** latent contextual bandits directly address cold-start personalization; LinUCB and Thompson sampling are established contextual algorithms. Their use in recommender research does not establish that every LLM agent deploys them.

**Costs / failure modes:** sparse/biased feedback, exploration costs, reward gaming, confounding, stale context, and incorrect off-policy evaluation.

**References:** [Latent Contextual Bandits, IJCAI 2016](https://ijcai.org/proceedings/2016/513); [Constrained Contextual Bandit for Limited-Budget Recommendation, 2024](https://www.sciencedirect.com/science/article/pii/S0952197623017426).

## 5. Active preference elicitation — information gain / value of clarification

**Mechanism:** ask **only** when a specific missing preference materially affects the outcome. Estimate expected value of clarification, e.g.:

`VOI(question) = expected_loss_without_question - expected_loss_after_answer - user_interruption_cost`

or use expected entropy reduction `H(intent) - E[H(intent | answer)]` as a proxy. If the best useful action is obvious and safe, act. If the action requires owner approval, obtain approval independently of preference uncertainty.

**Best for:** cold start, conflicting instructions, preference drift, high-cost wrong assumptions, personalized autonomy without excessive questioning.

**Loop:** (a) retrieve existing scoped preferences, (b) decide if the uncertainty is material, (c) ask one discriminative question if positive value, (d) act through existing permission gate, (e) solicit or accept corrections, and (f) update the relevant preference hypothesis with evidence. Never force a user to re-answer something already known and current.

**Research:** GATE (ICLR 2025) uses language-based interactive elicitation; IntentSim (NAACL 2025) estimates uncertainty over intents; PAHF (Feb 2026) combines pre-action clarification, explicit memory grounding and post-action feedback to adapt to drift. These support the *design*, not a guaranteed deployment uplift.

**Costs / failure modes:** interruptive over-asking, overconfident inferences, incentive to ask for secrets, self-reinforcing assumptions, ambiguity about whether feedback applies globally. Enforce user burden budget and separate user authority/consent from preference discovery.

**References:** [GATE, ICLR 2025](https://proceedings.iclr.cc/paper_files/paper/2025/hash/c9867d5a22653ce98b02595061e40f12-Abstract-Conference.html); [Clarify When Necessary / IntentSim, NAACL 2025](https://aclanthology.org/2025.findings-naacl.306/); [PAHF, Meta Research, Feb 2026](https://ai.meta.com/research/publications/learning-personalized-agents-from-human-feedback/); [Google Research preference elicitation, 2025](https://research.google/pubs/asking-clarifying-questions-for-preference-elicitation-with-large-language-models/).

---

## Proposed VIVIM architecture and ownership

```text
Owner text + corrections + consented observations
                     |
              Evidence normalizer
                     |
         scoped fact / event / preference log
                 /              \
  Temporal preference tracer    Rust KB/identity graph
                 \              /
              Query-adaptive retrieval
                        |
            dynamic context bundle (bounded)
                        |
       personal choice ranker (BT / bandit)
                        |
       clarify? (VOI / policy constraint)
                 /               \
            Ask owner        Host action contract
                 \               /
          result + feedback + receipt
                        |
          versioned preference update
```

### Boundaries of the four research workspaces

- `memory/`: durable evidence, event history, retention, editing/deletion, provenance; **not** the owner of preference *policy*.
- `context/`: task-specific retrieval and assembly into bounded dynamic context bundles; **not** durable truth.
- `learning/`: generic feedback machinery, experiments, parameter updating, evaluation, and drift detection.
- `personalization/`: personal preference state, domain/temporal applicability, uncertainty, individual utility ranking, clarification policy and personal behavior selection. **No duplicate persistence engine or permission system.**

### Suggested data and interface contracts (research proposals, not existing code)

```text
PreferenceEvidence {
  evidence_id, owner_scope, source_id, observed_at,
  channel: explicit | inferred | feedback,
  dimension, value, confidence, task_scope, provenance,
  allowed_purpose, retention_rule
}
PreferenceState {
  preference_id, owner_scope, domain, dimension,
  hypotheses[], explicit_override?, evidence_ids[],
  effective_from, effective_until?, version, updated_at
}
PersonalizationDecision {
  task_id, candidate_ids[], chosen_id?, scores[],
  selected_preference_ids[], evidence_ids[],
  uncertainty, asked_question?, approval_required,
  policy_version, receipt_id?
}
```

**Resolution precedence:** binding permissions/Host policy > current explicit owner instruction > relevant explicit preference > corroborated inference > population default. This is an engineering proposal, not a claim from the cited papers.

**Invariant:** a *preference* is never an authorization to act. Natural-language actions remain routed via Host-owned deterministic action contracts, scoped approval, receipts and recovery. Oracle has global orchestration responsibility but cannot override owner's consent or security enforcement. Prefer offline/local inference and user-controlled data; do not build dependency on upstream provider APIs.

## Suggested phased build and experiments

**P0 — evaluation corpus and deterministic baseline**
- Synthetic users with explicit/implicit preferences, task-local overrides, contradictions, gradual drift, abrupt drift, sparse history and cross-domain transfer.
- Baselines: no personalization; most-recent instruction; static profile; simple time-decayed profile; lexical/vector retrieval.
- Build a minimal preference log and versioned state contract; establish permission and provenance gates.

**P1 — state + retrieval + ask/act**
- Implement simplified temporal preference updater; graph-constrained hybrid retrieval; expected-value clarification gate.
- Test the complete loop across multiple simulated Oracle sessions and dynamic context bundles.
- Add SMC / weighted PPR only where simple baselines are inadequate.

**P2 — user-specific ranking**
- Train small regularized per-user BT/logistic preference scorer from explicit paired choices; compare with deterministic rules and general population rank.
- Do **not** train or tune foundation models by default.

**P3 — safe online adaptation**
- Deploy conservative contextual bandits on reversible candidate choices only, with hard exploration budget, replayable decisions, human corrections and clear off-switch.
- Shadow evaluation before online rollout; no experimental permission-changing or irreversible actions.

**P4 — proof and acceptance**
Measure preference adherence on **PrefEval**, temporal consistency and user-state changes on **PersonaMem / PersonaMem-v2**, retrieval/update/abstention on **LongMemEval**, and custom Oracle task simulations. Compare at equal token and latency budgets. Track:
- precision/recall of applied current preferences, contradiction recovery steps, and preference drift detection delay;
- correct contextual scope, false personalization rate, uncertainty calibration and correct abstention;
- unnecessary clarification rate, user-interruption cost and correction rate;
- task utility per token/latency, graph retrieval quality and scalability;
- **zero** unauthorized actions, privacy leaks across owner/workspace scopes, or permission escalation from learned preferences.

Public benchmark methods: [PrefEval, ICLR 2025](https://proceedings.iclr.cc/paper_files/paper/2025/hash/28a46044775d97a4efcbcf14e7f13209-Abstract-Conference.html); [PersonaMem, COLM 2025](https://www.microsoft.com/en-us/research/publication/know-me-respond-to-me-benchmarking-llms-for-dynamic-user-profiling-and-personalized-responses-at-scale/); [PersonaMem-v2](https://arxiv.org/abs/2512.06688); [LongMemEval, ICLR 2025](https://proceedings.iclr.cc/paper_files/paper/2025/hash/d813d324dbf0598bbdc9c8e79740ed01-Abstract-Conference.html).

## Evidence notes / caveats

1. The top-five ordering is a **VIVIM design prioritization**, not a scientific popularity ranking or proof of production adoption.
2. Performance improvements in the cited papers are measured under their own datasets, model backbones and baselines; results should not be numerically compared across papers.
3. Preprints (including MemORAI and PGraphRAG where applicable) have different levels of independent review than ACL/ICLR/IJCAI proceedings.
4. Personalization is not interchangeable with memory, nor is pairwise scoring the same as DPO fine-tuning.
5. Ancillary approaches worth researching later: reflective retrieval and verbal feedback [RMM (ACL 2025)](https://aclanthology.org/2025.acl-long.413/), [Reflexion (NeurIPS 2023)](https://papers.nips.cc/paper_files/paper/2023/hash/1b44b878bb782e6954cd888628510e90-Abstract-Conference.html), and multimodal personal memory [PersonaVLM (CVPR 2026)](https://arxiv.org/abs/2604.13074).

## Decision record

**Recommended:** use the current canonical Rust graph as *shared* identity/retrieval substrate; put a versioned individual preference model and relevance/decision policies in `personalization/`; reuse `context/` to assemble grounded task context; feed all correction/outcome events to generic `learning/`; treat `memory/` as evidence and persistence owner. Make every personal decision explainable, owner-correctable, scoped, and reversible where possible.

**Research only:** No code, backfill, migrations, or runtime integrations are claimed by this document.
