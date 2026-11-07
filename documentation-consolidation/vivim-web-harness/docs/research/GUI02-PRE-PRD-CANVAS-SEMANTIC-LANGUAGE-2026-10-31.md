# GUI02 — Pre-PRD research: Infinite Canvas, Visual Command Language, and Generative Bootstrap

**Date:** 2026-10-31. **Status:** RESEARCH, not an approved GUI02 design, runtime implementation or product milestone change. **Scope:** owner discussion recovery (Oct 29–31), verified Git sources in VIVIM/VOMEGA, contemporary public projects and HCI literature. **Relevant tasks:** `GUI02`, `GUI01`, `CANVAS01`, `VOMEGA01`, `BF01`, `BF03`, `DCB02`. **Main product contract:** [PRD 11](../../dev/prd/11-realtime-generative-ui-bootstrap-first.md). **Authoritative task statuses:** [roadmap](../roadmap.json).

## ERA 1 — Recovered owner intent versus still-exploratory ideas

### Owner-directed requirements recovered from recent ChatGPT work
- **2026-10-30, personal app Build:** paired phone with verified signed-in AI webapp; a nearly blank canvas and one prompt/composer; user builds their own app, not a preset dashboard. AI proposes interactive features; user can preview, activate, change, not now; Auto All-in-One Chat, different modes, time travel; **Talk and pairing stay intact**, Build is additive.
- **2026-10-30, schema/VAWS:** a small stable universal kernel, with user-specific domain types that evolve through genuinely evidenced signed-in webapp interactions; schema evolution is an owner-authorized declarative proposal, not arbitrary generated SQL or execution.
- **2026-10-30, mobile interactions:** hand/touch/gestures, long-press, drag/grouping, history scrub, pinch/zoom on canvas, semantic intent mapping, accessible visible tap equivalent, no gesture as effect authorization. See [interaction lab](../ux/MOBILE-INTERACTION-LAB-2026-10-30.md) and [28-binding schema catalog](../ux/MOBILE-INTERACTION-SCHEMA-MAPPING-2026-10-30.md).
- **2026-10-31, single-file HTML discovery:** user requested at least a broader top-30 source survey, specifically expanding to **infinite-canvas single HTML projects**. Earlier conversational repository lists had unverified freshness/identity; single-file HTML and infinite canvas were **exploration targets, not approved runtime mandates**.
- **2026-10-31, GUI product:** the first V1 Bootstrap MUST use the **same safe shared generative UI surface** as subsequent Build. F02/F01/D03–D07 are existing typed task protocols; none need a new model send merely to get rendered. Browser ownership, local authority, provenance and versioning stay intact.

