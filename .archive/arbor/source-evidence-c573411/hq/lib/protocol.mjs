// hq/lib/protocol.mjs — the agent inline-comment protocol and its linter.
// CONTRACT (hq/CONTRACT.md): deterministic, dependency-free, local only.
// The protocol is the contract between a coding agent and the repo's own
// history: structured comments that say why, what must hold, and where to look.
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { basename, dirname, join, resolve } from 'node:path';
import { VIVIM_TAXONOMY } from './nomenclature.mjs';
import { toPosix } from './util.mjs';

export const PROTOCOL_VERSION = 1;

export const ANNOTATION_KINDS = [
  'context', 'why', 'invariant', 'see', 'todo', 'owner', 'task', 'risk', 'deprecated',
];

// ---------------------------------------------------------------------------
// Protocol text
// ---------------------------------------------------------------------------

/**
 * Render the compact Markdown protocol handed to coding agents.
 * Deterministic: no clock, no absolute paths.
 *
 * @param {{workspace?: string}} [options]
 * @returns {string}
 */
export function renderAgentProtocol({ workspace } = {}) {
  const name = String(workspace ?? '').trim() || 'this workspace';
  const invariantLines = VIVIM_TAXONOMY.invariants.map((rule) => `- ${rule}`);
  return [
    `# Agent protocol (v${PROTOCOL_VERSION})`,
    '',
    `Workspace: ${name}`,
    '',
    'HQ reads these inline annotations to keep code context honest. They live in',
    'comments, next to the code they explain. Only explicit `@hq:` tags count;',
    'plain `TODO` comments are ignored by the linter.',
    '',
    '## File header',
    '',
    'Every source file longer than 20 lines starts with a header comment:',
    '',
    '```',
    '// @hq:context <what this file owns, one sentence>',
    '// @hq:owner <person or team>',
    '// @hq:task <task id or slug, only while the work is active>',
    '```',
    '',
    '## Annotation tags',
    '',
    '| Tag | Use it when |',
    '| --- | --- |',
    '| `@hq:context` | First lines of a file: what this file is for. |',
    '| `@hq:why` | A non-obvious decision, workaround or trade-off. |',
    '| `@hq:invariant` | A rule that must stay true: ordering, units, auth, bounds. |',
    '| `@hq:see` | A pointer to another file or line, e.g. `src/auth.mjs:42`. |',
    '| `@hq:todo` | Work deliberately left: `@hq:todo(owner): what remains`. |',
    '| `@hq:owner` | The person or team accountable for the code below. |',
    '| `@hq:task` | The task or ticket this code serves. |',
    '| `@hq:risk` | A known hazard: data loss, security, performance, migration. |',
    '| `@hq:deprecated` | Code being retired; always pair it with `@hq:see`. |',
    '',
    '## Comment quality',
    '',
    '- Say why, not what. The code already says what.',
    '- One idea per comment; keep lines under 120 characters.',
    '- No dates, no agent names, no "temporary" notes that outlive the change.',
    '- Update the annotation in the same change as the code it describes.',
    '- Give every exported symbol a doc comment on the line above:',
    '  `/** ... */`, `///`, `#`, `"""` or `--`.',
    '- Long units get a `@hq:why`; remove the comment when the reason is gone.',
    '- Point at real files with `@hq:see`; orphan refs are lint warnings.',
    '',
    '## Invariants',
    '',
    'These hold for every change, in every project:',
    '',
    ...invariantLines,
    '',
    'Annotations explain code; they never grant permission. Owner-facing surfaces',
    'use the VIVIM terms from the glossary (`glossaryMarkdown()` in',
    'hq/lib/nomenclature.mjs), while engineering surfaces keep the precise',
    'internal names.',
    '',
    '## Secrets',
    '',
    'Never put passwords, API keys, tokens, bearer values or connection strings in',
    'comments. `hq lint` reports them as `secret-in-comment` errors, and the',
    'workspace must stay clean.',
    '',
    '## Post-edit commands',
    '',
    'Run these after every edit:',
    '',
    '```sh',
    'hq index --changed   # re-index only the files that changed',
    'hq check             # verify the index and generated output',
    'hq lint              # protocol lint, including the rules above',
    '```',
    '',
    '## Generated output is read-only',
    '',
    '`wiki/**` pages and `hq/AGENT-PROTOCOL.md` are generated. Never edit them by',
    'hand: change the source and re-run the generator.',
    '',
    `Protocol version: ${PROTOCOL_VERSION}`,
    '',
  ].join('\n');
}

