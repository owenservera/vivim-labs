# Canonical taxonomy

**Schema:** `hq.canonical-taxonomy/1` · **Version:** `1.0.0` · **Digest:** `5c609a2b85c44013ee3221dd44d7715ad0c756bff53760a289ba0bda9c0f38ac`

**Machine source of record:** [`hq/registry/canonical-taxonomy.v1.json`](../hq/registry/canonical-taxonomy.v1.json)
**Loader:** [`hq/lib/taxonomy.mjs`](../hq/lib/taxonomy.mjs) · **Tests:** [`hq/test/taxonomy.test.mjs`](../hq/test/taxonomy.test.mjs)
**Backbone:** [`AGENTIC-RULEBOOK.md`](AGENTIC-RULEBOOK.md) · [`DCB-PROTOCOL.md`](DCB-PROTOCOL.md) · [`CANON-DECISIONS.md`](CANON-DECISIONS.md) · [`COMPONENT-REGISTRY.md`](COMPONENT-REGISTRY.md)

This is the one vocabulary the whole platform resolves through: the KB, the HQ
modules, the wiki, the DCB composer, the component registry, the console and the
VIVIM surfaces all name the same things with the same ids. It is not a glossary of
whatever the code happens to do. Where the code and a contract document disagree,
the code is recorded as the fact and the disagreement is recorded as a `drift`
note on that entry.

> A document never grants execution authority. This taxonomy names what exists; it
> does not permit anything.

## 1. Layers

The layer order is `kb` > `hq` > `surface`. A lower layer may not define what a
higher layer owns.

| id | Layer | Rank | May define | May not define |
| --- | --- | ---: | --- | --- |
| `layer.kb` | KB model authority | 1 | entities, types, schemas, flows, sources, actions, authority rules, features, use cases | local status values, execution permissions, the file layout of HQ, surface behaviour |
| `layer.hq` | HQ projection and evidence | 2 | local projections, indexes, derived views, receipts, context bundles, the local decision register, the local CANON register | entities the KB declares, execution authority, the meaning of a KB term, a replacement for an owner decision |
| `layer.surface` | Surface | 3 | presentation, argument parsing, output format | new record kinds, new statuses, authority, effects |
| `surface.cli` | Command line | 3 | command surface, argument parsing, text and JSON output | a second implementation, a private vocabulary |
| `surface.wiki` | Wiki | 3 | generated pages, page navigation | new records, write actions, authority |
| `surface.console` | Console | 3 | the command palette, the read-only result view | write commands, indexing, hook installation, sends |
| `surface.mcp` | MCP server | 3 | tool exposure, tool input schemas | a private index, a private vocabulary, unbounded output |
| `surface.plugin` | Harness plugin | 3 | event subscription, session instruction injection | a second index, write paths |

Rules:

- `layer.kb` — The KB owns the domain model; HQ materialises it and reports drift against it instead of overriding it. (docs/VIVIM-KB-INTAKE.md §8.3)
- `layer.hq` — HQ projects. A conflict between the model and an HQ table is reported as drift, never resolved locally. (hq/lib/kb.mjs; hq/lib/kb-intake.mjs; docs/VIVIM-KB-INTAKE.md §8.3)
- `layer.surface` — A surface may not introduce a concept the KB or HQ has not declared. (docs/WIKI-CONSOLE.md §1)
- `surface.cli` — The CLI is the reference surface: every other surface calls the same commands. (hq/bin/hq.mjs)
- `surface.wiki` — A wiki page is a rebuildable projection of the index, never a command and never a source of truth. (hq/lib/wiki.mjs; docs/COMPONENT-REGISTRY.md §8)
- `surface.console` — The console is a second CLI with no state: every command it exposes is read-only. (docs/WIKI-CONSOLE.md §1)
- `surface.mcp` — MCP is a transport over the same tools the CLI runs; a tool not in the registry does not exist. (hq/lib/mcp.mjs)
- `surface.plugin` — The plugin observes events and injects context; it never mutates provider state. (docs/SOTA-UPGRADE-TARGETS.md §6)

## 2. Entities

The domain entity kinds the platform resolves. `owner` says which layer declares
the entity; `persistence` says where its rows live, or that it has none.

| id | Entity | Owner | Persistence | id prefix | Status enum | Lifecycle |
| --- | --- | --- | --- | --- | --- | --- |
| `entity.project` | Project | hq | table:projects | `project` | — | current |
| `entity.document` | Document | hq | table:documents | `doc` | `status.document` | current |
| `entity.decision` | Decision | hq | table:decisions | `decision` | `status.decision` | current |
| `entity.canon_term` | CANON term | hq | table:canon_terms | `canon` | `status.canon_term` | current |
| `entity.session` | Session | hq | table:sessions | `session` | — | current |
| `entity.turn` | Turn | hq | none | `turn` | — | historical |
| `entity.note` | Note | hq | table:notes | `note` | — | current |
| `entity.symbol` | Symbol | hq | table:symbols | `symbol` | — | generated |
| `entity.file` | File | hq | table:files | `file` | — | current |
| `entity.commit` | Commit | hq | table:commits | `commit` | — | historical |
| `entity.event` | Event | hq | table:events | `evt` | — | current |
| `entity.component` | Component | hq | table:components | `component` | `status.component` | current |
| `entity.pipeline` | Pipeline | hq | table:pipelines | `pipeline` | `status.pipeline` | current |
| `entity.run` | Run | hq | table:pipeline_runs | `run` | `status.task_run` | historical |
| `entity.step` | Step | hq | table:pipeline_steps | `step` | `status.task_run` | historical |
| `entity.task` | Task | hq | table:automation_tasks | `task` | `status.task_run` | current |
| `entity.bundle` | Context bundle | hq | projection:context_bundle | `bundle` | — | generated |
| `entity.receipt` | Receipt | hq | table:receipts | `receipt` | — | historical |
| `entity.model` | Model | kb | none | `model` | — | current |
| `entity.feature` | Feature | kb | none | `feature` | `status.feature` | current |
| `entity.use_case` | Use case | kb | none | `usecase` | — | current |
| `entity.action` | Action | kb | projection:component | `action` | — | current |
| `entity.flow` | Flow | kb | none | `flow` | — | current |
| `entity.source` | Source | kb | none | `source` | — | current |

Notes:

