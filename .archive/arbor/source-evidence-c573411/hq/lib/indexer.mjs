// hq/lib/indexer.mjs — incremental workspace indexer.
// CONTRACT (hq/CONTRACT.md): one transaction per project, rows replaced per
// file, FTS populated from file text, imports resolved after parsing.
// Writes only to the SQLite database it is given.
import { existsSync, readFileSync, statSync } from 'node:fs';
import { isAbsolute, join, posix, resolve } from 'node:path';
import { nowISO, sha256, toPosix } from './util.mjs';
import { transaction, getMeta, setMeta } from './schema.mjs';
import { discoverFiles } from './walk.mjs';
import { isSourceLang } from './lang.mjs';
import {
  isGitRepo as isGitRepoHere,
  head as gitHead,
  branch as gitBranch,
  remote as gitRemote,
} from './git.mjs';
import { extractSymbols, extractImports, extractAnnotations } from './extract.mjs';
import { parseAnnotationV2, isVersionedId } from './annotation-v2.mjs';
import { getSemanticId, indexedSemanticAnchors, semanticAnchorChanges } from './semantic-registry.mjs';
import { isSensitiveSourcePath } from './search.mjs';

// The existing v2 scanner is a JS-style lexer, not a universal language parser.
// Do not treat JSX markup, Python strings, Rust raw strings, etc. as JS comments.
const rustHeader = entry => /\.rs$/i.test(entry.rel) && entry.lang === 'rust';
const supportsV2 = entry => rustHeader(entry) ||
  (/\.(?:mjs|cjs|js|mts|cts|ts)$/i.test(entry.rel)
  && (entry.lang === 'javascript' || entry.lang === 'typescript'));
// Rust observation is deliberately file-header-only: contiguous leading
// line comments and whitespace. Do not feed raw strings, lifetimes, block
// comments, macro bodies or arbitrary Rust syntax to the JS comment lexer.
function annotationText(entry,text){
  if(!rustHeader(entry))return text;
  const lines=[];
  for(const line of text.split('\n')){
    if(/^\s*(?:\/\/.*)?$/.test(line)){lines.push(line);continue;}
    break;
  }
  return lines.join('\n');
}

const DEFAULT_OPTIONS = {
  force: false,
  maxFileBytes: 1_500_000,
  ignores: [],
  useGit: true,
};

// Resolution order for extensionless specifiers (contract order).
const IMPORT_EXTENSIONS = [
  '.mjs', '.js', '.cjs', '.ts', '.tsx', '.jsx', '.py', '.go', '.rs', '.java',
  '.kt', '.cs', '.rb', '.php', '.lua', '.sh', '.ps1',
];

function normalizeOptions(options = {}) {
  return {
    ...DEFAULT_OPTIONS,
    ...(options && typeof options === 'object' ? options : {}),
    ignores: Array.isArray(options?.ignores) ? options.ignores : [],
  };
}

function projectAbsPath(root, project) {
  const raw = project?.abs || project?.path || project?.dir || '';
  const p = String(raw);
  if (isAbsolute(p)) return resolve(p);
  return resolve(root || process.cwd(), p);
}

function errorMessage(error) {
  return error && error.message ? String(error.message) : String(error);
}

// isWalkOmitted — did the bounded walk skip a file that is still on disk?
//
// discoverFiles (walk.mjs) drops every file over `maxFileBytes` without a trace,
// so "absent from the walk" is not proof that a path was deleted: it is equally
// consistent with a source that grew past the cap, or with a cap that was
// lowered since the last run. A tombstone is a claim that the source is gone,
// and only the filesystem can prove that. Callers use this to keep such rows
// instead of deleting them, and to report the run as an incomplete scan.
function isWalkOmitted(row, maxFileBytes, projectRoot) {
  const cap = Number(maxFileBytes);
  if (!Number.isFinite(cap) || cap <= 0) return false;
  const stored = typeof row?.abs_path === 'string' ? row.abs_path : '';
  const abs = stored || (typeof row?.path === 'string' ? resolve(projectRoot, row.path) : '');
  if (!abs) return false;
  try {
    if (!existsSync(abs)) return false;
    return statSync(abs).size > cap;
  } catch {
    return false;
  }
}

// countLines — newline count, ignoring a single trailing newline.
function countLines(text) {
  if (!text) return 0;
  let n = 1;
  for (let i = 0; i < text.length; i++) if (text.charCodeAt(i) === 10) n++;
  if (text.charCodeAt(text.length - 1) === 10) n--;
  return n < 0 ? 0 : n;
}

function readGitInfo(absPath, useGit) {
  const info = { kind: 'dir', remote: null, branch: null, head: null };
  if (useGit === false) return info;
  let isGit = false;
  try { isGit = isGitRepoHere(absPath); } catch { isGit = false; }
  if (!isGit) return info;
  info.kind = 'git';
  try { info.remote = gitRemote(absPath); } catch { info.remote = null; }
  try { info.branch = gitBranch(absPath); } catch { info.branch = null; }
  try { info.head = gitHead(absPath); } catch { info.head = null; }
  return info;
}

function upsertProject(db, { name, path, git }) {
  const now = nowISO();
  const selectByName = db.prepare('SELECT * FROM projects WHERE name = ?');
  const selectById = db.prepare('SELECT * FROM projects WHERE id = ?');
  const selectByPath = db.prepare('SELECT * FROM projects WHERE path = ?');
  let row = selectByName.get(name);
  if (!row) row = selectByPath.get(path);
  if (row) {
    db.prepare(
      'UPDATE projects SET name = ?, path = ?, kind = ?, remote = ?, branch = ?, head = ? WHERE id = ?',
    ).run(name, path, git.kind, git.remote, git.branch, git.head, row.id);
    return selectById.get(row.id);
  }
  const info = db.prepare(
    'INSERT INTO projects(name, path, kind, remote, branch, head, added_at) VALUES (?,?,?,?,?,?,?)',
  ).run(name, path, git.kind, git.remote, git.branch, git.head, now);
  return selectById.get(Number(info.lastInsertRowid));
}

