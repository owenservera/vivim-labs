# Frozen contract — read-only source connector

**Schema:** `hq.connector/1` · **Status:** frozen for waves 1–3 · **Owner:** L4 (build), L0 (change)
**Gate:** D4 · **First connector:** `vivim-host` · **Consumers:** D3, D5, P2, P3

---

## 1. Purpose

A connector observes a source that already has an owner, and lands its **read model** in the kernel
with provenance. It never becomes the source's writer, and it never "fixes" drift by writing back.

`docs/SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md:19`: the kernel is not the owner of source content
or effects. `:39`: source facts stay with the original subsystem; the KB references and verifies.

## 2. Manifest

```json
{
  "schema": "hq.connector/1",
  "id": "vivim-host",
  "label": "VIVIM Host bootstrap ledger",
  "kind": "sqlite",
  "root": "C:\\0-BlackBoxProject-0\\vivim-web-harness\\apps\\vivim-host",
  "database": "runtime/host.db",
  "readOnly": true,
  "tables": [
    {
      "name": "claims",
      "key": "id",
      "columns": ["id", "kind", "status", "created_at"],
      "cursor": { "type": "monotonic", "column": "id" },
      "sensitivity": "private",
      "kernelDomain": "bootstrap.claim"
    }
  ],
  "maxRowsPerRun": 5000,
  "maxBytesPerRow": 65536
}
```

Rules:

- `readOnly` **must** be `true`. A connector manifest declaring `false` is refused
  (`CONNECTOR_NOT_READ_ONLY`). There is no supported write connector in this contract.
- `columns` is an **allowlist**. A column not named is not read, not hashed and not stored. This is
  safer than a denylist and is the only accepted mechanism.
- `sensitivity` is one of `public` / `private` / `restricted`. `restricted` requires an explicit
  owner decision recorded in the roadmap (§11) before the connector may run.
- `kernelDomain` names the durable domain the rows land in.

## 3. Read-only enforcement (four independent mechanisms)

A connector is read-only only when all four hold. A test must assert each one.

1. The SQLite database is opened with `file:...?mode=ro` — a write attempt fails at the driver.
2. `PRAGMA query_only = 1` is set immediately after open.
3. Before and after every run, the source file is hashed (sha256) and the two hashes are compared.
   A difference is `CONNECTOR_SOURCE_MUTATED`, a hard failure, and the run's rows are not committed.
4. The connector module has no write API in its surface: no `put`, `insert`, `exec` with DDL/DML,
   no shell, no network.

Mechanism 4 is a review rule; 1–3 are enforced by the test in §7.

## 4. Cursor and idempotency

- Each table declares one cursor: `monotonic` (id or rowid), `timestamp`, or `revision`.
- The cursor is stored in `kernel_cursors(connector_id, table, cursor, source_revision, observed_at)`.
- A run reads `WHERE cursor > :last`, bounded by `maxRowsPerRun`. A short read is reported, never
  silently treated as complete.
- Content is hashed per row (`content_hash`). An unchanged row is `unchanged`, not re-written.
- A source revision that **moves backwards** is `CONNECTOR_SOURCE_DRIFT`: the run stops, reports,
  and commits nothing. It never resets the cursor automatically.
- A deleted source row becomes a `kernel_tombstones` entry, not a silent disappearance.

## 5. Provenance (mandatory on every row)

| Field | Meaning |
| --- | --- |
| `source_id` | Connector id |
| `source_table` | Table name in the source |
| `source_key` | Primary key value |
| `source_revision` | Cursor value at read time, or the source's own revision if declared |
| `content_hash` | sha256 of the canonical row payload |
| `observed_at` | Time of this read, outside the content hash |

A row without provenance is rejected by the store (`KERNEL_PROVENANCE_REQUIRED`).

## 6. Refusal and error codes

| Code | Meaning |
| --- | --- |
| `CONNECTOR_NOT_READ_ONLY` | Manifest declares `readOnly: false` |
| `CONNECTOR_ROOT_MISSING` | `root` or `database` does not resolve |
| `CONNECTOR_TABLE_MISSING` | A declared table is absent |
| `CONNECTOR_COLUMN_MISSING` | A declared column is absent; the run refuses rather than reading `NULL` |
| `CONNECTOR_SOURCE_DRIFT` | Cursor regressed or source revision changed unexpectedly |
| `CONNECTOR_SOURCE_MUTATED` | Source file hash changed across a run |
| `CONNECTOR_BOUND_EXCEEDED` | Row or byte bound hit; the run is partial and reported as such |
| `CONNECTOR_SECRET_COLUMN` | A declared column matches a secret pattern |

## 7. Acceptance

1. Two consecutive syncs: first `changed: N`, second `changed: 0`, same cursor.
2. Source DB sha256 is identical before and after a full sync (negative test, P3).
3. Opening with a write attempt throws; the test asserts `query_only` is on.
4. A removed source row produces a tombstone, not a gap.
5. A cursor manually rewound produces `CONNECTOR_SOURCE_DRIFT` and commits nothing.
6. A connector pointed at a missing table refuses with the exact table name.
7. A `restricted` sensitivity table without an owner decision is refused.
8. Bound exceeded → partial flag set, counts reported, no silent truncation.

## 8. Out of scope

- Writing to any source, ever.
- Live change-data-capture or streaming.
- Reading files outside the declared root.
- Connectors for provider cookies, browser storage or credentials — those are not data sources.