- `entity.project` — A registered directory with an absolute path; rows are created by `hq project add`.
- `entity.document` — Every indexed Markdown, YAML or ADR file, classified into one of the 21 families plus `unclassified`.
- `entity.decision` — A dated outcome with rationale, alternatives, consequences and evidence.
- `entity.canon_term` — A bounded term with a definition, aliases and explicit non-equivalences.
- `entity.session` — One harness session recorded from the provider transcript.
- `entity.turn` — A single exchange inside a session. There is no `turn` table; docs/VIVIM-KB-INTAKE.md §8.4 lists `turn` under entitiesMissingTable, and turn text lives in `session_messages`.
- `entity.note` — A tiered memory row (`now`, `later`, `canon`) with origin, confidence and evidence.
- `entity.symbol` — A declaration extracted from a source file by hq/lib/extract.mjs; rebuildable and never authoritative.
- `entity.file` — One indexed file with its language, size, hash and mtime.
- `entity.commit` — A git commit ingested into the event lake; immutable history, never rewritten.
- `entity.event` — The `events` table is declared in hq/lib/schema.mjs but no module in hq/lib inserts into it; the hq.event/1 store is contract-only (docs/AGENTIC-AUTOMATION-TOOLKIT.md §3).
- `entity.component` — A versioned, declared action with effects and authority; the table is created by hq/lib/components.mjs, not by hq/lib/schema.mjs.
- `entity.pipeline` — A versioned DAG of component references with edges, defaults and config.
- `entity.run` — One execution of a pipeline version; the row records config, input and output digests.
- `entity.step` — One attempt of one component inside a run.
- `entity.task` — A scheduled unit of work with a lease and attempts; the table and its statuses are contract-only (docs/AGENTIC-AUTOMATION-TOOLKIT.md §5).
- `entity.bundle` — The bounded, scored set of sources returned by hq/lib/context.mjs. It is not stored; the receipt that records it is.
- `entity.receipt` — An append-only evidence row: what was read, with source hashes and a digest.
- `entity.model` — The KB domain model block (`hq.vivim-model/1`). No module stores it yet; hq/lib/kb-model.mjs is the planned projection.
- `entity.feature` — A KB-declared feature that names the flows and actions that deliver it.
- `entity.use_case` — A KB-declared given/when/then with a feature, a flow and evidence; the traceability spine.
- `entity.action` — A KB-declared action that materialises as an HQ component with its declared effects and authority copied verbatim.
- `entity.flow` — A KB-declared sequence of steps; HQ compiles it to a DAG view and rejects cycles.
- `entity.source` — A KB-declared origin of data (file, table, api, capture) with a trust level and an owner.

## 3. Id grammar

Every id is immutable. A record whose content changes keeps its id; a record whose
id changes is a new record. An id is never reused for a different meaning.

| id | Prefix | Grammar | Charset | Max length | Example |
| --- | --- | --- | --- | ---: | --- |
| `grammar.project` | `project` | `project:<name>` | `[a-z0-9._-]` | 64 | `project:repos-hq` |
| `grammar.doc` | `doc` | `doc:<project>:<path>` | `[A-Za-z0-9._:/-]` | 400 | `doc:repos-hq:AGENTS.md` |
| `grammar.decision` | `decision` | `decision:dec_<24 hex>` | `[a-z0-9_]` | 32 | `decision:dec_0123456789abcdef01234567` |
| `grammar.canon` | `canon` | `canon:canon_<24 hex>` | `[a-z0-9_]` | 32 | `canon:canon_0123456789abcdef01234567` |
| `grammar.session` | `session` | `session:<session_id>` | `[A-Za-z0-9._-]` | 128 | `session:ses_01HQ8Z` |
| `grammar.turn` | `turn` | `turn:<session_id>/<seq>` | `[A-Za-z0-9._/-]` | 160 | `turn:ses_01HQ8Z/12` |
| `grammar.note` | `note` | `note:note_<24 hex>` | `[a-z0-9_]` | 32 | `note:note_0123456789abcdef01234567` |
| `grammar.symbol` | `symbol` | `symbol:<path>#<name>:<line>` | `[A-Za-z0-9._#:/-]` | 300 | `symbol:hq/lib/context.mjs#composeContext:412` |
| `grammar.file` | `file` | `file:<project>:<path>` | `[A-Za-z0-9._:/-]` | 400 | `file:repos-hq:hq/lib/context.mjs` |
| `grammar.commit` | `commit` | `commit:<40 hex>` | `[0-9a-f]` | 48 | `commit:9fceb02d0ae598e95dc970b74767f19372d61af8` |
| `grammar.evt` | `evt` | `evt:<24 hex>` | `[0-9a-f_]` | 28 | `evt:0123456789abcdef01234567` |
| `grammar.component` | `component` | `component:<id>@<semver>` | `[A-Za-z0-9._:-] plus @ and semver` | 200 | `component:hq.tool.index@1.2.0` |
| `grammar.pipeline` | `pipeline` | `pipeline:<id>@<semver>` | `[A-Za-z0-9._:-] plus @ and semver` | 200 | `pipeline:index-after-edit@2.0.0` |
| `grammar.run` | `run` | `run:<24 hex>` | `[0-9a-f_]` | 28 | `run:0123456789abcdef01234567` |
| `grammar.step` | `step` | `step:<run_id>/<step_id>` | `[A-Za-z0-9._/-]` | 200 | `step:run_0123456789abcdef01234567/index` |
| `grammar.task` | `task` | `task:<64 hex>` | `[0-9a-f_]` | 68 | `task:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef` |
| `grammar.bundle` | `bundle` | `bundle:<64 hex digest>` | `[0-9a-f]` | 70 | `bundle:9c1f0e2b8f7a4d1c6b3e5a09f2c7d4b1e8a05f3c6d9b2e7a41c8f05d3b6e9a2c` |
| `grammar.receipt` | `receipt` | `receipt:rcpt_<24 hex>` | `[a-z0-9_]` | 32 | `receipt:rcpt_0123456789abcdef01234567` |
| `grammar.model` | `model` | `model:<id>@<version>` | `[A-Za-z0-9._-] plus @ and version` | 200 | `model:vivim@2026-11-02` |
| `grammar.feature` | `feature` | `feature:<id>` | `[A-Za-z0-9._-]` | 120 | `feature:capability-discovery` |
| `grammar.usecase` | `usecase` | `usecase:<id>` | `[A-Za-z0-9._-]` | 120 | `usecase:UC-014` |
| `grammar.action` | `action` | `action:<id>` | `[A-Za-z0-9._:-]` | 120 | `action:hq.action.send-message` |
| `grammar.flow` | `flow` | `flow:<id>` | `[A-Za-z0-9._-]` | 120 | `flow:capture-and-read` |
| `grammar.source` | `source` | `source:<id>` | `[A-Za-z0-9._-]` | 120 | `source:studio-capture` |

