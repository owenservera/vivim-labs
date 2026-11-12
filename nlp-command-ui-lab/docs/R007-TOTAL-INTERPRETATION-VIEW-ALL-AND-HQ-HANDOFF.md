# R007 — Total Interpretation Composer / VIEW ALL vision

**Date:** 2026-11-12 · **Status:** expanded owner vision, **not implemented**.
**Location:** `vivim-labs/nlp-command-ui-lab/` research. Product interpretation and execution stay with the existing owners; this document grants no new authority.

## One-sentence north star

**A VIVIM text box should be able to show, inside and behind the actual words, *every interpretation that VIVIM's participating systems actually emit or consider*, with circles, rings, fibers, relationship wires, graph nodes, DCB families, memory-write proposals, evidence, effects, uncertainty, and a record of what was not evaluated.**

The user-facing chat box is the *reduced view* of the exact same trace, with only useful cues left visible.

The objective is **not** simply icon-based autocomplete or a command visualizer. The composer is a *multi-layer spatial projection of live semantic computation*. It may reveal semantic relationships not originally evident in the prose, yet it must not invent them.

## Source-backed HQ workstream

- **[HQ full interpretation surface and gap audit](https://github.com/owenservera/repos-hq/blob/research/interpretation-xray-intelligence-surface-20261112/research/semantic-interpretation-surface/HQ-SURFACE-AND-GAP-AUDIT.md):** 43 interpretation facets, current code anchors, 22 gaps, proposed shared read-only interpretation envelope and phased integration gates.
- **[HQ observed intelligence producer census](https://github.com/owenservera/repos-hq/blob/research/interpretation-xray-intelligence-surface-20261112/research/semantic-interpretation-surface/source-census.v0.json):** 31 source-observed module/contract entries across HQ, Host and Labs. **Not exhaustive; not a runtime manifest.**
- Prior Labs work: [R004 visual grammar](./100-SIGNAL-DENSE-COMPOSER-INNOVATIONS-R004.md), [R005 progressive UI sprint](./R005-PROGRESSIVE-SPRINT.md), [R006 predictive two-Tab intelligence](./R006-100-PREDICTIVE-REALTIME-COMPOSER-CAPABILITIES.md).

## ERA 1 — Word-level semantic layers, with no mandatory interpretation winner

A visual input should carry a source-anchored **overlapping semantic hypergraph**. No word belongs to just one box. A phrase can represent an assertion, a scope, an entity reference, a future action, a memory candidate, a DCB flavor, a DCB definition candidate, a predicted follow-up and an evidence link *at the same time*.

Example:

> **"Remember that I prefer rigorous source-backed updates for my projects. Connect that preference to my research and project context bundles, but do not send it to an AI yet."**

The same text supports multiple *independently sourced* projected layers:

| Source span | Interpretation node | Possible visual treatment | Truth or authority boundary |
| --- | --- | --- | --- |
| `Remember` | `memory.write_proposal` | open orange ring / write-mutation symbol | Proposed operation, **not a permanent-memory receipt** |
| `I prefer rigorous source-backed updates` | user preference assertion; possible memory content | long soft underline with source dot | Candidate claim, subject/retention must be resolved |
| `my projects` | project scope candidates | entity halo + explicit identity selector | Could be plural and unknown, never silently pick one |
| `that preference` | coreference edge back to preference clause | curved thread behind text | Deterministic or candidate relation with rationale |
| `research` | Host DCB flavor `research` | pale blue linked oval | Real flavor ID; **not** an actual DCB definition |
| `project context bundles` | Host DCB flavor `project`; zero or more known DCB definition refs | second oval + fanout edges | Definitions only when Host provides real owner-authorized IDs |
| `Connect ... to` | proposal to create relationship(s) | multi-target branch line | Not a graph write; must include affected target IDs |
| `do not send` | scoped negation of provider disclosure | clear stop bracket over exact clause | No browser/provider effect can be inferred |
| `yet` | temporal constraint to the negated effect | tiny temporal tick | Ambiguity about future action does not imply schedule |

The visual grammar is not aesthetic: **solid vs dashed, one line vs branch, ring vs bracket, and direct vs broken connector each express typed epistemic meaning**.

### Semantic layer stack behind one edit surface

1. **L0 unmodified text:** editable natural language with preserved keystrokes, selection, IME and undo; no DOM text replacement as an unsafe shortcut.
2. **L1 language anatomy:** tokens, clause spans, quotations, negation, temporal operators, user-defined aliases.
3. **L2 canonical identity:** recognized entities, references, relations, verified source identities, ambiguity and missing IDs.
4. **L3 knowledge:** evidence, alternative claims, contradictions, relevance and graph-neighbor links.
5. **L4 memory:** candidate persistent writes, revisions, deletes/forgetting, consent scope and expected receipts.
6. **L5 DCB:** flavors, actual definitions, family memberships, parent/composite links, source-selection, lifecycle, destination and disclosure.
7. **L6 operations:** inert commands, dependencies, potential effects, approval needs, Host authorization state and actual receipts if they exist.
8. **L7 prediction:** ghost completions, probable next slot, cached candidate contexts, learned preference explanations.
9. **L8 epistemic truth:** provenance, which producer actually ran, which was skipped/denied/stale, source revisions, uncertainties.
10. **L9 presentation mask:** quiet, assisted, VIEW ALL, debugging, accessibility—**all from the same signed/scope-limited trace revision**.

### Critical distinction

`memory.write_proposal` is NOT `memory.persisted`; `dcb.flavor_candidate` is NOT `dcb.definition_ref`; `knowledge_relevant` is NOT `disclosure_authorized`; `intent_candidate` is NOT `executable_action`; `source_declared` is NOT `runtime_verified`.

## ERA 2 — How the VIEW ALL visual field behaves

A VIEW ALL mode is a **layered annotation field inside the chat box**, not a separate mind-map editor.

- **Circles** wrap grounded or candidate mentions. Nested circles signal multi-layer interpretations; tiny corner numerals indicate hidden additional interpretations at the same span.
- **Open and filled rings** distinguish hypotheses from confirmed source-bound identities; use redundant labels for accessibility.
- **Brackets** scope negation, conditions and quoted literals; a user can inspect which words a restriction actually controls.
- **Wires** run behind/around text from a clause to its dependents; curved connectors represent coreference and cross-clause references, fanouts/hyperedges represent one phrase bound to multiple DCBs or tasks.
- **Line textures** distinguish asserted relations, inferred/conditional associations, provenance links, invalidated results and missing/denied evidence.
- **Color** describes domain or status with redundant type/shape; it never single-handedly implies a permission or memory commit.
- **Micro-nodes** float near annotated spans for DCB families, actual DCB definitions, memory targets, source references, competing interpretations and possible effects.
- **Focus behavior** expands only the annotation graph for the currently selected word or phrase. VIEW ALL still has a full density view for developers and audit/replay.
- **Layer toggling** can hide edges but not destroy interpretation data; re-enabling returns to the same input revision and evidence graph.
- **Prediction** may preview likely future nodes and wires as translucent *ghosts* while the user types; they remain visibly conditional until selected/grounded.
- **Geometry priority** avoids obscuring legibility: text foreground, thin wires behind glyphs, rings around line rects, intersections bundled into tracks, overflow count indicator. On a narrow phone, line bundles collapse into focusable relation badges rather than forming a spaghetti tangle.
- **Accessibility** provides an equivalent structured reading: phrase -> meaning -> relation -> source -> status; keyboard-only navigation and screen-reader labels must cover all authorized nodes/edges.

**VIEW ALL reveals all authorized data emitted by registered and actually consulted producers—not uncomputed mind reading or unpermitted private history.** A producer that did not run must be shown as not-run, not silently treated as no meaning.

## ERA 3 — HQ asks and product graduation

### Required HQ capability: inspect everything *that actually exists*

HQ should furnish a source-derived and runtime-checked **Intelligence Surface Manifest** across the canonical four repos. It must enumerate each participating semantic producer's canonical ID, input, output types, owned code, schema, read/effect status, privilege scope, source provenance and observed wiring. Importantly, it must report **unobserved/unregistered areas**, not claim coverage from a static file list.

HQ and Host then project utterance-level source spans, nodes, hyperedges, producer-run coverage and factual support into the proposed **read-only interpretation trace**. Rust may perform graph joins and ranking. TierFlow only renders; the composer never becomes a second authority.

**Non-negotiable first integration test:** the example sentence above produces **one memory-write candidate**, **two DCB flavor candidates**, **zero or more real DCB definitions**, a multi-target proposed relationship, a correctly scoped negated send, provenance for each interpretation, and an accurate producer coverage report—**without writing, storing private facts in public Labs, or sending anything**.

### Five sequential proof slices

| Slice | Requirement | Acceptance evidence |
| --- | --- | --- |
| A | HQ producer census/manifest with complete source-vs-runtime truth | All known producers accounted for; unknown/denied clearly visible |
| B | One sentence → overlapping spans, multiple node types and hyperedges | Deterministic tests including multiline/quotes/Unicode/negation |
| C | Memory+DCB reference traversal via authorized Host ports | Real IDs or explicit unavailable; candidate writes never executed |
| D | In-composer VIEW ALL drawing and Quiet projection from identical trace | Visual device/IME/accessibility tests, source/identity mapping |
| E | Prediction/prewarm over same trace, not a second brain | 2-Tab tests, stale-source cancellation, proof of no unauthorized effects |

### Unresolved and important

- The HQ 31-producer census is a source-observed **seed**; a full machine-generated inventory remains work to perform.
- No mounted Windows HQ index or real authorized Host personal archive was inspected in this repository audit.
- Actual DCB lifecycle contracts exist, but pure code is not proof of a mounted definition store or committed user bundle.
- HQ notes are not automatically the permanent personal-memory product: ownership and privacy boundaries must remain distinct.
- No producer currently examined exposes a proven universal text-span→memory→DCB→effect interpretation trace.
- This is not a production merge or tested multi-layer editor. The research has been documented for source-grounded implementation.

**Owner-level acceptance principle:** first achieve an accountable VIEW ALL surface for all participating interpretations; **only then optimize away everything a normal user does not need to see.**
