/**
 * Synthetic knowledge provenance / revocation fixtures.
 *
 * THESE ARE SYNTHETIC FIXTURES. They are not owner knowledge, not a KB export,
 * not a mirror of any private corpus, and not derived from any live Knowledge
 * Base. Every row below is invented here and describes a *state* the real
 * TierFlow projector must handle. Nothing in this file was read from a live
 * system, and no fixture may be presented as live-KB evidence.
 *
 * SOURCE PIN
 *   TierFlow checkout `C:/0-BlackBoxProject-0/vivim-ui`, branch `main`,
 *   commit ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7.
 *
 * MODULES UNDER TEST (read-only, imported live by provenance.test.mjs)
 *   src/visual-runtime/knowledge/client.ts     — Lane F, the read-only client
 *   src/visual-runtime/knowledge/projection.ts  — Lane F, the read-only projector
 *
 * WHERE EACH EXPECTATION COMES FROM
 *   Every `expectation_source` is a file:line in the pinned TierFlow checkout.
 *   Where the source does NOT state an expectation, the entry says so and the
 *   behaviour is characterised as observed, not as sanctioned.
 */

/** @type {{repo: string, local_path: string, branch: string, commit: string, closure_result: string, pinned_on: string}} */
export const TIERFLOW_SOURCE = Object.freeze({
  repo: 'owenservera/tierflow',
  local_path: 'C:/0-BlackBoxProject-0/vivim-ui',
  branch: 'main',
  commit: 'ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7',
  closure_result: 'VIVIM-VISUAL-RUNTIME/WAVE-2/INTEGRATION-2-CLOSURE-RESULT.md',
  pinned_on: '2026-11-06',
});

export const KNOWLEDGE_MODULES = Object.freeze({
  client: 'src/visual-runtime/knowledge/client.ts',
  projection: 'src/visual-runtime/knowledge/projection.ts',
});

/** Deterministic clock for every fixture run. No wall clock, no network. */
export const FIXED_NOW_MS = 1_792_000_000_000;

export const OWNER_REF = 'owner_a';
export const WORLD_REF = 'world_a';
export const DEVICE_REF = 'desktop_1';
export const SESSION_REF = 'session_1';

/** Strings that must NEVER appear in any page or projection for the hidden states. */
export const SECRET_MARKERS = Object.freeze({
  redacted: 'SECRET_BODY_REDACTED',
  tombstoned: 'SECRET_BODY_TOMBSTONED',
  revokedScope: 'SECRET_BODY_REVOKED',
  deviceScoped: 'SECRET_BODY_DEVICE_SCOPED',
  purposeScoped: 'SECRET_BODY_PURPOSE_SCOPED',
});

const source = (nativeId, nativeVersion) => ({
  authority: 'fixture.hq',
  nativeId,
  nativeVersion,
  digest: `sha256:synthetic-${nativeId}`,
});

/**
 * The synthetic rows. Shape matches `CanonicalKnowledgeRecord`
 * (knowledge/client.ts:45-64) exactly; no field is invented.
 */
