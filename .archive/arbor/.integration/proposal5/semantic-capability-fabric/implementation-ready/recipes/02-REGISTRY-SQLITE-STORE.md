# Recipe B1 — authoritative semantic registry + additive derived SQLite
**Owner B:** `hq/lib/semantic-registry.mjs`, `hq/lib/schema.mjs`, new `semantic-capability/{schema,store,registry-adapter}.mjs`. No new database file or graph store.

## Semantic registry
Existing `SEMANTIC_KINDS` = type/capability/action/schema/effect/event/contract; existing `registerSemanticId` forbids status current on creation, `promoteSemanticId(...,{ownerConfirmed:true})` explicitly approves. Keep these transitions EXACT. After review extend `SEMANTIC_KINDS` with `unit`, `role`, `relation`, `migration`, `verification-profile`, `observation-profile`, `explanation-pack`. Registry `name@version` syntax remains unchanged. No edits to approved Host catalog; an owner's proposed ID isn't current merely because referenced in code.

## Schema migration: exact desired SQL (design reference)
```sql
-- Additional DDL in HQ's existing .hq/index.db. Every key source pin includes repo identity.
CREATE TABLE IF NOT EXISTS scf_snapshots (
  pin_id TEXT PRIMARY KEY,
  repo_identity TEXT NOT NULL,
  commit_sha TEXT NOT NULL,
  tree_sha TEXT NOT NULL,
  manifest_digest TEXT NOT NULL,
  registry_digest TEXT NOT NULL,
  owner_catalog_digest TEXT NOT NULL,
  compiler_version TEXT NOT NULL,
  snapshot_digest TEXT NOT NULL,
  completeness TEXT NOT NULL CHECK (completeness IN ('complete','partial','missing','blocked')),
  accepted INTEGER NOT NULL DEFAULT 0 CHECK (accepted IN (0,1)),
  created_at TEXT NOT NULL,
  UNIQUE(repo_identity,commit_sha,manifest_digest,registry_digest,owner_catalog_digest,compiler_version)
);
CREATE TABLE IF NOT EXISTS scf_facts (
  fact_id TEXT PRIMARY KEY,
  pin_id TEXT NOT NULL REFERENCES scf_snapshots(pin_id) ON DELETE RESTRICT,
  subject TEXT NOT NULL,
  predicate TEXT NOT NULL,
  object_ref TEXT NOT NULL,
  path TEXT NOT NULL,
  blob_sha TEXT NOT NULL,
  source_start_line INTEGER NOT NULL CHECK (source_start_line>0),
  source_end_line INTEGER NOT NULL CHECK (source_end_line>=source_start_line),
  scope TEXT NOT NULL CHECK (scope IN ('file','symbol','unknown')),
  declaration_status TEXT NOT NULL,
  static_status TEXT NOT NULL,
  fact_digest TEXT NOT NULL,
  UNIQUE(pin_id,fact_digest)
);
CREATE INDEX IF NOT EXISTS scf_facts_pin_subject ON scf_facts(pin_id,subject);
CREATE INDEX IF NOT EXISTS scf_facts_pin_object ON scf_facts(pin_id,object_ref);
CREATE TABLE IF NOT EXISTS scf_passports (
  pin_id TEXT NOT NULL REFERENCES scf_snapshots(pin_id) ON DELETE RESTRICT,
  capability_id TEXT NOT NULL,
  passport_digest TEXT NOT NULL,
  passport_json TEXT NOT NULL,
  PRIMARY KEY(pin_id,capability_id)
);
CREATE TABLE IF NOT EXISTS scf_diagnostics (
  pin_id TEXT NOT NULL REFERENCES scf_snapshots(pin_id) ON DELETE RESTRICT,
  ordinal INTEGER NOT NULL CHECK(ordinal>=0),
  code TEXT NOT NULL,
  diagnostic_json TEXT NOT NULL,
  PRIMARY KEY(pin_id,ordinal)
);
CREATE TABLE IF NOT EXISTS scf_evidence_refs (
  pin_id TEXT NOT NULL REFERENCES scf_snapshots(pin_id) ON DELETE RESTRICT,
  capability_id TEXT NOT NULL,
  evidence_ref TEXT NOT NULL,
  source_digest TEXT NOT NULL,
  PRIMARY KEY(pin_id,capability_id,evidence_ref)
);
CREATE TABLE IF NOT EXISTS scf_cache_index (
  cache_key TEXT PRIMARY KEY,
  parser_digest TEXT NOT NULL,
  registry_digest TEXT NOT NULL,
  source_blob_digest TEXT NOT NULL,
  payload_digest TEXT NOT NULL,
  last_used_at TEXT NOT NULL
);
```
**Storage rule:** the proposed RESTRICT foreign keys prevent implicit deletion of accepted immutable snapshots. An optional future derived-only garbage collector must explicitly deny removal of accepted pins and remove child rows in one checked transaction. B/G must still prove migration on existing v1 databases, preserving old schema and rows. Only derived SCF records may be removed, never original `files/symbols/imports`/Host World. Snapshot ID must bind complete full SHA source identity (not branch label). No huge source bodies stored in passport_json.

## Proposed API
```ts
export const SCF_DB_SCHEMA="arbor.scf.sqlite/0.1";
export function installScfSchema(db): {ok:boolean;version:string;unchanged:boolean};
export function createScfRegistryAdapter(db,{ownerCatalogDigest}): ReadonlyRegistry;
export function persistScfSnapshot(db,snapshot): {ok:boolean;status:"stored"|"unchanged";digest:string};
export function getScfSnapshot(db,{pinId,requireAccepted=true}): Snapshot|null;
export function listScfPassports(db,{pinId,capabilityId?,limit=100,offset=0}): PassportPage;
export function pruneDerivedScfCache(db,{maxEntries,excludeAcceptedPins}): PruneReceipt;
```

## Transaction/consistency algorithm
1. Validate entire snapshot, canonical digest and all proof refs **before** DB mutation. Verify source input authority on ingest separately.
2. `BEGIN IMMEDIATE` using existing `transaction(db,fn)`, insert staged snapshot (`accepted=0`) and all facts/passports/diagnostics/evidence atomically. Duplicate identical pin/digest returns unchanged; same pin different immutable digest => `SCF_DIGEST_CONFLICT` without update.
3. Re-read record totals and digests before flipping accepted within same transaction. A failed write/storage full rolls back and preserves previous accepted snapshot.
4. On retrieval filter by pin id and authorized development source scope; never use DB row presence as permission. Return paginated sorted records.
5. On registry/source/catalog change create new derived cache generation or explicit invalidation event; immutable snapshots do not mutate.
6. Low disk: stop, rollback, surface `SCF_STORAGE_INCOMPLETE`; keep accepted snapshot, protect Host originals and personal data.

## Required tests
Migrating old `:memory:` and existing v1 DB; repeated migration idempotent; unknown kind proposal/refused promotion; same SHA different digest hard conflict; same capability multiple files; rollback on simulated ENOSPC; source grant revocation returns denial even with existing cached passport; two writers concurrency bounded; upgrade/revert leaves legacy wiki/index intact.