### Older owner-directed visual language (now source verified)
VOMEGA [Command Visual Language](https://github.com/owenservera/VOMEGA/blob/main/seed-docs/COMMAND-VISUAL-LANGUAGE-DESIGN.md) dated 2026-10-05 explicitly records a demand for real-time interpretation feedback, meaning-bearing icon handles (Windows PC vs AI/account/device), editable visual consequences, and customizable icon libraries. Its proposed syntax marks, colors and exact UI treatments are **candidate experiments**; neither visual syntax nor the 3D manifold was accepted as mandatory shipping technology. [Core Visual Language](https://github.com/owenservera/VOMEGA/blob/main/elephant-greenfield-omega/03_DEVELOPMENT_SYSTEMS/elephant-protocol-gym/CORE_VISUAL_LANGUAGE.md) calls for **machine ↔ typed visual projection ↔ human perception** parity, explicitly lab/design only.

**Evidence limitation:** prior-conversation retrieval recovered dated user directives and assistant summaries, not a complete independently auditable export of every conversation. A previous “top 30” repository claim does not establish 30 recently created, checked projects. The historical “67 meta-programs” count was not independently established in the inspected VOMEGA tree: `.project/pm/data/programs` currently exposes five `MP-*.json` files in the inspected commit. This does not prove the broader inventory never existed.

## ERA 2 — Source archaeology: real contracts worth preserving

| Source / actual file | Verified property | Reuse candidate, not automatic port |
|---|---|---|
| [vivim-final InfiniteCanvas.tsx](https://github.com/owenservera/vivim-final/blob/master/frontend/src/components/canvas/InfiniteCanvas.tsx) and [canvas types](https://github.com/owenservera/vivim-final/blob/master/frontend/src/canvas/types.ts) | Nodes, edges, positioning, viewport, groups, bookmarks, selection, pan/zoom, command mode and a `vivim.canvas/v1` shape | Stable object IDs and spatial projection can coexist with mobile card/list layouts |
| [vivim-final LivingCanvas.tsx](https://github.com/owenservera/vivim-final/blob/master/frontend/src/components/canvas/LivingCanvas.tsx) and [layout-intent.ts](https://github.com/owenservera/vivim-final/blob/master/frontend/src/shared/layout-intent.ts) | Viewport query/culling; clustering, timeline, mindmap, kanban, grid, free layouts, pinned nodes, agent-command seam | Project **layout intent and relationships**, not hard-coded AI-generated absolute coordinates |
| [canvas export](https://github.com/owenservera/vivim-final/blob/master/frontend/src/canvas/export.ts) and [TimeMachinePanel](https://github.com/owenservera/vivim-final/blob/master/frontend/src/components/canvas/TimeMachinePanel.tsx) | Re-importable JSON, SVG/PDF/PNG export concepts; historic versions, diff and restore UI | Portable user-owned designs and inspectable history; do not equate remote effect rollback with UI undo |
| [mutation-schema.ts](https://github.com/owenservera/vivim-final/blob/master/src/reprogrammability/mutation-schema.ts) / [MutationDiffPanel](https://github.com/owenservera/vivim-final/blob/master/frontend/src/components/canvas/MutationDiffPanel.tsx) | **Eight** mutation op variants: replace, insert, remove, reorder, restyle, rebind, set_property, set_slot, with provenance and idempotency | Compare to PRD11's five currently proposed ops **before GUI01 schema is frozen**; do not port historic authority/HTTP stack |
| [VOMEGA NLCL symbols.ts](https://github.com/owenservera/VOMEGA/blob/main/omega-baseline/plugins/vivim-nlcl-pure/src/symbols.ts) and [types.ts](https://github.com/owenservera/VOMEGA/blob/main/omega-baseline/plugins/vivim-nlcl-pure/src/types.ts) | **17 lexer families implemented**; mixed natural language and glyphs; versioned `WorldModel`, token spans, IR slots, alternatives, gaps and effect previews | Future optional semantic expression format, **not** a GUI02 bootstrap parser replacement |
| [NLCL interpret.ts](https://github.com/owenservera/VOMEGA/blob/main/omega-baseline/plugins/vivim-nlcl-pure/src/interpret.ts), [frames.ts](https://github.com/owenservera/VOMEGA/blob/main/omega-baseline/plugins/vivim-nlcl-pure/src/frames.ts), [project.ts](https://github.com/owenservera/VOMEGA/blob/main/omega-baseline/plugins/vivim-nlcl-pure/src/project.ts) | Pure interpretation design: scan → lex → modifiers → recognizers/frame match → entity grounding → candidates → token/effect projection; learned lexicon data, not grammar mutation | Shared **semantic-intent reference and explainability** between command phrases, touch handles and view proposals |
| [Command Visual Language](https://github.com/owenservera/VOMEGA/blob/main/seed-docs/COMMAND-VISUAL-LANGUAGE-DESIGN.md) and [Negative Intent Frontier](https://github.com/owenservera/VOMEGA/blob/main/seed-docs/NEGATIVE-INTENT-SIGNAL-ENGINE.md) | Interpretation, target grounding, readiness, authority, consequence, execution, observed evidence are separate dimensions; earliest valuable counterfactual exposed before expensive wrong action | GUI02 can preserve source spans, assumptions, alternative interpretation, accepted/rejected evidence and risks **without** making a second interpreter |
| [Core Visual Language](https://github.com/owenservera/VOMEGA/blob/main/elephant-greenfield-omega/03_DEVELOPMENT_SYSTEMS/elephant-protocol-gym/CORE_VISUAL_LANGUAGE.md) | Canonical machine meaning → typed visual spec → actual human rendering with parity proofs; user can reprogram visual appearance but not silently change meaning | Same semantic object can render card, node, timeline or glyph, while provenance and effect rules remain stable |
| [current VAWS evolution protocol](../architecture/vaws/EVOLUTION-PROTOCOL.md) and [current mobile interaction schema](../ux/schema/mobile-interaction.schema.json) | Schema proposals and semantic gesture intents are typed; runtime grants and risk are separate authority | GUI02's projected cards must not grant capabilities or add sensitive personal types by themselves |

### Exact existing 17 glyph families

`/` command, `@` grounded reference, `#` tag, `!` force (historical name), `?` query, `+` add, `-` remove, `&` combine/chain, `$` bind value, `*` wildcard, `%` configure, `∆` change, `✓` confirm marker, `=` assert/teach, `^` priority, `~` approximate, `→` direction. These are *lexer primitives*, not proven complete execution semantics. **Neither `!` nor `✓` bypasses required Host permission/consent, and neither a checkmark nor confidence score proves verified success.**

Illustrative semantic intent (not a new accepted DSL):

```text
"Make this project panel a timeline"
  = selected object @project-panel + ∆ layout → timeline
  = visual command: choose Timeline on selected panel
  = validated intent → inspect scope/capability/approval → local UI revision
```

A tap, language command or future canvas gesture should converge on the same **versioned semantic edit**, not independent hidden behavior.

### Three distinct identities must not collapse

- **Knowledge object**: a real user/project/interest claim or relation, evidence and privacy from the canonical World.
- **UI instance / spatial node**: a presentation of one or more knowledge objects with stable render instance ID, layout, theme and selection; potentially several views of the same object.
- **Command/intent**: a request to change a view or perform an effect, with target, parameters, owner grant, source and receipt; selection/interpretation does not imply execution.

### Current harness bootstrap contracts: exact GUI02 adapter seams

Inspected on current `vivim-web-harness/main` (2026-10-31):

| Producer | Actual parser/compiled output | Provenance and scope constraint | GUI02 integration significance |
|---|---|---|---|
| [F02 source](../../apps/mobile-relay/bootstrap-f02.mjs) | `parseF02Response`: exact `vivim.bootstrap-f02/2` + `pass:'F02'`; `compileF02RenderPlan`: `vivim.render-plan/1` with trusted `worldId,requestId,revision` | F02 output has no mandatory model-authored `generationId`; trusted Host envelope provides it if available. Safe CSS tokens/status `suggested`. | Translate **Host-validated** theme plan to GUI01 surface; do not add a model-required field or bypass existing color/URL validator |
| [F01 source](../../apps/mobile-relay/bootstrap-f01.mjs) | `compileF01Profile` returns local `claimId(worldId,section,label)`, claims, `suggestedCards`, unknowns, remembered (not confirmed) boundaries | `providerStatus` distinguishes owner-reported, provider-reported and tentative; `hardNos` stays empty rather than promoting AI claims | Preserve claim IDs and card references, evidence/status; do not invent owner-confirmed memory or hard prohibitions |
| [D03–D07 source](../../apps/mobile-relay/bootstrap-pass.mjs) | Exact `vivim.bootstrap-output/1`; sections/categories restricted per pass; `viewProposals` **only** kinds `card|collection|timeline|mode`; deterministic IDs derived from world, pass, kind, label | `schemaProposals` are nonexecutable and consent-required, observation `sensitive` tracked, `unknowns` explicit | Keep all existing output shapes; future spatial/group layouts should be **local derived projections**, not new required AI reply view kinds |
| [Host bootstrap service](../../apps/vivim-host/bootstrap/service.mjs) | `ingestF02/ingestF01/ingestPass` trust-scoped and receipt-deduplicated, with append-only theme revisions and trusted claims/observations | `requireGrant('visual'|'profile'|'pass')`, exact `worldId,requestId,generationId,threadRef`, `owner-theme-wins`, receipt hash conflicts | Attach shared GUI surface projection **after trusted Host ingestion** and use recorded receipts; never duplicate claim writer or resend a model prompt |

**Critical finding:** The checked-in `ingestPass` stores deep `observations`, `assertions`, `schemaProposals`, and reports `viewProposals` in its **receipt**, but does not visibly persist deep view proposals as independent records in that function. GUI02 needs to specify how accepted/proposed view intents are retained and reprojected from trusted receipts (or existing canonical storage) across restart **without creating a second competing personal-data authority**. Inspect the ledger/read-projection seam before selecting an implementation.

Other crucial details: F01 cards have label-derived stable IDs while request-specific activation/revocation remains separate; a title collision or changed user label needs explicit identity reconciliation before UI attachment. F02's existing owner-choice precedence cannot be reversed by late F02 output. `VIEW_KINDS` currently does not include freeform spatial canvas, so the new design must not silently expand the webapp's deep-pass contract.

## ERA 3 — External comparisons and consequences for GUI02

### Research/source matrix (not dependency recommendations)

| Source | Relevant fact | Suitable lesson | Avoid importing |
|---|---|---|---|
| [tldraw Make Real](https://github.com/tldraw/make-real-starter) and [spatial agent harness](https://tldraw.dev/blog/harnessing-the-agents) | Select shapes/annotations, generate and iteratively revise prototype; agent context prioritizes selected/visible shapes and summarizes remote shapes | Selected region + current structured state + previous working version + explicit visual corrections | Their API-key/model routes, general executable HTML and React SDK as mandatory dependency |
| [GenUI Canvas](https://github.com/LuisErlacher/genui-canvas) | Node-connected live HTML/JS components, canvas readback and screenshot; authors explicitly state unsandboxed scripts and in-memory state are PoC limitations | Bidirectional canvas state and local feedback to next AI turn | Untrusted JS executing in VIVIM app or discarding state on restart |
| [mAIndmap](https://github.com/ClaudevGuy/maindmap), [Second Brain](https://github.com/yogeshsinghkatoch9/second-brain), [AI Fine-Tuner](https://github.com/muhamadjawdatsalemalakoum/aifinetuner) | Single-file spatial maps, local snapshot/versioning, relations, command palette; live sliders drive precise visual values | Compact self-contained **exports/prototypes**, semantic connections, immediate direct manipulation | A mandated single HTML runtime, browser-localStorage as canonical PC authority |
| [A2UI 0.9.1](https://a2ui.org/specification/v0.9.1-a2ui/) | Current declarative production protocol with separate components and data model | Safe allowed catalog, separate structure/data, host compilation | Unverified token-stream assumptions; unrestricted client functions/dataRefs |
| [JSON Canvas 1.0](https://github.com/obsidianmd/jsoncanvas/blob/main/spec/1.0.md) and [OCIF 0.7](https://github.com/ocwg/ocif-spec/blob/main/spec/v0.7.0/spec.md) | Portable spatial nodes/edges; OCIF covers richer interchange and relations | Study import/export/canvas interoperability after stable internal IDs | Treating interchange files as an authorization/execution protocol |
| [Ink & Switch local-first](https://www.inkandswitch.com/essay/local-first/) | Offline data ownership, fast responsive local interaction, cross-device continuity | PC-backed authoritative state plus instant phone-local provisional edits | Cloud-only primary design store |
| [CHI 2026 canvas agent field study](https://doi.org/10.1145/3772363.3799044) | 8 designer study/5,838 actions suggests generate→curate/relate phase shift; limited sample | AI should become more of a curator as canvas matures | Presume statistical universality or autonomous acceptance |
| [IEEE VIS 2026 VisCanvas](https://d2.ieeevis.org/year/2026/program/content/271df69a-ce73-4d46-bfa0-e1520dd5a6ca/) | Branch/merge/revisit spatial AI visualizations; 20-user study | Preserve alternate visual hypotheses, branch and compare | Force every user into nonlinear infinite canvas on a phone |
| [Google adaptive generative UI research](https://research.google/pubs/self-evolving-systems-moving-beyond-deterministic-interfaces-to-adaptive-generative-interfaces/) | Directed synthesis, inferred synthesis and runtime adaptive UI as conceptual mechanisms | Suggestions may be personalized but remain clearly source-graded and reversible | Inference as permission, implicit personal fact or automatic host effect |

**Freshness discipline:** earlier Oct 31 conversation named 30 HTML repositories; verified source does not establish that all 30 were created/updated within the last two months. This report verifies selected strong public references, not such a 30-item recency ranking.

### GUI02 implications to decide **before full PRD**

1. **Meaning before presentation:** The post-parse adapter should produce a *typed intermediate semantic projection* with stable World evidence refs and UI view-proposal refs, then compile the visual component tree through GUI01. F02 style and F01/D03–D07 claims should not be flattened into anonymous decorative cards losing evidence, relations or future spatial layout options.
2. **Semantic parity:** A profile claim, project goal or relation must retain its identity through a card, timeline, grouped Space, map node or future custom visual command handle. UI layout is a projection, **not** a new memory authority.
3. **Two independent graphs:** World/knowledge/evidence graph and UI/spatial layout graph are distinct, connected by typed source refs. A command/intent/action graph is separately authority-gated. Do not infer that an arrow between UI nodes authorizes data transfer.
4. **Spatial-ready, not spatial-required:** V1 BF00/F02/F01 should render instantly as a readable mobile stack. GUI02 can retain `layoutIntent`, `groupRef`, `relationRefs`, and stable `objectRef` as **optional, inert metadata**, to support a later infinite canvas without blocking current mobile acceptance.
5. **Cross-input convergence:** Natural language, explicit button, future symbol/glyph and canvas drag must all generate the **same semantic intent ID/target/revision** and pass through existing Host risk/approval gates. GUI02 is a proposal/source adapter, not a replacement for NLCL's parser.
6. **Interpretation confidence ≠ evidence:** The adapter needs distinct `basis`, `confidence`, `sourcePass`, `sourceRequest`, `unknowns`, `alternativeViews`, `sensitivity`, and `effectClass`. The renderer never turns a hypothesis into an owner-confirmed claim or a locally active capability.
7. **Negative-intent frontier:** Prioritize exposing one material uncertainty (e.g. “Show private relationships on the phone?” or “Does ‘this project’ mean A or B?”) rather than many generic alternatives; no speculative disclosure or action pending resolution.
8. **Revision continuity:** Each pass is independent and possibly late. Dedup by trusted pass/request/evidence; adapt against **current** UI/World revision with CAS-aware plan or explicit conflict. An F02 result arriving late cannot overwrite owner-selected style; D03 cannot reorder a user-pinned panel without approval.
9. **Separate content vs design vs execution:** F02 safe theme, F01 claim evidence, D03–D07 knowledge, view suggestions, UI layout and capability invocation have distinct rights. Generated cards may prompt “Enable”; they cannot perform the action.
10. **Preserve GUI01 seam:** While GUI01 is being implemented, GUI02 research/design may specify immutable semantic fixtures and post-parse mapping, but **must not freeze undocumented GUI01 component props or mutate GUI01 files**. Coordinate first over catalog version, component IDs, dataRef scope, proposal operation types, reversible patch and error contract.

### Research-backed GUI02 acceptance probes to carry into the PRD

- Same validated F02 response yields identical typed visual proposal and deterministic hash under same versioned baseline; retries/restarts do not duplicate.
- F01 observed interest/project creates a rooted World reference plus one or more UI presentations, with its exact source/basis/unknowns intact; an ungrounded response creates no fictional personal card.
- Two independent presentation variants (card vs timeline) reference the **same knowledge objects**, not duplicated knowledge claims.
- A future NLCL phrase and an equivalent tap/drag produce the same canonical intent **in a test fixture**; neither gesture nor model output directly grants execution.
- Old AI pass result cannot overwrite newer direct style or a pinned/layout-edited UI. Version conflicts are visible and recoverable without provider resends.
- D06 relationship/third-party input never appears in default phone spatial projection, action preview or export.
- A generated `→` relationship is inert until an authorized action, and a generated `✓` is not verified completion.
- The real Android demo remains quick and simple: provisional first paint → F02 theme → F01 grounded card → optional deep view → same Build surface → select/edit/Undo → reconnect with exact Studio/Host receipts.

### Explicit decisions still open

- **GUI01/GUI02 interface:** Is the first UI patch grammar five PRD11 verbs or aligned to legacy eight-mutation variants? Compare capability before schema freeze; no independent second compiler.
- **Spatial layer:** Reserve inert semantic/spatial metadata in GUI02 V1 versus implement actual pan/zoom/semantic zoom after shared renderer acceptance. The former is strongly favored; latter is not yet release-authorized.
- **Semantic command runtime:** Reuse pure VOMEGA NLCL later as an optional interpreter; no separate GUI02 parser. Decide target namespace and `!`/confirmation compatibility only after security review.
- **Single-file HTML:** Candidate as a portable **export or sandboxed standalone mini-app**, not default privileged runtime. Separate execution permission, CSP/sandbox/host-object threat analysis.
- **Branching variants:** Preserve IDs and future branch points as metadata now, build multi-canvas branch/merge after minimum single-surface CAS proof.
- **Context:** Provider webapp receives only selected UI and approved DCB; viewport screenshots/drawings must be reviewed as new potentially sensitive egress, not silently forwarded.

## Explicit next action

Before drafting GUI02 implementation PRD, compare this research against the **actual GUI01 schema/entrypoints in progress**, resolve the minimal semantic projection/adapter interface, and then write one executable GUI02 PRD with fixtures and exact acceptance criteria. This research file does **not** change `docs/roadmap.json` statuses, authorize implementation, or mark live verification.
