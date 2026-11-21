# Conversation lessons — operational memory for the Planning agent
Captured 2026-11-21; **this is a user-requested engineering handoff, not ChatGPT global memory, not a running status feed.** If this file disagrees with a newer owner instruction, repository source, or an actual receipt, refresh and correct it rather than insisting on the older narrative.

## 1. Owner intent and current two-part finish line
- **Goal A:** user types into the existing, stable-address PWA composer; the AutoWiki appears and updates without pressing Send.
- **Goal B:** an *authorized* agent draft visibly types into the same composer; the user watches the changes, and the same AutoWiki reacts. It must not trigger an implicit Send, provider call, or fake receipt.
- These are Dev ChatGPT implementation objectives. This planning agent prepares the *next* Rust-first semantic/wiki layer and tests, rather than running overlapping coding work.
- Real Android rendering / user-visible repaint must be observed separately. No statement such as "phone works" from unit tests alone.

## 2. The owner corrected our execution behavior
1. ChatGPT is the source engineer in its own cloud/GitHub environment; ZCode and Desktop Commander are **not** convenient alternative coding environments.
2. For this session, Planning prepares real code and exact integration targets in Labs while the parallel Dev ChatGPT owns ARBOR application changes. Do not confuse "prep code" with "integrated feature".
3. Push prepared code/docs to `vivim-labs/main` with explicit SHA. No PRs, ephemeral branches, local source patches or workspace takeover.
4. The user wants action, not endless theoretical architecture: each continuation should close the next missing concrete gate with files and tests wherever possible.
5. Never ask the owner to restate an already established decision (PWA first, Rust fabric, host authority, stable URL, role split).

## 3. Two AutoWiki experiences coexist — audit both
- **First-contact Wiki-first PWA:** ARBOR `docs/FIRST-CUSTOMER-TRACKER.md` reports 180ms input debounce, current source-backed Rust WASM search and a read-only detail expander; a native WASM release ABI test passed in that source report, while actual Android repaint was **UNRUN** at the observed source revision.
- **General Wiki tab:** `apps/mobile-relay/app.js` has an explicit Search/Enter search control, page cards and source context. Observing *that* explicit search alone does **not** disprove the other first-contact typing path.
- Agent draft source: `core/programmability/src/agent_composer.rs`, `apps/vivim-host/control/agent-composer-bridge.mjs`, `apps/mobile-relay/first-contact-composer.ts` and existing approval/receipt checks. The latest inspected source already had a `typingIntervalMs` hook; avoid rebuilding visible typing from scratch without confirming Dev's state.
- Status statements are HEAD/time-scoped; build output, actual Host process revision, and phone screenshots are separate evidence levels.

## 4. Desired user experience is progressive disclosure, not a scrolling debug dump
- AutoWiki has a **stable main canvas/home** plus **ambient topic suggestions** that can update while a person types; do not unexpectedly replace the page the user is reading.
- Explicit selection opens a relevant page; pinning preserves user focus; breadcrumbs recover history. Overview -> Explain -> Evidence/Trace reveals more verified information only as needed.
- A "dynamic context bundle" is a bounded, permission-checked graph projection with a specific focus, scope, evidence and revision. It is **not** a second knowledge graph, unbounded agent memory, or authority to execute commands.
- Suggest topics for ambiguous draft text, but never use fuzzy matches to resolve exact command references or grant access.
- Preserve caret, IME composition, scroll, section expansion, panel visibility, accessibility and keyboard behaviors (including existing two-Tab completion and Enter).
- Display unknowns, freshness, missing sources, blocked command readiness and contradictions honestly.

## 5. Actual existing code to reuse instead of rebuilding
- `native/hqkg/src/context_retrieval.rs`: `retrieve` / `query_scoped`, authorization prefix, exclusions, relevance order, epoch, provenance, watermarks, bounds; keep its traversal.
- `core/programmability/src/wiki_read.rs`: Rust Wiki read/search/source revision and digest.
- `apps/vivim-host/chat/wiki-semantics.mjs`: canonical Wiki page/section identity and evidence classes.
- `core/programmability/src/live_system.rs`: existing live input/system/source region observations, not a new truth store.
- `core/programmability/src/context_grounding.rs`: pinned exact command references with ambiguity/stale refusal.
- `harness/src/context/dcb-composer.mjs`: source policy/selection intelligence to reuse conceptually, not copy its JS engine into a competing PWA runtime.
- `apps/mobile-relay/composer-assist.mjs`: existing interpretation, debouncing, latest-input-wins and nonexecuting suggestions.
- `core/programmability/Cargo.toml` + `native/hqkg/Cargo.toml`: core has no HQKG dependency; HQKG already optionally depends on core via `native-program`. Integrate Rust modules core-first and HQKG adapter *behind the existing feature*, preserving that direction.
- Always recheck GitHub main and current interface; this is a source map, not a warranty.

