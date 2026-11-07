# Dynamic Context Bundles (DCB) — recovered architecture and integration review

**Date:** 2026-10-31  
**Status:** design/research recovery; no new runtime implemented by this document.  
**Purpose:** decide how the legacy VIVIM DCB engine, VOMEGA MP-54, AION ContextPack and current VIVIM harness should converge. This is a *context projection* mechanism, not a new memory authority, router, scheduler, tool grant or independent browser operator.

## Executive decision

**Adopt one on-demand, deterministic and inspectable bundle composer with separate consumption policies for personal conversations and development agents.** Start with a thin **development-task bundle** sourced from the existing roadmap/capability map and Git/receipts; add **owner-scoped personal bundles** as a projection of the current `primary` memory, World/VAWS evidence and local chats after consent/sensitivity review. Neither requires replacing the current JSON authorities with SQLite. SQLite can later provide searchable evidence/claim indexes but should not become a gate for basic Build or Talk.

One composer = source resolution + authorization + evidence/freshness checking + ranking/budgeting + manifest/projection. Separate profiles/capability policies distinguish what may cross a provider boundary. It must not independently rediscover the codebase when a canonical dependency map/source index exists.

## Source-verified inheritance

1. **Legacy VIVIM Final:** `vivim-final/master/src/engines/{dcb-projector,dcb-profile,cortex-budget}.ts` define `DynamicContextBundle`, ten logical layers, confidence/recency-ranked items, eight profiles, a budget allocator, and projections to injected prompt, system message, capsule and panel card. The tests in `tests/unit/engines/dcb-projector.test.ts` validate these projectors. Implementation is **in another repository**, not proof of availability in current `wh`/Studio.
2. **VOMEGA MP-54:** `VOMEGA/main/.project/pm/data/programs/MP-54.json` and generated `PROGRAMS/MP-54.md`: context must be derived from task/gates, Git HEAD/diff, decisions, relevant tests, evidence/failures, not an agent's own summary. The Ratchet already emitted task packets there. Preserve `MP54-G1` (fresh-worker onboarding), `G2` (same task+HEAD deterministic file set), `G3` (bounded relevance), `G4` (adaptive vs static), `G5` (validated progress improvement). These MP-54 gates are **legacy plans**, not completed current-harness tests.
3. **AION Agent Fabric:** `aion/main/workspace/aion/agent-fabric/contracts/context-pack.v1.schema.json` defines audience principal/project/deployment, task/work, role, source URI/hash/classification/trust/sensitivity/freshness/ranges, token reserves, omitted-source reasons and manifest digest. `workspace/aion/handoffs/AGENT-FABRIC-M3-M5-IMPLEMENTATION.md` gives verifier/renderer/CLI design; explicitly says specification, not installed runtime.
4. **Current harness:** `studio/src/agents/personal-agent-continuity.mjs` already makes *local private continuity packets* from CANON/Immediate/Curated and current Work; `studio/src/store/versioned.mjs` owns primary revision state; `harness/src/dev/agent.mjs` has a bounded webapp-native tool loop; `docs/roadmap/CAPABILITY-DEPENDENCY-MAP.v1.json` describes capability/dependency/flow provenance; `docs/agents/PERFORMANCE-OPTIMIZER.md` PERF04 describes context-economics policies. These are fragments, **not a unified DCB engine**.

## The ten original logical layers

| Original layer | Meaning | v1 supply source |
|---|---|---|
| `L0Identity` | Subject identity | Explicit `primary` identity *only when relevant and authorized* |
| `L1GlobalPrefs` | Preferences/instructions | Owner-confirmed CANON/preference scope; not all personal preferences |
| `L2Topic` | Topical background | Relevant Curated memory, local World search, source anchors |
| `L3Entity` | Key entities and relations | Existing World/TemporalGraph nodes, verified capability references |
| `LpProjectState` | Current project/work | Task ID, code HEAD, host status receipts, branch/PR evidence |
| `LdDecisions` | Decisions and constraints | Relevant owner decisions, policy, exact acceptance constraints |
| `L4Conversation` | Conversation arc | Authorized current/native conversation or summarized local history |
| `L5JitContext` | New/relevant cross-app information | Local retrieved source excerpts only after destination filtering |
| `L6RecentHistory` | Recent activity | Verified recent attempts, failure capsule, Work status, narrow message ranges |
| `L7UserQuery` | Latest explicit user request | Sent separately as task/prompt; never silently rewritten by DCB projection |

