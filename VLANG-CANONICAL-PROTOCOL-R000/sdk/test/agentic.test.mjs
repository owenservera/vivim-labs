import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, mkdir, writeFile, symlink, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { createArborInspector } from '../adapters/arbor-hq.mjs';

async function sandbox(t) {
  const root = await mkdtemp(join(tmpdir(), 'vlang-agentic-'));
  t.after(async () => rm(root, { recursive: true, force: true }));
  const arbor = join(root, 'arbor'), workspace = join(root, 'project');
  await mkdir(join(arbor, 'hq', 'lib'), { recursive: true });
  await mkdir(join(workspace, 'src'), { recursive: true });
  // Explicit test adapter, not a claim of real ARBOR parser execution.
  await writeFile(join(arbor, 'hq', 'lib', 'annotation-v2.mjs'),
    'export function parseAnnotationV2(source, o) { return {' +
    ' schema: "hq.annotation/2", diagnostics: [],' +
    ' records: source.startsWith("// @hq:") ? [{kind:"capability", value:"demo@1", line:1,' +
    ' projectId:o.projectId, path:o.path, contentHash:o.contentHash}] : []' +
    '}; }\n');
  const body = '// @hq:capability demo@1\nexport function demo() {}\n';
  await writeFile(join(workspace, 'src', 'demo.ts'), body);
  return { root, arbor, workspace, body };
}

test('real filesystem read uses source pin; never grants runtime authority', async (t) => {
  const s = await sandbox(t);
  const inspector = await createArborInspector({arborRoot:s.arbor, workspaceRoot:s.workspace, projectId:'project'});
  const result = await inspector.inspect({path:'src/demo.ts'});
  assert.equal(result.annotations.length, 1);
  assert.equal(result.annotations[0].executionAuthorized, false);
  assert.equal(result.executionAuthorized, false);
  assert.equal(result.pin.sourceSha256.length, 64);
  const inMemory = await inspector.inspectUri({uri:pathToFileURL(join(s.workspace,'src','demo.ts')).href, source:s.body});
  assert.equal(inMemory.pin.sourceSha256, result.pin.sourceSha256);
});

test('traversal, unsupported language, and symlinks cannot escape workspace', async (t) => {
  const s = await sandbox(t);
  const inspector = await createArborInspector({arborRoot:s.arbor, workspaceRoot:s.workspace});
  await assert.rejects(() => inspector.inspect({path:'../foo.rs'}), /UNSAFE_PATH/);
  await assert.rejects(() => inspector.inspect({path:'.env/credential.ts',source:s.body}), /UNSAFE_PATH/);
  await assert.rejects(() => inspector.inspect({path:'src/view.tsx',source:s.body}), /UNSUPPORTED_LANGUAGE/);
  await symlink(join(s.arbor,'hq','lib','annotation-v2.mjs'), join(s.workspace,'src','external.ts'));
  await assert.rejects(() => inspector.inspect({path:'src/external.ts'}), /UNSAFE_PATH/);
});

test('invalid parser root is refused without falling back to heuristics', async t => {
  const s = await sandbox(t);
  await assert.rejects(() => createArborInspector({arborRoot:'',workspaceRoot:s.workspace}), /MISSING_ROOT/);
});
