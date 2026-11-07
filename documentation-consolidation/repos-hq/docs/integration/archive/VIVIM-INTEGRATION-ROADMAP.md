# VIVIM integration roadmap — data first, prototype second, UI gated

**Schema:** `hq.integration-roadmap/1` · **Date:** 2026-11-03 · **Status:** plan of record, not an
implementation report.

**What this document is.** The cross-repo integration plan for three repositories that are
currently developed as if they were three products. It states who owns which data, what must be
frozen before parallel work starts, what lands in which order, and what remains unknown.

**What this document is not.** It is not a second status ledger. Harness work stays in
`vivim-web-harness/docs/roadmap.json`; the shared-kernel gates stay in
`docs/SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md`; HQ gaps stay in
`docs/integration/gap-register.v1.json` (created with this document). This page is the
crosswalk between them and the order in which they are allowed to close.

**Reading rule.** Every factual claim below carries one of:

| Label | Meaning |
| --- | --- |
| **VERIFIED** | Re-run or re-read in this workspace on 2026-11-03; command and output cited. |
| **RECORDED** | Written in a repository artifact that exists at the cited path. |
| **UNKNOWN** | No artifact either way. Listed in §9, never silently assumed. |

---

## 1. The three repositories and the roles the owner assigned

| Repo | Path | Role now | Role in this roadmap | Runtime |
| --- | --- | --- | --- | --- |
| **repos_hq** | `C:\0-BlackBoxProject-0\repos_hq` | Code-intelligence system, SQLite index, DCB, wiki, MCP, KB intake | **Data-engine landing spot.** Durable kernel, model intake, connectors, projections | Bun, `bun:sqlite` |
| **vivim-web-harness** | `C:\0-BlackBoxProject-0\vivim-web-harness` | Prototype end-to-end concept: phone → relay/Host → Studio → Chrome, bootstrap personalization, generative UI | **Migration source.** Prove it works, then move *data and contracts* into HQ. Effects stay where they are | Node ≥ 22, `node:sqlite`; Studio/Chrome owner |
| **vivim-ui** | `C:\0-BlackBoxProject-0\vivim-ui` | TierFlow: a reprogrammable realtime UI core (world documents, drafts, CAS publish, WS cursors) plus the legacy 3-tier agent | **Experimental UI.** Adoption is a *timing decision* (§7), not part of phase 1 or 2 | Bun, `bun:sqlite`, Rust sidecar |

Owner direction recorded 2026-11-03: *"repos_hq is the data engine landing spot for now,
vivim-web-harness is the prototype e2e concept with stuff that still needs to be migrated to
repo_hq, vivim-ui is experimental UI system — assess integration timing, maybe now maybe not yet;
but first the data, the working studio + harness + core data engines."*

That sentence is the priority order. Phases 1 and 2 below are the data and the prototype.
Phase 3 is the UI, and it starts closed.

---

## 2. The one decision that governs the rest: data ownership

`docs/SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md:19` already states the principle: the kernel
is **not** the owner of all source content or effects. This roadmap turns that principle into
named stores. Nothing in this table may be duplicated by a later lane.

| Data | Canonical owner after integration | Store | What HQ does with it |
| --- | --- | --- | --- |
| KB entries, ontology, revisions, curation decisions | **HQ kernel** | durable kernel DB (§5.1) | owns |
| KB types/sources/jobs registries (44/30/33) | **HQ kernel**, migrated by versioned import | kernel DB | ingests, versions, diffs |
| Capability catalogue, schemas, semantic aliases | **HQ kernel** | kernel DB | owns descriptors; never grants execution |
| Personal World records, bootstrap claims, themes, receipts | **VIVIM Host** (`apps/vivim-host`) | Host SQLite | reads via connector; **never shadow-writes** |
| Provider send/approval journal, browser ownership | **Studio / harness** | Studio journal | reads provenance only |
| Execution lifecycle and cancellation | **TierFlow Tier 3** (`vivim-ui/src/tier3`) | TierFlow runtime | reads run status only |
| UI documents (layout/world revisions) | **TierFlow UI core**, *if* phase 3 returns GO | `runtime/state.db` | projects only after GO |
| Index, FTS, graph, wiki, timing | **HQ** (disposable) | `.hq/index.db` | rebuildable, never durable truth |

### 2.1 The naming collision must be frozen first

Two different things are both called **World**:

- Harness: the user's **personal knowledge world** — `world_projection` rows with privacy class,
  provenance and tombstones (`vivim-web-harness/docs/design/APK07-mobile-runtime-world-offline.md:51`).
- TierFlow: a **UI document** — objects, layers, shapes and data flows on an infinite 3D canvas
  (`vivim-ui/docs/WORLD-MODEL-SPEC.md:17`).

Until renamed, every design conversation about "the World" is ambiguous, and a lane that reads
one meaning will write the other. **Frozen rule:** harness/kernel keeps `World` /
`world_projection` for personal knowledge; TierFlow artefacts are called **`surface document`**
(the wire field `worldId` is retained for protocol compatibility and documented as a legacy
name). This rename is cheap now and expensive after two lanes code against it.

