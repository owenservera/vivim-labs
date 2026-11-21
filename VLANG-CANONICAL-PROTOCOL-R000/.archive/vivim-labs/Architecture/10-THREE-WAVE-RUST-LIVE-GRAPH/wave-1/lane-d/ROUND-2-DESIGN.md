# Wave 1 / Lane D — independently designed high-intensity Round 2

**Document:** ROUND-2-DESIGN.md  
**Date:** 2026-11-07  
**Status:** PROPOSAL ONLY; Round 2 NOT EXECUTED; Wave 2 NOT DESIGNED  
**Owner:** Lane D (intelligent wiki / derived integration-quality oracle)  
**Baseline safety:** Master Integration may be running. This publication changes documentation ONLY inside this lane's owned handoff folder. It does not modify the currently executing integration, HQ wiki builder, A/B/C/E/F producers, Rust importer, SQLite database, any browser profile, or a deployment.

## 1. Independent verdict and why another implementation round is required

**Round 1 result:** Lane D published a read-only HQKGX1 + generated-Markdown evaluator (wiki-oracle.mjs), 19 passing **synthetic** Node tests, a machine report, and a BLOCKED handoff. Its tests prove local diagnostic behavior, not a running integrated wiki. No real A+B+C feed, native Rust journal, HQ wiki regeneration or F-certified cross-domain result was demonstrated in the Round 1 evidence. Lane E's published RUN-STATUS.json independently says its real import/query/restart/compiled-comment gates were not run. Lane F's latest LANDED-CONTRACT-REVIEW.md independently reports a BLOCKED_CONTRACT_CROSSWALK between A/B/C producers and E's input envelope. A fresh master integration result was **not identified in the GitHub sources inspected for this proposal**; do not infer its success, failure or absence from that.

**Architectural correction:** The useful product is *not a better-looking wiki*, but a deterministically generated, source-linked explanation of **what the system is, means, can do, cannot yet do, depends on, writes, and what evidence backs those statements**. The wiki is a read-only projection and stringent acceptance instrument over the source-aware semantic compiler + SQLite-authoritative knowledge/registries + Rust graph + registered operation inventory. It must make ambiguous, stale, unverified, private and unconnected data visible as unknowns/refusals, not fictional completeness. Existing HQ hq/lib/wiki.mjs remains the **single wiki renderer/writer**; its owner must approve and implement any interface extension.

## 2. Full intended outcome recovered from document history (NOT status claims)

Controlling document hierarchy:
1. Current owner Wave 1 order: EXECUTE-WAVE-1-FIRST.md, MASTER-DOCUMENT-LED-WAVE-GOALS.md and COMPILER-LANGUAGE-AND-PLACEMENT-REVIEW.md. These require real A+B+C+E → generated HQ wiki traversal, integrity and quality diagnostics.
2. Recovered semantic-language architecture and contracts: documentation-consolidation/repos-hq/docs/research/semantic-language-kernel/{ARCHITECTURE,CONTRACT-AND-ROADMAP,SOURCE-MAP}.md. They require **separate** source-comment and human-NL grammars joined by accepted semantic IDs; source anchors, versioned config/preview/revisions/rollback, common typed intent from text/selector, and no permission from comments. Their historical "proposed" labels must be re-evaluated against newer source, not repeated as current truth.
3. Recovered 2026-11-04 intent audit: documentation-consolidation/repos-hq/docs/audit-2026-11-04/01-intent-front-end.md. It documents then-observed negation/clause-loss and source-wired-but-unverified actions; re-test, do not assume defects still reproduce after upstream changes.
4. Recovered GUI02/VOMEGA direction: documentation-consolidation/vivim-web-harness/docs/research/GUI02-PRE-PRD-CANVAS-SEMANTIC-LANGUAGE-2026-10-31.md: grounded token spans, meaning-preserving visual variants, ambiguity, risk, effect preview, owner confirmation and revision identity. **Wave 3 interface obligations only**, not authority for a new Wave 1 renderer or action executor.
5. Accepted TierFlow 2026-11-03 implementation assessment supersedes the older greenfield Rust-UI-state-writer proposal: preserve existing Bun/SQLite UI document write authority. Rust HQKG is a different read/compute system.
6. The documentation consolidation INVENTORY.json enumerates **1,234** documentation-format files (137 moved/1,097 retained). This is an architectural requirements discovery map; neither archived prose nor all 1,234 files are pre-authorized KB facts or runtime inputs.