export const KNOWLEDGE_FIXTURES = Object.freeze([
  {
    stableId: 'kn_fresh_0001',
    ownerRef: OWNER_REF,
    personalWorldRef: WORLD_REF,
    projectRef: 'project_one',
    kind: 'project_note',
    label: 'Fresh synthetic note',
    summary: 'Synthetic summary of a current owner-scoped note.',
    path: 'src/README.md',
    sensitivity: 'S0',
    availability: 'present',
    claimStatus: 'observed',
    source: source('kn_fresh_0001', 'rev-3'),
    evidenceRefs: ['fixture-evidence:kn_fresh_0001'],
    provenance: ['synthetic.corpus/1'],
    asOf: '2026-11-06T12:00:00.000Z',
  },
  {
    // STALE: the row is one canonical version behind (source.nativeVersion
    // rev-2 while the corpus's current version is rev-3).
    stableId: 'kn_stale_0002',
    ownerRef: OWNER_REF,
    personalWorldRef: WORLD_REF,
    projectRef: 'project_one',
    kind: 'project_note',
    label: 'Stale synthetic note',
    summary: 'Synthetic summary of a note whose canonical version is behind.',
    path: 'src/README.md',
    sensitivity: 'S1',
    availability: 'stale',
    claimStatus: 'observed',
    source: source('kn_stale_0002', 'rev-2'),
    evidenceRefs: ['fixture-evidence:kn_stale_0002'],
    provenance: ['synthetic.corpus/1'],
    asOf: '2026-11-04T12:00:00.000Z',
  },
  {
    // REDACTED: existence is itself private unless an explicit trusted policy
    // allows existence-only markers.
    stableId: 'kn_redacted_0003',
    ownerRef: OWNER_REF,
    personalWorldRef: WORLD_REF,
    projectRef: 'project_one',
    kind: 'project_note',
    label: SECRET_MARKERS.redacted,
    summary: 'Synthetic summary of a redacted note.',
    path: 'src/README.md',
    sensitivity: 'S1',
    availability: 'redacted',
    claimStatus: 'observed',
    source: source('kn_redacted_0003', 'rev-3'),
    evidenceRefs: ['fixture-evidence:kn_redacted_0003'],
    provenance: ['synthetic.corpus/1'],
    asOf: '2026-11-06T12:00:00.000Z',
  },
  {
    // UNAVAILABLE: the source cannot be read right now.
    stableId: 'kn_unavail_0004',
    ownerRef: OWNER_REF,
    personalWorldRef: WORLD_REF,
    projectRef: 'project_one',
    kind: 'project_note',
    label: 'Unavailable synthetic note',
    summary: 'Synthetic summary of a note the source cannot currently serve.',
    path: 'src/README.md',
    sensitivity: 'S0',
    availability: 'unavailable',
    claimStatus: 'observed',
    source: source('kn_unavail_0004', 'rev-3'),
    evidenceRefs: ['fixture-evidence:kn_unavail_0004'],
    provenance: ['synthetic.corpus/1'],
    asOf: '2026-11-06T12:00:00.000Z',
  },
  {
    // TOMBSTONED / DELETED: the source tombstoned the record. It must never be
    // resurrected through search, alias, cursor or cache.
    stableId: 'kn_tombstone_0005',
    ownerRef: OWNER_REF,
    personalWorldRef: WORLD_REF,
    projectRef: 'project_one',
    kind: 'project_note',
    label: SECRET_MARKERS.tombstoned,
    summary: 'Synthetic summary of a tombstoned note.',
    path: 'src/README.md',
    sensitivity: 'S0',
    availability: 'deleted',
    claimStatus: 'retracted',
    source: source('kn_tombstone_0005', 'rev-3'),
    evidenceRefs: ['fixture-evidence:kn_tombstone_0005'],
    provenance: ['synthetic.corpus/1'],
    asOf: '2026-11-06T12:00:00.000Z',
  },
  {
    // REVOKED SCOPE: the record's own access scope was revoked.
    stableId: 'kn_revoked_0006',
    ownerRef: OWNER_REF,
    personalWorldRef: WORLD_REF,
    projectRef: 'project_one',
    kind: 'project_note',
    label: SECRET_MARKERS.revokedScope,
    summary: 'Synthetic summary of a note whose scope was revoked.',
    path: 'src/README.md',
    sensitivity: 'S1',
    availability: 'revoked',
    claimStatus: 'retracted',
    source: source('kn_revoked_0006', 'rev-3'),
    evidenceRefs: ['fixture-evidence:kn_revoked_0006'],
    provenance: ['synthetic.corpus/1'],
    asOf: '2026-11-06T12:00:00.000Z',
  },
  {
    // MISSING PROVENANCE: the row is readable but carries no provenance and no
    // evidence references.
    stableId: 'kn_noprov_0007',
    ownerRef: OWNER_REF,
    personalWorldRef: WORLD_REF,
    projectRef: 'project_one',
    kind: 'project_note',
    label: 'Synthetic note without provenance',
    summary: 'Synthetic summary of a note whose provenance is absent.',
    path: 'src/README.md',
    sensitivity: 'S0',
    availability: 'present',
    claimStatus: 'observed',
    source: source('kn_noprov_0007', 'rev-3'),
    evidenceRefs: [],
    provenance: [],
    asOf: '2026-11-06T12:00:00.000Z',
  },
  {
    // UNKNOWN: the source cannot say whether it is current.
    stableId: 'kn_unknown_0008',
    ownerRef: OWNER_REF,
    personalWorldRef: WORLD_REF,
    projectRef: 'project_one',
    kind: 'project_note',
    label: 'Unknown synthetic note',
    summary: 'Synthetic summary of a note whose freshness is unknown.',
    path: 'src/README.md',
    sensitivity: 'S0',
    availability: 'unknown',
    claimStatus: 'observed',
    source: source('kn_unknown_0008', 'rev-3'),
    evidenceRefs: ['fixture-evidence:kn_unknown_0008'],
    provenance: ['synthetic.corpus/1'],
    asOf: '2026-11-06T12:00:00.000Z',
  },
  {
    // ROW-LEVEL SCOPE: readable by another device only.
    stableId: 'kn_devscope_0009',
    ownerRef: OWNER_REF,
    personalWorldRef: WORLD_REF,
    projectRef: 'project_one',
    kind: 'project_note',
    label: SECRET_MARKERS.deviceScoped,
    summary: 'Synthetic summary of a note bound to another device.',
    path: 'src/README.md',
    sensitivity: 'S0',
    availability: 'present',
    claimStatus: 'observed',
    source: source('kn_devscope_0009', 'rev-3'),
    evidenceRefs: ['fixture-evidence:kn_devscope_0009'],
    provenance: ['synthetic.corpus/1'],
    asOf: '2026-11-06T12:00:00.000Z',
    allowedDeviceRefs: ['desktop_other'],
  },
]);

