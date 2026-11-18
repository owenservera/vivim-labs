# ARBOR HQ Version Migration Report

**Target:** synthetic1  **Status:** SOURCE-ONLY / NOT RUNTIME ACCEPTED

Findings: 3; page 1–3 of 3.
Evidence completeness: **incomplete\_source\_evidence**. Incomplete findings: 2.
Page truncated: **false**; further pages: **false**.

## Pinned snapshots

| Pin | Repository | Immutable commit | Scan |
|---|---|---|---|
| synthetic0 | synthetic/repo0 | `1111111111111111111111111111111111111111` | partial |
| synthetic1 | synthetic/repo1 | `2222222222222222222222222222222222222222` | complete |

## N-way compact matrix

| Kind / ID | Target-relative finding | synthetic0 | synthetic1 |
|---|---|---|---|
| capability:cap:synthetic | DESIGN\_ONLY | UNOBSERVED | UNOBSERVED |
| source:delta0 | UNOBSERVABLE | UNOBSERVED | UNOBSERVED |
| source:delta1 | EXACT\_RELOCATION | UNOBSERVED | EXACT\_RELOCATION |

## Reviewed target deltas

| Finding | Source path:line | Target paths | Category / migration | Confidence | Evidence | Owner |
|---|---|---|---|---|---|---|
| cap:synthetic | docs/plan\.md | unknown | DESIGN\_ONLY / other | unknown (unknown) | incomplete | labs |
| delta0 | src/module\-0\.mjs:10-20 | unknown | UNOBSERVABLE / other | 0.5 (candidate) | indexed | hq |
| delta1 | src/module\-1\.mjs:10-20 | hq/relocated\-1\.mjs | EXACT\_RELOCATION / relocated | 0.5 (candidate) | indexed | hq |

## Priority-aware owner review plan

| Priority | Finding | Owner | Review action |
|---|---|---|---|
| P1 | delta0 | hq | Complete source scan and corroborate provenance |
| P2 | cap:synthetic | labs | Retain as design; no runtime claim |

Plan bounded: 2/2; truncated: false.

## Test receipt states (source reports are not acceptance)

| Test ID | Reported state | Command receipt | Exit | Trust |
|---|---|---|---|---|
| synthetic\-node\-test | UNRUN | false | UNRUN | unverified_claim |

## Limitations

- UNOBSERVED means no confirmed state for a pinned snapshot, not absence
- Evidence metadata is projection only; no file contents, test stdout or permission grants
- Runtime acceptance requires independent Host/device gate and verified receipts

Reproducibility digest (report projection): `e2155324c41e480bd40a338d88f8998fb9446d23e25478ab3700592788fbeae1`