The complete Lane D **Wave 1 end-state** is:
- **Navigable meaning:** per selected project/root/code symbol/capability/operation, show canonical logical identity, source revision/line, explicit purpose and I/O, accepted/proposed/deprecated semantic meaning, decision/KB evidence with trust status, declared vs source-wired vs runtime-verified command, precise syntax, actor/risk/scopes, dependencies and data owner; unknowns and limitations prominently surfaced.
- **A compiler-backed lineage:** code source bytes → @hq v1/v2 declaration or Rust @hq:wiki program (different grammars) → independently verified semantic/KB IDs → registered handler/CLI (non-granting) → E Rust node/edge, including intermediate compiled-program nodes → one HQ-owned wiki page/section. Every join must have typed relation, source references and evidence-class state.
- **Single derived writer:** the existing HQ wiki builder produces Markdown, search index and self-contained HTML; an approved *read-only integrated graph view* may be supplied to that builder by its owner. Neither Lane D nor Rust may create an alternative canonical wiki content feed, mutate semantic SQLite or promote a command. The existing wiki-program writer keeps exclusive versioned/owner-confirmed config activation and rollback.
- **A falsifiable oracle:** compare generated wiki to **actual scoped source+semantic+operation+Rust data**, not to itself; detect missing, misleading, unwired, stale, contradictory, privacy-leaking, broken-link, absent-root and unknown-evidence cases. Provide per-root/per-domain denominators, identity/evidence vectors, stable machine-diffable defect IDs and a human gap report.
- **Change safety:** unchanged import/wiki yields stable content and no-op; source edits, deleted/renamed declarations, KB revision changes and handler status changes propagate or are explicitly marked stale; restart and failed imports do not leave deceptive pages. E owns graph journal; HQ owns wiki output; F independently verifies.
- **Future visual intent compatibility:** preserve (without implementing UI) commandSemanticId, source spans, selected entity, typed slots/missing slots, negation/quotes/conjunction AST, alternatives, risk/scopes, input/world revisions, uncertainty, preview and provenance. Interpretations remain inert until the Host/native owner authorizes effects.
- **DEV/PRODUCT separation:** broad source observations and raw diagnostics restricted to owner-approved DEV; product wiki is only privacy-filtered, user-scoped read projection. Neither DEV fixtures nor graph nodes grant Host actions.

## 3. Round 1 truth audit — implemented, tested, wired, simulated, unsupported, unverified

