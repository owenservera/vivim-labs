# Evidence-led report contract — SOURCE-AGNOSTIC

Produce real human and machine artifacts. Keep sensitive content in approved local storage. Public GitHub receives sanitized receipts and summaries only.

## Mandatory outputs
```text
RUN-RECEIPT.json
01-EXECUTIVE-SUMMARY.md
02-REPOSITORY-REVISION-MATRIX.md
03-CAPABILITY-LANDING-MATRIX.md
04-MISSING-REPLACED-AND-DUPLICATE-AUTHORITIES.md
05-LABS-AND-RESEARCH-STATUS.md
06-PRIORITIZED-BACKLOG.md
07-VERIFICATION-AND-UNCERTAINTY.md
data/source-snapshots.json
data/source-parity.json
data/lineage.json
data/capability-atlas.json
data/evidence-index.json
data/backlog.json
```

Every source row carries `sourceId`, `snapshotId`, `versionType`, `contentDigest`, `completeness`, `capturedAt`, `sourceGrantReceiptId` (never secret), and **optionally verified** `commitSha`/`treeSha` for Git-specific sources. Never invent a 40-hex commit SHA to satisfy the older schema. Explicitly distinguish `local-captured-snapshot` from verified commit history.

Every finding includes source and target snapshot ID and actual path/digest identity, evidence IDs, confidence, source/capability/runtime status, explanation, ownership and recommendation. Source file presence is not capability presence; structural similarities are candidates; research documents are not deployed runtime. Never infer erased functionality from absent path alone.

Executive report: exact selected source snapshots, headline counts (unchanged, changed, moved, absent-at-capture, incomplete), confirmed capability landing vs unknown, high-impact source findings, security/authority overlaps and five evidence-linked proposed priorities. Do not claim a full historical repository comparison when only current folder captures are available.

`RUN-RECEIPT.json` records exact source connector used per input, real-input provenance, snapshot digest, optional Git pin if verified, actual executable commands/exits, report digests, truncation/unknown count, source mutation false, grant scope and `realComparisonExecuted` true/false. Report status `OPERATIONAL_REAL_COMPARISON_COMPLETE`, `PARTIAL_REAL_COMPARISON`, `BLOCKED_SOURCE_ACCESS` or `BLOCKED_RUNTIME` grounded in actual receipts.

Do not automatically implement suggested migrations, delete duplicates or write donor repositories.
