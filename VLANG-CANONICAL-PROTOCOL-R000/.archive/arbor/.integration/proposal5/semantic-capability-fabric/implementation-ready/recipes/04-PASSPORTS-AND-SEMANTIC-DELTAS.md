# Recipe C1 — capability passports and 2..N semantic deltas
**Owner C.** New `semantic-capability/{passport,semantic-delta,hvi-adapter}.mjs`. Exactly one HVI native comparator; SCF supplies optional semantic evidence, not a new source compare engine.

## Passport builder
```ts
function buildCapabilityPassports(facts, evidenceReadPort,{pin,coverage}) {
  const ids = uniqueSorted(facts.filter(f=>f.predicate==="supports").map(f=>f.objectRef));
  return ids.map(id => {
    const contributors = facts.filter(f=>f.predicate==="supports"&&f.objectRef===id)
      .map(f=>({path:f.source.path,blobSha:f.source.blobSha,
                scope:f.scope,unitRef:f.unitRef,staticStatus:f.staticStatus,role:f.role}));
    const contracts = relatedUnique(facts,id,"subject_to_contract");
    const tests = resolveTestsWithThreeStages(facts,id,evidenceReadPort);
    const claims = bindVerbatimIndependentReceipts(id, pin, evidenceReadPort);
    const verdict = deriveHonestAggregate(facts,tests,claims,coverage);
    return freezeValidatedPassport({schema:"arbor.scf.capability-passport/0.1",
      capabilityId:id,pin,owner:acceptedOwner(id),contributors:stableSort(contributors),
      contracts,tests,claims,coverage,verdict,snapshotDigest:digest()});
  });
}
```
No direct `UNKNOWN → false` collapse; `REFUTED` visible even when other tests pass. Multiple contributors may have different roles; don't imply all are canonical writer. Excluded/unannotated source counts remain explicit.

## HVI adapter contract
Target existing native HVI agent-discoverable service only after G1/G2 acceptance; check exact code and exported interface at implementation start. Adapter signature `compareCapabilityRevisions([{pin,passports,digest}],{targetPinId,hviCompare})` calls supplied HVI comparison service for **raw file-level** source lineage and N-way pin validation, then joins semantic facts by approved capability ID. If HVI isn't ready, only synthetic pure semantic-delta/unit tests are allowed; product gate BLOCKED, no independent `git clone` comparator as fallback.

## 2..N algorithm
1. Verify 2..N independently immutable pinIds, target among pins, no duplicated pinId with conflicting digests. Same repo with SHA A/B/C must remain three distinct columns; no mutating old pin when branch advances.
2. For each canonical approved feature ID build `[pinA: contributors, pinB: contributors,...]`. Versioned ID change (e.g. @1 to @2) records distinct capability records with explicit owner-approved lineage, NOT string-match "same".
3. Join each contributing source unit by exact blob/source symbol first, optional accepted `@hq:unit` identity second, explicit owner-approved migration/alias third; name-only or code similarity generates `CANDIDATE_REPLACEMENT` not verified equivalent.
4. For each pin calculate five orthogonal deltas: source files, capabilities, public contracts, test/proof receipts, runtime/Host evidence. Scans incomplete => `UNOBSERVED`, never `MISSING`.
5. Cross-language Rust↔TS rewrite with stable feature ID is continuity of *declared feature*, **not implementation equivalence**; require contracts, independently approved proof/model and actual integrated status for higher verdict.
6. Output `arbor.scf.semantic-delta/0.1` bounded pages: target-pin ID, N pins, feature-state matrix, source evidence refs, current vs prior certainty and unknown counts. Digest canonical.
7. Retired capabilities require an explicit owner tombstone with source/authority receipt. Absence of a tag never means retired.

## Example decisions
- `file A` supports features X+Y, moves paths unchanged blob => exact relocation, both memberships preserved.
- Same capability X supported by Rust module without test => `present_declared/static_checked`, not runtime equivalent.
- New version with zero tags but index still finds exported API => `UNANNOTATED_CANDIDATE`, not lost feature.
- A product action named X is described in source but Host catalog says no registered handler => `SOURCE_WIRED_UNVERIFIED` or `BLOCKED` depending on mandatory contract, not `RUNTIME_VERIFIED`.
- Snapshot with incomplete read permission => all affected comparisons `UNOBSERVED`, no conclusions from missing entries.

## Synthetic-only acceptance
Use invented repositories/fake blobs and content to exercise 3 same-repo SHAs, five synthetic repos, relocation, split/combine, cap rename, explicit owner lineage, unknown source and corrupt digest. Do not compare actual VIVIM donor/Labs snapshots under HVI's governing tooling-only scope. Emit deterministic JSON and Markdown samples with no private paths.
