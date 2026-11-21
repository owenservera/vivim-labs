# Wave 1 Lane D / Round 2 — numbered execution backlog

**DESIGN ONLY (not executed).** See ROUND-2-DESIGN.md for full audit, safety, evidence, and gate rules.

| ID | Priority | Can work while master runs? | D-owned source target | Required upstream | Test / explicit acceptance | Evidence & coverage denominator |
|---|---|---|---|---|---|---|
| **01 D2-01 Contract matrix** | P0 | YES, separate new files | contracts/actual-contract-matrix.json, contracts/validate-inputs.mjs, test/contracts.test.mjs | A/B/C actual schema, E assemble, F schema review | Unknown/changed schemas, missing source pin and metadata refuse; 0 silent coercions | Fields classified / fields observed; exact blob SHAs and exit codes |
| **02 D2-02 Typed lineage graph** | P0 | YES, new files | lineage/validate.mjs, lineage/traverse.mjs, test/lineage.test.mjs | E relation kinds, A source maps, B IDs, C handlers | code→compiled→semantic→registered command; orphan KB cannot count; 0 false VERIFIED | Verified chains / candidate chains, each edge proof |
| **03 D2-03 Provenance/state vectors** | P0 | YES, new files | lineage/provenance.mjs, lineage/states.mjs, test/provenance.test.mjs | root scope, content SHA, B authority, C runtime status | SHA-1 ≠ SHA-256; forged receipt, drift and claim-only status never verified | Verified proof fields / required fields; drift/refusal tests |
| **04 D2-04 Correct score and denominator** | P0 | YES | quality/score.mjs, quality/defect-schema.json, test/score.test.mjs | A root counts, B/C/E sample totals | zero denominator=NOT_MEASURED, critical gates override 0–100; no numerator overflow | Files per root, KB/semantic/action/compiled/wiki denominators |
| **05 D2-05 HQ wiki true integrated read view** | P0 | **NO** (spec only safe) | contracts/wiki-read-view.md, test/integrated-wiki-contract.test.mjs | E real Rust import, owner-approved HQ wiki patch | HQ existing buildWiki solely writes Markdown/search/HTML from A+B+C+E; real typed chain rendered with meaning/evidence/status; no second renderer | Output page manifest, page/graph SHA, F witnessed chain; verified wiki chains / eligible graph chains |
| **06 D2-06 Links/privacy/nav** | P1 | YES | quality/links.mjs, quality/privacy.mjs, test/links-security.test.mjs | HQ manifest format | Resolve relative links + anchors safely, case collisions and privacy leaks; all adversarial fixtures detected | Broken/checked links; number of seeded unsafe cases and misses |
| **07 D2-07 Meaning regression** | P1 | YES (synthetic/pinned public only) | fixtures/README.md, test/meaning-regression.test.mjs | recovered docs; B/C source cases | ≥30 documented conflict, negation, unsupported, misleading comment and stale fixtures; 0 unsafe pass | Cases per risk category, exact expected/actual |
| **08 D2-08 No-op/mutation/deletion/restart** | P1 | **NO** | quality/freshness.mjs, test/freshness-native.test.mjs, evidence/incremental-diff.json | real A/E journal, HQ writer, F | Stable re-run and restart; changed/deleted/renamed facts update/flag, no current-looking stale wiki; if additive E cannot delete then BLOCKED | Affected pages / affected graph entities; digest and native exit receipts |
| **09 D2-09 CLI/NL/wiki parity** | P1 | **NO** | quality/operation-parity.mjs, test/operation-parity-native.test.mjs | C real inventory/refusal corpus, E IDs, HQ writer | Exact operation ID/risk/scopes from same source; no command invocation; rejected negation stays rejected | Covered registered C ops / eligible, false executable claims = 0 |
| **10 D2-10 E2E native/F gate** | P1 | **NO** | bin/run-quality.mjs, quality/report.mjs, evidence/native-run/*, future HANDOFF update | A reset, B/C live, E Rust Cargo/journal/query, HQ wiki, F | one actual code→compiled comment→KB/semantic→handler→Rust→wiki chain, source/query/page digest parity and independent F rerun | Roots, native tests/exits, verified chains, no-op/restart and 0 forged claims |
| **11 D2-11 DEV/PRODUCT + visual-intent read model** | P2 | Spec safe; real test gated | contracts/product-view.md, contracts/visual-intent-read-model.md, test/product-redaction.test.mjs | B privacy, C typed intent, E IR, TierFlow owner | Only inert/redacted data; visual token spans, negation/quote structure, alternatives, revisions exposed or listed unsupported | Fields supported / required; redaction cases |
| **12 D2-12 Placement inputs** | P2 | **NO** native measurement; design safe | evidence/wiki-placement-input.md, quality/benchmarks.mjs | F Rust-vs-SQLite report, HQ/SQLite/wiki timings | Meaningful p50/p95 with sizes/cold-warm tests; owner-preserving placement recommendation only | Native benchmarks with environment and source pins |

### Dependency-ordered milestones

**M0 Design published:** D-owned documentation only; no implementation, no native or owner-machine effects.  
**M1 Parallel-safe tests:** independent strict readers, lineage, trust states, denominator metrics, links and adversarial corpus; all synthetic data labeled. Do not edit wiki-oracle.mjs (baseline master might import it).  
**M2 Integrator returns:** verify freshly published authoritative master HANDOFF + A/B/C→E converters + actual HQKGX1 exchange, DEV journal receipt, Rust queries and generated wiki files. Any absent gate remains BLOCKED.  
**M3 Writer-owner integration:** only owner-authorized HQ buildWiki read-only graph view patch, no D renderer or E/wiki second store. Record changed page identities.  
**M4 Native acceptance:** real source tests + Cargo/HQ + no-op/change/delete/restart + F independent witness. Stop after Wave 1 Round 2 result; **do not design Wave 2.**

### Stop/rollback list (baseline-invalidating)

1. **NO** edits to repos-hq/hq/lib/wiki.mjs, wiki-program.mjs, comment-wiki-bridge.mjs without owner and post-integration handoff.
2. **NO** changes to A/B/C/E contracts, source IDs, Rust import/journal/query, Lane F verifier or their tests while integrator baseline is active.
3. **NO** HQ DB/index reset, root registration, wiki re-generation in master destination, semantic promotion or permission changes.
4. **NO** action dispatch, browser/OS/device operations or provider APIs. Never use Playwright/Puppeteer.
5. **NO** claim that fixture tests validate real integration or that a wiki page is a canonical KB record.
6. **NO** Wave 2 planning. Version/lease-safe GitHub main-only publication after checking fresh head.