| Claim/surface | Inspected status | Evidence | Actual limitation |
|---|---|---|---|
| HQKGX1 parser and Markdown inspector | **Implemented; synthetically tested** | lane-d/wiki-oracle.mjs; 19/19 Node synthetic tests | Not exercised against E's real published assembler output plus actual HQ-generated pages |
| Deterministic synthetic page + defect score | **Implemented; tested only on fixtures** | lane-d/wiki-oracle.test.mjs, TEST-EVIDENCE.json | This is a fixture consistency score; it is not full-source coverage or native integration |
| Rust journal execution and query truth | **Unverified; documented blocked** | E RUN-STATUS.json | No actual DEV .hqj, Rust query, Cargo receipt or restart proof in this evidence |
| A/B/C native integrated export | **Source implemented, consumer unwired** | Lane A/B/C HANDOFFs and F LANDED-CONTRACT-REVIEW.md | Published A/B/C schemas are not E's required schema; no observed lowering converter |
| Rust comment compilation → B/C registered action | **Implemented in E's assembly logic, not end-to-end executed** | E assemble.mjs lines 96–116; E handoff | E emits intermediate compiled-wiki-program node and declared edges; no real native pinned compiler chain |
| HQ Markdown/HTML wiki generation | **Existing source implementation; integration unwired** | repos-hq/hq/lib/wiki.mjs buildWiki; hq/bin/hq.mjs cmdWiki | Existing loadModel reads HQ index and capability/session models; no proven read-only E graph input exposing B/C semantics in these pages |
| Registered action / CLI status | **Source-inventoried, runtime unverified** | Lane C handoff | C reports 2,000 descriptors, 410 handler-map mapped, 80 mapped with "(no CLI)", zero device/runtime proof; "wired" is not executed |
| Knowledge authority and source links | **Pure read-adapter implemented, fixture tested** | Lane B handoff | Real local owner KB and source SHA-matched records are absent in evidence; one accepted KB sidecar cannot be inferred from a semantic label |
| Wiki-to-graph correctness | **Unsupported as a real acceptance claim** | D validator needs a page containing 5 text terms | Co-occurrence in arbitrary text is not an actual typed graph→page join; missing KB participation and page owner receipt |
| Markdown links | **Partially tested** | D wikiLinkGaps | Regex-only .md href existence; no anchors, search.json/HTML integrity, source identity, symlink redirects or fully validated navigation |
| Native determinism, freshness, rebuild/diff | **Unverified** | E and D handoffs | E importer is additive; deletion/renames may preserve stale graph facts, and existing wiki pages can lag |
| Privacy/authority/no effects | **Partially defended; not independently verified end-to-end** | D safeMeta/SECRET; E source-only contracts | String checks are not evidence of consent, scope, link truth, test execution or product-safe projection |
| Wiki-program activation parity with text | **Existing owner code; NOT D integration** | hq/lib/wiki-program.mjs, comment-wiki-bridge.mjs | No owner-confirmed versioned program activation or current NL/widget parity tested here; D must remain observational |

**Critical fact from source comparison:** Lane E assemble.mjs maps node source provenance to source_root/source_path/source_commit/content_sha256 and edge evidence to evidence_class/evidence, whereas D sourceSafe() expects source_repo/source_path/source_sha and hasEvidence() expects link_status/evidence_ref. E also creates project, source-root and compiled-wiki-program node kinds, which D classify() reports as unknown. D's current traversal hardcodes code → semantic → command; E's Rust comment route is code → compiled-wiki-program → semantic → command and its created relations are DECLARED, not verified. These are **contract/meaning incompatibilities**, not mere naming aesthetics. Do not fix by blindly renaming metadata or treating declared as verified.

**Further hidden shortcuts:**
- A synthetic baseline wiki has a semantic ID and command on one page but a *separate orphan KB node*; D still counts one "verifiedWikiChain". Presence of B/knowledge is not identity-anchored proof.
- The current sample quality can be **80/100 while verdict BLOCKED**. Domain-weighted average obscures root-level missingness, false joins, invalid evidence and denominator-zero cases. It must never be named "integration completeness."
- Matching literal SHA strings/handler names in a page does not prove the underlying file bytes, canonical action identity or a valid wiki link target.
- Source-commit SHA-1 and content-byte SHA-256 are different provenance primitives; the field "source_sha" is ambiguous. Accept no automatic equivalence.
- A real HQ buildWiki() output from unchanged HQ index can be stable yet entirely omit B/C links. Determinism alone is not a semantic acceptance test.
- Raw graph exchange digest proves raw exchange bytes, **not** that Rust applied it, journaled it, reloaded it or that F independently saw the same graph.
- The oracle reads Markdown files in a directory that the caller names; it does not attest that this directory was freshly generated by the approved wiki writer for the same graph revision.
- Existing Markdown regex link search skips missing anchors and image/search/HTML consistency, and a bad or unsafely constructed wiki page may contain a command-looking string without a validated inert handler descriptor.
- The dev owner may have later unpublished changes; this proposal is pinned to checked-in evidence and must be revalidated against fresh main before implementation.