/** Trusted context variants. The client only ever sees a context from a broker. */
export function trustedContext(overrides = {}) {
  return {
    ownerRef: OWNER_REF,
    sessionRef: SESSION_REF,
    deviceRef: DEVICE_REF,
    personalWorldRef: WORLD_REF,
    surface: 'pc',
    allowedPurposes: ['task', 'explain', 'help'],
    allowPhoneS1: false,
    policyRevision: 'policy_1',
    sessionExpiresAt: FIXED_NOW_MS + 60_000,
    grantExpiresAt: FIXED_NOW_MS + 60_000,
    revoked: false,
    offline: false,
    ...overrides,
  };
}

/** A broker whose context can be replaced, modelling a separate trusted layer. */
export function mutableBroker(initial = trustedContext()) {
  let current = initial;
  return {
    broker: { readContext: () => current },
    setContext(next) {
      current = next;
    },
  };
}

/** A valid query. `entityRef` alone satisfies the client's query-shape rule. */
export function queryFor(stableId, overrides = {}) {
  return {
    schema: 'vivim.knowledge-query/1',
    requestId: 'req_knowledge_fixture',
    personalWorldRef: WORLD_REF,
    purpose: 'task',
    surface: 'pc',
    limit: 10,
    entityRef: stableId,
    ...overrides,
  };
}

/**
 * The state matrix. `expected` states what the pinned source says SHOULD
 * happen; `expectation_source` says where that comes from; `observed` is filled
 * in by the measured run and is reported in README.md.
 *
 * `expectation_source` values ending in "(observed)" mean the pinned source
 * does NOT state an expectation for that behaviour; it is characterised, not
 * sanctioned.
 */
