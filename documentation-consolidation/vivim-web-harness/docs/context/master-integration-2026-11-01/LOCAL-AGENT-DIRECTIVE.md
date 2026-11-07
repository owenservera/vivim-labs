# LOCAL AGENT DIRECTIVE — merge recovered context research into the real system

**Owner request (2026-11-01):** This is a second deep-pass research handoff for the current Context Lake / master agent context work. The local worker should **pull and incorporate**, not re-perform the research blind. Full Windows code/integration work is already authorized; provider effects still need scoped approval. Work directly on main, reconcile local dirty work, no extra controller and no Vercel deployment.

## First task — do this before new architecture

1. At C:\0-BlackBoxProject-0\vivim-web-harness, inspect git status, current HEAD and the existing local context work from the attached OpenCode session. Do not overwrite locally authored Context Lake PRD/blueprint, hooks, tests or staged edits.
2. Read this folder (README, atlas, master integration, gaps, chat atoms, first-session) and compare every claim to current source. Pull source refs in source-map.v1.json. Mark unverified/contradicted claims instead of papering them over.
3. Identify the real latest **Context Lake** worktree documents; in the session they were docs/architecture/CONTEXT-LAKE-V1.md and a PRD 13, but those names were not observed on remote main. If these are already committed under new names, reuse them.
4. Reconcile with root docs/INDEX.md, context.md, AGENTS.md, docs/CONTEXT-DISCOVERY.md, docs/agents/START-HERE.md, docs/roadmap.json and docs/library/DECISIONS-AND-CONFLICTS.md. No new master status ledger or nested AGENTS.
5. Produce concise difference table: already implemented / draft local / missing / needs decision. **Then code** the minimal path to satisfy "hi" acceptance.

## Ordered implementation

**I.** Preserve/reconcile CTX01–CTX06 work. Build a safe deterministic read-only local brief over Git HEAD + roadmap + task evidence before optional event lake complexity. Implement Git projection later using design in local Context Lake blueprint. Tests for dirty state, missing data, fixed budget, staleness, deterministic feed. Never commit sensitive file bodies.

**II.** Add the 1–3 line first-turn recipe to root AGENTS.md once the command exists; correct START-HERE contradictory freeze. Rebuild generated catalogs with existing reindex tools after changes; update doc discoverability. Do not bulk add all 133 AION filenames.

**III.** Wire read-only context feed adapter to *existing* harness/src/context/dcb-sources.mjs and reuse createDCBComposer; verify manifest/freshness/provenance. Keep personal and dev audience policies separate. Test source changes, denied sensitive refs, mixed-HEAD refusal.

**IV.** Implement minimal chat-atom schema and local addressable segmented provenance as a separately testable slice, using world-note-units pattern and canonical capture ownership. Keep journal/live projection metadata evidence separate from text storage. Add to existing DCB adapters only with permission.

**V.** Complete owner preview/disclosure and real CLI/phone UX after foundation; no automatic provider send or private body leakage. Add light prompt-injection tests: malicious source content must remain inert context data.

## Efficient source recon checklist

Read exact refs from SOURCE-ATLAS.md instead of full sibling repos. Review any current local work from AION/VOMEGA/vivim-final only when it changes the design. Detect whether the repo already has task packet, DCB source adapter, sqlite FTS index, object/atom schema, OpenCode session hook or startup command. Call a feature **absent** only after both source index and current worktree search. Use a single builder/owner; no need to spawn another controller or an unbounded subagent swarm.

## Required tests and evidence

- node --test on added/affected context and Studio/Host test files; npm run check:docs; npm run check:integration; existing query/index checks.
- Local read-only direct test of brief and DCB with real Git state, then actual clean OpenCode session "hi" on Windows.
- Verify no unknown sends/duplicate worker; no secret/prompt/reply content in committed feed; source relevance and fixed budget.
- Track unchanged vs changed files, test receipts, real commit SHA, unresolved blockers. Never claim live acceptance from source tests.
- If context modules depend on current producer changes, fence those interfaces with compatibility tests; do not race a second local agent working same files.
- Update **existing** roadmap item status only on actual matching receipts, not because this folder says "P0".

## Definition of done

A new OpenCode session opened at repo root responds productively to "hi"; it used the correct current HEAD/dirty/roadmap evidence, provides grounded next steps, loads scoped context when selected, makes no unapproved effect, and can explain exactly which source revisions made up its context. The master context architecture is represented by one DCB composer and authoritative upstream stores; the new Context Lake and chat atoms feed it rather than replace it.

## Suggested local first message (paste into the current agent)

"Pull main, inspect docs/context/master-integration-2026-11-01/README.md and LOCAL-AGENT-DIRECTIVE.md. Reconcile the attached session's uncommitted Context Lake design with the current working tree, then implement the smallest safe end-to-end 'fresh OpenCode session + hi → source-grounded next steps' slice. Reuse DCB and the existing authority layers. Run tests, integrate indexes and report real receipts."