Rules:

- `grammar.project` — undefined (hq/lib/schema.mjs:15 (projects.name))
- `grammar.doc` — undefined (hq/lib/kb.mjs docNodeId)
- `grammar.decision` — undefined (hq/lib/canon.mjs decisionId)
- `grammar.canon` — undefined (hq/lib/canon.mjs termId)
- `grammar.session` — undefined (hq/lib/kb.mjs sessionNode; sessions.sessions.id)
- `grammar.turn` — undefined (hq/lib/schema.mjs session_messages.seq)
- `grammar.note` — undefined (hq/lib/memory.mjs noteId)
- `grammar.symbol` — undefined (hq/lib/schema.mjs symbols(file_id, name, line))
- `grammar.file` — undefined (hq/lib/schema.mjs files(project_id, path))
- `grammar.commit` — undefined (hq/lib/schema.mjs commits.sha)
- `grammar.evt` — undefined (hq/lib/sessions.mjs:308 emits `evt_` + sha256(idBase).slice(0, 24))
- `grammar.component` — undefined (hq/lib/components.mjs ID_RE and version column)
- `grammar.pipeline` — undefined (hq/lib/components.mjs pipelines table)
- `grammar.run` — undefined (hq/lib/components.mjs recordReplayRun)
- `grammar.step` — undefined (hq/lib/components.mjs pipeline_steps(run_id, step_id))
- `grammar.task` — undefined (docs/AGENTIC-AUTOMATION-TOOLKIT.md §3)
- `grammar.bundle` — undefined (hq/lib/context.mjs bundleDigest)
- `grammar.receipt` — undefined (hq/lib/receipts.mjs receiptId)
- `grammar.model` — undefined (docs/VIVIM-KB-INTAKE.md §8.1)
- `grammar.feature` — undefined (docs/VIVIM-KB-INTAKE.md §8.2 features)
- `grammar.usecase` — undefined (docs/VIVIM-KB-INTAKE.md §8.2 useCases)
- `grammar.action` — undefined (docs/VIVIM-KB-INTAKE.md §8.2 actions)
- `grammar.flow` — undefined (docs/VIVIM-KB-INTAKE.md §8.2 flows)
- `grammar.source` — undefined (docs/VIVIM-KB-INTAKE.md §8.2 sources)
- `grammar.rule` — An id is immutable. A record whose content changes keeps its id; a record whose id changes is a new record. An id is never reused for a different meaning. 

## 4. Authority

Two vocabularies live here and are never conflated. The `document` vocabulary
labels a file; the `decision` vocabulary labels who decided something.

| id | Vocabulary | Label | Weight | Assignable by | Promotion rule |
| --- | --- | --- | ---: | --- | --- |
| `authority.vendor_documented` | document | `vendor_documented` | 1 | the tool vendor or the official documentation of the provider | Never promoted by HQ; a local file cannot claim it. |
| `authority.project_observed` | document | `project_observed` | 0.9 | the HQ indexer during the census | Observation never becomes current by itself. |
| `authority.local_custom` | document | `local_custom` | 0.7 | a project maintainer | Must not be promoted without owner confirmation. |
| `authority.proposed` | document | `proposed` | 0.5 | any author or agent | Stays proposed until the owner accepts it. |
| `authority.historical` | document | `historical` | 0.2 | the indexer, when a path segment marks archive, history or donor material | Never promoted to current without an owner decision. |
| `authority.owner` | decision | `owner` | — | explicit owner confirmation only | The only value that produces an accepted decision or a current CANON term. |
| `authority.architecture` | decision | `architecture` | — | the architecture decision group | Never promotes to `owner`; it is the default for extracted decisions and terms. |
| `authority.team` | decision | `team` | — | a named team | Never promotes to `owner`. |
| `authority.agent_proposed` | decision | `agent_proposed` | — | an agent acting without authority | Never promotes to `owner`. |
| `authority.source_annotation` | decision | `source_annotation` | — | the extractor, when it reads an @hq:why or @hq:invariant annotation | Never promotes to `owner`; it stays observed. |

Rules:

- `authority.vendor_documented` — An official provider or tool convention. (docs/AGENTIC-RULEBOOK.md §4; hq/lib/canon.mjs DOCUMENT_AUTHORITY)
- `authority.project_observed` — An actual repository file, observed in the census. (hq/lib/canon.mjs classifyDocument)
- `authority.local_custom` — A local convention that has not been verified. (docs/AGENTIC-RULEBOOK.md §4)
- `authority.proposed` — A candidate that has not been adopted. (docs/AGENTIC-RULEBOOK.md §4)
- `authority.historical` — Archived or donor material. It stays historical. (hq/lib/canon.mjs lifecycleFor; buildDocument)
- `authority.owner` — The owner of the record. (hq/lib/canon.mjs AUTHORITY; confirmDecision; confirmTerm)
- `authority.architecture` — Authority recorded by the architecture group. (hq/lib/canon.mjs normalizeAuthorityValue default)
- `authority.team` — Authority recorded by one team. (hq/lib/canon.mjs AUTHORITY)
- `authority.agent_proposed` — A proposal raised by an agent. (hq/lib/canon.mjs AUTHORITY)
- `authority.source_annotation` — A value observed in the source text rather than decided. (hq/lib/canon.mjs annotationDecisions; extractCanonTerms)

## 5. Lifecycle

| id | Value | Meaning | Rule | Promotion rule |
| --- | --- | --- | --- | --- |
| `lifecycle.current` | `current` | The live version of a record. | A current record is the one every surface reads. Only the owner can demote it. | A current record becomes historical only through an owner decision or an explicit supersession. |
| `lifecycle.historical` | `historical` | A record kept because it was once true. | Historical material stays historical and stays visible. | Never becomes current without an owner decision. |
| `lifecycle.generated` | `generated` | A rebuildable view derived from other records. | A generated view is never authoritative and is rewritten on the next build. | It is never promoted; it is replaced. |
| `lifecycle.archived` | `archived` | A record retired but retained. | An archived record is retained, not deleted. | Never promoted automatically. |