## 4. Cross-lane dependencies: what D must NOT assume

| Source/owner | Requested proof and minimal D-facing read contract | Refuse/incomplete behavior |
|---|---|---|
| **A** code index | Exact project/intent/root revision, every eligible/excluded root denominator, file+symbol path/line, Git head and byte SHA-256, v1/v2/comment-program grammar status, static vs observed relationships, reset/archive receipt | No full-coverage claim for sample roots, unscoped paths or old-run contamination |
| **B** KB/semantic SQLite | Current/proposed/deprecated identity, source+content hash links, authoritative status/decision, revisions, aliases/collisions, privacy-filtered descriptions, explicit unknowns | Candidate aliases or archived docs cannot become canonical truth; private rows omitted with count |
| **C** action/CLI/NL | Source-pinned command ID and syntax, actual handler source, actor/risk/approval, registry vs handler vs runtime vs device evidence states, refusal corpus including negation, quotes and conjunctions | No command marked runtime-wired/executable merely from a handler string; no NL candidate execution |
| **E** sole Rust bridge/journal | Validated HQKGX1 manifest/graph digest, original source ID crosswalk, explicit relation kinds, graph revision, Rust import/no-op/reload/query receipts and compiled Rust source-map chain | No treating "declared" compiler edges as independently verified; no graph journal writes by D |
| **F** independent verifier | Fresh source SHAs, native command exit codes, observed graph query equality, source-root denominators, code/KB/action/wiki gap ledger, Rust-vs-SQLite measurements, privacy/regression results | D must label own score unverified until F independently crosschecks; F's provisional JSON graph parser is not E HQKGX1 |
| **HQ wiki owner** | One approved read-only graph/IR view input to existing buildWiki(), coherent page registry and machine page manifest; output generated at pinned source/scope/journal rev | Do not add a second wiki renderer, silently patch wiki-program config or activate a new writer |

**Contract strategy:** A/B/C stay canonical and independent. E/integration owner may publish an explicit source-owned, evidence-preserving lowering/read adapter. D consumes **validated E output** via a **D-owned read-only view adapter** with strict schema-version support; it does not change E transport, rewrite HQ authors or treat wiki text as facts. A shared, tiny contract note can be proposed upstream; master decides timing/ownership.

## 5. Round 2 numbered high-intensity execution objectives

The detailed dependency, test and coverage grid is in ROUND-2-BACKLOG.md. Each D-owned code path below means **future** implementation and is NOT authorized by this design publication.

### D2-01 [P0; SAFE] Freeze real contract matrix and fail-closed compatibility
**Targets:** lane-d/contracts/actual-contract-matrix.json; contracts/validate-inputs.mjs; test/contracts.test.mjs.  
**Depend:** fresh A/B/C/E/F SHAs, E HQKGX1 source schema and F preflight.  
**Work:** Make a machine-diffable field/type/authority map from A/B/C public envelopes → E expected feed → actual E exchange → D read inputs → HQ writer, with no guessed key coercion. Handle unknown schema revisions and missing integration receipts.  
**Tests:** exact E sample representation, unknown/bad schema, 40-vs-64-hex, missing metadata, missing parent/project/root and duplicate IDs; each refusal preserves original evidence.  
**Pass:** 100% of observed published contract fields classified consumed/preserved/unknown/refused; zero silent coercions; schema mismatch returns BLOCKED and names accountable producer/consumer.  
**Proof:** manifest of Git blobs + isolated test transcript; no claim that actual native feed has been executed.

