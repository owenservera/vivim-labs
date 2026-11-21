// hq/lib/context.mjs — task-scoped context bundles.
//
// composeContext turns one task description into a small, budgeted slice of the
// local index: which files were read, exactly which lines, the sha256 of that
// text, and why each file earned its place. A bundle carries no authority — it
// is a read-only projection that another tool (or a person) can verify later
// with verifyContext, without trusting the bundle's retained text.
//
// Determinism is a feature: candidates are sorted by ref, the knapsack is
// solved in fixed 32-token units, and generatedAt is excluded from the digest.
import { closeSync, existsSync, openSync, readSync } from 'node:fs';
import { isAbsolute, join } from 'node:path';
import { userTerm } from './nomenclature.mjs';
import { alignmentForTask } from './vivim.mjs';
import { isVersionedId } from './annotation-v2.mjs';
import { findFiles, searchContent, searchSymbols, searchSemanticReferences, isSensitiveSourcePath,
  contentTokens, isNoisyPath, pathTokenMatch } from './search.mjs';
import {
  digestOf, estimateTokens, sha256, toPosix, tokenize,
} from './util.mjs';

const BUNDLE_SCHEMA = 'hq.context-bundle/1';
const ESTIMATOR = 'utf8-bytes/3';

// Excerpt byte cap; ordinary symbol/doc candidates also retain this read cap.
const MAX_FILE_BYTES = 24 * 1024;
// Verification needs at least the bytes composeContext was allowed to read.
const VERIFY_BYTES = 64 * 1024;
// Semantic candidates verify the whole indexed file, but excerpts retain MAX_FILE_BYTES.
const SEMANTIC_VERIFY_BYTES = 1_500_001;
const DOC_LINE_LIMIT = 120;
const COVERAGE_CHARS = 2500;
const TOKEN_UNIT = 32;
const OVERHEAD_TOKENS = 80;
const RECENCY_DAYS = 90;
const NO_DATE_RECENCY = 0.25;
const MAX_SYMBOLS = 60;

const CONTENT_HITS = 40;
const SYMBOL_HITS = 40;
const PATH_HITS = 20;
const DOC_HITS = 10;
const RECENT_HITS = 10;
const TOKEN_QUERIES = 12;
const SYMBOL_PER_TOKEN = 6;

// ZAIQ-013 ranking weights. The scorer rewards precision signals (a symbol
// literally named like the task, a path that names the concept) instead of a
// flat "any symbol hit" bonus, measures coverage over content words rather
// than every function word, and demotes test/fixture locations so
// implementation files answer first.
const SYMBOL_BONUS_EXACT = 1.2;
const SYMBOL_BONUS_PREFIX = 0.7;
const SYMBOL_BONUS_SUBSTRING = 0.3;
const PATH_BONUS_EXACT = 0.9;
const PATH_BONUS_PREFIX = 0.45;
const TEST_SCORE_DEMOTE = 0.6;
const PER_TOKEN_CONTENT_HITS = 8;

const REASON_ORDER = ['semantic', 'symbol', 'fts', 'path', 'doc', 'recent'];

// Secret material never becomes a candidate, not even as an omission ref.


const LABELS = Object.freeze({
  aperture: userTerm('aperture'),
  evidence: userTerm('evidence'),
});

function cmp(a, b) {
  const left = String(a ?? '');
  const right = String(b ?? '');
  return left < right ? -1 : left > right ? 1 : 0;
}

function round6(value) {
  return Math.round(value * 1e6) / 1e6;
}

function toCount(value, fallback, min = 0, max = Number.MAX_SAFE_INTEGER) {
  const n = Number(value);
  if (!Number.isFinite(n)) return fallback;
  return Math.min(max, Math.max(min, Math.trunc(n)));
}

function toIso(value) {
  if (value == null) return new Date().toISOString();
  if (value instanceof Date) return value.toISOString();
  if (typeof value === 'number') {
    const ms = Number(value);
    return Number.isFinite(ms) ? new Date(ms).toISOString() : new Date().toISOString();
  }
  const text = String(value);
  const ms = Date.parse(text);
  return Number.isFinite(ms) ? new Date(ms).toISOString() : text;
}

function isSecretPath(value) {
  return !String(value ?? '').trim() || isSensitiveSourcePath(value);
}

// readPrefix — read at most maxBytes from the start of a file. Returns null on
// any read failure so a single bad file never aborts a bundle.
function readPrefix(absPath, maxBytes) {
  let fd = null;
  try {
    fd = openSync(absPath, 'r');
    const buffer = Buffer.alloc(maxBytes);
    const read = readSync(fd, buffer, 0, maxBytes, 0);
    return buffer.subarray(0, read).toString('utf8');
  } catch {
    return null;
  } finally {
    if (fd != null) {
      try { closeSync(fd); } catch { /* already closed */ }
    }
  }
}

function splitLines(text) {
  if (!text) return [];
  const parts = text.split(/\r?\n/);
  if (parts.length && parts[parts.length - 1] === '' && text.endsWith('\n')) parts.pop();
  return parts;
}