- **Drift** — `lifecycle.archived`: hq/lib/canon.mjs lifecycleFor never returns `archived`; it maps archive, archived, history, historical and donor path segments to `historical`. The value is declared by the rule book but emitted by no module.

## 6. Statuses

One enum per record type. The values are the strings the code and the contracts
already use.

### decision

- id: `status.decision`
- values: `proposed`, `accepted`, `rejected`, `superseded`, `deprecated`, `observed`
- terminal: `rejected`, `superseded`, `deprecated`
- rule: Only an explicit owner confirmation moves a decision to `accepted`. A document that claims `Accepted` is extracted as `proposed` and the claim is kept as evidence.
- evidence: hq/lib/canon.mjs DECISION_STATUS, documentStatusOf, confirmDecision, supersedeDecision

| From | To |
| --- | --- |
| `proposed` | `accepted`, `rejected`, `deprecated` |
| `accepted` | `superseded`, `deprecated` |
| `observed` | `proposed` |
| `rejected` | — (terminal) |
| `superseded` | — (terminal) |
| `deprecated` | — (terminal) |

### canon_term

- id: `status.canon_term`
- values: `current`, `provisional`, `deprecated`, `historical`, `observed`
- terminal: `deprecated`, `historical`
- rule: Only an explicit owner confirmation moves a term to `current`. A document that claims `current` is extracted as `provisional`.
- evidence: hq/lib/canon.mjs termStatusOf, confirmTerm, deprecateTerm

| From | To |
| --- | --- |
| `provisional` | `current`, `deprecated` |
| `current` | `deprecated` |
| `observed` | `provisional` |
| `deprecated` | — (terminal) |
| `historical` | — (terminal) |

### document

- id: `status.document`
- values: `unclassified`, `family_01`, `family_02`, `family_03`, `family_04`, `family_05`, `family_06`, `family_07`, `family_08`, `family_09`, `family_10`, `family_11`, `family_12`, `family_13`, `family_14`, `family_15`, `family_16`, `family_17`, `family_18`, `family_19`, `family_20`, `family_21`
- terminal: none
- rule: A document has no manual status. The value is derived from the file path on every index run by the 217/133 registry, and `unclassified` is a real outcome rather than a failure.
- evidence: hq/lib/filetypes.mjs classifyPath; hq/lib/canon.mjs classifyDocument

### task_run

- id: `status.task_run`
- values: `queued`, `running`, `waiting_approval`, `succeeded`, `failed`, `uncertain`, `cancelled`
- terminal: `succeeded`, `failed`, `cancelled`
- rule: An uncertain effect is reconciled read-only before any retry. A blind resend of an uncertain effect is forbidden.
- evidence: docs/AGENTIC-AUTOMATION-TOOLKIT.md §3 and §5; hq/lib/components.mjs pipeline_runs.status

| From | To |
| --- | --- |
| `queued` | `running`, `cancelled` |
| `running` | `waiting_approval`, `succeeded`, `failed`, `uncertain` |
| `waiting_approval` | `running`, `cancelled` |
| `uncertain` | `running`, `failed` |
| `succeeded` | — (terminal) |
| `failed` | — (terminal) |
| `cancelled` | — (terminal) |

### component

- id: `status.component`
- values: `active`, `draft`, `deprecated`, `retired`
- terminal: `retired`
- rule: A published version is immutable. Retirement keeps the row queryable.
- evidence: hq/lib/components.mjs COMPONENT_STATUSES
- **drift:** docs/COMPONENT-REGISTRY.md §5 mentions only `deprecated` and `retired` as moves; the module also allows `draft`, which is the default for a forked pipeline version.

| From | To |
| --- | --- |
| `draft` | `active` |
| `active` | `deprecated` |
| `deprecated` | `retired` |
| `retired` | — (terminal) |

### pipeline

- id: `status.pipeline`
- values: `active`, `draft`, `deprecated`, `retired`
- terminal: `retired`
- rule: A published pipeline version is immutable; a change produces a new version with a parent link.
- evidence: hq/lib/components.mjs PIPELINE_STATUSES

| From | To |
| --- | --- |
| `draft` | `active` |
| `active` | `deprecated` |
| `deprecated` | `retired` |
| `retired` | — (terminal) |

### intake

- id: `status.intake`
- values: `accepted`, `refused`, `partial`
- terminal: `refused`
- rule: A refusal is a receipt, not an exception: a refused package writes nothing.
- evidence: docs/VIVIM-KB-INTAKE.md §5 and §6; hq/lib/kb-intake.mjs ingestKbPackage
- **drift:** No module emits the literal strings. The intake returns `ok: true` or `ok: false` plus `reasons[]` and `missingFromPackage[]`; `accepted` is `ok: true` with nothing missing, `refused` is `ok: false`, `partial` is `ok: true` with files reported missing from the package.

| From | To |
| --- | --- |
| `accepted` | `partial` |
| `partial` | — (terminal) |
| `refused` | — (terminal) |

### bootstrap

- id: `status.bootstrap`
- values: `uninitialized`, `identity`, `capability`, `authority`, `evidence`, `ready`
- terminal: `ready`
- rule: Stages are idempotent and cannot be skipped. `ready` requires owner confirmation and evidence.
- evidence: hq/CONTRACT.md hq/lib/bootstrap.mjs (lane O)
- **drift:** The stage list is declared in the contract; no module in hq/lib exports it yet.

| From | To |
| --- | --- |
| `uninitialized` | `identity` |
| `identity` | `capability` |
| `capability` | `authority` |
| `authority` | `evidence` |
| `evidence` | `ready` |
| `ready` | — (terminal) |

### feature

- id: `status.feature`
- values: `planned`, `partial`, `shipped`, `retired`
- terminal: `retired`
- rule: A feature is declared by the KB. HQ reports which features have flows, actions and evidence; it never promotes one.
- evidence: docs/VIVIM-KB-INTAKE.md §8.2 features

| From | To |
| --- | --- |
| `planned` | `partial`, `retired` |
| `partial` | `shipped`, `retired` |
| `shipped` | `retired` |
| `retired` | — (terminal) |

## 7. Effects

An effect is what a call causes in the world. HQ is read-only by default:
`defaultEffect` is deny and only the read effects are allowed without a scope.

