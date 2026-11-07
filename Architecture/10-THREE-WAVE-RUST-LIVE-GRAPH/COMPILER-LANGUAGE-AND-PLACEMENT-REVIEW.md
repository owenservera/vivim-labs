# HOW Review — Compiled Inline Comments, Deterministic Language, Rust Graph and SQLite

Date: 2026-11-07
Status: master high-effort architecture review. Working hypothesis + measurable Wave 1 decision gates, NOT a claim of full implementation.
Uses design goals from [MASTER-DOCUMENT-LED-WAVE-GOALS.md](MASTER-DOCUMENT-LED-WAVE-GOALS.md)

## 1. The mistaken architecture to avoid
Do not build:
- a code crawler that saves tables, followed by an unrelated knowledge graph, followed by a chat parser, followed by a wiki editor;
- a second command registry in Rust and a separate set of “wiki commands” in JS;
- one universal parser for both static code annotations and live human utterances;
- automatic semantic facts, execution permissions or verified statuses from documentation or comments;
- a Rust rewrite of all existing SQLite writers before native behavior and state migration are measured.

Instead, treat indexing as an **incremental semantic compilation pipeline** that emits reproducible *typed, versioned descriptions* and materializes them into specialized runtimes.

```text
PROJECT + INDEXING INTENT (one VIVIM project, N repos/folders)
  │
  ├── CODE: sources, symbols, imports, comments, signatures, tests, commits
  ├── KB: decisions, terms, claims, semantics, revisions, context, evidence
  ├── OPERATIONS: registered actions, CLI/MCP/Host/Studio handlers, risk, receipts
  └── DESIGN: historical + active docs used to define TARGETS (not presumed facts)
         ↓
SOURCE-AWARE FRONTEND
  scanner, parser(s), dedup, source hash, symbol/line binding, schema validation
         ↓
SEMANTIC LINKER
  stable IDs, aliases, source→KB and source→action crosswalk,
  authority/status provenance, version conflict, unknowns, stale references
         ↓
CANONICAL SEMANTIC IR (data-only, deterministic, typed, provenance-bearing)
         ├── SQLite: durable HQ KB/semantic revisions, FTS/search, journalled records
         ├── RUST HQKG: graph/engine/data/command/dependency topology + computations
         ├── command grammar/catalogue: exact+fuzzy candidate lookup by stable IDs
         ├── CLI/MCP: typed command discovery/introspection with execution owner refs
         └── GENERATED WIKI: read-only, meaningful output of A+B+C+Rust join
                       ↓
              LATER: LIVE NL/typed intent projection
              → visual token/slot/risk/alternate view
              → owner preview/approval → Host or native effect writer → receipts
```

The **semantic IR** is the key. Its definitions and acceptance fixtures must not depend on Bun, Rust, SQLite, TierFlow, Windows, or one provider's API.

## 2. Inline comments become a compiled, source-checked declaration language
**Do not start by inventing a new language.** Repos HQ already owns:
- `hq/lib/extract.mjs`: source-language aware symbols/imports/v1 inline comments.
- `hq/lib/annotation-v2.mjs`: pure typed comments `@hq:type, capability, effect, evidence, contract, emits, consumes, action, test`; text in strings is not treated as a declaration.
- `research/comment-program-rust/src/lib.rs`: real pure Rust comment-only wiki-program compiler, with `@hq:wiki`, closed `@hq:source/filter/layout/select/bind/requires/evidence` grammar, source map, SHA-256 digest and inert bindings.
- `hq/lib/wiki-runtime/rust-comment-adapter.mjs`: read-only trusted source-byte/digest verification and JS wiki validator path.
- `hq/lib/comment-wiki-bridge.mjs`: source-bound wiki gesture/NL-to-typed-command-intent mapping, no effect dispatch.
- `hq/lib/semantic-registry.mjs` and `wiki-program.mjs`: source-backed IDs and owner-confirmed, versioned configuration operations.

