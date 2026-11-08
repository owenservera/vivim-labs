/**
 * @hq:context Labs-supplied broker fixture for the NL typed-IR conformance corpus — crosswalk rows, descriptors, host catalogue and context builder.
 * @hq:owner vivim-labs
 * @hq:task ENGINEERING-EVIDENCE-FACTORY/W3-01
 *
 * WHY THIS EXISTS
 * TierFlow's compiler takes the crosswalk from a trusted broker and never
 * creates a registry of its own (src/visual-runtime/command/compile.ts:5-10,
 * "Supplied by a trusted integration broker; not derived from owner input/AI").
 * TierFlow's own test file supplies exactly such a fixture
 * (tests/visual-runtime/command.test.ts:8-19). This module is the Labs
 * equivalent: environment data only.
 *
 * This module holds NO expectations. It contains no outcome labels, so the
 * independent oracle can share it without sharing the corpus's answers.
 *
 * PROVENANCE OF EACH ROW
 *   ui.theme.show@1 / ui.theme.set@1 / ui.layout.preview@1
 *     Mirrors the row shapes and targets used by TierFlow's own command tests
 *     (tests/visual-runtime/command.test.ts:8-16) so the corpus exercises the
 *     same public semantic identities the real suite exercises.
 *   workspace.panel.move@1 / workspace.panel.reorder@1 / workspace.panel.show@1
 *     Spec-only identities named in VIVIM Labs
 *     docs/ENGINEERING-EVIDENCE-FACTORY.md:37-38. They are NOT TierFlow
 *     commands; they are registered here so the doc's example utterances have
 *     a deterministic typed-IR target.
 *   ui.theme.reset@1 / host.device.wipe@1 / host.device.expose@1
 *     Labs extensions used to exercise refusal branches that the doc requires
 *     (unsafe, unauthorised, unreviewed).
 *
 * SCOPE IS NOT PART OF THE VISUAL COMMAND IR.
 * `vivim.visual-command/1` (src/visual-runtime/command/ir.ts:17-28) carries no
 * scope field. The corpus records the grant scope the *owning* authority
 * demands, and the oracle proves that binding against the real owner modules
 * (src/visual-runtime/revisions/fixture-store.ts, src/visual-runtime/host/*).
 * The vocabulary is TierFlow's; the action -> scope binding is a Labs
 * judgement and is labelled as such in each case.
 */

const H = (c) => c.repeat(64);

