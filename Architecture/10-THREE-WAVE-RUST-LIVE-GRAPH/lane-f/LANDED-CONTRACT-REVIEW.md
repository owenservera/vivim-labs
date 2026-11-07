# Lane F landed-source independent contract review — 2026-11-07

**Verdict: BLOCKED_CONTRACT_CROSSWALK; Wave 1 Integration-1 remains BLOCKED.** All A-E implementations have landed as source, but source publication is not runtime execution.

## Real GitHub-source verification

At inspected Repos HQ head `34952780cb8dd85db3d48d60d50fed936ed41e2f`, the published A/B/C producers export `vivim.wave1.code-observations/1`, `vivim.wave1.kb-semantic/1`, and `vivim.wave1.command-candidates/1` respectively. The published Lane E assembler accepts exactly three `vivim.wave1.source-records/1` feeds, containing `nodes`, `edges`, `projectId`, `intentId`, `scopeRevision`, and source-root+commit pinned `provenance`. These producer outputs are not directly consumable by the assembled E importer.

Read the four exact source paths and Git blob SHA pins in `evidence/landed-source-contracts.json`. **No converter, no actual source-pinned converted records and no native Rust execution receipt were found in the inspected handoffs.** This finding is about evidenced compatibility, not proof that no unpublished converter exists on the owner's machine.

### Required owner-specific closure

- **A→E:** Convert indexed files/symbols/imports/annotations to source-root pinned `a:` nodes, static edges and explicit unknowns. Ensure A's absolute-path collection source and E's relative-prefix scope resolve to one identical real root, with actual bytes, current Git SHA and versioned intent.
- **B→E:** Convert bounded SQL records, semantic IDs and only exact verified source-SHA links. Preserve proposed/current statuses, unknown source refs, private-field omission and numeric HQ project ID provenance; don't invent `source:` endpoint nodes.
- **C→E:** Convert actual operations/CLI into `c:` records with source SHA+line and separate registry/handler/runtime/device evidence. `semanticId:null` requires independent source-linked canonical binding, not a guessed join. The documented `dont` negation and multi-clause legacy HQ route hazards remain must-test.
- **E:** Apply the converters or a separate integration-owned data-only linker, then validate with the existing assembler and Rust importer, not a second graph journal writer. Prove real compiled comment→B semantic→C handler in Rust or mark that subgate BLOCKED. Native no-op/deletion, mutation, stale/collision, restart and secret refusal receipts are still missing.
- **D:** Regenerate the source-owned HQ wiki **after** real E import, grade the same chain without treating wiki data as an independent authority.
- **F:** Run on the owner's actual repos-hq environment, collect raw exit codes/latencies/journal hashes, inspect HQKGX1 JSONL + Rust query and native wiki, then generate the observed coverage/placement/conformance/intent evidence. Keep a blocked state while any foundational domain or native Cargo receipt is missing.

## Executed independent verification (in this execution environment)

`node --check cross-lane-preflight.mjs`: exit 0. `node --test test/acceptance.test.mjs test/cross-lane-preflight.test.mjs`: exit 0, **12 passed, 0 failed**. Six newly added tests verify explicit schema mismatches, missing source refusal, schema version drift, noncanonical source envelopes, no false Rust proof from matching shapes, and source-byte hash changes. Fixtures are deterministic, synthetic contract cases. The actual on-GitHub four schema values were separately fetched and compared; they are recorded in the JSON source evidence.

`cargo` is not installed here. The owner's Windows machine, .hq/index.db, prior-run archive, real source-root manifests, private KB rows, DEV .hqj and corresponding generated wiki cannot be inspected or tested here. The full end-to-end gate therefore **does not pass**, regardless of isolated test success.

## F interface

`node cross-lane-preflight.mjs --repos-hq ABSOLUTE_REPOS_HQ_ROOT --out PRIVATE/source-contract-preflight.json` writes a no-effect machine report and returns exit 2 while incompatible producer/consumer contracts require adapters. The source-code checker never opens KB SQLite or initiates actions. Use `node --test test/*.test.mjs` for the independent test suite. The earlier `acceptance.mjs` is a provisional graph JSON adapter; do not feed E's HQKGX1 JSONL into it and treat the resulting gap numbers as evaluated Rust proof. After a real conversion, F must validate E's native exchange/query shape and D's oracle; until then this preflight refuses to self-certify.

No Wave 2 design or assignments are authorized by this report. Future interface data must preserve input/source spans, semantic identity, negation/quote/clause structure, alternatives, typed slots, required scopes, risk, world/input revisions and evidence for future visual-language input.
