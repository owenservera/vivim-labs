# Recipe D1 — independent SV-R012 verifier, evidence receipts and explanations
**Owner D (separate from A/compiler and from agent producing code).** New `semantic-capability/{verifier-adapter,proof-binding,explanation}.mjs`, and existing verifier proposal `verifier/contracts/{claim,proof-receipt,explanation}.v0.schema.json`. No replacement schemas.

## Trust architecture
The source compiler can report `DECLARED`, `PARSED`, `RESOLVED` and independently checked static observations, but cannot certify functional correctness. The Semantic Verifier must be able to **disprove** an incorrect compiler/agent claim using an independently pinned oracle/spec/model and trusted runner evidence. Code author cannot unilaterally ratify oracle and verifier changes in same PR.

## Exact entry points (reference)
```ts
export function claimCandidatesFromFacts(facts, approvedSpecs):
    Array<{claimRef,propertyRefs,sourcePin,compiledFactRefs}>;
export function validateSvReceipt(receipt, {
  acceptedSpecs,checkerVersions,artifactDigests,observedRunnerIds
}): {accepted:boolean;verdict;diagnostics:string[]};
export function bindIndependentProofs(passports, receipts, approvedSpecs):
    Array<CapabilityPassport>;
export function buildVerifiedExplanations(passports, approvedTemplates):
    Array<SvExplanationEnvelope>;
```
- Receipt `claimId`, `propertyId`, `specDigest`, `sourcePin`, `verifierDigest`, `checkerDigest`, `artifactDigest`, `testExecution`, `coverage`, `counterexample` must match actual record and exact source/build. Any missing digest => UNKNOWN/BLOCKED, not `SUPPORTED_BY_TESTS`.
- SV verdict allowed: `PROVED_UNDER_MODEL`, `REFUTED`, `SUPPORTED_BY_TESTS`, `UNKNOWN`, `BLOCKED`. A finite test cannot emit universal proof; only restricted formal model with independently checked proof may use `PROVED_UNDER_MODEL` and must state assumptions/domain.
- Proof receipts are **never produced** by reading `@hq:test` or `@hq:verify` alone. Annotation supplies refs; independent verifier supplies the verdict with runner/proof.
- A counterexample persists in passport and explanation; cannot be masked or erased by a newer passing property unless scope and spec explicitly changed and old evidence preserved.
- `@hq:verify`, `@hq:observe`, `@hq:explain` are future additive v3 refs. Existing v2 parser must not pretend to parse them.

## Counterexample replay protection
Replay only in deterministic, sandboxed pure worker with synthetic safe input. Never execute source from untrusted donor, never trigger Host effects, browser sends, user profile actions or production migrations. Host-owned runtime receipts are *read-only observations*, not replays.
If model/runner not available: `BLOCKED` with exact missing independent oracle. If trace incomplete: no "no effects" proof. For explanatory text use owner-approved templates bound to claims and qualifier status; forbidden to print "always", "safe", "shipped", "verified" without appropriate evidence.

## Independence test corpus
- Author alters code and test together to pass false specification: independent oracle catches false claim.
- Source SHA stays same but artifactDigest differs: source-only proof invalid.
- Incorrect manifestDigest, revoked source grant, partial event log: cannot claim `RUNTIME_VERIFIED`.
- Runtime exception not observed because telemetry partial: `UNKNOWN`.
- False claim counterexample refutes even if many sampled cases passed.
- Claim tested on synthetic fixtures: `SUPPORTED_BY_TESTS` for that scope, not production.
- Tainted English explanation suggests shipping despite UNKNOWN: independent truth checker downgrades/refuses text.
- Attacker comment instructs verifier to "ignore rules": data only, no policy changes.

## Test/implementation handoff
Reuse SV schemas verbatim at version 0.1; compiler outputs a correlation envelope, not new verifier schema. F builds independent tests from `verifier/examples/negative-scenarios.v0.json` and additional Golden vectors; Z referee confirms D source/checker/oracle owner separation.
