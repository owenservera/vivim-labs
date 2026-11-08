/**
 * Seam catalogue — the A -> B and A -> D handoffs of TierFlow's visual runtime.
 *
 * SOURCE PIN
 *   Every entry below describes behaviour of the TierFlow checkout at
 *   commit ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7 (branch `main`).
 *   Nothing here is copied from TierFlow; the modules are imported live by
 *   `differential.test.mjs` and the catalogue is a description of what those
 *   live modules actually do at that SHA.
 *
 * WHAT THIS CATALOGUE IS
 *   A characterisation of the OPEN seams recorded in
 *   `VIVIM-VISUAL-RUNTIME/WAVE-2/INTEGRATION-2-CLOSURE-RESULT.md` §6
 *   ("exact A/B and A/D incompatibilities (measured, not repaired)").
 *   It is NOT a repair proposal and NOT an approval of any behaviour.
 *
 * LOSSY DIMENSIONS
 *   `canonical-version-ref`  the canonical {id, version} action reference is
 *                             not carried across the seam in the artefact the
 *                             consumer actually receives.
 *   `provenance`             knowledgeRef / provenanceRef / evidenceRefs /
 *                             readScope never reach the consumer artefact.
 *   `permission-added`       the seam grants, implies or upgrades authority.
 *                             A `false` here is a *safety* observation.
 *   `content` / `semantic-role` / `semantic-focus-order` are additional,
 *   non-permission fidelity losses measured in the same runs.
 *
 * FAILURE MODE VOCABULARY
 *   `refusal`       the seam returns a typed refusal; nothing is silently lost.
 *   `silent-drop`   the seam accepts the input and discards part of it with no
 *                   refusal, no issue code and no marker.
 *   `degrade`       the seam accepts the input but changes its meaning.
 *   `safe`          the seam carries the value faithfully.
 */

/** @type {{repo: string, local_path: string, branch: string, commit: string, closure_result: string, pinned_on: string}} */
export const TIERFLOW_SOURCE = Object.freeze({
  repo: 'owenservera/tierflow',
  local_path: 'C:/0-BlackBoxProject-0/vivim-ui',
  branch: 'main',
  commit: 'ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7',
  closure_result: 'VIVIM-VISUAL-RUNTIME/WAVE-2/INTEGRATION-2-CLOSURE-RESULT.md',
  closure_section: '§6 — exact A/B and A/D incompatibilities (measured, not repaired)',
  pinned_on: '2026-11-06',
});

/** Lane A is the document authority; Lane B the component inspector; Lane D the renderer. */
export const LANE_MODULES = Object.freeze({
  A: 'src/visual-runtime/document/validate.ts',
  A_SCHEMA: 'src/visual-runtime/document/schema.ts',
  B: 'src/visual-runtime/components/catalog.ts',
  B_A11Y: 'src/visual-runtime/components/accessibility.ts',
  B_LAYOUT: 'src/visual-runtime/components/layout.ts',
  D: 'src/visual-runtime/render/dom.ts',
  ADAPTER: 'src/visual-runtime/integration-2/action-adapter.mjs',
});

/**
 * @typedef {object} Seam
 * @property {string} id
 * @property {'A->B'|'A->D'} seam
 * @property {string} a_source      file:line in Lane A that PRODUCES the shape
 * @property {string} b_or_d_source file:line in Lane B or Lane D that EXPECTS the shape
 * @property {string} a_input_shape
 * @property {string} b_or_d_expected_shape
 * @property {{field: string, a_type: string, a_value: string, b_type: string, b_value: string}} mismatch
 * @property {boolean} lossy
 * @property {string[]} lossy_dimensions
 * @property {'refusal'|'silent-drop'|'degrade'|'safe'} failure_mode
 * @property {string|null} failure_code
 * @property {string|null} failure_reason
 * @property {string} observed
 * @property {string} closure_ref
 * @property {string} owner
 * @property {string} evidence   test name in differential.test.mjs that measures it
 */

