# PWA-visible outcome tracker — R001 (review draft)

**Created:** 2026-11-21 · **Repository:** vivim-labs / existing DevOps-Planning folder · **State:** REVIEW / UNCLAIMED CANDIDATES, not an owner-approved delivery queue.

**Scope:** Reorganization of the conversation-only 79-item list. This file is a **review view**, not a new authoritative OWNER-WORK registry or a replacement for [TASK-CLAIMS](../TASK-CLAIMS.md). Preserve all existing Dev ownership/check-in records; no new OW IDs, source edits, assignments, or deployments occurred through this planning pass. Merge selected outcome items into the genuine canonical writer when verified.

## Non-negotiable admission rule

1. A **deliverable** is valid only when an owner can perform an action on the **same PWA URL** and visually confirm the result or failure state. Draft code, a cargo PASS, a Wiki document or a git commit are **not** acceptance alone.
2. A **dependency** is valid only if it identifies exactly which PWA deliverable it unlocks, has a bounded contract, and can be verified through that deliverable. Do not promote infrastructure/platform work as a standalone deliverable.
3. Everything else is **PARKED / OUT OF ACTIVE QUEUE**, even if technically attractive. Native GPUI, Android APK, provider plumbing and general architecture refactors do not count unless and until tied to a concrete PWA interaction.
4. Automatic ambient Wiki lookup is read-only. **Explicit submission** is mandatory before creating a task or executing an effect. Reuse existing Host, Rust, HQKG, Wiki and claim system; no parallel registry, action authority, graph, model router or agent bus.
5. **Acceptance receipts per outcome:** same PWA URL and deployed SHA, user input/action, visible before/after (or denial), actor/grant/source revision where relevant, real tested execution state, failure/undo. A browser PASS and phone-browser PASS are separately labeled; Android native is not a prerequisite.
6. **Ownership:** ChatGPT Dev retains source implementation of two AutoWiki goals and its existing checked-out bugs. Planning may shape these and prepare independent packets without claiming Dev paths. Claim protocol is [TASK-CLAIMS](../TASK-CLAIMS.md) with explicit acceptance. Oracle shapes outcomes; it does not take over assignments.
7. Every outcome/component below is **PROPOSED** until owner review. P0 is product sequencing priority, *not* evidence of active work, an assigned owner or an implemented milestone.

## 1. Product-facing outcome queue

Status of every row: `REVIEW` (not production acceptance). PWA tests describe proposed acceptance, **not completed demonstrations**.

| Outcome | Priority | User-visible deliverable | PWA test / observable action | Conversation source | Ownership note |
| --- | --- | --- | --- | --- | --- |
| **PV-01** | P0 | Human typing opens AutoWiki | On the unchanged PWA link, type an unfinished sentence; Wiki visibly opens/updates without pressing Search. Clearing/changing text refreshes or dismisses stale results. | P01 | Dev owns implementation; review item only |
| **PV-02** | P0 | Live agent typing opens the same AutoWiki | Trigger an authorized signed-in ChatGPT response; letters appear progressively in the PWA and the SAME Wiki reacts to the agent text, with source visibly labeled. | P02 | Dev owns implementation; review item only |
| **PV-03** | P0 | Stable-link PWA release and visible diagnostics | Reload the same PWA URL after a build; see revision/connection state, run a smoke check, view failure reason and recover or roll back without replacing the link. | P17 | Review |
| **PV-04** | P1 | Living AutoWiki for real knowledge | Open generated Wiki pages and search; real persisted context, navigation, live updates and source links appear; no synthetic filler. | P05,P12,P23 | Review after PV-01/PV-02 |
| **PV-05** | P1 | Wiki freshness, trust and corrections | Change/revoke source knowledge; PWA updates or invalidates the affected claim with revision, source, freshness and a user-visible unknown/denied state. | P13 | Review after PV-04 |
| **PV-06** | P1 | Natural-language actions with approvals and undo | Ask the PWA to do a safe, authorized action; preview, approve/deny, see execution receipt, and undo or recover. Typing alone never triggers an effect. | R01,R03,R16 | Review; requires Host authority |
| **PV-07** | P1 | Pairing, connection and grant controls | Pair/connect, inspect grant state, revoke, lose connection, reconnect; PWA visibly shows transitions and does not leak protected data. | P15 | Review |
| **PV-08** | P1 | Editable responsive PWA visual runtime | Change a visible layout/theme/element via approved UI action; PWA renders the change, saves revision, restores and undoes through existing VEC. | P22,R09 | Review |
| **PV-09** | P2 | PWA Studio/Atlas for signed-in web apps | Open a real Claude.ai surface capture in the PWA Studio Atlas; inspect controls/states and run one explicitly approved supervised action with provenance. | R25 | Review; not a Craft runtime replacement |
| **PV-10** | P1 | Live activity timeline and recoverable replay | See user/agent typing and permitted actions in an ordered PWA timeline; select an event to inspect its source and recover from a failed step. | P19 | Review |
| **PV-11** | P2 | Verified developer knowledge in the PWA | Open a source-linked Wiki/Dev view; verified claims show receipts, unsupported claims show unknown, and semantic slots navigate to grounded source. | R21,R23 | Review |
| **PV-12** | P2 | Reusable workflows surfaced in the PWA | Repeat an approved workflow; PWA offers a verified reusable recipe, execution/undo controls and measured deterministic vs AI-assisted cost where known. | R28,R29 | Review |
| **PV-13** | P2 | Private sharing, revoke and access feedback | From PWA share one authorized knowledge item privately, inspect access, revoke it and visibly confirm denial after revocation. | R20 | Review |
| **PV-14** | P1 | Request a product change from the PWA and see progress | Explicitly submit a build/change request, get one durable request ID, see planned/accepted/coded/tested/released/blocked status and any MANUAL bridge; cancel/retry safely. | D01,D02,D11 | Oracle-aligned candidate; no automatic agent dispatch claimed |

