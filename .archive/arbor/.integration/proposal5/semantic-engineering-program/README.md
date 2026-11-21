# ARBOR Proposal 5 — Semantic Engineering & Verification Program (SEVP)
**R001 | 2026-11-12 | FIRST_PASS_SCAFFOLD / DEEP_PREP_PENDING / NOT_IMPLEMENTED**
Branch: `work/p5-semantic-engineering-program-scaffold-r001`; base ARBOR main `539146ac741516e7d9711c63f4570585b3c3367e`. Re-pin before deep prep.

## Mission
Deliver one source-native developer platform: coding-agent annotation protocol → v1/v2-compatible v3 semantic language → compiler/typed source facts → independently reviewed behavioral claims → independent verifier/falsification and build/owner runtime evidence → claim-bound verbal explanations → HQ Wiki and HVI version intelligence → native agent/CI/IDE interfaces → reusable SDK. This is **not** a pure migration project. Reuse in-repo working code and selected research where justified, but implement missing contracts, trust roots and proof machinery.

## Read the scaffold
- [00 Charter and end state](00-CHARTER-AND-END-STATE.md)
- [01 Source truth and research intake](01-SOURCE-TRUTH-AND-INTAKE.md)
- [02 Architecture and authoritative owners](02-ARCHITECTURE-AND-OWNERSHIP.md)
- [03 Workstreams and vertical slices](03-WORKSTREAMS-AND-CANDIDATE-SLICES.md)
- [04 Metadata, claim and evidence contracts](04-CONTRACTS-AND-EVIDENCE.md)
- [05 Dependency DAG and possible phases](05-DEPENDENCIES-AND-PHASES.md)
- [06 Independent verification and acceptance gates](06-ACCEPTANCE-AND-REFEREE.md)
- [07 Reuse and migration decisions](07-REUSE-AND-MIGRATION.md)
- [08 Branches, workers and handoffs](08-BRANCHES-LANES-AND-HANDOFFS.md)
- [09 Next-turn deep prep](09-DEEP-PREP-BACKLOG.md)
- [10 Risks and open decisions](10-RISKS-AND-OPEN-DECISIONS.md)
- [11 Launch board](11-LAUNCH-BOARD.md)
- [Machine manifest](program-manifest.json); [receipt template](templates/PROOF-RECEIPT.template.md); [handoff template](templates/OWNER-HANDOFF.template.md); [future prompts](prompts/README.md)

## Research and precedent
- [SCF design branch](https://github.com/owenservera/ARBOR/tree/work/p5-semantic-capability-fabric-r001/.integration/proposal5/semantic-capability-fabric), [Semantic Verifier design](https://github.com/owenservera/ARBOR/tree/work/p5-semantic-capability-fabric-r001/.integration/proposal5/semantic-capability-fabric/verifier): draft PR #13, **not merged**, not implemented.
- [HVI execution/gates](../hq-version-intelligence/README.md), [HVI tooling-only correction](../hq-version-intelligence/10-NATIVE-TOOLING-ONLY-SCOPE.md), [HVI lane model](../hq-version-intelligence/04-OWNERSHIP-AND-WAVE-GATES.md).
- [Proposal 5 owner/gate precedent](../09-EXECUTION-PLAYBOOK.md).

## Protection
No executable prompts yet, no workers launched by this scaffold, no runtime tests or SDK implemented. The detailed dependency analysis, exact source pins, per-lane write scopes, tests, waves, Z referees, G gates and copyable prompts are reserved for next turn. Do not modify active HVI worker branches, execute real donor comparisons, assume current HVI GO, or auto-promote research designs.

**Canonical owners remain:** HQ one semantic registry/index/wiki writer; HQKG one Rust graph engine; Host sole effect/World authority; Studio sole provider sender; TierFlow sole UI publisher. No new graph/store, scheduler, effect executor, browser sender, wiki writer, provider APIs, Playwright/Puppeteer, unauthorized private source reads or automatic user-data cutover. Windows + paired PWA first.