---

## 3. Verified starting line

Run on 2026-11-03 in this workspace. These are the facts the plan is built on.

| # | Fact | Evidence |
| --- | --- | --- |
| 1 | TierFlow typechecks clean and passes 238 tests / 17 files | `bun run typecheck` → exit 0; `bun test tests/` → `238 pass, 0 fail, 1083 expect() calls` |
| 2 | **The UI core is not mounted.** A live server 404s every `/api/ui/*` route | `TIERFLOW_PORT=7801 bun run src/server.ts`; `GET /api/ui/worlds` → `404 {"error":"not found","path":"/api/ui/worlds"}`; same for `/api/ui/state`, `/api/ui/drafts` |
| 3 | The handlers exist; only the wiring is missing | `vivim-ui/src/surfaces/http.ts:199` (`handleUiHttpRequest`), `vivim-ui/src/surfaces/websocket.ts:156` (`UiSocketSession`); `UiSocketSession` is constructed only in `tests/ui-websocket-surface.test.ts:130` |
| 4 | `ui.publish` is not a capability; publish is gated exactly like a draft edit | `vivim-ui/src/security.ts:19` → `"read" \| "surface.write" \| "program.write" \| "exec"`; spec requires more at `vivim-ui/docs/UI-CORE-SPEC.md:165` |
| 5 | `UiCore` lacks `revisions()`, `projection()` and `draft()`, so two documented read routes degrade | methods present are `current`, `worlds`, `preview`, `onRevision`, `subscribe` (`vivim-ui/src/ui/commands.ts:334-419`); routes promised at `vivim-ui/docs/MOBILE-CLIENT-CONTRACT.md:28-32` |
| 6 | `vivim-ui` **was** unregistered; it is now indexed as a project (276 files, 3,532 symbols) | `bun hq/bin/hq.mjs project list` → `repos-hq`, `vivim-ui`, `vivim-web-harness`; `hq index --changed` → `vivim-ui: +276`. GAP-011 closed the same day |
| 7 | HQ inherits a VIVIM feed and KB intake that work | `hq vivim` exists (`hq/bin/hq.mjs:2409`); `hq/lib/kb-intake.mjs`, `hq/test/kb-intake.test.mjs` |
| 8 | **Model intake is not implemented.** `hq.lib.kb-intake` contains zero references to model/entities/useCases | `grep -ic "model\|entities\|useCases\|authorityRules" hq/lib/kb-intake.mjs` → `0`; documented honestly at `docs/KB-LINK.md:86-87` |
| 9 | A model-bearing package would be silently downgraded to documents | `readDirectoryPackage` walks every file and treats each as a document (`hq/lib/kb-intake.mjs:597`), no `model/` branch |
| 10 | There is **no durable kernel store**; everything lives under rebuildable `.hq/` | `.hq/` contains `index.db`, `config.json`, `wiki/`, `test-cache/`; `AGENTS.md:95` declares `.hq/` rebuildable; `docs/SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md:30` forbids mixing durable knowledge with disposable `.hq/index.db` |
| 11 | Runtime split is a choice, not a wall: `node:sqlite` works under **both** Node and Bun | probe: Node v24.22.1 → `sqlite_version 3.53.4`; Bun 1.4.2 → `3.53.2`; CRUD, prepared statements and WAL all succeeded |
| 12 | TierFlow's UI core is Bun-only as written | `vivim-ui/src/ui/store.ts:34` imports `bun:sqlite`; `node -e 'require("bun:sqlite")'` → `MODULE_NOT_FOUND` |
| 13 | `docs/VIVIM-LINK.md:136` says the `hq vivim` commands are absent. They exist | stale row vs `hq/bin/hq.mjs:2409` |
| 14 | The harness has a working UI-revision authority with CAS, approval and Undo | `vivim-web-harness/docs/roadmap.json` → `GUI04` status `done`; `apps/vivim-host/test/ui-revision.test.mjs` |
| 15 | The harness phone path polls; it does not stream | `apps/mobile-relay/generative-ui/phone-surface.mjs:55-70` — up to 25 polls at 1200 ms after one proposal |
| 16 | TierFlow has the cursor primitive but nothing consumes it | `vivim-ui/src/ui/realtime.ts:8-42` (epoch cursor); no server use |

**Not run in this session:** the harness test suite, the harness cold-reboot acceptance, any
Android device acceptance, and any TierFlow browser render. Their status below is quoted from
repository artefacts and labelled RECORDED, not upgraded.

---

## 4. Phase 0 — freeze before fan-out (serial, blocking)

This phase exists because the failure already happened once. `vivim-ui/docs/SESSIONS-NEXT-STEPS-2026-11-03.md:94-99`
records an orchestrator rewriting `src/ui/types.ts` and deleting `src/ui/document.ts` while four
lanes were coding against the previous contract: three deliverables lost or rewritten, one lane
ended at 0 pass / 1 error.

