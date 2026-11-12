# Living Composer — independent UI application, bounded by real VIVIM authority

**Round:** 2 · **2026-11-11** · **status:** experimental design + tested lab code, NOT a production feature.

## North star

The composer is a miniature full-stack application with its own expression, interpretation, affordance, evidence and interaction surfaces. It is not a chat box embellished with static buttons. The user may begin with half a sentence, a glyph, a voice note, a selected file or a highlighted paragraph. A bounded, purely visual projection turns each event into an understandable representation of *what VIVIM thinks they mean*, *why*, *what it can currently offer* and *what the surrounding knowledge says*.

The same intent should be expressible as natural prose, compact symbolic notation and accessible controls. Those must resolve to the same approved typed action IDs and argument schemas **only when the existing owner-approved crosswalk proves equivalence**. Visual clarity must never be confused with permission to act.

## Compositional visual grammar — far beyond icons

An incoming expression can be presented through multiple independently switchable **views** of one inert intent projection, without inventing any extra executable command:

1. **Typography as semantics:** verbs in strong weight; targets as underlined entity links; objects as subtle cards; intent boundaries as adjustable typography; literal user text as quoted blocks; provenance/uncertainty rendered explicitly. Multilingual/Mallorquí/Spanish labels must preserve original user text and character spans.
2. **Colors as visual categories:** one muted base with role accents for verbs, references, data, direction, uncertainty and warnings. No color itself grants risk, consent or authority. Text label and icon always accompany color.
3. **Outline grammar:** solid = claimed verified source when there is a receipt, dashed = source wired only, dotted = declaration/unknown, double = selected/focused, glow = temporary active preview. Configurable appearance must never rewrite readiness.
4. **Glyphs and symbols:** inherit Omega's 17 families (`/ @ # ! ? + - & $ * % ∆ ✓ = ^ ~ →`) as vocabulary hints. The `!` family must never bypass Host approval; labels must explain its denied/ignored status where applicable. Icons are per-source affordances, not command IDs or permission grants.
5. **Inline slot objects:** `email` creates recipient, subject, account and body affordances, each sourcing their allowed values from the owning read-only port; Tab replaces only the correct input span. Literal data is never misparsed as another command.
6. **Hyperlinked knowledge:** source-backed wiki/KB references, paths and line numbers become navigable; ambiguity, redaction and staleness remain visible. Only known public repo-relative source locations become GitHub links in this lab. Private/local anchors need a separately approved Host resolver, not a fabricated web URL.
7. **Flow geography:** a long sentence fans out into adjacent intent cards / dependency edges, not just a flattened text string. Sequential, compound, conditional, negative and mutually dependent actions must all retain their semantics or return a clear ambiguity/refusal.
8. **Time and origin:** a project assertion is labeled *designed / source-observed / verified / superseded*, with effective dates, provenance and revision. Imported conversation claims are never automatically treated as current facts.
9. **Spatial modes:** Inline (compact composer), Lens (composer + right evidence panel), Canvas (TierFlow graph), Timeline (source chronologies), and Focus (single action/slot). These are UI transformations of the same data, not five separate workflows.
10. **Accessible multimodal cues:** icon+text, color+shape+word, hover/keyboard/touch, optional speech/haptics, screen-reader announcements and reduced-motion mode. Context updates should not hijack a user's cursor or interrupt composition.

## A first-principles layout

```text
┌─────────────────────────────── EXPRESSION SPACE ───────────────────────────────┐
│  [Project] [Agent/Workflow] [Provider] [Attachments] [Privacy] [Mode]         │
│                                                                              │
│  I want to email [person▾] about [project↗] after [approval▾] ...             │
│                │                         │                                   │
│                └─ contact menu           └─ linked HQ/Personal evidence       │
│  ──────────────────────────────────────────────────────────────────────────── │
│  Interpretation in complete English: "Prepare a draft ..."                   │
│  [✉ email] → [♙ person] → [# topic] → [✓ approval needed]                     │
└──────────────────────────────────────────────────────────────────────────────┘
┌────────────── INTENT GRAPH ──────────────┐ ┌──────── LIVE KNOWLEDGE PAGE ──────┐
│ candidate, slots, alternates, gaps       │ │ contextual wiki / graph / history │
│ source owner / state / execution preview │ │ relevant citations, not generated │
│ no grant, approval or effect in the lab  │ │ facts without source evidence     │
└─────────────────────────────────────────┘ └───────────────────────────────────┘
```