/** @type {Seam[]} */
export const SEAMS = [
  // -------------------------------------------------------------------------
  // A -> B  (Lane A document node  ->  Lane B component inspector)
  // -------------------------------------------------------------------------
  {
    id: 'A->B/card-description-prop',
    seam: 'A->B',
    a_source: 'src/visual-runtime/document/validate.ts:72',
    b_or_d_source: 'src/visual-runtime/components/catalog.ts:30',
    a_input_shape: "card / profile-card / project-card props: { title: string, description?: string }",
    b_or_d_expected_shape: "card props: { title } only — PROP_KEYS.card is the closed list ['title']",
    mismatch: {
      field: 'props.description',
      a_type: 'string (present, non-empty, FORBIDDEN-free)',
      a_value: "any safe text, e.g. 'Synthetic project note'",
      b_type: 'key absent from the closed prop list',
      b_value: "'description'",
    },
    lossy: false,
    lossy_dimensions: [],
    failure_mode: 'refusal',
    failure_code: 'UNSAFE_PROP',
    failure_reason: 'unregistered/unsafe prop: description',
    observed:
      "Lane B refuses the whole Lane A card node. A B-native card { type:'card', props:{ title } } is accepted, " +
      "so the seam is the A/B vocabulary difference, not a broken module.",
    closure_ref: 'closure §6 — A → B vocabulary mismatch (measured refusal)',
    owner: 'Lane B — Wave 3 canonical map',
    evidence: 'A->B: Lane B refuses Lane A card vocabulary exactly as closure §6 records it',
  },
  {
    id: 'A->B/stack-props-direction-gap',
    seam: 'A->B',
    a_source: 'src/visual-runtime/document/validate.ts:70',
    b_or_d_source: 'src/visual-runtime/components/catalog.ts:29',
    a_input_shape: "stack props: { direction?: 'vertical'|'horizontal', gap?: 'sm'|'md'|'lg' }",
    b_or_d_expected_shape: "stack props: {} — PROP_KEYS.stack is empty",
    mismatch: {
      field: 'props.direction / props.gap',
      a_type: "string enum ('vertical'|'horizontal'|'sm'|'md'|'lg')",
      a_value: "'vertical'",
      b_type: 'key absent from the closed prop list',
      b_value: "'direction'",
    },
    lossy: false,
    lossy_dimensions: [],
    failure_mode: 'refusal',
    failure_code: 'UNSAFE_PROP',
    failure_reason: 'unregistered/unsafe prop: direction',
    observed: 'The Lane A root stack is refused by Lane B on its own direction prop.',
    closure_ref: 'closure §6 — A → B vocabulary mismatch (measured refusal)',
    owner: 'Lane B — Wave 3 canonical map',
    evidence: 'A->B: every node of the canonical Lane A document is refused by Lane B',
  },
  {
    id: 'A->B/grid-props-columns-gap',
    seam: 'A->B',
    a_source: 'src/visual-runtime/document/validate.ts:71',
    b_or_d_source: 'src/visual-runtime/components/catalog.ts:29',
    a_input_shape: "grid props: { columns?: number(1..4), gap?: 'sm'|'md'|'lg' }",
    b_or_d_expected_shape: "grid props: {} — PROP_KEYS.grid is empty",
    mismatch: {
      field: 'props.columns / props.gap',
      a_type: 'number(1..4) or gap enum',
      a_value: '2',
      b_type: 'key absent from the closed prop list',
      b_value: "'columns'",
    },
    lossy: false,
    lossy_dimensions: [],
    failure_mode: 'refusal',
    failure_code: 'UNSAFE_PROP',
    failure_reason: 'unregistered/unsafe prop: columns',
    observed: 'Lane B has no notion of Lane A grid column counts.',
    closure_ref: 'closure §6 — A → B vocabulary mismatch (measured refusal)',
    owner: 'Lane B — Wave 3 canonical map',
    evidence: 'A->B: every node of the canonical Lane A document is refused by Lane B',
  },
  {
    id: 'A->B/text-prop-value-vs-text',
    seam: 'A->B',
    a_source: 'src/visual-runtime/document/validate.ts:80',
    b_or_d_source: 'src/visual-runtime/components/catalog.ts:29',
    a_input_shape: "text props: { value: string (REQUIRED by Lane A), variant?: 'body'|'heading'|'muted' }",
    b_or_d_expected_shape: "text props: { text } — PROP_KEYS.text is ['text']",
    mismatch: {
      field: 'props.value vs props.text',
      a_type: 'string, mandatory (Lane A rejects a text node without it)',
      a_value: "'Shared project path'",
      b_type: 'different key name for the same payload',
      b_value: "'text'",
    },
    lossy: false,
    lossy_dimensions: [],
    failure_mode: 'refusal',
    failure_code: 'UNSAFE_PROP',
    failure_reason: 'unregistered/unsafe prop: value',
    observed:
      'Lane A makes props.value mandatory for text nodes, so a Lane A text node can NEVER satisfy Lane B. ' +
      'The mismatch is structural, not a property the caller can omit.',
    closure_ref: 'closure §6 — A → B vocabulary mismatch (measured refusal); Owner: Lane B — Wave 3 canonical map',
    owner: 'Lane B — Wave 3 canonical map',
    evidence: 'A->B: Lane A text and button nodes can never satisfy Lane B',
  },
  {
    id: 'A->B/action-ref-object-vs-string',
    seam: 'A->B',
    a_source: 'src/visual-runtime/document/validate.ts:202',
    b_or_d_source: 'src/visual-runtime/components/catalog.ts:129',
    a_input_shape: 'actionRefs: [{ id: string, version: number }] — object shaped, version is mandatory and bound',
    b_or_d_expected_shape: 'actionRefs: string[] — inert identifiers only, matched by REF = /^[A-Za-z0-9_-]{1,64}$/',
    mismatch: {
      field: 'actionRefs[] element',
      a_type: 'object { id, version }',
      a_value: "{ id: 'fixture.preview', version: 1 }",
      b_type: 'string',
      b_value: "'fixture.preview' (which B's REF would ALSO reject: '.' is outside B's alphabet)",
    },
    lossy: false,
    lossy_dimensions: [],
    failure_mode: 'refusal',
    failure_code: 'INVALID_COMPONENT',
    failure_reason: 'invalid binding/action references',
    observed:
      "Lane A binds the action version against a trusted fixture catalogue (UNBOUND_ACTION otherwise). " +
      "Lane B carries no version field at all, so the canonical version reference has no representation in " +
      "Lane B's actionRefs: readonly string[] (catalog.ts:20).",
    closure_ref: 'closure §6 — "Lane B additionally rejects … A\'s object-shaped actionRefs"',
    owner: 'Lane B — Wave 3 canonical map',
    evidence: 'A->B: object-shaped actionRefs and a11y.focusOrder are additionally rejected',
  },
  {
    id: 'A->B/a11y-focus-order',
    seam: 'A->B',
    a_source: 'src/visual-runtime/document/validate.ts:181',
    b_or_d_source: 'src/visual-runtime/components/accessibility.ts:33',
    a_input_shape: "a11y: { role, name, description?, focusOrder?: number, live? } — focusOrder is Lane A's semantic focus order",
    b_or_d_expected_shape: "a11y keys: role, name, description, tabIndex, live, touchTargetPx — 'focusOrder' is not a key",
    mismatch: {
      field: 'a11y.focusOrder',
      a_type: 'number (0..L.nodes, unique across the document)',
      a_value: '1',
      b_type: 'key absent from the closed a11y key set',
      b_value: "'focusOrder'",
    },
    lossy: false,
    lossy_dimensions: [],
    failure_mode: 'refusal',
    failure_code: 'INVALID_A11Y',
    failure_reason: 'unknown a11y field: focusOrder',
    observed:
      'Lane A REQUIRES a11y.focusOrder on every button (validate.ts:192). Lane B rejects the very same field. ' +
      'Therefore a Lane A button can never be inspected by Lane B.',
    closure_ref: 'closure §6 — "Lane B additionally rejects A\'s a11y.focusOrder"',
    owner: 'Lane B — Wave 3 canonical map',
    evidence: 'A->B: Lane A text and button nodes can never satisfy Lane B',
  },
  {
    id: 'A->B/a11y-role-vocabulary',
    seam: 'A->B',
    a_source: 'src/visual-runtime/document/validate.ts:182',
    b_or_d_source: 'src/visual-runtime/components/accessibility.ts:24',
    a_input_shape: "a11y.role ∈ {main, region, group, heading, text, button, list, listitem, status}",
    b_or_d_expected_shape: 'a11y.role ∈ ROLES[type] — stack:{group,region} grid:{group,region} card:{article,group} text:{paragraph} button:{button}',
    mismatch: {
      field: 'a11y.role',
      a_type: 'Lane A role enum',
      a_value: "'main' on the root stack, 'text' on text nodes",
      b_type: 'Lane B per-type role list',
      b_value: "'main' not in ROLES.stack, 'text' not in ROLES.text",
    },
    lossy: false,
    lossy_dimensions: [],
    failure_mode: 'refusal',
    failure_code: 'INVALID_A11Y',
    failure_reason: 'role is incompatible with component type',
    observed:
      'Lane A requires a role on every node (validate.ts:182), so the role vocabulary mismatch is unavoidable ' +
      'for the roles Lane A uses and Lane B does not know.',
    closure_ref: 'closure §6 — A → B vocabulary mismatch (measured refusal)',
    owner: 'Lane B — Wave 3 canonical map',
    evidence: 'A->B: the Lane A root stack and text node fail on the role vocabulary',
  },
  {
    id: 'A->B/layout-vocabulary',
    seam: 'A->B',
    a_source: 'src/visual-runtime/document/validate.ts:176',
    b_or_d_source: 'src/visual-runtime/components/layout.ts:29',
    a_input_shape: 'layout: { minWidth?, maxWidth?, columns?, gap?, phone?, desktop? }',
    b_or_d_expected_shape: 'layout keys: mode, gapPx, paddingPx, minItemWidthPx, maxColumns, maxWidthPx, motion',
    mismatch: {
      field: 'layout.gap',
      a_type: "enum 'sm'|'md'|'lg'",
      a_value: "'md'",
      b_type: 'numeric pixel constraint under a different key',
      b_value: 'gapPx',
    },
    lossy: false,
    lossy_dimensions: [],
    failure_mode: 'refusal',
    failure_code: 'INVALID_LAYOUT',
    failure_reason: 'unknown layout field: gap',
    observed: 'The two lanes share the word "layout" and nothing else about it.',
    closure_ref: 'closure §6 — A → B vocabulary mismatch (measured refusal)',
    owner: 'Lane B — Wave 3 canonical map',
    evidence: 'A->B: the Lane A layout vocabulary is not Lane B\'s layout vocabulary',
  },
  {
    id: 'A->B/node-type-vocabulary',
    seam: 'A->B',
    a_source: 'src/visual-runtime/document/schema.ts:12',
    b_or_d_source: 'src/visual-runtime/components/catalog.ts:8',
    a_input_shape: 'node.type ∈ {stack, grid, text, card, button, collection, profile-card, project-card, timeline, chart}',
    b_or_d_expected_shape: 'COMPONENT_TYPES = {stack, grid, card, text, heading, button, status, field, divider}',
    mismatch: {
      field: 'node.type',
      a_type: 'Lane A node type',
      a_value: "'collection' | 'profile-card' | 'project-card' | 'timeline' | 'chart'",
      b_type: 'Lane B closed component catalogue',
      b_value: 'none of those five exist in Lane B',
    },
    lossy: false,
    lossy_dimensions: [],
    failure_mode: 'refusal',
    failure_code: 'UNREGISTERED_COMPONENT',
    failure_reason: 'component type is not in closed catalog',
    observed: 'Five of Lane A\'s ten node types have no Lane B counterpart at all.',
    closure_ref: 'closure §6 — A → B vocabulary mismatch (measured refusal)',
    owner: 'Lane B — Wave 3 canonical map',
    evidence: 'A->B: five Lane A node types do not exist in the Lane B catalogue',
  },
  {
    id: 'A->B/node-id-alphabet',
    seam: 'A->B',
    a_source: 'src/visual-runtime/document/validate.ts:24',
    b_or_d_source: 'src/visual-runtime/components/catalog.ts:27',
    a_input_shape: 'node.id: /^[A-Za-z][A-Za-z0-9._:-]{0,95}$/ — dots, colons and 96 characters are legal',
    b_or_d_expected_shape: 'node.id: /^[A-Za-z0-9_-]{1,64}$/ — no dot, no colon, 64 characters max',
    mismatch: {
      field: 'node.id',
      a_type: 'string, Lane A alphabet',
      a_value: "'node.1' or 'node:1' or a 70-character id",
      b_type: 'string, Lane B alphabet',
      b_value: "'node.1' contains '.', which Lane B's REF rejects",
    },
    lossy: false,
    lossy_dimensions: [],
    failure_mode: 'refusal',
    failure_code: 'INVALID_COMPONENT',
    failure_reason: 'invalid opaque node ID',
    observed:
      "The same restriction applies to action reference strings: 'fixture.preview' is legal in Lane A " +
      "(validate.ts:203) and legal for Lane D (dom.ts:43) but illegal in Lane B.",
    closure_ref: 'closure §6 — A → B vocabulary mismatch (measured refusal)',
    owner: 'Lane B — Wave 3 canonical map',
    evidence: 'A->B: Lane B rejects identifier shapes that Lane A accepts',
  },
  {
    id: 'A->B/binding-ref-count',
    seam: 'A->B',
    a_source: 'src/visual-runtime/document/validate.ts:194',
    b_or_d_source: 'src/visual-runtime/components/catalog.ts:130',
    a_input_shape: 'bindingRefs: string[] of at most L.bindings = 128 entries per node',
    b_or_d_expected_shape: 'bindingRefs: at most 4 entries (refs(input.bindingRefs, 4))',
    mismatch: {
      field: 'bindingRefs.length',
      a_type: 'number, 0..128',
      a_value: '5',
      b_type: 'number, 0..4',
      b_value: '4',
    },
    lossy: false,
    lossy_dimensions: [],
    failure_mode: 'refusal',
    failure_code: 'INVALID_COMPONENT',
    failure_reason: 'invalid binding/action references',
    observed: 'A node with 5 valid Lane A binding references is refused wholesale by Lane B.',
    closure_ref: 'closure §6 — A → B vocabulary mismatch (measured refusal)',
    owner: 'Lane B — Wave 3 canonical map',
    evidence: 'A->B: Lane B rejects identifier shapes that Lane A accepts',
  },
  {
    id: 'A->B/binding-payload-has-no-channel',
    seam: 'A->B',
    a_source: 'src/visual-runtime/document/validate.ts:234',
    b_or_d_source: 'src/visual-runtime/components/catalog.ts:18',
    a_input_shape:
      'document.bindings[] carries { knowledgeRef, provenanceRef, readScope:"owner", availability, field, version }',
    b_or_d_expected_shape:
      'InspectedComponent exposes only bindingRefs: readonly string[] — there is no field for a binding payload',
    mismatch: {
      field: 'bindings[].provenanceRef / knowledgeRef / readScope',
      a_type: 'string ids + literal "owner" + availability enum',
      a_value: "'fixture-evidence:kn_8f3a0c71', 'kn_8f3a0c71', 'owner', 'present'",
      b_type: 'absent from the B contract',
      b_value: 'no representation',
    },
    lossy: true,
    lossy_dimensions: ['provenance'],
    failure_mode: 'silent-drop',
    failure_code: null,
    failure_reason: null,
    observed:
      'Lane B receives a NODE, not a document. The binding payload that carries provenance never enters ' +
      'Lane B, and InspectedComponent has no slot for it. A consumer that routes Lane A nodes to Lane B has ' +
      'no provenance channel at all — the binding ids survive, the provenance does not.',
    closure_ref: 'closure §6 — A → B is a vocabulary mismatch; this provenance gap is NOT named in §6 and is reported here as new',
    owner: 'Lane B — Wave 3 canonical map',
    evidence: 'A->B: the Lane B contract has no channel for binding provenance',
  },

  // -------------------------------------------------------------------------
  // A -> D  (Lane A document  ->  Lane D renderer)
  // -------------------------------------------------------------------------
  {
    id: 'A->D/action-ref-shape',
    seam: 'A->D',
    a_source: 'src/visual-runtime/document/validate.ts:202',
    b_or_d_source: 'src/visual-runtime/render/dom.ts:164',
    a_input_shape: 'actionRefs: [{ id, version }] — object shaped',
    b_or_d_expected_shape: 'actionRefs: inert string list only (isReferenceList -> every element must match the Lane D id regex)',
    mismatch: {
      field: 'node.actionRefs[] element',
      a_type: 'object { id, version }',
      a_value: "{ id: 'fixture.preview', version: 1 }",
      b_type: 'string',
      b_value: "'fixture.preview'",
    },
    lossy: false,
    lossy_dimensions: [],
    failure_mode: 'refusal',
    failure_code: 'INVALID_REFERENCE',
    failure_reason: 'The whole render plan is unavailable; the root is a read-only placeholder.',
    observed:
      'The raw Lane A document renders `unavailable` with issue INVALID_REFERENCE. The failure is fail-closed ' +
      'for the whole document, not a per-node degradation.',
    closure_ref: 'closure §6 — A → D reference-shape mismatch (bridged by the integrator adapter)',
    owner: 'Lanes A and D (shape difference); integrator owns the adapter',
    evidence: 'A->D: the raw Lane A document renders unavailable with INVALID_REFERENCE',
  },
  {
    id: 'A->D/canonical-version-ref-dropped-from-renderer-document',
    seam: 'A->D',
    a_source: 'src/visual-runtime/document/validate.ts:203',
    b_or_d_source: 'src/visual-runtime/integration-2/action-adapter.mjs:19',
    a_input_shape: "actionRefs: [{ id: 'fixture.preview', version: 1 }]",
    b_or_d_expected_shape:
      "Lane D consumes inert string references only. The committed adapter rewrites them to 'synthetic_ref_<i>_<j>' " +
      'and keeps { id, version, inert: true } in a SEPARATE read-only table.',
    mismatch: {
      field: 'rendererDocument.nodes[].actionRefs[]',
      a_type: 'object { id, version }',
      a_value: "{ id: 'fixture.preview', version: 1 }",
      b_type: 'string alias',
      b_value: "'synthetic_ref_4_0'",
    },
    lossy: true,
    lossy_dimensions: ['canonical-version-ref'],
    failure_mode: 'silent-drop',
    failure_code: null,
    failure_reason: null,
    observed:
      'After the adapter, the renderer document contains only the synthetic alias. The canonical ' +
      "{ id: 'fixture.preview', version: 1 } survives ONLY in actionVersionTable (adapter line 19), outside the " +
      'document Lane D actually renders. A consumer that reads only the render plan cannot recover the version. ' +
      'The adapter adds no execute permission and the table is marked inert: true.',
    closure_ref: 'closure §6 — "it adds no execute permission, and it preserves the canonical version reference in a separate read-only table (fixture.preview@1, inert: true)"',
    owner: 'integrator (adapter); the underlying A/D shape difference is owned by Lanes A and D',
    evidence: 'A->D: the canonical version reference survives only in the adapter side table',
  },
  {
    id: 'A->D/binding-provenance-dropped',
    seam: 'A->D',
    a_source: 'src/visual-runtime/document/validate.ts:234',
    b_or_d_source: 'src/visual-runtime/render/dom.ts:28',
    a_input_shape: 'document.bindings[] carries knowledgeRef, provenanceRef, readScope, availability',
    b_or_d_expected_shape:
      'RenderPlan (dom.ts:28-39) has no bindings field; project() reads only type, props, a11y, children.',
    mismatch: {
      field: 'document.bindings',
      a_type: 'array of binding records',
      a_value: "[{ knowledgeRef:'kn_8f3a0c71', provenanceRef:'fixture-evidence:kn_8f3a0c71', readScope:'owner' }]",
      b_type: 'not read by the renderer at all',
      b_value: 'no representation',
    },
    lossy: true,
    lossy_dimensions: ['provenance'],
    failure_mode: 'silent-drop',
    failure_code: null,
    failure_reason: null,
    observed:
      'The render plan is byte-for-byte free of the binding provenance: no knowledgeRef, no provenanceRef, ' +
      'no readScope, no availability and not even the binding id. A node that Lane A bound to an owner-scoped ' +
      'knowledge record renders as ordinary content with no provenance marker and no issue code.',
    closure_ref: 'closure §6 does not name this loss; reported here as new',
    owner: 'Lane D (renderer contract)',
    evidence: 'A->D: the render plan carries no binding provenance',
  },
  {
    id: 'A->D/card-description-not-projected',
    seam: 'A->D',
    a_source: 'src/visual-runtime/document/validate.ts:72',
    b_or_d_source: 'src/visual-runtime/render/dom.ts:122',
    a_input_shape: "card props: { title, description } — both are legitimate Lane A payload",
    b_or_d_expected_shape:
      "Lane D ALLOWS description in acceptableProps (dom.ts:114) but safeContent() reads only props.title " +
      "(dom.ts:123). description is never projected and never flagged.",
    mismatch: {
      field: 'props.description',
      a_type: 'string',
      a_value: "'Synthetic project note'",
      b_type: 'accepted by the allow-list, unread by the projector',
      b_value: 'silently discarded',
    },
    lossy: true,
    lossy_dimensions: ['content'],
    failure_mode: 'silent-drop',
    failure_code: null,
    failure_reason: null,
    observed:
      'The rendered card carries the title and no aria-description. The description text is accepted, dropped, ' +
      'and reported as outcome "rendered" with code "OK".',
    closure_ref: 'closure §6 does not name this loss; reported here as new',
    owner: 'Lane D (renderer contract)',
    evidence: 'A->D: Lane D accepts the card description prop and never projects it',
  },
  {
    id: 'A->D/semantic-focus-order-replaced',
    seam: 'A->D',
    a_source: 'src/visual-runtime/document/schema.ts:63',
    b_or_d_source: 'src/visual-runtime/render/dom.ts:216',
    a_input_shape: "a11y.focusOrder: authored semantic order (schema.ts:63 'Focus order is semantic, not inferred from geometry')",
    b_or_d_expected_shape:
      'Lane D ignores a11y.focusOrder and derives focusOrder from tree traversal plus button-ness (dom.ts:211-217).',
    mismatch: {
      field: 'a11y.focusOrder',
      a_type: 'authored number',
      a_value: '1',
      b_type: 'derived array of button ids',
      b_value: "['action-button']",
    },
    lossy: true,
    lossy_dimensions: ['semantic-focus-order'],
    failure_mode: 'degrade',
    failure_code: null,
    failure_reason: null,
    observed:
      'The authored order is replaced by the renderer\'s own traversal order, with no issue code and no marker ' +
      'that the authored value was discarded.',
    closure_ref: 'closure §6 does not name this loss; reported here as new',
    owner: 'Lane D (renderer contract)',
    evidence: 'A->D: Lane D replaces the authored semantic focus order with its own traversal order',
  },
  {
    id: 'A->D/a11y-role-remapped',
    seam: 'A->D',
    a_source: 'src/visual-runtime/document/validate.ts:182',
    b_or_d_source: 'src/visual-runtime/render/dom.ts:209',
    a_input_shape: "a11y.role = 'group' on a card",
    b_or_d_expected_shape: "Lane D maps any non-'note' role to 'region' (dom.ts:209)",
    mismatch: {
      field: 'a11y.role',
      a_type: "'group'",
      a_value: "'group'",
      b_type: 'role string in the render plan',
      b_value: "'region'",
    },
    lossy: true,
    lossy_dimensions: ['semantic-role'],
    failure_mode: 'degrade',
    failure_code: null,
    failure_reason: null,
    observed: 'The rendered card carries role="region" although Lane A authored role="group".',
    closure_ref: 'closure §6 does not name this loss; reported here as new',
    owner: 'Lane D (renderer contract)',
    evidence: 'A->D: Lane D remaps the Lane A accessibility role',
  },
  {
    id: 'A->D/no-permission-added',
    seam: 'A->D',
    a_source: 'src/visual-runtime/document/validate.ts:198',
    b_or_d_source: 'src/visual-runtime/render/dom.ts:211',
    a_input_shape: 'A button carrying one action bound to the trusted action catalogue',
    b_or_d_expected_shape:
      'Lane D renders the button with aria-disabled="true", tabindex="0" and the description ' +
      '"Read-only preview. No action can be performed here." and trace code INERT_ACTION. It adds no handler, ' +
      'no URL and no grant.',
    mismatch: {
      field: 'authority surface',
      a_type: 'no authority beyond the inert catalogue reference',
      a_value: 'actionRefs[{id,version}]',
      b_type: 'inert description only',
      b_value: 'aria-disabled / tabindex / INERT_ACTION, no dispatch path',
    },
    lossy: false,
    lossy_dimensions: [],
    failure_mode: 'safe',
    failure_code: null,
    failure_reason: null,
    observed:
      'NO PERMISSION IS ADDED at this seam. The button stays focusable for announcement but is explicitly ' +
      'disabled and the render plan contains no execute or dispatch surface. This is a safety observation.',
    closure_ref: 'closure §6 — "it adds no execute permission"',
    owner: 'integrator (adapter); Lane D',
    evidence: 'A->D: the seam adds no permission and no effect path',
  },
];

/** Seams that lose something. */
export const LOSSY_SEAMS = SEAMS.filter((seam) => seam.lossy);

/** Seams that grant authority. Measured: none. */
export const PERMISSION_ADDING_SEAMS = SEAMS.filter((seam) => seam.lossy_dimensions.includes('permission-added'));

/** Refusal seams (fail-closed) vs silent drops (fail-open). */
export const REFUSAL_SEAMS = SEAMS.filter((seam) => seam.failure_mode === 'refusal');
export const SILENT_DROP_SEAMS = SEAMS.filter((seam) => seam.failure_mode === 'silent-drop');

/** Compact table for README rendering. */
export function seamTable(seams = SEAMS) {
  return seams.map((seam) => ({
    id: seam.id,
    seam: seam.seam,
    mismatch: `${seam.mismatch.field}: A ${seam.mismatch.a_type} -> B/D ${seam.mismatch.b_type}`,
    lossy: seam.lossy ? seam.lossy_dimensions.join('+') : 'no',
    mode: seam.failure_mode,
    code: seam.failure_code ?? '-',
    a_source: seam.a_source,
    b_or_d_source: seam.b_or_d_source,
  }));
}
