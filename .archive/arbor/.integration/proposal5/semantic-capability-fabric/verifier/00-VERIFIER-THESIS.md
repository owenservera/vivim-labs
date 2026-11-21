# SV — goals, epistemology and trust boundaries
## Three fundamentally different questions
1. **Compilation fidelity:** Did the compiler *read and translate* the source annotation and code structure correctly? Validate typed AST/source spans, registry references and compile transformations, independently of emitted prose.
2. **Behavior fidelity:** Does a particular built artifact, under an explicit environment and assumptions, satisfy a typed precondition/postcondition/invariant/effect specification? Verify through allowed methods: bounded static proofs, isolated property/model tests, differential and fault-injection tests, and witnessed runtime traces.
3. **Explanation fidelity:** Does each sentence in a generated "this code does X" explanation follow from **verified structured claims at the same source/build/world revision**? Constrain certified prose to reviewed templates and approved claim edges. Unsupported sentences become `UNKNOWN` or are withheld.

A fourth, historical question: **version fidelity** — did a property remain true from source/build pin A to B to C, with actual compatibility, tests and runtime witnesses? HVI compares receipts per pin instead of guessing equivalence from matching comment IDs.

## Core rule
**Declarations are hypotheses. Verifier receipts are scoped evidence.** An agent controlling source code and inline comments is not a trusted independent witness. A generated test controlled by that same agent is not independent proof. A fabricated or replayed CI log is not a receipt. One claimed runtime test without exact artifact/build and environment binding cannot attest the deployed executable.

## Verdict semantics
- `PROVED_UNDER_MODEL`: small trusted checker accepts formal proof or exhaustive bounded-state result for a precisely quantified claim and explicit model assumptions. Does not imply arbitrary runtime/real world correctness.
- `REFUTED`: checked type contradiction or witnessed counterexample violating an exact claim under its scoped assumptions. Include replayable minimal counterexample and mismatch.
- `SUPPORTED_BY_TESTS`: independently specified tests/property suites executed successfully against an attested artifact under declared environments, no exhaustive proof claimed.
- `UNKNOWN`: no trustworthy sufficiency or completeness; could be true or false. Typical for uninstrumented output, missing external acknowledgement or partial source.
- `BLOCKED`: mandatory scope, permission, toolchain, schema or trust prerequisite not met; execution/checking must not proceed.
Additional metadata: `coverageComplete`, `assumptions`, `verificationMethods`, `testEvidence`, `counterexample`, `risk`, `expiry/revision`. Do not flatten all facets to `verified:true`.

## Threat model
Adversary or fallible agent may forge tags/test paths, rename functions, duplicate IDs, omit branches, falsify prose, mutate the oracle, capture a valid test against different bytes, omit effect instrumentation, replace running artifact, reuse stale world grants, inject instructions in comments, and alter test output. Cryptographic digests protect identity **only when trustworthy producers/receipts are pinned**; signatures do not establish semantic truth. No system can in general prove equivalence between arbitrary unrestricted programs and prose, or prove absence of unobserved effects from an incomplete trace.

## Trusted computing base (minimum)
- An *independently reviewed small proof-checking library*: strict schema decoder, canonical hash, pin/identity verifier, typed predicate interpreter, receipt/assumption checker, allowed evidence source and signature/attestation validator where available, fail-closed verdict aggregator.
- Owner-approved spec registry and trusted CI/Host attestation roots **outside the coding agent's unilateral editing scope**.
- Trusted source read boundary and the actual runtime/Host event origin, with complete instrumentation only for a stated monitored effect subset.
Everything else—compiler, code under test, generated tests, LLM explanation, Git comments, design docs, telemetry claims—is untrusted evidence until independently checked.
