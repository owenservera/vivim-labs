# Labs — the operating scaffold

**This is Labs-local scaffolding.** It is not a platform contract, not a new orchestrator and not a second
knowledge base. It reuses the existing HQ orchestration (`bun hq/bin/hq.mjs …`, `bun scripts/sync.mjs`) and
adds one thing: a small, honest **work-item record** with an enforced maturity ladder, so a Labs result can
never be described as more than it is.

Labs is **not** a KB, a UI publisher, a Host effect authority, a command registry or a product runtime
(vivim-labs `README.md`, `docs/ENGINEERING-EVIDENCE-FACTORY.md:12`). Nothing in this directory grants source,
Host, KB or release authority. Every item names the canonical owner that does hold it.

## What Labs is

From `docs/ENGINEERING-EVIDENCE-FACTORY.md:9-12`:

- **ChatGPT** reduces uncertainty about what VIVIM *should* be — architecture, canonical contracts,
  decisions, source implementation in explicitly reserved Wave 3 paths.
- **ZCode / Labs** reduces uncertainty about what VIVIM *demonstrably* can do — local Git/test operations,
  large-scale experiment, asset and corpus creation, integration verification, environment and device
  validation where available, benchmark and tooling work, evidence collection and reproducibility.
- **Canonical integration authority** — acceptance and promotion into the owning production repos — stays
  with the owners. Labs cannot grant itself Host, KB or UI source authority.

## The eight always-on responsibilities

From `docs/WAVE-BOOSTERS.md:21-30`:

1. **Git/process operations** — safe fetch and fast-forward where eligible, status of dirty and active
   worktrees, source SHAs, write-path conflicts, integration report and evidence linking. No unapproved
   main promotion.
2. **Test operations** — run native applicable test commands, independent clean builds, error and failure
   reproduction, trend monitoring with **real exit codes**, not summary-only model claims.
3. **Catalogue federation** — continuously find references to canonical catalogues while keeping their
   separate owners.
4. **Asset inventory** — propose, specify, test and link reusable components, fixtures, templates, recipes,
   adapters, agents and skills.
5. **Agentic excellence** — document ZCode/HQ tools, models, skills and agent recipes, with independent
   evaluation and no redundant orchestrator.
6. **Security/privacy** — adversarial inputs and denied, uncertain and revoked actor/device states. **No
   synthetic proof inflated to live permission.**
7. **Evidence graph** — link capability, implementation, contract, consumer, test, evidence and platform to
   the Repos HQ canonical identity, **without becoming a second knowledge authority**.
8. **Production preparedness** — continually investigate clean builds, dependencies, release profiles,
   migration and rollback, but only the owner grants live changes.

## The work-item format

The twelve required fields are the evidence record from `docs/ENGINEERING-EVIDENCE-FACTORY.md:94`, which
itself calls it "a discussion-level data list, not a mandatory schema". Labs validates it against itself
and nothing else.

| Field | Meaning |
| --- | --- |
| `task_id` | `LABS-W3-001` style identifier. |
| `source_sha` | The commit the item was written against. |
| `owner` | Who runs the item. |
| `contract_version` | The version of the **Labs work-item contract** (`labs.work-item/1`) the item is written against. The canonical contract a booster targets is named separately in `booster_guidance.canonical_contract`, because Labs does not mint platform contract versions. |
| `inputs` | What the item consumes. |
| `expected_output` | What the item must produce. |
| `allowed_paths` | Repository-anchored absolute paths (`/labs/…`, `/work/…`, `/tests/…`) the item may write. |
| `test_command` | The real command that produces the evidence. Required from evidence level 3. |
| `evidence_level` | The ladder stage actually reached. |
| `result` | The recorded outcome of the last run. |
| `artifact_hash` | The hash of the artefact the run produced. |
| `reviewer` | Who checked it. Required from evidence level 4. |

