/**
 * Knowledge provenance / revocation — REAL Lane F client + REAL Lane F projector.
 *
 * WHAT THIS FILE IS
 *   A characterisation of how the pinned TierFlow knowledge client
 *   (`src/visual-runtime/knowledge/client.ts`) and its read-only projector
 *   (`src/visual-runtime/knowledge/projection.ts`) behave for each synthetic
 *   knowledge state: fresh, stale, redacted, unavailable, tombstoned/deleted,
 *   revoked-scope and missing-provenance.
 *
 *   It CHARACTERISES CURRENT BEHAVIOUR. Where the current behaviour loses
 *   information, the loss is named and backed by a FAILING-BY-DESIGN negative
 *   test that would go red if TierFlow were repaired. Nothing in TierFlow is
 *   modified to produce any of these observations.
 *
 * SYNTHETIC ONLY
 *   The fixtures in `./fixtures.mjs` are invented here. No live KB, no owner
 *   knowledge, no private corpus, no FTS index and no provider was contacted.
 *   The evidence level of this file is `unit verified` against a synthetic
 *   fixture corpus — NOT `integrated` with a live KB and NOT environment or
 *   production verified.
 *
 * SOURCE PIN
 *   TierFlow checkout `C:/0-BlackBoxProject-0/vivim-ui`, branch `main`,
 *   commit ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7, asserted at runtime
 *   against the on-disk .git metadata.
 *
 * LEAK-NEGATIVE MUTATION HOOK
 *   `VIVIM_LABS_KNOWLEDGE_FIXTURE_MODULE` points the suite at an alternative
 *   fixtures module. The mutation proof uses it to load a scratch copy whose
 *   redacted/tombstoned rows have been flipped to `present`, which MUST make
 *   the leak-negative test fail. The proof is: run with the override, observe
 *   the failure, restore, observe the pass.
 *
 * RUN
 *   cd C:/0-BlackBoxProject-0/vivim-labs && bun test corpora/knowledge/provenance.test.mjs
 */