| id | Effect | Description | Default | HQ performs | Requires scope | Approval scope |
| --- | --- | --- | --- | --- | --- | --- |
| `effect.fs:read` | `fs:read` | Read a file from the local disk. | allow | yes | `read` | — |
| `effect.fs:write` | `fs:write` | Create or change a file on the local disk. | deny | yes | `index:write`, `wiki:write`, `hooks:write` | `owner` |
| `effect.db:read` | `db:read` | Read rows from the local index. | allow | yes | `read` | — |
| `effect.db:write` | `db:write` | Write rows to the local index. | deny | yes | `index:write` | `owner` |
| `effect.git:read` | `git:read` | Read git history and refs. | allow | yes | `read` | — |
| `effect.git:write` | `git:write` | Create a commit, a branch or a hook in a repository. | deny | no | `hooks:write`, `owner` | `owner` |
| `effect.network` | `network` | Send or receive data over the network. | deny | no | — | — |
| `effect.browser:effect` | `browser:effect` | Drive a browser or change provider state. | deny | no | — | — |
| `effect.owner:approval` | `owner:approval` | Wait for an owner decision before continuing. | deny | yes | `owner` | `owner` |
| `effect.send` | `send` | Send a message to a provider or a person. | deny | no | — | — |

Rules:

- `effect.fs:read` — Allowed by default under the read scope; secret paths stay excluded by the walker. (hq/lib/components.mjs READ_EFFECT_RE)
- `effect.fs:write` — Denied by default. Requires one of the write scopes or an explicit owner approval. (hq/lib/components.mjs READ_EFFECT_RE; docs/AGENTIC-AUTOMATION-TOOLKIT.md §8)
- `effect.db:read` — Allowed by default under the read scope. (hq/lib/components.mjs READ_EFFECT_RE)
- `effect.db:write` — Denied by default. Requires the index scope or an explicit owner approval. (hq/lib/components.mjs READ_EFFECT_RE; docs/AGENTIC-AUTOMATION-TOOLKIT.md §8)
- `effect.git:read` — Allowed by default under the read scope. A project without git falls back to a directory walk. (hq/lib/lake.mjs; hq/lib/git.mjs)
- `effect.git:write` — Denied by default. HQ installs its own managed hooks and never creates a commit, a branch or a push on its own. (hq/lib/automation.mjs; docs/AGENTIC-AUTOMATION-TOOLKIT.md §8)
- `effect.network` — HQ never performs a network effect. A component that declares one is refused at registration. (hq/lib/components.mjs FORBIDDEN_EFFECT_RE)
- `effect.browser:effect` — HQ never performs a browser effect. A component that declares one is refused at registration. (hq/lib/components.mjs FORBIDDEN_EFFECT_RE; docs/COMPONENT-REGISTRY.md §10)
- `effect.owner:approval` — An approval gate is the only way a write effect may proceed without a standing scope, and the decision is recorded as a receipt. (docs/AGENTIC-AUTOMATION-TOOLKIT.md §6; hq/lib/components.mjs approval)
- `effect.send` — HQ never sends. A component that declares a send effect is refused at registration. (hq/lib/components.mjs FORBIDDEN_EFFECT_RE; hq/lib/nomenclature.mjs invariants)

## 8. Scopes

A scope is a capability label, not permission. It becomes permission only
through a policy decision, and the default policy effect is deny.

| id | Scope | Unlocks |
| --- | --- | --- |
| `scope.read` | `read` | `search`, `find`, `file`, `context`, `brief`, `docs`, `decisions`, `canon`, `kb`, `receipts` |
| `scope.index:write` | `index:write` | `index`, `ingest`, `lake ingest`, `kb intake` |
| `scope.wiki:write` | `wiki:write` | `wiki`, `wiki --changed` |
| `scope.hooks:write` | `hooks:write` | `hooks install`, `watch` |
| `scope.owner` | `owner` | `decisions confirm`, `decisions supersede`, `canon confirm`, `canon deprecate`, `note promote --to canon`, `approval gates` |
| `scope.admin` | `admin` | `project add`, `project remove`, `config`, `reset`, `scope grants` |

Rules:

- `scope.read` — Every agent holds the read scope by default. It opens no write path. (docs/AGENTIC-AUTOMATION-TOOLKIT.md §8)
- `scope.index:write` — Allows writing to the local index. It does not allow writing files outside the index. (hq/lib/indexer.mjs; hq/lib/kb-intake.mjs)
- `scope.wiki:write` — Allows regenerating the derived wiki pages. A generated page is never authoritative. (hq/lib/wiki.mjs)
- `scope.hooks:write` — Allows installing HQ-managed git hooks and the file watcher. (hq/lib/automation.mjs)
- `scope.owner` — The owner scope is what makes a confirmation legal. It is granted by the owner, never by a document. (hq/lib/canon.mjs confirmDecision; docs/AGENTIC-RULEBOOK.md §1)
- `scope.admin` — Covers the registry of spaces and the configuration. It does not imply a send or a browser effect. (hq/bin/hq.mjs)

## 9. Trust

Trust describes where a source came from. It is not the same axis as authority:
a capture is trustworthy evidence of what was seen and carries no authority.

| id | Level | Meaning | Authority label | DCB weight |
| --- | --- | --- | --- | ---: |
| `trust.vendor_documented` | `vendor_documented` | The source is an official provider or tool convention. | `vendor_documented` | 1 |
| `trust.project_observed` | `project_observed` | The source is a file that was actually observed in the repository. | `project_observed` | 0.9 |
| `trust.local_custom` | `local_custom` | The source is a local convention that has not been verified. | `local_custom` | 0.7 |
| `trust.capture` | `capture` | The source is a runtime capture of what a provider actually returned. | — | — |
| `trust.inferred` | `inferred` | The value was derived by HQ rather than read from a source. | — | — |

Rules:

- `trust.vendor_documented` — Scores highest in a context bundle and may surface without a task-token match. (docs/DCB-PROTOCOL.md §3.2)
- `trust.project_observed` — Scores below vendor material and above an unverified local convention. (docs/DCB-PROTOCOL.md §3.2)
- `trust.local_custom` — Scores below observed material; a strong local convention still needs a task match to surface. (docs/DCB-PROTOCOL.md §3.2)
- `trust.capture` — Evidence of what was seen, never a declared standard. The KB model carries capture sources with a trust level; HQ keeps them out of the authority vocabulary. (docs/VIVIM-KB-INTAKE.md §8.2 sources)
  - **drift:** No numeric DCB weight is declared for `capture` in docs/DCB-PROTOCOL.md §3.2; only the five authority labels carry weights.