### Candidate sequence, not a second queue

- **First finish line:** PV-01 -> PV-02. These are Dev-owned, currently NOT accepted as live PWA DONE.
- **Make those testable:** PV-03; support them with C03/C04/C06/C12/C34 only to the extent needed.
- **Expand usable product:** PV-04 -> PV-05, PV-07, PV-10 and PV-06.
- **Close owner-to-product loop:** PV-14. A durable PWA build request is distinct from ambient Wiki search and must not silently launch an agent.
- **Later, only after the above:** PV-08, PV-09, PV-11, PV-12, PV-13.

## 2. Bounded dependency components (never independent projects)

**Status for all rows:** `REVIEW / UNCLAIMED` as planning candidates. A listed existing Dev implementation may already have a separate live claim; do NOT claim it again. Each component's *visible acceptance* is inherited from its `Parent` outcome; no detached platform completion.

| Component | Parent PWA outcome | Bounded component / what it unlocks | Original 79-item IDs |
| --- | --- | --- | --- |
| C01 | PV-01 | Rust/Host-backed PWA composer admission and transport | P03,P09 |
| C02 | PV-01 | Incremental Wiki query/slot update from human input | P04 |
| C03 | PV-01 | Repair nonresponding composer and universal-dock Wiki mount regression; test it live | P16 |
| C04 | PV-01,PV-02 | Desktop-browser PWA acceptance scripts and exact human/agent frames | P20,P24 |
| C05 | PV-02,PV-09 | Signed-in web-app interaction adapter, no provider API | P06 |
| C06 | PV-02 | Real paced agent typing frames and text/event capture | P07,P11 |
| C07 | PV-02 | Single actor-labeled revision/event pipeline for human and agent Wiki projection | P08,P10 |
| C08 | PV-04 | Reuse D4 persisted knowledge and existing HQKG identities/authorized bounded retrieval | P05,P14,R07,R08 |
| C09 | PV-04 | Second-screen Wiki displays updates from real state | P12 |
| C10 | PV-05 | Revision fences, invalidation, dedup, source provenance and revocation presentation | P13 |
| C11 | PV-07 | Host/PWA pairing, grants, reconnect and revocation plumbing | P15 |
| C12 | PV-03 | Stable URL release, PWA-visible revision and client errors | P17,P18 |
| C13 | PV-10 | Ordered user/agent/action event timeline and receipts | P19 |
| C14 | PV-03 | Optional actual phone-browser PWA proof; not native Android/APK prerequisite | P21 |
| C15 | PV-08 | VEC/TierFlow live visual diffs, revision/session/grant coherence | P22,R09,R10 |
| C16 | PV-04 | Intelligent Wiki search, hub navigation and suggestions grounded in data | P23 |
| C17 | PV-06 | Existing Rust fabric/Host permission, identity, action receipts and recovery reuse | R01,R02,R16 |
| C18 | PV-06 | Native intent DAG, command planning and source-grounded execution parity | R03,R04 |
| C19 | PV-06 | Personal rules: propose, remember, revoke, rollback with visible receipt | R05,R06 |
| C20 | PV-09 | Rust-capable Studio/Host integration, capture, source-aware send and recipes | R14,R15,R17 |
| C21 | PV-09 | Retire superseded JS paths only after real PWA Studio/Host parity | R18 |
| C22 | PV-08,PV-09 | Scope file-level Rust migration disposition only for capabilities entering those PWA views | R19 |
| C23 | PV-13 | Private-share grant/integrity implementation and visible revoke result | R20 |
| C24 | PV-11 | Native VLANG declaration/compiler/independent claim verifier behind developer Wiki | R21 |
| C25 | PV-11 | Only PWA-facing source navigation/diagnostics integration; LSP/MCP-only work stays parked | R22,R23 |
| C26 | PV-09 | CraftAnything read-only capture/import/Atlas adapter for Claude surface; reuse, don't replace, Studio | R24,R25,R26,R27 |
| C27 | PV-12 | Supervised agent observation, approved action, feedback and recovery contracts | R28 |
| C28 | PV-12 | Evidence-backed interaction logs to deterministic Rust recipes with visible savings | R29 |
| C29 | PV-03 | Required native runtime discovery, package and serving chain for the SAME PWA URL | R30 |
| C30 | PV-14 | Find/reuse canonical task source; enforce claims, ownership, check-out/in, leases and honest progress states | D01,D02,D03,D04,D06,D14 |
| C31 | PV-14 | Roles, SITREP last-five/next-ten, source-to-PWA work status and Dev ownership boundaries | D05,D07,D08,D10 |
| C32 | PV-14 | Oracle shaping, Planning source-pinned work packets and no duplicate authority | D11,D12,D13 |
| C33 | PV-14 | Dependency-aware request queue and collision-aware scheduling visible against one PWA request | D16 |
| C34 | PV-03 | Truthful CI, build, Rust/Host/browser PWA acceptance and test/run receipts | D15,D18,D19,D20 |
| C35 | PV-03,PV-14 | Build/deploy/observe/repair loop, rollback and PWA readback; no silent claims of phone proof | D21 |
| C36 | PV-10,PV-03 | Link live PWA activity/errors to a user-visible defect and trace ID | D22 |
| C37 | PV-03 | Cloud GitHub coding and controlled release-only local verification boundary | D09 |
| C38 | PV-04,PV-08 | Only integration convergence that unblocks a live Wiki or PWA visual change | D17 |
| C39 | PV-14 | Minimum continuity/handoff docs needed to resume a user-submitted PWA request | D25 |