**No lane in §6 may start until every frozen artefact below exists and is hashed.** Freezing means:
the file is committed, its schema id is in the header, and changes require a version bump plus a
sync-bus broadcast.

| Freeze | Artefact | Schema id | Contents | Blocks |
| --- | --- | --- | --- | --- |
| **F0.1** | `docs/integration/contracts/kernel-store.md` | `hq.kernel-store/1` | DB path, tables, revision model, single-writer rule, backup/restore, "never inside `.hq/`" | L1, L2, L4 |
| **F0.2** | `docs/integration/contracts/model-intake.md` | `hq.vivim-model/1` | The nine sections, validation order, refusal codes, drift report shape | L2, L3 |
| **F0.3** | `docs/integration/contracts/connector.md` | `hq.connector/1` | Connector discovery, read-only guarantee, cursor/revision, provenance fields, no-write assertion | L4 |
| **F0.4** | `docs/integration/contracts/naming.md` | `hq.naming/1` | `World` vs `surface document`, legacy `worldId`, forbidden aliases, lint list | all |
| **F0.5** | `docs/integration/gap-register.v1.json` | `hq.integration-gap-register/1` | Every known gap with evidence pointer and gate (§8) | all |
| **F0.6** | `docs/integration/lanes.v1.json` | `hq.integration-lanes/1` | Lanes, scopes, deps, waves, acceptance commands (§6) | all |

**Gate F0** — `bun scripts/integration/check.mjs` exits 0. It verifies: unique ids, acyclic
dependency graph, every gap has resolvable evidence, every lane has an acceptance command, and no
two lanes in the same wave own overlapping path prefixes. A freeze that the checker cannot verify
is not frozen.

**Frozen-contract protocol.** A lane that needs a contract change stops, sends
`kind=question` on `scripts/sync.mjs` with topic `integration-contract`, and waits. The
integrator either grants a version bump or rejects. Lanes never edit a file listed in another
lane's `forbidden` scope — the checker fails the build if scopes overlap.

---

## 5. Phase 1 — the data engine lands in HQ (priority 1)

Goal: HQ holds **durable, revisioned, queryable truth** that both the harness and a future UI can
read, rather than a disposable index plus prose. This is kernel gates 1–5 of
`docs/SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md:89-93`.

### 5.1 Durable kernel store (gate D1)

- A store that is **not** `.hq/index.db`, and not deleted by `hq reset`.
- Tables for: entries, entity/type/schema registrations, revisions (append-only), provenance,
  sensitivity class, tombstones, import receipts, connectors, cursors.
- One logical writer per KB domain; explicit single-writer lock.
- `hq reset` must leave it intact — that is the test, not a comment.

**Acceptance:** an insert survives `hq reset` + reindex; `hq kernel verify` reports the revision
chain intact; two writers on one file produce a typed error, not corruption.

### 5.2 Model intake — the nine sections (gate D2)

The contract already specifies entities, types, schemas, flows, sources, actions, authority,
features and use cases (`docs/VIVIM-KB-INTAKE.md:255-288`). None of it exists (gap GAP-004).

Non-negotiable failure mode: **a `model/` package must never ingest partially and report `ok`**.
Today it would ingest every model file as a document and report success (GAP-005). The intake
must either ingest all nine sections or refuse with a named missing section.

**Acceptance:** a fixture package with all nine sections ingests and `hq model drift` names real
gaps; a package missing `useCases` is refused with `missing_section: useCases` and writes nothing;
every field type resolves or is an error; a relation to a missing entity is refused.

### 5.3 KB corpus migration and parity (gate D3)

Input corpus: `vivim-web-harness/docs/knowledge/` — 17 files, including `kb-types.v1.json`
(44 types), `kb-sources.v1.json` (30 sources), `kb-jobs.v1.json` (33 jobs) and the schema
registry. Last recorded HQ intake: 25 files, digest `ad9dcad4…` (`docs/KB-LINK.md:78-80`).

The two counts differ and no artefact explains why. That is GAP-008: the identity of what was
ingested is unproven. The migration must publish a **file-by-file parity table** (source path →
kernel row → hash), not a count.

**Acceptance:** every corpus file has exactly one kernel row or a recorded refusal reason; a
second identical import is a no-op with an identical digest; the 44/30/33 counts are asserted in a
test.

### 5.4 Read-only connectors (gate D4)

Start with the harness Host, because it owns the only personal data that matters:

- `connector:vivim-host` reads Host SQLite **read-only** (URI `mode=ro`), never writes.
- Cursor/revision per source table; provenance field on every imported row; staleness reported.
- A negative test asserts the Host DB mtime and contents are unchanged after a full sync.

**Acceptance:** sync twice → second run `changed: 0`; Host DB hash identical; a tampered Host
revision is detected and reported, not silently accepted.