## 6. Semantic truth is distinct from elegant prose
The user explicitly wanted Wikipedia-like readable top pages, not a compiler-debug view masquerading as text. Maintain three separate expressions of one fact:
1. Canonical graph/source claims with identity, revision, provenance, uncertainty and scope.
2. Semantic representation / VLANG/command explanation, not necessarily natural language.
3. Human-readable overview/explanation, source-traceable sentence by sentence.

A historical action receipt proves an action occurred, not that its result is still true; a source annotation proves occurrence, not successful execution; an inferred relation stays a candidate until checked. A page-quality error should be routed to the **earliest responsible stage** (source evidence, semantic typing, language rendering, context selection, navigation, runtime verification), and become a reviewed task candidate—not silently rewritten VLANG code.

## 7. Existing preparation has actual files, but not integrated runtime status
Labs folder `../DESIGN/AUTOWIKI-FOUNDATION-R001/` contains:
- `src/context_lens.rs`: revision-fenced focus/pin/depth logic;
- `src/bundle_projection.rs`: graph-bundle admission and exact related links;
- `src/owned_context.rs`: bridge from owned HQKG receipt data;
- `src/narrative.rs`: verified-claim sentence templates;
- `src/quality_feedback.rs`: inert diagnostics and reviewed candidates;
- `integration/hqkg_context_bridge.rs`: feature-gated native integration adapter;
- `tests/preintegration_acceptance.rs`: human/agent projection parity and scope checks;
- `R003-EXECUTION-HANDOFF.md`: exact two-crate merge plan, dependencies and commands.

At the observed R003 commit, the package had **37 authored Rust test cases, statically counted**. These were **not compiled or executed** during that work; don't upgrade TESTED status from a static count. Check available cloud toolchain afresh rather than reusing an old "no cargo" assertion blindly.

## 8. Source-of-truth and cross-agent hazards uncovered
- The connected Labs GitHub `ZCODE-DESKTOP-COMMANDER-OPERATIONS` tree contains `DEVOPS-PLANNING`, `CHATGPT-DEV`, `ORACLE`, `DESIGN` and root policy docs. It **does not currently expose** some references in older role docs: `OWNER-WORK/`, `DESIGN/PRE-CODE/`, `code/`, `CONTEXT-MAP.md`. These may exist locally/ignored; do not invent GitHub URLs or silently treat them as empty work queues.
- **One registry:** owner OW registry is canonical when accessible. A planning bundle index is only a pointer list. Do not create fresh identifiers or a shadow backlog to paper over missing metadata.
- Parallel ChatGPT sessions have no automatic shared live memory. A GitHub commit records an artifact; it is not proof that Dev saw or adopted it. Acknowledgment must be explicit.
- Don't edit Oracle or Dev role-owned files just because they are accessible. Avoid modifying ARBOR application files assigned to another current agent.
- Confidence labels must be precise: inspected source, proposed code, compiled, tested, pushed, deployed, and phone-observed are different assertions.
- Re-pull GitHub HEAD before any actual merge; R003 sampled ARBOR `446a33766fb11a437670260d9b206f00dafedb47`, which is a historical pin, not a frozen branch.

## 9. Definition of truly ready-to-go prep
A package is `READY_FOR_DEV_REVIEW` only when it includes an OW ID (or explicitly unregistered owner directive), exact owner and intended repo, current inspected SHA, changed/source path inventory, nonoverlapping file ownership, Rust code and tests or a precise reason they cannot be authored, supported existing ABI, authorized scope/Host effects, commands to run, rollback, unexecuted gates and a recipient handoff. It is **not** `CODED_IN_ARBOR` until Dev performs that merge and confirms source/test results.

When a missing critical source or security prerequisite is identified, label it `BLOCKED_BY_DEPENDENCY` with the single specific next verification. Keep moving on independent preparatory work; do not fill gaps with fake adapters, fictitious APIs or optimistic acceptance claims.

## 10. Minimum feedback loop to future Planning selves
For each continuation: **inspect current heads -> pick one gap -> compare with Dev activity -> produce code/contract/test kit -> save to one OW-scoped place -> verify remote SHA -> update pointer/receipt -> stop before claiming deployment**. This is preferable to sprawling project restarts or reading the entire repository repeatedly.
