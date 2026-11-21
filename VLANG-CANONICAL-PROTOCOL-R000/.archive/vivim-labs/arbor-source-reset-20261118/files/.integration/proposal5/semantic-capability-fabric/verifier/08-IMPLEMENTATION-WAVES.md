# SV development waves — safely staged behind HVI and SCF
**Status:** future execution design, not launched; source baseline `ARBOR main f9823a5131e73e7562db5a4ec14e1ec481557d69` when authored. Re-pin before any work. Current HVI parallel workers and G integration retain exclusive ownership.

## V0 — Source truth and verifier TCB (HQ D + independent F)
Pin HQ `protocol.mjs`, `annotation-v2.mjs`, `semantic-registry.mjs`, `indexer.mjs`, `wiki.mjs`, `arbor-service.mjs`, SWC research `semantic-wiki-compiler.mjs`, HVI contracts/actual accepted source. Inventory what existing tests and receipt ports *actually* prove. Review the small trusted checker, accepted oracle roots and schemas. Gate: design/theory explicit; no premature runtime GO.

## V1 — ClaimSpec and additive v3 parser (D + F)
Implement strict approved claim/predicate/profile records and additive `@hq:verify`, `@hq:observe`, `@hq:explain` parser (owner review, backward compatibility v1/v2). Bind source-symbol/file scopes to exact Git/tree pins, no executing untrusted comments. Gate: type/ref errors, malformed lexical cases, proof receipt compatibility negative tests.

## V2 — Independent compiler/claim checker (F separate from D)
Implement independent source span/AST rederivation; checker of typed SC-ABI source claims, registry resolution, trusted spec digests and explanation template binding. Seed false claim fixtures and cause checker to refute. Gate: demonstrably detects compiler's mistakes and prevents proof laundering; no need for runtime.

## V3 — Pure function witness runner (A existing Rust compute + F Node harness)
Run data-only deterministic property/mutation/metamorphic/differential suites in bounded isolation against exact source/build manifests, test oracle approved separately. Optional bounded model checker with small proof checker; output claim-specific receipts and counterexamples. Gate: runnable Node/Cargo tests with exact exit/runner, fail closed on incomplete artifacts.

## V4 — Domain-owned runtime observation (B Host, C Studio, E UI, F)
Introduce **read-only verifier adapters** to already-owned Host journal/grant receipts, Studio send observation and TierFlow revisions. Verify actual build identity, actor scope, trace completeness, external uncertainty. No new executor or unsafe replay; no PWA privileged path until original security gates accepted. Gate: real allow/deny/revoke/source drift and boundary tests; physical device acceptance separately.

## V5 — Verified Wiki explanation & HVI proof delta (D + HVI B/C/E + G)
Produce reviewed template-bound prose and proof-carried Capability Passport, connect to existing Wiki writer and HVI immutable N-way comparisons, with coverage and counterexample navigation. Gate: 3 commits same repo + 5 repo migration report; no false equivalence or `always` from sampled cases; no new operational store.

## V6 — SDK + agent workflow (F/D/G, optional Studio)
Agent on-edit/warnings, CI verifier receipts and safe counterexample repair; backed by exact git/build/model evidence. Use staged touched-file enforcement first, no mass legacy rewrite. Gate: mutation and adversarial regression suites, performance/low disk/redaction, no bypass. SDK should remain optional until accepted in ARBOR.

## Branching / integration protocol
Any implementation requires new exact work orders from **current accepted G main SHA**, isolated D/F/A/B/E write scopes, independent referee, then serial G merge through PR with real test exits and source closure. The SCF draft PR may be updated by this design, but **not merged as product during active HVI without reconciliation**. Do not modify existing beta release gates.