**Architectural challenge:** the Rust comment compiler currently targets **bounded wiki programs**, not a general all-capabilities semantic IR. The v2 annotation parser targets declarations, not commands that can run. We should **link these complementary outputs** through accepted semantic IDs and provenance, not replace them with an enormous compiler rewrite in Wave 1.

### Recommended compilation phases
1. **Scan** a verified language/file in the selected logical project. Ignore strings, generated/vendor files and known secret paths; record source SHA, original path, symbol/line/span.
2. **Parse** separate grammar families: legacy @hq v1, semantic @hq v2, and `@hq:wiki` program grammar; record unsupported tags instead of pretending they compiled.
3. **Link** compiler-declared capability/contract/action refs against real semantic registry IDs and actual handler catalogs. A non-registered/ambiguous name must not be assigned by guess.
4. **Typecheck** exact parameter schemas, effect categories, read/write authority, scope requirements, dependencies, and source/test evidence. Annotated values are *claims*; recognized binding still needs independent verification.
5. **Lower** into one inert versioned IR manifest: project/root/source IDs, canonical semantic node/edge IDs, command descriptors, source maps, dataflow/effect declarations, evidence status, wiki view descriptors, diagnostics and canonical digest.
6. **Materialize** source-owned projections: HQ SQLite FTS/KB views, Rust HQKGX1 graph import, command discovery, wiki generation. These are outputs of compilation, not independent semantic writers.
7. **Compare** to expected architecture: unbound capability, unknown implementation, duplicate name, stale contract, absent test, missing CLI handler, missing wiki meaning and unproven runtime link.
8. **Incrementally recompile** affected source roots/nodes only; prove no-op, deletions/renames, revision conflicts and graph/writer restart. Do not delete unrelated entities when only part of a source scan is available.

### Minimal illustrative authoring (NOT active grammar extension)
```ts
/**
 * @hq:context Show the current compiler and graph health without effects.
 * @hq:capability vivim.graph.health@1
 * @hq:action vivim.graph.health@1
 * @hq:effect read-only
 * @hq:evidence test:tests/graph-health.test.mjs
 * @hq:see contract:vivim.graph.health@1
 */
export function graphHealth() { /* existing implementation */ }
```
An actual build must use precisely **supported** parser syntax and registered values, not assume this hypothetical sample compiles as written. The compiler can propose an action descriptor and a documentation page; it cannot manufacture a handler, approve a scope or assert that the test ran.

## 3. How deterministic NL can feel almost LLM-level for a bounded domain
**Truth boundary:** broad human language is open-ended; a deterministic language engine cannot promise general LLM reasoning. It **can** attain very high recognition/clarification quality for the registered VIVIM command universe, with reproducible explanations, given sufficient language data and grounded slots.

**Reuse before writing**:
- `vivim-omega/plugins/vivim-nlcl-pure`: pure `interpret(input, world)`, token spans, frame matching, entity grounding, multiple candidates, suggestion and visual effect projection.
- Harness `control-language.mjs` + `intent-planner.mjs`: actual command candidate/Host plan; previous audit reported unsafe negation/clause handling.
- Repos HQ `research/deterministic-language/lexicon`: versioned explicit lexicon and locale-aware exact normalization (not fuzzy by itself).
- Repos HQ `composer`: exact typed slots, sequence/conjunction/negation AST, bounded canonical form (no intent search).
- Repos HQ `actions`: descriptive action algebra and bounded typed schema/compatibility (not an executor).
- Repos HQ `semantic-optimizers.mjs`: pure alias, evidence rank, context packing, impact/drift; existing HQ `action-router.mjs` is deliberately a basic deterministic scorer.
- Historic `vivim-final/src/engines/nlcl/`: fuzzy matcher (Levenshtein/Jaro-Winkler/Dice), layered resolver, TF-IDF and dense embedding fallback; **candidate algorithm sources**, not a permission or runtime port mandate.

