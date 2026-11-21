// Sprint 02 / Lane 3: pure, fail-closed Rust comment-program -> HQ wiki bridge.
// This is not an effect dispatcher. Only the existing Host/Studio action owner may execute.
import { createHash } from 'node:crypto';
import { isVersionedId } from './annotation-v2.mjs';
import { getSemanticId } from './semantic-registry.mjs';
import { route } from './action-router.mjs';
import {
  validateProgramConfig, previewProgram, activateProgram, rollbackProgram,
  getActiveProgram, listProgramVersions,
} from './wiki-program.mjs';

export const COMMENT_PROGRAM_SCHEMA = 'hq.comment-program/0.1';
export const COMMAND_INTENT_SCHEMA = 'vivim.command-intent/0.1';
const EVENT_TYPES = new Set(['click', 'submit', 'change']);
const MAX_DOCUMENT_BYTES = 262144;
const SAFE_KEYS = new Set(['schema', 'compilerVersion', 'sourceMap', 'config', 'bindings', 'diagnostics']);
const SECRET_SEGMENTS = /^(?:\.env.*|.*\.env|.*\.(?:pem|p12|keystore)|cookies|login data|session\.json|storage-state\.json|release-profile\.json)$/i;
const BANNED_KEYS = /^(?:__proto__|prototype|constructor|command|exec|shell|script|eval|spawn|token|secret|password|provider|sudo|grant|permissions)$/i;
const UNSAFE_VALUE = /(?:https?:\/\/|file:\/\/|\$\{|\b(?:eval|exec|shell|sudo|spawn|publish|privilege)\b)/i;
const cmp = (a, b) => a < b ? -1 : a > b ? 1 : 0;
const obj = (value) => value !== null && typeof value === 'object' && !Array.isArray(value) && (Object.getPrototypeOf(value) === Object.prototype || Object.getPrototypeOf(value) === null);
const string = (value) => typeof value === 'string' && value.trim().length > 0;
const posInt = (value) => Number.isSafeInteger(value) && value > 0;
const error = (errors, code, path) => errors.push({ code, path });
function canonical(value) {
  if (Array.isArray(value)) return value.map(canonical);
  if (obj(value)) return Object.fromEntries(Object.keys(value).sort(cmp).map((k) => [k, canonical(value[k])]));
  return value;
}
function digest(value) { return createHash('sha256').update(JSON.stringify(canonical(value))).digest('hex'); }
function safePath(path) {
  return string(path) && path.length <= 512 && !path.includes('\\') && !path.startsWith('/')
    && !/^[a-z]:/i.test(path) && !path.includes('\0') && !path.includes('//')
    && path.split('/').every((p) => p !== '' && p !== '.' && p !== '..' && !SECRET_SEGMENTS.test(p));
}
function safeData(value, depth = 0) {
  if (depth > 8) return false;
  if (value === null || typeof value === 'boolean') return true;
  if (typeof value === 'number') return Number.isFinite(value);
  if (typeof value === 'string') return value.length <= 1024 && !UNSAFE_VALUE.test(value);
  if (Array.isArray(value)) return value.length <= 32 && value.every((v) => safeData(v, depth + 1));
  if (obj(value)) return Object.keys(value).length <= 32 && Object.entries(value).every(([k, v]) =>
    /^[a-zA-Z][a-zA-Z0-9_-]{0,63}$/.test(k) && !BANNED_KEYS.test(k) && safeData(v, depth + 1));
  return false;
}
function readInput(input, errors) {
  if (typeof input !== 'string') return input;
  if (Buffer.byteLength(input, 'utf8') > MAX_DOCUMENT_BYTES) {
    error(errors, 'oversized_manifest', ''); return null;
  }
  try { return JSON.parse(input); } catch { error(errors, 'invalid_json', ''); return null; }
}
function lookupAction(binding, options) {
  const row = options.lookupSemantic
    ? options.lookupSemantic(binding.actionId)
    : options.db ? getSemanticId(options.db, binding.actionId) : null;
  if (!row || row.id !== binding.actionId || row.kind !== 'action' || row.status !== 'current') return null;
  const plainId = binding.actionId.slice(0, binding.actionId.lastIndexOf('@'));
  const action = Array.isArray(options.actions) ? options.actions.find((a) => a?.id === plainId) : null;
  if (!action || action.status !== 'wired' || !obj(action.handler) || action.mutates !== false && action.mutates !== true
      || !['low', 'medium', 'high'].includes(action.risk) || !Array.isArray(action.actors) || !action.actors.length
      || (action.risk === 'high' && action.approval !== true)) return null;
  return action;
}

/** Validate untrusted compiler output, including real semantic/action registry resolution. */
export function validateCommentProgram(input, options = {}) {
  const errors = [];
  const m = readInput(input, errors);
  if (!obj(m)) { if (!errors.length) error(errors, 'invalid_manifest', ''); return { ok: false, errors }; }
  for (const k of Object.keys(m)) if (!SAFE_KEYS.has(k)) error(errors, 'unknown_key', k);
  if (m.schema !== COMMENT_PROGRAM_SCHEMA) error(errors, 'invalid_schema', 'schema');
  if (!string(m.compilerVersion) || !/^\d+\.\d+\.\d+(?:[-+][a-zA-Z0-9.-]+)?$/.test(m.compilerVersion)) error(errors, 'invalid_compiler_version', 'compilerVersion');
  const sm = m.sourceMap;
  if (!obj(sm) || Object.keys(sm).some((k) => !['path', 'lineStart', 'lineEnd', 'digest'].includes(k))
      || !safePath(sm?.path) || !posInt(sm?.lineStart) || !posInt(sm?.lineEnd)
      || sm.lineEnd < sm.lineStart || !/^[a-f0-9]{64}$/.test(sm.digest ?? '')) error(errors, 'invalid_source_map', 'sourceMap');
  const configValidation = validateProgramConfig(m.config);
  for (const e of configValidation.errors) error(errors, e.code, `config${e.path ? '.' + e.path : ''}`);
  if (!obj(m.config) || !string(m.config.id) || !string(m.config.source)) error(errors, 'missing_config_fields', 'config');
  if (!Array.isArray(m.diagnostics) || m.diagnostics.length > 100 || m.diagnostics.some((d) =>
    !obj(d) || !['info', 'warning'].includes(d.severity) || !string(d.code) || !string(d.message)
    || !safeData(d.code) || !safeData(d.message))) error(errors, 'unsafe_diagnostics', 'diagnostics');
  if (!Array.isArray(m.bindings) || m.bindings.length > 128) error(errors, 'invalid_bindings', 'bindings');
  else {
    const ids = new Set();
    for (let i = 0; i < m.bindings.length; i++) {
      const b = m.bindings[i]; const path = `bindings[${i}]`;
      if (!obj(b) || Object.keys(b).some((k) => !['id', 'event', 'actionId', 'requiredScopes', 'args'].includes(k))) {
        error(errors, 'invalid_binding', path); continue;
      }
      if (!string(b.id) || !/^[a-z0-9][a-z0-9._-]{0,127}$/.test(b.id) || !EVENT_TYPES.has(b.event)
          || !isVersionedId(b.actionId) || !Array.isArray(b.requiredScopes)
          || b.requiredScopes.length === 0 || b.requiredScopes.length > 16
          || b.requiredScopes.some((s) => !isVersionedId(s))
          || !obj(b.args ?? {}) || !safeData(b.args ?? {})) {
        error(errors, 'invalid_binding', path); continue;
      }
      const key = `${b.id}\0${b.event}`;
      if (ids.has(key)) error(errors, 'duplicate_binding', path);
      ids.add(key);
      if (!lookupAction(b, options)) error(errors, 'unregistered_or_unwired_action', `${path}.actionId`);
    }
  }
  errors.sort((a,b) => cmp(a.path,b.path) || cmp(a.code,b.code));
  return { ok: errors.length === 0, errors, ...(errors.length ? {} : { program: m, manifestDigest: digest(m) }) };
}

/** Preview both HQ config diff and source/binding provenance; never writes. */
export function previewCommentProgram(db, input, options = {}) {
  const checked = validateCommentProgram(input, { ...options, db });
  if (!checked.ok) return checked;
  const { program: m, manifestDigest } = checked;
  const wiki = previewProgram(db, m.config);
  const active = getActiveProgram(db, m.config.id);
  return { ok: wiki.ok, errors: wiki.errors, schema: COMMENT_PROGRAM_SCHEMA, config: m.config,
    manifestDigest, sourceMap: m.sourceMap, compilerVersion: m.compilerVersion,
    bindings: m.bindings.map((b) => ({ ...b, args: canonical(b.args ?? {}) })),
    sourceChanged: Boolean(active && active.receipt?.manifestDigest !== manifestDigest),
    wiki, expectedRevision: wiki.revision };
}

/** Explicit second-step confirmation; stale revision or preview digest fails closed. */
export function activateCommentProgram(db, input, options = {}) {
  if (options.ownerConfirmed !== true) return { ok: false, reason: 'owner_confirmation_required' };
  const preview = previewCommentProgram(db, input, options);
  if (!preview.ok) return { ok: false, reason: 'invalid_comment_program', errors: preview.errors };
  if (!string(options.expectedPreviewDigest) || options.expectedPreviewDigest !== preview.wiki.digest
      || !Number.isSafeInteger(options.expectedRevision) || options.expectedRevision !== preview.wiki.revision
      || options.expectedManifestDigest !== preview.manifestDigest)
    return { ok: false, reason: 'stale_or_unreviewed_preview' };
  const receipt = { schema: 'hq.comment-wiki-receipt/0.1', manifestDigest: preview.manifestDigest,
    sourceMap: preview.sourceMap, compilerVersion: preview.compilerVersion,
    bindings: preview.bindings, previewDigest: preview.wiki.digest };
  return activateProgram(db, preview.config, { ownerConfirmed: true, receipt });
}

/** Undo creates a new historic version, preserving the old version's bindings. */
export function rollbackCommentProgram(db, { id, toVersion, ownerConfirmed, expectedRevision } = {}) {
  if (ownerConfirmed !== true) return { ok: false, reason: 'owner_confirmation_required' };
  const active = getActiveProgram(db, id);
  if (!active || expectedRevision !== active.revision) return { ok: false, reason: 'stale_revision' };
  const target = listProgramVersions(db, { id, limit: 1000 }).find((x) => x.version === toVersion);
  if (!target) return { ok: false, reason: 'unknown_version' };
  return rollbackProgram(db, { id, toVersion, ownerConfirmed: true, receipt: target.receipt });
}

function activeBindings(db, id) {
  const active = getActiveProgram(db, id);
  return active?.receipt?.schema === 'hq.comment-wiki-receipt/0.1' && Array.isArray(active.receipt.bindings)
    ? active.receipt.bindings : null;
}
function intent(binding, action, provenance, context) {
  return { schema: COMMAND_INTENT_SCHEMA, outcome: 'candidate', actionId: binding.actionId,
    args: canonical(binding.args ?? {}), risk: action.risk,
    requiredScopes: [...binding.requiredScopes],
    worldRevision: context.worldRevision ?? null, catalogRevision: context.catalogRevision ?? null,
    audience: context.audience ?? null, scope: context.scope ?? null,
    provenance };
}
const denied = (reason) => ({ schema: COMMAND_INTENT_SCHEMA, outcome: 'refused', refusal: { reason } });

/** A click is never a dispatch: it returns typed data for a separate Host gate. */
export function resolveWikiGesture(db, { programId, bindingId, event } = {}, options = {}) {
  const bindings = activeBindings(db, programId);
  if (!bindings) return denied('program_not_active');
  const binding = bindings.find((b) => b.id === bindingId && b.event === event);
  if (!binding) return denied('unknown_gesture');
  const action = lookupAction(binding, { ...options, db });
  if (!action) return denied('action_not_registered');
  return intent(binding, action, { surface: 'wiki', programId, bindingId }, options);
}

/** NL and clicks use the SAME approved binding table; no free-form execution. */
export function resolveWikiNaturalLanguage(db, { programId, text } = {}, options = {}) {
  const bindings = activeBindings(db, programId);
  if (!bindings) return denied('program_not_active');
  if (!string(text) || text.length > 2048) return denied('invalid_text');
  if (/\b(?:not|never|don't|dont|stop|avoid|no)\b/i.test(text)) return denied('negated_command');
  if (/\b(?:and|or|then|after|before)\b|[;&|]/i.test(text))
    return { schema: COMMAND_INTENT_SCHEMA, outcome: 'interpretation_required', refusal: { reason: 'compound_command' } };
  const registered = bindings.map((b) => ({ binding: b, action: lookupAction(b, { ...options, db }) }))
    .filter(({ action }) => Boolean(action));
  const unique = new Map(registered.map(({ action }) => [action.id, action]));
  const routed = route([...unique.values()], text, { limit: unique.size || 1 });
  if (!routed.ok) return denied('no_action_match');
  const top = routed.matches[0];
  if (top.score < 5 || (routed.matches[1]?.score ?? -1) === top.score)
    return { schema: COMMAND_INTENT_SCHEMA, outcome: 'clarification_required', alternatives: routed.matches.map((x) => x.action.id) };
  const candidates = registered.filter(({ action }) => action.id === top.action.id);
  if (candidates.length !== 1) return { schema: COMMAND_INTENT_SCHEMA, outcome: 'clarification_required',
    alternatives: candidates.map((x) => x.binding.id) };
  return intent(candidates[0].binding, candidates[0].action, { surface: 'natural_language', programId }, options);
}