function prepareStatements(db) {
  return {
    filesForProject: db.prepare('SELECT id, path, abs_path, size, mtime_ms, hash, kind, is_generated FROM files WHERE project_id = ?'),
    allFiles: db.prepare('SELECT id, path FROM files WHERE project_id = ?'),
    countFiles: db.prepare('SELECT count(*) AS c FROM files WHERE project_id = ?'),
    countSymbols: db.prepare('SELECT count(*) AS c FROM symbols WHERE project_id = ?'),
    projectById: db.prepare('SELECT * FROM projects WHERE id = ?'),
    updateCounts: db.prepare('UPDATE projects SET file_count = ?, symbol_count = ?, indexed_at = ? WHERE id = ?'),
    insertFile: db.prepare(
      `INSERT INTO files(project_id, path, abs_path, lang, kind, size, mtime_ms, hash, lines, is_test, is_generated, indexed_at)
       VALUES (?,?,?,?,?,?,?,?,?,?,?,?)`,
    ),
    updateFile: db.prepare(
      `UPDATE files SET abs_path = ?, lang = ?, kind = ?, size = ?, mtime_ms = ?, hash = ?, lines = ?,
       is_test = ?, is_generated = ?, indexed_at = ? WHERE id = ?`,
    ),
    updateStat: db.prepare('UPDATE files SET size = ?, mtime_ms = ? WHERE id = ?'),
    deleteFile: db.prepare('DELETE FROM files WHERE id = ?'),
    deleteSymbols: db.prepare('DELETE FROM symbols WHERE file_id = ?'),
    deleteImports: db.prepare('DELETE FROM imports WHERE file_id = ?'),
    deleteAnnotations: db.prepare('DELETE FROM annotations WHERE file_id = ?'),
    deleteFts: db.prepare('DELETE FROM content_fts WHERE rowid = ?'),
    insertFts: db.prepare('INSERT INTO content_fts(rowid, path, content, project, lang) VALUES (?,?,?,?,?)'),
    insertSymbol: db.prepare(
      `INSERT INTO symbols(file_id, project_id, name, kind, line, end_line, signature, exported, doc)
       VALUES (?,?,?,?,?,?,?,?,?)`,
    ),
    insertImport: db.prepare(
      'INSERT INTO imports(file_id, project_id, specifier, kind, line) VALUES (?,?,?,?,?)',
    ),
    insertAnnotation: db.prepare(
      `INSERT INTO annotations(file_id, project_id, line, kind, text, owner, ref)
       VALUES (?,?,?,?,?,?,?)`,
    ),
    projectImports: db.prepare(
      `SELECT i.id, i.specifier, i.kind AS import_kind, f.path AS file_path, f.lang AS importer_lang
       FROM imports i JOIN files f ON f.id = i.file_id
       WHERE i.project_id = ?`,
    ),
    setImportResolution: db.prepare(
      'UPDATE imports SET resolved_file_id = ?, resolved_project_id = ? WHERE id = ?',
    ),
  };
}

// ---------------------------------------------------------------------------
// resolveImport
// ---------------------------------------------------------------------------

function normalizeFileIndex(fileIndex) {
  const out = new Map();
  if (!fileIndex) return out;
  if (fileIndex instanceof Map) {
    for (const [key, value] of fileIndex) out.set(toPosix(key), value);
    return out;
  }
  if (fileIndex instanceof Set) {
    for (const key of fileIndex) out.set(toPosix(key), toPosix(key));
    return out;
  }
  if (typeof fileIndex === 'object') {
    for (const [key, value] of Object.entries(fileIndex)) out.set(toPosix(key), value);
  }
  return out;
}

function isRelativeSpecifier(spec) {
  if (spec === '.' || spec === '..') return true;
  if (spec.startsWith('./') || spec.startsWith('../')) return true;
  // Python-style relative imports: `.mod`, `..pkg.mod`.
  if (/^\.\.?[^./]/.test(spec)) return true;
  return false;
}

function resolveRelativeTarget(importerPath, spec) {
  const dir = posix.dirname(toPosix(importerPath) || '');
  if (/^\.\.?[^./]/.test(spec)) {
    const dots = /^\.+/.exec(spec)[0].length;
    const rest = spec.slice(dots);
    const up = '../'.repeat(Math.max(0, dots - 1));
    return posix.normalize(posix.join(dir, up + rest));
  }
  return posix.normalize(posix.join(dir === '' ? '.' : dir, spec));
}

function candidatePaths(target) {
  if (!target || target === '.') return [];
  const out = [target];
  for (const ext of IMPORT_EXTENSIONS) out.push(target + ext);
  for (const ext of IMPORT_EXTENSIONS) out.push(`${target}/index${ext}`);
  // TypeScript convention: `./x.js` written for a `./x.ts` source file.
  const jsExt = /\.(mjs|cjs|jsx|js)$/.exec(target);
  if (jsExt) {
    const base = target.slice(0, -jsExt[0].length);
    out.push(`${base}.ts`, `${base}.tsx`, `${base}.mts`, `${base}.cts`);
  }
  out.push(`${target}/__init__.py`);
  return out;
}

function bareCandidates(spec, importerPath = '') {
  const clean = String(spec).replace(/^node:/, '').replace(/[/\\]+$/, '');
  const out = [];
  if (clean) out.push(clean);
  const segs = clean.split('/').filter(Boolean);
  if (clean.startsWith('@') && segs.length >= 2) {
    out.push(segs.slice(0, 2).join('/'));
    if (segs.length > 2) out.push(segs.slice(0, 3).join('/'));
  } else if (segs.length >= 1) {
    out.push(segs[0]);
  }
  // Alias prefixes used by bundlers that point back at the project root.
  if (importerPath) {
    if (clean.startsWith('~/')) out.push(clean.slice(2));
    if (clean.startsWith('@/')) out.push(clean.slice(2));
  }
  return [...new Set(out.filter(Boolean))];
}