### 5.5 Identity parity (gate D5)

Kernel gate 4: VIVIM and TierFlow must receive **identical semantic objects** for identical
scope. One test, two consumers, byte-equal output.

**Acceptance:** `tests/kernel-parity` runs the same query through the CLI and the in-process API
and compares canonical JSON.

---

## 6. Phase 2 — the working prototype, then migration (priority 2)

Owner priority is "the working studio + harness + core data engines". Migration of a prototype
that is not yet accepted merely moves uncertainty.

### 6.1 Prototype acceptance first (gate P1)

Recorded state: Direct Talk has live receipts from 2026-10-31
(`vivim-web-harness/docs/ARCHITECTURE-CURRENT-AND-TARGET.md:60-61`), while Android repaint,
general provider/model support and cold-reboot acceptance are explicitly not accepted
(`:33-44`, roadmap item `APK07-ACCEPT` = `awaiting_device_acceptance`).

The minimum bar for calling the prototype "working" is one receipt chain, not a feature list:

1. paired phone → Host → SQLite → projection → **on-device repaint** → Undo;
2. timing recorded for each hop;
3. an offline/re-pair run with no silent replay.

**Acceptance:** a dated receipt file with command ids and timings, produced by the owner, not by an
agent.

### 6.2 Then migrate — in this order (gate P2)

| Order | Data | Source | Destination | Why this order |
| --- | --- | --- | --- | --- |
| 1 | KB corpus + registries | `docs/knowledge/` | kernel | Already reachable; highest reuse |
| 2 | Capability catalogue | `docs/capabilities/atomic-registry.json` | kernel | Needed by every other projection |
| 3 | Session/transcript archive | `sessions/` | kernel, provenance-tagged | Evidence for drift and handoff |
| 4 | Bootstrap ledger **read model** | Host SQLite | kernel projection | Only after §6.1; Host stays writer |
| 5 | Personal World **read model** | Host SQLite | kernel projection | Privacy-classed; default excludes D06 |

**Never migrated:** Host effect authority, Studio's browser ownership and send-once journal, relay
credentials, provider cookies, Android platform effects. The kernel references them; it does not
own them (`SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md:19,37-44`).

### 6.3 What is deliberately *not* migrated (gate P3)

A negative test must prove no shadow write: after a full kernel sync, every source-native database
is byte-identical. Any lane that "fixes" drift by writing back into the Host is wrong by
definition and fails this gate.

---

## 7. Phase 3 — vivim-ui timing (gated; starts closed)

The experimental UI is genuinely better in three places, and those are the reasons to keep it:

1. a declarative document with **stable node identity** rather than a fixed component tree;
2. **draft → preview → CAS publish → rollback** with an audit row in the same transaction;
3. **`{epoch, seq}` resume cursors**, so a reconnect can prove it lost nothing.

But it is also, today, unreachable (GAP-001), under-capable at the boundary (GAP-002),
under-implemented in reads (GAP-003), and unmeasured against the harness path (GAP-009).

### 7.1 The decision criteria — all six must have a recorded answer

| # | Criterion | How it is answered | Default if unanswered |
| --- | --- | --- | --- |
| U1 | Does the UI core serve a real client today? | `/api/ui/worlds` returns 200 on a live server | **NO-GO** |
| U2 | Is the phone repaint loop measurably faster than the harness poll loop? | same workload, p50/p95 of edit→repaint, both paths | **NO-GO** |
| U3 | Can authorization be expressed without a second identity system? | harness device grant mapped to TierFlow capability set | **NO-GO** |
| U4 | Does the document model cover the harness's actual surfaces? | feature-by-feature map: cards, timeline, chart, profile/project cards | **NO-GO** |
| U5 | Is there one UI revision authority after adoption? | written ownership row in §2, not two | **NO-GO** |
| U6 | Can the core run where the Host runs without a third process? | Host under Bun, or a store port proven on `node:sqlite` | **NO-GO** |

**"Efficient" is currently a hypothesis, not a measurement.** The harness path pays a relay round
trip and up to 30 s of polling; TierFlow holds a WebSocket and replays from a cursor. That is
*architecturally* fewer hops, but no p50/p95 exists for either path. U2 is what turns the claim
into evidence.

### 7.2 If NO-GO: harvest, do not adopt

Adopt the four primitives as kernel/design inputs without adopting the runtime:

- stable node identity and document versioning as a kernel schema;
- CAS publish + idempotency record shape;
- epoch/seq cursor semantics;
- the draft/preview/rollback state machine.

The harness keeps its Host authority. No process, no second store, no migration.

### 7.3 If GO: the only permitted sequence

`mount → capability split → measure → parity → migrate`. Mounting is not optional and is not a
later polish item: a service nothing can reach cannot be evaluated (GAP-001). The full conditional
plan is lane **L7** in `docs/integration/lanes.v1.json`, and it stays `status: blocked` until U1-U6
are answered.

---