### D2-02 [P0; SAFE] Replace substring-based identity checks with typed lineage
**Targets:** lane-d/lineage/validate.mjs; lineage/traverse.mjs; test/lineage.test.mjs; leave current oracle in place during master baseline.  
**Depend:** documented E relations COMPILES_WIKI_PROGRAM, DECLARES_INTENT_FOR, HAS_REGISTERED_HANDLER; A/B/C source maps.  
**Work:** Walk named directed relations with bounded depth and explicit state vector. Prove code→compiled declaration→semantic ID→registered C operation AND a trusted KB/decision link when one is claimed. Distinguish supported plain code→semantic→command edges from compiler intermediates. Test swapped semantic IDs, shared names, missing KB, duplicates and cycles; never fabricate a link from text similarity.  
**Pass:** every counted chain has source ID/bytes, matched B ID and C ID, typed edge evidence, and an explicit wiki page binding; declared/claimed-only chains **cannot** become VERIFIED.  
**Proof:** corpus with known-positive and seeded false positives; false verified acceptance = 0.

### D2-03 [P0; SAFE] Source provenance, authority and evidence vectors
**Targets:** lane-d/lineage/provenance.mjs; lineage/states.mjs; test/provenance.test.mjs.  
**Depend:** A root manifest and hashes, B accepted/proposed/revision, C dispatch status, E query digest.  
**Work:** Separate Git head SHA-1 / exact content SHA-256 / source path+line / semantic identity / owner authority / compiler claims / handler-source / runtime-tested / observed-device flags. Validate evidence types, pinned compiler version, source maps, scoped root ancestry and revision compatibility; no trust upgrade based on producer strings.  
**Pass:** every verified claim cites proof type/source SHA/revision; 100% of status dimensions independent; forged receipt, stale SHA, untrusted alias, no test and conflicting status all downgrade to UNKNOWN/BLOCKED.  
**Proof:** negative trust corpus, emitted state-vector snapshots and no-effects tests.

### D2-04 [P0; SAFE] Build actual quality denominators and defect taxonomy
**Targets:** lane-d/quality/score.mjs; quality/defect-schema.json; test/score.test.mjs.  
**Depend:** A root expected/eligible/excluded counts; B/C counts; E graph; F proposed actual/target fields.  
**Work:** Report scoped expected and observed files, annotations, source-linked KB, accepted IDs, registered commands, runtime-verified handlers, graph chain candidates, wiki-bound chains, broken links, stale projections; preserve unobservable vs absent vs verified missing. Avoid arbitrary universal percentage and block PASS on any critical unsatisfied requirement.  
**Pass:** zero denominator = NOT_MEASURED; incomplete roots trigger INCOMPLETE_SCOPE; score cannot be high if KB disconnected, source root incomplete or no verified chain; numerators never exceed denominators.  
**Proof:** machine-diffable per-root denominator report, mutation/property tests, no masked critical blocker.

### D2-05 [P0; INTEGRATION-GATED] Make the *existing* wiki express combined graph truth
**Targets:** D owns lane-d/contracts/wiki-read-view.md, test/integrated-wiki-contract.test.mjs and acceptance fixtures. **Only the existing HQ wiki writer owner** may change repos-hq/hq/lib/wiki.mjs or its approved read adapter; E owns graph serialization.  
**Depend:** integration owner clears A/B/C→E converters and publishes native E output; explicit single-writer patch authorization; baseline lease.  
**Work:** Specify/prove read-only graph/IR view passed to existing buildWiki. Page/section content must show exact source, semantic description/authority, KB claims/unknowns, CLI action syntax and handler status, dependency relation class and evidence; distinct declared/verified labels; program data remains inert. One page must correctly render one real source→comment→B→C→Rust chain.  
**Pass:** no second renderer, registry or wiki source feed; existing HQ writer alone emits Markdown/search/HTML; page manifest binds graph digest and scope/source revision; 100% of required chain fields match typed graph data or are marked unknown.  
**Proof:** actual generated diff and E/F-checked lineage, owner-reviewed patch/receipt and native wiki run exit codes.

