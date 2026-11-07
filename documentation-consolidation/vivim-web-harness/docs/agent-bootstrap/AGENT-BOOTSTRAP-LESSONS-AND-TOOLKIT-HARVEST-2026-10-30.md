# VIVIM — Agent Bootstrap Lessons & Toolkit Harvest

**Date:** 2026-10-30  
**Track:** Independent agent-bootstrap research; separate from historical-platform IP, cross-repo capability harvest, mobile UX schema and active roadmap.  
**Status:** **SOURCE-VERIFIED TRIAGE + PROPOSED INTEGRATION**; no agent provisioning, tool installation, permission change or live activation performed.

## Executive finding

The user's earlier 100+ agentic-file research has a strong concrete match: **[owenservera/hermes-company-os](https://github.com/owenservera/hermes-company-os)** contains a 165-entry role roster (source blob `529090a7c60ffd4c1876a93a4090bb3a36b40cfe`), with a four-document-per-role design (`SOUL.md`, `ROLE.md`, `BOOTSTRAP.md`, `CAPABILITY-SEED.md`). This means **660 expected document references, not 660 independently inspected files, and not 660 functional tools**. The roster divides into **9 core / 15 leadership / 141 catalog** roles. The repo README explicitly calls it a **design and bootstrap pack, not a live Hermes installation**. Two exemplar role documents were independently opened; the other generated role files are referenced by convention and still need file-level verification.

