# Typed SV protocol: proof obligations, witness trace and receipts
## Linked machine artifacts
1. **ClaimSpec** (`arbor.sv.claim/0.1`): reviewed statement tied to `unit@version`, feature(s), source pin, contract, effect boundary, spec pin/digest and independent oracle refs. May contain several properties.
2. **ObservationProfile** (`arbor.sv.observation/0.1`, proposed): exact allowed event types/fields, redaction, start/end fence and completeness/coverage conditions. A closed-world claim must have a trusted owner-provided boundary and complete event set; absence from partial telemetry is never proof of no effects.
3. **BuildProvenance** (`arbor.sv.build/0.1`, proposed): Git tree/commit SHA, dependency lock digests, compiler/toolchain version, build flags and resulting executable/container/module digest. Test receipt binds the **actual artifact that ran**.
4. **ProofReceipt** (`arbor.sv.proof-receipt/0.1`): one result per property and source/build/model pin, method, runner identity, independent oracle pin, assumptions, test/failure details, execution exit, trace digest/coverage and checker verdict.
5. **ExplanationEnvelope** (`arbor.sv.explanation/0.1`): sentence/claim-ID and qualifier/evidence bindings, with independently checked verdict; only verified clauses may be represented as proven.

## Proof obligation = specification, not just a tag
A property must state who approved the requirement and what observations could falsify it. E.g. `pinned_sha_unchanged` is a data-only predicate comparing input/resolved SHA at the same repo pin; a `no_host_write` property requires owner-provided complete effect boundary instrumentation or static proof, not just an empty `writes` array.

## Small predicate algebra (proposed restricted grammar)
Allow only typed primitives from a versioned verifier predicate registry: `eq`, `neq`, `schema_valid`, `subset`, `unique`, `hash_equal`, `ordered`, `within`, `before`, `state_transition`, `event_count`, `error_is`, `replay_stable`, `bounded_resource`. Combinators `all`, `any`, `not`, `implies` only on bounded typed operands; no evaluation of arbitrary JS/Rust/SQL/regex with catastrophic ambiguity. A predicate result on sampled observations is `SUPPORTED_BY_TESTS`, not universal `PROVED_UNDER_MODEL`.

## Proof independence
- Code author may supply annotations and suggest tests, but a trusted **separate owner or test harness** must supply accepted verification oracle and the checker version. A single agent changing code, test and verifier together does not establish independent proof.
- Built binary digest, Git source digest, toolchain manifest and test commands/exits must be bound in receipt. Source compilation is a separate property; code compiled isn't evidence it was deployed, invoked or produced an external effect.
- Runtime evidence must be from existing owner instrumented boundaries (Host receipt/journal, Studio send observation, UI publisher revision) with exact claim scope. External provider delivery may remain `UNKNOWN`.
- Cases lacking toolchain, trace completeness, authoritative oracle, verified runner or consent return `UNKNOWN` / `BLOCKED`, never `true`.

## Proposed v0.1 JSON schemas
`contracts/claim.v0.schema.json`, `contracts/proof-receipt.v0.schema.json`, `contracts/explanation.v0.schema.json`. These provide a strict first pass; later schema versions may add richer AST/attestation but may not downgrade trust claims without a migration and negative tests.
