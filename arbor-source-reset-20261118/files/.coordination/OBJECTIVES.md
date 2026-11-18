# ARBOR / VIVIM — CANONICAL OBJECTIVES (owner-authorized, 2026-11-18)

Sources of authority: owner directives (this thread, 2026-11-18) > ChatGPT orchestrator rulings
(.coordination/pdr/chatgpt-guidance-20261118-*.md + chatgpt-product-design-deepdive-20261118.md,
pushed 30bbaff) > this canon. Keeper is sole writer of docs/product/roadmap/roadmap.v1.json.

## The product definition (ChatGPT deep dive, governing reference)

**VIVIM** = a user-owned, local-first AI computing environment that understands your language,
connects your knowledge, lets you create and operate your own software, and evolves useful
capabilities over time. Six ownership dimensions: My AI · My knowledge · My machine · My apps ·
My history · My evolution.

**ARBOR** = the integrated semantic engineering + autonomous development system that builds,
tests through the real product, and improves VIVIM under owner control. ARBOR's value is measured
by resulting VIVIM capabilities, never by cycles/receipts/agents produced.

Core interaction: **Tell VIVIM what you want → VIVIM understands it, shows you what it means
(live preview), and helps turn it into lasting software you control** (Host authorizes effects).

## Development sequence (in order; every increment must move toward the complete experience)

1. Make the semantic interaction real: NL input → grounded typed interpretation (VEC) → live
   visual preview → authorized Host effect → visible changed state. Start with one Wiki operation
   through existing code; prove the whole boundary.
2. Make the interface explain its understanding: semantic highlighting, source links, ambiguity
   choices, two-TAB completion, VIEW ALL (semantic X-ray). Not decorative polish.
3. Make knowledge follow the user: screen context → authorized knowledge → source-backed,
   revision-aware AutoWiki.
4. Make the user's AI genuinely persistent: conversation continuity, provider-aware knowledge,
   memory, agents, project state in one user-facing experience.
5. Make capabilities reusable and evolvable: validated semantic operations → reusable typed
   behaviors → deterministic execution (stop paying reasoning cost twice).

## The 10-cycle target (per lane, product checkpoints — not commits)

One complete vertical journey: **user types a natural-language Wiki update in VIVIM Chat →
VEC generates a typed proposal → user reviews and approves → Host validates revision and records
the mutation → Wiki visibly updates from authoritative Host state.** If unsupported, fix the
first missing production edge in dependency order. Never simulate missing behavior.

## Lane ownership (frozen for the 10 cycles; disjoint files)

| Lane | Files (exclusive) | Capability |
|---|---|---|
| chat | apps/vivim-host/public/pages/chat.html, chat.js (+ test/life-chat-preview.test.mjs) | real /chat: command in → interpretation + outcome visible |
| vec | apps/vivim-host/chat/vec-compiler.mjs (NEW, only after identifying the canonical compiler seam; never a competing interpreter) (+ test/life-vec-compiler.test.mjs) | NL → bounded typed candidate or refusal; hands to Host |
| host | apps/vivim-host/server.mjs, apps/vivim-host/chat/product-journal.mjs (+ test/life-host-mutation.test.mjs) | approved mutation exactly once + journal; stale/unapproved/duplicate refused |
| wiki | apps/vivim-host/public/pages/wiki.html, wiki.js (+ test/life-wiki-projection.test.mjs) | live Host-backed state, revision + provenance visible |

FROZEN for all lanes: apps/mobile-relay/app.js, apps/mobile-relay/control-chat.mjs,
.zcode/, scripts/roadmap/, scripts/batch-evidence/, docs/product/roadmap/, all orchestration.
Host lane alone owns server.mjs. Max 2 simultaneous code writers; one integration writer
stages ONLY the lane's exact files (explicit paths) and commits to main; never git add . /
never reset the shared tree.

## Acceptance levels (testing = real UI to core, per owner)

- UNIT_PASS: focused unit/contract test, real exit code.
- HOST_JOURNEY_PASS: real HTTP through the isolated scratch Host
  (`bun scripts/life/scratch-host.mjs session` → real pipeline → correlate /api/state,
  /api/wiki, /api/journal → stop). curl-only is NOT browser evidence.
- BROWSER_E2E_PASS: real Chrome, human/agent-controlled, against the scratch Host's real served
  pages (type in the real UI, inspect rendered result). No Playwright/Puppeteer, no cross-repo
  plumbing. No browser interaction = BROWSER_E2E_UNRUN. Final product acceptance needs
  BROWSER_E2E_PASS; unit/host are supporting checks.
- Negative case required per cycle (stale revision / unapproved / unsupported / duplicate).

## Hard rules

- A cycle with no product diff and no newly evidenced defect does not earn another cycle.
- No false green: UNRUN is never PASS; no assertion-based acceptance; no simulated effects.
- Cron sweep stays disabled (automation-f2dd9a68 = HOLD) until the manager reactivates it.
- ChatGPT = design authority for architectural changes (via the proven studio drive channel);
  owner = final authority; keeper = sole roadmap writer; manager = integration writer.
- Success metric: the owner can USE a new capability in VIVIM.