function buildProjectNameIndex(db) {
  const map = new Map();
  let rows = [];
  try { rows = db.prepare('SELECT id, name, path FROM projects').all(); } catch { rows = []; }
  for (const row of rows) {
    const key = String(row.name || '').toLowerCase();
    if (key && !map.has(key)) map.set(key, row.id);
  }
  for (const row of rows) {
    try {
      const pkgPath = join(row.path, 'package.json');
      const pkg = JSON.parse(readFileSync(pkgPath, 'utf8'));
      const key = String(pkg?.name || '').toLowerCase();
      if (key && !map.has(key)) map.set(key, row.id);
    } catch { /* no package.json or unreadable: ignore */ }
  }
  return map;
}

// Rust module resolution uses exact, already-indexed files. Never guess a
// dependency from a crate/package string or add an edge outside this index.
// Only explicitly indexed local Cargo path dependencies may become cross-crate
// Rust edges. Unknown registry packages and ambiguous paths stay unresolved.
function rustManifest(text){
  let section='',packageName=null,libName=null,libPath='src/lib.rs';
  const paths=[];
  for(const line of String(text).split(/\r?\n/)){
    const heading=/^\s*\[{1,2}([A-Za-z0-9_.-]+)\]{1,2}\s*(?:#.*)?$/.exec(line);
    if(heading){section=heading[1];continue;}
    let m;
    if(section==='package'&&(m=/^\s*name\s*=\s*"([^"]+)"/.exec(line)))packageName=m[1];
    if(section==='lib'){
      if((m=/^\s*name\s*=\s*"([^"]+)"/.exec(line)))libName=m[1];
      if((m=/^\s*path\s*=\s*"([^"]+)"/.exec(line)))libPath=m[1];
    }
    if(['dependencies','dev-dependencies','build-dependencies'].includes(section)){
      m=/^\s*([A-Za-z0-9_-]+)\s*=\s*\{[^}]*\bpath\s*=\s*"([^"]+)"[^}]*\}/.exec(line);
      if(m)paths.push([m[1].replace(/-/g,'_'),m[2]]);
    }
  }
  return {packageName,libName,libPath,paths};
}
function indexedRustCrates(entries,index,readFile){
  const crates=new Map();
  for(const entry of entries){
    const path=toPosix(entry.rel);
    if(posix.basename(path)!=='Cargo.toml'||!index.has(path)||entry.size>200000)continue;
    let data;
    try{data=rustManifest(readFile(entry.abs,'utf8'))}catch{continue}
    if(!data.packageName)continue;
    const lib=posix.normalize(posix.join(posix.dirname(path),data.libPath));
    if(!lib.endsWith('.rs')||lib.startsWith('../')||!index.has(lib))continue;
    crates.set(path,{rootDir:posix.dirname(lib),rootFile:lib,
      libName:data.libName??data.packageName.replace(/-/g,'_'),
      paths:data.paths,aliases:new Map()});
  }
  for(const [manifest,crate]of crates){
    for(const [alias,relative]of crate.paths){
      const target=posix.normalize(posix.join(posix.dirname(manifest),relative,'Cargo.toml'));
      const other=crates.get(target);
      if(other&&other!==crate)crate.aliases.set(alias,other);
    }
  }
  return crates;
}

function resolveRustLocal({importerPath,specifier,kind,index,rustCrates=null}){
  const miss={file_id:null,project_id:null,kind:'external'};
  if(!importerPath.endsWith('.rs')||!['use','mod'].includes(kind))return miss;
  const path=toPosix(importerPath);
  // A bin under src/bin is a distinct crate, not a module of src/lib.rs.
  const bin=/(^|\\/)src\\/bin\\/.test(path);
  let rootDir=null,rootFile=null,dir=posix.dirname(path);
  while(dir&&dir!=='.'){
    for(const name of ['lib.rs','main.rs']){
      const candidate=posix.join(dir,name);
      if(index.has(candidate)){rootDir=dir;rootFile=candidate;break}
    }
    if(rootDir)break;
    const parent=posix.dirname(dir);if(parent===dir)break;dir=parent;
  }
  if(!rootDir)return miss;
  if(posix.basename(path)==='main.rs'&&index.has(path))rootFile=path;
  const rel=posix.relative(rootDir,path);
  if(rel.startsWith('../')||rel==='..')return miss;
  let components=rel.split('/');
  const file=components.pop();
  if(file!=='lib.rs'&&file!=='main.rs'&&file!=='mod.rs')components.push(file.slice(0,-3));
  // For a bin crate, local mod declarations are bin-relative, and crate::
  // means the bin itself; never silently bind it to library exports.
  if(bin){
    if(kind==='use'&&specifier.trim().startsWith('crate::'))return miss;
    const binParts=rel.split('/');binParts.pop();
    components=binParts;
    if(file!=='main.rs'&&file!=='mod.rs')components.push(file.slice(0,-3));
  }
  let parts;
  if(kind==='mod'){
    if(!/^[A-Za-z_]\\w*$/.test(specifier))return miss;
    parts=[...components,specifier];
  }else{
    const tokens=String(specifier).trim().split('::');
    const first=tokens.shift();
    if(first==='crate'){
      if(bin)return miss;parts=[];
    }else if(first==='self'){
      parts=[...components];
    }else if(first==='super'){
      parts=[...components];
      if(!parts.length)return miss;
      parts.pop();
      while(tokens[0]==='super'){
        if(!parts.length)return miss;
        parts.pop();tokens.shift();
      }
    }else{
      const manifest=rustCrates?.get(posix.join(posix.dirname(rootDir),'Cargo.toml'));
      const target=manifest?.aliases.get(first)||
        (bin&&manifest?.libName===first?manifest:null);
      if(!target)return miss;
      rootDir=target.rootDir;rootFile=target.rootFile;parts=[];
    }
    for(const token of tokens){
      if(!/^[A-Za-z_]\\w*$/.test(token))break;
      parts.push(token);
    }
    // crate::{Error,Graph} names exactly the library root, not a module.
    if(!parts.length&&first==='crate'&&rootFile&&index.has(rootFile)){
      return {file_id:index.get(rootFile),project_id:null,kind:'file'};
    }
  }
  // Prefer the deepest concrete module actually present. This handles both
  // src/module.rs and src/module/mod.rs and paths to re-exported symbols.
  for(let length=parts.length;length>=1;length--){
    const base=posix.join(rootDir,...parts.slice(0,length));
    for(const candidate of [base+'.rs',posix.join(base,'mod.rs')]){
      if(candidate!==path&&index.has(candidate)){
        return {file_id:index.get(candidate),project_id:null,kind:'file'};
      }
    }
  }
  return miss;
}