## 8. Gap register — everything known to be open

Full machine-readable form: `docs/integration/gap-register.v1.json` (schema
`hq.integration-gap-register/1`). Summary:

| Id | Sev | Gap | Gate |
| --- | --- | --- | --- |
| GAP-001 | blocker | TierFlow UI core is not mounted; every `/api/ui/*` route 404s | U1 |
| GAP-002 | high | `ui.publish` capability does not exist; publish == draft edit | U1 |
| GAP-003 | high | `revisions()`, `projection()`, `draft()` missing; two documented reads degrade | U1 |
| GAP-004 | blocker | Model intake (`hq.vivim-model/1`, nine sections) unimplemented | D2 |
| GAP-005 | high | A model package would be ingested as documents and reported successful | D2 |
| GAP-006 | ~~blocker~~ | ~~No durable kernel store; all state is rebuildable `.hq/`~~ — **closed 2026-11-03**; `kernel/kernel.db` survives `hq reset`; 22/22 kernel tests | D1 |
| GAP-007 | high | Two UI revision authorities (harness `GUI04` vs TierFlow `ui_revisions`) | U5 |
| GAP-008 | medium | KB corpus (17 files) vs last intake (25 files): identity unproven | D3 |
| GAP-009 | medium | No measured comparison of poll loop vs WS cursor loop | U2 |
| GAP-010 | medium | Runtime split (`bun:sqlite` vs `node:sqlite`) is unresolved | U6 |
| GAP-011 | ~~medium~~ | ~~`vivim-ui` not registered in the HQ index~~ — **closed 2026-11-03**, now 276 files / 3,532 symbols | D1 |
| GAP-012 | low | `docs/VIVIM-LINK.md` "implemented vs specified" table is stale | D5 |
| GAP-013 | high | Harness live acceptance incomplete while source status reads complete | P1 |
| GAP-014 | medium | 141 KB 3D renderer has never rendered in a browser | U4 |
| GAP-015 | medium | Publish atomicity has no failure-injection test | U1 |
| GAP-016 | medium | "World" means two different things in two repos | F0.4 |
| GAP-017 | medium | Cross-repo kernel identity parity has no test | D5 |
| GAP-018 | medium | Session archive retention/ownership undecided | P2 |
| GAP-019 | high | No negative test proves connectors never write to the Host | D4 |
| GAP-020 | low | No owner-visible gate prevents claiming model intake done | F0 |
| GAP-021 | low | Every integration document lands unclassified (family 00); no registry pattern | F0 |
| GAP-022 | high | `lanes.v1.json` schedules Wave 2 in parallel; `TEAM.v1.md` defaults it to serial (T1) | F0 |
| GAP-023 | high | `verify --project` / `check --project` silently ignore the flag; `selectProjects` does not exist | F0 |
| GAP-024 | medium | `sessions/*.json` churn makes the freshness gate red by construction in a live workspace | F0 |
| GAP-025 | high | Semantic vocabulary (ids, aliases, links, wiki programs) lives in disposable `.hq/index.db` | D1 |

---

## 9. Unknowns — genuinely undecided, not to be guessed

| Id | Unknown | Why it matters | Blocking gate |
| --- | --- | --- | --- |
| UNK-001 | Can the whole harness Host service run under Bun in production? | The probe proves `node:sqlite` works under Bun; it does not prove the Host, Studio and its child processes do | U6 |
| UNK-002 | Does the owner want the phone to render the personal World, or only edit surfaces? | Decides whether the kernel needs a device projection at all | U4 |
| UNK-003 | Which repo owns the eventual product name and versioning? | Harness calls itself VIVIM; TierFlow is `@software-hq/tierflow` | F0.4 |
| UNK-004 | Are `sessions/` transcripts evidence to keep or noise to discard? | 14.4 MB policy unresolved in TierFlow; affects kernel provenance | P2 |
| UNK-005 | Is multi-user ever in scope? | Single-writer assumptions are safe only if single-user stays true | D1 |

An unknown is closed by an owner decision recorded here or in the kernel decision log — never by
an agent choosing a default and moving on.

---

## 10. Parallel execution plan

Full lane definitions, scopes and dependencies: `docs/integration/lanes.v1.json`.

**Rules that make fan-out safe:**

1. **Max 3 live lanes**, matching the harness roadmap rule already in force.
2. One lane = one repository + one path scope. No lane edits another lane's scope.
3. Freeze before fan-out (§4). A contract change is a version bump, broadcast, and integrator
   ack — never an in-place edit.
4. **The integrator (L0) is the only lane that marks a gate passed.** A lane reports evidence; it
   does not grade itself.
5. Every lane merges only after `bun scripts/integration/check.mjs` and its own acceptance command
   are green.
6. Six files are **shared-serial**: `hq/bin/hq.mjs`, `hq/lib/schema.mjs`, `package.json` and the
   three ledger files. Only L0 edits them; every other lane sends a proposed delta. This is the
   file set whose concurrent editing already cost three deliverables once.

