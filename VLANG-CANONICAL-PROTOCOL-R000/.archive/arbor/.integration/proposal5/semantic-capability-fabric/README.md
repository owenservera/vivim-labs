# Proposal 5 — Semantic Capability Fabric (SCF) / Agent-Authored Source Protocol
**Proposal identifier:** SCF-R011-001 · **Date:** 2026-11-12 · **State:** IMPLEMENTATION_BLUEPRINT_COMPLETE / NOT_PRODUCT_PROMOTED · **Owner:** ARBOR HQ (development intelligence). 
**Work branch:** `work/p5-semantic-capability-fabric-r001` forked from ARBOR main `f9823a5131e73e7562db5a4ec14e1ec481557d69`. **Coordination:** active `.integration/proposal5/hq-version-intelligence/` (HVI) wave. DO NOT edit its files or merge this design over an active worker's pin.

## Implementation-ready engineering kit — START HERE

- [Complete engineering blueprint](implementation-ready/README.md): exact file list, target interfaces, near-code algorithms, SQLite DDL, contracts, golden fixtures and test gates.
- [Single-agent master work order](implementation-ready/prompts/00-MASTER-IMPLEMENT-ALL.md): source-pin lock, code work and independent verification without redesign.
- [Parallel/serial launch index](implementation-ready/08-LAUNCH-INDEX.md): S0 → A/B/F → C/D/E → Z1 → G1 → H/U → Z2 → G2 → R.
- **Current HVI policy:** native tooling only, synthetic invented Git fixture comparisons; real VIVIM/ARBOR/Labs migration analysis expressly deferred by HVI `10-NATIVE-TOOLING-ONLY-SCOPE.md`.
- These are implementation instructions, **not product code**. PR #13 must remain draft until active HVI source gates are reconciled.

## Core design
**The source file is a small signed-off declaration; the wiki is the compiled view.** Require coding agents to maintain stable feature membership, file purpose, owner and contracts within real inline comments. HQ combines these **untrusted declarations** with actual symbols/imports/dependencies, canonical semantic IDs, source-pinned Git trees, tests, Host catalog and runtime evidence into a reproducible, source-linked **Semantic Capability ABI (SC-ABI)**. The existing HQ wiki and indexer become human/machine projections; HVI compares the same ABI between 2..N immutable pins. No duplicate knowledge/writer/graph/executor.

One edit → changed annotation parsed → identities resolved → source facts cross-checked → incremental graph and capability passports → deterministic wiki and architecture views → migration delta and context → read-only product UI. Only Host can approve effects; TierFlow visual projections remain inert; Studio remains only signed-in browser sender.

## Documents
- [00 — Source sitrep](00-SOURCE-SITREP.md): verified existing code vs research/design.
- [01 — Next-generation experience](01-PRODUCT-VISION.md): product and developer goals, feature passports, architecture time machine.
- [02 — Coding-agent protocol](02-AGENT-CONTRIBUTOR-PROTOCOL.md): how every changed first-party source unit is documented and checked.
- [03 — Additive inline meta-language](03-SEMANTIC-COMMENT-LANGUAGE.md): v1/v2 compatibility, proposed v3 identity/migration extensions, examples.
- [04 — Typed facts and ABI](04-SEMANTIC-FACT-ABI.md): capability graph facts, trust/evidence, pin/lineage and HVI.
- [05 — Compiler/index/wiki architecture](05-INCREMENTAL-COMPILER-AND-WIKI.md): bounded incremental pipeline, one writer, static/dynamic evidence.
- [06 — Version-intelligence convergence](06-HVI-MIGRATION-OPTIMIZATION.md): how tags make HVI cheap and precise without falsifying legacy gaps.
- [07 — App and agent UX](07-INTERACTION-AND-UX.md): open a feature/screen and know its actual role.
- [08 — Upgrade plan and owners](08-UPGRADE-WAVES-AND-OWNERS.md): bounded stages after current HVI acceptance.
- [09 — Verification and red-team matrix](09-ACCEPTANCE-MATRIX.md): security, correctness, performance, migration regression.
- [10 — Decision contract](10-DECISIONS-AND-HANDOFF.md): integration-ready owner tasks.
- [Machine-readable scope and status](proposal-manifest.json).

## Verification counterpart (SV-R012)
**[Semantic Verifier — typed proof-carrying claims, execution evidence and explanation checking](verifier/README.md)** is a proposed independent verifier complement to SWC/SCF. It specifies minimal additive `@hq:verify`, `@hq:observe`, `@hq:explain` references, claim/proof/explanation schemas, a small independent checker, bounded falsification tests, runtime-owner witness interfaces and HVI 2..N proof deltas. **Design only; tests and runtime integration UNRUN.** It remains on this isolated branch and must not modify active HVI worker source pins.

## Critical boundaries
Preserve `hq/lib/protocol.mjs` (v1), `hq/lib/annotation-v2.mjs` (v2), `hq/lib/semantic-registry.mjs`, `hq/lib/indexer.mjs`, `hq/lib/wiki.mjs`, `hq/lib/wiki-program.mjs`, `hq/lib/arbor-service.mjs` and HVI as canonical owned components. The **existing SWC v0.1** research compiler is an inert typed projection; opt-in v3 extensions are not currently parsed/installed. For low-cost rollout, require only a tiny metadata header and leave derived facts to the compiler. An annotation cannot prove that functionality is reachable, tested, shipped or permitted; source and runtime evidence remain independent.