/**
 * resolveImport — resolve one specifier to a file and/or project.
 * @returns {{file_id: number|null, project_id: number|null, kind: 'file'|'project'|'external'}}
 */
export function resolveImport({ db, projectId = null, file = null, specifier = '', fileIndex = null, projectNames = null, importerLang = null, importKind = null, rustCrates = null } = {}) {
  const miss = { file_id: null, project_id: null, kind: 'external' };
  const spec = String(specifier ?? '').trim();
  if (!spec) return miss;
  const importerPath = typeof file === 'string' ? file : (file?.path ? String(file.path) : '');
  const index = normalizeFileIndex(fileIndex);

  if(importerLang==='rust'&&['use','mod'].includes(importKind)){
    return resolveRustLocal({importerPath,specifier:spec,kind:importKind,index,rustCrates});
  }

  if (isRelativeSpecifier(spec)) {
    const target = resolveRelativeTarget(importerPath, spec);
    if (!target || target === '.' || target === '..') return miss;
    if (target.startsWith('../')) return miss; // outside the project root
    for (const candidate of candidatePaths(target)) {
      const id = index.get(candidate);
      if (id !== undefined && id !== null) {
        return { file_id: id, project_id: null, kind: 'file' };
      }
    }
    return miss;
  }

  let names = projectNames;
  if (!(names instanceof Map)) {
    names = db ? buildProjectNameIndex(db) : new Map();
  }
  for (const candidate of bareCandidates(spec, importerPath)) {
    const id = names.get(candidate.toLowerCase());
    if (id !== undefined && id !== null) {
      return { file_id: null, project_id: id, kind: 'project' };
    }
  }
  return miss;
}

// ---------------------------------------------------------------------------
// indexProject
// ---------------------------------------------------------------------------

/**
 * indexProject — index one project's files.
 * @returns {{project: object, added:number, changed:number, unchanged:number,
 *   removed:number, symbols:number, imports:number, annotations:number,
 *   fts:number, errors: Array<{project:string, path:string, message:string}>,
 *   ms:number}}
 */
