# PDR input — arbor-roadmap lane (keeper) — round 2, 2026-11-14

**Status: INPUT DRAFT for consensus — not decisions.** Consensus-first binds (bus `9a71f7`): nothing product-shaped lands until the refinement converges AND the owner gates. Registered queue items stay queued; no run launch/build from this lane. Full round-2 critique on bus: `df2b24` + `c22343` (topic=design-input-r2).

## 1. Registered L2 item texts (verbatim, `docs/product/roadmap/roadmap.v1.json` → `.layers.L2`)

**CHAT-BOX-BASELINE** — "Owner baseline reset 2026-11-14: PAGE 1 is the baseline user interaction page — PURE CHAT BOX ONLY, THE product surface iterated from. AI-use native day 1: chat serves human and AI through the SAME endpoints; machine-readable state primary. Demo thread acceptance: full realtime stateful machine-readable state + UX mirror + realtime read of rendered wiki (vivim-use, no screenshots). Proposal artifact: docs/product/proposals/chat-wiki-baseline/chat.html.draft + state.mjs.draft. 5 existing day-1 capabilities (sendMessage, readState, executeIntent, readJournal, getCapabilities)."

**WIKI-BASELINE** — "Owner baseline reset 2026-11-14: PAGE 2 is the realtime interpreted wiki. Wiki = roadmap live view: roadmap.v1.json + journal rendered live = source of truth; writers = owner-via-chat, AI-via-intents, workstreams-via-write-back. Needs 7 wiki actions added to action-catalog.v1.json (createPage, editSection, movePage, linkPages, readWikiSection, listPages, deletePage). Proposal artifact: docs/product/proposals/chat-wiki-baseline/wiki.html.draft."

**RDCB-BASELINE** — "Owner directive 2026-11-14: start using Dynamic Context Bundles with a special class = Reprogramming Dynamic Context Bundles. Highly structured source-code + vlang native context bundles built on the fly during ANY reprogramming work; linked and tied to the auto wiki as a special section; as vlang improves and is implemented into source and compiled, RDCB generates vlang reprogramming commands / scripts / workflows based on users."

buildQueue (owner-approved batch 1, all `status: "queued"`): order 1 CHAT-BOX-BASELINE, 2 WIKI-BASELINE, 3 RDCB-BASELINE. All three currently `dependsOn: []` — see §1a.

### 1a. PROPOSED Option-A amendments to these texts (NOT applied — owner-gated, P7 pending)

- CHAT-BOX-BASELINE description += "Served at the product chat route on the production Host; /demo/ retains machine artifacts only (journal, acceptance records, feedback). Write endpoint = ONE typed product endpoint, identical envelope for human and AI, `actor` + `schemaVersion` fields day 1."
- WIKI-BASELINE description += "Renders the file-surface special section (Stage A seeds = the loop's own machine files: state.mjs, controller.js, server.mjs ASSETS map). One parser serves both stages."
- RDCB-BASELINE: `dependsOn: []` → `["WIKI-BASELINE"]` (bundles reference file-surface records produced by Stage A); description += "Generator v1 targets the Stage A seeded corpus; bundles reference file-surface records + NLCL action ids."

## 2. Gate-evidence rules (batch-1 acceptance evidence; ride `arbor.item-receipt/1` at gate flip)

Why these exist: the implemented-work exclusion reads HQ-index "exists" + receipts "runs"; batch-1 items are NEW, so no such evidence exists yet. These rules define the evidence the build loop must produce so the exclusion can never false-exclude batch 1 and the Acceptance-replay phase has deterministic assertions.

- **CHAT-BOX-BASELINE** — receipt shows all three:
  - (a) **route:** the product chat route serves 200 with the machine-readable state endpoint;
  - (b) **capabilities:** `getCapabilities` enumerates 12; each executes a deterministic record through the REAL NLCL engine (`apps/vivim-host/control/command-intent.mjs` + `intent-planner.mjs`; mobile parity `apps/mobile-relay/nlcl/`);
  - (c) **parity:** same-envelope test — machine POST ≡ owner browser send → identical state mutation; envelope digest recorded on the receipt.
- **WIKI-BASELINE**:
  - (a) roadmap.v1.json + journal rendered live on poll;
  - (b) the 7 wiki CRUD actions (deletePage as soft-delete/archive) mutate deterministically with journal records;
  - (c) the file-surface special section renders `file-surface.v1.json` for the Stage A seeds.
- **RDCB-BASELINE**:
  - (a) the generator emits a schema-valid bundle for the seeded corpus;
  - (b) digest stable across 2 consecutive runs (DCB digest rule, `generatedAt` excluded);
  - (c) references resolve (bundle file paths exist; NLCL action ids present in the action catalog);
  - (d) no execution claims — format + generator only (gate-status honesty canon).

## 3. Def hole-prompt revision proposal (bounded edit at next def touch — no run change now)

**CURRENT** (`arbor-roadmap.dwf.ts`, "Owner prioritization round" return-hole): "The owner marks priorities item by item; then the build queue is written and one item is expanded to atomic tasks."

**CONFLICT:** the self-iterating canon (owner 2026-11-14, `arbor-loop-design.v1.md` §loop) makes per-item rounds owner-free; the owner enters on exception or batch approval. The canon is newer and owner-issued — the def prompt predates it.

**PROPOSED REPLACEMENT:** "The owner approves/reorders the proposed ordered batch (3-8 items) and picks the batch's atomic-expansion target. Per-item rounds are owner-free (self-iterating canon, owner 2026-11-14); deterministic NLCL-assertable checks run owner-free with receipts. The gate decision lands as a journal record of kind owner-gate; the keeper thread executes the roadmap.v1.json mutation and marks the entry executed (or failed)."

**ALSO at the same def touch:** the emphasis gate stays args-carried; the hole parks only when genuinely ambiguous.

## 4. Position recap (one line each — full critique on bus `df2b24`/`c22343`)

- **P1 accept+adjust** — restore deletePage (12 capabilities, not 11); soft-delete/archive semantics.
- **P2 accept+adjust** — ONE parser for both stages (Stage B rides existing wiki.mjs + comment-wiki-bridge.mjs); Stage A seeds = the loop's own machine files.
- **P3 accept+add** — RDCB-BASELINE `dependsOn ["WIKI-BASELINE"]`.
- **P4 adjust** — the mutate envelope kind=gate-decision RECORDS the decision; the keeper thread EXECUTES the roadmap.v1.json mutation (`recorded → executed|failed`); one-writer rule for roadmap.v1.json (keeper sole writer, pages/wiki read-only).
- **P5 accept+adjust** — endpoint path is Option-A-dependent (`/api/mutate` under A, `/demo/mutate` under B); envelope gains `actor` + `schemaVersion` day 1.
- **P6 accept** — per-batch owner gate verbatim (matches self-iterating canon); journal gate-card semantics per P4.
- **P7 — Option A** (amendments in §1a); re-walk CONFIRMED under A: gated items re-walk the REAL product surface (stronger than demo mocks); signal stays the `rewalk: pending → done` queue-entry file contract; verdicts re-enter the journal (append-only JSONL per P5).