Optional fields: `factory`, `wave`, `repo`, `maturity` (the stage **claimed**), `maturity_target` (the stage
**aimed at**), `evidence_ceiling` (the highest stage the item's command can justify), `booster_guidance`,
`results`, `last_result`, `updated`.

A work item lives in one file: `work/work-items.json`. The write is atomic (temporary file plus rename)
and serialized by a lockfile, so two Labs workers cannot interleave a read-modify-write. The W3 plan lives
in `work/boost-plan.json` and is the set of work orders; the ledger holds the recorded results.

## The maturity ladder

Six stages, from `docs/ENGINEERING-EVIDENCE-FACTORY.md:62-68`:

| Level | Stage | Meaning |
| --- | --- | --- |
| 1 | discovered | Item identified. |
| 2 | specified | Contract, inputs, outputs, risk and boundaries defined. |
| 3 | unit verified | Isolated deterministic tests pass. |
| 4 | integrated | Actual neighboring production modules interoperate. **Mocked neighbours do not count.** |
| 5 | environment verified | The relevant real browser, OS or device executed the item. |
| 6 | production verified | Owner-authorized release and real operational receipts. |

**The ladder is enforced, not advisory.** `validateMaturityClaim({ claimed, evidence_level })` refuses any
claim above the evidence held, so a fixture-only result can never be described as real Android, Host or
browser proof. `labs:verify` only ever raises an item to the stage its command actually justifies; the
default ceiling is `unit verified`, and a failing command drops the item back to `specified` rather than
leaving a stale verification in place.

`maturity` is a claim and is judged. `maturity_target` is a plan and is not. A plan may target
`production verified`; only an owner can promote an item there.

## Commands

| Command | What it does |
| --- | --- |
| `bun run labs:plan` | Prints the W3 booster set (factory, wave, maturity target, allowed paths, test command) and **fails** if two items share a write scope. |
| `bun run labs:validate` | Validates every work item against the model. Exit 1 if any item is invalid. |
| `bun run labs:verify -- <task_id>` | Runs that item's `test_command` for real, captures the exit code and the output tail, records the result and updates the evidence level only as far as the evidence supports. A failure is recorded as a failure. |
| `bun run labs:status` | `task_id \| factory \| wave \| maturity \| evidence_level \| last_result \| source_sha \| updated`. |
| `bun run labs:evidence` | The capability → implementation → contract → consumer → test → evidence → platform **linking index**. It names the canonical owner of each relation; it copies no authority. |
| `bun run labs:capacity` | A measured machine and fleet capacity profile as JSON. |
| `bun run labs:report` | The markdown readiness summary: counts by factory, by maturity, by evidence level, and the blockers. |
| `bun test` | The whole suite: the scaffold tests in `tests/` and the per-item booster acceptance tests in `labs/tests/`. |
| `bun run typecheck` | TypeScript 7 over every `// @ts-check` module. |

## Forbidden paths, and where they came from

`labs/forbidden-paths.mjs` is **data**, not code. Every entry names its owner and the document line it was
read from, so a refusal can be checked against the owning repository.

| Reserved space | Owner | Source |
| --- | --- | --- |
| Eight Wave 3 lane source/test pairs, e.g. `/src/visual-runtime/document/wave3-action-semantics.ts` | tierflow lanes A–H | `vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:9-16` |
| `/src/visual-runtime/integration-2/**`, `/tests/visual-runtime/integration-2/**` | tierflow integrator | `…/WAVE-3-OWNERSHIP.md:18` |
| Lane handoff and evidence paths | tierflow lanes A–H | `…/WAVE-3-OWNERSHIP.md:18` |
| `/hq/bin/hq.mjs`, `/hq/lib/schema.mjs`, `/package.json` and the three ledger files | repos_hq L0 (integrator), shared-serial | `repos_hq docs/integration/archive/lanes.v1.json:26-32`, `docs/integration/archive/VIVIM-INTEGRATION-ROADMAP.md:358-359` |
| `/docs/integration/**` | repos_hq L0 | `repos_hq docs/integration/archive/lanes.v1.json:52` |
| Studio channel manager-merge-only paths | vivim-web-harness channel manager | `vivim-labs docs/WAVE-3-READINESS-HANDOFF.md:190-193` |

A path only means something inside a repository, so a forbidden entry is matched against the repository the
work item is scoped to. In addition, a work item may only write inside the Labs write scope (`/labs`,
`/work`, `/tests`, plus the root `package.json` and `tsconfig.json`): **Labs may not write another
repository's source**, and cannot grant itself the authority to.

## Measured capacity, and the constraints it imposes

`bun run labs:capacity` measures this machine at run time — CPU cores, memory, free disk on `C:`, the
toolchain versions, the local seat roster, the agent definition files, and the spawn caps read out of the
local HQ code. **No number in it is a stored estimate.** A source that cannot be read is reported as null
with the reason.

Measured on this machine (2026-11-06):

- 8 logical CPU cores; 21.90 GiB RAM total, roughly 12.3–12.5 GiB free at that moment.
- **C: has only about 31.6 GiB free.** Disk, not CPU, is the first local wall: 24 isolated checkouts would
  need 4.8–24 GiB on top of that.
- bun 1.4.2, node v24.22.1, git 2.51.2.windows.1, cargo 1.99.1, TypeScript 7.0.2. The ZCode CLI is
  0.17.1 and is **not on PATH** (only a `.cmd` shim); it has no `seat` or `spawn` subcommand.
- Provider concurrency reality: the highest number of in-flight provider calls ever observed was 18, and 904
  of 1,163 recorded errors (77.6%) were `rate_limited`. Real quota is not measurable without a billable
  call, so treat provider throughput as unproven.
- Spawn caps read from `repos_hq/hq/lib/spawn.mjs`: `DEFAULT_LIMITS = { maxDepth: 3, maxChildren: 4 }` and
  `FLEET_LIMITS = { maxDepth: 4, maxChildren: 12 }`. The CLI passes no non-default limit and has no
  `--max-children` flag, so the documented 3 × 8 topology is currently refused with `FANOUT_EXCEEDED`.
  **The W3 plan therefore schedules at most four concurrent items per conversation**, not eight.
- The highest fan-out ever demonstrated through `hq/bin/spawn.mjs` was 6 workers in 14 ms.
- `hq team` reports 53 high-severity defects: every agent definition pins a provider rule whose declared
  reasoning levels do not include the one it asks for, so **those agents do not launch**. The plan does not
  depend on named HQ agents: it assumes free-form ZCode conversations.

## What this scaffold reuses rather than reinvents

- **One file plus one proof command per work order**, the discipline `bun hq/bin/hq.mjs work list|claim|done`
  already uses for its orders. The Labs ledger is the same shape, kept Labs-local.
- **Coordination over the existing bus**: `bun scripts/sync.mjs check --as <id>` (exit 3 means unread),
  `send`, `claims`. `bun hq/bin/hq.mjs seat wake <seat> --prompt "<slice>"` and `seat verify` for a
  receipt-grade wake proof.
- **Context and verification**: `bun hq/bin/hq.mjs context "<task>" --budget N`, `context --verify bundle.json`,
  `bun hq/bin/hq.mjs verify` (writes no file) and `handoff`.
- **Routing and roster**: `bun hq/bin/hq.mjs route "<task>"`, `project add`, and `route-model` (read-only, the
  local quota proxy).
- **The canonical graph** in `repos_hq:hq/lib/graph.mjs` stays the fact authority; `labs:evidence` links to
  it and to every other owner's surface.
- **The sibling Labs lanes' evidence records** under `evidence/` are linked, not duplicated: see
  [Linked evidence records](#linked-evidence-records).

## The W3 booster plan

`work/boost-plan.json` holds eleven work items across factories 01, 02, 03, 04, 05, 07, 09 and 12. Every
allowed path is inside `vivim-labs` and the scopes are disjoint by construction; `labs:plan` fails if that
ever stops being true. Factory 06 (repeatable clean builds) is deliberately absent **from the plan items**:
`docs/WAVE-BOOSTERS.md` places clean-build reproducibility in W4 and the release scaffold in W6. It is
present as *linked evidence* instead — see below.

Each item ships a first slice of real content under `labs/fixtures/` and a real acceptance command under
`labs/tests/`, so `labs:verify` can be run today and the plan never depends on a file another lane is still
generating. Where a sibling corpus is the real proof, the item names it in `inputs` as a description, never
in its `test_command`.

The campaign volumes in the program document — 10k natural-language cases, 1k candidates, 500 assets, 2k
adversarial cases — are **targets, not claims**. A first slice proves the shape of the work, never its size,
and volume never proves correctness.

## Linked evidence records

Other Labs lanes own evidence records in `evidence/`. `labs:report` and `labs:evidence` **link** them and
read their measured numbers at run time, straight out of the owning file, so the report can never drift
from the evidence it points at. Labs copies none of their content and asserts none of their claims.

| record | factory | owner |
| --- | --- | --- |
| `evidence/repro/clean-build-reproducibility.md` | 06 | labs clean-build lane |
| `evidence/adversarial/adversarial-suite-results.md` | 04, 05 | labs adversarial lane |
| `evidence/catalogue/federated-catalogue-findings.md` | 02, 03 | labs catalogue lane |