export function indexProject({ root, db, project, options = {} }) {
  const started = Date.now();
  const opts = normalizeOptions(options);
  const name = String(project?.name || project?.id || '');
  const absPath = projectAbsPath(root, project);
  const errors = [];
  const result = {
    project: { name, path: absPath },
    added: 0,
    changed: 0,
    unchanged: 0,
    removed: 0,
    symbols: 0,
    imports: 0,
    annotations: 0,
    semanticChanges: [],
    semanticChangeComplete: false,
    incomplete: [],
    fts: 0,
    errors,
    ms: 0,
  };

  if (!existsSync(absPath)) {
    errors.push({ project: name, path: absPath, message: 'project path does not exist' });
    result.ms = Date.now() - started;
    return result;
  }

  const readFile = typeof opts.readFile === 'function' ? opts.readFile : readFileSync;
  const extract = {
    symbols: typeof opts.extractors?.extractSymbols === 'function' ? opts.extractors.extractSymbols : extractSymbols,
    imports: typeof opts.extractors?.extractImports === 'function' ? opts.extractors.extractImports : extractImports,
    annotations: typeof opts.extractors?.extractAnnotations === 'function' ? opts.extractors.extractAnnotations : extractAnnotations,
  };

  const git = readGitInfo(absPath, opts.useGit);
  const entries = discoverFiles(absPath, {
    maxFileBytes: opts.maxFileBytes,
    extraIgnores: opts.ignores,
    ignores: opts.ignores,
    useGit: opts.useGit,
  }).filter(entry => !isSensitiveSourcePath(entry.rel))
    .sort((a, b) => (a.rel < b.rel ? -1 : a.rel > b.rel ? 1 : 0));

  const finalRow = transaction(db, () => {
    // Snapshot before upsertProject overwrites head/branch: a no-op reindex
    // (watcher catch-up on an already-current tree, hook after a commit that
    // changed no indexed file) must not bump indexed_at — the project wiki
    // pages embed it, and an unconditional bump churns them with no settle
    // scheduled on the zero-delta watch path (wiki --check then fails after
    // every watcher restart).
    const priorRow = db
      .prepare('SELECT id, head, branch, file_count, symbol_count, indexed_at FROM projects WHERE name = ?')
      .get(name) || null;
    const projectRow = upsertProject(db, { name, path: absPath, git });
    const stmts = prepareStatements(db);
    // Backfill existing database files on the first successful v2 index only.
    const v2Key = 'annotation_v2:' + projectRow.id + ':' + absPath;
    const backfillV2 = getMeta(db, v2Key) !== '3';
    // Re-extract existing Rust mod/use records once after resolver expansion.
    const rustIndexKey='rust_local_imports_v2:'+projectRow.id+':'+absPath;
    const refreshRustIndex=getMeta(db,rustIndexKey)!=='2';

    const existing = new Map();
    for (const row of stmts.filesForProject.all(projectRow.id)) existing.set(row.path, row);

    const seen = new Set();
    const anchors=row=>row&&row.kind==='source'&&!row.is_generated
      ?indexedSemanticAnchors(db,{fileId:row.id,project:projectRow.name,path:row.path,hash:row.hash}):[];
    const changes=(path,before,after)=>result.semanticChanges.push(...semanticAnchorChanges(before,after)
      .map(change=>({path,...change})));

    for (const entry of entries) {
      seen.add(entry.rel);
      const prev = existing.get(entry.rel) || null;

      const refreshV2 = backfillV2 && supportsV2(entry);
      const refreshRust=refreshRustIndex&&entry.lang==='rust';
      if (!opts.force && !refreshV2 && prev && !isSourceLang(entry.lang) && prev.size === entry.size && prev.mtime_ms === entry.mtimeMs) {
        result.unchanged++;
        continue;
      }

      let data;
      try {
        data = readFile(entry.abs);
      } catch (error) {
        errors.push({ project: name, path: entry.rel, message: errorMessage(error) });
        continue;
      }
      const buffer = Buffer.isBuffer(data) ? data : Buffer.from(String(data), 'utf8');
      const hash = sha256(buffer);

      if (!opts.force && !refreshV2 && !refreshRust && prev && prev.hash === hash) {
        stmts.updateStat.run(entry.size, entry.mtimeMs, prev.id);
        result.unchanged++;
        continue;
      }

      const text = buffer.toString('utf8');
      const lang = entry.lang;
      const kind = entry.kind;
      const source = isSourceLang(lang);

      let symbols = [];
      let imports = [];
      let annotations = [];
      try {
        if (source) {
          symbols = extract.symbols(lang, text) || [];
          imports = extract.imports(lang, text) || [];
          annotations = extract.annotations(lang, text) || [];
          if (supportsV2(entry) && text.includes('@hq:')) {
            const parsed = parseAnnotationV2(annotationText(entry,text), {
              projectId: String(projectRow.id), path: entry.rel, contentHash: hash,
              registry: { lookup: id => getSemanticId(db, id) },
            });
            annotations = [...annotations, ...parsed.records.map(record => ({
              line: record.line, kind: 'v2:' + record.kind, text: record.value,
              ref: isVersionedId(record.value) ? record.value : null,
            }))];
            // Unknown/deprecated IDs remain observations, not registry writes.
            // Malformed/secret values are not emitted by the existing parser.
          }
        } else if (lang === 'markdown') {
          annotations = extract.annotations(lang, text) || [];
        }
      } catch (error) {
        errors.push({ project: name, path: entry.rel, message: errorMessage(error) });
        continue;
      }

      const lines = countLines(text);
      const indexedAt = nowISO();
      let fileId;
      if (prev) {
        stmts.updateFile.run(
          entry.abs, lang, kind, entry.size, entry.mtimeMs, hash, lines,
          entry.isTest ? 1 : 0, entry.isGenerated ? 1 : 0, indexedAt, prev.id,
        );
        fileId = prev.id;
        result.changed++;
      } else {
        const info = stmts.insertFile.run(
          projectRow.id, entry.rel, entry.abs, lang, kind, entry.size, entry.mtimeMs,
          hash, lines, entry.isTest ? 1 : 0, entry.isGenerated ? 1 : 0, indexedAt,
        );
        fileId = Number(info.lastInsertRowid);
        result.added++;
      }

      const beforeAnchors=anchors(prev);
      // Replace children transactionally (we are already inside the project transaction).
      stmts.deleteSymbols.run(fileId);
      for (const symbol of symbols) {
        if (!symbol || !symbol.name) continue;
        stmts.insertSymbol.run(
          fileId, projectRow.id, String(symbol.name), String(symbol.kind || 'variable'),
          Number(symbol.line) || 0, Number(symbol.end_line) || Number(symbol.line) || 0,
          symbol.signature ? String(symbol.signature) : null,
          symbol.exported ? 1 : 0,
          symbol.doc ? String(symbol.doc) : null,
        );
        result.symbols++;
      }

      stmts.deleteImports.run(fileId);
      for (const imp of imports) {
        if (!imp || !imp.specifier) continue;
        stmts.insertImport.run(
          fileId, projectRow.id, String(imp.specifier),
          String(imp.kind || 'import'), Number(imp.line) || 0,
        );
        result.imports++;
      }

      stmts.deleteAnnotations.run(fileId);
      for (const annotation of annotations) {
        if (!annotation || !annotation.kind) continue;
        stmts.insertAnnotation.run(
          fileId, projectRow.id, Number(annotation.line) || 0,
          String(annotation.kind), String(annotation.text ?? ''),
          annotation.owner ? String(annotation.owner) : null,
          annotation.ref ? String(annotation.ref) : null,
        );
        result.annotations++;
      }

      changes(entry.rel,beforeAnchors,anchors({id:fileId,path:entry.rel,kind,is_generated:entry.isGenerated,hash}));
      stmts.deleteFts.run(fileId);
      stmts.insertFts.run(fileId, entry.rel, text, projectRow.name, lang);
      result.fts++;
    }

    // Remove rows for files that disappeared from disk.
    //
    // A path the bounded walk never returned is not necessarily deleted: walk.mjs
    // silently drops files over `maxFileBytes`, so a source that grew past the cap
    // (or a cap lowered since the last run) looks identical to a removed file.
    // Such a row is kept and the run is reported incomplete; deleting it would
    // tombstone a live source and its semantic anchors in the native graph.
    const stale = [...existing.values()]
      .filter((row) => !seen.has(row.path))
      .sort((a, b) => (a.path < b.path ? -1 : a.path > b.path ? 1 : 0));
    for (const row of stale) {
      if (isWalkOmitted(row, opts.maxFileBytes, absPath)) {
        result.incomplete.push({ project: projectRow.name, path: row.path, reason: 'over_file_cap' });
        continue;
      }
      // Reaching here means the file is not on disk at all: the deletion is
      // proven by the filesystem, not inferred from the walk, so its tombstone
      // is safe to emit even in a run that omitted other files.
      changes(row.path,anchors(row),[]);
      stmts.deleteFts.run(row.id);
      stmts.deleteFile.run(row.id);
      result.removed++;
    }

    // Resolve imports once every file row exists.
    const fileIndex = new Map();
    for (const row of stmts.allFiles.all(projectRow.id)) fileIndex.set(row.path, row.id);
    const projectNames = buildProjectNameIndex(db);
    const rustCrates=indexedRustCrates(entries,fileIndex,readFile);
    for (const imp of stmts.projectImports.all(projectRow.id)) {
      const resolution = resolveImport({
        db,
        projectId: projectRow.id,
        file: { path: imp.file_path },
        specifier: imp.specifier,
        importerLang: imp.importer_lang,
        importKind: imp.import_kind,
        rustCrates,
        fileIndex,
        projectNames,
      });
      stmts.setImportResolution.run(resolution.file_id, resolution.project_id, imp.id);
    }

    const fileCount = Number(stmts.countFiles.get(projectRow.id)?.c || 0);
    const symbolCount = Number(stmts.countSymbols.get(projectRow.id)?.c || 0);
    const indexChurned = (result.added + result.changed + result.removed) > 0
      || !priorRow
      || priorRow.head !== projectRow.head
      || priorRow.branch !== projectRow.branch
      || Number(priorRow.file_count) !== fileCount
      || Number(priorRow.symbol_count) !== symbolCount;
    if (indexChurned || !priorRow?.indexed_at) {
      stmts.updateCounts.run(fileCount, symbolCount, nowISO(), projectRow.id);
    }
    // Zero-delta reindex on an unchanged tree: counts already correct,
    // indexed_at stays — nothing happened worth churning derived pages for.

    if (backfillV2 && errors.length === 0) setMeta(db, v2Key, '3');
    if (refreshRustIndex && errors.length === 0 && result.incomplete.length === 0) setMeta(db,rustIndexKey,'2');
    return stmts.projectById.get(projectRow.id);
  });

  result.project = finalRow;
  // A run that could not read every file, or that the bounded walk could not see
  // every file of, is not a complete scan: it may not publish semantic deltas as
  // complete (see result.incomplete).
  result.semanticChangeComplete = errors.length === 0 && result.incomplete.length === 0;
  result.ms = Date.now() - started;
  return result;
}