- `trust.inferred` — Never an authority and never a substitute for evidence. It is recorded as inference, not as fact. (docs/AGENTIC-RULEBOOK.md §12)
  - **drift:** No numeric DCB weight is declared for `inferred` in docs/DCB-PROTOCOL.md §3.2.
- `trust.dcb_scoring` — A context bundle scores each candidate with `1 + 3.6 * coverage + 1.2 * symbolBonus + 0.8 * recency + 0.6 * authority + 0.5 * proximity` and then selects under the token budget with an exact knapsack. The authority term is the only place trust enters the score. (docs/DCB-PROTOCOL.md §4; hq/lib/context.mjs)
  - **drift:** hq/lib/context.mjs docAuthority() does not read the registry authority label. It returns 1 for README.md and AGENTS.md, 0.6 for any other Markdown file and for anything under docs/, and 0 for everything else. The path heuristic is what the code does; the registry weights above are the declared model.

## 10. Families

The 21 document families. The 217 file records and 133 catalog patterns stay in
[`hq/registry/agentic-file-registry.v1.json`](../hq/registry/agentic-file-registry.v1.json);
this section only summarises the families themselves.

| Family | Name | Role | Typical files |
| --- | --- | --- | --- |
| 10 | Tool configuration & hooks | config | `opencode.json`, `opencode.jsonc`, `hooks.json`, `.mcp.json`, `settings.json` |
| 11 | Memory & durable learning | memory | `MEMORY.md`, `NOTES.md`, `PREFERENCES.md`, `LESSONS.md`, `FACTS.md` |
| 12 | State, status & telemetry | state | `STATE.md`, `SITREP.md`, `HEALTH.md`, `METRICS.md`, `TIMING.md` |
| 13 | Session boundaries & recovery | continuity | `HANDOFF.md`, `CHECKPOINT.md`, `RESUME.md` |
| 14 | Work inventory & scheduling | work | `TASKS.md`, `TODO.md`, `BACKLOG.md`, `MILESTONES.md` |
| 15 | Planning & decomposition | plan | `PLAN.md`, `ROADMAP.md`, `WORKSTREAM.md`, `WORKSTREAMS.md` |
| 16 | Execution & operations | operations | `RUNBOOK.md`, `COMMANDS.md`, `DEPLOY.md`, `WORKLOG.md` |
| 17 | Testing & validation | validation | `TEST-PLAN.md`, `VERIFY.md`, `EVAL.md`, `BENCHMARKS.md` |
| 18 | Proof, lineage & audit | evidence | `EVIDENCE.md`, `PROVENANCE.md`, `RECEIPTS.md` |
| 19 | Collaboration & handoffs | collaboration | `INBOX.md`, `ESCALATION.md`, `DECISIONS-NEEDED.md` |
| 20 | Decision & governance record | decision | `DECISIONS.md`, `DECISION_LOG.md`, `ADR.md`, `CONFLICT-REGISTER.md` |
| 21 | Closeout, release & retirement | closeout | `CLOSEOUT_PROTOCOL.md`, `RELEASE.md`, `ROLLBACK.md`, `POSTMORTEM.md` |
| 01 | Agent entry & bootstrap | instruction | `AGENTS.md`, `BOOTSTRAP.md`, `START-HERE.md` |
| 02 | Provider-native instruction rules | instruction | `CLAUDE.md`, `GEMINI.md`, `.cursorrules`, `.cursor/rules/*.mdc`, `.github/copilot-instructions.md` |
| 03 | Agent roles & identity | identity | `ROLE.md`, `PERSONA.md`, `CHARTER.md`, `GENOME.md`, `FLEET.md` |
| 04 | Context & retrieval | context | `CONTEXT.md`, `PROJECT-CONTEXT.md`, `GLOSSARY.md`, `CANON-DICTIONARY.md`, `WORLD.md`, `APERTURE.md` |
| 05 | Law, policy & guardrails | law | `RULES.md`, `POLICY.md`, `INVARIANTS.md`, `AUTHORITY.md`, `GUARDRAILS.md` |
| 06 | Goals & product intent | intent | `VISION.md`, `GOALS.md`, `PRD.md`, `SPEC.md`, `NORTH-STARS.md` |
| 07 | Architecture & interfaces | architecture | `ARCHITECTURE.md`, `CONTRACT.md`, `API.md`, `SCHEMA.md` |
| 08 | Capabilities & discovery | capability | `CAPABILITIES.md`, `TOOLS.md`, `MCP.md`, `INTEGRATIONS.md` |
| 09 | Skills & reusable methods | skill | `SKILL.md`, `PLAYBOOK.md`, `RECIPES.md` |

Rules:

- `family.10` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
- `family.11` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
- `family.12` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
- `family.13` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
  - **drift:** HANDOFF.md is listed under both family 13 (session boundaries) and family 19 (collaboration); the registry decides which one a given path lands in.
- `family.14` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
- `family.15` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
- `family.16` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
- `family.17` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
- `family.18` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
- `family.19` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
  - **drift:** HANDOFF.md is listed under both family 13 (session boundaries) and family 19 (collaboration); the registry decides which one a given path lands in.
- `family.20` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
  - **drift:** The rule book and hq/lib/canon.mjs call this family "Decision & governance"; the registry carries "Decision & governance record". The registry name is the source of record.
- `family.21` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
- `family.01` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
- `family.02` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
  - **drift:** The rule book and hq/lib/canon.mjs call this family "Provider-native rules"; the registry carries "Provider-native instruction rules". The registry name is the source of record.
- `family.03` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
  - **drift:** The rule book and hq/lib/canon.mjs call this family "Roles & identity"; the registry carries "Agent roles & identity". The registry name is the source of record.
- `family.04` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
- `family.05` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
- `family.06` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
- `family.07` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
- `family.08` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)
- `family.09` — Family membership is navigation, not a ranking and not an authority order. (source of record: `hq/registry/agentic-file-registry.v1.json`)

## 11. Vocabulary

Owner surfaces use the owner term. Code, schemas and logs use the engineering
term. The two columns are the same thing, never two vocabularies.