## Live wiki interaction — the real stepping stone

The first page projection uses `repos-hq/hq/lib/kb.mjs:kbQuery`, `hq/lib/search.mjs:searchContent`, and the existing `.hq/index.db`. The UI turns each utterance into an ephemeral, small, read-only working page with title, focus, matching KB evidence, code references, source revisions and unknowns. It is **not** an independently published wiki and stores no new graph.

- **T = keystroke:** type-based parser/project selection and visual spans; no external effects.
- **T + debounce (~260 ms):** cancel old HQ query; query the read-only index with bounded tokens; return provenance and project/head/index timing when available.
- **T + evidence:** show source-backed article-like page, links to known public relative source files, and evidence cards. No evidence means an empty/error state, not a confident narrative.
- **T + composition:** the user can explicitly pin a source, choose a candidate entity, or ask for a more detailed view. Source and permission identity survive UI projection.
- **T + future user knowledge:** Host-authorized personal graph/KB projection may join the same visual response envelope, not the HQ development SQLite database. Personal snippets remain private and scoped; no private user transcript content enters public Labs or a model automatically.

## Reuse the actual existing infrastructure

- HQ: development graph, registries, code/wiki/KB read interfaces, DCB algorithms, indexing, provenance and orphan detection.
- Host: user permissions, signed-in Chrome/Studio, client source identity, personal evidence, World, command planner, runtime receipts. No provider API exception.
- Shared Rust graph-node compute: common algorithms but **development and personal namespaces remain distinct**. Never copy the personal corpus into the HQ development DB as a shortcut.
- TierFlow: production rendering, visual interactions and revision authority; the lab never publishes a World document on its own.
- Historical Omega and VIVIM Final: recovered language/token algorithms, symbols, composer patterns and interaction idioms; they are donor designs, not active authority.
- Labs personal-knowledge-lifecycle: `runtime/` already stages user-selected ChatGPT export parsing, T=0 intake and scoped Rust proposals under explicit Host ports, synthetic-tested only. The composer should consume its future approved *read projection*, not directly import archive bytes or instantiate a second corpus store.

## The portable read envelope to standardize later

**Proposal only** (do not count this as an implemented Host contract):

```json
{
  "schema": "vivim.context-lens/1",
  "worldId": "owner-opaque-id",
  "inputRevision": 42,
  "sourceRevisions": [{"owner":"hq","digest":"..."},{"owner":"host","revision":"..."}],
  "query": {"text":"...","focus":"...","purpose":"compose"},
  "sections": [{"kind":"wiki|evidence|history|graph|actions","items":[{"id":"...","source":"...","status":"observed","title":"...","snippet":"..."}]}],
  "privacy": {"personalIncluded":false,"destinationDisclosureAllowed":false},
  "diagnostics": [{"code":"STALE_SOURCE","severity":"warning"}]
}
```

## Non-negotiable falsification

1. No current HQ checkout or SQLite index → explicit unavailable, zero fabricated references.
2. New keystroke during slow lookup → abort previous request and ignore stale result, preserving text/caret.
3. A command ID observed in HQ must never imply a Host action is enabled. Revoke/check permissions at the effect owner.
4. Personal sessions/notes disabled by default; never share personal snippets into source-controlled fixtures or unconsented model prompts.
5. Source text is data; imported conversations cannot create command syntax, trusted author instructions or executed tool calls.
6. More than one candidate contact or action → show alternatives and request human selection rather than selecting silently.
7. Formatting changes never alter command meaning, actor context, risk or approval requirements.
8. Every wiki card can explain its source/revision and admit when index freshness is unknown.