# R010 — Semantic Wiki Compiler: A Deterministic, Explainable Language for VIVIM

**Date:** 2026-11-12 · **Status:** expanded design, source-backed proof-of-compiler research, no product integration.
**Canonical technical home:** [HQ Semantic Wiki Compiler](https://github.com/owenservera/repos-hq/tree/research/semantic-wiki-compiler-20261112/research/semantic-wiki-compiler).
[Architecture](https://github.com/owenservera/repos-hq/blob/research/semantic-wiki-compiler-20261112/research/semantic-wiki-compiler/ARCHITECTURE-SWC-001.md) · [Product PRD](https://github.com/owenservera/repos-hq/blob/research/semantic-wiki-compiler-20261112/research/semantic-wiki-compiler/PRD-SWC-001.md) · [Meta coding language](https://github.com/owenservera/repos-hq/blob/research/semantic-wiki-compiler-20261112/research/semantic-wiki-compiler/META-CODING-LANGUAGE-SWC-001.md) · [64 tasks](https://github.com/owenservera/repos-hq/blob/research/semantic-wiki-compiler-20261112/research/semantic-wiki-compiler/IMPLEMENTATION-ORDERS-SWC-001.md).

## ERA 1 — The new principle: wiki knowledge is also executable *meaning*

Imagine that when you type a request, VIVIM does not ask a model to "guess which function to call." It traverses a compiled source-linked semantic wiki:

- Code and annotations declare what a capability **means**, which typed input/output/effects/contracts/tests it has and what it consumes/emits.
- The registered Host/TierFlow capability catalog establishes which actions are **actually owned** and can be planned.
- Curated, tested language packs associate natural-language phrases with typed semantic frames, aliases, entity types, slot schemas and constraints.
- Rust efficiently computes pure matches, related entities, structured multi-intent DAGs and source provenance.
- The composer shows exact **word-level interpretation** before any action: entities circled, slots surfaced, DCB branches, memory proposals, negation scopes, knowledge links and possible effect gates.
- The existing Host owner validates selected intent, admits effects under its own risk/approval controls, and returns actual receipts.
- Human wiki and AutoWiki explain *why* the system understood the request.

**The wiki is a compiled semantic service, not a database with omnipotent commands.**

## ERA 2 — The interaction this could unlock

Typing `full` can reveal:
`Full update [project?] · [next steps/issues/decisions?]`.

One Tab grounds the project and the second selects a focus. The same source-backed compiler can:
- display exactly which action/query frame the phrase maps to;
- show what code comments, semantic registry IDs and tests define its meaning;
- prewarm relevant permitted HQ/Host reads;
- expose why the interpretation is incomplete or ambiguous;
- explicitly say if an actual runtime capability is not available.

"Make the wiki intelligent" really means **compile behavior and explanation from one reviewed semantic source**, so neither the UX nor the documentation has to invent semantics.

Example developer comment v2:
~~~ts
/**
 * @hq:action vivim.status.show@1
 * @hq:capability vivim.status.read@1
 * @hq:contract vivim.status.request@1
 * @hq:effect host:read-only
 * @hq:evidence test:tests/status.test.mjs
 */
export function statusRead() {}
~~~

The HQ research compiler can already extract a **non-executing** typed semantic graph from the actual v2 comment parser (with known registered IDs and an explicit Host catalog crosswalk). Future owner-reviewed v3 meta-comment tags would refer to **registered language pattern packs, slot types, safe visual roles and wiki topics**. No comment grants permission, registers a new executor or asserts a test passed.

### Two genuinely different possible responses

- The user says "show status" → source-linked matching candidate; existing Host validates and potentially performs a **real read**.
- The user says "what does publishing do?" → semantic/wiki explanation of `ui.publish`; **no publish operation** just because the word matches.

This distinction is essential for a system that *feels* smart but remains safe and reliable.

## ERA 3 — What it will take

**Five subsystems to converge, not five new competing systems:**

1. **HQ semantic compiler:** source comments + registry IDs + capability schema → typed semantic wiki IR + human pages + diagnostics.
2. **Rust pure runtime:** compiled hot grammar, slots, constraints, graph context, source version checks and typed candidate traces.
3. **Host owner admission:** same Host action planner, exact runtime catalog, risk, permission, approval, idempotency and receipts as today.
4. **Visual lens:** same trace for Quiet/Assisted/VIEW ALL, with semantic chips, rings, typed wires, source explanations and explicit unresolved state.
5. **AutoWiki + Rock Core:** contextual human explanation from authorized source graph; minimal compiled language/semantic index protected locally, cold wiki materialization bounded and rehydratable.

**Research result already committed:** HQ branch has a pure compiler, explicit catalog crosswalk, and 22 deterministic tests evaluated successfully under V8 against the actual HQ v2 annotation parser. It intentionally sets `executable:false`, `sourceDigestVerified:false` and `runtimeVerified:false`. Full Node/Bun/Windows/ARBOR acceptance remains unrun.

**The 90% ambition** must be measured on *frequent supported VIVIM capability requests*, not all human conversations. Correct means whole typed frame and context, not merely choosing a close action. Open-ended creative/research work remains genuinely model-beneficial; out-of-scope deterministic commands abstain.

**ARBOR migration guard:** Proposal 5 is independently integrating existing Host/HQ/Rust/TierFlow sources. Keep this work on HQ's isolated research branch until integration owner G has accepted a pinned baseline. Then bring over the approved **read-only semantic IR**, never introduce a second runtime action router.

### Product-level mental model

**The wiki knows what things mean. The deterministic kernel determines what the words could mean. The user confirms what they meant. The Host decides what is permitted. The runtime reports what actually happened. The composer shows all of that as it unfolds.**

That's the vision connecting R004–R009 with this new R010.