// selectWindow — contiguous 1-based line window clipped to maxBytes. The byte
// budget is measured on the joined text exactly as it will be hashed.
function selectWindow(parts, { start = 1, maxLines = null, maxBytes = MAX_FILE_BYTES } = {}) {
  const total = parts.length;
  if (!total) return { text: '', lines: [1, 1] };
  const first = Math.min(Math.max(1, Math.trunc(start) || 1), total);
  const last = maxLines == null ? total : Math.min(total, first + Math.max(0, Math.trunc(maxLines)) - 1);
  const chosen = [];
  let bytes = 0;
  for (let line = first; line <= last; line++) {
    const value = parts[line - 1];
    const extra = Buffer.byteLength(value, 'utf8') + (chosen.length ? 1 : 0);
    if (chosen.length && bytes + extra > maxBytes) break;
    chosen.push(value);
    bytes += extra;
  }
  if (!chosen.length) {
    chosen.push(Buffer.from(parts[first - 1], 'utf8').subarray(0, maxBytes).toString('utf8'));
  }
  return { text: chosen.join('\n'), lines: [first, first + chosen.length - 1] };
}

function mergeRanges(ranges) {
  const sorted = ranges
    .map((range) => [Math.max(1, Math.trunc(range[0]) || 1), Math.max(1, Math.trunc(range[1]) || Math.trunc(range[0]) || 1)])
    .sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const merged = [];
  for (const [start, end] of sorted) {
    const tail = merged[merged.length - 1];
    if (tail && start <= tail[1] + 1) {
      tail[1] = Math.max(tail[1], end);
    } else {
      merged.push([start, end]);
    }
  }
  return merged;
}

function selectSymbolWindow(parts, ranges) {
  const merged = mergeRanges(ranges).slice(0, 3);
  if (!merged.length) return selectWindow(parts, { start: 1 });
  const start = merged[0][0];
  const end = merged[merged.length - 1][1];
  if (start > parts.length) return selectWindow(parts, { start: 1 });
  return selectWindow(parts, { start, maxLines: end - start + 1 });
}

function docAuthority(filePath) {
  const posix = toPosix(filePath).toLowerCase();
  const base = posix.split('/').pop() || '';
  if (base === 'readme.md' || base === 'agents.md' || base === 'readme' || base === 'agents') return 1;
  if (base.endsWith('.md') || base.endsWith('.mdx')) return 0.6;
  if (posix.startsWith('docs/') || posix.includes('/docs/')) return 0.6;
  return 0;
}

function reasonLabel(reasons) {
  const parts = REASON_ORDER.filter((reason) => reasons.has(reason));
  return parts.length ? parts.join('+') : 'fts';
}

// sourceDigest — sha256 of the canonical source metadata plus the exact text.
// verifyContext recomputes this after re-reading the selected lines.
function sourceDigest(source) {
  return digestOf({
    ref: source.ref,
    project: source.project,
    path: source.path,
    kind: source.kind,
    lang: source.lang,
    lines: source.lines,
    sha256: source.sha256,
    estimatedTokens: source.estimatedTokens,
    score: source.score,
    reason: source.reason,
    ...(source.contentHash ? { contentHash: source.contentHash, semanticReferences: source.semanticReferences } : {}),
    text: source.text,
  });
}

// bundleDigest — canonical bundle without generatedAt, digest and the optional
// absPath resolution hints (those are machine-local, not evidence).
function bundleDigest(bundle) {
  const sources = (Array.isArray(bundle?.sources) ? bundle.sources : []).map((source) => ({
    ref: source.ref,
    project: source.project,
    path: source.path,
    kind: source.kind,
    lang: source.lang,
    sha256: source.sha256,
    lines: source.lines,
    estimatedTokens: source.estimatedTokens,
    score: source.score,
    reason: source.reason,
    ...(source.contentHash ? { contentHash: source.contentHash, semanticReferences: source.semanticReferences } : {}),
    digest: source.digest,
  }));
  return digestOf({
    schema: bundle?.schema ?? BUNDLE_SCHEMA,
    task: bundle?.task ?? '',
    ...(bundle?.sourceSemanticIds?.length ? { sourceSemanticIds: bundle.sourceSemanticIds } : {}),
    project: bundle?.project ?? null,
    ...(bundle?.semanticPacket ? {semanticPacket:bundle.semanticPacket}:{}),
    budget: bundle?.budget ?? null,
    sources,
    omissions: bundle?.omissions ?? [],
    symbols: bundle?.symbols ?? [],
    fragment: bundle?.fragment ?? '',
    authority: bundle?.authority ?? 'read_only_projection',
    labels: bundle?.labels ?? LABELS,
  });
}

function emptyBundle(base) {
  const bundle = {
    schema: BUNDLE_SCHEMA,
    task: base.task,
    ...(base.sourceSemanticIds?.length ? { sourceSemanticIds: base.sourceSemanticIds } : {}),
    project: base.project ?? null,
    generatedAt: base.generatedAt,
    budget: {
      maxTokens: base.budgetTokens,
      reserved: base.reservedForOutput,
      capacity: base.capacity,
      used: 0,
      estimator: ESTIMATOR,
    },
    sources: [],
    omissions: [],
    symbols: [],
    fragment: '',
    digest: '',
    authority: 'read_only_projection',
    labels: LABELS,
  };
  bundle.digest = bundleDigest(bundle);
  return bundle;
}

function resolveProject(db, project) {
  const requested = project == null
    ? ''
    : typeof project === 'object'
      ? String(project.name ?? project.path ?? '').trim()
      : String(project).trim();
  if (!requested) return { id: null, name: null, missing: false };
  const row = db.prepare(`
    SELECT id, name, path FROM projects
    WHERE lower(name) = lower(?) OR path = ?
    ORDER BY name ASC LIMIT 1`).get(requested, requested);
  if (!row) return { id: null, name: requested, missing: true };
  return { id: Number(row.id), name: row.name, missing: false };
}