Keep layer names for legacy compatibility, but do **not** enforce fixed per-layer minima that crowd out the task or mandate identity in technical prompts. The budget solver is a candidate to adapt, not copy unquestioningly. Explicit owner instructions and safety constraints are distinct from retrieved context; model-generated snippets are always untrusted data even when displayed within a prompt.

## Eight inherited profiles (profiles are *selection policies*, not automatic disclosures)

- `seed`: minimal first interaction, optional explicit owner context + current request.
- `reunion`: reestablish an existing user's prior context after a gap.
- `convergence`: compare recent context across authorized sources.
- `continuum`: resume durable conversation/project continuity.
- `handoff`: transfer bounded relevant state to another model/agent; **must obtain destination-specific disclosure authority**.
- `probe`: source-backed retrieval and discovery with no effect authorization.
- `deep_research`: broader source and evidence references with hard scope/budget.
- `decision_brief`: current decision, options, constraints, evidence and disputed/unknown facts.

For `wh dev`, begin with a **task-handoff projection** of `handoff` rather than a ninth mandatory profile. For phone chat, a practical `continuum` profile can show the user a preview of the context that is about to cross the webapp boundary.

## Composer design

```text
user request / task ID / receiving provider / principal / project / permission scope
                  |
                  v
resolve source refs using existing authorities
  Git HEAD and diff + roadmap/capabilities + Work/primary + World + received evidence
                  |
                  v
verify hash/freshness, relevance, audience ACL, source trust, sensitivity
                  |
                  v
rank, deduplicate, budget with reserved user prompt + output budget
                  |
                  +--> immutable local manifest (includes + omissions + digest)
                  |
                  +--> read-only preview/projection: prompt fragment, agent handoff, capsule, panel
                  |
                  v
*existing* owner-approved router.sendOnce / Studio capability path
                  |
                  v
journal receipt binds bundle digest to request ID + exact native thread ID
```

The **composer never sends**. Only the existing browser/router, authorization, thread and journal owners do. No second CDP controller, no public host port and no new permissive tool capabilities.

## Minimum v1 manifest (illustrative shape, not an installed schema)

```json
{
  "schema": "vivim.dynamic-context-bundle/1",
  "id": "dcb-task-opaque-id",
  "profile": "handoff",
  "purpose": "dev_task",
  "taskRef": "DEV01",
  "audience": {
    "principalRef": "owner",
    "projectRef": "owenservera/vivim-web-harness",
    "destinationProvider": "chatgpt",
    "destinationModelVerified": false
  },
  "sources": [
    {
      "ref": "docs/roadmap.json",
      "revision": "git-commit-or-content-sha",
      "ranges": ["L1-L30"],
      "classification": "task_evidence",
      "trust": "observed",
      "sensitivity": "internal",
      "freshness": "version_pinned"
    }
  ],
  "omissions": [
    {"ref": "private-personal-memory", "reason": "denied_by_scope"}
  ],
  "budget": {
    "maxInputTokens": 1200,
    "reservedForUserPrompt": 400,
    "reservedForOutput": 600,
    "estimatedIncludedTokens": 0
  },
  "sourceFingerprint": "sha256",
  "manifestDigest": "sha256"
}
```

The numbers above are **example limits**, not observed provider context capacity. Actual allowance must honor the target's *verified* send envelope and the host's hard payload size; when unknown, fail closed or use a conservative configurable limit. The original user request and owner instruction channels remain separate from the projection. Stable bundle digests exclude wall-clock-only volatility and bind canonical ordering, source digests, permissions and policy version.

## Component interfaces and correlation