## 3. Parked from the active PWA-first queue

These are retained for **traceability only** and must not be independently scheduled. They require a future, specific PWA-visible parent and fresh owner review.

| Original item | Reason |
| --- | --- |
| R11,R12,R13 | Native GPUI, Tauri-to-GPUI migration and native-only viewport work — Owner-paused; no PWA-deliverable path while standalone |
| D23 | ZCode/Desktop Commander standalone plug-in engineering — Not independently PWA visible; local release checks are already C37 |
| D24 | Provider/model inventories, budgets and NVIDIA NIM repair — No attached PWA feature or owner approval for this slice |
| R22 subset | LSP/MCP-only SDK features without PWA display or action — Park that scope until attached to PV-11 |
| D25 subset | General archive/documentation expansion beyond minimum PWA request continuity — Park unless required for PV-14 |

## 4. Cross-session safeguards and known work

- [Current shared check-in/check-out overlay](../TASK-CLAIMS.md): at the time of this document, it recorded **ChatGPT Dev** as the claimant for `PWA-UTF8-R001` (CODED/PUSHED, full suite UNRUN) and `PWA-FIRST-WIKI-MOUNT-R002` (ACTIVE); never overwrite those scopes in a Planning round.
- [Existing master-plan R001](MASTER-PLAN-R001.md): 37 Planning preparation candidates are retained as research, **not** automatically 37 queue entries. This file adds PWA deliverable parentage, not duplicate execution claims.
- [Canonical reconciliation](TRACKER-RECONCILIATION-R001.md): historical local `OWNER-WORK/` not available in remote Labs GitHub observations; do not mint OW IDs or assert complete task history.
- [Oracle PWA build-ingress signal](../ORACLE/TO-PLANNING.md): **mapped to PV-14/C30–C33/C35** for review; this mapping is not proof of execution acknowledgment, unsupported agent dispatch or user acceptance.
- [Dev source-to-live playbook](../CHATGPT-DEV/RELEASE-AND-INTEGRATION-PLAYBOOK.md): build/test/package/serve/auth/HTTPS/phone gates must be independently evidenced.
- Always work Rust first and keep browser glue TypeScript 7+ where needed. Reuse current real source; don't build another command bus, storage engine, auth layer or Wiki writer.

## 5. Review / promotion workflow

For any row the owner wants to promote, record: (a) outcome ID, (b) what changes on the PWA screen, (c) exact action + expected visible response, (d) parent dependencies, (e) current owner/check-out after reconciling claims, (f) exact acceptance evidence, and (g) whether the item was merged, split or rejected.

Do not assign or mark work DONE from this review document. Planning may amend the candidates during debate; only the canonical task writer and recorded claim handoffs can activate execution.

## 6. Coverage audit

**79/79 original conversation items accounted for:** P01–P24, R01–R30 and D01–D25 either directly as PWA outcomes, as bounded components with named PWA parents, or in the nonactive parking lot. Some references repeat intentionally (a deliverable also contains its implementation dependency). No original item was silently dropped.