**Waves** (the lane file is authoritative; this diagram is the reading aid):

```
Wave 0 (serial)   L0  freeze + scaffold                         gate F0
                      |
Wave 1 (3 lanes)  L1 kernel store   L5 harness receipts   L6 UI assessment
                  [repos_hq]        [harness]             [vivim-ui, 1 file]
                      |                   |                     |
Wave 2 (3 lanes)  L2 model intake   L3 corpus parity      L4 Host connector
                  [repos_hq]        [repos_hq]             [repos_hq]
                      |                   |                     |
Wave 3            L7 mount + adopt (conditional on U1-U6, starts blocked)
                      |
                  L0 integrates, verifies, closes gates
```

| Lane | Repo | Write scope | Gate | Why it can start when it does |
| --- | --- | --- | --- | --- |
| **L0** | repos_hq | `docs/integration/**`, the roadmap, `scripts/integration/**` | F0 | Serial by definition; owns the ledgers |
| **L1** | repos_hq | `hq/lib/kernel/**`, `hq/kernel/**`, kernel tests | D1 | Needs only the frozen store contract |
| **L5** | harness | `apps/vivim-host/**`, `apps/mobile-relay/**`, `docs/receipts/**` | P1 | Different repo; no shared files |
| **L6** | vivim-ui | `docs/integration-assessment.md` only | U1 | Read-only assessment; one writable file |
| **L2** | repos_hq | `hq/lib/model.mjs`, `model-intake.mjs`, `kb-intake.mjs`, test | D2 | Needs the store (L1) to exist first |
| **L3** | repos_hq | `scripts/kernel/**`, `docs/kernel/**`, parity test | D3 | Needs the store (L1); runs beside L2 |
| **L4** | repos_hq | `hq/lib/connectors/**`, `hq/test/connector/**` | D4 | Needs the store (L1); codes to the connector contract |
| **L7** | vivim-ui | `src/server.ts`, `src/ui/**`, `src/surfaces/**`, `tests/**` | U1 | Blocked until the U1-U6 decision is recorded |

Wave 1 deliberately spans three repositories, so it has **zero** scope overlap by construction.
Wave 2 puts three lanes in one repository but on three disjoint directories; `hq/bin/hq.mjs` stays
shared-serial, which is why L1 and L2 do not both register their CLI verbs.

L3 is scoped to the **document corpus** parity, which needs only the store. Model-section parity is
L2's acceptance, so the two stay independent instead of serialising.


---

## 11. Owner decisions required

