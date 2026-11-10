# Z.ai round N — integration and strategic-decision checkpoint

**Status:** TEMPLATE; create a distinct `rounds/RNN-REPORT.md` and `rounds/RNN-MANIFEST.json` for each executed round. Avoid inventing tests, counts or SHAs.

## 1. Four-repository pinned baseline

| Repo | Remote base SHA | End SHA | Change class | Tests (command + exit) | Bundle name + verify |
|---|---|---|---|---|---|
| repos-hq | TBD | TBD | TBD | UNRUN | TBD |
| vivim-web-harness | TBD | TBD | TBD | UNRUN | TBD |
| tierflow | TBD | TBD | TBD | UNRUN | TBD |
| vivim-labs | TBD | TBD | TBD | UNRUN | TBD |

Record actual environment, start time, tool versions, dependency/install reproducibility and protected source ownership. Do not claim all four repos changed if only one changed. **A report-only baseline is not a successful round: state the tested implementation improvements and HQ operating-tool gains.**

## 2. HQ operating-tool improvement and reuse

Record:
- **Task attempted via HQ:** actual CLI/context/index/graph/wiki queries or tools, their source pins, result quality, freshness and failures.
- **Baseline versus end-of-round:** questions answered without broad source exploration; repeated searches avoided; raw files inspected; stale/missing source edges; task-scoped context size/tokens or latency where measurable. Report unknown metrics honestly.
- **HQ repairs landed:** identity/source adapters, missing call chains, index freshness, graph relationships, CLI projections, context assembly, test selection, receipts, documentation regeneration; exact commits and regression tests.
- **Next-round reuse proof:** demonstrate a realistic follow-up engineering question answered using the enhanced HQ tool with a bounded context bundle and source references. Never substitute an ungrounded model guess for a passing query.
- **Efficiency goal:** progressively lower re-exploration while maintaining or increasing source accuracy. In Round 4, HQ should be the primary discovery/navigation/testing interface.

## 3. Conceptual duplicate discovery

For **each concept**: stable source-linked identifier; all independent implementations (repo@commit:path#symbol); behavior overlap; unique contributions; state/effect owners; tests and observed callers; classification; evidence confidence; best-of-breed decision and rationale; exact components to merge/retain; retirement candidates with safe negative-consumer proof; unresolved risks. Include newly found orphans and capabilities that previous catalogs missed.

## 4. Maximal single-version roadmap delta

List: newly discovered concepts; one selected go-forward owner per authority; unique abilities not yet integrated; updated compatibility edges; current V1 closure tasks versus deferred maximal capabilities; exact next work order (source-owned), dependencies, receipts and acceptance gates. Publish a derived HQ view, not a new Labs status database.

## 5. Strategic decisions and unresolved items

- **New/updated ZAID entries:** link to [DECISIONS](../governance/DECISIONS.md), state explicit owner-approval status.
- **New/updated/resolved/carried ZAIQ entries:** link to [OPEN-ITEMS](../governance/OPEN-ITEMS.md) and source-owned work order.
- **Decision debt:** unchosen canonical candidates and required evidence.
- **Feature preservation:** every dropped module's unique value traced to destination or acknowledged deferment.

## 6. Source/test receipts and rollback

Describe exact file edits, SHA, migration/rebuild sequence, negative consumer checks, compatibility, tests with exit codes, safety/publication constraints, user approvals, rollback and remaining unverified device/Windows gates.

## 7. Bundle checkpoint

Use [BUNDLE-PROTOCOL](./BUNDLE-PROTOCOL.md). Four bundle filenames, SHA256 hashes, `git bundle verify` outputs, restore test (or explicitly UNRUN), exact refs, new manifest SHA, application instructions and source availability. Never commit raw private bundles to public Labs unless separately security cleared.

## 8. Quality scorecard and stretch findings

Include: index source coverage, concepts classified, duplicates resolved versus pending, unique capabilities retained, runtime imports/consumers proved, documentation and graph freshness, automation improvements, model/test costs and all unverified conditions. Evaluate optional [STRETCH-GOALS](../STRETCH-GOALS.md) only after mandatory gates.

## 9. Next round admission

GO | GO_WITH_LIMITS | BLOCKED; exact conditions, risks and next bounded work. One accountable integrator must sign off the coherent four-repository checkpoint.
