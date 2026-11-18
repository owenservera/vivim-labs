# Coding agent protocol — small declaration, compiler does the hard work
**Policy name:** SCF Agent Source Contract v0 (proposal; does not yet change active ARBOR lint). **Scope:** new and modified first-party source files and public interfaces; extensions for docs/contracts/tests. Roll out baseline-first and touched-file enforcement, not a surprise repo-wide release block.

## Before writing a new first-party file
1. Know the **canonical owner** and at least one **registered** capability ID or explicitly declare `unclassified_pending_review` in the structured PR metadata (not a fake @hq:capability). Read the relevant Capability Passport and verify no existing canonical implementation should be reused.
2. Add tiny **real source comment** near file header with `@hq:context`, `@hq:owner`, and repeat `@hq:capability name@version` for each real feature supported. For exported API/handler add `@hq:contract` and real `@hq:effect` when applicable. Tie tests with `@hq:test test:<repo-relative-test>` only where actual test exists; include `@hq:invariant` and `@hq:why` for non-obvious correctness/tradeoff.
3. Add a symbol-level comment for a specific exported function that materially changes semantics; file header is broader membership, never all functions automatically implementing every capability.
4. On every edit, update/remove stale tags in the same commit; do not add auto-approved IDs, permission claims or imaginary tests. Generated/vendor/binary/fixtures excluded by declared walker policy and source coverage counters.
5. Run `hq protocol check` and the eventual `hq semantic check --changed` after the parser is integrated. Missing tool means record UNRUN; do not assert checks passed. Add source/test receipts from the real CI run.
6. PR must include compiler-generated Capability Delta (new/changed/stale/unknown facts), impact, evidence gaps, observed test commands/exits and exact Git SHA. Do not hand-edit generated wiki or compiled passports.

## Routine file annotation (v1/v2 compatible)
```ts
/**
 * @hq:context Immutable source snapshots used by development intelligence.
 * @hq:owner hq
 * @hq:capability vivim.hvi.pin-source@1
 * @hq:capability vivim.hvi.compare-revisions@1
 * @hq:contract vivim.hvi.source-manifest@1
 * @hq:test test:tests/hq-version-intelligence/sources.test.mjs
 * @hq:invariant A resolved commit SHA is immutable for the lifetime of a pin.
 */
export function snapshotAt(pin: PinnedSource) { /* implementation */ }
```
**Illustrative only:** example IDs and test path must be registered/exist before real adoption; they are not claims about ARBOR's current source. Multiple `@hq:capability` tags are a many-to-many declared association; the compiler does not infer two runnable features from this alone. Existing v2 parser may parse annotations per provided source-symbol context; trustworthy file-vs-symbol binding requires the upcoming compiler.

## Low-friction tiers
- **Tier 0 / tiny helper:** file purpose + owner + one capability/explicit pending-classification; no repeated comments for every line.
- **Tier 1 / public boundary:** Tier 0 plus typed contract, effects, tests, consumers/emits if relevant.
- **Tier 2 / sensitive logic:** Tier 1 plus invariants, risk, decision and explicit proof obligations; reviewed owner action crosswalk.
- **Tier 3 / migration/refactor:** Tier 2 plus proposed v3 stable unit and registered migration lineage refs, original source SHAs, compatibility proofs and rollback.
Every tier uses the same compiler; high-risk tags create obligations, not capabilities.

## Agent PR handshake
PREPARE: pin repo SHA, read passports and target contracts.
DECLARE: annotate only touched source units with registered IDs.
IMPLEMENT: write code/tests; refuse unauthorized new writer/dispatcher.
COMPILE: lexer+registry+static scanner → proposed facts, typed errors.
VERIFY: source identity and changed dependency edges, actual tests and Host/owner facts.
REVIEW: human/owner review needed for new ID promotion, semantic equivalence, high-risk effect and release.
PUBLISH: accepted source commit creates immutable SC-ABI snapshot; wiki and HVI are derived. No auto-run/effect.

## Special cases
- A file implements three features: repeat capability ID, preserve common owner; compiler derives feature→file and role confidence separately.
- One feature has 40 files: one capability passport aggregates their contributions; no duplicated feature write authority.
- Split/rename: stable declared unit ID (future v3) + explicit reviewed lineage ref, plus Git blob/symbol evidence.
- Partial tests: test annotation is an **obligation**, not executed receipt.
- Agent-created stale tag: static/code/test mismatch yields diagnostic and blocks promotion rather than minting fictitious truth.
- Auto-generated files: no manual tag; annotate generator/source-of-truth and mark artifact derived.
- Non-first-party code: read-only source scan with incomplete annotations; never rewrite third-party code simply to satisfy policy.