| # | Decision | Default if unanswered | Consequence of the default |
| --- | --- | --- | --- |
| O1 | Approve the `World` / `surface document` naming split (§2.1) | Keep the split as proposed | Every lane reads "World" one way; the other repo's meaning is lost |
| O2 | Durable kernel store path outside `.hq/` | `C:\0-BlackBoxProject-0\repos_hq\kernel\` (gitignored, backed up) | A rebuild wipes knowledge, which is the gap this roadmap exists to close |
| O3 | Does the harness Host ever move to Bun? | No; keep Node, use connectors | Rules out in-process UI adoption (U6) but not harvesting |
| O4 | vivim-ui adoption: decide now or after D1-D5? | After D1-D5 | Adoption proceeds against an unproven data layer |
| O5 | `sessions/` retention | Keep, provenance-tagged | 14.4 MB stays untracked and unmanaged |
| O6 (=T1) | Parallel writes in one repo: path-scoped commits by L0, or serialize every wave? | **Path-scoped commits by L0** (recorded in `lanes.v1.json` `commitPolicy`) | Serialising Wave 2 removes the fan-out; the whole-tree commit is the constraint, not the write |
| O7 (=T2) | Approve the team roster (`TEAM.v1.md`, the machine-read team sheet) as the team of record? | Not approved; runs as a proposal | Lanes execute, but no role change is permanent |
| O8 (=T4) | Adopt `lane-L0` … `lane-L7` as distinct bus identities? | Adopt | Two sessions keep posting as `hq-agent` and per-agent cursors cannot distinguish them |

Two numbering schemes are live: this table (`O*`) and `docs/integration/TEAM.v1.md` (`T*`). They
are cross-referenced above rather than renumbered, because renumbering another lane's artifact is
the kind of silent edit this scaffold exists to prevent. `T3` (bot schedules) and `T5` (model tiers)
are outside this roadmap's scope and stay in the team document.

The `O6` ruling, in one line: **parallel writes, serialized path-scoped commits.** `git-main-sync.mjs`
stages with `git add -A` (`:216`) and already defers while any changed file is younger than
`minAge` (`:86`), so active writing is protected; the unsafe window is finished-but-unverified work
that has aged past `minAge`. A checkpoint commit is not gate evidence. Only a recorded gate verdict
is.

---

## 12. Non-goals

- No second global TODO ledger. Harness work stays in `roadmap.json`; kernel gates stay in the
  architecture decision; this page is the crosswalk.
- No cloud dependency, no Vercel deployment in this path.
- No migration of effect authority: Host, Studio, Android broker and Tier 3 keep theirs.
- No WASM runtime, no plugin marketplace, no multi-host consensus.
- No claim that a green test suite proves live acceptance. Only an owner receipt does that.
- No Rust work. `docs/RUST-WORKSTREAM.md` gates G1-G6 are untouched.

---

## 13. Current verdict and next safe action

**Verdict:** the data engine is the only piece that is both highest-value and currently absent.
The harness is a working prototype with an incomplete acceptance chain; the experimental UI is
promising but unreachable and unmeasured. Doing the UI first would invert the owner's order and
build on a store that a rebuild deletes.

**Wave 0 state (2026-11-03):** scaffolded and green. `bun scripts/integration/check.mjs` →
`ok: true`, 24 gaps across 15 gates, 8 lanes, max 3 concurrent. The four contracts exist at
`docs/integration/contracts/`. Dogfood evidence: quality gate `quick` 5/5 in 27.6s, scoped lint
exit 0, `hq context` and `hq brief` both route to this scaffold.

Two caveats keep F0 from being called *accepted*:

- **O7/T2**: the team roster is a proposal, not an approved team of record.
- **COLLISION-001/002**: at least two sessions are writing as L0. Until one session holds the
  integrator identity, `TEAM.v1.md`, the gap register and the checker are contended files, and
  a contended ledger is not a ledger.

**Next safe action (one):** resolve the L0 identity (O8/T4: post as `lane-L0`, one session only)
and land GAP-023 — make `verify`/`check` honour `--project` or remove the flag so it stops lying.
Both are small, both unblock every merge gate, and neither can be done by a lane that is not L0.

**Explicitly not the next action:** mounting the TierFlow server (L7, gated), migrating Host data
(L4, after D1), or answering the UI adoption question (L6, free to answer "not yet").

### 13.1 Wave 1 verdict (2026-11-03)

| Lane | Gate | Verdict | Evidence | Open |
| --- | --- | --- | --- | --- |
| **L1** | D1 | **PASSED** | `bun test hq/test/kernel` → 22 pass / 0 fail; `hq kernel verify` → 12 revisions, 0 broken before and after `hq reset --yes` + unscoped `hq index` (4,259 files, 125 s); commit `e8962df` | Two implementation notes: the store uses SQLite-native `VACUUM INTO` because Bun's `node:sqlite` has no online backup API (documented in `hq/lib/kernel/store.mjs`, round-trip tested); the survival test scopes its index to `repos-hq` to fit the gate timeout, and Keeper ran the unscoped index separately. |
| **L5** | P1 | **NOT PASSED — lane build accepted** | `node --test apps/vivim-host/test/ui-revision.test.mjs` → 8/0; `receipt-runner.test.mjs` → 1/0; receipt 8 hops, 6 measured, 2 `not_run: device_unavailable`; commits `f4103bb` + `d0c6d08` | The owner must run the paired-phone → Host → repaint → Undo chain on a device and produce the dated receipt. The dry run is explicitly not P1 evidence. |
| **L6** | U1 | **ASSESSMENT DELIVERED — DECISION OPEN** | `vivim-ui/docs/integration-assessment.md` answers U1–U6; commit `16ef242`; Lens upheld every load-bearing citation | U1 = NO (404, not mounted); U2 = could not measure; U3 = expressible but no `ui.publish`; U4 = partial; U5 = two authorities; U6 = Bun-only today. No owner GO; L7 stays blocked. |

**Gate evidence is a recorded verdict plus the named command, never a checkpoint commit.** The L0
scaffold commit is `f53be2f`. The harness `doc-precommit` hook auto-staged eight generated index
files into the L5 commit; they were restored to their parent state by follow-up commit `d0c6d08`,
so the lane's effective diff is the five receipt files.

**L0 identity:** this wave ran as `lane-L0` only. At start, live bus claims were empty and the one
prior L0 claim (`integrator-L0`) had been released. A concurrent session added GAP-025 and six
integration documents during the wave without a claim; they are preserved, not overwritten.
GAP-023 remains contested: HEAD (`583f049`) contains `selectProjects` and scoped `verify`/`check`,
while an uncommitted working-tree delta removes them; that delta predates L1, was isolated from the
L1 commit, and must be resolved by its owning session. Until then the merge gate is
`bun scripts/integration/check.mjs` plus the lane acceptance, not `hq verify`.

**Next safe action (one):** the owner runs L5's on-device receipt chain. Wave 2 (L2, L3, L4) may
start only after D1's store is accepted; all three code against `hq.kernel-store/1`.

---

## 14. Automation — what keeps this maintained with zero human overhead

The rule for this section: **an automation that needs a human to read its output is not an
automation, it is a chore with extra steps.** Everything below either self-heals, fails closed, or
writes a receipt nobody has to read. Everything that needs judgement is in §14.2 instead.

### 14.1 Automations

| Id | Trigger | Command | Keeps true | On failure | Human actions / month | Exists? |
| --- | --- | --- | --- | --- | --- | --- |
| **A1** | every gate run: CI push/PR, `/gate`, release | add `scaffold: { command: ['scripts/integration/check.mjs'] }` to `STEPS` and to `quick/local/ci/release` in `scripts/quality-gate.mjs:27-54` | evidence pointers resolve, lane scopes disjoint, roster ↔ lanes parity, roadmap ↔ register parity | fail-closed (step exit 1) | **0** | step **not yet added**; gate and CI exist (`quality-gate.mjs:27`, `.github/workflows/ci.yml:39`) |
| **A2** | file edited (OpenCode plugin) | `index --changed --quiet` (`.opencode/plugins/hq/index.ts:15`) | HQ index freshness | silent, next edit retries | 0 | yes |
| **A3** | session event | `lake ingest --quiet` (`.opencode/plugins/hq/index.ts:16`, `:275-290`) | session evidence reaches the lake | silent | 0 | yes |
| **A4** | 5-minute timer | `scripts/git-main-sync.mjs --repo <space> --json` (`.opencode/plugins/hq-main-git-sync.ts:109`) | local commits, FF to origin, defer on active edits | defer/escalate with receipt | 0 | yes |
| **A5** | CI push/PR | `bun scripts/quality-gate.mjs --profile ci` (`.github/workflows/ci.yml:39`) | the whole gate, including A1 once added | fail-closed | 0 | yes |
| **A6** | one-time, then automatic | add a `docs/integration/**` + roadmap pattern to `hq/registry/agentic-file-registry.v1.json` | DCB family routing for integration docs (today all family 00, GAP-021) | n/a | 0 after the one edit | no |
| **A7** | every check | assert each `sharedSerial` path exists (4 lines in `scripts/integration/check.mjs`) | protection entries point at real files | fail-closed | 0 | proposed, not landed — a deleted roster entry protected nothing for several minutes |
| **A8** | every check | derive lane readiness from gate records instead of the `status` field | stale `status` in `lanes.v1.json` cannot claim progress | n/a | 0 | no |

A1 is the whole automation programme in one line: the checker already knows how to detect every
class of scaffold rot; A1 is what makes it run on every push, every PR, every `/gate` and every
release without anyone remembering to invoke it.

**Smallest diff that delivers A1:**

```js
// scripts/quality-gate.mjs
const STEPS = {
  // ...
  scaffold: { label: 'Integration scaffold', command: ['scripts/integration/check.mjs'], required: true, timeoutMs: 60_000 },
};
const PROFILES = {
  quick: ['runtime', 'workflows', 'scaffold', 'typecheck', 'index-build', 'lint'],
  local: [/* ... */ 'scaffold'],
  ci: [/* ... */ 'scaffold'],
  release: [/* ... */ 'scaffold'],
};
```

Proof: `bun scripts/quality-gate.mjs --profile quick` prints `ok  Integration scaffold (…ms)` and
`bun scripts/integration/check.mjs` exits 0. Break one evidence pointer and the gate goes red with
the exact path — that is the acceptance test.

### 14.2 Deliberately not automated

| Thing | Why it stays human | Cost of automating it |
| --- | --- | --- |
| Contract version bumps | A contract change alters what several lanes may write; it needs an ack, not a cron | Lanes silently adopt an interface nobody approved |
| Roster approval (O7/T2) | Authority is owner-granted (`docs/AGENTIC-RULEBOOK.md`) | A document would grant execution authority |
| Gate verdicts | L0 must look at the evidence; a lane grading itself is the failure mode §4 exists to prevent | Green without evidence |
| Release, publish, provider send | Outward-facing and irreversible | A background process acts without the owner |
| Closing a gap | Closing means "the cited evidence no longer describes reality", which needs a check that may itself have effects | A passing command closes a gap that is still true in another scope |
| Model intake acceptance | The model is the KB's; HQ validates, it does not approve | HQ would become the schema authority |

### 14.3 Claimed versus actual automation

| Claim | Reality | Verdict |
| --- | --- | --- |
| `AGENTS.md` "Auto-update": `opencode.json` hooks run `index --changed` and `lake ingest` | `opencode.json` has no `hooks` block; the behaviour lives in `.opencode/plugins/hq/index.ts:15-16` | Behaviour exists, the document names the wrong file. Not a gap, a precision fix |
| `AGENTS.md`: git sync starts on session activity and repeats every 5 minutes | `.opencode/plugins/hq-main-git-sync.ts:109` (`setInterval`, `FIVE_MINUTES`) | Verified |
| Bus delta: `selectProjects()` plus `--project` on `verify`/`check` landed | Neither exists; `cmdVerify`/`cmdCheck` call `projectsForIndex(root)` unfiltered | **False as stated** — GAP-023 |