| id | Owner term | Engineering term | Definition |
| --- | --- | --- | --- |
| `vocab.world` | My World | workspace registry + local index | The registry of projects and the local index built over them. |
| `vocab.space` | Space | registered project | A project directory that is registered with HQ and indexed. |
| `vocab.personalAgent` | My Agent | owner-scoped agent surface | The agent surface bound to the owner rather than to one project. |
| `vocab.fleet` | My AI Team | registered agent roster | The roster of agents registered to work on this workspace. |
| `vocab.swarm` | Team Session | parallel agent session | A session in which several agents run at the same time. |
| `vocab.role` | Specialty | agent specialty | The named specialisation an agent is assigned to. |
| `vocab.genome` | Agent DNA | agent manifest (references only, never credentials) | The manifest that describes an agent; it holds references and never credentials. |
| `vocab.memory` | Memory | notes table with tiers + provenance | The tiered note store with provenance that backs Recent, Saved Knowledge and Core Memory. |
| `vocab.work` | Task | task / handoff bundle | A unit of tracked work with a handoff bundle. |
| `vocab.sitrep` | Progress Report | brief | The situation report that summarises the current state. |
| `vocab.capability` | Ability | CLI command or MCP tool | One callable action exposed as a CLI command or an MCP tool. |
| `vocab.authority` | Permissions | scope + approval boundary | The boundary that decides which effect a caller is allowed to cause. |
| `vocab.evidence` | Activity Record | receipts ledger | The append-only ledger of what was read and what was produced. |
| `vocab.aperture` | Discover | context bundle | The bounded bundle of context assembled for one task. |
| `vocab.forge` | Build | wiki generation | Generation of the derived wiki pages. |
| `vocab.host` | VIVIM on my computer | local HQ runtime | The HQ runtime running on the owner machine. |
| `vocab.studio` | Advanced panel | CLI | The command-line surface where the advanced controls live. |
| `vocab.bridge` | Phone Connection | MCP / CLI transport | The transport that connects a remote surface to HQ. |

Rules:

- Owner surfaces use the owner term; code, schemas and logs use the engineering term.
- Memory tiers: `now` = Recent, `later` = Saved Knowledge, `canon` = Core Memory. Recent and Saved Knowledge are added freely; Core Memory requires owner confirmation.
- Evidence: hq/lib/nomenclature.mjs VIVIM_TAXONOMY; docs/AGENTIC-RULEBOOK.md §3

## 12. Non-equivalences

Each pair names two things that look alike and are not the same.

| id | Not the same as | Meaning | Source |
| --- | --- | --- | --- |
| `neq.self-knowledge` | Self-knowledge ≠ authority | What a system knows about itself does not authorise it. | docs/AGENTIC-RULEBOOK.md §12 |
| `neq.grounding` | Grounding ≠ authorization | Being grounded in a source does not authorise an effect. | docs/AGENTIC-RULEBOOK.md §12 |
| `neq.confidence` | Confidence ≠ proof | A confidence score is not a proof. | docs/AGENTIC-RULEBOOK.md §12 |
| `neq.interpretation` | Interpretation ≠ execution | Reading a record is not performing an effect. | docs/AGENTIC-RULEBOOK.md §12 |
| `neq.intent` | Intent ≠ Work | A stated intention is not completed work. | docs/AGENTIC-RULEBOOK.md §12 |
| `neq.representation` | Representation ≠ canonical identity | A displayed form is not the canonical identity of the record. | docs/AGENTIC-RULEBOOK.md §12 |
| `neq.visual-projection` | Visual projection ≠ execution authority | A rendered page carries no execution authority. | docs/AGENTIC-RULEBOOK.md §12 |
| `neq.historical-name` | Historical name ≠ current canonical meaning | The name a record used to carry is not what it means now. | docs/AGENTIC-RULEBOOK.md §12 |
| `neq.document` | Document ≠ law | A document never becomes law by writing itself down. | docs/AGENTIC-RULEBOOK.md §12 |
| `neq.memory` | Memory ≠ authority | A remembered fact is not permission. | docs/AGENTIC-RULEBOOK.md §12 |
| `neq.capability` | Capability ≠ authorization | Being able to act is not being allowed to act. | docs/AGENTIC-RULEBOOK.md §12 |
| `neq.evidence` | Evidence ≠ inference | What was observed is not what was concluded. | docs/AGENTIC-RULEBOOK.md §12 |
| `neq.projection` | Projection ≠ source | The indexed copy of a record is not the record itself. | docs/COMPONENT-REGISTRY.md §8 |
| `neq.bundle` | Bundle ≠ authority | A context bundle is evidence of what was read, never permission to act. | docs/DCB-PROTOCOL.md §8 |
| `neq.wiki-page` | Wiki page ≠ command | A generated page documents a command; it never replaces it. | docs/WIKI-CONSOLE.md §1 |
| `neq.console` | Console ≠ write surface | The console is a read-only lens over the same commands. | docs/WIKI-CONSOLE.md §1 |
| `neq.model` | Model ≠ implementation | A declared model is not the code that implements it. | docs/VIVIM-KB-INTAKE.md §8.4 |
| `neq.use-case` | Use case ≠ acceptance | A use case that cites no evidence is not accepted. | docs/VIVIM-KB-INTAKE.md §8.4 |

## 13. Schema ids

Every versioned contract in the platform, with the module that owns it.

| Schema id | Title | Owner | State |
| --- | --- | --- | --- |
| `hq.context-bundle/1` | Context bundle | hq/lib/context.mjs | implemented |
| `hq.dcb-protocol/1` | DCB protocol | hq/lib/context.mjs | implemented |
| `hq.receipt/1` | Receipt | hq/lib/receipts.mjs | implemented |
| `hq.component/1` | Component | hq/lib/components.mjs | implemented |
| `hq.pipeline/1` | Pipeline | hq/lib/components.mjs | implemented |
| `hq.event/1` | Event | hq/lib/events.mjs (planned) | contract |
| `hq.task/1` | Task | hq/lib/scheduler.mjs (planned) | contract |
| `hq.workflow/1` | Workflow | hq/lib/workflow.mjs (planned) | contract |
| `hq.policy/1` | Policy | hq/lib/policy.mjs (planned) | contract |
| `hq.eval/1` | Eval | hq/lib/evals.mjs (planned) | contract |
| `hq.bootstrap/1` | Bootstrap | hq/lib/bootstrap.mjs (planned) | contract |
| `hq.kb/1` | KB read model | hq/lib/kb.mjs | implemented |
| `hq.coverage/1` | Coverage selection | hq/lib/coverage.mjs (planned) | contract |
| `hq.realtime/1` | Realtime event plan | hq/lib/realtime.mjs | implemented |
| `hq.vivim-bridge/1` | VIVIM bridge | hq/lib/vivim-bridge.mjs (planned) | contract |
| `hq.vivim-model/1` | VIVIM model | hq/lib/kb-model.mjs (planned) | contract |
| `hq.vivim-kb-intake/1` | VIVIM KB intake | hq/lib/kb-intake.mjs | implemented |
| `hq.wiki-console/1` | Wiki console | hq/lib/wiki-console.mjs (planned) | contract |
| `hq.agentic-file-registry/1` | Agentic file registry | hq/lib/filetypes.mjs (loader over hq/registry/agentic-file-registry.v1.json) | implemented |
| `hq.vivim-alignment/1` | VIVIM alignment | hq/lib/vivim.mjs | implemented |

