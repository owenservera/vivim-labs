# Wave 1.5 — Continuous Context + Agentic File Corpus
**Status:** design published for experimentation; NOT implementation, deployment, or Wave 2 authorization.  
**Scope:** VIVIM Labs research supporting canonical Repos HQ; current Wave 1 DEV baseline `acad98697575c2f3fe2fcbf847e19a8dcd3670db`.  
**Date:** 2026-11-07.

## Outcome
Design a continuous, bounded, evidence-qualified context controller that supplies a changing agent with the right source spans at the right safe delivery boundary; design a corpus-driven file authoring system capable of proposing project-, task-, and agent-specific contents for all 217 observed agentic-file *instances*, using 133 candidate naming patterns and 21 family recipes without mass-spawning documents or claiming authority.

## Documents
1. [ALGORITHMIC-DESIGN.md](ALGORITHMIC-DESIGN.md) — complete corpus compiler and draft verification pipeline, 21 families, constraints, interfaces and algorithms.
2. [DYNAMIC-CONTEXT-LRU.md](DYNAMIC-CONTEXT-LRU.md) — event-driven context queue, two-tier revision-aware LRU, invalidation, permissions and delivery.
3. [SIMULATION-AND-ACCEPTANCE.md](SIMULATION-AND-ACCEPTANCE.md) — virtual agents, scenario goldsets, baselines, metrics, ablations, safety and stop rules.
4. [CORPUS-217-DRAFT-PLANS.json](CORPUS-217-DRAFT-PLANS.json) — one proposed recipe/required-section/grounding plan for each of the 217 registry records, provenance and disposition; **no agent files have been drafted or activated**.

## Source canon and verified/not verified
- Existing code: [HQ composeContext](https://github.com/owenservera/repos-hq/blob/main/hq/lib/context.mjs), [filetypes](https://github.com/owenservera/repos-hq/blob/main/hq/lib/filetypes.mjs), [HQ DCB protocol](https://github.com/owenservera/repos-hq/blob/main/docs/DCB-PROTOCOL.md), [rulebook](https://github.com/owenservera/repos-hq/blob/main/docs/AGENTIC-RULEBOOK.md).
- Actual census: [217-file registry](https://github.com/owenservera/repos-hq/blob/main/hq/registry/agentic-file-registry.v1.json), [217-file inventory](https://github.com/owenservera/repos-hq/blob/main/research/agentic-knowledge-graph/file-inventory.json), [13 section types](https://github.com/owenservera/repos-hq/blob/main/research/agentic-knowledge-graph/section-types.json), [1,010 heading variants research](https://github.com/owenservera/repos-hq/blob/main/research/agentic-knowledge-graph/INTEGRATION-STATUS-2026-11-03.md), [22 algorithm-node records](https://github.com/owenservera/repos-hq/blob/main/research/agentic-knowledge-graph/algorithms.json), and [217×10 metadata heuristic scores](https://github.com/owenservera/repos-hq/blob/main/research/agentic-knowledge-graph/scores-217x10-v0.csv).
- Older lab recovery: [DCB architectural review](https://github.com/owenservera/vivim-labs/blob/main/documentation-consolidation/vivim-web-harness/docs/architecture/DYNAMIC-CONTEXT-BUNDLES-REVIEW-2026-10-31.md), [context integration](https://github.com/owenservera/vivim-labs/tree/main/documentation-consolidation/vivim-web-harness/docs/context/master-integration-2026-11-01).
- Existing Harness composer and source adapters: [dcb-composer.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/harness/src/context/dcb-composer.mjs), [dcb-sources.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/harness/src/context/dcb-sources.mjs).
- Prior experiments: [LAB#1 dogfeed](https://github.com/owenservera/repos-hq/blob/main/docs/LABS/LAB-01-REALTIME-DOGFEED.md), planned LAB#2 context quality, [working memory](https://github.com/owenservera/repos-hq/tree/main/research/personal-intelligence/working-memory).
- Wave 1 proof and caveats: [Wave 2 input dossier](https://github.com/owenservera/repos-hq/blob/main/hq/live-graph/wave1/master/RESULTS/WAVE-2-INPUT-DOSSIER.md).

### Census interpretation
217 **real file records** = 209 sources + 8 donor snapshots; 195 records were sampled from AION; 133 **candidate filename patterns** are not 133 observed files. There are 140 type records in the current registry because it also includes observed-only types. Only eight of the 21 families occur among the 217 census files (01 53; 02 7; 04 46; 08 5; 09 67; 10 7; 12 20; 19 12). The prior corpus research reports 217/217 pinned snapshot hashes and 1,010 extracted heading variants. This publication inspected the complete metadata registry/inventory, score/section contracts and selected original bodies; it did **not** independently reread every one of the 217 original text bodies in this run. Do not misstate per-file semantic validation.

## Owners and placement
- **VIVIM Labs:** research, scenarios, generated candidate manifests, reports, non-authoritative fixtures.
- **Repos HQ:** authoritative agentic-file registry/taxonomy; index, graph, source spans, context composer and evidence; proposed draft compiler and simulation harness after owner authorization.
- **Rust HQKG:** bounded typed traversal/compute of source→identity→dependency→evidence; neither document publisher nor policy owner.
- **Web Harness/Host:** existing DCB personal disclosure policy, real agent/provider/browser delivery; no new sender or duplicate primary memory.
- **Agentic files:** effective provider-native instructions only after explicit owner-approved activation at their rightful paths. Historical copies remain inert.

## First experiment order
1. Pin source heads; recover actual corpus bodies and per-section source hashes; make the 217-plan audit reproducible.
2. Add a *read-only* `filetypes.autoExpose` → `composeContext` candidate seam, and bounded Rust graph neighborhood input; compare to baseline unchanged.
3. Add event-driven source-revision invalidation, a cache with byte ceiling, and a context queue that does not rewrite a model's in-flight request.
4. Draft inert candidate agent files from verified spans under 21 family recipes; validate exact provider syntax and effective instruction precedence.
5. Run virtual-agent goldsets/negative tests, cache ablations, behavioral outcomes, and independent acceptance. Report failures before any live activation.

## Hard rules
A document never grants execution authority. A draft cannot create a live AGENTS.md, SKILL.md, hook or provider rule without the owner and runtime's explicit authorization. No external provider sends, APIs, Playwright/Puppeteer, duplicate browser controller, broad personal data export, second canonical wiki/memory writer or altered Wave 1 acceptance evidence. Read-only operations must not change canonical repositories. No proposed cache or DCB substitutes for the owner's actual user request or task.
