# VLANG R000 — Round 1: archive-grounded research and SDK architecture

Date: 2026-11-21. **Status: first-round findings and executable research SDK scaffold; not R000 convergence or ARBOR R001.**

## Sources reviewed

- Canonical `README.md`, `BASELINE.md`, `ACCEPTANCE.md`, `MASTER-R000.txt` and `.archive/{README,SOURCE-CATALOG,NEXT-ROUNDS}.md`.
- Historical research conversation in `ARBOR-VLANG-UPGRADE/research/chat-VLANG Semantic Protocol Design (2).txt`.
- Historical `@hq:semantic/3` SCF proposal and agent SDK protocol; historical source snapshots of HQ v1/v2 and semantic registry.
- Current ARBOR main inspected at `7d1f4cb2750a8c864b3d524eb494cffe602f5a60` (2026-11-21 02:49:20 UTC): `hq/lib/{protocol,annotation-v2,indexer,semantic-registry,context}.mjs`, `hq/test/annotation-v2.test.mjs`, `hq/bin/hq.mjs`, `native/hqkg/src/semantic_source.rs`.

## Finding: actual protocol generations

| Layer | What exists | Round-1 decision |
| --- | --- | --- |
| `@hq:` v1 | `hq/lib/protocol.mjs`; guidance/lint for context, owner, invariant etc. | preserve and do not absorb into v2 |
| `@hq:` v2 | `hq.annotation/2`: 9 typed tags, registry resolution, comment-aware JS/TS scanner | R0 SDK delegates to it; no new grammar |
| Rust | HQ indexer now uses file-header-only preprocessing before v2 scanner | retain exact conservative limit |
| JSX/TSX | excluded from current v2 scan | explicit unsupported, no guessed support |
| `@hq:semantic/3` | SCF extension proposal, not accepted live grammar | keep future RFC-only |
| `@v.` | original conceptual VSL grammar in chat export | keep as design alternative; no silent renaming |
| Semantic identity | HQ registry uses versioned IDs and conservative file anchors; moves are not proven renames | no auto-identity; require reviewed lineage |
| HQKG | imported graph reads source anchors from existing owned graph and journal | adapter only; no second graph |
| Context | HQ composes bounded, source-hash-verifiable packets | require independent verifier before use |
| Approval | Host owns effects; source annotation never grants runtime capability | SDK has no execute/grant/install methods |

## Candidate SDK boundaries (not new services)

```text
Human / coding agent / editor
       |
    VLANG SDK: discover / inspect / context / delta (read-only R0)
       | typed source pin + versioned ID + diagnostics
       +--> HQ v1/v2 parser + existing registry + HQ index
       +--> HQ context + independent source verification
       +--> HQKG canonical graph (future read-only adapter)
       +--> AutoWiki / verifier projection (future, existing owner)
       +--> Host authority (future proposal handoff only; NEVER SDK approval)
```

Language-agnostic contract = source identity (repo/project/path/full content hash), annotation occurrence, assurance tier, supported-language scope, diagnostics, and a verified context packet. Wire types should be pinned and backwards-compatible; no SDK status bit is treated as runtime readiness.

## First-round implementation

- Added SDK package, pure `inspectSource`, `getVerifiedContext`, `compareSourcePins` methods, explicit safety/refusal codes, and Node contract tests.
- `inspectSource` verifies supplied source bytes but **does not** prove live filesystem freshness. Parser injection preserves HQ grammar ownership.
- `getVerifiedContext` requires the adapter to return independently verified source currentness, including the requested SHA.
- All annotations remain `declared-unverified` and `executionAuthorized:false`, regardless of registry resolution.
- No new SQLite, git dependency, hosted API, model dependency, compiler, VEC executor, or Host permission route.

## Design tradeoffs and unresolved questions

1. Two grammars (`@v.` vs `@hq:`): choose **compatible HQ v2** for MVP; decide formal vocabulary only after R000 lane 02.
2. Stable ID: use registered semantic IDs; distinct Git source/hash/file anchor; no rename inference until owner-reviewed migration contracts.
3. AST: tree-sitter supports incremental parsing, while rust-analyzer / TypeScript should resolve actual symbols. A line-comment-only scan cannot prove inline Rust or TSX symbol attachment.
4. Evidence trust: `test:` ref is declared relationship, not passing test evidence. Next phase requires independently validated receipts and negative tests.
5. Versioning: SDK `0.1-research` is deliberately separate from HQ `annotation/2`; no silent major-version migration.
6. Context: exact current-source check must be implemented in the production adapter; mock port tests are contract checks, **not proof of ARBOR integration**.
7. No existing user-owned personal state belongs in source annotations; local Host remains authoritative.

## Round-2 ordered build target

1. Implement real injected ARBOR HQ adapter and run actual parser/indexer/context tests on current HEAD.
2. Finalize one source-binding contract and Rust/TSX parser-negative corpus; validate against current source.
3. Implement source revision fencing, on-demand context, delta/preservation and claims without new graph storage.
4. Add source-backed evidence verifier and multi-client lifecycle demonstration.
5. Promote only tested, compatible pieces to ARBOR `main`, with concurrency-safe fast-forward commits.

## Verification ledger

- Source review: GitHub read of named files and archive; **completed**.
- SDK JS contract tests: **UNRUN against ARBOR** at research authoring time; no claim of integration.
- Rust/TSX semantic binding: **UNSUPPORTED** in R0.
- Product wiring, Host acceptance, device tests: **UNRUN**.
- Historical archive contents were not modified.

References: `.archive/SOURCE-CATALOG.md`, `.archive/README.md`, `BASELINE.md`, `ARBOR-VLANG-UPGRADE/README.md`, current ARBOR repo above. External research: tree-sitter official docs, SCIP code intelligence, Cargo workspaces; cited in accompanying response.
