// Real ARBOR HQ parser adapter. Read-only; refuses unsafe workspace paths and symlink escapes.
import { readFile, realpath, stat } from 'node:fs/promises';
import { resolve, relative, sep, isAbsolute, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { createVlangSdk, sourceSha256, VlangSdkError, MAX_SOURCE_BYTES } from '../src/index.mjs';

const SECRET = /^(?:\.env.*|.*\.env|.*\.pem|.*\.p12|.*\.keystore|session\.json|storage-state\.json|release-profile\.json|cookies|login data)$/i;
const inside = (base, p) => p !== base && !relative(base, p).startsWith('..' + sep) && relative(base, p) !== '..' && !isAbsolute(relative(base, p));

function sourcePath(path) {
  if (typeof path !== 'string' || !path || path.includes('\\') || path.startsWith('/') ||
      path.split('/').some(s => !s || s === '.' || s === '..' || SECRET.test(s))) {
    throw new VlangSdkError('UNSAFE_PATH', 'Expected safe workspace-relative source path');
  }
  return path;
}

export async function createArborInspector({ arborRoot, workspaceRoot, projectId = 'arbor' }) {
  if (!arborRoot || !workspaceRoot) throw new VlangSdkError('MISSING_ROOT', 'ARBOR_ROOT and VLANG_WORKSPACE_ROOT required');
  const root = await realpath(workspaceRoot);
  const arbor = await realpath(arborRoot);
  const modPath = join(arbor, 'hq', 'lib', 'annotation-v2.mjs');
  const modReal = await realpath(modPath);
  if (!inside(arbor, modReal)) throw new VlangSdkError('UNSAFE_ADAPTER', 'HQ parser must remain inside ARBOR');
  const { parseAnnotationV2 } = await import(pathToFileURL(modReal).href);
  if (typeof parseAnnotationV2 !== 'function') throw new VlangSdkError('MISSING_PARSER', 'Expected ARBOR HQ v2 parser');
  const unavailable = async () => { throw new VlangSdkError('CONTEXT_NOT_WIRED', 'Verified HQ context adapter not yet connected'); };
  const sdk = createVlangSdk({ parseAnnotations: parseAnnotationV2, composeContext: unavailable, verifyContext: unavailable });

  function resolvePath(path) {
    const rel = sourcePath(path);
    const absolute = resolve(root, rel);
    if (!inside(root, absolute)) throw new VlangSdkError('UNSAFE_PATH', 'Source escapes workspace');
    return { rel, absolute };
  }
  async function inspect({ path, source }) {
    const { rel, absolute } = resolvePath(path);
    let bytes = source;
    if (bytes === undefined) {
      const actual = await realpath(absolute);
      if (!inside(root, actual)) throw new VlangSdkError('UNSAFE_PATH', 'Symlink escapes workspace');
      const info = await stat(actual);
      if (!info.isFile() || info.size > MAX_SOURCE_BYTES) throw new VlangSdkError('SOURCE_LIMIT', 'Source must be a bounded regular file');
      bytes = await readFile(actual, 'utf8');
    }
    return sdk.inspectSource({ pin: { projectId, path: rel, sourceSha256: sourceSha256(bytes) }, source: bytes });
  }
  async function inspectUri({ uri, source }) {
    if (typeof uri !== 'string' || !uri.startsWith('file://')) throw new VlangSdkError('INVALID_URI', 'Only local file documents are supported');
    const abs = fileURLToPath(uri);
    const rel = relative(root, abs).split(sep).join('/');
    return inspect({ path: rel, source });
  }
  return Object.freeze({ inspect, inspectUri, compareSourcePins: sdk.compareSourcePins, workspaceRoot: root, schema: sdk.schema });
}
