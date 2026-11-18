# Proposal 5 / SCF-R012 — Semantic Verifier (SV): proof-carrying meaning
**Status:** DESIGN_AND_CONTRACT_PREPARED / NOT_IMPLEMENTED / TESTS_UNRUN
**Date:** 2026-11-12
**Parent:** `.integration/proposal5/semantic-capability-fabric/` on `work/p5-semantic-capability-fabric-r001`.
**No automatic promotion to main.** HVI and SCF source owners and current branch pins remain protected.

## The thesis
The Semantic Wiki Compiler (SWC) creates descriptive semantic claims from source. The independent **Semantic Verifier** is a bounded falsification/proof engine checking each claim against immutable source/build identities, reviewed specifications, independent static observations, constrained execution evidence, trace coverage and independently checked explanations. The verifier must not trust the compiler's own interpretation or code-generated tests as its sole oracle. The output is a **Proof-Carrying Capability Passport**: every sentence about a function/feature has an accountable verdict and replayable evidence.

The answer to "does the code do what the sentence says?" is **claim-specific and scoped**, never a universal binary answer for arbitrary programs. `PROVED_UNDER_MODEL`, `REFUTED`, `SUPPORTED_BY_TESTS`, `UNKNOWN` and `BLOCKED` are the core truthful outcomes. A counterexample disproves a universal claim; passing a finite test suite does not prove one.

## Deliverables
- [00 — Problem, trust and goals](00-VERIFIER-THESIS.md)
- [01 — Comment language and per-function protocol](01-CLAIM-LANGUAGE-AND-TAGS.md)
- [02 — Typed Claim/Proof ABI](02-SPEC-AND-PROOF-ABI.md)
- [03 — Independent verification kernel](03-INDEPENDENT-VERIFICATION-KERNEL.md)
- [04 — Compiled artifact and runtime observability](04-EXECUTION-AND-OBSERVABILITY.md)
- [05 — Verbal explanation fidelity](05-EXPLANATION-TRUTH-CHECKING.md)
- [06 — Version Intelligence and feature proof](06-FEATURE-AND-NWAY-VERIFICATION.md)
- [07 — Agent/SDK contributor lifecycle](07-AGENT-SDK-PROTOCOL.md)
- [08 — Staged development and owners](08-IMPLEMENTATION-WAVES.md)
- [09 — Negative and adversarial acceptance](09-REDTEAM-ACCEPTANCE.md)
- [Schema: claim](contracts/claim.v0.schema.json), [schema: proof receipt](contracts/proof-receipt.v0.schema.json), [schema: explanation](contracts/explanation.v0.schema.json)
- [Example typed claim](examples/pinned-snapshot.claim.v0.json), [falsification fixture](examples/negative-scenarios.v0.json)
- [Machine-readable manifest](verifier-manifest.json)

## Important integration decisions
- **One existing HQ semantic registry and wiki/indexer.** Do not create a second canonical KB, graph, wiki writer, parser-to-action router or effect executor.
- The verifier is a *small independent proof checker* plus disposable untrusted test/analysis workers, not another all-knowing LLM. It consumes SWC facts but derives proof from independent inputs.
- Source comments may **point to** reviewed spec IDs; they cannot define arbitrary executable assertions, grant effect permission, install a Host action, fake tests or approve their own claims.
- Existing `hq.annotation/2` keeps working. Proposed `@hq:verify`, `@hq:observe`, `@hq:explain` are *future additive v3 references*, not currently installed tags.
- Host owns runtime authority, journal, claims and receipts; Studio owns provider browser sending; TierFlow owns UI publisher; native HQKG pure compute remains one owner. Runtime verification obtains **read-only witnessed evidence**, not a duplicate executor.
- Pure deterministic functions may obtain bounded proof under a specified model; nondeterministic/external actions require correctly scoped observations and may remain `UNKNOWN`.
