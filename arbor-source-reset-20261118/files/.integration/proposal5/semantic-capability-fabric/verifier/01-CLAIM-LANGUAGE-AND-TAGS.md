# SV meta-language v3 extension — tags link source units to separately reviewed obligations
**Compatibility:** v1 nine explanatory tags and v2 nine typed tags remain unchanged. Current `hq/lib/annotation-v2.mjs` does **not** parse these proposed v3 tags. The SWC v3 proposal contains `@hq:example`, `@hq:constraint`, `@hq:migration` etc; SV adds a few references within that **same future language**, not a competing syntax.

## Minimal source tags
For first-party source headers: existing `@hq:context`, `@hq:owner`, `@hq:capability`. For contract-bearing exports/functions: existing `@hq:contract`, `@hq:effect`, `@hq:test` plus three opt-in v3 reference tags:
- **`@hq:verify registered.verification-profile@1`**: points to a signed/reviewed data-only contract bundle defining properties and oracles, not shell commands or user-authored executable assertions.
- **`@hq:observe registered.observation-profile@1`**: names a reviewed observation schema/boundary and completeness rule, not an instruction to instrument arbitrary sensitive traffic.
- **`@hq:explain registered.explanation-pack@1`**: points to constrained, claim-bound explanation templates, not a raw natural-language assertion of functionality.
Other existing/proposed tags: `@hq:unit` for stable source unit, `@hq:role`, `@hq:migration`, `@hq:relation`, `@hq:invariant`, `@hq:risk`, `@hq:why`, `@hq:example`. No new `@hq:run`, `@hq:executor`, `@hq:grant`, `@hq:command`, or arbitrary `@hq:assert <JS>`.

## Example: one function with 3 independently checkable statements (ILLUSTRATIVE)
```ts
/**
 * @hq:context Resolve an immutable Git source snapshot for development HQ.
 * @hq:owner hq
 * @hq:unit arbor.hvi.source.snapshot-at@1
 * @hq:capability arbor.hvi.pin-source@1
 * @hq:capability arbor.hvi.compare-revisions@1
 * @hq:contract arbor.hvi.snapshot-at@1
 * @hq:effect host:read-only
 * @hq:verify arbor.hvi.snapshot-at.verify@1
 * @hq:observe arbor.hvi.snapshot-at.observe@1
 * @hq:explain arbor.hvi.snapshot-at.explanation@1
 * @hq:test test:tests/hq-version-intelligence/snapshot.test.mjs
 * @hq:invariant Existing pinned commits are never silently retargeted.
 */
export function snapshotAt(pin: PinnedSource): SourceManifest {
  // Actual code must be independently inspected and exercised.
}
```
IDs and test paths are **hypothetical**, not installed. A single function may claim several feature memberships but have independent proof obligations for each declared behavior.

## Semantic contract granularity
A source unit's reviewed spec links to **one or more property IDs**: `P1` input validation, `P2` output correctness, `P3` resource bounds, `P4` forbidden effects, `P5` error semantics, `P6` performance only if explicitly bounded to measured hardware/conditions. Each property explicitly defines:
- quantifier: `sampled_inputs`, `bounded_all_states`, or `all_inputs_with_formal_proof`;
- domain, assumptions, preconditions, allowed inputs and versioned type refs;
- postconditions, allowed external side effects and forbidden transitions;
- independent evidence method / oracle / reference model and required evidence;
- observation surface, target environment and acceptable result state;
- failure/refusal semantics and severity.

## Keep comments small, specs rich
Do **not** stuff arbitrary postconditions or scripts into source comments. Store typed, reviewed constraints in source-controlled JSON schemas/registry refs within the approved owner domain, pinned by digest and reviewed independently. The agent adds a few references; HQ resolves them and derives compilation/test impact. Coding agent may propose a new spec in PR, but cannot independently ratify a safety-critical spec and certify it with code it wrote.

## Function attachment
The scanner must resolve real syntax-tree attachment to exported function/method/type, not file-wide text nearest in line number. Separate file-level feature membership from function-level behavior obligations. Generated/vendor/non-owned code gets `OUTSIDE_POLICY` and source coverage, not auto-generated claims. Unknown v3 tags in old parser remain `UNSUPPORTED_GRAMMAR`, never silently interpreted as verified.