function createCollector(db) {
  const lookup = db.prepare(`
    SELECT f.id AS id, f.path AS path, f.abs_path AS abs_path, f.lang AS lang,
           f.kind AS kind, f.size AS size, f.mtime_ms AS mtime_ms, f.lines AS lines,
           f.is_test AS is_test, p.name AS project
    FROM files f
    JOIN projects p ON p.id = f.project_id
    WHERE p.name = ? AND f.path = ?`);
  const map = new Map();

  function add(hit, reason, extra = {}) {
    if (!hit || !hit.project || !hit.path) return null;
    const filePath = toPosix(hit.path);
    if (isSecretPath(filePath)) return null;
    const ref = `${hit.project}:${filePath}`;
    let candidate = map.get(ref);
    if (!candidate) {
      let row = null;
      try { row = lookup.get(hit.project, filePath); } catch { row = null; }
      if (!row) return null;
      candidate = {
        ref,
        project: hit.project,
        path: filePath,
        absPath: row.abs_path,
        fileId: Number(row.id),
        kind: row.kind || 'source',
        lang: row.lang || 'other',
        size: Number(row.size ?? 0),
        mtimeMs: Number(row.mtime_ms ?? 0),
        isTest: Number(row.is_test ?? 0) === 1 || isNoisyPath(filePath),
        reasons: new Set(),
        symbolRanges: [],
        symbolHits: [],
        semanticReferences: [],
      };
      map.set(ref, candidate);
    }
    candidate.reasons.add(reason);
    if (extra.semantic) {
      const { snippet, ...reference } = extra.semantic;
      candidate.semanticReferences.push(reference);
    }
    if (extra.symbol) {
      const line = Number(extra.symbol.line) || 1;
      const endLine = Number(extra.symbol.end_line) || line;
      candidate.symbolRanges.push([line, Math.max(line, endLine)]);
      candidate.symbolHits.push({
        name: String(extra.symbol.name ?? ''),
        kind: String(extra.symbol.kind ?? 'symbol'),
        line,
      });
    }
    return candidate;
  }

  return { add, candidates: () => [...map.values()] };
}

function collectSymbolHits(db, task, projectName, add) {
  const seen = new Set();
  let count = 0;
  const push = (hit) => {
    if (!hit || count >= SYMBOL_HITS) return;
    const key = `${hit.project}\0${hit.path}\0${hit.name}\0${hit.line}`;
    if (seen.has(key)) return;
    seen.add(key);
    if (add(hit, 'symbol', { symbol: hit })) count++;
  };

  try {
    for (const hit of searchSymbols(db, { query: task, project: projectName, limit: SYMBOL_HITS }).results) push(hit);
  } catch { /* an unusual task string is not fatal */ }

  for (const token of tokenize(task).slice(0, TOKEN_QUERIES)) {
    if (count >= SYMBOL_HITS) break;
    try {
      for (const hit of searchSymbols(db, { query: token, project: projectName, limit: SYMBOL_PER_TOKEN }).results) push(hit);
    } catch { /* keep the other signals */ }
  }
}

function collectPathHits(db, tokens, projectName, add) {
  const touched = new Set();
  for (const token of tokens) {
    if (touched.size >= PATH_HITS) break;
    let results = [];
    try { results = findFiles(db, { pattern: token, project: projectName, limit: SYMBOL_PER_TOKEN }).results; } catch { results = []; }
    for (const hit of results) {
      if (touched.size >= PATH_HITS) break;
      const candidate = add(hit, 'path');
      if (candidate) touched.add(candidate.ref);
    }
  }
}

function collectDocHits(db, task, tokens, projectName, add) {
  const docs = new Map();
  const consider = (hit) => {
    if (!hit || !hit.project || !hit.path) return;
    const path = toPosix(hit.path);
    if (isSecretPath(path)) return;
    const ref = `${hit.project}:${path}`;
    if (!docs.has(ref)) docs.set(ref, { project: hit.project, path, ref });
  };

  try {
    for (const hit of searchContent(db, { query: task, project: projectName, kind: 'doc', limit: DOC_HITS }).results) consider(hit);
  } catch { /* docs are advisory, never fatal */ }

  for (const token of tokens.slice(0, 4)) {
    try {
      for (const hit of searchContent(db, { query: token, project: projectName, kind: 'doc', limit: 4 }).results) consider(hit);
    } catch { /* keep going */ }
  }

  // README / AGENTS are always worth a slot: they carry the project's own rules.
  for (const pattern of ['README', 'AGENTS.md', 'docs/']) {
    try {
      for (const hit of findFiles(db, { pattern, project: projectName, limit: 5 }).results) consider(hit);
    } catch { /* ignore */ }
  }

  const ordered = [...docs.values()].sort((a, b) => (
    docAuthority(b.path) - docAuthority(a.path) || cmp(a.ref, b.ref)
  )).slice(0, DOC_HITS);
  for (const doc of ordered) add(doc, 'doc');
}

function collectRecent(db, projectName, add) {
  let rows = [];
  try {
    const where = projectName ? 'WHERE lower(p.name) = lower(?)' : '';
    const params = projectName ? [projectName] : [];
    rows = db.prepare(`
      SELECT c.date AS date, cf.path AS path, p.name AS project
      FROM commits c
      JOIN commit_files cf ON cf.sha = c.sha
      JOIN projects p ON p.id = c.project_id
      ${where}
      ORDER BY c.date DESC, c.sha DESC, cf.path ASC`).all(...params);
  } catch {
    return;
  }
  const touched = new Set();
  for (const row of rows) {
    if (touched.size >= RECENT_HITS) break;
    if (isSecretPath(row.path)) continue;
    const candidate = add({ project: row.project, path: row.path }, 'recent');
    if (candidate) touched.add(candidate.ref);
  }
}

