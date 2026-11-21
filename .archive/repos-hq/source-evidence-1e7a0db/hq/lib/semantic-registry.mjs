/**
 * @hq:context Versioned stable semantic IDs (`name@version`) for types,
 *   capabilities, actions, schemas, effects, events and contracts, with
 *   aliases and source anchors. The upgradability backbone: a type can evolve
 *   without breaking references.
 * @hq:owner hq
 * @hq:task semantic-registry-v1
 * @hq:invariant No network, no timers, no random source, no browser effects.
 * @hq:invariant A record never grants execution authority. `proposed` becomes
 *   `current` only through an explicit owner confirmation; nothing auto-promotes.
 * @hq:invariant Bad user input fails closed: every exported function returns
 *   `{ ok: false, reason }` or a safe empty result, and never throws.
 * @hq:invariant Deprecation and lookup retain rows. History is never deleted.
 * @hq:see hq/lib/semantic-optimizers.mjs
 * @hq:see docs/research/semantic-language-kernel/CONTRACT-AND-ROADMAP.md
 */
import { statSync } from 'node:fs';
import { isAbsolute, resolve } from 'node:path';
import { nowISO, sha256File, toPosix } from './util.mjs';
import { buildAliasIndex, detectDrift, resolveAlias } from './semantic-optimizers.mjs';

export const SEMANTIC_REGISTRY_SCHEMA = 'hq.semantic-registry/1';

export const SEMANTIC_KINDS = Object.freeze([
  'type', 'capability', 'action', 'schema', 'effect', 'event', 'contract',
]);

export const SEMANTIC_STATUS = Object.freeze(['proposed', 'current', 'deprecated']);

const NAME_RE = /^[a-z0-9][a-z0-9._-]*$/;
const VERSION_RE = /^\d+(\.\d+)*$/;

// Same rule the HQ-wide secret filter uses in canon, filetypes and kb-intake:
// a path is refused when any segment matches. `.env*`, `*.env`, `*.keystore`,
// session.json, storage-state.json, release-profile.json, cookies, login data.
// The check is per segment, so `dir/.env` and `dir.env/file.txt` are both
// refused. `*.pem` and `*.p12` extend the rule to the semantic optimizers'.
const SECRET_BASENAMES = new Set([
  'session.json', 'storage-state.json', 'release-profile.json', 'cookies', 'login data',
]);

const compare = (a, b) => (a === b ? 0 : a < b ? -1 : 1);
const str = (value) => String(value ?? '').trim();
const blank = (value) => str(value).length === 0;

function hasDb(db) {
  return Boolean(db) && typeof db.prepare === 'function';
}

function boundedLimit(value, fallback = 100, maximum = 1000) {
  const n = Number(value);
  if (!Number.isFinite(n)) return fallback;
  return Math.max(0, Math.min(maximum, Math.trunc(n)));
}

/** Parse `name@version`. Returns null when the stable ID is malformed. */
function parseId(value) {
  const id = str(value);
  const at = id.indexOf('@');
  if (at <= 0 || at === id.length - 1) return null;
  if (id.indexOf('@', at + 1) !== -1) return null;
  const name = id.slice(0, at);
  const version = id.slice(at + 1);
  if (!NAME_RE.test(name) || !VERSION_RE.test(version)) return null;
  return { id, name, version };
}

function isSecretSegment(segment) {
  const name = String(segment ?? '').trim().toLowerCase();
  if (!name) return false;
  if (name.startsWith('.env')) return true;
  if (name.endsWith('.env')) return true;
  if (name.endsWith('.keystore')) return true;
  if (name.endsWith('.pem') || name.endsWith('.p12')) return true;
  return SECRET_BASENAMES.has(name);
}

function isSecretPath(value) {
  const path = toPosix(str(value));
  if (!path) return false;
  return path.split('/').some(isSecretSegment);
}

function idRow(row) {
  if (!row) return null;
  return {
    id: row.id,
    kind: row.kind,
    name: row.name,
    version: row.version,
    label: row.label ?? null,
    description: row.description ?? null,
    status: row.status,
    authority: row.authority,
    provenance: row.provenance ?? null,
    created_at: row.created_at,
  };
}

function linkRow(row) {
  if (!row) return null;
  return {
    id: row.id,
    semanticId: row.semantic_id,
    path: row.path,
    line: row.line ?? null,
    symbol: row.symbol ?? null,
    contentHash: row.content_hash ?? null,
    kind: row.kind ?? null,
    status: row.status,
    observedAt: row.observed_at,
  };
}

