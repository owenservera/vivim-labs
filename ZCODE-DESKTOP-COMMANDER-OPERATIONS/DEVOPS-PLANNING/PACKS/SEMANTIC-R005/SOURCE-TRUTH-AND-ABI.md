# Real source contract and proposed Semantic Verifier proof ABI — R005

Source pinned 2026-11-21: [ARBOR `7ae24ff`](https://github.com/owenservera/ARBOR/tree/7ae24ff78a797581b510fe7a78105bbe301c0de4). **This file reports inspected source, not execution.**

## Existing source signals, classified truthfully

| Producer and path | Existing content / symbol | Trust class | R005 behavior |
|---|---|---|---|
| `apps/vivim-host/chat/wiki-semantics.mjs` | `pageId = 'wiki:page:' + encodeURIComponent(name)`; `wikiClaimEvidence` classifies declared, observed, inferred, committed events, applied receipts | Host Wiki source occurrence or historical Host effect *only* | Do not equate `assurance:'verified'` for a committed-structure occurrence with a proven application capability. Exact IDs only; no guessed slug |
| `native/hqkg/src/semantic_source.rs` | `sources(graph, semantic_id)` returns source anchors and native graph epoch; annotation occurrence never proves behavior | Indexed static source evidence | Map to `StatementKind::SourceOccurrence` -> `Verification::Unknown` for behavioral English |
| `native/hqkg/src/development_engine.rs` | `evidence:'indexed-static-only'` and `graphEpoch` in development projection | Static code/graph evidence, not runtime proof | Do not promote to verified runtime claim |
| `core/programmability/src/semantic_observation.rs` | Passive aggregation of already-validated owner-feedback events and proposed rules | Proposed observations, no activation authority | Do not treat as SV or Host effect execution |
| `core/programmability/src/context_grounding.rs` | Exact source pinned reference resolver, not provenance oracle/effect grant | Source-reference candidate | Reuse for command references only; no Wiki trust elevation |
| `core/programmability/src/live_system.rs` | Source + view revision/epoch and read-only region changes | Current state lens | Use existing `Lens` source fences; no second runtime state store |
| Existing R003 `narrative::realize` | Bounded deterministic safe grammatical templates requiring `Verification::Verified` and `current_at_source` | Presentation only | R005 maps real independent proof to these existing fields; **does not reimplement grammar** |
| Existing R004 `readback_guard` | Request sequence + graph epoch + Wiki revision and SHA-256 digest | Source-currentness fence | R005 pipeline checks before presenting and keeps separate semantic source revision |

## Semantic Verifier status is DESIGN ONLY

Actual proposed files are under `.integration/proposal5/semantic-capability-fabric/verifier/`: `README.md`, `contracts/claim.v0.schema.json`, `contracts/proof-receipt.v0.schema.json`, and the feature-wide proof design.

The proposed `arbor.sv.claim/0.1` identifies immutable source pins, approved contract, claim ID, property ID, reviewed property definitions, oracle and assumptions. The proposed `arbor.sv.proof-receipt/0.1` includes:

- `receiptId`, `claimId`, `propertyId`, `sourcePin.repository` and 40-hex `commitSha`
- `specDigest`, `verifierDigest`, `checkerDigest`, `artifactDigest` (64-hex)
- `verdict`: `PROVED_UNDER_MODEL | REFUTED | SUPPORTED_BY_TESTS | UNKNOWN | BLOCKED`
- `methods`, `assumptions`, `runner`, `evidence`, `coverage.scopeComplete/unknownPaths`, `testExecution` and `counterexample`.

These are **proposed JSON shape contracts**, not proof that the independent runner, checker or authenticated ingestion exists. Do **not** insert a permissive stub that sets `independently_authenticated=true` based on parsed JSON, matching digests, a self-authored test or a compiler-generated comment.

## Exact R005 adapter policy

`semantic_evidence::admit_semantic_sentences` receives only Host-authorized `ProposedSentence`, a source-attested `CurrentEvidenceScope` and optional `&dyn IndependentProofReader`.

- **Current production default: `reader=None`.** Resulting claims cannot produce verified capability prose; they remain unknown. A truthful empty introduction renders.
- When a future independently authorized verifier exists, its owning adapter must validate complete proposed JSON schema, owner-approved immutable ClaimSpec, exact repo/tree/artifact/source pin, independent checker run, checker/verifier/runner identity, evidence and counterexample replay, assumption coverage, and actual explanation/model-scope fidelity **before** returning `IndependentlyCheckedProof`.
- `ModelBoundContract` can reach `Verified` only with `PROVED_UNDER_MODEL`, complete scope/no unknown paths, authenticated checker, exact source/evidence pins, no counterexample, and independently checked explanation + limitation. A boolean in an untrusted browser payload never qualifies.
- `SUPPORTED_BY_TESTS` => `Candidate` (may describe a test-supported sample only in a separately reviewed view). `REFUTED` plus independently checked counterexample => `Disputed`. `UNKNOWN/BLOCKED` => `Unknown`.
- `RuntimeEffect`, `SourceOccurrence` and `DeclaredFact` do not become verified from SV metadata. Real runtime effect claims will require a distinct Host receipt owner path; this pack deliberately does not build it.
- Source-revision domains are **not equal**: `ProposedSentence.semantic_source_revision` is checked against the independently attested native semantic graph/source revision. Only then does the accepted sentence bind to **current Host Wiki projection revision** via `SentenceClaim.source_revision`. Coincidentally equal revision numbers confer no trust.

## Unresolved owner-specific seams that Dev must not invent

1. A real independent SV proof verifier/checker with authenticated receipt writer and reviewer authorization is absent in inspected ARBOR source.
2. Current exact semantic-source revision/readbacks and the Host Wiki revision/digest must be fetched from the respective source owners, not from a submitted UI read request.
3. The current authorized Host read port for HQKG contextual Wiki data must be approved; existing `development-read-port.mjs` is desktop owner-only, not a public phone source.
4. Host permission revocation may happen **without changing graph epoch**. Actor grant revision must be independently rechecked before and after any async read.
5. The R004 exact exclusions builder still needs the actual graph-scoped source owner; without an attested complete set, read access must refuse rather than fall back to a broad prefix.