### Recommended deterministic resolution tiers (pure candidate generation only)
| Tier | Input/transform | Example | Proof/limits |
|---|---|---|---|
| T0 | Structural safety scanner: negation, quote boundaries, conjunction, destination, scope, locale | “don't disconnect the PC” | Negation never silently dropped; no clause hidden |
| T1 | Exact semantic command/frame/alias match | “show graph health” | Full typed slots/identity; exact rules |
| T2 | Versioned lexicon: forms, locale, synonyms, abbreviations | “show me dependency map” | Sense collisions returned, not guessed |
| T3 | Typo and phrase similarity over **registered** candidate descriptors | “shwo dependncy map” | Jaro-Winkler/Dice/edit distance with hard candidate bounds |
| T4 | Sparse semantic retrieval: SQLite FTS5/BM25, TF-IDF, n-grams, weighted field lookup | “what might break if I change this?” | Reproducible ranked candidates, no invented handlers |
| T5 | Optional pinned local dense-vector retrieval for paraphrases (fixed model/version/numeric profile) | “trace consequences of edits” | Retrieval-only; explicit reproducibility/cost test; may be approximate cross-hardware |
| T6 | Typed constraints + graph context: selected project/root, known entity, command risk, required slots, capability readiness | “on this repo” | Re-rank and reject based on verified scope; no privileges inferred |
| T7 | Calibrated candidate response: resolve, partial, ambiguous, unknown, unsupported, forbidden | competing “remove” intents | Return alternatives/reason/required confirmation; no silent winner |
| T8 | Canonical typed intent/plan + effect preview; same shape for CLI, wiki, UI and NL | `{actionId,args,worldRevision}` | Still an inert candidate; Host/native executor remains sole effect authority |

**Why this can be strong:** source annotations and command contracts continuously produce a small, structured, type-grounded search space. A registered command carries roles, aliases, type restrictions, examples, explicit effects and a live graph neighborhood. Resolving within a closed, known environment is easier and safer than open-world question answering.

**Hard constraints:**
- **Intent must not be inferred from negation as if negation were a standalone verb**; negation is an AST operator or a refusal/scope constraint, depending on grammar.
- Quoted text and provider payloads are data, not nested local commands. Sentence splitting must preserve all clauses or refuse to compose.
- Never discard a conflicting clause to pick a “safe looking” remaining command.
- Confidence numbers from match heuristics are not automatically calibrated probabilities. Measure rejection quality and score margins against held-out labelled commands.
- High-impact effects demand stronger acceptance and explicit owner authorization; a broad search match can never turn a declared action into verified.
- User-taught synonyms/paraphrases belong in versioned, scoped, auditable lexicon records with a rollback path, not dynamically rewritten parser source.
- A local embedding model, if used, is a read-only retrieval provider, not the canonical semantic registry. Pin model, tokenizer, platform and numeric tolerance; refuse unversioned runtime swapping.
- Optional LLM assistance (via owner-authorized signed-in webapps) can only propose candidates for the same typed validator, not execute unvalidated or outside-graph effects. No provider APIs assumed.

### Wave 1 must measure the **starting** language truth
Run a small but broad corpus for exact, ambiguity, typo, paraphrase, negation, combined clauses, quotation, missing param, unsupported action and wrong target. Include English + Spanish where explicitly registered; absence of a locale is an explicit gap, not “deterministic multilingual readiness.” Compare actual Host/CLI/NL and proposed tier outputs; *do not* require full T0–T8 implementation in Wave 1.

## 4. Native Rust vs SQLite vs existing owners — proposed responsibility map
**Major architectural distinction:** “Rust core” names the reusable graph/compute engine, while “SQLite DB” names a durable data engine; it is usually a false binary to say everything must be natively owned by one or the other. Rust can call SQLite through an adapter **without owning a second canonical truth**.