export const FIXTURE_STATES = Object.freeze([
  {
    state: 'fresh',
    stableId: 'kn_fresh_0001',
    kind: 'fresh entity',
    expected: {
      visible: true,
      item_availability: 'present',
      item_visibility: 'visible',
      page_status: 'ok',
      page_completeness: 'complete',
      projection_state: 'ready',
      projection_cards: 1,
      provenance_label: 'synthetic.corpus/1',
      source_version: 'rev-3',
      body_visible: true,
    },
    expectation_source: 'knowledge/client.ts:153 (deleted/revoked/missing are filtered), :155-171 (toItem), knowledge/projection.ts:26-32 (makeCard)',
  },
  {
    state: 'stale',
    stableId: 'kn_stale_0002',
    kind: 'stale entity (canonical version behind: source.nativeVersion rev-2 vs rev-3)',
    expected: {
      visible: true,
      item_availability: 'stale',
      item_visibility: 'visible',
      page_status: 'partial',
      page_completeness: 'partial',
      projection_state: 'partial',
      projection_cards: 1,
      status_label: 'Knowledge stale — confirm source before relying on it',
      provenance_label: 'synthetic.corpus/1',
      body_visible: true,
    },
    expectation_source: 'knowledge/client.ts:222 (stale|unknown|unavailable make the page partial), knowledge/projection.ts:29 (statusLabel)',
  },
  {
    state: 'redacted',
    stableId: 'kn_redacted_0003',
    kind: 'redacted entity',
    expected: {
      visible_strict: false,
      strict_items: 0,
      strict_page_status: 'ok',
      visible_permissive: true,
      permissive_item_visibility: 'redacted',
      permissive_item_label: 'Redacted',
      permissive_item_stable_id: 'withheld',
      permissive_item_summary: '',
      permissive_item_provenance: [],
      permissive_item_as_of: '',
      permissive_item_source: 'withheld',
      permissive_page_status: 'ok',
      projection_cards_permissive: 0,
      body_visible: false,
      marker_must_not_leak: SECRET_MARKERS.redacted,
    },
    expectation_source:
      'knowledge/client.ts:151 (redacted existence is private unless an explicit trusted policy allows markers), :156-170 (withheld fields), knowledge/projection.ts:26 (a non-visible item yields no card)',
  },
  {
    state: 'unavailable',
    stableId: 'kn_unavail_0004',
    kind: 'unavailable entity',
    expected: {
      visible: true,
      item_availability: 'unavailable',
      item_visibility: 'visible',
      page_status: 'partial',
      page_completeness: 'partial',
      projection_state: 'partial',
      projection_cards: 1,
      status_label: 'Knowledge unavailable — confirm source before relying on it',
      body_visible: true,
    },
    expectation_source: 'knowledge/client.ts:222 (unavailable makes the page partial), knowledge/projection.ts:29 (statusLabel)',
  },
  {
    state: 'tombstoned',
    stableId: 'kn_tombstone_0005',
    kind: 'tombstoned / deleted entity',
    expected: {
      visible: false,
      items: 0,
      never_resurrected: true,
      body_visible: false,
      marker_must_not_leak: SECRET_MARKERS.tombstoned,
    },
    expectation_source: 'knowledge/client.ts:153 (deleted is not permitted), :152 (no resurrection through search, alias, cursor or cache), :229-244 (tombstone source event)',
  },
  {
    state: 'revoked_scope',
    stableId: 'kn_revoked_0006',
    kind: 'revoked-scope entity (record scope revoked)',
    expected: {
      visible: false,
      items: 0,
      never_resurrected: true,
      body_visible: false,
      marker_must_not_leak: SECRET_MARKERS.revokedScope,
    },
    expectation_source: 'knowledge/client.ts:153 (revoked is not permitted), :152 (no resurrection through search, alias, cursor or cache)',
  },
  {
    state: 'missing_provenance',
    stableId: 'kn_noprov_0007',
    kind: 'missing-provenance entity (readable, but no provenance and no evidence refs)',
    expected: {
      visible: true,
      item_availability: 'present',
      item_provenance: [],
      item_evidence_refs: [],
      page_status: 'ok',
      projection_state: 'ready',
      projection_cards: 1,
      provenance_label: '',
      body_visible: true,
    },
    expectation_source:
      'knowledge/projection.ts:30 (provenanceLabel = item.provenance.join(", "), so an empty provenance array yields an EMPTY label) — the source states no refusal or marker for this case (observed)',
  },
  {
    state: 'unknown',
    stableId: 'kn_unknown_0008',
    kind: 'unknown entity (freshness unknowable)',
    expected: {
      visible: true,
      item_availability: 'unknown',
      page_status: 'partial',
      projection_state: 'partial',
      projection_cards: 1,
      status_label: 'Knowledge unknown — confirm source before relying on it',
      body_visible: true,
    },
    expectation_source: 'knowledge/client.ts:222 (unknown makes the page partial), knowledge/projection.ts:29 (statusLabel)',
  },
  {
    state: 'device_scoped_row',
    stableId: 'kn_devscope_0009',
    kind: 'row-level scope exclusion (another device only)',
    expected: {
      visible: false,
      items: 0,
      body_visible: false,
      marker_must_not_leak: SECRET_MARKERS.deviceScoped,
      note: 'Row-level exclusion is silent: the page reports complete with zero items.',
    },
    expectation_source: 'knowledge/client.ts:145 (allowedDeviceRefs excludes the current device) — the source states no denial for a row-level exclusion (observed)',
  },
]);

