/**
 * @hq:context Additive v2 annotation grammar: typed semantic tags parsed from source comments only.
 * @hq:owner hq
 * @hq:task semantic-kernel-annotation-v2
 * @hq:invariant Pure: no database, filesystem, network, clock or randomness. Never throws on bad input.
 * @hq:invariant A comment never creates a registered executable action; unknown refs are reported, never accepted.
 * @hq:invariant v1 tags (`context`, `why`, `invariant`, `see`, `todo`, `owner`, `task`, `risk`, `deprecated`)
 *   stay owned by hq/lib/protocol.mjs. This module ignores them, so a v1-only file yields zero output here.
 * @hq:see hq/lib/protocol.mjs
 * @hq:see docs/research/semantic-language-kernel/ARCHITECTURE.md
 *
 * Design notes
 * - protocol.mjs keeps its comment scanner private and returns one `{line, text}` per
 *   block comment, which cannot give the exact line of each tag inside a multi-line
 *   block. This module therefore ships its own character scanner (quotes, template
 *   literals and regex literals are skipped, so tags inside strings never count).
 * - The registry is injected. This module imports nothing and knows no other module.
 * - Malformed input is a no-op: a non-string `source` returns an empty result with
 *   no diagnostic, because "there was no text" is not a finding about the text.
 * - Generated and vendor file exclusion happens in the walker, before this parser:
 *   the parser is pure and sees only text a caller decided to scan.
 */

export const ANNOTATION_V2_SCHEMA = 'hq.annotation/2';
export const SOURCE_LINK_SCHEMA = 'hq.source-link/1';

export const ANNOTATION_V2_TAGS = [
  'type', 'capability', 'effect', 'evidence', 'contract', 'emits', 'consumes', 'action', 'test',
];

const TAG_SET = new Set(ANNOTATION_V2_TAGS);

// versioned-id tags resolve through the injected registry; effect/evidence/test do not.
const VERSIONED_TAGS = new Set(['type', 'capability', 'contract', 'emits', 'consumes', 'action']);

const compare = (a, b) => (a < b ? -1 : a > b ? 1 : 0);
const textOrNull = (value) => {
  if (typeof value !== 'string') return null;
  const trimmed = value.trim();
  return trimmed ? trimmed : null;
};

// ---------------------------------------------------------------------------
// Value grammar
// ---------------------------------------------------------------------------

const VERSIONED_ID_RE = /^[a-z0-9][a-z0-9._-]*@\d+(?:\.\d+)*$/;
const EFFECT_CLASS_RE = /^[a-z0-9-]+:[a-z0-9-]+$/;

/** Validate `name@version`: lowercase name, numeric dotted version. */
export function isVersionedId(value) {
  return typeof value === 'string' && VERSIONED_ID_RE.test(value);
}

/** Validate an effect class such as `host:config-write`. */
export function isEffectClass(value) {
  return typeof value === 'string' && EFFECT_CLASS_RE.test(value);
}

/** Validate an evidence ref: a `test:` prefix plus a non-empty remainder. */
export function isEvidenceRef(value) {
  if (typeof value !== 'string') return false;
  if (!value.startsWith('test:')) return false;
  return value.slice('test:'.length).trim().length > 0;
}

const GRAMMAR = {
  type: isVersionedId,
  capability: isVersionedId,
  contract: isVersionedId,
  emits: isVersionedId,
  consumes: isVersionedId,
  action: isVersionedId,
  effect: isEffectClass,
  evidence: isEvidenceRef,
  test: isEvidenceRef,
};

// ---------------------------------------------------------------------------
// Secret paths
// ---------------------------------------------------------------------------

// Mirrors the HQ-wide secret rule used by realtime, canon, filetypes, kb-intake and
// the semantic optimizers: per segment, case-insensitive, after `\` -> `/`.
const SECRET_BASENAMES = new Set([
  'session.json', 'storage-state.json', 'release-profile.json', 'cookies', 'login data',
]);

function isSecretSegment(segment) {
  const name = String(segment ?? '').trim().toLowerCase();
  if (!name) return false;
  if (name.startsWith('.env')) return true;
  if (name.endsWith('.env')) return true;
  if (name.endsWith('.keystore')) return true;
  if (name.endsWith('.pem') || name.endsWith('.p12')) return true;
  return SECRET_BASENAMES.has(name);
}

function hasSecretPath(value) {
  return String(value ?? '').replace(/\\/g, '/').split('/').some(isSecretSegment);
}

