# ARBOR Proposal 5 — FA-PGZ / Independent Foundation Pre-G Convergence Referee

**RUN AFTER PG00/PG01/PG02/PG03 HAVE COMMITTED OR BEEN EXPLICITLY BLOCKED.** Read only from their isolated branches. **This is NOT original W1-G and has no merge or release authority.**

**Repository:** `owenservera/ARBOR`
**Create:** `work/p5-fa-pgz-convergence-r001` based on current pinned `work/p5-foundational-architecture-prep-r001` HEAD, not on any specific PG worker's branch.
**Write ONLY:** `.integration/proposal5/foundations/proofs/PGZ-convergence/`.
**Completion:** `PREPARED_NOT_PROMOTED`; no merge, no other lane edits, no main/G/root changes.
**First read:** `.integration/proposal5/foundations/prompts/00-SHARED-PRE-G-CONTRACT.md`, `../WAVE-MANIFEST.v1.json`, `../GAP-REGISTER.v1.json`, `../SOURCE-ATLAS.v1.json`, `../FOUNDATIONAL-ARCHITECTURE.md`, `../PROMPT-FACTORY-SCAFFOLD.md`. All refs are relative to `.integration/proposal5/foundations/` except shared contract, which is in `prompts/`.

## Required primary inputs

Verify the **exact** HEAD and files, and read all actual source/fixtures rather than only summaries:

| Lane | Expected branch | Exclusive staged output |
|---|---|---|
| PG00 Source Truth | `work/p5-fa-pg00-source-truth-r001` | `.integration/proposal5/foundations/proofs/PG00-source-truth/` |
| PG01 Native ABI | `work/p5-fa-pg01-native-abi-r001` | `.integration/proposal5/foundations/proofs/PG01-native-abi/` |
| PG02 Semantic | `work/p5-fa-pg02-semantic-fabric-r001` | `.integration/proposal5/foundations/proofs/PG02-semantic-fabric/` |
| PG03 PWA/Rock | `work/p5-fa-pg03-pwa-storage-r001` | `.integration/proposal5/foundations/proofs/PG03-pwa-storage/` |

Read `README.md`, `SOURCE-DELTA.json`, `OWNER-HANDOFF.md`, `NEGATIVE-ACCEPTANCE.md`, `RUN-LEDGER.json`, and code/ABI/fixture artifacts from each available branch. **If a branch is absent or blocked, do not wait**; mark `NOT_RUN`/`BLOCKED` and proceed with the others.

Read current frozen A–F refs and their `integration/worker-reports/PREG-*.md`. Check original G `integrate/proposal5-beta-r001` ref and `.integration/proposal5/prompts/W1/G.md` independent of the worker reports. **If G has appeared in the meantime**, distinguish `BRANCH_EXISTS` from `G_ACCEPTED` and don't change G.

Read existing S-semantic, V-visual, W-autowiki, N0-io, P2-T/K/C and H1 actual test/manifest sources so new PG outputs are never blindly recopied or incorrectly treated as new behavior.

## Mission: make the remaining integration an auditable, minimal delta

1. **Normalize source origin and status:** verify original Git refs/blob SHAs used by each lane, source drift, and which files were copied or duplicated. Reconcile any contradictions between PG00 canonical owner map and individual source-specific PG01/02/03 findings.
2. **Cross-owner seam:** map exactly how these prepared contracts *could* connect AFTER G through the current original source owners: A pure Rust generations/events, B Host actor/plan/grants, C Studio thread/agent context, D HQ SemanticWikiIR/source wiki, E TierFlow/PWA, F type/test parity. **Do not create a new event router, permission writer or global compiler.**
3. **Native reality check:** source-present `EngineRegistry` and pure typed DAG are not a reprogrammable user platform without verified actor attestation, manifest/ABI, source digest, rollback and real Cargo tests. If PG01 only produced fixtures with UNRUN Cargo, categorize native proof as **NOT YET VERIFIED**.
4. **Semantic reality check:** S v2 comment compiler, B Host NLCL, D wiki-program and E visual commands are distinct. Review PG02 adapter and decide which narrow source-reviewed IR projection is viable, with no comment→runtime-effect promotion.
5. **PWA trust and Rock:** identify true beta P0 security/packaging/source issues in PG03. Retire mandatory APK build dependency for PWA-first **only through future A/G authorized source change**. Require actual browser signer/Host static route/replay and source-protected physical disk guards before claiming safety.
6. **Scope gate:** reduce all proposed work to no more than **five minimal, source-specific post-G owner actions** needed to demonstrate one semantic read → visual feedback → authorized Host read and one native swap/rollback proof. Do not promote features that G hasn't integrated.
7. **Acceptance gate:** for each planned work item distinguish `SOURCE_PRESENT`, `PREPARED_NOT_PROMOTED`, `TEST_EXECUTED`, `RUNTIME_VERIFIED`, `DEVICE_ACCEPTED`, `BLOCKED`. Include exact commands and exit evidence. No "PASS" inherited from a worker's claim without a source/check receipt.
8. **Release and privacy:** preserve no provider API, no duplicate source store/effect owner, no personal data ingestion, **PWA first/APK future**, and no intent to delay G.

## Exact outputs under your subtree

- `README.md`: authoritative observed branches, scope coverage, missing workers and current original G truth.
- `FOUNDATION-CONVERGENCE-MATRIX.json`: each PG source/ABI seam, owner, source SHA, conflict, decision `KEEP | ADAPT | TEST_ONLY | DEFER | BLOCKED | DUPLICATE_REVIEW`, expected accepted G integration gate.
- `CONTRACT-COLLISIONS.md`: any semantic IR/Node0/UI/Host/native/SourceRef schema incompatibility, exact test owner and no invented unified type.
- `MINIMUM-SOURCE-PATCHES.md`: ≤5 source-owner tasks after actual G HEAD, exact allowed path, code module owner, negative acceptance and fallback/rollback.
- `READINESS-MATRIX.md`: per lane GO/NOT_RUN/BLOCKED and combined acceptance path with actual evidence.
- `NEGATIVE-ACCEPTANCE.md`: end-to-end effect, source privacy, cancellation, stale source, storage and PWA tests.
- `RUN-LEDGER.json`: GitHub compare changed paths, source pins, actual Node/V8/Cargo tests versus UNRUN, **PREPARED_NOT_PROMOTED**.

## STOP

Write only `proofs/PGZ-convergence/`, verify HEAD and GitHub comparison no other paths, and STOP. Do not merge original G, branch-import the four isolated proofs, modify main/release/root or authorize real personal data. The review decision is **PREPARED_NOT_PROMOTED** with owner handoffs; only G and later original post-G owners have adoption authority.