/**
 * Write the protocol to `<root>/hq/AGENT-PROTOCOL.md`.
 *
 * @param {string} root workspace root
 * @returns {{path: string, bytes: number}}
 */
export function writeAgentProtocol(root) {
  const base = resolve(String(root));
  const text = renderAgentProtocol({ workspace: basename(base) });
  const path = join(base, 'hq', 'AGENT-PROTOCOL.md');
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, text);
  return { path, bytes: Buffer.byteLength(text, 'utf8') };
}

// ---------------------------------------------------------------------------
// Lint
// ---------------------------------------------------------------------------

const SEVERITY_RANK = { error: 0, warning: 1, info: 2 };

// secret-in-comment matches a credential assignment, not prose. The key must be
// followed by a value that looks like a literal secret: a quoted literal, or an
// unquoted opaque token. "not a password: it is generated once" is not one.
const SECRET_RE = /(password|passwd|api[_-]?key|secret|token|bearer)\s*[:=]\s*(\S+)/i;
const SECRET_BARE_MIN = 6;

// A value only counts as a credential when it is a quoted literal or an opaque
// token. Environment lookups, dotted identifiers and short prose never do.
function looksLikeSecretValue(raw) {
  const value = String(raw ?? '').trim();
  if (!value) return false;
  if (/^process\.env/i.test(value) || value.includes('${')) return false;
  const quoted = /^(['"`])([\s\S]*)\1$/.exec(value);
  if (quoted) return /\w/.test(quoted[2]);
  const bare = value.replace(/[.,;:!?)\]}]+$/, '');
  if (bare.length < SECRET_BARE_MIN) return false;
  if (/^[A-Za-z_$][\w$]*(\.[A-Za-z_$][\w$]*)*$/.test(bare)) return false;
  return /\d/.test(bare);
}

// Refuse to load huge blobs into memory; the indexer's default file cap.
const MAX_LINT_BYTES = 1_500_000;

const DOC_MARKERS = ['/**', '///', '#', '"""', '--'];

const JS_LIKE = new Set([
  'javascript', 'typescript', 'vue', 'svelte', 'go', 'rust', 'java', 'kotlin',
  'scala', 'csharp', 'fsharp', 'vbnet', 'c', 'cpp', 'swift', 'php', 'graphql',
  'protobuf',
]);
const HASH_LIKE = new Set(['python', 'ruby', 'shell', 'powershell', 'yaml', 'toml', 'ini']);

// commentSyntax — which comment forms a language uses. Deliberately small:
// the linter only needs comment bodies, not perfect parsing.
function commentSyntax(lang) {
  if (HASH_LIKE.has(lang)) return { line: ['#'], block: [], quotes: true };
  if (lang === 'sql') return { line: ['--'], block: [['/*', '*/']], quotes: true };
  if (lang === 'lua') return { line: ['--'], block: [['--[[', ']]']], quotes: true };
  if (lang === 'css') return { line: [], block: [['/*', '*/']], quotes: false };
  if (lang === 'html' || lang === 'xml') return { line: [], block: [['<!--', '-->']], quotes: false };
  if (lang === 'markdown') return { line: [], block: [['<!--', '-->']], quotes: false };
  if (JS_LIKE.has(lang)) return { line: ['//'], block: [['/*', '*/']], quotes: true };
  return { line: [], block: [], quotes: false };
}

/**
 * Scan comments only. Strings are skipped where the language uses them so a
 * `//` inside a URL, or an `api_key` assignment in code, is not mistaken for a
 * comment. Returns [{line, text}] with 1-based line numbers.
 */