### D2-06 [P1; SAFE] Robust navigation, content/link safety and attribution
**Targets:** lane-d/quality/links.mjs; quality/privacy.mjs; test/links-security.test.mjs.  
**Depend:** approved HQ page manifest/schema; Rust/HQ IDs.  
**Work:** Audit Markdown links and anchors, duplicate slugs/casefold collisions, path normalization/percent-encoding, absolute traversal, unsafe markdown/HTML, missing search.json/index.html targets, source URL provenance and broken cross-root links. Bounded reads; symlinks/device nodes refused; no external network fetching.  
**Pass:** each generated link points to actual declared target/anchor within allowed root; no secret path/body or executable action leakage; 100% seeded adversarial fixture escapes produce a specific defect/refusal.  
**Proof:** deterministic security matrix and exact defect locations; separate product privacy gates.

### D2-07 [P1; SAFE until native replay] Regression corpus for meaning and contradiction
**Targets:** lane-d/fixtures/README.md; test/meaning-regression.test.mjs; docs optional diagnostic examples (not actual wiki truth).  
**Depend:** source-pinned real public examples (after privacy review), B/C independent status, compiler fixtures.  
**Work:** Cases: misleading @hq comments, shadowed IDs, alias collisions, proposed vs current/deprecated, stale docs, missing handler/CLI, runtime-negative NL/quote/compound route, private content omission, static import mislabelled observed IPC, old-run contamination.  
**Pass:** no fixture causes an unsafe proposed operation to be presented as executable; all known ambiguity/false certainty cases flagged; case coverage recorded by risk class and source origin.  
**Proof:** regression inventory with at least 30 independently specified adversarial and boundary cases; record expected/actual, no synthetic "real" claims.

### D2-08 [P1; INTEGRATION-GATED] Freshness, deletion, no-op, rebuild and restart
**Targets:** lane-d/quality/freshness.mjs; test/freshness-native.test.mjs; evidence/incremental-diff.json.  
**Depend:** real scoped A index, B revisions, E imported DEV journal and source record reconciliation, HQ wiki owner writer, F witness.  
**Work:** Pinned baseline → unchanged rebuild → one source change and reindex → deletion/rename → KB/handler status change → stale journal refusal → restore/restart → compare Rust query/wiki page/manifest. E's existing importer is additive, so **do not declare no-op correctness equivalent to deletion support**.  
**Pass:** unchanged input yields byte-identical meaningful pages and no unnecessary rewrites; changed entities have exact affected page IDs; deletions cannot leave apparently current orphan pages; partial graph/reindex explicitly BLOCKED until E solves stale retention.  
**Proof:** timestamps/SHAs/digests, native exit codes, before/after wiki page manifest and F source independent check.

### D2-09 [P1; INTEGRATION-GATED] CLI/NL/wiki action parity as read-only observations
**Targets:** lane-d/quality/operation-parity.mjs; test/operation-parity-native.test.mjs; report command-state matrix.  
**Depend:** C real source-inventoried ops and negation corpus; F safety baseline, E canonical ID linkage, HQ wiki owner.  
**Work:** Same stable operation ID on graph page, CLI discovery and inert wiki candidate; display syntax, actor/risk/permission owner, missing slots, no match, unsafe negatives and clauses. Do NOT invoke or authorize any action or build NL interpreter.  
**Pass:** zero claimed executable actions without independent runtime evidence; one verified same-ID CLI/wiki/graph route or a precise BLOCKED gap; negation/quotes/compound cases are shown as refused/ambiguous if not proved safe.  
**Proof:** source, CLI/intent result, page snippet and F negative corpus, all with source SHAs.