The second key source is [AION's agent-bootstrap-kit](https://github.com/owenservera/aion/tree/main/workspace/aion/shared/agent-bootstrap-kit): actual tracked instructions for role resolution, SITREP, shared intelligence, three-tier memory and skills, with an agent/stage tracker. Its README explicitly says it is *instructions and copy/paste prompts*, **not an installed launcher or daemon**.

**Recommendation:** Give the VIVIM harness a **read-only catalog and selective installer/adopter** with real tests; **not** a 165-agent orchestration system. The 165-role source references are now indexed under [`catalog/hermes-companyos-role-seeds.v1.json`](./catalog/hermes-companyos-role-seeds.v1.json) for future work. `integrated:false` / `installed:false` are intentional.

## 1. Source-of-truth harvest map

| ID | Source / inspected artifact | High-value lessons | Verified boundary | Reuse decision |
|---|---|---|---|---|
| AB-01 | [Hermes CompanyOS roster](https://github.com/owenservera/hermes-company-os/blob/main/org/roster.json), SHA `529090a7...` | 165 unique roles across tiered activation; 660 expected per-role document paths | Roster parsed and counted; roles are not executable capability proofs | **Adopt metadata**, map roles into future VIVIM templates |
| AB-02 | [Hermes acquisition loop](https://github.com/owenservera/hermes-company-os/blob/main/docs/03-CAPABILITY-ACQUISITION.md) | Inspect → inventory → research → gap score → minimal skill prototype → approval → test → publish → improve | A design contract, no current Hermes host inspection | **Adopt pattern** as task workflow |
| AB-03 | [Hermes integration matrix](https://github.com/owenservera/hermes-company-os/blob/main/docs/13-INTEGRATION-MATRIX.md) | Built-in first, differentiate skills/MCP/plugins/tools, least privilege, adoption economics | All candidates explicitly uninstalled | **Adopt categorization and permission review** |
| AB-04 | [Hermes bootstrap operations](https://github.com/owenservera/hermes-company-os/blob/main/docs/14-AGENT-BOOTSTRAP-OPERATIONS.md) | Maturity states from CATALOG through TRUSTED/SUSPENDED; prove a first assignment; dormant roster | Design guidance, not live evidence | **Adopt maturity and measured activation** |
| AB-05 | [Hermes capability-request schema](https://github.com/owenservera/hermes-company-os/blob/main/schemas/capability-request.schema.json) | Requester, target profiles, kind, risk tier, review, tests and rollback | Schema is permissive (`additionalProperties:true`); not sufficient security gate | **Adapt**, validate more strictly in host |
| AB-06 | [AION Bootstrap Kit](https://github.com/owenservera/aion/tree/main/workspace/aion/shared/agent-bootstrap-kit) | Scoped identity → SITREP → shared intelligence → CANON/Immediate/Curated memory → skills; staged tracked progress | No independent scheduler or runtime; tracker records self-report and source checks | **Adopt compact startup lifecycle**, not AION-specific command syntax |
| AB-07 | [AION master bootstrap](https://github.com/owenservera/aion/blob/main/workspace/aion/shared/agent-bootstrap-kit/MASTER-BOOTSTRAP.md) | Resume missing stages, inherit prior results, avoid duplicate identities/authority | Instruction macro, not product command | **Adapt for phone bootstrap state**, never call it a daemon |
| AB-08 | [AION skills first pass](https://github.com/owenservera/aion/blob/main/workspace/aion/shared/agent-bootstrap-kit/prompts/04-SKILLS-FIRST-PASS.md) | One useful, tested role-owned skill over many speculative empty definitions | No implied Windows installation from committed source | **Adopt skill quality and evidence requirements** |
| AB-09 | [VIVIM Harness OpenCode agent](https://github.com/owenservera/vivim-harness/blob/main/.opencode/agents/local-opencode.md) | Reproduce from live receipts, trace full CLI→browser→provider path, preserve send-once/profile ownership | Development instruction, not a shipped reusable skill | **Adapt as debugging/evidence skill** |
| AB-10 | [VOMEGA product knowledge](https://github.com/owenservera/VOMEGA/blob/main/seed-docs/README.md) | Canonical product seed vs archived bootstrap history; inherited truth != inherited solution | Different sovereign semantic product; quick tests not provider proof | **Reference and selectively reuse** |
| AB-11 | [VIVIM personal first-user decisions](https://github.com/owenservera/vivim-harness/blob/main/docs/product/FIRST-USER-ONBOARDING-DECISIONS-2026-10-27.md) | User chooses personal workspace; Chrome profile as World; accounts, privacy and source integrity | Decisions include parked future design; no assumption of implemented behavior | **Preserve requirements** |
| AB-12 | [OpenCode agent permissions](https://opencode.ai/docs/agents) and [skills](https://opencode.ai/docs/skills) | Task-specific permissions; discover `SKILL.md` on demand, not dump every skill into context | Runtime config is version-sensitive; docs != installed CLI | **Adopt portable skill strategy**, probe host |
| AB-13 | [Agent Skills specification](https://agentskills.io/specification) | Portable `SKILL.md` with metadata, optional scripts/references/assets; progressive disclosure | Portable *format*, not authorization, install or execution | **Use for selected converted skills** |

### Evidence gap on the exact 100+ corpus

A prior Hermes CompanyOS packaging exercise reported around 700 files; current GitHub source demonstrates **165 role definitions with four expected doc references each**, plus shared documents, but this pass did not enumerate the repository's whole git tree to independently certify a total file count. Do not advertise “700 verified tools.” Other prior agent-document taxonomies (e.g. agent instructions, states, plans, evidence, handoffs, recovery and skills) informed this map, but a complete original 21-family source artifact has **not** been independently located in this GitHub sweep.

## 2. Distinguish the seven things commonly called “tools”

| Class | What it is | Harness behavior before activation |
|---|---|---|
| Role template | Identity, expertise, responsibilities and task categories | Catalog and preview only |
| Procedure / skill | A task-specific `SKILL.md` with triggers, steps and constraints | Display metadata, run tests before offering |
| Native harness capability | Existing validated browser/host commands | Expose via verified capability manifest |
| External CLI / binary | Installed executable with version and OS context | Read-only detection; explicit install review |
| MCP server | Remote/local tool surface and protocol access | No implicit network/credential grants; inspect tool allowlist |
| Plugin | Extends another runtime, potentially executable hooks | Pin version, review code, permissions and rollback |
| Knowledge/memory pack | Sourced references, runbooks, source-specific memory | Local index with expiry/provenance; never secretly elevate to policy |

**Never conflate templates with active agents or documentation with available tools.** A role may need zero additional plugins; user value and usable demonstrations determine what to enable.

## 3. The agent bootstrap lifecycle VIVIM should adopt (PROPOSED)

**Stage 0 — Resolve:** identify user, paired host, personal World, agent role, and explicit authority. No duplicate bootstrap or arbitrary SOUL creation. Offline state is truthful.

**Stage 1 — Orient:** retrieve local CANON (stable instructions/constraints), Immediate (recent status/next actions), Curated (verified reusable knowledge), the role's actual last state, working provider capabilities and blocked dependencies. **Do not copy huge shared histories into every agent.**

**Stage 2 — Discover:** read-only inventory: signed-in web providers, models/effort, local harness tools, installed SKILL.md, MCPs/plugins *in their actual runtimes*, authorizations and workspace constraints. Classify `reported`, `found`, `reachable`, `verified`, `enabled` separately.

**Stage 3 — Diagnose:** compare real task needs with existing tools; score candidate improvements by value, expected frequency, integration risk, maintenance, cost, testability, duplication and reversibility. Prefer the smallest increment that helps the user today.

**Stage 4 — Propose:** show cards on the mobile Build canvas (clear benefit, source, permissions, provider, risk, cost, and undo limits). The AI may recommend, **never self-authorize** a new install, provider send, filesystem mutation or public Share.

**Stage 5 — Activate:** owner approves exact action/target, host revalidates permissions and trust, performs bounded changes, and records an idempotent receipt. Provider interactions continue through approved signed-in webapp UI, **no model APIs**, one browser controller, no uncertain resend.

**Stage 6 — Demonstrate:** execute one falsifiable acceptance workflow under real host conditions (and negative tests), attach evidence. Distinguish `source_present`, `locally_tested`, `live_verified`, `trusted`.

**Stage 7 — Evolve:** promote a proven skill with owner-defined scope, memory/procedure updates and documented costs. Store resumption state; refresh only when task, failure, version change or approved cadence justifies. No 165 simultaneous workers.

**Stage 8 — Recover and retire:** handle disconnected host, stale providers, changed skills and owner revocation. Suspension and removal of capability are separate from deleting the user's local knowledge history.

## 4. Proposed compact VIVIM runtime object model (not wired)

One local source of truth for bootstrap state; the handset displays a projection. **Do not invent a second approval or provider router.**

```json
{
  "schema": "vivim.agent-bootstrap/1",
  "agentId": "personal-researcher",
  "templateRef": {
    "repository": "owenservera/hermes-company-os",
    "sourceRoleId": "research-scientist",
    "revision": "PIN_THE_VALIDATED_SOURCE_COMMIT"
  },
  "worldId": "local-world-id",
  "stage": "discover",
  "maturity": "CATALOG",
  "capabilities": [],
  "permissions": { "requested": [], "granted": [] },
  "lastReceiptRef": null,
  "nextAction": "Inspect verified host capability manifest"
}
```

This is illustrative only, **not** a validated reference to a verified `research-scientist` role. Role IDs and source revisions must be selected from the actual 165-role index; runtime host data must not be inferred from template seeds.

Minimum fields when formalized: `agentId`, `templateRef`, `worldId`, `bootstrapStage`, `maturity`, `capabilityRefs`, `grants`, `skillRefs`, `modelPreferences`, `costPolicy`, `evidenceRefs`, `configRevision`, `lastSITREPAt`, `nextAction`, `suspendedReason`. All external writes must carry the existing host journal/approval refs; never store raw credentials, cookies or private browser transcripts in public files.

### Maturity mapping

| State | Meaning | Trust/rights |
|---|---|---|
| `CATALOG` | Source references only | None |
| `PROVISIONED` | Local profile/World object exists | No new external authority |
| `CONNECTED` | Actual model/tool connection read-only verified | Read-only unless separately granted |
| `BOOTSTRAPPED` | Stage pack and first skills produced with evidence | Bounded tests only |
| `OPERATIONAL` | Completed one authorized real task with receipt | Approved per-action operations |
| `TRUSTED` | Independent validation plus explicit scoped grant | Only exact allowed categories |
| `SUSPENDED` | Stale, revoked, unsafe or failed | No action until revalidated |

## 5. Which lessons belong in VIVIM Gate 1?

| Order | Candidate package | Benefit to mobile bootstrap | Source | Integration target | Evidence gate |
|---|---|---|---|---|---|
| **G1-A** | Capability introspection and honest readiness | “What can my AI do?” with verified status | AION 01 + Hermes 02 | `wh caps` + phone capability manifest | Host read-only provider/tool probe, timestamp/TTL |
| **G1-B** | Bootstrapping with continuity | Start, resume and recover without repeating setup | AION tracker/master | Build onboarding stage store | Two sessions / host restart / no duplicated activation |
| **G1-C** | Role template browser | Pick personal researcher, builder, reviewer, organizer | Hermes roster metadata | Local toolkit catalog → Build proposals | Actual role IDs, no installation implied |
| **G1-D** | Consent/authority profile | Understand what each agent may read/send/change | Hermes integration matrix | Existing harness approval layer | Host rejects tampered client intent / unapproved action |
| **G1-E** | First task proof | One useful AI result with source and journal | AION evidence + VIVIM Harness | Bootstrap completion and SITREP | Real phone→host→signed-in provider return |
| **G1-F** | Skill-pack preview | See what skill a role could gain | AION 04 + Agent Skills | Local SKILL.md metadata only | Parse + validate + version + permission display |

### Later, not Gate 1

- Auto-generating 165 roles or activating nine default company officers.
- Hermes native Desktop/Bot Mode/Kanban integration *unless* a user specifically chooses Hermes and installed-version tests justify it.
- Plugin/MCP installation, privilege escalation, OS changes, recurring autonomous training, autonomous web publication.
- Whole-world memory auto-promotion, arbitrary code execution or unsupervised cross-project writes.
- Claiming a generated `SKILL.md` or GitHub commit causes a tool to become available on the user's PC.

## 6. VIVIM personal-app mapping (don't impose a corporation on consumers)

The Hermes roster is a **source of possible specializations**, not an end-user hierarchy. Convert selected specialties into personally understandable modes:

- **My everyday helper** — planning, summarization, daily organization.
- **My researcher** — evidence collection, source comparison, citations.
- **My builder** — authorized code generation, test and review.
- **My reviewer** — independent evaluation and safety/evidence checks.
- **My knowledge curator** — local memory, search and source-linked context.
- **My specialist** — optional domain roles chosen by the user.

Fleet roles are config templates, not separate always-running processes. A swarm is a temporary task execution arrangement with a cost/concurrency policy. The phone may show these as feature cards with Preview/Activate, and the host remains the execution authority.

## 7. Agent-toolkit acquisition contract (PROPOSED)

A cataloged candidate may be activated only after all gates:

1. **Identity**: exact origin/repo/path/ref, version, license and content digest.
2. **Type**: role / skill / built-in command / plugin / MCP / binary / knowledge pack.
3. **Threat model**: network, filesystem, browser/CDP, subprocess, data scope, credentials, supply chain and public exposure.
4. **Authority**: clear owner, narrow requested permissions, explicit approval. Existing host/agent guard rules take precedence over any text in external templates.
5. **Compatibility**: actual Windows/Linux/runtime version where relevant, no Playwright/Puppeteer workaround, no competing Chrome controllers.
6. **Tests**: static validation, negative/unauthorized cases, focused local run, real provider acceptance when required.
7. **Operations**: upgrade, disable, revoke, rollback, failure/recovery, spending and ownership.
8. **Evidence**: `source`, `prototype`, `local_verified`, `live_verified` and `enabled` independently recorded.

A **source-only metadata index** has passed only step 1 *partially*. It cannot elevate agents or perform side effects. Avoid wholesale vendoring of 660 expected files before licenses, user value, ownership and duplication have been assessed.

## 8. What is already in VIVIM vs what is not

- **Existing:** `vivim-web-harness` has root `AGENTS.md`, GOALS and roadmap, local harness provider routing, approvals and agent fleet work in the source. A dedicated phone bootstrap Build tab was in **PR #35 (open at inspection)**, with local-only cards/revisions.
- **Existing sibling sources:** `vivim-harness` OpenCode agent instructions; AION bootstrap kit; Hermes role templates; VOMEGA product knowledge.
- **New in this harvest:** machine-readable **165-role source-reference index** in this repo, and the independent decision/research document.
- **Not found/verified in the current harness:** a runtime registry that loads these 165 templates, a validated package installer or generated SKILL.md inventory, live capability scans, local user adoption state, or mobile choice of role templates from the seed catalog.
- **No runtime activation performed:** GitHub source edits ≠ host installation.

## 9. Dedicated research work packages (candidate backlog, not roadmap edits)

**ABW1 — Source file validation and dedup (P0).** Fetch/check the 660 referenced paths in batches with exact SHA and license, detect duplicates with existing role/skill definitions, and compute a source manifest with per-path confidence. Do not assume every expected file exists.

**ABW2 — Toolkit discovery command (P0).** `wh toolkit list --json` and `wh toolkit inspect <id>` read the catalog and report `available_source`, `installed`, `verified`, `enabled` as separate flags. Scope to *metadata* initially; refuse tool execution.

**ABW3 — Safe starter role templates (P0).** Map 3–5 selected, personally useful role specializations from actual source IDs to VIVIM-specific plain-language cards. No full corporate org chart.

**ABW4 — Progressive bootstrap state (P0).** Adopt AION-style stage receipts and CANON/Immediate/Curated memory with clear owner-approved promotion; source-of-truth stays in the local World. Resume midstage deterministically.

**ABW5 — Skill admission (P1).** Portable `SKILL.md` lint/validator, signatures/hashes, permission budgets and sandbox testing, then explicit enablement, with no auto-download of executables or unknown MCP servers.

**ABW6 — Phone-first evaluation (P1).** On a real phone with a verified paired host: choose researcher → preview skill needs → approve minimal setup → complete one reviewed webapp-native task → restart/resume → see evidence → disable agent.

**ABW7 — Lifecycle review (P2).** Capability aging, cost, evaluation, source refresh and retirement. Dormant entries never consume runtime/model tokens.

### Acceptance matrix

| Package | Pass | Fail |
|---|---|---|
| ABW1 | 165 unique source roles reconciled, linked to verified file statuses | Count invented or paths silently assumed |
| ABW2 | Catalog lists metadata without running tools | Listing instantiates 165 agents or marks them enabled |
| ABW3 | Simple personal role cards from verified source IDs | Enterprise org mandated at setup |
| ABW4 | Reconnect resumes progress without duplicate sends/actions | Bootstrap starts over or rewrites past state |
| ABW5 | Only signed/approved/tested skills become enabled | `SKILL.md` grants host permissions by existence |
| ABW6 | Real phone receipts demonstrate first useful task | Mock-only or source-only verification shown as live |
| ABW7 | Expired unsafe capability suspended without deleting history | Unverified automations keep running |

## 10. Scaffold review relationship

[Mobile Bootstrap Scaffold Review](../ux/MOBILE-BOOTSTRAP-SCAFFOLD-REVIEW-2026-10-30.md) contains **two blockers** (Talk draft overwrite and CSS-hidden Build view) and other findings. **Resolve those before coupling the new toolkit to UI card activation.** An unbacked `B.enabled` toggle is not an agent installation or toolkit activation.

## 11. Source review scope and next decision

This harvest inspected the Hermes README, actual 165-role roster, acquisition/integration/bootstrap operations docs, schema and two Chief-of-Staff example files; AION bootstrap-kit README/master/tracker/skills stage; VIVIM harness agent instructions; VOMEGA seed README; the VIVIM Web Harness AGENTS/GOALS/roadmap; mobile PR #35 diff; and official OpenCode/Agent Skills documentation. It **did not** read all 660 expected per-role files or validate 165 executable capabilities. Some original agent-tool research may still exist only in earlier conversation artifacts or private local archives.

**Proposed next call:** Assign ABW1 and ABW2 once the local agent has digested the existing cross-repo harvest. This document intentionally creates no new roadmap IDs, queue ownership, live agent sessions, host operations, or runtime permissions. The existing authoritative `docs/roadmap.json` remains unchanged.