function scanComments(lang, text) {
  const syntax = commentSyntax(lang);
  if (!syntax.line.length && !syntax.block.length) return [];
  const comments = [];
  const n = text.length;
  let i = 0;
  let line = 1;
  let quote = null;
  let block = null;

  while (i < n) {
    const ch = text[i];

    if (block) {
      const endIdx = text.indexOf(block.end, i);
      const stop = endIdx === -1 ? n : endIdx;
      const chunk = text.slice(i, stop);
      block.text += chunk;
      for (let k = 0; k < chunk.length; k++) if (chunk.charCodeAt(k) === 10) line++;
      if (endIdx === -1) {
        i = n;
      } else {
        i = endIdx + block.end.length;
        const body = block.text.trim();
        if (body) comments.push({ line: block.line, text: body });
        block = null;
      }
      continue;
    }

    if (quote) {
      if (ch === '\\') { i += 2; continue; }
      if (ch === quote) { quote = null; i++; continue; }
      if (ch === '\n') { quote = null; line++; i++; continue; }
      i++;
      continue;
    }

    if (syntax.quotes && (ch === '"' || ch === "'" || ch === '`')) {
      quote = ch;
      i++;
      continue;
    }

    if (ch === '\n') { line++; i++; continue; }

    let opened = false;
    for (const [open, close] of syntax.block) {
      if (text.startsWith(open, i)) {
        block = { end: close, line, text: '' };
        i += open.length;
        opened = true;
        break;
      }
    }
    if (opened) continue;

    let mark = null;
    for (const candidate of syntax.line) {
      if (text.startsWith(candidate, i)) { mark = candidate; break; }
    }
    if (mark) {
      let endIdx = text.indexOf('\n', i);
      if (endIdx === -1) endIdx = n;
      const body = text.slice(i + mark.length, endIdx).trim();
      if (body) comments.push({ line, text: body });
      i = endIdx;
      continue;
    }

    i++;
  }

  return comments;
}

