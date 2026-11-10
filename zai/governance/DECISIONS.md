# Strategic decision register — campaign evidence only

**Purpose:** Explain *where and why* Z.ai makes architecture or implementation selections. Append new decision entries; do not rewrite prior rationale. Owner approvals and operational status live in the existing authoritative system, not here.

## Required entry template

### ZAID-YYYYMMDD-NNN — Short concept title

- **Round / date / proposer:**
- **Status:** PROPOSED | OWNER_APPROVED | REJECTED | SUPERSEDED | IMPLEMENTED_VERIFIED
- **Decision class:** SELECT_CANONICAL | COMPOSE_BEST_OF_BREED | KEEP_INDEPENDENT | KEEP_AS_ADAPTER | RETIRE_AFTER_MIGRATION | ARCHIVE_HISTORICAL | DEFER_PENDING_PROOF
- **Concept/intent and affected users:**
- **Candidates with immutable provenance:** repo, full commit SHA, path, symbol, consumer
- **Overlap:** features shared by candidates
- **Distinct value:** what each implementation does that alternatives do not
- **Alternatives considered:** including doing nothing
- **Best-of-breed rubric + hard-gate evaluation:** cite actual tests, unknowns and source ownership
- **Decision and rationale:** identify go-forward responsibility and components to carry over
- **Impact:** existing authority, migration plan, compatibility, footprint and dependencies
- **Implementation tasks and source-owned work orders:** IDs/link only; no shadow task state
- **Verification and rollback:** evidence links and explicit unverified gates
- **Open items linked:** ZAIQ-* IDs from OPEN-ITEMS.md
- **Owner authorization evidence:** link to approved issue/record; do not assume approval
- **Supersedes / superseded by:**

## Initial policy decisions (owner-provided, not Z.ai architecture inventions)

- **ZAI-BASE-001:** Exactly four primary VIVIM repositories; `vivim-ui` is local folder alias for GitHub `tierflow`.
- **ZAI-BASE-002:** Existing HQ indexing, Rust graph and wiki generation are the single cross-repo development intelligence backbone.
- **ZAI-BASE-003:** Harness retains live effects, Host, one Android app and current product roadmap authority pending verified migration.
- **ZAI-BASE-004:** Labs archives/evidence are public-safe only; no separate live capability/roadmap authority.
- **ZAI-BASE-005:** Prefer composed best-of-breed solutions and preserve unique capabilities before retiring duplicate code.

These are scope policies; future ZAID decisions must carry candidate evidence, authorization and tests.
