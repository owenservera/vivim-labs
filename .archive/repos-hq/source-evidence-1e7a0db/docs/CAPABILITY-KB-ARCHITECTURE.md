# Capability and KB architecture — authority, data flow, strategy

**Status:** proposed architecture, implemented in part · **Date:** 2026-11-03
**Scope:** HQ capability registry, the VIVIM knowledge base, and the consumers that may use either.
**Related:** [`CAPABILITY-REGISTRY.md`](CAPABILITY-REGISTRY.md) · [`CONTINUATION-PLAN.md`](CONTINUATION-PLAN.md) §8 · [`KB-LINK.md`](KB-LINK.md) · [`VIVIM-KB-INTAKE.md`](VIVIM-KB-INTAKE.md)

This document is the architecture and strategy layer. It states what owns what, how data moves, which authority each row carries, and the order in which the system earns the right to execute anything. It is deliberately stricter than the implementation: the implementation may lag, but it may not quietly relax a boundary.

---

## 1. The problem in one paragraph

A dependency tree contains useful machinery and unknown risk in the same 8 MB. A personal knowledge base contains owner intent and second-hand claims in the same package. Both need to be searchable and useful; neither may become executable merely because it is present. The architecture therefore separates four questions that are usually collapsed: **what exists** (inventory), **what it says** (documentation), **what it might do** (capability mapping), and **what is allowed** (typed contracts with owner confirmation). HQ answers all four; it never performs the fourth.

---

## 2. System map

```text
                    ┌──────────────────────────────────────────────┐
                    │ HQ — capability authority (read-only)         │
                    │                                              │
 source tree ──────▶│ scanner ──▶ capability_* tables              │
 harvest/update-    │              ├─ package / file / dependency   │
 latest             │              ├─ document + FTS               │
 (lockfile tracked) │              ├─ evidence                     │
                    │              ├─ taxonomy + mapping           │
                    │              └─ contracts (proposed only)    │
                    └───────────────┬──────────────────────────────┘
                                    │ read-only projections
              ┌─────────────────────┼──────────────────────┐
              ▼                     ▼                      ▼
        kbQuery(kind=          wiki capabilities.md    home card
        capability)            (candidate labels)      (counts only)
              ▲
              │ union read model
┌─────────────┴─────────────────────────────────────────────────────┐
│ VIVIM KB — owner knowledge (package intake, evidence only)        │
│ docs/knowledge/*.md, kb-types/sources/jobs/schemas                │
│   ──▶ hq kb intake ──▶ documents                                  │
│                    ──▶ decisions (proposed)                       │
│                    ──▶ canon_terms (provisional)                  │
│                    ──▶ projects (only with a real declared path)  │
└─────────────┬─────────────────────────────────────────────────────┘
              │ approved contract only
              ▼
      Host typed action interface ──▶ VIVIM execution
      (validate, authorize, run, receipt)
```

Two flows meet in the read model and nowhere else. The capability scanner never reads the VIVIM KB; the KB intake never reads the capability tables; `kbQuery` joins their **results**, not their authority.

---

## 3. Authority ladder

Every row in either store sits on one rung. Promotion moves one row, one version, with a receipt.

| Rung | Capability registry | VIVIM KB | Who may promote |
| --- | --- | --- | --- |
| 0 Observed | `capability_package`, `capability_file`, `capability_document`, `capability_evidence` | ingested `documents` | the scanner / intake, automatically, as observations |
| 1 Candidate | `capability_map` (`status=candidate`) | — | the mapper, automatically, with reasons |
| 2 Proposed | `capability_contract` (`status=proposed`, `owner_confirmed=0`) | `decisions` (`proposed`), `canon_terms` (`provisional`) | a lane may draft; no lane may approve |
| 3 Confirmed | contract `owner_confirmed=1`; decision `accepted`; term `current` | same | **owner only**, explicitly, per version |
| 4 Executing | Host action record + receipt | — | Host, only after rung 3 |

Law: **observation is not evidence of behaviour; evidence is not authorization; a document is not law.** The registry can prove a README says a function renders templates. It cannot prove the function is safe, and it cannot grant the call.