// ---------------------------------------------------------------------------
// indexWorkspace
// ---------------------------------------------------------------------------

/**
 * indexWorkspace — index every registered project.
 * @returns {{projects: Array<object>, totals: {files:number, symbols:number,
 *   imports:number, annotations:number, errors:number}, errors: Array<object>}}
 */
export function indexWorkspace({ root, db, projects = [], options = {} }) {
  const list = Array.isArray(projects) ? projects : [];
  const rows = [];
  const errors = [];
  const totals = { files: 0, symbols: 0, imports: 0, annotations: 0, incomplete: 0, errors: 0 };

  for (const project of list) {
    const name = String(project?.name || project?.id || '');
    try {
      const result = indexProject({ root, db, project, options });
      for (const error of result.errors) errors.push(error);
      // An omitted file is not an error: the read never failed, the bounded walk
      // simply could not see the path. It is reported separately so `errors`
      // keeps meaning "something went wrong", and `semanticChangeComplete`
      // still goes false for the run.
      const incomplete = Array.isArray(result.incomplete) ? result.incomplete : [];
      rows.push({
        name: result.project?.name ?? name,
        added: result.added,
        changed: result.changed,
        unchanged: result.unchanged,
        removed: result.removed,
        symbols: result.symbols,
        imports: result.imports,
        annotations: result.annotations,
        semanticChanges:result.semanticChanges,
        semanticChangeComplete:result.semanticChangeComplete,
        incomplete,
        ms: result.ms,
      });
      const row = db.prepare('SELECT id, file_count, symbol_count FROM projects WHERE name = ?').get(result.project?.name ?? name);
      if (row) {
        totals.files += Number(row.file_count || 0);
        totals.symbols += Number(row.symbol_count || 0);
        const imports = db.prepare('SELECT count(*) AS c FROM imports WHERE project_id = ?').get(row.id);
        totals.imports += Number(imports?.c || 0);
        const annotations = db.prepare('SELECT count(*) AS c FROM annotations WHERE project_id = ?').get(row.id);
        totals.annotations += Number(annotations?.c || 0);
      }
      totals.incomplete += incomplete.length;
    } catch (error) {
      errors.push({ project: name, path: String(project?.abs || project?.path || ''), message: errorMessage(error) });
      rows.push({
        name, added: 0, changed: 0, unchanged: 0, removed: 0, symbols: 0, imports: 0,
        annotations: 0, semanticChanges: [], semanticChangeComplete: false, incomplete: [], ms: 0,
      });
    }
  }

  totals.errors = errors.length;
  return { projects: rows, totals, errors };
}

// ---------------------------------------------------------------------------
// Runtime-consumption analysis over an existing index.
//
// The index records WHAT a file imports, not WHETHER the import ever ran.
// Two questions need that distinction, and both are answerable only from the
// index plus an explicit allowlist of what actually executed:
//
//   HQ-025  which imported modules were never executed?
//   HQ-026  which capabilities are exercised only by test fixtures?
//
// Neither conclusion may be asserted from import presence alone: a module
// imported by a test is consumed, and a module imported by production code
// has been executed as soon as that code runs. These helpers therefore
// report the STATIC facts (who imports what, which files are tests) and
// leave the execution verdict to the caller's allowlist.
// ---------------------------------------------------------------------------

/**
 * importConsumers — every resolved inbound import edge of a target file,
 * tagged with the importer's test flag and the edge kind.
 *
 * @param {object} db
 * @param {{target?: string, project?: string|null, limit?: number}} options
 * @returns {Array<{from:string, fromProject:string, fromIsTest:number, specifier:string, kind:string, line:number}>}
 */