| Producer | Context fragment | Receiving component | Hard dependency | Explicit boundary |
|---|---|---|---|---|
| Roadmap/capability graph + Git/Work | Task, owner lane, dependencies, acceptance, relevant files/tests | `wh dev` | `DEV01` source, roadmap + workspace read | No raw arbitrary SQL/shell or hidden auto-send |
| `primary` CANON/Immediate/Curated | Owner-directed relevant memory/identity | Studio Personal AI, Build | Existing `primary` and versioned store | User-confirmed > provider-reported; no secret transfer |
| World/VAWS records | Source-qualified entity/relations/claims | DCB resolver | Existing World service; optional W02/W03 indexing | Historical/rejected claims labeled, no forced certainty |
| Studio conversation + exact thread | Current conversation arc, authorized retrieved excerpts | Harness/phone Talk | H07 verified provider binding, C03 journal | Cross-provider data sharing is explicit and visible |
| Relay mobile command | Intent/task and destination context request only | PC local composer | P02/Q19 + private host | **Do not put private DCB content in Vercel relay payload** |
| `wh fleet`/developer reviewer | Task- and role-specific handoff | Existing router/worker | Q02/DEV02 | Child context is the permitted subset, not parent full memory |
| Browser Companion | Bounded tab presence metadata | Host state/hints | E01/E02 | Observation only; not logged-in proof or user context |

**Trace keys:** `bundleId` / `manifestDigest` → `taskRef` / `workRef` → `requestId` → provider native thread binding → journal receipt; for mobile `conversationId` → cloud command ID → local bridge ID → request ID. Keep the private joined correlation only on the PC; use redacted status/digests on phone and public relay.

## v1 commands / surfaces (proposal, no implementation)

- `wh context plan --task DEV01 --profile handoff --json`: source candidates, estimated token cost, trust/freshness and omissions. **Read-only**.
- `wh context preview <bundle-id>`: locally preview authorized material. No model send.
- `wh context verify <bundle-id>`: source hashes and permission scope still valid. Fail on stale/private/denied.
- `wh dev --context <bundle-id>`: after owner-approved or scoped standing approval, have the existing router submit the same verified projection once. Persist digest in dev job/journal record.
- A later Studio-hosted `/personal-agent/context/preview` returns a **bounded private owner-view preview**. The cloud phone receives an opaque reference/limited projection, not entire personal memory.

Prefer reuse of current Studio continuity packet and work-run store, `harness/src/dev` and AION-like source manifest. Avoid another background synchronizer, new memory tiers or independent schema/graph analyzer.

## Staged acceptance gates and blockers

**DCB1-G1 Thin Dev Bundle (independent of SQLite):** same task+Git SHA+permissions yields byte-stable manifest; known task/acceptance/decisions, named source lines, relevant tests, current failure; included and omitted refs; no secret spill; can start `wh dev` without broad repo reread. Negative tests: denied files, symlink traversal, stale commit, changed audience, oversized send.

**DCB1-G2 Verified downstream correlation:** one real existing signed-in browser task uses only the existing router and send-once journal; durable bundle digest + request ID + exact provider thread ref. Interrupted uncertain send is reconciled read-only, never repeated; no alternate controller.

**DCB1-G3 Personal projected bundle:** explicit owner + provider scope, no raw private records through cloud relay, user-confirmed facts never displaced by hypotheses; changed `primary` revision invalidates existing projection. Phone can preview what will be shared, skip transfer, or ask for a reduced disclosure.

**DCB1-G4 Source-aware selection:** when W02 stable World IDs or W03 SQLite evidence index exists, use it as a source resolver, not a second index. Test disagreements, missing sources, policy denial and stale citation.

**DCB1-G5 A/B quality gate:** compare same bounded tasks with plain task brief versus DCB; track prompt bytes/token estimates, startup time, time to first correct action, test correctness, source citation validity, retry/uncertain incidents and missing-important-source rate. Keep complexity only if it produces measured better outcomes. Do not confuse context compression with higher-quality decisions.

**Do not gate** Gate 1 phone bootstrap, first usable Build, or current working Talk on DCB G4–G5, SQLite, MP-21/MP-55/MP-60, or optional retained Elephant domain sessions.

## Roadmap placement without parallel implementation conflicts

- **P0, protect existing experience:** HOST01, P05, RG01, H07; no DCB requirement for basic Talk.
- **P1, thin bundle:** one new `DCB01` owner under development lane L6 (actual implementation), borrowing PERF04 performance contracts and existing `wh dev` workspace. No writes into another agent's active file without handoff.
- **P1, personal and source-aware:** `DCB02` after DCB01, H07/primary consent and W02 source IDs; source-specific selection optional, W03 evidence index later.
- **P1, validation:** PERF04 A/B measures the benefit; DATA02 contributes ontology mapping. No competing VAWS, memory service, context pack or roadmap state authority.

**Core principle:** dynamic bundles are *temporary, reconstructible, evidence-qualified views of the user's evolving world and the live task*, not stored biographies or rights to execute effects.