function collectCandidates(db, { task, projectName, sourceSemanticIds = [], strict = false }) {
  const collector = createCollector(db);
  const tokens = tokenize(task).slice(0, TOKEN_QUERIES);

  // Compiled contract identities are exact source lookups, not free-text synonyms
  // or grants. Prioritize them before identities in the free-text task.
  const semanticQuery = strict ? sourceSemanticIds.join(' ') : [...sourceSemanticIds, task].join(' ');
  for (const hit of searchSemanticReferences(db, { query: semanticQuery, project: projectName, limit: strict?100:CONTENT_HITS }).results) {
    collector.add(hit, 'semantic', { semantic: hit });
  }
  if(strict)return collector.candidates();

  try {
    for (const hit of searchContent(db, { query: task, project: projectName, limit: CONTENT_HITS }).results) collector.add(hit, 'fts');
  } catch { /* fall through to the other collectors */ }

  // ZAIQ-013: the full-task MATCH above is an implicit AND, so a file that
  // names only some of the task's concepts (the implementation file when the
  // question is prose) never enters through it. A per-token content pass —
  // mirroring the per-token symbol collector — lets the coverage scorer, not
  // the AND gate, decide what belongs in the bundle.
  const seenContent = new Set();
  let contentCount = 0;
  for (const token of contentTokens(task).slice(0, TOKEN_QUERIES)) {
    if (contentCount >= CONTENT_HITS) break;
    try {
      for (const hit of searchContent(db, { query: token, project: projectName, limit: PER_TOKEN_CONTENT_HITS }).results) {
        if (contentCount >= CONTENT_HITS) break;
        const candidate = collector.add(hit, 'fts');
        if (candidate && !seenContent.has(candidate.ref)) {
          seenContent.add(candidate.ref);
          contentCount++;
        }
      }
    } catch { /* per-token content is advisory */ }
  }

  collectSymbolHits(db, task, projectName, collector.add);
  collectPathHits(db, tokens, projectName, collector.add);
  collectDocHits(db, task, tokens, projectName, collector.add);
  collectRecent(db, projectName, collector.add);

  return collector.candidates();
}

function newestCommitDates(db, projectName) {
  const where = projectName ? 'WHERE lower(p.name) = lower(?)' : '';
  const params = projectName ? [projectName] : [];
  let rows = [];
  try {
    rows = db.prepare(`
      SELECT c.date AS date, cf.path AS path, p.name AS project
      FROM commits c
      JOIN commit_files cf ON cf.sha = c.sha
      JOIN projects p ON p.id = c.project_id
      ${where}`).all(...params);
  } catch {
    return new Map();
  }
  const dates = new Map();
  for (const row of rows) {
    if (isSecretPath(row.path)) continue;
    const ms = Date.parse(row.date);
    if (!Number.isFinite(ms)) continue;
    const key = `${row.project}\0${toPosix(row.path)}`;
    const previous = dates.get(key);
    if (previous == null || ms > previous) dates.set(key, ms);
  }
  return dates;
}

function importProximity(db, projectName, candidateIds) {
  const where = projectName ? 'WHERE lower(p.name) = lower(?)' : '';
  const params = projectName ? [projectName] : [];
  let rows = [];
  try {
    rows = db.prepare(`
      SELECT i.file_id AS a, i.resolved_file_id AS b
      FROM imports i
      JOIN projects p ON p.id = i.project_id
      ${where}`).all(...params);
  } catch {
    return new Set();
  }
  const close = new Set();
  for (const row of rows) {
    if (row.a == null || row.b == null) continue;
    const a = Number(row.a);
    const b = Number(row.b);
    if (candidateIds.has(a) && candidateIds.has(b)) {
      close.add(a);
      close.add(b);
    }
  }
  return close;
}

// symbolMatchBonus — graded instead of flat: a symbol literally named like a
// task word is worth the full bonus, a prefix match most of it, an incidental
// substring hit ("show" vs "how") almost nothing.
function symbolMatchBonus(symbolHits, gradeTokens) {
  let bonus = 0;
  for (const hit of symbolHits ?? []) {
    const name = String(hit?.name ?? '').toLowerCase();
    if (!name) continue;
    for (const token of gradeTokens) {
      if (token.length < 2) continue;
      if (name === token) bonus = Math.max(bonus, SYMBOL_BONUS_EXACT);
      else if (name.startsWith(token) || token.startsWith(name)) bonus = Math.max(bonus, SYMBOL_BONUS_PREFIX);
      else if (name.includes(token) || token.includes(name)) bonus = Math.max(bonus, SYMBOL_BONUS_SUBSTRING);
    }
  }
  return bonus;
}

function pathMatchBonus(path, gradeTokens) {
  const match = pathTokenMatch(gradeTokens, path);
  if (match === 'exact') return PATH_BONUS_EXACT;
  if (match === 'prefix') return PATH_BONUS_PREFIX;
  return 0;
}

