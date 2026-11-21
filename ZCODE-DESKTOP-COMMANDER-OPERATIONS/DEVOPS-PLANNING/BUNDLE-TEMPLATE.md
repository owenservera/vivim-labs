# Ready-to-code bundle template — DevOps / Planning

**Purpose:** one OW-scoped source-verifiable handoff for **ChatGPT Dev**, with actual Rust code and tests where useful. This file is not a new backlog. Use [PROFILE](PROFILE.md), [conversation lessons](CONVERSATION-LESSONS.md) and [bundle index](BUNDLE-INDEX.md) to avoid repeated research.

## A. Identity and user-visible finish line
- OW ID or *owner directive pending registry reconciliation*:
- Date/time zone; preparation owner; receiving Dev:
- Canonical product repo `owenservera/ARBOR`; Labs package location:
- Exact **fetched** ARBOR main commit SHA; exact Labs main SHA:
- User-visible scenario in one sentence:
- Non-goals and active Dev-owned overlapping surfaces:

## B. Existing source forensic disposition
| Exact path / exported symbol | Seen at SHA? | REUSE / REPAIR / NEW / UNKNOWN | Owner | Evidence |
| --- | --- | --- | --- | --- |
| | | | | |

- Dependency on other OW requests:
- Current producer -> transport -> Rust/Host consumer -> UI readback:
- **Distinguish first-contact PWA from general Wiki tab** where applicable:
- Existing implementation vs merely a design, old test, release artifact, or source annotation:
- File collision check: Dev's active files, Oracle's role files, uncommitted changes unknown:

## C. Implementation-ready change set
- Smallest vertical slice and stable user-visible acceptance:
- Paths to **prepared Rust modules**, signatures, structs, conversion, bounded inputs/outputs:
- Exact integration insertion points and module registration / feature gate:
- Reused graph retrieval / source identity / LiveSystemState / Host Control functions:
- Browser/DOM glue requiring TypeScript 7+ (why Rust impractical):
- Source/revision/grant/graph-epoch and privacy boundaries:
- Negative cases (stale/revoked/ambiguous/permission denied/human edit races):
- Migration/compatibility and rollback:
- Explicit **do-not-build-duplicate** list:

## D. Verification ladder and honest evidence
| Gate | Exact command or device action | Expected observable proof | Observed result |
| --- | --- | --- | --- |
| Static source and feature-gate check | | | UNRUN |
| Rust format/lint | | | UNRUN |
| Rust unit tests | | | UNRUN |
| Host/Rust/transport integration | | | UNRUN |
| PWA browser first-contact, live input and IME | | | UNRUN |
| Authorized agent delivery, owner approval, receipt and no-Send | | | UNRUN |
| Scope/revocation/privacy negative suite | | | UNRUN |
| Unchanged URL on real Android, screenshot/readback | | | UNRUN |

Do not populate a PASS cell without actual command, environment, exit code, relevant artifact/digest or device evidence. A source match and statically counted tests are **not** compile/test receipts. If cloud toolchain unavailable, name that blocker and preserve staged code.

## E. GitHub and recipient handoff
- Prep files created/updated on Labs `main`:
- Remotely verified prep commit SHA / link:
- ARBOR product files **not changed**, unless this role was explicitly authorized as source implementer:
- Freshness rule: revalidate code and fence contracts when ARBOR main or any owned file changes.
- Status: `PREPARED` / `BLOCKED_BY_DEPENDENCY` / `READY_FOR_DEV_REVIEW` (never implied `INTEGRATED`).
- Exact next Dev action, sorted by dependency:
- Recipient acknowledgment: `NOT_OBSERVED` until Dev confirms; do not assume cross-thread communication.
- Oracle disagreement/verification request if current claims conflict:
- If/when Dev pushes: authorized local deployment executor, rollback, unchanged PWA URL and Android-specific acceptance evidence:

**Rule:** a good bundle enables the coder to start from exact implementation paths and real tests, not from another open-ended design investigation.