---

## 4. Identity and naming law

| Namespace | Shape | Owner | Example |
| --- | --- | --- | --- |
| Capability taxonomy | `<domain>.<node>` | HQ | `templates.render` |
| Capability contract | `cap.<domain>.<action>` with integer `version` | HQ | `cap.templates.render` v1 |
| Source | operator-supplied slug | HQ | `update-latest` |
| Semantic extension | `hq.ext.<slug>` | HQ | `hq.ext.kb-decision` |
| KB type | `KB-*` (unchanged) | VIVIM KB | `KB-DECISION` |
| Package | npm name | upstream | `templates` |

Rules:

1. HQ owns the id grammar. A KB type never becomes an HQ canonical id; it binds as an `extension` row whose `name` preserves the KB id.
2. A collision is recorded as a conflict, never merged and never overwritten.
3. Renaming a display label never changes an id.
4. A contract id is stable across versions; the version is part of the key.

---

## 5. Integration contracts

### 5.1 Scanner → registry

`scanCapabilitySource(root, opts)` returns a pure object; `persistCapabilityScan(db, scan)` writes it in one transaction, idempotent per `source_id`. The digest covers root, lockfile, taxonomy version, contract version, package manifest digests, file hashes and document hashes; it excludes `scanned_at`.

### 5.2 Registry → read model

`kbQuery(db, { query })` unions:

- `documents` / `decisions` / `canon_terms` (existing knowledge),
- `capability_document` via `capability_document_fts` as `kind = 'capability'`,
- package matches via `capability_package_fts` (name/description/keywords/manifest).

The query contract is result-level only: one list, each row names its `source`, `kind`, `path` and `snippet`. A capability row never inherits KB authority and vice versa.

### 5.3 Registry → owner surfaces

The wiki renders `capabilities.md` from tables, never from the source tree, so the page works after the tree is gone. The home card prints counts and one sentence: zero approved contracts. Every row is labelled `candidate` or `proposed`.

### 5.4 Contract → Host

The Host receives a contract only when `status = 'approved'` and `owner_confirmed = 1`. The payload is `{id, version, nodeId, effects, scopes, verification}`. The Host re-validates effects against its own policy; an HQ approval is necessary, not sufficient. HQ never calls the Host, and the Host never reads HQ tables directly.

### 5.5 KB package reality

The real package at `vivim-web-harness/runtime/kb-package` contains **25 files, 0 decisions, 0 terms, 21 unclassified** and **no `KB-*` markers**. Type-aware intake is therefore correct but inert on the current package. Two additions close the gap:

1. **Id binding** — `KB-*`, `SRC-*`, `JOB-*` and schema ids bind as `hq.ext.*` extension entries. This makes the KB vocabulary visible in `hq semantic list` without changing a single KB file.
2. **File classification fallback** — `kb-binding.v1.json` carries optional `filePatterns`; `classifyKbPath(path, binding)` maps a real path to a KB type when the document has no marker. Intake then routes it, with the classification recorded as evidence, not authority.

Neither addition promotes anything. A KB registry document remains a `provisional` observation until the owner confirms a specific entry.

---

## 6. Strategy — five phases, each with an exit gate

| Phase | Outcome | Exit gate | Current |
| --- | --- | --- | --- |
| **P0 Inventory** | Every package, file, field and document is searchable. | `hq capabilities status` → 625 packages / 3,538 files / 652 docs; FTS returns README text; export opens with matching counts. | **done** (digest `86127b0f`) |
| **P1 Bind** | KB types, sources, jobs and schemas bind to HQ ids without migration; the real package classifies without markers. | `hq semantic list --kind extension` shows every bound id; `classifyKbPath` maps all 25 real files; conflicts reported; no canonical row overwritten. | in progress |
| **P2 Unify** | One query answers across code, capability docs, KB docs, decisions and canon. | `hq kb query "template engine"` returns capability + KB rows; `hq kb status` counts decisions/terms. | partial — query union landed; real-package routing pending P1 |
| **P3 Promote** | A reviewed candidate becomes a native HQ implementation with a benchmark. | one contract reaches `approved` with a fixture gate and a measured result; still zero package imports. Protocol: [`CAPABILITY-PROMOTION.md`](CAPABILITY-PROMOTION.md). | not started |
| **P4 Expose** | VIVIM receives only approved contracts through Host. | Host receipt for one approved contract; registry rows unchanged by execution. | not started |
| **P5 Second source** | A second dependency tree proves the scanner is generic. | scan + bind + query on a different tree with no code change. | not started |