function prepareCandidates(db, candidates, { task, projectName, nowMs, preferredSemanticAnchor, strict=false }) {
  const tokens = tokenize(task);
  // Grading and coverage run over content words: function words ("how does
  // the") appear in nearly every file and previously levelled the coverage
  // signal between real answers and filler.
  const words = contentTokens(task);
  const gradeTokens = words.length ? words : tokens;
  const ids = new Set(candidates.map((candidate) => candidate.fileId));
  const close = importProximity(db, projectName, ids);
  const commitDates = newestCommitDates(db, projectName);

  for (const candidate of candidates) {
    const semantic = strict?candidate.semanticReferences.filter(r=>r.semanticAnchor?.state==='indexed'&& !['invalid_kind','deprecated'].includes(r.resolution)):candidate.semanticReferences;
    if(strict&&!semantic.length){candidate.unreadable=true;continue;}
    const raw = readPrefix(candidate.absPath, semantic.length ? SEMANTIC_VERIFY_BYTES : MAX_FILE_BYTES);
    if (semantic.length && (raw == null || sha256(raw) !== semantic[0].contentHash)) {
      candidate.unreadable = true;
      continue;
    }
    if (raw == null) {
      candidate.unreadable = true;
      continue;
    }
    const parts = splitLines(raw);
    if (!parts.length) {
      candidate.unreadable = true;
      continue;
    }

    let window;
    if (semantic.length) {
      // A source-surface caller can request an observed annotation near the
      // selected implementation. Never accept an unindexed line as semantic evidence.
      const preferred = candidate.ref === preferredSemanticAnchor?.ref
        ? semantic.find(hit => hit.line === preferredSemanticAnchor.line &&
          hit.semanticId === preferredSemanticAnchor.semanticId) : null;
      const line = (preferred ?? semantic[0]).line;
      const first=strict?Math.min(...semantic.map(r=>r.line)):line;
      const last=strict?Math.max(...semantic.map(r=>r.line)):line;
      const start=Math.max(1,first-2);
      window = selectWindow(parts, { start, maxLines: strict?Math.max(16,last-start+1):16 });
      if(strict&&window.lines[1]<last){candidate.unreadable=true;continue;}
      if (line < window.lines[0] || line > window.lines[1] || Buffer.byteLength(window.text, 'utf8') > MAX_FILE_BYTES) {
        candidate.unreadable = true;
        continue;
      }
      candidate.contentHash = semantic[0].contentHash;
      candidate.semanticReferences = semantic.filter(hit => hit.line >= window.lines[0] && hit.line <= window.lines[1]);
      if(strict)candidate.semanticReferences=candidate.semanticReferences.map(({fileId,projectId,...reference})=>reference);
    }
    else if (candidate.symbolRanges.length) window = selectSymbolWindow(parts, candidate.symbolRanges);
    else if (candidate.kind === 'doc') window = selectWindow(parts, { start: 1, maxLines: DOC_LINE_LIMIT });
    else window = selectWindow(parts, { start: 1 });

    if (strict && /(?:-----BEGIN [A-Z ]*PRIVATE KEY-----|\b(?:api[_-]?key|access[_-]?token|client[_-]?secret|password)\s*[:=]\s*["'][^"'\r\n]{8,}["'])/i.test(window.text)) {
      candidate.unreadable=true;continue;
    }
    if (!window.text) {
      candidate.unreadable = true;
      continue;
    }

    const haystack = window.text.slice(0, COVERAGE_CHARS).toLowerCase();
    const matched = gradeTokens.filter((token) => haystack.includes(token)).length;
    const coverage = gradeTokens.length ? matched / gradeTokens.length : 0;
    const symbolBonus = symbolMatchBonus(candidate.symbolHits, gradeTokens);
    const pathBonus = pathMatchBonus(candidate.path, gradeTokens);
    const dateMs = commitDates.get(`${candidate.project}\0${candidate.path}`);
    const mtimeMs = candidate.mtimeMs;
    const ageMs = dateMs != null
      ? nowMs - dateMs
      : (Number.isFinite(mtimeMs) && mtimeMs > 0 ? nowMs - mtimeMs : null);
    const recency = ageMs == null
      ? NO_DATE_RECENCY
      : Math.exp(-Math.max(0, ageMs / 86_400_000) / RECENCY_DAYS);
    const authority = docAuthority(candidate.path);
    const proximity = close.has(candidate.fileId) ? 1 : 0;

    candidate.text = window.text;
    candidate.lines = window.lines;
    candidate.sha256 = sha256(window.text);
    // Charge the semantic evidence header against the same knapsack budget.
    candidate.estimatedTokens = candidate.contentHash
      ? estimateTokens('SOURCE ' + JSON.stringify({
        ref: candidate.ref, lines: candidate.lines, sha256: candidate.sha256,
        contentHash: candidate.contentHash, semanticReferences: candidate.semanticReferences,
      }) + '\n' + window.text + '\n\n')
      : estimateTokens(window.text);
    candidate.reason = reasonLabel(candidate.reasons);
    const rawScore = 1
      + 3.6 * coverage
      + symbolBonus
      + 0.8 * recency
      + 0.6 * authority
      + 0.5 * proximity
      + pathBonus
      + (semantic.length ? 12 : 0);
    candidate.score = strict?0:round6(candidate.isTest ? rawScore * TEST_SCORE_DEMOTE : rawScore);
    candidate.digest = sourceDigest(candidate);
  }

  return candidates;
}

