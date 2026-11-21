import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, rmSync, realpathSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createDevIntelligence } from '../lib/arbor-service.mjs';
import { createControlPlanner } from '../../apps/vivim-host/control/intent-planner.mjs';
import { resolveBuildRequest } from '../../apps/mobile-relay/build-composer.mjs';
import { createSurface } from '../../apps/mobile-relay/generative-ui/catalog.mjs';
import { createVecCompiler } from '../../apps/mobile-relay/vec/compiler.mjs';

const sourceRoot = relative => realpathSync(fileURLToPath(new URL(relative, import.meta.url)));

test('real VLANG command and Build compiler contracts resolve through authorized HQ context', () => {
  // Index the real production compiler and consumer directories, not copied
  // demo functions or a handcrafted database. Only the index is temporary.
  const root = mkdtempSync(join(tmpdir(), 'arbor-vlang-context-'));
  const projects = [
    ['vlang-command', sourceRoot('../../apps/vivim-host/control/')],
    ['vlang-vec', sourceRoot('../../apps/mobile-relay/vec/')],
    ['vlang-ui-consumer', sourceRoot('../../apps/mobile-relay/generative-ui/')],
  ];
  try {
    const hq = createDevIntelligence({
      mode: 'diagnostic', dataDir: root, approvedRoots: projects.map(([, path]) => path),
    });
    for (const [name, path] of projects) hq.registerSource({ name, path });
    assert.equal(hq.index().errors.length, 0);

    const plan = createControlPlanner().planText(
      'edit wiki section main/about saying Updated from VIVIM', { source: 'desktop' });
    assert.equal(plan.actionId, 'wiki.section.edit');
    assert.equal(plan.outcome, 'approval_required');
    const command = plan.language.vec;
    assert.equal(command.authorization, 'none');
    assert.equal(command.persisted, false);
    assert.deepEqual(command.sourceSemanticIds, [
      'vivim.vec-compile@0.1', 'vivim.command-intent@0.1',
    ]);

    const build = resolveBuildRequest({ request: 'Add a card called Research', surface: createSurface() });
    assert.equal(build.outcome, 'candidate');
    assert.equal(build.authorization, 'none');
    assert.equal(build.persisted, false);
    assert.deepEqual(build.sourceSemanticIds, [
      'vivim.vec-compile@0.1', 'vivim.ui-proposal@1',
    ]);

    const linkedContext = (candidate, project, path, semanticId, kind) => {
      const bundle = hq.context({
        task: 'Trace how this compiled candidate is handled',
        project, budgetTokens: 10000, maxFiles: 24,
        sourceSemanticIds: candidate.sourceSemanticIds,
      });
      assert.deepEqual(bundle.sourceSemanticIds, [...candidate.sourceSemanticIds].sort());
      assert.equal(bundle.authority, 'read_only_projection');
      const observed = bundle.sources.find(source => source.path === path &&
        source.semanticReferences?.some(ref => ref.semanticId === semanticId &&
          ref.annotationKind === kind));
      // If the existing budget/ranking selects the annotated file, it must
      // include the exact annotation and source hash, not a made-up snippet.
      assert.ok(observed, 'Expected real indexed contract in ' + project + '/' + path);
      assert.match(observed.contentHash, /^[a-f0-9]{64}$/);
      assert.equal(hq.verifyContext(bundle).valid, true);
      assert.equal(hq.verifyContext({ ...bundle, sourceSemanticIds: ['invented.contract@1'] }).valid, false);
      assert.ok(observed.semanticReferences.every(ref => ref.registryStatus !== 'current'),
        'source indexing must not promote registry identity');
    };
    linkedContext(command, 'vlang-command', 'command-intent.mjs', 'vivim.command-intent@0.1', 'type');
    linkedContext(command, 'vlang-command', 'intent-planner.mjs', 'vivim.command-intent@0.1', 'consumes');
    linkedContext(build, 'vlang-vec', 'ui-adapter.mjs', 'vivim.ui-proposal@1', 'emits');
    linkedContext(build, 'vlang-ui-consumer', 'compiler.mjs', 'vivim.ui-proposal@1', 'consumes');
    assert.throws(() => hq.context({
      task: 'trace', sourceSemanticIds: ['../../secret'],
    }), /HQ_INVALID_SOURCE_SEMANTIC_IDS/);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test('refused, uninstalled and schema-mismatched adapters cannot assert source identity', () => {
  const fake = createVecCompiler({ adapters: {
    command: () => ({ outcome: 'candidate', proposal: { schema: 'unrelated/1' },
      sourceSemanticIds: ['invented.contract@1'], authorization: 'full', executed: true }),
  } }).compile({ domain: 'command', text: 'test' });
  assert.equal(fake.authorization, 'none');
  assert.equal(fake.executed, false);
  assert.equal(fake.sourceSemanticIds, undefined);
  assert.equal(createVecCompiler().compile({ domain: 'wiki', text: 'test' }).sourceSemanticIds, undefined);
});