import { test, expect } from 'bun:test';
import { existsSync, readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const HERE = dirname(fileURLToPath(import.meta.url));
const TIERFLOW_ROOT = process.env.VIVIM_LABS_TIERFLOW_ROOT
  ? resolve(process.env.VIVIM_LABS_TIERFLOW_ROOT)
  : resolve(HERE, '../../..', 'vivim-ui');

const FIXTURE_MODULE = process.env.VIVIM_LABS_KNOWLEDGE_FIXTURE_MODULE
  ? resolve(process.env.VIVIM_LABS_KNOWLEDGE_FIXTURE_MODULE)
  : fileURLToPath(new URL('./fixtures.mjs', import.meta.url));

const FIX = await import(FIXTURE_MODULE);
const { createFixtureKnowledgeClient } = await import(resolve(TIERFLOW_ROOT, FIX.KNOWLEDGE_MODULES.client));
const { projectKnowledgePage } = await import(resolve(TIERFLOW_ROOT, FIX.KNOWLEDGE_MODULES.projection));

const MUTATED = Boolean(process.env.VIVIM_LABS_KNOWLEDGE_FIXTURE_MODULE);

/** Build a real client over the synthetic rows with a chosen trusted context. */
function clientWith(contextOverrides, rows = FIX.KNOWLEDGE_FIXTURES) {
  const holder = FIX.mutableBroker(
    contextOverrides === null ? null : FIX.trustedContext(contextOverrides),
  );
  const client = createFixtureKnowledgeClient(rows, holder.broker, { now: () => FIX.FIXED_NOW_MS });
  return { client, holder };
}

async function readOne(stableId, contextOverrides = {}, rows = FIX.KNOWLEDGE_FIXTURES) {
  const { client } = clientWith(contextOverrides, rows);
  const page = await client.query(FIX.queryFor(stableId));
  return { page, projection: projectKnowledgePage(page) };
}

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

/**
 * FAILING-BY-DESIGN harness — same pattern as the differential corpus.
 * `probe()` returns { ok, expect, detail } where `ok` says whether the CORRECT
 * behaviour currently holds. The registered `test.skip` is the real negative
 * test; the companion PROOF test keeps the defect observable in a green suite.
 */
function failingByDesign(id, title, probe) {
  test.skip(`FAILING-BY-DESIGN ${id}: ${title}`, async () => {
    const result = await probe();
    expect(result.ok).toBe(true);
  });
  test(`FAILING-BY-DESIGN PROOF ${id}: ${title}`, async () => {
    const result = await probe();
    expect(result.ok).toBe(false);
    expect(result.detail).toContain('DEFECT PRESENT');
    expect(result.detail).toContain(result.expect);
  });
}

// ===========================================================================
// 0. Source pin and fixture integrity
// ===========================================================================

test('source pin: the TierFlow checkout under test is the pinned commit', () => {
  expect(FIX.TIERFLOW_SOURCE.commit).toBe('ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7');
  expect(readCheckedOutSha(TIERFLOW_ROOT)).toBe(FIX.TIERFLOW_SOURCE.commit);
});

test('source pin: both knowledge modules exist in the pinned checkout', () => {
  for (const relative of Object.values(FIX.KNOWLEDGE_MODULES)) {
    expect(existsSync(resolve(TIERFLOW_ROOT, relative))).toBe(true);
  }
});

test('fixtures: every state has a stable id, a unique id and a cited expectation', () => {
  expect(FIX.FIXTURE_STATES.length).toBeGreaterThan(0);
  const ids = FIX.FIXTURE_STATES.map((state) => state.stableId);
  expect(new Set(ids).size).toBe(ids.length);
  for (const state of FIX.FIXTURE_STATES) {
    expect(ids).toContain(state.stableId);
    expect(state.expectation_source.length).toBeGreaterThan(0);
    expect(state.kind.length).toBeGreaterThan(0);
  }
});

test('fixtures: every required knowledge state is covered', () => {
  const states = FIX.FIXTURE_STATES.map((state) => state.state);
  for (const required of ['fresh', 'stale', 'redacted', 'unavailable', 'tombstoned', 'revoked_scope', 'missing_provenance']) {
    expect(states).toContain(required);
  }
});

// ===========================================================================
// 1. Per-state projection behaviour
// ===========================================================================

test('state fresh: the entity is visible, complete and carries its provenance', async () => {
  const { page, projection } = await readOne('kn_fresh_0001');
  expect(page.status).toBe('ok');
  expect(page.completeness).toBe('complete');
  expect(page.items).toHaveLength(1);
  expect(page.items[0].visibility).toBe('visible');
  expect(page.items[0].availability).toBe('present');
  expect(page.items[0].provenance).toEqual(['synthetic.corpus/1']);
  expect(projection.state).toBe('ready');
  expect(projection.cards).toHaveLength(1);
  expect(projection.cards[0].provenanceLabel).toBe('synthetic.corpus/1');
  expect(projection.cards[0].sourceVersion).toBe('rev-3');
  expect(projection.cards[0].statusLabel).toBe('Observed (not an authorization)');
  expect(projection.cards[0].interactive).toBe(false);
});

test('state stale: the entity is visible but the page is partial', async () => {
  const { page, projection } = await readOne('kn_stale_0002');
  expect(page.status).toBe('partial');
  expect(page.completeness).toBe('partial');
  expect(page.items).toHaveLength(1);
  expect(page.items[0].availability).toBe('stale');
  expect(page.items[0].source.nativeVersion).toBe('rev-2');
  expect(projection.state).toBe('partial');
  expect(projection.cards).toHaveLength(1);
  expect(projection.cards[0].statusLabel).toBe('Knowledge stale — confirm source before relying on it');
  expect(projection.statusMessage).toContain('does not authorize actions');
});

test('state unavailable: the entity is visible but the page is partial', async () => {
  const { page, projection } = await readOne('kn_unavail_0004');
  expect(page.status).toBe('partial');
  expect(page.items[0].availability).toBe('unavailable');
  expect(projection.state).toBe('partial');
  expect(projection.cards[0].statusLabel).toBe('Knowledge unavailable — confirm source before relying on it');
});

test('state unknown: the entity is visible but the page is partial', async () => {
  const { page, projection } = await readOne('kn_unknown_0008');
  expect(page.status).toBe('partial');
  expect(page.items[0].availability).toBe('unknown');
  expect(projection.state).toBe('partial');
  expect(projection.cards[0].statusLabel).toBe('Knowledge unknown — confirm source before relying on it');
});

test('state redacted: existence is private under a strict trusted context', async () => {
  const { page, projection } = await readOne('kn_redacted_0003');
  expect(page.items).toHaveLength(0);
  // Characterisation (observed): the empty result is reported as a COMPLETE
  // page, not as a denial. The source states no denial for this case.
  expect(page.status).toBe('ok');
  expect(page.completeness).toBe('complete');
  expect(projection.state).toBe('ready');
  expect(projection.cards).toHaveLength(0);
});

test('state redacted: an explicit trusted policy yields an existence-only marker with no body', async () => {
  const { page, projection } = await readOne('kn_redacted_0003', { maySeeRedactedMarkers: true });
  expect(page.items).toHaveLength(1);
  const item = page.items[0];
  expect(item.visibility).toBe('redacted');
  expect(item.stableId).toBe('withheld');
  expect(item.kind).toBe('withheld');
  expect(item.projectRef).toBe('withheld');
  expect(item.label).toBe('Redacted');
  expect(item.summary).toBe('');
  expect(item.provenance).toEqual([]);
  expect(item.evidenceRefs).toEqual([]);
  expect(item.asOf).toBe('');
  expect(item.source).toEqual({ authority: 'withheld', nativeId: 'withheld', nativeVersion: 'withheld', digest: 'withheld' });
  // The projector drops the marker entirely: no card is emitted.
  expect(projection.cards).toHaveLength(0);
});

test('state tombstoned: the record is not resurrected', async () => {
  const { page, projection } = await readOne('kn_tombstone_0005');
  expect(page.items).toHaveLength(0);
  expect(projection.cards).toHaveLength(0);
});

test('state tombstoned: a live tombstone source event does not resurrect the record', async () => {
  const { client } = clientWith({});
  const before = await client.query(FIX.queryFor('kn_fresh_0001'));
  expect(before.items).toHaveLength(1);
  client.applyFixtureSourceEvent({ type: 'tombstone', stableId: 'kn_fresh_0001' });
  const after = await client.query(FIX.queryFor('kn_fresh_0001'));
  expect(after.items).toHaveLength(0);
  const projection = projectKnowledgePage(after);
  expect(projection.cards).toHaveLength(0);
  // The epoch advanced, so any outstanding cursor is invalidated.
  expect(client.fixtureMetrics().epoch).toBeGreaterThan(1);
});

test('state revoked_scope: the record is not resurrected', async () => {
  const { page, projection } = await readOne('kn_revoked_0006');
  expect(page.items).toHaveLength(0);
  expect(projection.cards).toHaveLength(0);
});

test('state device_scoped_row: another device\'s record is omitted', async () => {
  const { page } = await readOne('kn_devscope_0009');
  expect(page.items).toHaveLength(0);
  // Characterisation (observed): a row-level scope exclusion is silent — the
  // page reports complete with zero items rather than denying.
  expect(page.status).toBe('ok');
  expect(page.completeness).toBe('complete');
});

test('state missing_provenance: the entity is readable but the card has no provenance', async () => {
  // characterisation: true
  // DEFECT PINNED: the projection emits a normal-looking card whose
  // provenanceLabel is an empty string (projection.ts:30 joins an empty
  // provenance array). Nothing in the card or the page says that the
  // provenance is missing.
  const { page, projection } = await readOne('kn_noprov_0007');
  expect(page.items).toHaveLength(1);
  expect(page.items[0].provenance).toEqual([]);
  expect(page.items[0].evidenceRefs).toEqual([]);
  expect(projection.state).toBe('ready');
  expect(projection.cards).toHaveLength(1);
  expect(projection.cards[0].provenanceLabel).toBe('');
  expect(projection.cards[0].statusLabel).toBe('Observed (not an authorization)');
});

failingByDesign(
  'knowledge/missing-provenance',
  'a card with no provenance must be marked or refused, not shown as a normal card',
  async () => {
    const { projection } = await readOne('kn_noprov_0007');
    const card = projection.cards[0];
    const marked = Boolean(card) && (card.provenanceLabel !== '' || /provenance/i.test(card.statusLabel));
    return {
      ok: marked,
      expect: 'missing provenance',
      detail: marked
        ? 'the projection marked the missing provenance'
        : `DEFECT PRESENT: the missing provenance is not marked — the projection emitted a normal card with an empty provenanceLabel (${JSON.stringify(card)}) — source: knowledge/projection.ts:30`,
    };
  },
);

// ===========================================================================
// 2. Revocation must DENY, not degrade
// ===========================================================================

for (const contextState of FIX.CONTEXT_STATES) {
  test(`revocation ${contextState.state}: the query is denied, not degraded to an empty page`, async () => {
    const { client } = clientWith(contextState.overrides);
    const page = await client.query(FIX.queryFor('kn_fresh_0001'));
    expect(page.status).toBe(contextState.expected.page_status);
    expect(page.reasonCode).toBe(contextState.expected.reason_code);
    expect(page.items).toHaveLength(contextState.expected.items);
    expect(page.completeness).toBe('unknown');
    // The denial must not be a silent empty "ok" page.
    expect(page.status).not.toBe('ok');
    // The denial must carry no owner, session, device, world, policy or record
    // identifier, and no timestamp, revision or count.
    const serialised = JSON.stringify(page);
    for (const forbidden of [FIX.OWNER_REF, FIX.SESSION_REF, FIX.DEVICE_REF, FIX.WORLD_REF, 'policy_1', 'kn_fresh_0001']) {
      expect(serialised).not.toContain(forbidden);
    }
    expect(page.asOf).toBeNull();
    expect(page.projectionRevision).toBeNull();
  });
}

for (const requestState of FIX.REQUEST_SCOPE_STATES) {
  test(`scope ${requestState.state}: a request outside the trusted scope is denied, not degraded`, async () => {
    const { client } = clientWith(requestState.context_overrides ?? {});
    const page = await client.query({
      ...FIX.queryFor('kn_fresh_0001'),
      ...requestState.query,
    });
    expect(page.status).toBe(requestState.expected.page_status);
    expect(page.reasonCode).toBe(requestState.expected.reason_code);
    expect(page.items).toHaveLength(requestState.expected.items);
    // A denial, not an empty "ok" page: this is the "denies rather than
    // degrades silently" requirement, asserted on the real client.
    expect(page.status).not.toBe('ok');
    expect(page.completeness).toBe('unknown');
  });
}

test('revocation: an unauthenticated read denies before any search happens', async () => {
  const { client } = clientWith(null);
  const page = await client.query(FIX.queryFor('kn_fresh_0001'));
  expect(page.status).toBe('denied');
  expect(page.reasonCode).toBe('NO_TRUSTED_CONTEXT');
  expect(page.items).toHaveLength(0);
  // The denial happened before any discovery scan.
  expect(client.fixtureMetrics().discoveryScans).toBe(0);
});

// ===========================================================================
// 3. LEAK NEGATIVE — the projection must never leak a hidden body
// ===========================================================================

test('leak-negative: no redacted, tombstoned or revoked body reaches the page or the projection', async () => {
  const hidden = [
    { state: 'redacted', stableId: 'kn_redacted_0003', marker: FIX.SECRET_MARKERS.redacted },
    { state: 'tombstoned', stableId: 'kn_tombstone_0005', marker: FIX.SECRET_MARKERS.tombstoned },
    { state: 'revoked_scope', stableId: 'kn_revoked_0006', marker: FIX.SECRET_MARKERS.revokedScope },
    { state: 'device_scoped_row', stableId: 'kn_devscope_0009', marker: FIX.SECRET_MARKERS.deviceScoped },
  ];
  for (const entry of hidden) {
    // Strict trusted context: the hidden record is not returned at all.
    const strict = await readOne(entry.stableId, {});
    expect(strict.page.items).toHaveLength(0);
    expect(JSON.stringify(strict.page)).not.toContain(entry.marker);
    expect(JSON.stringify(strict.projection)).not.toContain(entry.marker);
    expect(strict.projection.cards).toHaveLength(0);

    // Permissive trusted context: the ONLY thing that may appear is an
    // existence-only withheld marker with no body.
    const permissive = await readOne(entry.stableId, { maySeeRedactedMarkers: true });
    expect(JSON.stringify(permissive.page)).not.toContain(entry.marker);
    expect(JSON.stringify(permissive.projection)).not.toContain(entry.marker);
    expect(permissive.projection.cards).toHaveLength(0);
    for (const item of permissive.page.items) {
      expect(item.visibility).toBe('redacted');
      expect(item.summary).toBe('');
      expect(item.label).toBe('Redacted');
      expect(item.provenance).toEqual([]);
      expect(item.evidenceRefs).toEqual([]);
      expect(item.stableId).toBe('withheld');
      expect(item.source.authority).toBe('withheld');
    }
  }
});

test('leak-negative: the permissive redacted marker carries no body in any field', async () => {
  const { page, projection } = await readOne('kn_redacted_0003', { maySeeRedactedMarkers: true });
  const serialised = JSON.stringify(page);
  expect(serialised).not.toContain(FIX.SECRET_MARKERS.redacted);
  expect(page.items[0].summary).toBe('');
  expect(page.items[0].label).toBe('Redacted');
  expect(JSON.stringify(projection)).not.toContain(FIX.SECRET_MARKERS.redacted);
});

test('leak-negative: no hidden body appears in a multi-entity page either', async () => {
  const { client } = clientWith({});
  const page = await client.query({
    schema: 'vivim.knowledge-query/1',
    requestId: 'req_knowledge_all',
    personalWorldRef: FIX.WORLD_REF,
    purpose: 'task',
    surface: 'pc',
    limit: 50,
    query: 'SECRET_BODY',
  });
  expect(page.items).toHaveLength(0);
  expect(JSON.stringify(page)).not.toContain('SECRET_BODY');
  expect(JSON.stringify(projectKnowledgePage(page))).not.toContain('SECRET_BODY');
});

test('leak-negative: the fixture suite is the pinned one unless a mutation override is set', () => {
  // Documents the proof mechanism: the mutation run sets
  // VIVIM_LABS_KNOWLEDGE_FIXTURE_MODULE, and the leak-negative test above
  // then FAILS, which is the required proof that this suite is not vacuous.
  expect(typeof MUTATED).toBe('boolean');
  expect(FIX.FIXTURE_STATES.length).toBe(9);
});