function findIdRow(db, id) {
  try { return db.prepare('SELECT * FROM semantic_id WHERE id = ?').get(id) ?? null; } catch { return null; }
}

/**
 * Register a stable semantic ID. Identical re-registration is idempotent;
 * different content under the same ID is a conflict, never a silent overwrite.
 * `status: 'current'` is refused — only `promoteSemanticId` may reach it.
 */
export function registerSemanticId(db, entry) {
  const input = entry && typeof entry === 'object' ? entry : {};
  const parsed = parseId(input.id);
  if (!parsed) return { ok: false, id: str(input.id), status: null, reason: 'invalid_id' };
  const id = parsed.id;
  const kind = str(input.kind);
  if (!SEMANTIC_KINDS.includes(kind)) return { ok: false, id, status: null, reason: 'invalid_kind' };
  const requested = str(input.status) || 'proposed';
  if (requested === 'current') return { ok: false, id, status: null, reason: 'owner_confirmation_required' };
  if (requested !== 'proposed') return { ok: false, id, status: null, reason: 'invalid_status' };
  if (!hasDb(db)) return { ok: false, id, status: null, reason: 'unavailable' };

  const label = blank(input.label) ? null : str(input.label).slice(0, 300);
  const description = blank(input.description) ? null : str(input.description).slice(0, 4000);
  const authority = blank(input.authority) ? 'project_observed' : str(input.authority).slice(0, 120);
  const provenance = blank(input.provenance) ? null : str(input.provenance).slice(0, 1000);

  try {
    const existing = findIdRow(db, id);
    if (existing) {
      const same = existing.kind === kind
        && (existing.label ?? null) === label
        && (existing.description ?? null) === description;
      if (!same) return { ok: false, id, status: existing.status, reason: 'duplicate_id' };
      return { ok: true, id, status: existing.status, reason: 'unchanged' };
    }
    db.prepare(
      `INSERT INTO semantic_id(id, kind, name, version, label, description, status, authority, provenance, created_at)
       VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
    ).run(id, kind, parsed.name, parsed.version, label, description, 'proposed', authority, provenance, nowISO());
    return { ok: true, id, status: 'proposed', reason: 'registered' };
  } catch {
    return { ok: false, id, status: null, reason: 'unavailable' };
  }
}

/** proposed -> current, owner-confirmed only. Deprecated IDs never come back. */
export function promoteSemanticId(db, id, options) {
  const target = str(id);
  const ownerConfirmed = (options ?? {}).ownerConfirmed === true;
  if (ownerConfirmed !== true) return { ok: false, id: target, status: null, reason: 'owner_confirmation_required' };
  const parsed = parseId(target);
  if (!parsed) return { ok: false, id: target, status: null, reason: 'invalid_id' };
  if (!hasDb(db)) return { ok: false, id: target, status: null, reason: 'unavailable' };

  try {
    const existing = findIdRow(db, target);
    if (!existing) return { ok: false, id: target, status: null, reason: 'unknown_id' };
    if (existing.status === 'deprecated') return { ok: false, id: target, status: 'deprecated', reason: 'invalid_transition' };
    if (existing.status === 'current') return { ok: true, id: target, status: 'current', reason: 'unchanged' };
    db.prepare('UPDATE semantic_id SET status = ?, authority = ? WHERE id = ?').run('current', 'owner', target);
    return { ok: true, id: target, status: 'current', reason: 'promoted' };
  } catch {
    return { ok: false, id: target, status: null, reason: 'unavailable' };
  }
}

/** Retire an ID without deleting it. Owner-confirmed only. */
export function deprecateSemanticId(db, id, options) {
  const target = str(id);
  const ownerConfirmed = (options ?? {}).ownerConfirmed === true;
  if (ownerConfirmed !== true) return { ok: false, id: target, status: null, reason: 'owner_confirmation_required' };
  const parsed = parseId(target);
  if (!parsed) return { ok: false, id: target, status: null, reason: 'invalid_id' };
  if (!hasDb(db)) return { ok: false, id: target, status: null, reason: 'unavailable' };

  try {
    const existing = findIdRow(db, target);
    if (!existing) return { ok: false, id: target, status: null, reason: 'unknown_id' };
    if (existing.status === 'deprecated') return { ok: true, id: target, status: 'deprecated', reason: 'unchanged' };
    db.prepare('UPDATE semantic_id SET status = ? WHERE id = ?').run('deprecated', target);
    return { ok: true, id: target, status: 'deprecated', reason: 'deprecated' };
  } catch {
    return { ok: false, id: target, status: null, reason: 'unavailable' };
  }
}

/** One semantic ID by its stable ID, or null. */
export function getSemanticId(db, id) {
  const target = str(id);
  if (!parseId(target) || !hasDb(db)) return null;
  try { return idRow(findIdRow(db, target)); } catch { return null; }
}

/** Bounded listing sorted by ID ascending (deterministic, string compare). */
export function listSemanticIds(db, options) {
  const { kind = null, status = null, limit = 100 } = options ?? {};
  if (!hasDb(db)) return [];
  const where = [];
  const params = [];
  if (!blank(kind)) { where.push('kind = ?'); params.push(str(kind)); }
  if (!blank(status)) { where.push('status = ?'); params.push(str(status)); }
  try {
    const rows = db.prepare(
      `SELECT * FROM semantic_id ${where.length ? `WHERE ${where.join(' AND ')}` : ''}`,
    ).all(...params);
    return rows.map(idRow).sort((a, b) => compare(a.id, b.id)).slice(0, boundedLimit(limit));
  } catch {
    return [];
  }
}

/** Attach an alias to an existing ID. Referential integrity is enforced. */
export function registerSemanticAlias(db, entry) {
  const { alias = null, targetId = null, kind = null } = entry ?? {};
  const name = str(alias);
  if (!name) return { ok: false, alias: name, reason: 'invalid_alias' };
  const target = str(targetId);
  if (!target) return { ok: false, alias: name, reason: 'unknown_target' };
  if (!hasDb(db)) return { ok: false, alias: name, reason: 'unavailable' };

  try {
    if (!findIdRow(db, target)) return { ok: false, alias: name, reason: 'unknown_target' };
    const label = blank(kind) ? null : str(kind).slice(0, 120);
    const existing = db.prepare('SELECT * FROM semantic_alias WHERE alias = ? AND target_id = ?').get(name, target);
    if (existing) {
      if ((existing.kind ?? null) === label) return { ok: true, alias: name, reason: 'unchanged' };
      db.prepare('UPDATE semantic_alias SET kind = ? WHERE alias = ? AND target_id = ?').run(label, name, target);
      return { ok: true, alias: name, reason: 'updated' };
    }
    db.prepare('INSERT INTO semantic_alias(alias, target_id, kind) VALUES (?, ?, ?)').run(name, target, label);
    return { ok: true, alias: name, reason: 'registered' };
  } catch {
    return { ok: false, alias: name, reason: 'unavailable' };
  }
}

/** Every alias, sorted by alias then target ID. */
export function listSemanticAliases(db) {
  if (!hasDb(db)) return [];
  try {
    return db.prepare('SELECT alias, target_id, kind FROM semantic_alias').all()
      .map((row) => ({ alias: row.alias, targetId: row.target_id, kind: row.kind ?? null }))
      .sort((a, b) => compare(a.alias, b.alias) || compare(a.targetId, b.targetId));
  } catch {
    return [];
  }
}

/**
 * Resolve a phrase against the registry lexicon. Ambiguity is reported with
 * every target; it is never guessed away and never confers authority.
 */
export function resolveSemanticAlias(db, phrase, options) {
  const prefix = (options ?? {}).prefix === true;
  let rows = [];
  try {
    const aliases = listSemanticAliases(db);
    rows = buildAliasIndex(aliases.map((row) => ({ alias: row.alias, target: row.targetId }))).rows;
  } catch {
    rows = [];
  }
  const index = { schema: 'hq.semantic-optimizers/1', rows };
  const result = resolveAlias(index, phrase, { prefix });
  return { ...result, source: 'registry' };
}

/** Anchor a semantic ID to a source location. Secret paths are refused. */
export function linkSemanticSource(db, link) {
  const input = link && typeof link === 'object' ? link : {};
  const semanticId = str(input.semanticId);
  const path = str(input.path);
  const id = str(input.id) || `${semanticId}::${path}`;
  if (!semanticId || !path) return { ok: false, id, reason: 'invalid_link' };
  if (isSecretPath(path)) return { ok: false, id, reason: 'secret_path' };
  if (!hasDb(db)) return { ok: false, id, reason: 'unavailable' };

  try {
    if (!findIdRow(db, semanticId)) return { ok: false, id, reason: 'unknown_target' };
    const line = Number.isSafeInteger(input.line) && input.line > 0 ? input.line : null;
    const symbol = blank(input.symbol) ? null : str(input.symbol).slice(0, 300);
    const contentHash = blank(input.contentHash) ? null : str(input.contentHash);
    const kind = blank(input.kind) ? null : str(input.kind).slice(0, 120);
    const existing = db.prepare('SELECT * FROM semantic_link WHERE id = ?').get(id);
    if (existing) {
      const same = (existing.line ?? null) === line
        && (existing.symbol ?? null) === symbol
        && (existing.content_hash ?? null) === contentHash
        && (existing.kind ?? null) === kind
        && (existing.semantic_id ?? null) === semanticId
        && (existing.path ?? null) === path;
      if (same) return { ok: true, id, reason: 'unchanged' };
      db.prepare(
        `UPDATE semantic_link SET semantic_id = ?, path = ?, line = ?, symbol = ?, content_hash = ?, kind = ?, observed_at = ?
          WHERE id = ?`,
      ).run(semanticId, path, line, symbol, contentHash, kind, nowISO(), id);
      return { ok: true, id, reason: 'updated' };
    }
    db.prepare(
      `INSERT INTO semantic_link(id, semantic_id, path, line, symbol, content_hash, kind, status, observed_at)
       VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`,
    ).run(id, semanticId, path, line, symbol, contentHash, kind, 'current', nowISO());
    return { ok: true, id, reason: 'linked' };
  } catch {
    return { ok: false, id, reason: 'unavailable' };
  }
}

/** Bounded link listing, sorted by link ID. */
export function listSemanticLinks(db, options) {
  const { semanticId = null, limit = 100 } = options ?? {};
  if (!hasDb(db)) return [];
  try {
    const rows = blank(semanticId)
      ? db.prepare('SELECT * FROM semantic_link').all()
      : db.prepare('SELECT * FROM semantic_link WHERE semantic_id = ?').all(str(semanticId));
    return rows.map(linkRow).sort((a, b) => compare(a.id, b.id)).slice(0, boundedLimit(limit));
  } catch {
    return [];
  }
}

/**
 * Compare pinned link hashes against the current file bytes. Read-only: it
 * reports drift and never rewrites a link or promotes anything. Unreadable
 * files are treated as missing sources.
 */
export function detectSemanticDrift(db, options) {
  const root = (options ?? {}).root ?? null;
  const base = root ? resolve(str(root)) : process.cwd();
  let links = [];
  try {
    links = hasDb(db) ? db.prepare('SELECT * FROM semantic_link').all() : [];
  } catch {
    links = [];
  }
  const records = links.map((row) => {
    const raw = str(row.path);
    const abs = raw && isAbsolute(raw) ? raw : resolve(base, raw);
    let exists = false;
    let actual = null;
    try {
      exists = statSync(abs).isFile();
      if (exists) actual = sha256File(abs);
    } catch {
      exists = false;
      actual = null;
    }
    return {
      id: row.id,
      expectedDigest: row.content_hash ?? null,
      actualDigest: actual,
      expectedVersion: undefined,
      actualVersion: undefined,
      exists,
    };
  });
  const result = detectDrift(records);
  // `checked` is a non-enumerable diagnostic so the return value stays a plain
  // drift array (map/filter/deepEqual unaffected) while still reporting scope.
  Object.defineProperty(result, 'checked', { value: records.length, enumerable: false });
  return result;
}

/**
 * Report the authorization enforcement points for a named registry operation.
 *
 * Read-only: never mutates the DB, never grants authority, never throws.
 * Each point names what is enforced, where in source (file:line), the gate
 * condition, and the decision applied when the gate is not satisfied.
 * Unknown operations return an empty points array (fail closed). Existing
 * exports and their behavior are unchanged; this only surfaces the
 * operation-level allow/deny decisions already present in this module.
 */
export function enforcementPoints(operation) {
  const op = str(operation);
  const byOperation = {
    registerSemanticId: [
      { what: "registration cannot claim 'current' status", where: 'hq/lib/semantic-registry.mjs:132', gate: "requested === 'current'", decision: 'denied' },
    ],
    promoteSemanticId: [
      { what: 'promotion proposed->current requires owner confirmation', where: 'hq/lib/semantic-registry.mjs:164', gate: 'ownerConfirmed === true', decision: 'denied_when_unconfirmed' },
    ],
    deprecateSemanticId: [
      { what: 'deprecation requires owner confirmation', where: 'hq/lib/semantic-registry.mjs:185', gate: 'ownerConfirmed === true', decision: 'denied_when_unconfirmed' },
    ],
  };
  const points = byOperation[op] ?? [];
  return {
    operation: op,
    enforced: points.length > 0,
    points: points.map((p) => ({ ...p })),
  };
}