| Capability / data | Candidate native implementation | Durable owner now | Wave 1 decision |
|---|---|---|---|
| Source/file/symbol/import index, FTS5 | HQ Bun/Node indexer + SQLite SQL/FTS | Dedicated selected HQ index DB | KEEP; project/run/root provenance; benchmark Rust query projection need |
| Annotated comment scanning / grammar | Existing HQ pure JS v2 scanner + Rust comment compiler | Source code is authority; compiled records are derived | ADAPT, compare native Rust vs JS conformance, no wholesale rewrite |
| Semantic ID/types/aliases and curation | Registry validators, Rust graph projection | **HQ durable kernel SQLite** / explicit semantic owner, verify canonical writer per type | KEEP SQLite writer; Rust read/projection; no automatic promotions |
| Trusted KB claims, decisions, corrections, privacy/retention | HQ KB + SQLite transactions/event ledger | HQ KB/kernel or Host as data-domain owner | KEEP source-native writer; Rust index/knowledge compute only |
| CLI/NL/action catalog and declarative descriptors | Shared typed IR, deterministic resolvers/adapters | Registered action/contract owner, Host effect owner | DUAL: Rust can compute discovery; catalog identity canonical in HQ |
| Engine slots, generations, graph link topology, computation | Existing Rust HQKG Graph/EngineRegistry/scheduler/journal | Rust graph runtime for its own state; source-native system for facts | USE Rust NOW; no duplicate engine registry |
| Index+KB+command cross-domain edges | Rust graph link/impact/traversal; source evidence tables in SQLite | Raw evidence remains source-owned; Rust owns materialized graph revision | USE Rust NOW; test traversal/provenance |
| Search query and fuzzy retrieval | SQLite FTS5/BM25 + pure ranked resolver; optional Rust scorer | Search indexes are derived | BENCH to choose hot path; avoid duplicate FTS engine |
| Wiki pages and safe programs | Existing HQ wiki generator/program validator | Pages derived; configuration revisions via existing HQ writer | GENERATE from IR; do not make wiki a KB authority |
| Visual document, drafts, publication, UI revision | TierFlow existing TypeScript/Bun + SQLite | TierFlow UI source-native writer | PRESERVE; no Rust/UI state takeover |
| Browser/OS/phone effects, consent, receipts | VIVIM Host/Studio/Android native adapters | Host/Studio/platform owners | NEVER graph-owned; graph references verified receipts |

### Decisive experiment (Wave 1)
For each tested vertical slice record:
- source count/bytes and SQLite sizes;
- initial scan and incremental update times;
- compiled IR size/digest and native JS/Rust conformance;
- Rust import time, memory, snapshot size, no-op behavior and query p50/p95;
- SQLite FTS query p50/p95 and source/term selectivity;
- graph traversal/predicate richness and cost of rehydrating after restart;
- failure atomicity, provenance, stale detection, duplicate identities and migration risks;
- whether the operation *creates unique durable meaning* (belongs to a canonical writer) or only computes a projection (may move freely);
- whether a Rust-native implementation actually reduces overhead/latency enough to justify porting.

Recommendations must be assigned one of `RUST_NATIVE_NOW`, `RUST_ADAPTER`, `SQLITE_CANONICAL`, `SQLITE_DERIVED`, `KEEP_EXTERNAL_OWNER`, `DEFER_MEASUREMENT`. Include evidence pointer or explicit insufficiency.