/** Session / grant level revocations. These are CONTEXT states, not row states. */
export const CONTEXT_STATES = Object.freeze([
  {
    state: 'revoked_session',
    overrides: { revoked: true },
    expected: { page_status: 'denied', reason_code: 'SCOPE_DENIED', items: 0, identifiers_leaked: false },
    expectation_source: 'knowledge/client.ts:192 (contextValid fails on ctx.revoked -> denied SCOPE_DENIED), :111-114 (the denial carries no identifiers, counts, timestamps, cursor or policy revision)',
  },
  {
    state: 'expired_session',
    overrides: { sessionExpiresAt: FIXED_NOW_MS - 1 },
    expected: { page_status: 'denied', reason_code: 'SCOPE_DENIED', items: 0, identifiers_leaked: false },
    expectation_source: 'knowledge/client.ts:138 (sessionExpiresAt > now is required), :192',
  },
  {
    state: 'expired_grant',
    overrides: { grantExpiresAt: FIXED_NOW_MS - 1 },
    expected: { page_status: 'denied', reason_code: 'SCOPE_DENIED', items: 0, identifiers_leaked: false },
    expectation_source: 'knowledge/client.ts:138 (grantExpiresAt > now is required), :192',
  },
  {
    state: 'offline',
    overrides: { offline: true },
    expected: { page_status: 'unavailable', reason_code: 'OFFLINE', items: 0, identifiers_leaked: false },
    expectation_source: 'knowledge/client.ts:195 (no possibly revoked cached payload is replayed offline), :115-117',
  },
  {
    state: 'no_trusted_context',
    overrides: null,
    expected: { page_status: 'denied', reason_code: 'NO_TRUSTED_CONTEXT', items: 0, identifiers_leaked: false },
    expectation_source: 'knowledge/client.ts:191 (a missing broker context denies before anything else)',
  },
]);

/** Request-level scope mismatches. These must DENY, not degrade to an empty page. */
export const REQUEST_SCOPE_STATES = Object.freeze([
  {
    state: 'foreign_personal_world',
    query: { personalWorldRef: 'world_b' },
    expected: { page_status: 'denied', reason_code: 'SCOPE_DENIED', items: 0 },
    expectation_source: 'knowledge/client.ts:194 (request.personalWorldRef !== ctx.personalWorldRef -> denied SCOPE_DENIED), :188 (trust and scope are resolved BEFORE searching, filtering, ranking, counting or issuing cursors)',
  },
  {
    state: 'foreign_surface',
    query: { surface: 'android' },
    expected: { page_status: 'denied', reason_code: 'SCOPE_DENIED', items: 0 },
    expectation_source: 'knowledge/client.ts:194 (request.surface !== ctx.surface -> denied SCOPE_DENIED)',
  },
  {
    state: 'purpose_not_allowed',
    query: { purpose: 'help' },
    context_overrides: { allowedPurposes: ['task'] },
    expected: { page_status: 'denied', reason_code: 'SCOPE_DENIED', items: 0 },
    expectation_source: 'knowledge/client.ts:194 (!ctx.allowedPurposes.includes(request.purpose) -> denied SCOPE_DENIED)',
  },
]);
