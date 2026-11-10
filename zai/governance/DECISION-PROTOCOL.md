# Decision protocol: best-of-breed, authority and preservation

1. **Discover:** conceptual user/system responsibility first; enumerate independent and overlapping implementations by imports, contracts, effects and actual consumers, including orphaned capabilities.
2. **Compare:** record overlapping features and every unique component. Build honest evidence-qualified scorecards; avoid favoring newer or larger code without proof.
3. **Choose:** canonical owner or carefully composed architecture; ensure only one writer/effect authority per responsibility. Select KEEP/ADAPT/ABSORB/RETIRE_AFTER_PROOF/ARCHIVE/DEFER for every non-selected candidate.
4. **Propose:** append a ZAID to [DECISIONS](./DECISIONS.md) and any ZAIQ to [OPEN-ITEMS](./OPEN-ITEMS.md). Proposed choices are not approvals.
5. **Authorize:** seek source-owner or explicit user approval before modifying authority, dropping capabilities, changing compatibility or retiring live paths. Link to existing task/issue.
6. **Implement:** write only to assigned owning repo, with exact base SHA and a rollback path. Protect live agents, receipts, device shells and provider-send owners.
7. **Verify:** run tests and negative consumer checks, compare expected/actual capability set, and record unverified gates. No physical phone acceptance claim from cloud tooling.
8. **Reconcile:** every round publishes newly decided, changed/superseded and pending decisions, open items, exact evidence and next executable actions. This Labs log is historical provenance; HQ-generated views consume it read-only.

**No silent losses:** each unique component must point to an adopted source, retained independent implementation, adapter or explicitly deferred item. **No silent renaming:** native IDs and source revisions remain stable. **No silent write-authority change:** HQ indexes, Host effects, Harness roadmap and TierFlow visual contracts remain separately governed until proven migration.