export function importConsumers(db, options = {}) {
  const target = options.target ? String(options.target) : null;
  const project = options.project ?? null;
  const limit = Number(options.limit ?? 500);
  const clauses = [];
  const params = [];
  if (target) {
    clauses.push('lower(tf.path) = lower(?)');
    params.push(toPosix(target));
  }
  if (project) {
    clauses.push('sp.name = ?');
    params.push(project);
  }
  const where = clauses.length ? ` AND ${clauses.join(' AND ')}` : '';
  const rows = db.prepare(`
SELECT sf.path AS from_path, sp.name AS from_project, sf.is_test AS from_test,
       i.specifier AS specifier, i.kind AS kind, i.line AS line
FROM imports i
JOIN files sf ON sf.id = i.file_id
JOIN projects sp ON sp.id = sf.project_id
JOIN files tf ON tf.id = i.resolved_file_id
WHERE i.resolved_file_id IS NOT NULL${where}
ORDER BY sp.name ASC, sf.path ASC, i.line ASC
LIMIT ?`).all(...params, limit);
  return rows.map((r) => ({
    from: r.from_path,
    fromProject: r.from_project,
    fromIsTest: Number(r.from_test),
    specifier: r.specifier,
    kind: r.kind,
    line: Number(r.line),
  }));
}

/**
 * executionEvidence — for each imported module, the set of consumers split by
 * whether the consumer is production code or a test fixture, and whether the
 * edge is static or dynamic.
 *
 * A module is only EVER-EXERCISED-BY-TESTS when every consumer is a test
 * file. The caller supplies the executed allowlist for what is known to run
 * at runtime; this function supplies the static evidence it is judged against.
 *
 * @param {object} db
 * @param {{project?: string|null, limit?: number}} options
 * @returns {Array<{path:string, project:string, productionConsumers:Array<object>, testConsumers:Array<object>, staticEdges:number, dynamicEdges:number}>}
 */
export function executionEvidence(db, options = {}) {
  const project = options.project ?? null;
  const limit = Number(options.limit ?? 500);
  const projectClause = project ? ' AND sp.name = ?' : '';
  const params = project ? [project] : [];
  const rows = db.prepare(`
SELECT tf.path AS to_path, tp.name AS to_project,
       sf.path AS from_path, sp.name AS from_project, sf.is_test AS from_test,
       i.specifier AS specifier, i.kind AS kind, i.line AS line
FROM imports i
JOIN files sf ON sf.id = i.file_id
JOIN projects sp ON sp.id = sf.project_id
JOIN files tf ON tf.id = i.resolved_file_id
JOIN projects tp ON tp.id = tf.project_id
WHERE i.resolved_file_id IS NOT NULL${projectClause}
ORDER BY tp.name ASC, tf.path ASC, sp.name ASC, sf.path ASC, i.line ASC
LIMIT ?`).all(...params, limit);

  const byTarget = new Map();
  for (const row of rows) {
    const key = `${row.to_project}\u0000${row.to_path}`;
    if (!byTarget.has(key)) {
      byTarget.set(key, {
        path: row.to_path,
        project: row.to_project,
        productionConsumers: [],
        testConsumers: [],
        staticEdges: 0,
        dynamicEdges: 0,
      });
    }
    const entry = byTarget.get(key);
    const consumer = {
      from: row.from_path,
      fromProject: row.from_project,
      specifier: row.specifier,
      kind: row.kind,
      line: Number(row.line),
    };
    if (Number(row.from_test) === 1) entry.testConsumers.push(consumer);
    else entry.productionConsumers.push(consumer);
    if (String(row.kind) === 'dynamic') entry.dynamicEdges++;
    else entry.staticEdges++;
  }
  return [...byTarget.values()];
}

/**
 * testOnlyCapabilities — exported symbols that live in a test file and have
 * no production consumer: capabilities that work ONLY through test fixtures.
 *
 * @param {object} db
 * @param {{project?: string|null, limit?: number}} options
 * @returns {Array<{name:string, kind:string, path:string, project:string, line:number, productionConsumers:number}>}
 */
export function testOnlyCapabilities(db, options = {}) {
  const project = options.project ?? null;
  const limit = Number(options.limit ?? 500);
  const projectClause = project ? ' AND p.name = ?' : '';
  const params = project ? [project] : [];
  const rows = db.prepare(`
SELECT s.name AS name, s.kind AS kind, s.line AS line,
       f.path AS path, p.name AS project,
       (SELECT COUNT(*) FROM imports i
        JOIN files pf ON pf.id = i.file_id
        WHERE i.resolved_file_id = f.id AND i.file_id <> f.id AND pf.is_test = 0) AS production_consumers
FROM symbols s
JOIN files f ON f.id = s.file_id
JOIN projects p ON p.id = f.project_id
WHERE s.exported = 1 AND f.is_test = 1${projectClause}
ORDER BY p.name ASC, f.path ASC, s.line ASC
LIMIT ?`).all(...params, limit);
  return rows
    .filter((r) => Number(r.production_consumers) === 0)
    .map((r) => ({
      name: r.name,
      kind: r.kind,
      path: r.path,
      project: r.project,
      line: Number(r.line),
      productionConsumers: 0,
    }));
}

// ---------------------------------------------------------------------------
// Planned-feature cross-reference.
//
// A planned feature is a claim made in a plan/roadmap/canon ledger. The index
// records what the source actually contains. Cross-referencing the two answers
// two distinct AII questions:
//
//   HQ-021  which planned features are ALREADY implemented?
//   HQ-087  which planned features are UNNECESSARY because an equivalent
//            capability already exists?
//
// Both are computed from the same primitives: an exported symbol (or a file)
// whose normalized name matches the planned feature's terms. "Implemented" means
// the symbol exists. "Unnecessary" means the symbol exists AND is already
// reachable from production code (a non-test consumer imports it), so
// re-planning it would duplicate live behaviour rather than fill a hole.
//
// The matcher is deliberately conservative: a planned feature matches a symbol
// only when EVERY significant term of the feature title appears as a token in
// the symbol name. A single shared word is never enough, because "relay" alone
// matches dozens of unrelated symbols in a large index.
// ---------------------------------------------------------------------------

