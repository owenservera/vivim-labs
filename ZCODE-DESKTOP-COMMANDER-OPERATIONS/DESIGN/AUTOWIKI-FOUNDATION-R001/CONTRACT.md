# Semantic-to-readable wiki contract

## Two representations, three views
Canonical data: typed graph assertions with entity IDs, relation IDs, temporal validity, source provenance, revision, verification and visibility. Semantic claims: bounded interpretations of assertions with a deterministic link to their evidence. Human page: a readable narrative synthesized only from those claims. A compiler token stream does not need grammatical English.

The three views: human prose (default); semantic claims (review/debug); trace/VLANG syntax (developer). All views share one page ID and revision, never duplicate facts.

## Claim model
- id; subject_id; predicate_id; object_id or literal; evidence_refs[]; source_revision; observed_at; visibility; verification = verified | candidate | disputed | unknown; confidence if measured (not fabricated); authority_scope.
- Inference must carry inference method + supporting claim IDs, and must never be silently promoted to direct evidence.
- Statements must retain negations, uncertainties, tense, units, actors and relation direction.

## Publication gates
1. Grounding: every factual sentence maps to at least one claim with provenance.
2. Coherence: readable grammar, defined referents, no dangling pronouns, no duplicate/contradictory claims without qualification.
3. Hierarchy: each hub has an intro and typed outbound links; every link target exists or is explicitly a pending stub.
4. Fidelity: reversible trace from sentence -> claim IDs -> graph/source revision; language cannot change the meaning.
5. Temporal truth: show stale/unknown status; do not convert catalogued capability into verified live functionality.
6. Access: redaction before rendering; no private source leakage through linked titles or search snippets.
7. Reproducibility: same input graph revision and configuration produce same deterministic structural page, with any optional natural-language enrichment stored as an explicitly versioned candidate.

## Feedback into VLANG
A page quality failure is a diagnostic on the *earliest responsible stage*: missing evidence = intake/graph; ambiguous relationship = semantic typing/VLANG; incoherent explanatory bridge = language-realizer; broken navigation = wiki projection; incorrect executed-status = command registration/runtime evidence.
Emit a typed diagnostic {page_id, claim_ids, graph_revision, stage, code, severity, suggested_fix, evidence_refs}. No autonomous compiler rewrite. Feed diagnostics to a reviewed tracker proposal via existing owner writer.

## Feedback into AutoWiki command space
Proposed operation families: wiki.read, wiki.search, wiki.explain, wiki.follow_link, wiki.show_evidence, wiki.show_trace, wiki.refresh_preview, wiki.review_candidate, wiki.report_quality. Only read/preview functions are allowed without action authority; promotion/refresh activation must remain behind existing Host and wiki-program revision/approval ownership. Unknown command -> NO_MATCH, not guessed execution.

## Acceptance metrics (per revision)
- 100% displayed factual assertions traceable to claims/evidence (or explicitly marked unverified)
- 0 silently broken links; 0 declared-as-executed misstatements
- All 7 hubs render with purposeful intros and reachable navigation; no empty gibberish page
- Same revision yields stable page ID/link structure
- Update from typing yields supersedable preview; accepted graph revision updates page without fabricated writes
- Redacted facts remain redacted across prose, snippets, semantic view and trace