/** Rows mirroring TierFlow's own test fixture, extended for the Factory 01 spec. */
export const CROSSWALK_ROWS = Object.freeze([
  {
    semanticRef: { id: 'ui.theme.show@1', version: '1', sourceHash: H('a') },
    target: { owner: 'host_effect', actionId: 'ui.theme.show', catalogVersion: 6 },
    argsSchema: {},
    argsSchemaHash: H('b'),
    risk: 'R0',
    riskSource: { authority: 'host_catalog', sourceHash: H('c') },
    implementationStatus: 'source_wired',
    reviewedEnabled: true,
    textAliases: ['show my theme'],
  },
  {
    semanticRef: { id: 'ui.theme.set@1', version: '1', sourceHash: H('d') },
    target: { owner: 'host_effect', actionId: 'ui.theme.set_preset', catalogVersion: 6 },
    argsSchema: { preset: { type: 'enum', values: ['lab', 'tide', 'dusk', 'ember', 'grove', 'ink', 'dawn'] } },
    argsSchemaHash: H('e'),
    risk: 'R1',
    riskSource: { authority: 'host_catalog', sourceHash: H('f') },
    implementationStatus: 'source_wired',
    reviewedEnabled: true,
  },
  {
    semanticRef: { id: 'ui.theme.reset@1', version: '1', sourceHash: H('1') },
    target: { owner: 'host_effect', actionId: 'ui.theme.reset', catalogVersion: 6 },
    argsSchema: {},
    argsSchemaHash: H('2'),
    risk: 'R1',
    riskSource: { authority: 'host_catalog', sourceHash: H('3') },
    implementationStatus: 'source_wired',
    reviewedEnabled: true,
  },
  {
    semanticRef: { id: 'ui.layout.preview@1', version: '1', sourceHash: H('4') },
    target: { owner: 'ui_document', actionId: 'ui.publish', catalogVersion: 1 },
    argsSchema: {},
    argsSchemaHash: H('5'),
    risk: 'R1',
    riskSource: { authority: 'ui_contract', sourceHash: H('6') },
    implementationStatus: 'verified',
    reviewedEnabled: true,
  },
  {
    semanticRef: { id: 'workspace.panel.move@1', version: '1', sourceHash: H('7') },
    target: { owner: 'ui_document', actionId: 'ui.workspace.panel.move', catalogVersion: 1 },
    argsSchema: {
      position: { type: 'enum', values: ['above', 'below', 'top', 'bottom', 'before', 'after'] },
      anchor: { type: 'enum', values: ['activity', 'projects', 'inbox', 'calendar'] },
    },
    argsSchemaHash: H('8'),
    risk: 'R1',
    riskSource: { authority: 'ui_contract', sourceHash: H('9') },
    implementationStatus: 'source_wired',
    reviewedEnabled: true,
  },
  {
    semanticRef: { id: 'workspace.panel.reorder@1', version: '1', sourceHash: H('a') },
    target: { owner: 'ui_document', actionId: 'ui.workspace.panel.reorder', catalogVersion: 1 },
    argsSchema: { order: { type: 'enum', values: ['first', 'second', 'last'] } },
    argsSchemaHash: H('b'),
    risk: 'R1',
    riskSource: { authority: 'ui_contract', sourceHash: H('c') },
    implementationStatus: 'source_wired',
    reviewedEnabled: true,
  },
  {
    semanticRef: { id: 'workspace.panel.show@1', version: '1', sourceHash: H('d') },
    target: { owner: 'ui_document', actionId: 'ui.workspace.panel.show', catalogVersion: 1 },
    argsSchema: {},
    argsSchemaHash: H('e'),
    risk: 'R0',
    riskSource: { authority: 'ui_contract', sourceHash: H('f') },
    implementationStatus: 'source_wired',
    reviewedEnabled: true,
    textAliases: ['show my dashboard'],
  },
  {
    semanticRef: { id: 'host.device.wipe@1', version: '1', sourceHash: H('1') },
    target: { owner: 'host_effect', actionId: 'host.device.wipe', catalogVersion: 6 },
    argsSchema: {},
    argsSchemaHash: H('2'),
    risk: 'R3',
    riskSource: { authority: 'host_catalog', sourceHash: H('3') },
    implementationStatus: 'verified',
    reviewedEnabled: true,
  },
  {
    semanticRef: { id: 'host.device.expose@1', version: '1', sourceHash: H('4') },
    target: { owner: 'host_effect', actionId: 'host.device.expose', catalogVersion: 6 },
    argsSchema: {},
    argsSchemaHash: H('5'),
    risk: 'R2',
    riskSource: { authority: 'host_catalog', sourceHash: H('6') },
    implementationStatus: 'declared',
    reviewedEnabled: true,
  },
]);

/** Capability evidence attached to every descriptor, mirroring TierFlow's fixture. */
export const DEFAULT_CAPABILITY = Object.freeze({
  supportedByBuild: true,
  availableNow: 'yes',
  permittedForActor: 'unknown',
  verifiedByEvidence: false,
  reasonCodes: ['FIXTURE_ONLY'],
});

export const CURRENT_VERSIONS = Object.freeze({
  hostWorld: '3',
  policy: 'p1',
  kb: 'k1',
  uiRevision: 'u1',
});

/** Version pins every corpus input asserts, so a pin drift is a refusal, not a silent pass. */
export const EXPECTED_VERSIONS = Object.freeze({ policy: 'p1' });

function descriptorFor(row, capability) {
  return {
    target: { ...row.target },
    argsSchemaHash: row.argsSchemaHash,
    sourceHash: row.riskSource.sourceHash,
    risk: row.risk,
    implementationStatus: row.implementationStatus,
    capability: { ...capability },
  };
}

/**
 * Host action catalogue used only to prove the scope vocabulary binding.
 * `requiredScopes` mirrors src/visual-runtime/host/port.ts:15 and is enforced
 * by src/visual-runtime/host/fixture.ts:105-113.
 */