### D2-10 [P1; INTEGRATION-GATED] End-to-end cross-domain oracle + independent F gate
**Targets:** lane-d/bin/run-quality.mjs; quality/report.mjs; evidence/native-run/*; HANDOFF update after actual run.  
**Depend:** A safe reset, B/C real data, E Cargo/DEV .hqj where/what/talks-to/gap and import receipts, HQ wiki regenerated, F contract-compatible independent verifier.  
**Work:** Run source→compiled semantic→KB→registered C action→Rust→HQ wiki path; grade evidence and status, re-execute no-op/restart/error inputs and query comparison. Preserve raw failing outputs, no promotion.  
**Pass:** native acceptance never passes unless all required A/B/C/E identities and source bytes agree, a real KB link exists, one actual handler is registered (runtime state accurately labeled), and F independently verifies; otherwise record BLOCKED/PARTIAL with denominator and reason.  
**Proof:** journal hash, exchange digest, query transcript, wiki page digests, Rust/Cargo/HQ commands+exit codes and F report.

### D2-11 [P2; SAFE documentation / GATED tests] DEV/PRODUCT and Wave 3 interface preservation
**Targets:** lane-d/contracts/product-view.md; contracts/visual-intent-read-model.md; test/product-redaction.test.mjs (synthetic until integration).  
**Depend:** B privacy boundaries, C intent shape, E IR schema, TierFlow visual document owner, F release checks.  
**Work:** Document a privacy-filtered inert page/view projection and the needed visual-language metadata fields without replacing TierFlow renderer, Host executor or semantic/NL parser.  
**Pass:** no DEV root paths, untrusted input, test-only handlers, personal bulk KB or hidden approval state in product fixture; all required visual-intent fields are either provided or explicitly unsupported.  
**Proof:** contract and exhaustive fixture field matrix; no claimed Wave 3 UI implementation.

### D2-12 [P2; INTEGRATION-GATED] Evidence-based Rust/SQLite/wiki placement recommendations
**Targets:** lane-d/evidence/wiki-placement-input.md; quality/benchmarks.mjs (read-only, only if approved).  
**Depend:** F native placement report, E graph/query metrics, HQ wiki/SQLite index timing.  
**Work:** Measure graph→read model→wiki generation, page count, memory, p50/p95 rebuild, SQLite FTS versus graph traversal and freshness; recommend placement only where performance and writer authority justify it.  
**Pass:** no Rust takeover of HQ wiki/KB/SQLite writer or TierFlow UI document; benchmarks include input sizes, cold/warm environment, code/version pins and absent-data labels.  
**Proof:** reproducible raw measurements and recommendation from the enumerated categories RUST_NATIVE_NOW / RUST_ADAPTER / SQLITE_CANONICAL / SQLITE_DERIVED / KEEP_EXTERNAL_OWNER / DEFER_MEASUREMENT.

## 6. Execution partition while Master Integration is running

**SAFE TO UNDERTAKE IN A SEPARATE ROUND 2 EXECUTION SESSION:** D2-01/02/03/04/06/07 and documentation/fixture-only D2-11. All implementation changes must stay under lane-d/**; use new files during master baseline, do not alter current wiki-oracle.mjs until its baseline/receipt is frozen. Run Node isolated tests against *synthetic or pinned non-private public* sources; do not read/write owner's local index, live journal, or wiki output. Only write to new non-overlapping DEV temp paths.

**WAIT FOR MASTER INTEGRATION FINAL HANDOFF / OWNER PATCH AUTHORIZATION:** D2-05/08/09/10/12 and any modifications of existing Lane D oracle behavior used by master; any changes to HQ hq/lib/wiki.mjs or its configuration writer, E graph schema, A/B/C producer contract, F independent checker, source-native SQLite records or real index state. Master retains the right to decide connector interface and patch timing. D may issue a *read-only proposed patch contract* in lane-d meanwhile.

**Potential baseline-invalidating actions—PROHIBITED while master runs:** (i) rewrite E HQKGX1 metadata or entity identity semantics, (ii) alter source-root scope/indexing intent or reset/reindex existing DB, (iii) change wiki generator output/layout/page IDs or force regenerate master's wiki destination, (iv) reclassify 'wired'/'verified' or promote semantic IDs, (v) mutate Rust .hqj/journal or F evidence inputs, (vi) run any handler, activate a wiki program or grant Host permissions, (vii) change current oracle exit codes/defect schema while master consumes them. Such work requires explicit post-handoff authorization and a fresh-head check; never force or rebase over master without reviewing collisions.

## 7. Acceptance, score semantics and owner decisions

Round 2 cannot be declared done because source code or synthetic tests merely exist. Require **two separate statuses**:
- **D implementation correctness:** isolated tests/safety fixtures pass, strict source/contract/refusal invariants verified and benchmarks identified.
- **Cross-lane integration readiness:** real source-anchored graph + wiki output, full A/B/C/E mapping, native Cargo/HQ + query receipts, no-op/freshness + owner/authority proof and F independent verification. Must be BLOCKED if any foundational domain, real graph or generated wiki chain is absent.

Mandatory measures:
- Scope coverage: eligibleIndexedFiles / eligibleDeclaredFiles, by project/root and exclusions; unknown eligibility stays UNKNOWN, never 100%.
- Annotation compilation: source-byte anchored accepted/diagnosed/unsupported grammar tags by family and root.
- Semantic coverage: independently source-linked accepted semantic IDs / eligible IDs; alias collisions and proposed-only separately.
- Command coverage: source-inventoried, handler-map matched, CLI-dispatch present, runtime-verified and device-observed separately; no single "wired" total substitutes.
- KB link coverage: independently accepted/linked knowledge records / eligible exported records; redacted records counted but not exposed.
- Rust chain coverage: fully verified typed chains / eligible distinct discovered candidate chains, with separate compiled-comment subgate.
- Wiki coverage: pages/sections correctly representing proven chains / eligible verified graph chains; broken links / links checked; stale pages / page manifest entries.
- Determinism/freshness: sha256 before/after stable for no-op; changed/deleted entity page invalidation, restart query equality.
- Trust/safety: count false VERIFIED and false executable claims (must be 0), unsafe leakage and unsupported grammar/cross-version cases (must not silently pass).
- Native performance: real graph/wiki/SQLite p50/p95, input size, environment, timing provenance; don't invent targets without measured baseline.

**Verdict rules:** INTEGRATION PASS requires all mandatory subgates and F witness; PARTIAL only for bounded, clearly enumerated subset when the campaign charter permits it; BLOCKED for missing roots, source/KB/command, Rust import/query, real generated wiki chain, source-binder mismatch or untested native runtime. A pleasing score is irrelevant to safety gates.

## 8. Deliverables, ownership and handoff

**Owned location only:** vivim-labs/Architecture/10-THREE-WAVE-RUST-LIVE-GRAPH/wave-1/lane-d/. No branches/PRs/Actions/force pushes. Round 2 *proposal publication* consists of:
- ROUND-2-DESIGN.md (this record)
- ROUND-2-BACKLOG.md (numbered, executable objectives and gates)
- ROUND-2-EXECUTION-PROMPT.md (copy/paste mandate, not a launched run)
- ROUND-2-EVIDENCE.json (source pins, first-round proof tiers, integration mismatches, untouched surfaces)

**Existing authoritative owner boundaries:** Repos HQ hq/lib/wiki.mjs is the only wiki renderer/writer; hq/lib/wiki-program.mjs remains sole approved wiki program revision writer; B owns canonical knowledge/semantic SQLite, A indexing, C command inventory/safety, E graph projection/import/journal/read-only query, F independent verification, TierFlow owns UI document/render/revision, Host owns privileged effect execution and consent receipts. D owns **read-only integration acceptance**, test vectors, quality scores, diagnostics, and design—not new data authority.

## 9. Recommended implementation order and STOP RULE

Start independent D2-01, D2-02, D2-03, D2-04, D2-06 and D2-07 on isolated data; finish regression and interface drafts. On receipt of master HANDOFF, **re-read** exact heads, lane state, source SHAs and actual generated/wiki/journal artifacts; reconcile and revise only D's plan under owned scope. Ask owner of existing wiki builder for single-writer integrated read-model adoption; only after approval execute gated D2-05/08/09/10/12. If master currently relies on wiki-oracle.mjs, do not modify it at all before a signed-off baseline snapshot.

**No Wave 2 design or code execution is authorized by this document. Do not run Round 2 as part of publishing it.**
