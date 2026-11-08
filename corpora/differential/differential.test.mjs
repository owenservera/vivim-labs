/**
 * Differential compatibility lab — REAL Lane A -> Lane B and Lane A -> Lane D.
 *
 * WHAT THIS FILE IS
 *   A characterisation of the OPEN, MEASURED A->B and A->D seam incompatibilities
 *   recorded in TierFlow's
 *   `VIVIM-VISUAL-RUNTIME/WAVE-2/INTEGRATION-2-CLOSURE-RESULT.md` §6
 *   ("exact A/B and A/D incompatibilities (measured, not repaired)").
 *
 *   It CHARACTERISES CURRENT BEHAVIOUR, INCLUDING KNOWN-BAD BEHAVIOUR.
 *   It does not assert that the current behaviour is correct. Where the current
 *   behaviour loses information, the loss is named, labelled, and backed by a
 *   FAILING-BY-DESIGN negative test that would go red if TierFlow were repaired.
 *
 * EVIDENCE LEVEL
 *   `real_module_integration` for every call below: the real committed TierFlow
 *   modules are imported and executed in-process. NOT `server_e2e`, NOT
 *   `native_desktop`, NOT `android_device`, NOT `live_cutover_verified`. No live
 *   Host, KB, Studio, Android, provider browser or device is contacted, no
 *   network call is made, no TierFlow file is written, and no TierFlow file is
 *   modified by this corpus.
 *
 * SOURCE PIN
 *   TierFlow checkout `C:/0-BlackBoxProject-0/vivim-ui`, branch `main`,
 *   commit ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7. The pin is asserted at
 *   runtime against the on-disk .git metadata, so a moved checkout fails loudly
 *   instead of silently describing a different source.
 *
 * RUN
 *   cd C:/0-BlackBoxProject-0/vivim-labs && bun test corpora/differential/differential.test.mjs
 */