## 5. Critical Wave 1 source-contract blind spots
- Multiple IDs: HQ `name@version`, Action Algebra `namespace::id@semver`, `hqg1:<digest>`, Host action ID, comments' @hq:wiki ID, and TierFlow document nodes. An **explicit identity crosswalk** is necessary; don't collapse them by string similarity.
- Semantic status drift: `proposed/current/deprecated`, `declared/wired`, `bound/verified`, Host `source_wired`, runtime `accepted/rejected`, device `tested`. Use an evidence-state vector rather than one misleading ordinal “completed”.
- Call graphs based on imports are **static**; CDP/browser calls, bus events, IPC, WebSocket, plugins and reflective invocations need different evidence classes. Unknown isn't absent.
- Source comments can be stale, misleading, or malicious. Compiler validation and registry lookup cannot turn an annotation into trusted runtime proof.
- A project index scope is not the same as one source repository. Old-run data must not leak, while archived design goals remain available to the wave designer.
- Rust HQKGX1 importer accepts data-only records and source ownership; it is not a general permission granting plugin loader or live synchronization layer.
- Rust comment compiler's default registry only binds **active read-only** actions; broad effects and generic command compilation remain gaps to design safely.
- A generated wiki can be perfectly deterministic while semantically incomplete. Grade coverage and link correctness against verified A+B+C, not byte stability alone.
- Older Host intent audit documented negation/clause-loss risks; **test before enabling any natural-language executable action**.
- TierFlow's early pure-Rust state blueprint was explicitly displaced by its later accepted Bun/SQLite evolution strategy; avoid designing Rust as a surprise UI state authority.

## 6. Preserve Wave 3's real-time visual input contract NOW
Wave 1's typed IR and graph must at minimum carry or derive: `commandSemanticId`, `sourceSpan(s)`, `selectedEntityId`, `candidateIntent`, `typedSlots`, `missingSlots`, `negation/quote/clauseStructure`, `alternatives`, `status`, `risk`, `requiredScopes`, `sourceEvidence`, `worldRevision`, `compilerVersion`, `inputRevision`. This is a **design checklist**; don't force all fields onto static code declarations or freeze an over-broad schema prematurely.

In Wave 3, the pure interpreter produces a **visual projection**, not a second UI command grammar:
```text
key stroke / visual choice → immutable input revision
  → pure incremental interpretation of bounded text and scoped world snapshot
  → candidate typed IR + token roles + grounded objects + alternatives + risk
  → replaceable icon/underline/card projection (desktop and phone)
  → user correction rewrites proposed intent; no effect yet
  → typed plan pinned to source+world revision, owner preview/consent
  → native Host or TierFlow UI configuration owner commits
  → verified receipt and UI feedback, preserving partial/uncertain outcomes
```
Input/caret revision cancellation must prevent old parse updates from replacing new output. An inconclusive result must show uncertainty and one useful disambiguation rather than pretend understanding. Quoted payloads, negation, unavailable accounts, cross-provider sends, partial failures and undo-vs-effect-reversal are essential user scenarios.

The **VOMEGA Command Visual Language** and `vivim-nlcl-pure/project.ts` already explain token roles and effect preview; TierFlow's existing renderer and UI revision contracts are the presentation owner. Design UI adapters around those, not a new bespoke compiler or second visual document writer.

## 7. Required Wave 1 proof artifacts and design decision
At minimum:
1. `semantic-ir.sample.json`: actual source-pinned IR from real A+B+C compiler/linker, invalid/collision/old revision diagnostics.
2. `compile-conformance.json`: comment parser + Rust compiler + JS wiki validator fixtures, actual pass/fail, source hashes, unrecognized declarations.
3. `intent-safety-baseline.json`: real NLP command corpus with negation/quotes/conjunction/typo/ambiguity, exact observed outcomes and whether any unsafe executable plan was formed.
4. `rust-sqlite-placement.json` and `.md`: per-component owner, state, recommendation, native performance tests and blockers.
5. `graph-vs-target.json` and wiki output: verified links/code/KB/commands + missing documentation/errors; no fourth wiki owner.
6. `WAVE-2-INPUT-DOSSIER.md`: derives next wave plan from compiler, graph and language defects; includes preserved Wave 3 visual-language interface obligations.

**No Wave 2 detailed implementation assignment until these have actually run.**