// refTarget — normalize an hq see-ref to a repo-relative path.
// Strips backticks, quotes, leading ./, and a trailing :line.
function refTarget(ref) {
  let value = String(ref ?? '').trim();
  if (!value) return '';
  value = value.replace(/^[`'"]+|[`'"]+$/g, '').trim();
  if (/^https?:\/\//i.test(value)) return '';
  value = toPosix(value).replace(/^\.\//, '');
  value = value.replace(/:\d+(?:-\d+)?$/, '');
  return value.replace(/^\/+/, '');
}

function severityOf(rule) {
  if (rule === 'secret-in-comment') return 'error';
  if (rule === 'todo-without-owner' || rule === 'context-orphan') return 'warning';
  return 'info';
}

function compareFindings(a, b) {
  const sa = SEVERITY_RANK[a.severity];
  const sb = SEVERITY_RANK[b.severity];
  if (sa !== sb) return sa - sb;
  if (a.path !== b.path) return a.path < b.path ? -1 : 1;
  if (a.line !== b.line) return a.line - b.line;
  if (a.rule !== b.rule) return a.rule < b.rule ? -1 : 1;
  if (a.project !== b.project) return a.project < b.project ? -1 : 1;
  return a.message < b.message ? -1 : a.message > b.message ? 1 : 0;
}

function hasDocAbove(lines, line) {
  if (line < 2) return false;
  const prev = String(lines[line - 2] ?? '').trim();
  if (!prev) return false;
  if (DOC_MARKERS.some((marker) => prev.startsWith(marker))) return true;
  // A multi-line /** ... */ block ends with `*/` on the line above the symbol.
  return prev.endsWith('*/');
}

/**
 * Lint the agent protocol across indexed projects.
 *
 * `changedOnly` keeps only files whose mtime is newer than the project's
 * `indexed_at`, or newer than the project's last indexed commit. When neither
 * baseline exists the file counts as changed (nothing proves it is clean).
 *
 * @param {{db: object, root: string, projects?: Array<{name: string, path?: string}>,
 *   project?: string|null, changedOnly?: boolean, maxFindings?: number}} options
 * @returns {{findings: Array<{project: string, path: string, line: number,
 *   rule: string, severity: string, message: string}>,
 *   summary: {files: number, findings: number, byRule: Record<string, number>}}}
 */
export function lintProtocol({
  db,
  root,
  projects = [],
  project = null,
  changedOnly = false,
  maxFindings = 200,
} = {}) {
  if (!db) throw new Error('hq: lintProtocol requires a database');

  const cap = Number.isFinite(Number(maxFindings))
    ? Math.max(0, Math.floor(Number(maxFindings)))
    : 200;

  const allowed = new Set();
  for (const entry of Array.isArray(projects) ? projects : []) {
    if (typeof entry === 'string' && entry) allowed.add(entry);
    else if (entry && entry.name) allowed.add(entry.name);
  }

  let sql = `
    SELECT f.id, f.project_id, f.path, f.abs_path, f.lang, f.kind, f.size,
           f.lines, f.mtime_ms, f.is_test, f.is_generated,
           p.name AS project, p.indexed_at
    FROM files f
    JOIN projects p ON p.id = f.project_id`;
  const params = [];
  if (project) {
    sql += ' WHERE p.name = ?';
    params.push(project);
  }
  sql += ' ORDER BY p.name, f.path';

  const rows = db.prepare(sql).all(...params)
    .filter((row) => !allowed.size || allowed.has(row.project));

  // Every indexed path, plus every path suffix, so `lib/x.mjs` resolves
  // whether the ref is exact or a shorter unique-ish suffix.
  const knownPaths = new Set();
  for (const row of db.prepare('SELECT path FROM files').all()) {
    const p = toPosix(row.path);
    knownPaths.add(p);
    const segments = p.split('/').filter(Boolean);
    for (let k = 1; k < segments.length; k++) knownPaths.add(segments.slice(k).join('/'));
  }

  const baselineCache = new Map();
  function baseline(row) {
    if (baselineCache.has(row.project_id)) return baselineCache.get(row.project_id);
    const commit = db.prepare('SELECT MAX(date) AS d FROM commits WHERE project_id = ?').get(row.project_id);
    const commitMs = commit && commit.d ? Date.parse(commit.d) : NaN;
    const indexedMs = row.indexed_at ? Date.parse(row.indexed_at) : NaN;
    const value = {
      commitMs: Number.isFinite(commitMs) ? commitMs : null,
      indexedMs: Number.isFinite(indexedMs) ? indexedMs : null,
    };
    baselineCache.set(row.project_id, value);
    return value;
  }

  const textCache = new Map();
  function fileText(row) {
    const abs = row.abs_path || (root ? join(root, row.path) : null);
    if (!abs) return null;
    if (textCache.has(abs)) return textCache.get(abs);
    let text = null;
    if (!Number(row.size) || Number(row.size) <= MAX_LINT_BYTES) {
      try { text = readFileSync(abs, 'utf8'); } catch { text = null; }
    }
    textCache.set(abs, text);
    return text;
  }

  const annStmt = db.prepare(
    'SELECT line, kind, text, owner, ref FROM annotations WHERE file_id = ? ORDER BY line, id',
  );
  const symStmt = db.prepare(
    `SELECT name, kind, line, end_line, signature, exported, doc
     FROM symbols WHERE file_id = ? ORDER BY line, name`,
  );

  const findings = [];
  let fileCount = 0;

  for (const row of rows) {
    if (changedOnly) {
      const { indexedMs, commitMs } = baseline(row);
      const mtime = Number(row.mtime_ms) || 0;
      const changed = (indexedMs === null && commitMs === null)
        || (indexedMs !== null && mtime > indexedMs)
        || (commitMs !== null && mtime > commitMs);
      if (!changed) continue;
    }

    fileCount++;
    const rel = toPosix(row.path);
    const project = row.project;
    const text = fileText(row);
    const lines = text ? text.split(/\r?\n/) : null;
    const annotations = annStmt.all(row.id);
    const symbols = symStmt.all(row.id);

    const add = (rule, line, message) => {
      findings.push({ project, path: rel, line, rule, severity: severityOf(rule), message });
    };

    // secret-in-comment (error): comments must never carry credentials.
    if (text) {
      for (const comment of scanComments(row.lang, text)) {
        const match = SECRET_RE.exec(comment.text);
        if (match && looksLikeSecretValue(match[2])) {
          add('secret-in-comment', comment.line,
            `comment looks like it contains a credential ("${match[1].toLowerCase()}"); remove it and read the value from the environment`);
        }
      }
    }

    // todo-without-owner (warning): @hq:todo must name an owner.
    for (const ann of annotations) {
      if (ann.kind === 'todo' && !ann.owner) {
        add('todo-without-owner', ann.line,
          '@hq:todo has no owner; write @hq:todo(owner): what remains');
      }
    }

    // context-orphan (warning): @hq:see must point at an indexed file.
    for (const ann of annotations) {
      if (ann.kind !== 'see') continue;
      const target = refTarget(ann.ref);
      if (!target) {
        add('context-orphan', ann.line, '@hq:see has no ref; add <path> or <path>:<line>');
      } else if (!knownPaths.has(target)) {
        add('context-orphan', ann.line, `@hq:see ref "${target}" does not resolve to an indexed file`);
      }
    }

    // missing-file-header (info): source files > 20 lines need @hq:context.
    if (row.kind === 'source' && !row.is_generated && Number(row.lines) > 20) {
      const headerInText = lines
        ? lines.slice(0, 5).some((l) => l.includes('@hq:context'))
        : annotations.some((a) => a.kind === 'context' && a.line <= 5);
      if (!headerInText) {
        add('missing-file-header', 1,
          `source file has ${row.lines} lines but no @hq:context header in the first 5 lines`);
      }
    }

    // missing-export-doc (info): exported symbols need a doc comment above.
    if (lines) {
      for (const sym of symbols) {
        if (!sym.exported) continue;
        if (sym.doc && String(sym.doc).trim()) continue;
        if (hasDocAbove(lines, Number(sym.line))) continue;
        add('missing-export-doc', Number(sym.line),
          `exported symbol "${sym.name}" has no doc comment on the line above`);
      }
    }

    // long-function-no-why (info): anything over 80 lines explains why.
    for (const sym of symbols) {
      const start = Number(sym.line) || 0;
      const end = Number(sym.end_line) || 0;
      if (end - start <= 80) continue;
      const hasWhy = annotations.some((a) => a.kind === 'why' && a.line >= start && a.line <= end);
      if (!hasWhy) {
        add('long-function-no-why', start,
          `symbol "${sym.name}" spans ${end - start} lines with no @hq:why annotation inside it`);
      }
    }

    // deprecated-without-see (info): a retired block must point somewhere.
    for (const ann of annotations) {
      if (ann.kind !== 'deprecated') continue;
      if (ann.ref) continue;
      const near = annotations.some((a) => a.kind === 'see' && Math.abs(a.line - ann.line) <= 2);
      if (!near) {
        add('deprecated-without-see', ann.line,
          '@hq:deprecated has no nearby @hq:see; point at the replacement');
      }
    }
  }

  findings.sort(compareFindings);
  const capped = findings.slice(0, cap);

  // Counts describe every finding, not only the reported slice, so a capped
  // report still states the true total.
  const counts = {};
  const bySeverity = { error: 0, warning: 0, info: 0 };
  for (const finding of findings) {
    counts[finding.rule] = (counts[finding.rule] || 0) + 1;
    bySeverity[finding.severity] = (bySeverity[finding.severity] || 0) + 1;
  }
  const byRule = {};
  for (const rule of Object.keys(counts).sort()) byRule[rule] = counts[rule];

  return {
    findings: capped,
    summary: {
      files: fileCount,
      findings: findings.length,
      reported: capped.length,
      bySeverity,
      byRule,
    },
  };
}
