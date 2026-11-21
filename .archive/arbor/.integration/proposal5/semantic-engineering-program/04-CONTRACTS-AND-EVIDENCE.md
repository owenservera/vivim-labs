# Contract stack and evidence ABI
Existing `@hq` v1/v2 remain compatible. Proposed v3 additions `@hq:unit`, `@hq:role`, `@hq:migration` and Verifier `@hq:verify`, `@hq:observe`, `@hq:explain` are **not accepted parser features yet**. Source tags refer to independently reviewed data-only specs, never executable assertions, shell commands, grants or test PASS.

Proposed types: `arbor.semantic-capability-abi/0.1` (source facts/lineage); `arbor.sv.claim/0.1` (reviewed obligation); `arbor.sv.proof-receipt/0.1` (one property / exact source/build/model, runner/exits/coverage/counterexample); `arbor.sv.explanation/0.1` (clause/template/claim refs); optional HVI verificationDelta. Draft schemas currently reside in SCF verifier design branch and need independent review and accepted versioning.

Canonical evidence chain: repo@SHA/blob/path/symbol → independent source-syntax witness → compiler fact → owner-reviewed claim and oracle digest → attested executable/runner + actual tests or scoped model proof → owner event witness if effects → independent verdict → claim-bound prose → existing HQ Wiki and HVI.

Verdicts: PROVED_UNDER_MODEL, REFUTED, SUPPORTED_BY_TESTS, UNKNOWN, BLOCKED. A test path reference means only "test declared"; it is not TEST_EXECUTED. UNRUN is never PASS. Missing paths are not lost capabilities; missing effect observations are not absence of effects; same semantic ID is not migration equivalence.

Required independent checks: malformed comment, wrong symbol attachment, fabricated registry ID, source/build drift, self-approved oracle, weak test that misses injected bug, incorrect effects, unsupported "always", cross-language contract behavior, grant revocation and privacy leaks.