const PLAN_STOPWORDS = new Set([
  'the', 'a', 'an', 'and', 'or', 'to', 'of', 'for', 'in', 'on', 'with', 'via',
  'from', 'by', 'at', 'is', 'are', 'be', 'so', 'that', 'this', 'it', 'as',
  'up', 'out', 'no', 'not', 'add', 'use', 'using', 'make', 'get', 'set',
  'new', 'all', 'its', 'own', 'can', 'will', 'than', 'into', 'them', 'they',
  'when', 'then', 'there', 'their', 'each', 'any', 'one', 'two', 'per',
]);

// planTerms — significant lowercase terms of a planned-feature title.
export function planTerms(title) {
  return String(title ?? '')
    .toLowerCase()
    .split(/[^a-z0-9]+/)
    .filter((w) => w.length > 2 && !PLAN_STOPWORDS.has(w));
}

// symbolTokens — split an identifier into lowercase tokens so that
// "recordReceipt" and "record_receipt" both yield ["record", "receipt"].
export function symbolTokens(name) {
  return String(name ?? '')
    .replace(/([a-z0-9])([A-Z])/g, '$1 $2')
    .toLowerCase()
    .split(/[^a-z0-9]+/)
    .filter(Boolean);
}

/**
 * plannedFeatureStatus — cross-reference planned features against the index.
 *
 * A planned feature matches an exported symbol when EVERY term of the feature
 * appears as a token of the symbol name. Terms come from `feature.terms` when
 * given, otherwise from the title (see planTerms).
 *
 * The exported-symbol catalogue is read ONCE per call and turned into a
 * token -> symbols inverted index, so matching is a posting-list intersection
 * rather than a scan per term. No catalogue cap is applied: a match can never
 * be dropped by truncation.
 *
 * `implemented` means a matching symbol exists in the index.
 * `reachable` means at least one matching symbol lives in a file that
 * production (non-test) code imports — the capability is wired into live code.
 *
 * @param {object} db
 * @param {{features?: Array<{id?:string, title?:string, terms?:string[]}>,
 *          project?: string|null, limit?: number, maxMatches?: number}} options
 * @returns {Array<{id:string, title:string, terms:string[], implemented:boolean,
 *   reachable:boolean, matches:Array<{name:string, path:string, project:string, line:number, productionConsumers:number}>}>}
 */
export function plannedFeatureStatus(db, options = {}) {
  const features = Array.isArray(options.features) ? options.features : [];
  const limit = Number(options.limit ?? 200);
  const maxMatches = Number(options.maxMatches ?? 20);
  const projectClause = options.project ? ' AND p.name = ?' : '';
  const projectParams = options.project ? [options.project] : [];

  const catalogueStmt = db.prepare(`
SELECT s.name AS name, s.kind AS kind, s.line AS line,
       f.path AS path, p.name AS project, f.id AS file_id
FROM symbols s
JOIN files f ON f.id = s.file_id
JOIN projects p ON p.id = f.project_id
WHERE s.exported = 1${projectClause}
ORDER BY p.name ASC, f.path ASC, s.line ASC`);

  // token -> rows that carry it
  const inverted = new Map();
  const rows = catalogueStmt.all(...projectParams);
  rows.forEach((row, index) => {
    for (const token of new Set(symbolTokens(row.name))) {
      let list = inverted.get(token);
      if (!list) inverted.set(token, (list = []));
      list.push(index);
    }
  });

  // Production consumers are only needed for symbols that actually matched.
  const consumersStmt = db.prepare(`
SELECT COUNT(*) AS c
FROM imports i
JOIN files pf ON pf.id = i.file_id
WHERE i.resolved_file_id = ? AND i.file_id <> ? AND pf.is_test = 0`);
  const consumersCache = new Map();

  return features.slice(0, limit).map((feature) => {
    const title = String(feature?.title ?? '');
    const terms = Array.isArray(feature?.terms) && feature.terms.length
      ? feature.terms.map(String)
      : planTerms(title);

    // Intersect the posting lists of every term: a match must carry ALL of them.
    let hits = null;
    for (const term of terms) {
      const list = inverted.get(term) || [];
      if (!list.length) { hits = []; break; }
      if (hits === null) { hits = list.slice(); continue; }
      const keep = new Set(list);
      hits = hits.filter((i) => keep.has(i));
      if (!hits.length) break;
    }
    if (hits === null) hits = [];

    const matches = hits.map((i) => {
      const row = rows[i];
      let consumers = consumersCache.get(row.file_id);
      if (consumers === undefined) {
        consumers = Number(consumersStmt.get(row.file_id, row.file_id).c);
        consumersCache.set(row.file_id, consumers);
      }
      return {
        name: row.name,
        kind: row.kind,
        line: Number(row.line),
        path: row.path,
        project: row.project,
        productionConsumers: consumers,
      };
    });
    matches.sort((a, b) =>
      b.productionConsumers - a.productionConsumers
      || a.project.localeCompare(b.project)
      || a.path.localeCompare(b.path)
      || a.line - b.line);

    const reachable = matches.some((m) => m.productionConsumers > 0);
    return {
      id: String(feature?.id ?? ''),
      title,
      terms,
      implemented: matches.length > 0,
      reachable,
      matches: matches.slice(0, maxMatches),
    };
  });
}

/**
 * duplicatePlannedFeatures — planned features whose capability already exists
 * and is already reachable from production code, so planning them again would
 * duplicate live behaviour (HQ-087).
 *
 * @param {object} db
 * @param {{features?: Array<{id?:string, title?:string, terms?:string[]}>,
 *          project?: string|null, limit?: number}} options
 * @returns {Array<{id:string, title:string, equivalent:{name:string, path:string, project:string, line:number}}>}
 */
export function duplicatePlannedFeatures(db, options = {}) {
  const status = plannedFeatureStatus(db, options);
  const out = [];
  for (const row of status) {
    if (!row.reachable) continue;
    const equivalent = row.matches.find((m) => m.productionConsumers > 0);
    if (!equivalent) continue;
    out.push({
      id: row.id,
      title: row.title,
      equivalent: {
        name: equivalent.name,
        path: equivalent.path,
        project: equivalent.project,
        line: equivalent.line,
      },
    });
  }
  return out;
}
