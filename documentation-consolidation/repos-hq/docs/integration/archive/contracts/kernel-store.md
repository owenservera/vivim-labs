# Frozen contract — durable kernel store

**Schema:** `hq.kernel-store/1` · **Status:** frozen for waves 1–3 · **Owner:** L1 (build), L0 (change)
**Gate:** D1 · **Supersedes:** nothing · **Consumers:** L2, L3, L4, D5 parity

A change to this document requires a version bump, a sync-bus question on
`integration-contract`, and an L0 ack. Lanes may not edit this file.

---

## 1. Why this store exists

HQ's current state lives in `.hq/`, which `AGENTS.md:95` declares **rebuildable and gitignored**,
and `docs/SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md:30` forbids for durable knowledge. The
kernel store is the first HQ store whose loss is a data loss, not a rebuild.

## 2. Location and runtime

| Decision | Value | Reason |
| --- | --- | --- |
| Default path | `<repo>/kernel/kernel.db` | Outside `.hq/`; survives `hq reset` |
| Override | `HQ_KERNEL_DB` environment variable | Tests and alternate installs |
| Runtime | `node:sqlite` (`DatabaseSync`) | Verified working on Node v24.22.1 **and** Bun 1.4.2; avoids the `bun:sqlite` lock-in recorded as GAP-010 |
| Journal | WAL, `synchronous = FULL`, `busy_timeout = 5000` | Same discipline as the existing stores |
| Writer | exactly one process | Single-writer lock, §4 |
| Network | none | Local-only, like every other HQ surface |

The path is a **product decision** recorded as O2 in the roadmap. If the owner moves the default,
the schema id does not change; only the path constant does.

## 3. Schema

Minimum tables. Column lists are normative for the named keys; additional columns are additive
and require a version bump only when a consumer reads them.

| Table | Keys | Purpose |
| --- | --- | --- |
| `kernel_meta` | `key`, `value` | `user_version`, install id, created_at |
| `kernel_revisions` | `seq` (monotonic), `domain`, `target_id`, `revision`, `content_hash`, `parent_revision`, `actor`, `source`, `created_at`, `payload` | **Append-only.** One row per accepted mutation |
| `kernel_entries` | `id`, `domain`, `kind`, `title`, `status`, `authority`, `sensitivity`, `current_revision`, `content_hash`, `deleted_at` | Current pointer per entry |
| `kernel_entities` | `id`, `title`, `kind`, `owner`, `fields` (json), `relations` (json), `invariants` (json), `source` | Model section `entities` |
| `kernel_types` | `id`, `kind`, `of`, `values` (json) | Model section `types` |
| `kernel_schemas` | `id`, `version`, `target`, `json_schema` (json) | Model section `schemas` |
| `kernel_flows` | `id`, `title`, `actors`, `steps`, `inputs`, `outputs`, `authority`, `source` | Model section `flows` |
| `kernel_sources` | `id`, `kind`, `trust`, `owner`, `description` | Model section `sources` |
| `kernel_actions` | `id`, `title`, `actor`, `effects`, `authority`, `idempotency`, `input_schema`, `output_schema` | Model section `actions` |
| `kernel_authority` | `id`, `subject`, `may`, `may_not`, `requires_owner`, `rationale` | Model section `authority` |
| `kernel_features` | `id`, `title`, `status`, `flows`, `actions` | Model section `features` |
| `kernel_usecases` | `id`, `title`, `actor`, `feature`, `flow`, `given`, `when`, `then`, `evidence` | Model section `useCases` |
| `kernel_provenance` | `target_id`, `source_id`, `source_table`, `source_key`, `source_revision`, `content_hash`, `observed_at` | Where a row came from |
| `kernel_tombstones` | `target_id`, `reason`, `source`, `created_at` | Deletion is a record, not a `DELETE` |
| `kernel_imports` | `id`, `schema`, `package_digest`, `counts`, `result`, `created_at` | Import receipts |
| `kernel_connectors` | `id`, `kind`, `root`, `read_only`, `manifest`, `last_run_at`, `status` | Connector registry |
| `kernel_cursors` | `connector_id`, `table`, `cursor`, `source_revision`, `observed_at` | Incremental reads |
| `kernel_receipts` | `id`, `action`, `subject`, `result`, `evidence`, `created_at` | Evidence ledger for kernel operations |

`kernel_lock` is a single-row table: `owner_pid`, `owner_started_at`, `acquired_at`.

## 4. Invariants

1. **Append-only history.** `kernel_revisions` rows are never updated or deleted. A rollback writes
   a new revision whose `payload` references the older one.
2. **One writer.** Acquiring the lock twice from different pids returns `KERNEL_LOCKED` with the
   current owner. Stale locks (pid gone) may be reclaimed only with an explicit `--force` that
   writes a receipt.
3. **Transactional import.** One package is one transaction. A refusal writes nothing, including
   no `kernel_imports` success row — the refusal is its own receipt with `result: 'refused'`.
4. **Outside `.hq/`.** `hq reset` must not touch the kernel DB. The test asserts the file hash is
   unchanged after reset.
5. **No secrets.** A value matching the intake secret patterns is refused, never stored, never
   hashed into a receipt.
6. **Deterministic content hash.** `content_hash = sha256(canonical_json(payload))` with sorted
   keys and no timestamps inside the hashed payload.
7. **Provenance is mandatory.** A row with no `kernel_provenance` entry is invalid; the store
   rejects it rather than accepting an unattributed fact.

## 5. Commands the store must expose

```ts
initKernel({ dbPath })                       // create + migrate, idempotent
kernelWrite(domain, targetId, payload, {actor, source, parentRevision}) // -> revision
kernelRead(domain, targetId, {revision = 'current'})
kernelHistory(domain, targetId, {limit = 50})
kernelVerify()                                // -> {ok, chain[], broken[]}
kernelBackup(destPath)                        // SQLite backup API, not file copy
kernelRestore(srcPath, {expectDigest})
```

Command names are descriptive, not final. The **semantics** are final: revision allocation, current
pointer, provenance and receipt commit atomically.

## 6. Refusal codes

| Code | Meaning |
| --- | --- |
| `KERNEL_LOCKED` | Another process owns the writer lock |
| `KERNEL_SCHEMA_MISMATCH` | `user_version` is newer than the code |
| `KERNEL_ORDINAL_GAP` | Revision chain is broken; verify failed |
| `KERNEL_PROVENANCE_REQUIRED` | Write attempted without provenance |
| `KERNEL_SECRET_REFUSED` | Payload matched a secret pattern |
| `KERNEL_PATH_FORBIDDEN` | A caller tried to open the store inside `.hq/` |

## 7. Acceptance

1. `bun test hq/test/kernel-store.test.mjs` — create, write, read, history, verify.
2. A write survives `hq reset` + `hq index` byte-identically.
3. A second process returns `KERNEL_LOCKED`; a stale lock is reclaimable with a receipt.
4. Rollback appends a revision; `kernel_history` length increases by one.
5. `kernelVerify` detects a manually deleted revision row and reports the gap.
6. Opening a path under `.hq/` is refused.
7. `kernelBackup` + `kernelRestore` round-trips with an identical `kernelVerify` chain.

## 8. Out of scope

- Multi-host replication and consensus.
- Encryption at rest (recorded as an owner decision if the kernel ever holds private content).
- Automatic backup scheduling.
- Any provider, browser or device effect.