P0 is complete. P1–P2 are the current fan-out. P3 is deliberately blocked on the owner, not on engineering.

---

## 7. Ownership matrix

| Concern | HQ | Host | VIVIM | AION |
| --- | --- | --- | --- | --- |
| Package/file/field inventory | owns | reads approved subset | reads UI projection | reads for research |
| Documentation text | owns (copy + FTS) | — | reads | reads |
| Capability taxonomy | owns | — | — | proposes changes |
| Typed action contract | drafts + stores | validates + executes | requests + approves | — |
| Owner confirmation | records | enforces | presents | — |
| KB package production | ingests | — | produces | — |
| KB type ids | binds as extension | — | owns source ids | — |
| Receipts | stores own | emits execution receipt | displays | — |

---

## 8. Non-goals and refusals

- No package from `update@latest` is imported, required, installed or spawned.
- No network, browser, send or credential effect is representable in v1 contracts.
- No automatic promotion of a mapping, contract, decision or term.
- No migration of VIVIM KB ids into HQ canonical ids.
- No second vocabulary: a new KB type maps to an existing HQ id or is explicitly `unbound`.
- No claim that a mapped capability works. Only a fixture gate can claim that.

---

## 9. Risk register

| Risk | Signal | Mitigation |
| --- | --- | --- |
| False confidence from a high mapping score | a 9.0 score read as "safe" | every surface prints `candidate`; contracts require a fixture, not a score |
| Schema drift | DDL in a template literal, concurrent writers | separate `capability-schema.mjs`; one writer per hotspot |
| Export stall | per-row commits on Windows | memory journal + single transaction (measured 2 s) |
| FTS growth | 652 documents / ~2 MB text | per-document cap 512 KiB; no source bodies |
| KB duplication | two id systems | extension binding, conflict report, no merge |
| Authority leakage | a KB decision read as accepted | intake demotes to `proposed`; tests assert it |
| Source loss | OneDrive tree moves | copy into `harvest/update-latest`; lockfile tracked |

---

## 10. Acceptance matrix

| Check | Command | Pass |
| --- | --- | --- |
| Inventory | `hq capabilities status` | 625 / 3,538 / 652 · 16 proposed |
| Fields preserved | `hq capabilities package <name> --json` | `manifest` includes unknown fields |
| Documents searchable | `hq capabilities search "template engine"` | README snippets |
| Determinism | scan twice with pinned `now` | identical digest |
| Idempotency | scan twice | row counts unchanged |
| Export | `hq capabilities export --sqlite FILE` | counts match; < 30 s |
| Contracts | `hq capabilities contracts` | 16 proposed · 0 approved |
| KB bind | `hq semantic list --kind extension` | bound types, conflicts listed |
| Unified query | `hq kb query "<term>"` | capability + KB rows |
| No execution | `rg "child_process|import\\(" hq/lib/capabilities.mjs` | no execution path |

---

## 11. Open owner decisions

| # | Decision | Default in force | Reversible? |
| --- | --- | --- | --- |
| D1 | central kernel vs per-repo | central kernel + thin client | yes, before P4 |
| D2 | bundle contents | engine + coordination ship | yes |
| D3 | coordination owner | HQ git-main-sync | yes |
| D4 | KB id authority | HQ owns ids; KB binds as extension | no after P1 lands |

Until the owner says otherwise, P1–P2 proceed on D4's default. P3 does not proceed without an explicit owner confirmation of one contract.

---

*This document is a projection of current evidence. Where it conflicts with the code or a newer owner decision, the code and the owner win; correct this document.*
