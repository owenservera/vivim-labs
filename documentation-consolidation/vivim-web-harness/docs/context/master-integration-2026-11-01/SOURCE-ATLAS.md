# Second-pass source atlas: discovered, reusable, missing

**Inspected against GitHub main 2026-11-01.** Repository references below are links to source, not claims of live acceptance. GitHub paths correspond to Windows roots C:\0-BlackBoxProject-0\<repo-name>, except where the actual folder differs.

## A — Existing vivim-web-harness runtime/source

| Source | Existing responsibility | Integration recommendation |
|---|---|---|
| [harness/src/context/dcb-composer.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/harness/src/context/dcb-composer.mjs) | Deterministic task/personal DCB composition, scope, ranking, size budgets, manifest; profiles seed/reunion/convergence/continuum/handoff/probe/deep_research/decision_brief | **Reuse one composer**; do not replace it with Context Lake |
| [harness/src/context/dcb-sources.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/harness/src/context/dcb-sources.mjs) | Repo roadmap, registry, PRDs, Git/source, CANON/Immediate/Curated adapters | Add consented chat-atom adapter and context-feed adapter once contracts exist; preserve exact scope |
| [studio/src/agents/personal-agent-service.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/agents/personal-agent-service.mjs) | Versioned primary Personal Agent and contextBundle() integration | Reuse; no second personal memory writer |
| [studio/src/agents/personal-agent-continuity.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/agents/personal-agent-continuity.mjs) | Local CANON/Immediate/Curated continuity packet, work summary, export | Reuse for private agent continuity; not an automatic context disclosure grant |
| [apps/vivim-host/conversations/live-projection.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/conversations/live-projection.mjs) | Source-verified metadata-only conversation/message lifecycle | Link chat atoms via stable verified message identities, never reconstruct transcript text from metadata |
| [studio/src/capture/thread-turn-journal.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/capture/thread-turn-journal.mjs) | Send-once journal, exact-thread delivery, reply digests and revisions | Read/verify provenance; not a chat atom store or new sender |
| [studio/src/studio/world-note-units.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/studio/world-note-units.mjs) | Existing content-derived atomic note units with stable fragments/paths | **Pattern to reuse** for segmented chat citations; do not conflate notes with chats |
| [docs/CONTEXT-DISCOVERY.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/CONTEXT-DISCOVERY.md) | Task-specific doc/capability/source routing | Expose the new context brief here, not a separate root instruction hierarchy |
| [docs/agents/TASK-CONTEXT-ROUTING.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/agents/TASK-CONTEXT-ROUTING.md) | Normative skill/component routing | Keep component-aware loading selective |
| [docs/roadmap.json](https://github.com/owenservera/vivim-web-harness/blob/main/docs/roadmap.json) | **Only** task-status authority | Context Lake observes; it must not overwrite statuses |
| [docs/capabilities/atomic-registry.json](https://github.com/owenservera/vivim-web-harness/blob/main/docs/capabilities/atomic-registry.json) | Atomic **capability** IDs | Never mistake capability records for atomic chat content |
| [docs/architecture/DYNAMIC-CONTEXT-BUNDLES-REVIEW-2026-10-31.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/architecture/DYNAMIC-CONTEXT-BUNDLES-REVIEW-2026-10-31.md) | Recovered DCB design plus legacy migration directions | Architecture decision still applies: one composer, separate policies |
| [dev/prd/06-personal-world-memory-and-context.md](https://github.com/owenservera/vivim-web-harness/blob/main/dev/prd/06-personal-world-memory-and-context.md) | WORLD-08 deterministic bundling, WORLD-09 owner-visible disclosure, WORLD-10 transfer, WORLD-16 deletion | Product intent; verify implementation independently |
| [docs/agents/START-HERE.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/agents/START-HERE.md) | Dated docs entrypoint | **Known contradiction**: still says documentation-only freeze despite newer owner authorization |
| [AGENTS.md](https://github.com/owenservera/vivim-web-harness/blob/main/AGENTS.md) and [context.md](https://github.com/owenservera/vivim-web-harness/blob/main/context.md) | Root context instructions | Inject bounded first-action recipe; preserve one root each |

## B — Proven cross-repo ancestors

| Source | What is actually reusable | Caution |
|---|---|---|
| [VIVIM Final DCB projector](https://github.com/owenservera/vivim-final/blob/master/src/engines/dcb-projector.ts), [profiles](https://github.com/owenservera/vivim-final/blob/master/src/engines/dcb-profile.ts) | Ten original layers (including L7UserQuery), render forms inject_prompt/system_message/capsule/panel_card, profile policy | Modern composer lists nine source layers; L7 is query input, avoid duplicating |
| [VIVIM Final ContextPackager](https://github.com/owenservera/vivim-final/blob/master/devops/agentic/packager.ts) | ~300–500 token handoff and fresh-worker resume prompt, task+state+tests | Avoid treating self-reported completion as verified |
| [VOMEGA MP-54](https://github.com/owenservera/VOMEGA/blob/main/.project/pm/data/programs/MP-54.json) | Task/HEAD/evidence-derived context, measurable replacement of stale rereads; Ratchet packets | Development accelerator, not a second Personal World |
| [AION context-pack schema](https://github.com/owenservera/aion/blob/main/workspace/aion/agent-fabric/contracts/context-pack.v1.schema.json) | Scope, audience, source hashes/ranges, trust, freshness, omissions and manifest digest | AION contract is a reusable schema reference, not automatically installed in VIVIM |
| [AION bootstrap kit](https://github.com/owenservera/aion/blob/main/workspace/aion/shared/agent-bootstrap-kit/MASTER-BOOTSTRAP.md) | Four stages: role SITREP, shared intelligence, CANON/Immediate/Curated, skills | It's an instruction macro, not an installed daemon or CLI |
| [AION agentic file catalog](https://github.com/owenservera/aion/blob/main/workspace/aion/sessions/roles/15-product-to-market/core-library/AGENTIC-FILE-CATALOG.json) | 21 functional families, 133 candidate filename patterns, provenance categories | **Do not create 133 template files**. A reconstructed taxonomy is not a list of 133 standards |
| [AION living operating model](https://github.com/owenservera/aion/blob/main/workspace/aion/shared/agent-communications/LIVING-OPERATING-MODEL.md) | Agent-owned shared intelligence, per-role memories, non-ceremonial updates | Do not duplicate AION authority or copy private agent memories to a repo |

## C — Recovered from the attached local-agent session, not verified on GitHub main

- The local agent drafted docs/architecture/CONTEXT-LAKE-V1.md and a PRD 13 Context Lake: an incremental Git-derived event lake, attributed commits, staleness/contradiction detection, bounded brief, deterministic committed feed, guarded hooks and optional watch.
- The session proposed tools/context/lake.mjs, docs/context/context-feed.v1.json, docs/context/CONTEXT-FEED.md, and test slices CTX01–CTX06. Fetches for these new main-branch paths returned 404 at this checkpoint; inspect the **local working tree before recreating** them.
- Session evidence described local OpenCode global AGENTS.md, no repo-root OpenCode configuration, no first-action instruction for a plain greeting, a missing docs/session/current.md and conflicting ultracave invocation conventions. These are **agent observations**, not inspected current machine evidence by this pass. Run the local probes in FRESH-SESSION-BOOT.md.
- Session research concluded there is no authoritative *chat atom* schema or stable span/citation contract, despite note units, message metadata and journal digests. This remains a **gap hypothesis** until a fresh full-tree search checks it.

## D — Synthesis

**Five independent areas must be connected, not collapsed:** Git/development evidence (Context Lake), personal World and consent, canonical DCB projection, atomic chat segment retrieval, and CLI session boot. Write-only status artifacts do not create a working boot path. One live read test on the actual machine outranks a clean markdown handoff.
