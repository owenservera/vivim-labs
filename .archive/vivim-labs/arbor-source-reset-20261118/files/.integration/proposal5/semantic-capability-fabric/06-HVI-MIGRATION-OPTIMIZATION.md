# How SCF turns costly HVI archaeology into cheap deterministic comparisons
**GOVERNING HVI CORRECTION:** Implementation/readiness tests are **synthetic invented Git fixtures only**. HVI `10-NATIVE-TOOLING-ONLY-SCOPE.md` defers real VIVIM donor/Labs/ARBOR migration comparisons until explicitly requested. Full implementation-ready kit: [implementation-ready/README.md](implementation-ready/README.md).

HVI `03-NWAY-CONTRACT.md` and six A–F workers stay unchanged. SCF adds a **preferred input ABI**, not a new N-way comparator or merge authority. Do not modify their active branches.

## Today (unannotated legacy)
Tree SHA → path/hash match → relocation candidate → symbol and contract analysis → source graph → ownership guess/review → tests/runtime proof. This is appropriate for the original 676 HQ files and the 361 absent paths; no annotation can retroactively prove what old code did.

## Tomorrow (SCF-enrolled source)
Given two or more immutable versions of the **same repo or different repos**:
1. Resolve 2..N Git commit pins exactly once.
2. Read compiled SC-ABI for each. Each unit already declares stable feature memberships + owner + contracts and has static-evidence diagnostics.
3. Join by canonical feature ID across pins; join by stable unit ID if present or exact Git blob/real symbol when historical; preserve multiple files per capability and multiple capabilities per file.
4. Compute five independent deltas: `source`, `semantic_feature`, `contract`, `test/proof`, `runtime/integration`. A feature can retain same ID while implementation moves; structural change is not automatic semantic regression.
5. Emit lineage: `unchanged`, `changed`, `new`, `moved_exact`, `split_candidate`, `combined_candidate`, `removed_at_path`, `unmapped`, `blocked`, `unknown_incomplete`. Verified replacement requires source and owner evidence.
6. HVI's existing report generator produces migration atlas, lost-code shortlist, dependency blast radius and owner merge plan. No reinterpretation of Host authority.

## Worked example (illustrative)
At ARBOR commit `A`, files `snapshot.mjs` and `compare.mjs` both carry `@hq:capability vivim.hvi.compare-revisions@1`. At commit `B`, `compare.mjs` moves under another directory but maintains its semantic unit ID; `snapshot.mjs` gains another feature membership. HVI knows **from metadata** that the feature is still represented, knows **from Git/symbol evidence** whether the implementation moved or changed, and knows **from independent test receipts** whether functionality still works. It does *not* claim runtime parity just because the same ID occurs in both sources.

## Coverage obligation
For every scanned source pin publish:
- `eligibleFiles`, `annotatedFiles`, `parsedValid`, `semanticResolved`, `verifiedStatic`, `testedObserved`, `runtimeVerified`, `excludedFiles`, `unknownFiles`.
- `featureCoverage` with denominator defined by registered owner-approved capabilities, not file count.
- `annotationStaleness`, `ownershipConflict`, `unmappedExternal` and `unprovenMigration` counts.
- Break apart evidence from annotations, source observation, CI and device/Host receipts.

## Critical edge cases
- If file X includes IDs A/B/C but none of its symbols legitimately implement B: flag declared-without-evidence; do not copy B into current application feature list as active.
- If feature A has no declared files but code is present: label `UNANNOTATED_IMPLEMENTATION_CANDIDATE`, not missing.
- If an implementation changes from JS to Rust: feature ID continuity + explicit source lineage + ABI/test parity, not just substring similarity.
- If annotations are copied into dead code: reachability and runtime checks expose source-only implementation.
- If a branch tip moves: existing snapshot remains at old SHA; new SHA yields separate SC-ABI.
- If same repo is pinned at three SHAs: timeline and pairwise matrix are valid; maintain each pin independently.
- If Labs documentation refers to a proposed capability only: `DESIGN_ONLY` until actual code and tests/read path are admitted.
- If Host revokes source access: previously compiled public facts may remain as permitted, but any sensitive derived content is redacted and access rechecked before disclosure.

## Migration policy
Adopt SCF first in *new or touched ARBOR code*. Backfill high-value canonical interfaces with explicit source evidence, not mass auto-tagged guesses. Use HVI independent corpus audit as **starting truth baseline**, and then gradually convert ambiguous findings into verified lineage decisions. Never auto-merge missing legacy HQ files solely because the comments claim equivalence.
