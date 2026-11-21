# Independent Verifier Kernel — make compiler mistakes observable
## Source of trustworthy negative/positive answers
The proof checker validates each compiled *claim* without assuming that the compiler's own generated semantic graph is correct. The verifier works as a separately versioned program over immutable Git/source and trusted spec roots. A report of `PROVED_UNDER_MODEL` requires a formal derivation or accepted exhaustive bounded exploration, while source parsing, type checking and unit tests have narrower verdicts.

## Modules and independent verification stages
**K0 Pin & trust:** trusted developer source root/record, immutable source SHA+blob, build artifact digest, verifier/checker digest, spec digest, accepted oracle owner. Ref drift, source or artifact mismatch => `BLOCKED`.
**K1 Parse witness:** compiler emits source span, token/kind, unit attachment and semantic ref. Independent parser/AST walker re-derives and compares reference. Duplicate tags, strings masquerading as comments, wrong symbol attachments => `REFUTED`.
**K2 Semantic type checker:** check registry/ABI/schema version, owner mapping, effect type, typed predicate refs, status and non-circular proof dependency. No owner action ID invented by inline comment.
**K3 Static obligations:** inspect actual imports/calls/contracts, forbidden effects, reachable ownership and declared vs observed symbol behavior. Conservative unknown on reflection/dynamic imports, aliasing, FFI, unsafe code, untrusted libraries.
**K4 Dynamic checker:** execute **approved pure or sandbox-safe test targets** against independently reviewed oracles and mutation/property/metamorphic cases. All code under test untrusted. Run no user World effects, browser sends or credential access. Record exact binary/script hash, exit, seed, inputs, outputs, errors, coverage and trace digest. Any counterexample => `REFUTED`.
**K5 Bounded model/formal checker (optional):** for small pure state machines/transition systems and bounded input domains, exhaustively explore under **explicit assumptions** and check returned derivation using a minimal trusted checker. Properties of unrestricted code remain unknown unless legitimate proof exists.
**K6 Runtime witness checker:** verify Host/Studio/UI owner event receipts, actor/version/nonce and observation completeness. External work that was requested but not acknowledged as completed cannot be upgraded from `pending/uncertain` to success.
**K7 Explanation entailment checker:** require every factual clause to point to an accepted typed claim with identical source/build/spec scope and appropriate modality; refuse uncited/unverifiable broad claims.
**K8 Verdict aggregator:** combine independent evidence with the fixed state rules in the machine contract; never turn N successful examples into a universal proof. Report explicit proof limitations, counterexamples and unknowns.

## Prevent common-mode failure
Two independent implementation paths where meaningful: actual compiler annotation parser vs minimal independent source span/AST checker; code under test vs reviewed reference model; generated test suggestions vs independently authored property/oracle; explanation generator vs independent claim-binding validator. The verifier itself has versioned release tests, seeded wrong-answer fixtures, mutation tests and adversarial contract fuzzing. A small proof checker should be simpler than the generator, reviewable and unable to execute arbitrary claims.

## Proof-carrying compiler output
SWC produces `SemanticWikiIR + ExplanationClaims + declared source witnesses`. The verifier consumes it, rederives source facts independently and attaches **separate proof receipts** rather than inserting a magic boolean into SWC IR. The wiki/HVI may join by typed claim/source ID but remain projections. No current SWC v0.1 production source change is authorized by this design.

## Claim coverage
Each verification run records `eligibleClaims`, `checkedClaims`, `refutedClaims`, `supportedClaims`, `provedClaims`, `unknownClaims`, `blockedClaims`, plus all missing observed effect boundaries. An absence of a claim is not evidence that a code path is safe. Verification plans can be complete only for explicitly enumerated source scope and claim set.
