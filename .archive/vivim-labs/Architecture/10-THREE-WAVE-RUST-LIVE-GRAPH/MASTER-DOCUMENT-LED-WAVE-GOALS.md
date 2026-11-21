# MASTER REVIEW — Document-Led Wave Goals (Not Code-Led Roadmapping)

Date: 2026-11-07
Status: architectural outcome definition and critical evaluation. Wave 1 is executable; Waves 2 and 3 are outcome targets, **not launched plans or fixed lane assignments**.
Parent: [Wave 1 execution](EXECUTE-WAVE-1-FIRST.md)
Companions: [Compiler and Deterministic Language HOW](COMPILER-LANGUAGE-AND-PLACEMENT-REVIEW.md), [Wave 1 scoped reset and indexing intent](WAVE-1-INDEX-RESET-AND-INTENT.md)

## The critical instruction
**The collected design documentation defines WHAT the system must become. The actual source code, native tests and running graph establish WHERE we currently stand. We then design the HOW and prioritize missing work from evidence.**

Do not use the cleaned source tree as a complete requirements inventory. The [documentation consolidation manifest](../../documentation-consolidation/INVENTORY.json) enumerates 1,234 source documentation-format files: 137 moved into Labs, 1,097 retained in source as of the migration report. Some relocated docs carry major architectural decisions absent from current source paths. Conversely a historical design document is not proof of a live implementation or current authority. Track both along with supersession and confidence.

**Two distinct discovery passes:**
1. **Design recovery** (historical archived + retained architecture, roadmaps, user-directed goals): establish end state, constraints, contradictions, decisions and intended use cases. This pass defines wave outcomes; it is not a runtime input ingestion mandate.
2. **Implementation discovery** (current source, registries, runtime, tests, CLI, receipts): establish observable state, gaps, boundaries and evidence. This pass drives actual code integration and is the feed to the Rust graph.

The doc inventory is a requirements **source map for designing the waves**. Whether selected docs should also contribute bounded, provenance-labeled KB evidence is a separate decision, not implied by the migration.

