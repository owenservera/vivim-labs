# From agent protocol to eventual public SDK
## Minimum contributor obligations (not a full code rewrite)
**File header**: purpose, canonical owner, 1..N feature refs or explicit `UNCLASSIFIED` waiver in PR metadata.
**Public function / method**: stable unit ref when accepted, contract ID, effect class, verification profile and observation profile (where meaningful), tests linked, explanation template refs. Private simple helpers may inherit feature scope without their own heavy annotation.
**Effectful boundary**: owner-approved action ID, threat/risk model, explicit side-effect/negative obligations; no inline permission grant.
**Refactor/migration**: explicit reviewed old→new lineage spec and compatibility obligations; do not self-certify equivalence.
**Agent change record**: source SHA, changed claims, test candidates, owner-reviewed oracle, runner evidence and unresolved gaps.

## Agent loop
1. **Read** existing Capability Passport and verification profiles before edits.
2. **Propose** spec or new claim IDs separately from implementation; high-risk/owner changes require independent approval.
3. **Code** normal JS/TS/Rust; maintain minimal comment refs at file/export scope. No executable assertions in comments.
4. **Compile** SCF source semantics. The compiler emits claims and **diagnostics**, not a pass bit.
5. **Challenge** with SV: independently re-parse and compare, run bounded independent oracle tests, falsification/mutation and read approved owner runtime witnesses. Return per-property verdicts and counterexamples.
6. **Repair** or amend specs with reviewed reasoning. A failed property cannot be fixed merely by deleting the specification without a recorded owner decision and downstream impact check.
7. **Publish** exact test/verification receipt and verified explanation pack to existing HQ wiki/capability graph and HVI historical snapshot; no new writer. Source changed → invalidate old evidence.
8. **Accept** only at actual CI/owner gate; product effects still require Host approval/receipts.

## Future SDK structure (proposal)
`@vivim/source-protocol`: comment/parser ref/template.
`@vivim/semantic-compiler`: existing HQ SWC adapter; not duplicate registry.
`@vivim/claim-spec`: strict JSON Schema + typed predicate registry.
`@vivim/verifier-kernel`: minimal independent checker and proof receipt.
`@vivim/verifier-runners`: optional isolated Node/Rust/property/model harness, host observation readers.
`@vivim/passport`: existing HQ Wiki/HVI report projections with evidence/verdicts.
These are logical packages/names only, **not shipped modules**.

## Developer UX
An IDE hover over a tagged function shows `Declared 5 • Statical checks 4 • Supported 2 • Unknown 2 • Refuted 1`, with one-click replay of a **safe** counterexample and links to exact artifact/spec/test. A PR check comments on semantic delta and property proofs. In VIVIM UI, "Why is this feature labeled working?" opens verified claim breakdown; never a raw model confidence.
