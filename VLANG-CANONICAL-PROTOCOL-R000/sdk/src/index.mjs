// VLANG SDK R0: read-only, adapter-driven facade over existing ARBOR HQ.
// No compiler, semantic registry, persistence, execution, or approval authority.
import { createHash } from 'node:crypto';

export const SDK_SCHEMA = 'vivim.vlang-sdk/0.1-research';
export const HQ_ANNOTATION_SCHEMA = 'hq.annotation/2';
export const MAX_SOURCE_BYTES = 1_500_000;
const SHA256 = /^[a-f0-9]{64}$/;
const SUPPORTED = /\.(?:mjs|cjs|js|mts|cts|ts|rs)$/i;
const UNSAFE_SEGMENT = /^(?:\.env.*|.*\.env|.*\.pem|.*\.p12|.*\.keystore|session\.json|storage-state\.json|release-profile\.json|cookies|login data)$/i;

export class VlangSdkError extends Error {
  constructor(code, detail) {
    super(code + ': ' + detail);
    this.name = 'VlangSdkError';
    this.code = code;
  }
}

function refuse(code, detail) { throw new VlangSdkError(code, detail); }
export function sourceSha256(source) {
  if (typeof source !== 'string') refuse('INVALID_SOURCE', 'Source must be UTF-8 text');
  return createHash('sha256').update(source, 'utf8').digest('hex');
}
export function validateSourcePin(pin) {
  if (!pin || typeof pin !== 'object' || Array.isArray(pin)) refuse('PIN_REQUIRED', 'Explicit source identity is required');
  const { projectId, path, sourceSha256: digest } = pin;
  if (typeof projectId !== 'string' || !/^[a-zA-Z0-9][\w.-]{0,127}$/.test(projectId))
    refuse('INVALID_PROJECT', 'Bounded project identifier required');
  if (typeof path !== 'string' || !path || path.startsWith('/') || path.includes('\\') ||
      path.split('/').some(s => !s || s === '.' || s === '..' || UNSAFE_SEGMENT.test(s)))
    refuse('UNSAFE_PATH', 'Use a relative, non-sensitive source path');
  if (typeof digest !== 'string' || !SHA256.test(digest))
    refuse('INVALID_HASH', 'A full lowercase SHA-256 of source bytes is required');
  return Object.freeze({ projectId, path, sourceSha256: digest });
}
function onlyHeaderComments(source) {
  const header = [];
  for (const line of source.split('\n')) {
    if (/^\s*(?:\/\/.*)?$/.test(line)) header.push(line);
    else break;
  }
  return header.join('\n');
}
function ensurePorts(ports) {
  if (!ports || typeof ports.parseAnnotations !== 'function' ||
      typeof ports.composeContext !== 'function' || typeof ports.verifyContext !== 'function')
    refuse('MISSING_PORT', 'HQ parser, context composer and context verifier adapters required');
}
export function createVlangSdk(ports) {
  ensurePorts(ports);
  return Object.freeze({
    schema: SDK_SCHEMA,
    async inspectSource({ pin, source, registry = null }) {
      const bound = validateSourcePin(pin);
      if (typeof source !== 'string' || Buffer.byteLength(source, 'utf8') > MAX_SOURCE_BYTES)
        refuse('SOURCE_LIMIT', 'Source must be text within the HQ indexer limit');
      if (sourceSha256(source) !== bound.sourceSha256)
        refuse('STALE_SOURCE', 'Supplied source does not match its SHA-256 pin');
      if (!SUPPORTED.test(bound.path)) refuse('UNSUPPORTED_LANGUAGE', 'Only JS/TS and Rust file headers in R0; JSX/TSX excluded');
      const scope = bound.path.endsWith('.rs') ? 'rust-file-header-only' : 'js-ts-comment-only';
      const scanned = scope === 'rust-file-header-only' ? onlyHeaderComments(source) : source;
      const response = await ports.parseAnnotations(scanned, {
        projectId: bound.projectId, path: bound.path,
        contentHash: 'sha256:' + bound.sourceSha256, schemaVersion: '2', registry,
      });
      if (!response || response.schema !== HQ_ANNOTATION_SCHEMA ||
          !Array.isArray(response.records) || !Array.isArray(response.diagnostics))
        refuse('BAD_ADAPTER_RESULT', 'Expected the existing hq.annotation/2 response');
      if (response.records.length > 1000 || response.diagnostics.length > 1000)
        refuse('RESULT_LIMIT', 'HQ parser output exceeded SDK bound');
      for (const item of response.records) {
        if (!item || item.projectId !== bound.projectId || item.path !== bound.path ||
            item.contentHash !== 'sha256:' + bound.sourceSha256)
          refuse('UNBOUND_RECORD', 'A returned annotation lost its exact source binding');
      }
      return Object.freeze({
        schema: SDK_SCHEMA, pin: bound, scope,
        annotations: response.records.map(r => ({ ...r, assurance: 'declared-unverified', executionAuthorized: false })),
        diagnostics: response.diagnostics,
        sourceCurrentAtRead: 'not-checked-against-filesystem',
        executionAuthorized: false,
      });
    },
    async getVerifiedContext({ task, pin, maxTokens = 4096 }) {
      const bound = validateSourcePin(pin);
      if (typeof task !== 'string' || !task.trim() || task.length > 4000)
        refuse('INVALID_TASK', 'A bounded task is required');
      if (!Number.isSafeInteger(maxTokens) || maxTokens < 1 || maxTokens > 16384)
        refuse('INVALID_BUDGET', 'Token budget must be 1..16384');
      // The actual ARBOR adapter owns source reads, bundle generation and full
      // verifyContext semantics. This wrapper neither caches nor mutates the packet.
      const packet = await ports.composeContext({ task, pin: bound, maxTokens });
      if (packet == null || typeof packet !== 'object')
        refuse('BAD_CONTEXT', 'HQ context adapter returned no packet');
      const verdict = await ports.verifyContext({ packet, pin: bound });
      if (verdict?.ok !== true || verdict?.sourceSha256 !== bound.sourceSha256)
        refuse('UNVERIFIED_CONTEXT', 'Context requires independent current-source verification');
      return Object.freeze({
        schema: SDK_SCHEMA, pin: bound, packet,
        contextVerified: true, executionAuthorized: false,
      });
    },
    compareSourcePins(previous, current) {
      const a = validateSourcePin(previous), b = validateSourcePin(current);
      if (a.projectId !== b.projectId || a.path !== b.path)
        refuse('IDENTITY_MISMATCH', 'Path moves require explicit owner-reviewed lineage');
      return Object.freeze({
        schema: SDK_SCHEMA, previous: a, current: b,
        changed: a.sourceSha256 !== b.sourceSha256,
        dependentClaims: a.sourceSha256 === b.sourceSha256 ? 'unchanged-source' : 'requires-revalidation',
        executionAuthorized: false,
      });
    },
  });
}