// knapsack — exact 0/1 selection in 32-token units. Value is score in
// millionths (integer) so floating point can never reorder equal scores.
function knapsack(items, capacity, maxFiles) {
  const unitCap = Math.floor(Math.max(0, capacity) / TOKEN_UNIT);
  const count = items.length;
  const fileCap = Math.max(0, Math.trunc(maxFiles));
  if (!count || unitCap <= 0 || fileCap <= 0) return [];

  const width = fileCap + 1;
  const size = (unitCap + 1) * width;
  const layers = new Array(count + 1);
  const zero = new Int32Array(size).fill(-1);
  zero[0] = 0;
  layers[0] = zero;

  const weights = items.map((item) => Math.max(1, Math.ceil(item.estimatedTokens / TOKEN_UNIT)));
  const values = items.map((item) => Math.round(item.score * 1e6));

  for (let i = 1; i <= count; i++) {
    const previous = layers[i - 1];
    const next = previous.slice();
    const weight = weights[i - 1];
    const value = values[i - 1];
    for (let j = weight; j <= unitCap; j++) {
      for (let k = 1; k <= fileCap; k++) {
        const before = previous[(j - weight) * width + (k - 1)];
        if (before < 0) continue;
        const index = j * width + k;
        const total = before + value;
        if (total > next[index]) next[index] = total;
      }
    }
    layers[i] = next;
  }

  const top = layers[count];
  let bestValue = -1;
  let bestTokens = 0;
  let bestFiles = 0;
  for (let j = 0; j <= unitCap; j++) {
    for (let k = 0; k <= fileCap; k++) {
      const value = top[j * width + k];
      if (value > bestValue) {
        bestValue = value;
        bestTokens = j;
        bestFiles = k;
      }
    }
  }
  if (bestValue < 0) return [];

  let j = bestTokens;
  let k = bestFiles;
  const picked = new Array(count).fill(false);
  for (let i = count; i >= 1; i--) {
    const current = layers[i][j * width + k];
    const above = layers[i - 1][j * width + k];
    if (current !== above) {
      picked[i - 1] = true;
      j -= weights[i - 1];
      k -= 1;
    }
  }
  return items.filter((_, index) => picked[index]);
}

function buildSymbols(selected) {
  const symbols = new Map();
  for (const candidate of selected) {
    for (const hit of candidate.symbolHits) {
      if (!hit.name) continue;
      const key = `${candidate.project}\0${candidate.path}\0${hit.line}\0${hit.name}`;
      if (symbols.has(key)) continue;
      symbols.set(key, {
        name: hit.name,
        kind: hit.kind,
        path: candidate.path,
        project: candidate.project,
        line: hit.line,
      });
    }
  }
  return [...symbols.values()]
    .sort((a, b) => cmp(a.path, b.path) || a.line - b.line || cmp(a.name, b.name) || cmp(a.project, b.project))
    .slice(0, MAX_SYMBOLS);
}

/**
 * composeContext — build one task-scoped, budgeted, verifiable context bundle.
 *
 * @param {import('node:sqlite').DatabaseSync} db open HQ index
 * @param {{task: string, project?: string|null, budgetTokens?: number,
 *          reservedForOutput?: number, maxFiles?: number, now?: string|Date|number|null}} options
 * @returns {object} hq.context-bundle/1
 */
// Select a complete cover of exact requested IDs, not the highest scoring
// partial set. Bounded by the authorized source surface (16 files) and limit.
// Search budget and file budget failures are explicit, never partial safety facts.
export function selectMandatorySources(candidates,ids,{capacity,maxFiles}){
  const valid=candidates.filter(c=>!c.unreadable&&c.semanticReferences?.length)
    .sort((a,b)=>cmp(a.ref,b.ref));
  const wanted=new Set(ids);
  const covers=c=>new Set(c.semanticReferences.filter(r=>wanted.has(r.semanticId)&&
    r.semanticAnchor?.state==='indexed'&&!['invalid_kind','deprecated'].includes(r.resolution)).map(r=>r.semanticId));
  const covered=new Set(valid.flatMap(c=>[...covers(c)]));
  const missing=ids.filter(id=>!covered.has(id));
  if(missing.length)return {status:'incomplete',missing,selected:[]};
  // Strict source packets have at most 16 candidate files and 3 selected files.
  // Do not silently truncate a search that cannot prove a complete cover.
  if(valid.length>16||maxFiles>3)return {status:'incomplete',missing:[],selected:[]};
  let best=null;
  function visit(start,chosen,seen,cost){
    if(seen.size===wanted.size){
      const total=cost+estimateTokens(JSON.stringify(semanticPacketManifest(ids,chosen)));
      if(total<=capacity&&(!best||total<best.cost||(total===best.cost&&
          cmp(chosen.map(c=>c.ref).join('\u0000'),best.selected.map(c=>c.ref).join('\u0000'))<0)))
        best={cost:total,selected:[...chosen]};
      return;
    }
    if(chosen.length>=maxFiles)return;
    for(let i=start;i<valid.length;i++){
      const c=valid[i],next=new Set([...seen,...covers(c)]);
      if(next.size===seen.size)continue;
      visit(i+1,[...chosen,c],next,cost+c.estimatedTokens);
    }
  }
  visit(0,[],new Set(),0);
  return best?{status:'complete',missing:[],selected:best.selected}:
    {status:'budget_exceeded',missing:[],selected:[]};
}
export function semanticPacketManifest(ids,sources,status='complete',missing=[]){
  const dependencies=sources.flatMap(source=>(source.semanticReferences||[]).map(r=>({
    ref:source.ref,semanticId:r.semanticId,annotationKind:r.annotationKind,line:r.line,
    anchorId:r.semanticAnchor?.anchorId??null,fingerprint:r.semanticAnchor?.fingerprint??null,
    contentHash:source.contentHash,resolution:r.resolution
  }))).sort((a,b)=>cmp(JSON.stringify(a),JSON.stringify(b)));
  return {schema:'hq.semantic-context-packet/1',trustMode:'source-evidence-only',
    authority:'read_only_projection',planningEligible:false,
    status,requiredSemanticIds:[...ids],missingSemanticIds:[...missing],dependencies,
    mandatoryConstraints:{status:'incomplete',missing:['verified-action-contract','typed-argument-constraints','access-policy']},
    sourceTextTreatment:'untrusted-data',estimator:ESTIMATOR};
}

