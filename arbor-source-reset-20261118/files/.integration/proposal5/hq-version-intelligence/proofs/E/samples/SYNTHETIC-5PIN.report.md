# ARBOR HQ Version Migration Report

**Target:** synthetic4  **Status:** SOURCE-ONLY / NOT RUNTIME ACCEPTED

Findings: 6; page 1–6 of 6.
Evidence completeness: **incomplete\_source\_evidence**. Incomplete findings: 2.
Page truncated: **false**; further pages: **false**.

## Pinned snapshots

| Pin | Repository | Immutable commit | Scan |
|---|---|---|---|
| synthetic0 | synthetic/repo0 | `1111111111111111111111111111111111111111` | partial |
| synthetic1 | synthetic/repo1 | `2222222222222222222222222222222222222222` | complete |
| synthetic2 | synthetic/repo2 | `3333333333333333333333333333333333333333` | complete |
| synthetic3 | synthetic/repo3 | `4444444444444444444444444444444444444444` | complete |
| synthetic4 | synthetic/repo4 | `5555555555555555555555555555555555555555` | complete |

## N-way compact matrix

| Kind / ID | Target-relative finding | synthetic0 | synthetic1 | synthetic2 | synthetic3 | synthetic4 |
|---|---|---|---|---|---|---|
| capability:cap:synthetic | DESIGN\_ONLY | UNOBSERVED | UNOBSERVED | UNOBSERVED | UNOBSERVED | UNOBSERVED |
| source:delta0 | UNOBSERVABLE | UNOBSERVED | UNOBSERVED | UNOBSERVED | UNOBSERVED | UNOBSERVED |
| source:delta1 | EXACT\_RELOCATION | UNOBSERVED | EXACT\_RELOCATION | UNOBSERVED | UNOBSERVED | UNOBSERVED |
| source:delta2 | SPLIT\_CANDIDATE | UNOBSERVED | UNOBSERVED | SPLIT\_CANDIDATE | UNOBSERVED | UNOBSERVED |
| source:delta3 | EXACT\_RELOCATION | UNOBSERVED | UNOBSERVED | UNOBSERVED | EXACT\_RELOCATION | UNOBSERVED |
| source:delta4 | SPLIT\_CANDIDATE | UNOBSERVED | UNOBSERVED | UNOBSERVED | UNOBSERVED | SPLIT\_CANDIDATE |

## Reviewed target deltas

| Finding | Source path:line | Target paths | Category / migration | Confidence | Evidence | Owner |
|---|---|---|---|---|---|---|
| cap:synthetic | docs/plan\.md | unknown | DESIGN\_ONLY / other | unknown (unknown) | incomplete | labs |
| delta0 | src/module\-0\.mjs:10-20 | unknown | UNOBSERVABLE / other | 0.5 (candidate) | indexed | hq |
| delta1 | src/module\-1\.mjs:10-20 | hq/relocated\-1\.mjs | EXACT\_RELOCATION / relocated | 0.5 (candidate) | indexed | hq |
| delta2 | src/module\-2\.mjs:10-20 | hq/relocated\-2\.mjs | SPLIT\_CANDIDATE / split\_candidate | 0.5 (candidate) | indexed | hq |
| delta3 | src/module\-3\.mjs:10-20 | hq/relocated\-3\.mjs | EXACT\_RELOCATION / relocated | 0.5 (candidate) | indexed | hq |
| delta4 | src/module\-4\.mjs:10-20 | hq/relocated\-4\.mjs | SPLIT\_CANDIDATE / split\_candidate | 0.5 (candidate) | indexed | hq |

## Priority-aware owner review plan

| Priority | Finding | Owner | Review action |
|---|---|---|---|
| P1 | delta0 | hq | Complete source scan and corroborate provenance |
| P1 | delta2 | hq | Require owner review of split lineage |
| P1 | delta4 | hq | Require owner review of split lineage |
| P2 | cap:synthetic | labs | Retain as design; no runtime claim |

Plan bounded: 4/4; truncated: false.

## Test receipt states (source reports are not acceptance)

| Test ID | Reported state | Command receipt | Exit | Trust |
|---|---|---|---|---|
| synthetic\-node\-test | UNRUN | false | UNRUN | unverified_claim |

## Limitations

- UNOBSERVED means no confirmed state for a pinned snapshot, not absence
- Evidence metadata is projection only; no file contents, test stdout or permission grants
- Runtime acceptance requires independent Host/device gate and verified receipts

Reproducibility digest (report projection): `34907a01203c4d10db4bea58f4caad889591bfe21238de2addc55d66853aa941`