// ---------------------------------------------------------------------------
// Comment scanner
// ---------------------------------------------------------------------------

// A `/` starts a regex literal (not a division) after these characters.
const REGEX_PREFIX = new Set([
  '(', ',', '=', ':', '[', '!', '&', '|', '?', '{', ';', '+', '-', '*', '%', '<', '>', '~', '^', '/',
]);
const REGEX_KEYWORD = /(?:^|[^\w$])(?:return|typeof|instanceof|case|in|of|new|delete|void|do|else|yield|await|throw)$/;

function lineStarts(text) {
  const starts = [0];
  for (let i = 0; i < text.length; i++) if (text.charCodeAt(i) === 10) starts.push(i + 1);
  return starts;
}

function lineAt(starts, index) {
  let lo = 0;
  let hi = starts.length - 1;
  while (lo < hi) {
    const mid = (lo + hi + 1) >> 1;
    if (starts[mid] <= index) lo = mid;
    else hi = mid - 1;
  }
  return lo + 1;
}

// Skip a quoted string; returns the index just past the closing quote. Template
// literals are skipped whole, interpolations included: conservative, so a tag
// inside one can never be mistaken for a comment.
function skipQuoted(text, start, quote) {
  const n = text.length;
  let i = start + 1;
  while (i < n) {
    const ch = text[i];
    if (ch === '\\') { i += 2; continue; }
    if (ch === quote) return i + 1;
    if (ch === '\n' && quote !== '`') return i + 1; // unterminated literal: stop at end of line
    i++;
  }
  return n;
}

// Skip a regex literal; returns `start` when this is not a regex after all.
function skipRegex(text, start) {
  const n = text.length;
  let i = start + 1;
  let inClass = false;
  while (i < n) {
    const ch = text[i];
    if (ch === '\\') { i += 2; continue; }
    if (ch === '\n') return start;
    if (ch === '[') inClass = true;
    else if (ch === ']') inClass = false;
    else if (ch === '/' && !inClass) return i + 1;
    i++;
  }
  return start;
}

/**
 * Scan `//` line comments and block comments, returning their *content* spans as
 * absolute `{start, end}` offsets (delimiters excluded, so `//` inside a string or
 * a regex literal never opens a comment). Line numbers are derived from offsets.
 */