export function composeContext(db, options = {}) {
  const supplied = options.sourceSemanticIds ?? [];
  if (!Array.isArray(supplied) || supplied.length > 12 ||
      supplied.some(id => typeof id !== 'string' || id.length > 160 || !isVersionedId(id)))
    throw new TypeError('HQ_INVALID_SOURCE_SEMANTIC_IDS');
  const sourceSemanticIds = [...new Set(supplied)].sort();
  const strict=options.requireSemanticCoverage===true;
  const requestedTask=String(options.task ?? '').trim();
  // Exact identity selection needs no private user prose in the persisted/digested packet.
  const task=strict&&requestedTask?'Exact semantic source lookup':requestedTask;
  const project = resolveProject(db, options.project);
  const budgetTokens = toCount(options.budgetTokens, 6000, 0, 10_000_000);
  const reservedForOutput = toCount(options.reservedForOutput, 800, 0, budgetTokens);
  const maxFiles = toCount(options.maxFiles, 24, 0, 500);
  const generatedAt = toIso(options.now);
  const parsed = Date.parse(generatedAt);
  const nowMs = Number.isFinite(parsed) ? parsed : Date.now();
  const capacity = Math.max(0, budgetTokens - reservedForOutput - OVERHEAD_TOKENS);

  const base = {
    task, sourceSemanticIds,
    project: project.name ?? null,
    generatedAt,
    budgetTokens,
    reservedForOutput,
    capacity,
  };
  const finishEmpty=(status,missing=sourceSemanticIds)=>{
    const b=emptyBundle(base);
    if(strict){b.semanticPacket=semanticPacketManifest(sourceSemanticIds,[],status,missing);b.digest=bundleDigest(b);}
    return b;
  };
  if (!task || project.missing) return finishEmpty('incomplete');

  // An authorized caller may narrow this same compiler to a previously
  // verified set of indexed source references. A filter never grants read access.
  const allowedSourceRefs = options.allowedSourceRefs;
  if (allowedSourceRefs !== undefined && !(allowedSourceRefs instanceof Set))
    throw new TypeError('HQ_INVALID_SOURCE_SCOPE');
  if(strict&&(!sourceSemanticIds.length||allowedSourceRefs===undefined||allowedSourceRefs.size>16||maxFiles>3))throw new TypeError('HQ_STRICT_CONTEXT_REQUIRES_SCOPE');
  const candidates = collectCandidates(db, { task, projectName: project.name, sourceSemanticIds,strict })
    .filter(candidate => allowedSourceRefs === undefined || allowedSourceRefs.has(candidate.ref));
  prepareCandidates(db, candidates, { task, projectName: project.name, nowMs,
    preferredSemanticAnchor: options.preferredSemanticAnchor,strict });

  const readable = candidates.filter((candidate) => !candidate.unreadable);
  const mandatory=strict?selectMandatorySources(readable,sourceSemanticIds,{capacity,maxFiles}):null;
  if(mandatory&&mandatory.status!=='complete')return finishEmpty(mandatory.status,mandatory.missing);
  const selected = (mandatory?mandatory.selected:knapsack(readable, capacity, maxFiles))
    .sort((a, b) => b.score - a.score || cmp(a.ref, b.ref));

  const sources = [];
  const fragments = [];
  const chosen = new Set();
  let used = 0;
  for (const candidate of selected) {
    if (chosen.has(candidate.ref)) continue;
    if (used + candidate.estimatedTokens > capacity) continue;
    used += candidate.estimatedTokens;
    chosen.add(candidate.ref);
    sources.push({
      ref: candidate.ref,
      project: candidate.project,
      path: candidate.path,
      kind: candidate.kind,
      lang: candidate.lang,
      sha256: candidate.sha256,
      lines: candidate.lines,
      estimatedTokens: candidate.estimatedTokens,
      score: candidate.score,
      reason: candidate.reason,
      ...(candidate.contentHash ? { contentHash: candidate.contentHash, semanticReferences: candidate.semanticReferences } : {}),
      digest: candidate.digest,
      // Local resolution hint for verifyContext; excluded from every digest.
      absPath: candidate.absPath,
    });
    const header = JSON.stringify({
      ref: candidate.ref,
      lines: candidate.lines,
      sha256: candidate.sha256,
      ...(candidate.contentHash ? { contentHash: candidate.contentHash, semanticReferences: candidate.semanticReferences } : {}),
    });
    fragments.push(`SOURCE ${header}\n${candidate.text}`);
  }

  const omissions = (strict?[]:candidates)
    .filter((candidate) => !chosen.has(candidate.ref))
    .map((candidate) => ({
      ref: candidate.ref,
      reason: candidate.unreadable ? 'unreadable' : 'budget',
    }))
    .sort((a, b) => cmp(a.ref, b.ref));

  const symbols = buildSymbols(selected.filter((candidate) => chosen.has(candidate.ref)));

  const bundle = {
    schema: BUNDLE_SCHEMA,
    task,
    ...(sourceSemanticIds.length ? { sourceSemanticIds } : {}),
    project: project.name ?? null,
    generatedAt,
    budget: {
      maxTokens: budgetTokens,
      reserved: reservedForOutput,
      capacity,
      used,
      estimator: ESTIMATOR,
    },
    sources,
    omissions,
    symbols:strict?[]:symbols,
    fragment: fragments.join('\n\n'),
    digest: '',
    authority: 'read_only_projection',
    labels: LABELS,
    // VIVIM alignment is advisory: it is deliberately excluded from
    // bundleDigest so `context --verify` keeps proving source bytes only.
    vivim: vivimBlockForTask(db, task),
  };
  if(strict){
    bundle.semanticPacket=semanticPacketManifest(sourceSemanticIds,sources);
    // Charge actual serialized packet metadata in addition to the source
    // excerpts. Estimates cover the semantic payload, not HTTP framing.
    const mandatoryTokens=estimateTokens(JSON.stringify(bundle.semanticPacket));
    bundle.budget.used+=mandatoryTokens;
    if(bundle.budget.used>capacity)return finishEmpty('budget_exceeded',[]);
  }
  bundle.digest = bundleDigest(bundle);
  return bundle;
}

