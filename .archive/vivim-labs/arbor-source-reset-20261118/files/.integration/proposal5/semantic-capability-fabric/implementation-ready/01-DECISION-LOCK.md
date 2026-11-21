# Implementation decisions — binding, no redesign in worker chats
1. **One namespace:** `@hq:`; v1 and v2 tags are unchanged. SCF introduces additive typed v3 refs, never executable code in comments.
2. **One language mode switch:** v3 opt-in per source/project `semanticProtocolVersion=3`; old v2 parser sees v3 as unsupported/ignored and never claims completeness. No new global scanner that changes legacy parsing.
3. **Minimal file declaration:** context, canonical owner, 1..N registered capability refs. A waived file is explicit `UNCLASSIFIED` in PR metadata, not a forged ID.
4. **Units are finer-grained:** file scope and exported symbol scope are separate; when ambiguous, no auto-attach.
5. **Stable capability ref:** existing `name@version`; source unit ID is additional and does not override Git commit/blob or owner identity.
6. **Registry owner:** existing `semantic_id`; additive kinds only: `unit`, `role`, `relation`, `migration`, `verification-profile`, `observation-profile`, `explanation-pack`. Must preserve proposed→current owner confirmation.
7. **No automatic registry writes:** agent annotations may reference only approved/current IDs; all proposals use existing review workflow.
8. **No permission from comment:** `@hq:effect` and `@hq:action` describe potential behavior and owner link, never authorize.
9. **No proof from compiler:** independent SV checker/approved oracles produce claim-specific receipts under pinned build/source/model.
10. **Provenance exact:** pinned repo/commit/tree SHA, path/blob SHA, byte range/line+symbol anchor, grammar and registry/catalog digests. Dirty worktree is `WORKSPACE_EPHEMERAL`, not a commit.
11. **Hashing:** use SHA-256 of canonical JSON with sorted object keys, lexicographic arrays only where set semantics; no date/locale/local path in content digest. Git object SHA remains Git SHA-1/256 as reported; don't conflate.
12. **One derived index:** additive SCF tables under existing `.hq/index.db`; derived data rebuildable. No new graph DB or independent authoritative ledger.
13. **One wiki renderer:** `buildWiki` accepts optional passport read model; generated pages are never modified by agent hand.
14. **One HVI:** SCF produces optional semantic artifacts keyed by pins; HVI remains the native N-way comparison tool and raw source fallback.
15. **HVI explicit usage only:** No real donor migration comparison in implementation/verification stage; synthetic invented fixture repos for 2/3/N tests.
16. **No implicit real runtime verification:** available source path, imported symbol, fixture test, static reachability, observed CI, and actual Host/device receipt are distinct.
17. **SV evidence binding:** reuse `arbor.sv.claim/0.1`, `arbor.sv.proof-receipt/0.1`, `arbor.sv.explanation/0.1`; independent checker can refute the compiler. Refutation persists in passport.
18. **Permission:** source snapshot acquisition and disclosure recheck exact actor/session/live approvedRoots per operation; cloned/cached sensitive data cannot bypass revocation.
19. **Performance:** incremental parsed source cache keyed blob+parser/registry digest; dependency invalidation bounded; low disk retains last confirmed generation and returns INCOMPLETE.
20. **Rollout:** touched first-party files warning initially; strict only after baseline coverage+false-positive review and independent CI; vendor/generated excluded explicitly with counters.
21. **No mass annotation:** never auto-declare features for untagged legacy code; produce unknown/candidate source analysis instead.
22. **No new executor:** Host only effect authority; Studio only signed-in browser sender; TierFlow only UI publisher; Rust HQKG stays one native graph compute owner.
23. **Gate:** agent may implement on dedicated branch after S0; independent Z review then G serial PR. Never merge source if negative tests UNRUN or PWA security falsely accepted.
24. **Rollback:** revert one reviewed feature merge, retain historical immutable source/evidence; disable read-only feature flag; regenerate only derived SCF tables/old wiki; no deletion of personal originals.