## Document precedence / contradictions
- Current **owner direction** outranks old brainstorming. Accepted, dated architecture decisions outrank superseded drafts for current implementation. Actual runtime evidence outranks code comments for tested readiness.
- Old [TierFlow greenfield Rust-authority UI blueprint](../../documentation-consolidation/tierflow/realtime-stateful-ui-blueprint/01-system-architecture.md) is an important aspiration, **superseded for implementation** by [accepted TierFlow 2026-11-03 assessment](../../documentation-consolidation/tierflow/realtime-stateful-ui-blueprint/07-current-implementation-assessment.md): preserve its Bun/SQLite UI document writer and existing bus; Rust can compute/aggregate, not seize UI transaction authority on speculation. Do not conflate this UI Rust sidecar with the separate HQKG Rust knowledge graph.
- [Archived semantic-language architecture](../../documentation-consolidation/repos-hq/docs/research/semantic-language-kernel/ARCHITECTURE.md) distinguishes source-comment declaration grammar from user NLCL grammar; sharing IDs does NOT mean making one grammar or action permission model. Original note called v2 tags “proposed”; later source now includes a pure `annotation-v2.mjs` parser and `research/comment-program-rust`, so assess implementation against newer actual code, not static status prose.
- [Archived 2026-11-04 intent audit](../../documentation-consolidation/repos-hq/docs/audit-2026-11-04/01-intent-front-end.md) found negation and clause-loss bugs and catalog/verification limitations at that date. These are **regression targets to re-test**, not assertions about the latest runtime until the current binary is exercised.
- [Repos HQ shared KB/kernel decision](https://github.com/owenservera/repos-hq/blob/main/docs/SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md) and [compounding architecture](https://github.com/owenservera/repos-hq/blob/main/docs/architecture/2026-11-04-COMPOUNDING-VIVIM-ARCHITECTURE.md) favor source-native writers and reusable typed semantics; do not create a second KB or authorizer.
- [VOMEGA visual command language design](https://github.com/owenservera/VOMEGA/blob/main/seed-docs/COMMAND-VISUAL-LANGUAGE-DESIGN.md) defines the future **user experience**, not a mandatory new UI framework. [Archived Harness GUI02 semantic canvas research](../../documentation-consolidation/vivim-web-harness/docs/research/GUI02-PRE-PRD-CANVAS-SEMANTIC-LANGUAGE-2026-10-31.md) recovered same intent and carefully distinguishes proposed from approved technology.
- [TierFlow visual waves](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVES.md) and [final convergence](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/FINAL-WAVE/README.md) assign visual document/revision/render and Host effect ownership independently of this architecture campaign. These are **separate** existing Wave 1–6 contracts, not this new 3-wave Rust/HQ campaign; do not overwrite their gate truth.

## The one strategic end state
**The indexing system matures into the central, incremental, source-aware semantic compiler of a stateful knowledge and command environment.** One project may span many source roots; an indexing-intent declaration tells it why a set of sources matters. It discovers code, symbols, annotations, schemas, contracts, dataflows, knowledge, command definitions, tests and implementation evidence; compiles a validated, versioned **semantic intermediate representation (IR)** with stable identity and provenance.

From that IR:
- **Rust HQKG:** live graph of semantic entities, actual engine/capability/command implementations, dependencies, transformations, I/O/communication evidence, gaps and impact queries; may own pure compilation/validation/reasoning kernels when proven beneficial.
- **SQLite:** canonical durable HQ semantic/KB revisions where existing owner, source indexes/FTS and searchable tables, append-only evidence where appropriate; maintains transactional write invariants.
- **Command layer:** one bound typed operation vocabulary for NLCL, CLI, agents, wiki gesture and visual commands; an interpreter suggests or resolves, an authorized Host or native owner executes.
- **Intelligent wiki:** *derived, self-explaining documentation and programmable safe views* from the integrated A (code) + B (knowledge/identity) + C (command) facts. Missing/incorrect wiki output is a diagnostic about these layers and their graph joins, not a fourth separate registry.
- **Visual language:** real-time projection of the *same typed intent*, explaining every important token, grounding, alternative, scope, effect, risk and authority state before any commit.

This is **one semantic model**, not one universal database writer, one executable DSL, or one physical Rust process for every existing system.

## Three wave GOALS: documents define the destination; evidence determines the work

### WAVE 1 — WORKING COMPILER/GRAPH MICROSCOPE
**End-goal:** *We can run the actual Rust HQKG on the owner machine, populated from one freshly scoped multi-root VIVIM project; use it to see what code/knowledge/commands exist, how they link, how well they are documented, and which parts of the compiler belong natively in Rust versus in SQLite.*

**Build and test (six current lanes A–F):**
- Scoped reset of the **old indexing run only**; retain the multi-repo engine but treat selected roots as **one logical project** with versioned **indexing intent**. Preserve curated KB/SQLite records. Record archive/reset evidence.
- A: index source units, imports, symbols, annotation grammar, implementation/owner/ref evidence; B: KB, canonical/proposed terms, sources, semantic IDs and revision evidence; C: actual CLI/action vocabulary, NL parser behavior and advertised vs wired vs tested handlers.
- **Inline-comments-to-typed-IR first pass**: do not reinvent the parser/compiler. Inspect and test existing `annotation-v2.mjs`, **Rust** `research/comment-program-rust`, `comment-wiki-bridge.mjs`, current semantic registries, deterministic language Lexicon/Composer/Action Algebra; define stable IR joins and emit useful compiled descriptors for at least one verified code→semantic→command chain. Treat comments as claims, not grants.
- E: run existing Rust graph/import/journal against **actual live HQ source projections** (not research-only canned graph). Query where/what/communicates-with/operates-by/actual-vs-target and source evidence.
- D: generate the wiki **from that A+B+C+E integration**, then use missing descriptions, unbound CLI, broken links and absent provenance to grade the compiler/graph. No independent wiki source authority.
- F: run real native Cargo + HQ/Node/Bun code, restart/no-op/conflict tests, measure coverage and generate evidence-backed blind spots.
- Deliver **measured Rust/SQLite placement matrix**: each compile/query/index/KB/wiki/command component assessed for state ownership, persistence, read/write rate, latency, failure isolation, locality, deterministic replay and existing writers. Mark recommended / provisional / cannot move / insufficient evidence.
- Deliver an honest **deterministic-NLP baseline**: genuine command registry and a corpus including negation, quoted payloads, missing slots, ambiguity, typos, synonyms and multi-command clauses; record matched / refused / unsafe results. Full open-vocabulary NLP is not a Wave 1 acceptance requirement.

**Gate:** a working Rust graph and query CLI; one real cross-domain code→knowledge→command chain; generated wiki accurately reflects it; scoped reset/intent preserved; measurable quality/unknowns; Rust-vs-SQLite decision report; runnable evidence and failure transcript. If absent: BLOCKED, not a synthetic GO.

**Wave 1 output used to design Wave 2:** link quality, realistic corpus failures, semantic/type mismatches, compiler conformance, migration risk, graph/storage performance and concrete blind spots.

### WAVE 2 — STATEFUL SEMANTIC COMPILER + INTELLIGENT COMMAND/KNOWLEDGE SYSTEM
**End-goal (NOT six-lane design):** *The central compiler incrementally maintains a reliable shared semantic universe: knowledge, commands, engines, schemas, aliases, source evidence and compiled wiki remain coherent as code and user-owned knowledge evolve. Deterministic NL works over the actual registered capabilities with grounded fuzzy understanding and explicit abstention.*

**Product-facing capabilities to prove:**
- One compiler/IR contract and one semantic identity across code comments, KB, graph, action algebra, CLI, wiki and read-only client adapters. Sources and semantic history survive moves/renames/corrections with versioned provenance.
- Incremental changes propagate safely through SQLite FTS/records, Rust graph, command lookup and derived wiki. Conflict/stale/missing evidence is exposed; no second canonical writer.
- **Layered deterministic language**: normalized words/locale lexicon; curated synonyms and inflections; exact grammar/composer; type-aware entity grounding; lexical retrieval and typo/phrase fuzzy ranking; context and capability constraints; contradictions, negation, conjunction and quoted-text boundaries; refusal/disambiguation when confidence margin or authority evidence insufficient. Optional fixed local embeddings may help recall but are never execution grants.
- Working action parity: a registered command and its CLI/NL/wiki equivalents identify the same canonical operation and agree on risk, revision and required Host/native approval. No arbitrary shell/OS action from a comment or UI projection.
- Contextual intelligent wiki explains meaning, implementation, evidence, commands, source changes, unknowns and next safe actions, and proves equality against canonical IR rather than generating seductive fiction.
- Continuously refreshed actual-vs-intended graph and compiler conformance kit. Measurable retrieval/intent correctness, memory/resource budgets, safe failure and migration criteria.

**Design gates:** detailed Wave 2 six-lane plan is *created only from the real Wave 1 dossier*. Do not move into broad fuzzy NLP before source identity, stable typed IR and status/authority semantics have been verified.

### WAVE 3 — REAL-TIME USER INPUT → MACHINE INTENT → VISUAL LANGUAGE
**End-goal (PRESERVED VISION):** *The user types or manipulates a control on PC or paired phone and sees—immediately and continuously—how the machine interprets the intended operation, target, dependencies, assumptions, alternatives, consequences, permission requirements and expected result. They can correct the visual meaning before execution; all modalities produce the same canonical typed intent.*

**Product proof:**
- Each input revision produces a bounded, explainable interpretation and token/slot/entity projection. Stale asynchronous parses cannot overwrite newer text. Unknown input is visible as unknown.
- Display concise token roles, entity grounding, interpreted scope, permissions, readiness and an important negative/alternate interpretation when warranted. Keep unsupported capabilities visibly unavailable.
- Text/NL, canonical symbolic syntax, command line, wiki action, keyboard/touch/gesture and Studio share typed operation identity and semantics. Visual representation must be replaceable without changing meaning.
- Suggestions are not effects; preview is not approval; a rendered icon or `!`/`✓` symbol cannot bypass Host permissions. The exact authorized effect is executed solely by the existing Host/Studio/native broker with receipts, revocation and recovery.
- Existing TierFlow visual document/render/revision ownership and existing VIVIM PC/Android clients are reused—no competing app/renderer, second UI writer or second Host.
- User can explore and reprogram safe UI/view configuration through preview/diff/CAS/version/undo; never silently convert a wiki or text suggestion into uncontrolled execution.
- Verify on real text editing and device surfaces, compare meaning before and after visual corrections, test accessibility, offline/reconnect/revisions, negative intents, scoped privacy, responsiveness and denial.

**Preparation from earlier waves:** Wave 1 must preserve source spans, stable semantic IDs, operation descriptors, inference/refusal status and provenance. Wave 2 must produce a canonical typed IR, ranked alternatives, uncertainty/negative intent representation and candidate effect preview that Wave 3 can render without rebuilding the interpreter. *These are design constraints today; they do not authorize Wave 3 code.*

## What to challenge, not assume
1. **“One Rust core”** should mean one graph/compute semantic substrate, not forcibly relocating authoritative KB, UI and Host stores. Competing status models between old HQ `wired`, action algebra `bound/verified` and Host `sourceWired/verified` must be explicitly crosswalked.
2. **“Deterministic almost LLM-level”** is only meaningful for a bounded, registered command domain with tested coverage and abstention. It is not arbitrary open-ended human language reasoning.
3. **“Comments compile into language”** means comments emit typed declarations, descriptors, bindings and safe wiki/command metadata; they do **not** let arbitrary prose become active executable code or permissions.
4. **“Live wiki”** is an output-quality signal; generating pretty pages from index-only records does not certify B/C completeness.
5. **“Indexed”** is not **wired**, **accepted**, **tested**, **device verified** or **production ready**. Each needs evidence.
6. Archived docs remain **design inputs**; do not automatically merge them as source data or allow stale docs to replace live Host consent/state.
7. Preserve the earlier VisualSpec and negative-intent UI vision from VOMEGA; do not design a separate Wave 3 interpreter when Wave 2 is supposed to expose one shared typed IR.
8. Accept the later TierFlow Bun/SQLite UI authority decision until current measured results and an explicitly authorized migration overturn it.

## Required end-of-Wave-1 question set
- What is the smallest stable typed semantic IR that both comment/compiler and NL resolver can share **without sharing grammar**?
- What native Rust graph primitives or adapters are missing for commands, revisioned KB links, source spans and evidence?
- Which algorithms are best Rust-native pure functions, which stay in HQ JS/Bun, and which must store state in SQLite? Measure, do not guess.
- How many real source declarations resolve to canonical semantic IDs, actual handlers and generated wiki entries?
- How many operation descriptions have verified runtime receipts vs only source declarations?
- Can the generated wiki expose ambiguous or conflicting descriptions instead of silently inventing a single truth?
- Does source edit / index / Rust import / wiki generation preserve exact provenance, idempotency and incremental intent scope?
- What are the current negation/clause/quoted-payload safety regressions and the minimum safe language baseline?
- Which Wave 3 visual input fields need to be carried in the Wave 1 IR even if no UI is built now?
- Where are archived authoritative design docs in conflict with current implementations, and which accepted decision controls?