function scanCommentSpans(text) {
  const spans = [];
  const n = text.length;
  let i = 0;
  let previous = ''; // last significant character outside a comment or literal
  let word = '';     // last identifier/number token outside a comment or literal

  while (i < n) {
    const ch = text[i];
    const next = text[i + 1];

    if (ch === '/' && next === '/') {
      let end = text.indexOf('\n', i + 2);
      if (end === -1) end = n;
      spans.push({ start: i + 2, end });
      i = end;
      previous = ';'; // a line comment ends a statement
      word = '';
      continue;
    }

    if (ch === '/' && next === '*') {
      const close = text.indexOf('*/', i + 2);
      const end = close === -1 ? n : close;
      spans.push({ start: i + 2, end });
      i = close === -1 ? n : close + 2;
      continue;
    }

    if (ch === '"' || ch === "'" || ch === '`') {
      i = skipQuoted(text, i, ch);
      previous = ch;
      word = '';
      continue;
    }

    if (ch === '/' && (previous === '' || REGEX_PREFIX.has(previous) || REGEX_KEYWORD.test(word))) {
      const skipped = skipRegex(text, i);
      if (skipped > i) {
        i = skipped;
        previous = '/';
        word = '';
        continue;
      }
    }

    if (ch === ' ' || ch === '\t' || ch === '\r' || ch === '\n') { i++; continue; }

    if (/[\w$]/.test(ch)) {
      let j = i;
      while (j < n && /[\w$]/.test(text[j])) j++;
      word = text.slice(i, j);
      previous = text[j - 1];
      i = j;
      continue;
    }

    previous = ch;
    word = '';
    i++;
  }

  return spans;
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

function emptyResult() {
  return {
    schema: ANNOTATION_V2_SCHEMA,
    sourceLinkSchema: SOURCE_LINK_SCHEMA,
    records: [],
    diagnostics: [],
  };
}

function diagnostic(code, severity, kind, value, line, message) {
  return { code, severity, kind, value, line, message };
}

function lookupRef(registry, id) {
  if (!registry || typeof registry.lookup !== 'function') return null;
  try {
    const entry = registry.lookup(id);
    return entry && typeof entry === 'object' ? entry : null;
  } catch {
    return null; // a broken registry is an unknown ref, never an exception
  }
}

/**
 * Parse the additive v2 annotation grammar out of source comments.
 *
 * @param {string} source file text
 * @param {{path?: string, contentHash?: string, projectId?: string, symbolId?: string,
 *   registry?: {lookup(id: string): {id?: string, status?: string, kind?: string}|null},
 *   schemaVersion?: string}} [options]
 * @returns {{schema: string, sourceLinkSchema: string,
 *   records: Array<{kind: string, value: string, projectId: string|null, path: string|null,
 *     contentHash: string|null, symbolId?: string, line: number, schemaVersion: string,
 *     provenance: {kind: string, path: string|null, line: number}}>,
 *   diagnostics: Array<{code: string, severity: string, kind: string, value: string,
 *     line: number, message: string}>}}
 */
export function parseAnnotationV2(source, options = {}) {
  const result = emptyResult();
  if (typeof source !== 'string' || !source) return result;

  const opts = options && typeof options === 'object' ? options : {};
  const ctx = {
    projectId: textOrNull(opts.projectId),
    path: textOrNull(opts.path),
    contentHash: textOrNull(opts.contentHash),
    symbolId: textOrNull(opts.symbolId),
    schemaVersion: textOrNull(opts.schemaVersion) || '2',
    registry: opts.registry ?? null,
  };

  const starts = lineStarts(source);
  const seen = new Set();
  const records = [];
  const diagnostics = [];

  for (const span of scanCommentSpans(source)) {
    const body = source.slice(span.start, span.end);
    const re = /@hq:([a-z][a-z0-9-]*)/g;
    let match;
    while ((match = re.exec(body)) !== null) {
      const kind = match[1];
      if (!TAG_SET.has(kind)) continue; // v1 tag or unknown tag: not this parser's business

      // The value runs to the end of the line, stops at the next @hq: tag, and never
      // reaches past the comment body, so a block-comment terminator is excluded.
      const valueStart = match.index + match[0].length;
      let valueEnd = body.indexOf('\n', valueStart);
      if (valueEnd === -1) valueEnd = body.length;
      const nextTag = body.indexOf('@hq:', valueStart);
      if (nextTag !== -1 && nextTag < valueEnd) valueEnd = nextTag;
      const value = body.slice(valueStart, valueEnd).trim();
      const line = lineAt(starts, span.start + match.index);

      const key = `${line}|${kind}|${value}`;
      if (seen.has(key)) {
        diagnostics.push(diagnostic('duplicate_tag', 'warning', kind, value, line,
          `duplicate @hq:${kind} "${value}" on the same line; one record kept`));
        continue;
      }
      seen.add(key);

      if (hasSecretPath(value)) {
        diagnostics.push(diagnostic('secret_path', 'error', kind, value, line,
          'annotation value names a secret-looking path segment; refused and never emitted'));
        continue;
      }

      if (!GRAMMAR[kind](value)) {
        diagnostics.push(diagnostic('invalid_value', 'error', kind, value, line,
          `${kind} value "${value}" does not match the ${kind} grammar`));
        continue;
      }

      if (VERSIONED_TAGS.has(kind)) {
        const entry = lookupRef(ctx.registry, value);
        if (!entry) {
          diagnostics.push(diagnostic('unknown_ref', 'error', kind, value, line,
            `${kind} "${value}" is not registered; unknown refs are reported, never accepted`));
        } else if (entry.status === 'deprecated') {
          diagnostics.push(diagnostic('deprecated_ref', 'warning', kind, value, line,
            `${kind} "${value}" is deprecated; point at a current ref`));
        }
      }

      const record = {
        kind,
        value,
        projectId: ctx.projectId,
        path: ctx.path,
        contentHash: ctx.contentHash,
      };
      if (ctx.symbolId) record.symbolId = ctx.symbolId;
      record.line = line;
      record.schemaVersion = ctx.schemaVersion;
      record.provenance = { kind: 'source-comment', path: ctx.path, line };
      records.push(record);
    }
  }

  records.sort((a, b) => a.line - b.line || compare(a.kind, b.kind) || compare(a.value, b.value));
  diagnostics.sort((a, b) => a.line - b.line || compare(a.code, b.code) || compare(a.value, b.value));

  result.records = records;
  result.diagnostics = diagnostics;
  return result;
}
