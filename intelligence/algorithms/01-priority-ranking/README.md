# PULSE-1 — Evidence-aware priority and blocker ranking

**First dedicated assignment; research only.**

**Question:** Which project, task, blocker or decision deserves the owner's attention next, **and why**?

**Read first:** Repos HQ `hq/lib/kb.mjs` and `hq/lib/graph.mjs`; TierFlow `src/visual-runtime/knowledge/wave3-feed-projection.ts`; actual source/provenance contracts. Record current main SHAs before experimenting.

**Inputs:** authorized bounded project/task/blocker candidates, dependency edges, source identity/version, evidence refs, claim status, availability and (only where evidenced) urgency, impact and actionability.

**Output:** deterministic Top-K with stable IDs, evidence-backed reasons, unresolved conflicts, missing information and safe abstention. No hidden item/count leakage. A rank is *advice*, never permission or Host action.

**First hypotheses to test:**
- Compare weighted linear, lexicographic, dependency-aware, uncertainty-sensitive and bounded multi-objective ranking.
- Starting candidate score, not a deployed rule: `0.30 impact + 0.25 dependency + 0.20 actionability + 0.15 urgency + 0.10 freshness`, with features in [0,1]. Unknown evidence stays unknown; authorization and evidence validity are hard eligibility checks, never additive score bonuses.
- Graph cycles, contradictory claims, stale versions, unverifiable deadlines and adversarial priority text must not invent certainty.

**Run:**
1. Inventory what real read contracts expose; distinguish evidenced vs absent signals.
2. Build >=100 independently checked scenarios (critical blocker, irrelevant cosmetic task, unknown Host receipt, inaccessible record, stale/contradictory sources, dependency cycles).
3. Implement algorithms **only in this Labs directory** as pure isolated experimental functions; don't modify HQ/TierFlow/Harness.
4. Compare against existing deterministic search/dependency ordering. Report NDCG@5, critical-blocker recall, false evidence claims, determinism, p50/p95 and memory at 100/1k/10k candidates. Targets to investigate: >=90% critical-blocker recall and zero invented refs; no measured improvements claimed yet.
5. Report the smallest *optional* future adapter to the existing HQ read interface, or REJECT if no clear improvement.

**Artifacts:** put benchmark code, scenario fixtures, raw results and a short decision here as they become real. No new database, service, agent, scheduler, KB authority or production write. Run only authorized reads. This folder currently contains **a brief, not implemented or benchmarked algorithm**.