export const HOST_CATALOG_ACTIONS = Object.freeze([
  {
    actionId: 'ui.theme.show',
    catalogVersion: 6,
    risk: 'R0',
    readiness: 'source_wired',
    requiredScopes: ['control:read'],
    argsSchema: {},
    evidence: 'source_inspected',
    prerequisites: [],
  },
  {
    actionId: 'ui.theme.set_preset',
    catalogVersion: 6,
    risk: 'R1',
    readiness: 'source_wired',
    requiredScopes: ['control:execute'],
    argsSchema: { preset: { type: 'enum', values: ['lab', 'tide', 'dusk', 'ember', 'grove', 'ink', 'dawn'] } },
    evidence: 'source_inspected',
    prerequisites: [],
  },
  {
    actionId: 'ui.theme.reset',
    catalogVersion: 6,
    risk: 'R1',
    readiness: 'source_wired',
    requiredScopes: ['control:execute'],
    argsSchema: {},
    evidence: 'source_inspected',
    prerequisites: [],
  },
  {
    actionId: 'host.device.wipe',
    catalogVersion: 6,
    risk: 'R3',
    readiness: 'verified',
    requiredScopes: ['control:execute'],
    argsSchema: {},
    evidence: 'verified',
    prerequisites: [],
  },
  {
    actionId: 'host.device.expose',
    catalogVersion: 6,
    risk: 'R2',
    readiness: 'verified',
    requiredScopes: ['control:execute'],
    argsSchema: {},
    evidence: 'verified',
    prerequisites: [],
  },
]);

export const HOST_CATALOG_VERSION = 6;
export const HOST_CATALOG_DIGEST = H('c');

/**
 * Build the compile context for a case. `overrides` is a closed, declarative
 * vocabulary so a case can move the compiler into a specific branch without
 * hand-writing a whole context (and without letting a case smuggle in an
 * unreviewed row).
 *
 * Supported override keys:
 *   descriptorCapability  { actionId, patch }  -> patch descriptor capability
 *   rowPatch              { semanticId, patch }-> patch a crosswalk row
 *   duplicateSemanticId   string               -> AMBIGUOUS_MAPPING branch
 *   aliasCollision        { alias, semanticId }-> AMBIGUOUS_TEXT branch
 *   currentVersions       object               -> pin drift
 *   expectedVersions      object               -> stale pin from the input side
 */
export function buildContext(overrides = null) {
  const rows = CROSSWALK_ROWS.map((row) => structuredClone(row));
  const descriptors = CROSSWALK_ROWS.map((row) => descriptorFor(row, DEFAULT_CAPABILITY));
  const context = {
    rows,
    descriptors,
    currentVersions: { ...CURRENT_VERSIONS },
  };
  if (!overrides) return context;

  if (overrides.descriptorCapability) {
    const { actionId, patch } = overrides.descriptorCapability;
    const descriptor = context.descriptors.find((d) => d.target.actionId === actionId);
    if (!descriptor) throw new Error(`unknown descriptor actionId ${actionId}`);
    descriptor.capability = { ...descriptor.capability, ...patch };
  }
  if (overrides.rowPatch) {
    const { semanticId, patch } = overrides.rowPatch;
    const row = context.rows.find((r) => r.semanticRef.id === semanticId);
    if (!row) throw new Error(`unknown row semanticId ${semanticId}`);
    Object.assign(row, structuredClone(patch));
  }
  if (overrides.duplicateSemanticId) {
    const row = context.rows.find((r) => r.semanticRef.id === overrides.duplicateSemanticId);
    if (!row) throw new Error(`unknown row semanticId ${overrides.duplicateSemanticId}`);
    const twin = structuredClone(row);
    twin.semanticRef = { ...twin.semanticRef, sourceHash: H('0') };
    context.rows.push(twin);
  }
  if (overrides.aliasCollision) {
    const { alias, semanticId } = overrides.aliasCollision;
    const row = context.rows.find((r) => r.semanticRef.id === semanticId);
    if (!row) throw new Error(`unknown row semanticId ${semanticId}`);
    row.textAliases = [alias];
    row.argsSchema = {};
  }
  if (overrides.currentVersions) {
    Object.assign(context.currentVersions, overrides.currentVersions);
  }
  return context;
}

/** Expected version pins for a case, honouring a stale-pin override. */
export function buildExpectedVersions(overrides = null) {
  if (overrides && overrides.expectedVersions) return { ...overrides.expectedVersions };
  return { ...EXPECTED_VERSIONS };
}