import { test, expect } from 'bun:test';
import { existsSync, readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { TIERFLOW_SOURCE, LANE_MODULES, SEAMS, LOSSY_SEAMS, PERMISSION_ADDING_SEAMS, seamTable } from './seams.mjs';

const HERE = dirname(fileURLToPath(import.meta.url));

/** TierFlow root: overridable so a scratch copy can point at the same checkout. */
const TIERFLOW_ROOT = process.env.VIVIM_LABS_TIERFLOW_ROOT
  ? resolve(process.env.VIVIM_LABS_TIERFLOW_ROOT)
  : resolve(HERE, '../../..', 'vivim-ui');

// ---------------------------------------------------------------------------
// Real TierFlow modules. Nothing is mocked, stubbed or re-implemented here.
// ---------------------------------------------------------------------------
const { validateUiDocument } = await import(resolve(TIERFLOW_ROOT, LANE_MODULES.A));
const { UI_DOCUMENT_SCHEMA } = await import(resolve(TIERFLOW_ROOT, LANE_MODULES.A_SCHEMA));
const { inspectComponent } = await import(resolve(TIERFLOW_ROOT, LANE_MODULES.B));
const { renderUiDocument } = await import(resolve(TIERFLOW_ROOT, LANE_MODULES.D));
const { projectInertActionRefs } = await import(resolve(TIERFLOW_ROOT, LANE_MODULES.ADAPTER));

// ---------------------------------------------------------------------------
// Source pin
// ---------------------------------------------------------------------------
function readCheckedOutSha(repoRoot) {
  const gitDir = resolve(repoRoot, '.git');
  const head = readFileSync(resolve(gitDir, 'HEAD'), 'utf8').trim();
  if (!head.startsWith('ref: ')) return head;
  const ref = head.slice('ref: '.length).trim();
  const loose = resolve(gitDir, ref);
  if (existsSync(loose)) return readFileSync(loose, 'utf8').trim();
  const packed = readFileSync(resolve(gitDir, 'packed-refs'), 'utf8');
  const line = packed.split('\n').find((candidate) => candidate.endsWith(' ' + ref));
  return line ? line.split(' ')[0].trim() : null;
}

// ---------------------------------------------------------------------------
// The canonical Lane A document, shaped exactly like the closure's document.
// ---------------------------------------------------------------------------
const ACTION_REF_ID = 'fixture.preview';
const ACTION_REF_VERSION = 1;
const KNOWN_ACTIONS = [{ id: ACTION_REF_ID, version: ACTION_REF_VERSION }];

const CANONICAL_DOCUMENT = {
  schema: UI_DOCUMENT_SCHEMA,
  documentId: 'doc.differential.lab',
  scope: { ownerRef: 'owner_a', workspaceRef: 'world_a' },
  revision: 1,
  rootId: 'root',
  nodes: [
    { id: 'root', type: 'stack', children: ['panel', 'grid-1', 'action-button'], props: { direction: 'vertical' }, a11y: { role: 'main', name: 'Workspace' }, bindingRefs: [] },
    { id: 'panel', type: 'card', children: [], props: { title: 'Shared project path', description: 'Synthetic project note' }, a11y: { role: 'group', name: 'Shared project path' }, bindingRefs: ['binding-1'] },
    { id: 'grid-1', type: 'grid', children: ['cell-a'], props: { columns: 2 }, a11y: { role: 'group', name: 'Panel grid' }, bindingRefs: [] },
    { id: 'cell-a', type: 'text', children: [], props: { value: 'Shared project path', variant: 'body' }, a11y: { role: 'text', name: 'Shared project path' }, bindingRefs: [] },
    { id: 'action-button', type: 'button', children: [], props: { label: 'Preview' }, actionRefs: [{ id: ACTION_REF_ID, version: ACTION_REF_VERSION }], a11y: { role: 'button', name: 'Preview', focusOrder: 1 }, bindingRefs: [] },
  ],
  bindings: [{
    id: 'binding-1',
    version: 1,
    nodeId: 'panel',
    field: 'title',
    knowledgeRef: 'kn_8f3a0c71',
    provenanceRef: 'fixture-evidence:kn_8f3a0c71',
    readScope: 'owner',
    availability: 'present',
  }],
  views: [
    { id: 'view-desktop', version: 1, kind: '2d', viewport: 'desktop' },
    { id: 'view-phone', version: 1, kind: '2d', viewport: 'phone' },
    { id: 'view-spatial', version: 1, kind: 'spatial', fallbackViewId: 'view-desktop', sourceRef: 'fixture.spatial.source' },
  ],
};

const validated = validateUiDocument(CANONICAL_DOCUMENT, { knownActions: KNOWN_ACTIONS });
if (!validated.ok) {
  throw new Error(`canonical document must be accepted by Lane A: ${JSON.stringify(validated.diagnostics)}`);
}
const DOCUMENT = validated.document;

/** Inspect every Lane A node with the real Lane B inspector. */
function inspectAllNodes(document) {
  return document.nodes.map((node) => {
    const result = inspectComponent(node);
    return {
      id: node.id,
      type: node.type,
      ok: result.ok,
      code: result.ok ? null : result.code,
      reason: result.ok ? null : result.reason,
      value: result.ok ? result.value : null,
    };
  });
}

/**
 * FAILING-BY-DESIGN harness.
 *
 * `probe()` returns `{ ok, detail }` describing whether the CORRECT behaviour
 * (the behaviour the seam should have) currently holds.
 *
 *  - the registered `test.skip` is the real negative test: its body asserts the
 *    CORRECT behaviour, so un-skipping it reproduces the defect as a RED test;
 *  - the companion PROOF test runs the same probe NOW and asserts that the
 *    defect IS present at the pinned SHA, which keeps the suite green while
 *    making the defect observable instead of hiding it behind a skip.
 *
 * No TierFlow file is modified to produce this. It is a Labs-side observation.
 */
function failingByDesign(id, title, probe) {
  test.skip(`FAILING-BY-DESIGN ${id}: ${title}`, () => {
    const result = probe();
    expect(result.ok).toBe(true);
  });
  test(`FAILING-BY-DESIGN PROOF ${id}: ${title}`, () => {
    const result = probe();
    expect(result.ok).toBe(false);
    expect(result.detail).toContain('DEFECT PRESENT');
    expect(result.detail).toContain(result.expect);
  });
}

// ===========================================================================
// 0. Source pin and catalogue integrity
// ===========================================================================

test('source pin: the TierFlow checkout under test is the pinned commit', () => {
  expect(TIERFLOW_SOURCE.commit).toBe('ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7');
  const sha = readCheckedOutSha(TIERFLOW_ROOT);
  expect(sha).toBe(TIERFLOW_SOURCE.commit);
});

test('source pin: every lane module the corpus depends on exists in the pinned checkout', () => {
  for (const relative of Object.values(LANE_MODULES)) {
    expect(existsSync(resolve(TIERFLOW_ROOT, relative))).toBe(true);
  }
});

test('catalogue: every seam entry carries a_source and b_or_d_source as file:line', () => {
  expect(SEAMS.length).toBeGreaterThan(0);
  for (const seam of SEAMS) {
    expect(seam.a_source).toMatch(/^src\/visual-runtime\/[\w./-]+:\d+$/);
    expect(seam.b_or_d_source).toMatch(/^src\/visual-runtime\/[\w./-]+:\d+$/);
    expect(seam.lossy_dimensions).toBeArray();
    expect(['refusal', 'silent-drop', 'degrade', 'safe']).toContain(seam.failure_mode);
    expect(seam.owner.length).toBeGreaterThan(0);
  }
});

test('catalogue: no measured seam adds permission at the A/B or A/D boundary', () => {
  expect(PERMISSION_ADDING_SEAMS).toEqual([]);
});

// ===========================================================================
// 1. A -> B  (Lane A document node -> Lane B component inspector)
// ===========================================================================

test('A->B: every node of the canonical Lane A document is refused by Lane B', () => {
  const inspected = inspectAllNodes(DOCUMENT);
  expect(inspected.map((entry) => entry.id)).toEqual(['root', 'panel', 'grid-1', 'cell-a', 'action-button']);
  for (const entry of inspected) {
    expect(entry.ok).toBe(false);
    expect(entry.code).not.toBeNull();
  }
  // The exact refusal each node earns, in Lane B's own check order
  // (props -> children -> refs -> layout -> a11y).
  expect(inspected.map((entry) => [entry.id, entry.code])).toEqual([
    ['root', 'UNSAFE_PROP'],
    ['panel', 'UNSAFE_PROP'],
    ['grid-1', 'UNSAFE_PROP'],
    ['cell-a', 'UNSAFE_PROP'],
    ['action-button', 'INVALID_COMPONENT'],
  ]);
});

test('A->B: Lane B refuses Lane A card vocabulary exactly as closure §6 records it', () => {
  const card = DOCUMENT.nodes.find((node) => node.id === 'panel');
  const result = inspectComponent(card);
  expect(result.ok).toBe(false);
  expect(result.code).toBe('UNSAFE_PROP');
  expect(result.reason).toBe('unregistered/unsafe prop: description');

  // A B-native card IS accepted, which shows the seam is the vocabulary
  // difference and not a broken module (closure §6).
  const bNative = inspectComponent({
    id: 'panel',
    type: 'card',
    props: { title: 'Shared project path' },
    children: [],
    a11y: { role: 'group', name: 'Shared project path' },
  });
  expect(bNative.ok).toBe(true);
});

test('A->B: Lane A text and button nodes can never satisfy Lane B', () => {
  // Lane A makes props.value mandatory for text nodes; Lane B wants props.text.
  const text = DOCUMENT.nodes.find((node) => node.id === 'cell-a');
  const textResult = inspectComponent(text);
  expect(textResult.ok).toBe(false);
  expect(textResult.code).toBe('UNSAFE_PROP');
  expect(textResult.reason).toBe('unregistered/unsafe prop: value');

  // Lane A makes a11y.focusOrder mandatory on buttons; Lane B rejects the field.
  const button = DOCUMENT.nodes.find((node) => node.id === 'action-button');
  const buttonResult = inspectComponent(button);
  expect(buttonResult.ok).toBe(false);
  expect(buttonResult.code).toBe('INVALID_COMPONENT');
  expect(buttonResult.reason).toBe('invalid binding/action references');

  // The focusOrder refusal, isolated from the actionRefs refusal.
  const focusOnly = inspectComponent({
    id: 'b1',
    type: 'button',
    props: { label: 'Preview' },
    actionRefs: ['preview_1'],
    a11y: { role: 'button', name: 'Preview', focusOrder: 1 },
  });
  expect(focusOnly.ok).toBe(false);
  expect(focusOnly.code).toBe('INVALID_A11Y');
  expect(focusOnly.reason).toBe('unknown a11y field: focusOrder');
});

test('A->B: the minimal Lane A-valid subset that Lane B does accept', () => {
  // Lane A accepts stack/grid/card nodes whose props and roles Lane B also
  // knows. The intersection is real but narrow: it excludes text, button and
  // the document ROOT, because Lane A forces the root to role 'main'
  // (validate.ts:211) and Lane B's stack roles are {group, region}.
  const minimal = {
    schema: UI_DOCUMENT_SCHEMA,
    documentId: 'doc.differential.minimal',
    scope: { ownerRef: 'owner_a', workspaceRef: 'world_a' },
    revision: 1,
    rootId: 'root',
    nodes: [
      { id: 'root', type: 'stack', children: ['s1', 'g1', 'c1'], props: {}, a11y: { role: 'main', name: 'Workspace' }, bindingRefs: [] },
      { id: 's1', type: 'stack', children: [], props: {}, a11y: { role: 'region', name: 'S' } },
      { id: 'g1', type: 'grid', children: [], props: {}, a11y: { role: 'group', name: 'G' } },
      { id: 'c1', type: 'card', children: [], props: { title: 'T' }, a11y: { role: 'group', name: 'T' } },
    ],
    bindings: [],
    views: [{ id: 'v1', version: 1, kind: '2d', viewport: 'desktop' }],
  };
  const laneA = validateUiDocument(minimal, { knownActions: KNOWN_ACTIONS });
  expect(laneA.ok).toBe(true);
  const inspected = inspectAllNodes(laneA.document);
  expect(inspected.map((entry) => [entry.id, entry.ok])).toEqual([
    ['root', false],
    ['s1', true],
    ['g1', true],
    ['c1', true],
  ]);
  expect(inspected[0].code).toBe('INVALID_A11Y');
  expect(inspected[0].reason).toBe('role is incompatible with component type');
});

test('A->B: object-shaped actionRefs and a11y.focusOrder are additionally rejected', () => {
  // closure §6 names these two rejections explicitly.
  const objectRefs = inspectComponent({
    id: 'b1',
    type: 'button',
    props: { label: 'Preview' },
    actionRefs: [{ id: ACTION_REF_ID, version: ACTION_REF_VERSION }],
    a11y: { role: 'button', name: 'Preview' },
  });
  expect(objectRefs.ok).toBe(false);
  expect(objectRefs.code).toBe('INVALID_COMPONENT');

  // Lane B's reference alphabet excludes '.', so even a stringified canonical
  // action id is refused.
  const dottedRef = inspectComponent({
    id: 'b1',
    type: 'button',
    props: { label: 'Preview' },
    actionRefs: [ACTION_REF_ID],
    a11y: { role: 'button', name: 'Preview' },
  });
  expect(dottedRef.ok).toBe(false);
  expect(dottedRef.code).toBe('INVALID_COMPONENT');

  // An alphabet Lane B accepts is accepted.
  const plainRef = inspectComponent({
    id: 'b1',
    type: 'button',
    props: { label: 'Preview' },
    actionRefs: ['preview_1'],
    a11y: { role: 'button', name: 'Preview' },
  });
  expect(plainRef.ok).toBe(true);
});

test('A->B: the Lane A root stack and text node fail on the role vocabulary', () => {
  const stackRole = inspectComponent({
    id: 's1',
    type: 'stack',
    props: {},
    a11y: { role: 'main', name: 'Workspace' },
  });
  expect(stackRole.ok).toBe(false);
  expect(stackRole.code).toBe('INVALID_A11Y');
  expect(stackRole.reason).toBe('role is incompatible with component type');
});

test('A->B: the Lane A layout vocabulary is not Lane B\'s layout vocabulary', () => {
  const layout = inspectComponent({
    id: 'c1',
    type: 'card',
    props: { title: 'T' },
    a11y: { role: 'group', name: 'T' },
    layout: { gap: 'md' },
  });
  expect(layout.ok).toBe(false);
  expect(layout.code).toBe('INVALID_LAYOUT');
  expect(layout.reason).toBe('unknown layout field: gap');
});

test('A->B: five Lane A node types do not exist in the Lane B catalogue', () => {
  for (const type of ['collection', 'profile-card', 'project-card', 'timeline', 'chart']) {
    const result = inspectComponent({ id: 'x1', type, props: { title: 'T' }, children: [], a11y: { role: 'group', name: 'T' } });
    expect(result.ok).toBe(false);
    expect(result.code).toBe('UNREGISTERED_COMPONENT');
  }
});

test('A->B: Lane B rejects identifier shapes that Lane A accepts', () => {
  // Lane A accepts '.', ':' and up to 96 characters.
  const laneA = validateUiDocument({
    schema: UI_DOCUMENT_SCHEMA,
    documentId: 'doc.differential.id',
    scope: { ownerRef: 'owner_a', workspaceRef: 'world_a' },
    revision: 1,
    rootId: 'root',
    nodes: [
      { id: 'root', type: 'stack', children: ['node.1'], props: {}, a11y: { role: 'main', name: 'W' }, bindingRefs: [] },
      { id: 'node.1', type: 'card', children: [], props: { title: 'T' }, a11y: { role: 'group', name: 'T' } },
    ],
    bindings: [],
    views: [{ id: 'v1', version: 1, kind: '2d', viewport: 'desktop' }],
  }, { knownActions: KNOWN_ACTIONS });
  expect(laneA.ok).toBe(true);

  const result = inspectComponent(laneA.document.nodes[1]);
  expect(result.ok).toBe(false);
  expect(result.code).toBe('INVALID_COMPONENT');
  expect(result.reason).toBe('invalid opaque node ID');
});

test('A->B: Lane B refuses a node with more than four binding references', () => {
  const result = inspectComponent({
    id: 'c1',
    type: 'card',
    props: { title: 'T' },
    a11y: { role: 'group', name: 'T' },
    bindingRefs: ['b1', 'b2', 'b3', 'b4', 'b5'],
  });
  expect(result.ok).toBe(false);
  expect(result.code).toBe('INVALID_COMPONENT');
});

test('A->B: the Lane B contract has no channel for binding provenance', () => {
  // characterisation: true
  // DEFECT PINNED: Lane A's binding records carry knowledgeRef / provenanceRef /
  // readScope / availability (validate.ts:234). Lane B's InspectedComponent
  // exposes only bindingRefs: readonly string[] (catalog.ts:18), so the binding
  // PROVENANCE has no representation in the Lane B contract at all.
  // The ids survive; the provenance does not. Silent, no refusal, no marker.
  const card = DOCUMENT.nodes.find((node) => node.id === 'panel');
  const bNative = inspectComponent({
    id: 'panel',
    type: 'card',
    props: { title: 'Shared project path' },
    children: [],
    a11y: { role: 'group', name: 'Shared project path' },
    bindingRefs: ['binding-1'],
  });
  expect(bNative.ok).toBe(true);
  const value = bNative.value;
  expect(value.bindingRefs).toEqual(['binding-1']);
  const serialised = JSON.stringify(value);
  expect(serialised).toContain('binding-1');
  expect(serialised).not.toContain('kn_8f3a0c71');
  expect(serialised).not.toContain('fixture-evidence');
  expect(serialised).not.toContain('readScope');
});

failingByDesign(
  'A->B/binding-provenance',
  'Lane B must expose the provenance of the bindings it carries',
  () => {
    const card = DOCUMENT.nodes.find((node) => node.id === 'panel');
    const result = inspectComponent(card);
    const serialised = result.ok ? JSON.stringify(result.value) : '';
    const carriesProvenance = serialised.includes('kn_8f3a0c71') || serialised.includes('fixture-evidence');
    return {
      ok: carriesProvenance,
      expect: 'binding provenance',
      detail: carriesProvenance
        ? 'Lane B exposed the binding provenance'
        : `DEFECT PRESENT: Lane B exposed no binding provenance: ${serialised || '(refused)'} (source: catalog.ts:18, validate.ts:234)`,
    };
  },
);

// ===========================================================================
// 2. A -> D  (Lane A document -> Lane D renderer)
// ===========================================================================

test('A->D: the raw Lane A document renders unavailable with INVALID_REFERENCE', () => {
  const plan = renderUiDocument(DOCUMENT, { viewportPx: 1280 });
  expect(plan.status).toBe('unavailable');
  expect(plan.issues.map((issue) => issue.code)).toEqual(['INVALID_REFERENCE']);
  expect(plan.focusOrder).toEqual([]);
  expect(plan.revision).toBeNull();
});

test('A->D: the adapted document renders partial with SPATIAL_UNSUPPORTED at 320 / 360 / 1280', () => {
  const adapted = projectInertActionRefs(DOCUMENT);
  for (const viewportPx of [320, 360, 1280]) {
    const plan = renderUiDocument(adapted.rendererDocument, { viewportPx });
    expect(plan.status).toBe('partial');
    expect(plan.issues.map((issue) => issue.code)).toContain('SPATIAL_UNSUPPORTED');
    expect(plan.focusOrder).toEqual(['action-button']);
  }
});

test('A->D: the canonical version reference survives only in the adapter side table', () => {
  // characterisation: true
  // DEFECT PINNED: the canonical { id, version } action reference is NOT in the
  // document Lane D renders. Only the synthetic alias is. The canonical pair
  // survives in the adapter's separate read-only table (closure §6 describes
  // exactly this), so a consumer of the render plan alone cannot recover it.
  const adapted = projectInertActionRefs(DOCUMENT);
  const rendererDocument = JSON.stringify(adapted.rendererDocument);
  expect(rendererDocument).not.toContain(ACTION_REF_ID);
  expect(rendererDocument).toContain('synthetic_ref_');
  expect(adapted.actionVersionTable).toEqual([
    { alias: 'synthetic_ref_4_0', nodeId: 'action-button', id: ACTION_REF_ID, version: ACTION_REF_VERSION, inert: true },
  ]);
  // The table is inert: it carries no grant, no URL and no handler.
  expect(JSON.stringify(adapted.actionVersionTable)).not.toContain('execute');
  expect(JSON.stringify(adapted.actionVersionTable)).not.toContain('grant');
});

failingByDesign(
  'A->D/canonical-version-ref',
  'the rendered document itself must retain the canonical { id, version } action reference',
  () => {
    const adapted = projectInertActionRefs(DOCUMENT);
    const rendererDocument = JSON.stringify(adapted.rendererDocument);
    const carriesVersion = rendererDocument.includes(ACTION_REF_ID) && rendererDocument.includes(`"version":${ACTION_REF_VERSION}`);
    return {
      ok: carriesVersion,
      expect: 'canonical version reference',
      detail: carriesVersion
        ? 'the renderer document retained the canonical version reference'
        : `DEFECT PRESENT: the canonical version reference is absent from the renderer document, which holds only synthetic aliases ` +
          `(source: integration-2/action-adapter.mjs:19, dom.ts:164; closure §6 — the canonical reference is preserved in a SEPARATE read-only table, not in the rendered document)`,
    };
  },
);

test('A->D: the render plan carries no binding provenance', () => {
  // characterisation: true
  // DEFECT PINNED: Lane D reads only type/props/a11y/children (dom.ts:186-237).
  // document.bindings is never read, so the knowledgeRef, provenanceRef,
  // readScope, availability and even the binding id are absent from the render
  // plan. Silent: no issue code, outcome "rendered", code "OK".
  const adapted = projectInertActionRefs(DOCUMENT);
  const plan = renderUiDocument(adapted.rendererDocument, { viewportPx: 1280 });
  const serialised = JSON.stringify(plan);
  expect(serialised).not.toContain('kn_8f3a0c71');
  expect(serialised).not.toContain('fixture-evidence');
  expect(serialised).not.toContain('binding-1');
  expect(serialised).not.toContain('readScope');
  expect(plan.status).toBe('partial');
  expect(plan.issues.map((issue) => issue.code)).toEqual(['SPATIAL_UNSUPPORTED']);
});

failingByDesign(
  'A->D/binding-provenance',
  'the render plan must carry the provenance of the bindings it renders',
  () => {
    const adapted = projectInertActionRefs(DOCUMENT);
    const plan = renderUiDocument(adapted.rendererDocument, { viewportPx: 1280 });
    const serialised = JSON.stringify(plan);
    const carriesProvenance = serialised.includes('kn_8f3a0c71') || serialised.includes('fixture-evidence');
    return {
      ok: carriesProvenance,
      expect: 'binding provenance',
      detail: carriesProvenance
        ? 'the render plan carried the binding provenance'
        : 'DEFECT PRESENT: the render plan carries no binding provenance (source: dom.ts:28 RenderPlan has no bindings field; dom.ts:186-237 reads only type/props/a11y/children)',
    };
  },
);

test('A->D: Lane D accepts the card description prop and never projects it', () => {
  // characterisation: true
  // DEFECT PINNED: acceptableProps allows 'description' on a card (dom.ts:114),
  // but safeContent reads only props.title (dom.ts:123). The description is
  // accepted, dropped, and reported as rendered / OK.
  const adapted = projectInertActionRefs(DOCUMENT);
  const plan = renderUiDocument(adapted.rendererDocument, { viewportPx: 1280 });
  const serialised = JSON.stringify(plan);
  expect(serialised).toContain('Shared project path');
  expect(serialised).not.toContain('Synthetic project note');
  const cardTrace = plan.trace.find((entry) => entry.id === 'panel');
  expect(cardTrace.outcome).toBe('rendered');
  expect(cardTrace.code).toBe('OK');
});

test('A->D: Lane D replaces the authored semantic focus order with its own traversal order', () => {
  // characterisation: true
  // DEFECT PINNED: Lane A authors a11y.focusOrder (schema.ts:63 — "Focus order
  // is semantic, not inferred from geometry or DOM rendering order"). Lane D
  // ignores it and derives focusOrder from traversal plus button-ness
  // (dom.ts:211-217). PROOF: changing the authored focusOrder from 1 to 7
  // produces a byte-identical render plan, so the authored value has no effect.
  const adapted = projectInertActionRefs(DOCUMENT);
  const plan = renderUiDocument(adapted.rendererDocument, { viewportPx: 1280 });
  expect(plan.focusOrder).toEqual(['action-button']);

  const variant = {
    ...DOCUMENT,
    nodes: DOCUMENT.nodes.map((node) =>
      node.id === 'action-button'
        ? { ...node, a11y: { ...node.a11y, focusOrder: 7 } }
        : node,
    ),
  };
  const variantPlan = renderUiDocument(projectInertActionRefs(variant).rendererDocument, { viewportPx: 1280 });
  expect(JSON.stringify(variantPlan)).toBe(JSON.stringify(plan));

  // The authored numeric value is nowhere in the plan; the plan's own
  // `focusOrder` array is Lane D's derived button list.
  const serialised = JSON.stringify(plan);
  expect(serialised).not.toContain('"focusOrder":1');
  expect(serialised).not.toContain('"focusOrder":7');
});

test('A->D: Lane D remaps the Lane A accessibility role', () => {
  // characterisation: true
  // DEFECT PINNED: Lane A authored a11y.role = 'group' on the card; the render
  // plan carries role="region" (dom.ts:209 maps every non-'note' role to
  // 'region'). The authored role is silently replaced.
  const adapted = projectInertActionRefs(DOCUMENT);
  const plan = renderUiDocument(adapted.rendererDocument, { viewportPx: 1280 });
  const card = plan.root.children[0].children.find((node) => node.id === 'panel');
  expect(card.attributes.role).toBe('region');
  expect(card.attributes['aria-label']).toBe('Shared project path');
});

test('A->D: the seam adds no permission and no effect path', () => {
  // SAFETY OBSERVATION (the negative of the required check).
  const adapted = projectInertActionRefs(DOCUMENT);
  const plan = renderUiDocument(adapted.rendererDocument, { viewportPx: 1280 });
  const button = plan.root.children[0].children.find((node) => node.id === 'action-button');
  expect(button.attributes.role).toBe('button');
  expect(button.attributes['aria-disabled']).toBe('true');
  expect(button.attributes.tabindex).toBe('0');
  expect(button.attributes['aria-description']).toBe('Read-only preview. No action can be performed here.');
  const serialised = JSON.stringify(plan);
  for (const forbidden of ['execute', 'dispatch', 'onclick', 'onClick', 'href', 'http://', 'https://', 'javascript:']) {
    expect(serialised.toLowerCase()).not.toContain(forbidden.toLowerCase());
  }
  // The renderer is a description, not a DOM.
  expect(typeof plan.root).toBe('object');
  expect(plan.root.nodeType).toBeUndefined();
});

// ===========================================================================
// 3. Catalogue / README cross-checks
// ===========================================================================

test('catalogue: the measured seam set matches the live behaviour', () => {
  const table = seamTable();
  const ids = table.map((row) => row.id);

  // Every seam the closure §6 names is present in the catalogue.
  expect(ids).toContain('A->B/card-description-prop');
  expect(ids).toContain('A->B/action-ref-object-vs-string');
  expect(ids).toContain('A->B/a11y-focus-order');
  expect(ids).toContain('A->D/action-ref-shape');
  expect(ids).toContain('A->D/canonical-version-ref-dropped-from-renderer-document');
  expect(ids).toContain('A->D/no-permission-added');

  // The lossy set is exactly the set the live run measured.
  const lossyIds = LOSSY_SEAMS.map((seam) => seam.id);
  expect(lossyIds).toContain('A->D/canonical-version-ref-dropped-from-renderer-document');
  expect(lossyIds).toContain('A->D/binding-provenance-dropped');
  expect(lossyIds).toContain('A->B/binding-payload-has-no-channel');
  expect(lossyIds).not.toContain('A->D/no-permission-added');
});