Rules:

- `hq.context-bundle/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (hq/lib/context.mjs:20)
- `hq.dcb-protocol/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (docs/DCB-PROTOCOL.md)
- `hq.receipt/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (hq/lib/receipts.mjs; docs/AGENTIC-AUTOMATION-TOOLKIT.md §3)
- `hq.component/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (hq/lib/components.mjs:12)
- `hq.pipeline/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (hq/lib/components.mjs:13)
- `hq.event/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (docs/AGENTIC-AUTOMATION-TOOLKIT.md §3)
- `hq.task/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (docs/AGENTIC-AUTOMATION-TOOLKIT.md §3)
- `hq.workflow/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (docs/AGENTIC-AUTOMATION-TOOLKIT.md §3)
- `hq.policy/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (docs/AGENTIC-AUTOMATION-TOOLKIT.md §3)
- `hq.eval/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (docs/AGENTIC-AUTOMATION-TOOLKIT.md §3)
- `hq.bootstrap/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (hq/CONTRACT.md hq/lib/bootstrap.mjs (lane O))
- `hq.kb/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (hq/lib/kb.mjs:15)
- `hq.coverage/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (hq/CONTRACT.md hq/lib/coverage.mjs (lane Q))
- `hq.realtime/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (hq/lib/realtime.mjs:14)
- `hq.vivim-bridge/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (hq/CONTRACT.md hq/lib/vivim-bridge.mjs (lane T))
- `hq.vivim-model/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (hq/CONTRACT.md hq/lib/kb-model.mjs (lane V))
- `hq.vivim-kb-intake/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (hq/lib/kb-intake.mjs:33)
- `hq.wiki-console/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (docs/WIKI-CONSOLE.md §7)
- `hq.agentic-file-registry/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (hq/registry/agentic-file-registry.v1.json:2)
- `hq.vivim-alignment/1` — A contract id is versioned. A breaking change publishes a new id; the old id stays readable. (hq/lib/vivim.mjs:11)

## 14. Naming

| id | Style | Applies to | Example |
| --- | --- | --- | --- |
| `naming.ids` | `snake_case` | record ids, table columns and stored JSON field names | source_path, last_verified, binds_to, canon_terms |
| `naming.paths` | `kebab-case` | file path segments and CLI toggles | docs/CANONICAL-TAXONOMY.md, --owner-confirmed, --no-wiki |
| `naming.pascal` | `PascalCase` | type and class names | Evidence, Authority, Receipt |
| `naming.kinds` | `lowercase` | record kinds, statuses, lifecycle values, effects and scopes | decision, canon, current, fs:read, index:write |
| `naming.versions` | `@semver` | component and pipeline versions | hq.tool.index@1.2.0, ^1.2.0 |

Rules:

- `naming.ids` — Machine identifiers are snake_case and stay stable across releases. (hq/lib/schema.mjs; hq/lib/canon.mjs)
- `naming.paths` — Paths use kebab-case segments; top-level documents are written in uppercase kebab-case. (hq/bin/hq.mjs; hq/registry/agentic-file-registry.v1.json)
- `naming.pascal` — PascalCase appears nowhere except CANON Terms, which are written as titles. (hq/lib/nomenclature.mjs VIVIM_TAXONOMY)
- `naming.kinds` — Every enumeration value is lowercase so it can be compared without normalisation. (hq/lib/canon.mjs; hq/lib/components.mjs)
- `naming.versions` — A version is a semver string; a published version is immutable and a change produces a new version. (hq/lib/components.mjs resolveVersion; docs/COMPONENT-REGISTRY.md §2)

## 15. Extension

A new term, entity, status, effect, scope or schema id enters the taxonomy in
this order. There is no other path in.

| id | Step | Definition | Rule |
| --- | --- | --- | --- |
| `extension.propose` | Proposal | A request to add a term, an entity, a status, an effect, a scope or a schema id to the taxonomy. | The proposal names the new item, the reason it is needed and the surfaces that would use it. |
| `extension.evidence` | Evidence | The code, contract or observation that shows the new item is real. | No proposal advances without evidence that points at a file, a contract or a recorded observation. |
| `extension.confirm` | Owner confirmation | The owner accepts the proposal. | Only the owner confirms. A document, an agent or a registry never confirms itself. |
| `extension.version` | New taxonomy version | A published revision of the taxonomy with a new version and a new digest. | A confirmed change publishes a new version. The previous version stays readable so old bundles still verify. |
| `extension.no-silent-edit` | No silent edit | A change that alters the meaning of an existing entry. | Never applied in place. The entry is amended by publishing a new version that records the change. |
| `extension.no-redefinition` | No redefinition | Reusing an existing id for a different concept. | Never allowed. A different concept gets a new id; the old id keeps its meaning and its history. |

- `extension.propose` — docs/AGENTIC-RULEBOOK.md §15
- `extension.evidence` — docs/AGENTIC-RULEBOOK.md §9
- `extension.confirm` — docs/AGENTIC-RULEBOOK.md §1
- `extension.version` — docs/DCB-PROTOCOL.md §12
- `extension.no-silent-edit` — docs/AGENTIC-RULEBOOK.md §15
- `extension.no-redefinition` — docs/AGENTIC-RULEBOOK.md §15

## Counts

| Section | Rows |
| --- | ---: |
| layers | 8 |
| entities | 24 |
| idGrammar | 25 |
| authority | 10 |
| lifecycle | 4 |
| statuses | 9 |
| effects | 10 |
| scopes | 6 |
| trust | 6 |
| families | 21 |
| vocabulary | 19 |
| nonEquivalences | 18 |
| schemaIds | 20 |
| naming | 5 |
| extension | 6 |
| **total** | 191 |
