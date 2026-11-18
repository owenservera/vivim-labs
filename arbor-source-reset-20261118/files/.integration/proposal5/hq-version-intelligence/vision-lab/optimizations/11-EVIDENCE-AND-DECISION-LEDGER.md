# 11 — Evidence, unknowns and future decisions
## Current state, as of pinned source inspection
- **Verified:** 59 specific candidate module paths exist at ARBOR `98f15e327957956ec94189772d5e5813e7b6b07b`; exact blob SHAs recorded in machine data.
- **Verified:** HQ SQLite WAL NORMAL and TierFlow WAL FULL; Studio log defaults; Rust pure/typed DAG; Rust Journal compaction source; ignored benchmarks; Stage-only Chronos contract.
- **Historical source comment only:** prior Rust debug target size 4.9GiB; NOT current disk usage.
- **Unknown:** live entrypoint frequency, idle duty cycle, native hotpath usage, database page counts, actual RSS, I/O wait, p95/99, battery drain, platform acceptance and current developer build cost.
- **Not done:** execution of tests, profiler, synthetic benchmark runner, real repository comparison, migration, source changes or runtime security assessment.

## Evidence classification
`SOURCE_OBSERVED` = actual pinned source exists; `CONFIG_OBSERVED` = specific literal default/contract read; `HISTORICAL_REPORTED` = older measurement appears in source commentary, not independently reproduced; `HYPOTHESIS` = plausible optimization; `MEASURED` = executed, repeatable profiler receipt; `PARITY_VERIFIED` = exact correctness/negative tests; `OWNER_DECIDED` = explicit owner decision; `INTEGRATED` = committed and accepted product change.

## Stop signs and ownership
- **No loss of authority:** Host remains sole effect writer, Studio browser sender, TierFlow publisher, HQ dev intelligence, Rust canonical pure graph machinery.
- **No data-loss benchmark:** if a faster path drops raw events, changes fsync ACK semantics or evicts protected originals, it fails.
- **No silent clock correctness change:** time optimization must preserve sequence, expiry, idempotency and replay.
- **No compiler “speedup” that broadens accepted input:** exact refusal parity is mandatory.
- **No unmeasured Rust port:** source-language migration is an architectural decision, not a priority-score consequence.
- **No real HVI repository comparison:** this research stays subordinate to `10-NATIVE-TOOLING-ONLY-SCOPE.md` and defers live corpus audits.

## Fields for a later decision
`candidateId, evidencePin, observedCallerFrequency, measuredBase, measuredVariant, effectSizeAndUncertainty, correctnessReceipt, authorityReview, privacyReview, diskReserve, rollbackMethod, maintainabilityDelta, sourceOwner, approvalRef, finalDecision, decisionDate`.

## Relationship to nearby Proposal 5 research
- [Rescore expansion](../../../rescore-expansion/README.md) asks **which existing JS computation might move to Rust?**
- This optimization observatory asks **how should ARBOR spend fewer CPU cycles, memory bytes, disk writes, wall time, energy and engineering hours while remaining correct?**
- HVI Vision Lab asks **what could a future living software universe feel like?**
All three are distinct; any overlap references the original canonical source rather than creating a duplicate implementation.