// VIVIM alignment block for a bundle: the phase and the capabilities/gaps whose
// titles match the task. Never throws, never blocks a bundle when unsynced.
function vivimBlockForTask(db, task) {
  try {
    const alignment = alignmentForTask(db, String(task ?? ''));
    if (!alignment || !alignment.phase) return null;
    return {
      phase: { id: alignment.phase.id, status: alignment.phase.status ?? null },
      capabilities: alignment.details?.capabilities ?? [],
      gaps: alignment.details?.gaps ?? [],
    };
  } catch {
    return null;
  }
}

function resolveSourcePath(bundle, source) {  const hint = source?.absPath ?? source?.abs_path;
  if (hint && isAbsolute(String(hint))) return String(hint);

  const rel = toPosix(source?.path ?? '');
  if (!rel) return null;
  const tries = [];
  const cwd = process.cwd();
  const root = bundle?.workspaceRoot ?? bundle?.root ?? null;
  if (root) {
    tries.push(join(String(root), rel));
    if (source?.project) tries.push(join(String(root), String(source.project), rel));
  }
  tries.push(join(cwd, rel));
  if (source?.project) tries.push(join(cwd, String(source.project), rel));
  for (const candidate of tries) {
    if (existsSync(candidate)) return candidate;
  }
  return null;
}

/**
 * verifyContext — recompute a bundle from disk without trusting retained text.
 * Every source is re-read, the selected lines are hashed again, and the source
 * digest plus the bundle digest are compared.
 *
 * @param {object} bundle a hq.context-bundle/1 object
 * @returns {{valid: boolean, expected: string|null, current: string, changed: string[]}}
 */
export function verifyContext(bundle) {
  const sources = Array.isArray(bundle?.sources) ? bundle.sources : [];
  const changed = [];
  const currentSources = [];
  const currentFragments=[];
  let currentPayloadTokens=0;

  for (const source of sources) {
    const absPath = resolveSourcePath(bundle, source);
    if (!absPath) {
      changed.push(String(source?.ref ?? ''));
      currentSources.push(source);
      continue;
    }
    const raw = readPrefix(absPath, source.contentHash ? SEMANTIC_VERIFY_BYTES : VERIFY_BYTES);
    if (raw == null) {
      changed.push(String(source?.ref ?? ''));
      currentSources.push(source);
      continue;
    }
    const parts = splitLines(raw);
    const range = Array.isArray(source.lines) ? source.lines : [1, 1];
    const start = Math.max(1, Math.trunc(range[0]) || 1);
    const end = Math.max(start, Math.trunc(range[1]) || start);
    const text = parts.slice(start - 1, Math.min(parts.length, end)).join('\n');
    const currentSha = sha256(text);
    if(bundle?.semanticPacket){
      const header=JSON.stringify({ref:source.ref,lines:source.lines,sha256:currentSha,
        contentHash:sha256(raw),semanticReferences:source.semanticReferences});
      const fragment='SOURCE '+header+'\n'+text;
      currentFragments.push(fragment);
      const cost=estimateTokens(fragment+'\n\n');
      currentPayloadTokens+=cost;
      if(cost!==source.estimatedTokens)changed.push(String(source.ref));
    }
    const currentDigest = sourceDigest({
      ref: source.ref,
      project: source.project,
      path: source.path,
      kind: source.kind,
      lang: source.lang,
      lines: source.lines,
      sha256: currentSha,
      estimatedTokens: source.estimatedTokens,
      score: source.score,
      reason: source.reason,
      ...(source.contentHash ? { contentHash: sha256(raw), semanticReferences: source.semanticReferences } : {}),
      text,
    });
    if (currentSha !== source.sha256 || currentDigest !== source.digest) {
      changed.push(String(source.ref ?? ''));
    }
    currentSources.push({ ...source, sha256: currentSha, digest: currentDigest,
      ...(source.contentHash ? { contentHash: sha256(raw) } : {}) });
  }

  const current = bundleDigest({ ...bundle, sources: currentSources });
  const expected = typeof bundle?.digest === 'string' && bundle.digest ? bundle.digest : null;
  const packet=bundle?.semanticPacket;
  const packetValid=!packet||(packet.status==='complete'&&packet.planningEligible===false&&
    digestOf(packet)===digestOf(semanticPacketManifest(bundle.sourceSemanticIds||[],sources))&&
    (bundle.sourceSemanticIds||[]).every(id=>packet.dependencies.some(d=>d.semanticId===id&&d.anchorId&&d.fingerprint&&!['invalid_kind','deprecated'].includes(d.resolution)))&&
    bundle.fragment===currentFragments.join('\n\n')&&
    bundle.budget.used===currentPayloadTokens+estimateTokens(JSON.stringify(packet))&&
    bundle.budget.used<=bundle.budget.capacity);
  const valid = packetValid && changed.length === 0 && expected !== null && expected === current;
  return { valid, expected, current, changed };
}
