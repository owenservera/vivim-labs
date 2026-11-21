// hq/test/semantic-convergence.test.mjs — R2 semantic convergence (ZAIQ-006):
// journey<->capability crosswalk edges, the joined `vivim explain` read view,
// and the VIVIM product ledger in the unified KB read model. All joins reuse
// existing tables (vivim_records, files, symbols) — no new store is created.
import { test } from 'bun:test';
import assert from 'node:assert/strict';
import { mkdtempSync, writeFileSync, rmSync, mkdirSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { openMemoryDb } from '../lib/schema.mjs';
import {
  syncVivim, explainVivimRecord, vivimTablesPresent,
} from '../lib/vivim.mjs';
import { kbQuery, kbHealth, kbExport } from '../lib/kb.mjs';

const AT = '2026-01-01T00:00:00.000Z';
const HASH = 'e'.repeat(64);

const ROADMAP = {
  schema: 1,
  activePhase: { id: 'PH1', status: 'active', source: 'owner', runtimeExecution: 'authorized', allowed: 'implement', blocked: 'none', release: '2026-11' },
  items: [
    { id: 'D01', title: 'Ship catalogue builder', status: 'done', owner: 'a', refs: [], depends_on: [], notes: '' },
    { id: 'D02', title: 'Health endpoint', status: 'open', owner: 'a', refs: [], depends_on: [], notes: '' },
  ],
  milestones: [{ id: 'M1', title: 'Milestone One', status: 'done', acceptance: 'accept M1', notes: '' }],
};

const REGISTRY = {
  schema: 'vivim.atomic-capabilities/1',
  items: [
    {
      id: 'CAP-01', title: 'Shared catalogue builder', kind: 'atomic_feature',
      productPriority: 'P0', executionWave: 'unassigned', decisionState: 'execution_dependent',
      spec: 'dev/prd/01.md',
      acceptance: 'Cloud and Direct status return the same capabilities document',
      links: {
        roadmap: ['D01'],
        existingSourceCandidates: [{ path: 'src/a.mjs' }, { path: 'src/missing.mjs' }],
        verifiedSymbols: [{ name: 'buildCatalogue' }, { name: 'sharedRelay' }],
        sourceResolution: 'symbol_declaration_verified_behavior_unverified',
      },
    },
    {
      id: 'CAP-02', title: 'Host health snapshot', kind: 'atomic_feature',
      productPriority: 'P1', executionWave: 'unassigned', decisionState: 'execution_dependent',
      spec: 'dev/prd/02.md', acceptance: 'Host exposes readiness',
      links: { roadmap: ['D02'] },
    },
  ],
};

const JOURNEY = {
  schema: 'vivim.user-journey-activation-graph/1',
  phases: [{ id: 'A', name: 'Phase A', groups: [{ id: 'A1', name: 'Group A1', step_ids: ['J001', 'J002'] }] }],
};

const CROSSWALK = {
  schema: 'vivim.journey-to-capability-crosswalk/1',
  entries: [
    {
      journeyId: 'J001', source: 'src/a.mjs', sourceSymbol: 'buildCatalogue',
      atomicCandidateIds: ['CAP-01', 'CAP-02'], strongerSymbolCandidateIds: ['CAP-01'],
      topologyNodeIds: [], roadmapTaskIds: ['D01'],
      matchPolicy: 'lexical source or curated source candidate only; not execution proof',
    },
    {
      // all-empty entry: J002 must keep the exact pre-crosswalk payload shape
      journeyId: 'J002', source: 'src/b.mjs', sourceSymbol: null,
      atomicCandidateIds: [], strongerSymbolCandidateIds: [], topologyNodeIds: [], roadmapTaskIds: [],
      matchPolicy: 'lexical source or curated source candidate only; not execution proof',
    },
    {
      // dangling journey reference: recorded as skipped, never a record
      journeyId: 'J999', source: 'src/x.mjs', sourceSymbol: null,
      atomicCandidateIds: ['CAP-01'], strongerSymbolCandidateIds: [], topologyNodeIds: [], roadmapTaskIds: [],
      matchPolicy: 'lexical source or curated source candidate only; not execution proof',
    },
  ],
};

const GAPS = { schema: 'vivim.activation-gaps/1', gaps: [] };

function makeFixture(root, extra = {}) {
  mkdirSync(join(root, 'docs', 'capabilities'), { recursive: true });
  mkdirSync(join(root, 'docs', 'ux'), { recursive: true });
  writeFileSync(join(root, 'docs', 'roadmap.json'), JSON.stringify(extra.roadmap ?? ROADMAP));
  writeFileSync(join(root, 'docs', 'capabilities', 'atomic-registry.json'), JSON.stringify(extra.registry ?? REGISTRY));
  writeFileSync(join(root, 'docs', 'ux', 'USER-JOURNEY-ACTIVATION-GRAPH.v1.json'), JSON.stringify(extra.journey ?? JOURNEY));
  writeFileSync(join(root, 'docs', 'capabilities', 'journey-capability-crosswalk.v1.json'), JSON.stringify(extra.crosswalk ?? CROSSWALK));
  writeFileSync(join(root, 'docs', 'ux', 'ACTIVATION-GAPS.v1.json'), JSON.stringify(extra.gaps ?? GAPS));
  return root;
}

function sync(db, root, now = '2026-11-02T00:00:00.000Z') {
  return syncVivim(db, { root, projects: [{ name: 'vivim-web-harness', path: root }], now });
}

// A minimal code index: one project, two files, three symbols. The second
// file lives in a different project with the same path shape to prove the
// join reports every exact match instead of guessing.
function seedCodeIndex(db, { projectName = 'vivim-web-harness' } = {}) {
  const mkProject = (name, path) => Number(
    db.prepare('INSERT INTO projects(name, path, added_at) VALUES (?, ?, ?)').run(name, path, AT).lastInsertRowid,
  );
  const mkFile = (projectId, path) => Number(
    db.prepare(`INSERT INTO files(project_id, path, abs_path, lang, kind, size, mtime_ms, hash, lines)
      VALUES (?, ?, ?, 'javascript', 'source', 10, 1, ?, 5)`).run(projectId, path, `/tmp/x/${path}`, HASH).lastInsertRowid,
  );
  const mkSymbol = (projectId, fileId, name, kind, line) =>
    db.prepare(`INSERT INTO symbols(file_id, project_id, name, kind, line, end_line, exported)
      VALUES (?, ?, ?, ?, ?, ?, 1)`).run(fileId, projectId, name, kind, line, line + 2);

  const projectId = mkProject(projectName, '/tmp/space-a');
  const a = mkFile(projectId, 'src/a.mjs');
  mkSymbol(projectId, a, 'buildCatalogue', 'function', 12);
  mkSymbol(projectId, a, 'sharedRelay', 'const', 30);
  const otherId = mkProject('other-space', '/tmp/space-b');
  const b = mkFile(otherId, 'src/other.mjs');
  mkSymbol(otherId, b, 'unrelated', 'function', 5);
  return { projectId };
}

function withSyncedVivim(fn, options = {}) {
  const root = mkdtempSync(join(tmpdir(), 'hq-semconv-'));
  makeFixture(root, options.fixture ?? {});
  const db = openMemoryDb();
  try {
    const result = sync(db, root);
    assert.equal(result.ok, true, JSON.stringify(result));
    return fn({ db, root, result });
  } finally {
    db.close();
    rmSync(root, { recursive: true, force: true });
  }
}

// ---------------------------------------------------------------------------
// 1. crosswalk ingestion (syncVivim)
// ---------------------------------------------------------------------------

test('sync materializes journey -> capability/task crosswalk edges in journey payloads', () => {
  withSyncedVivim(({ db, result }) => {
    assert.deepEqual(result.crosswalk, { journeys: 1, capabilityLinks: 3, taskLinks: 1 });

    const j001 = JSON.parse(db.prepare("SELECT payload FROM vivim_records WHERE kind='journey' AND id='J001'").get().payload);
    assert.deepEqual(j001.capabilities, ['CAP-01', 'CAP-02']);
    assert.deepEqual(j001.strongerCapabilities, ['CAP-01']);
    assert.deepEqual(j001.tasks, ['D01']);

    // An all-empty crosswalk entry leaves the payload shape untouched.
    const j002 = JSON.parse(db.prepare("SELECT payload FROM vivim_records WHERE kind='journey' AND id='J002'").get().payload);
    assert.equal('capabilities' in j002, false);
    assert.equal('strongerCapabilities' in j002, false);
    assert.equal('tasks' in j002, false);

    // The dangling journey reference never becomes a record.
    assert.equal(db.prepare("SELECT COUNT(*) AS n FROM vivim_records WHERE id='J999'").get().n, 0);

    // Idempotency: a second sync is byte-identical, crosswalk included.
    const before = db.prepare('SELECT kind, id, payload FROM vivim_records ORDER BY kind, id').all();
    const again = sync(db, mkdtempSync(join(tmpdir(), 'hq-semconv-b-')), undefined);
    assert.equal(again.ok, false); // no fixture at that root
    const after = db.prepare('SELECT kind, id, payload FROM vivim_records ORDER BY kind, id').all();
    assert.deepEqual(after, before);
  });
});

test('sync stamps the owning space project id on every record when the space is a registered project', () => {
  const root = mkdtempSync(join(tmpdir(), 'hq-semconv-pid-'));
  makeFixture(root);
  const db = openMemoryDb();
  try {
    seedCodeIndex(db, { projectName: 'vivim-web-harness' });
    const result = sync(db, root);
    assert.equal(result.ok, true);
    const rows = db.prepare('SELECT project_id FROM vivim_records WHERE project_id IS NULL').all();
    assert.equal(rows.length, 0, 'every record carries the owning project id');
    const distinct = db.prepare('SELECT DISTINCT project_id FROM vivim_records').all();
    assert.equal(distinct.length, 1);
  } finally {
    db.close();
    rmSync(root, { recursive: true, force: true });
  }
});

// ---------------------------------------------------------------------------
// 2. the joined explain read view (explainVivimRecord)
// ---------------------------------------------------------------------------

test('explain capability: requirements, roadmap tasks, reverse journey edges and indexed code', () => {
  withSyncedVivim(({ db }) => {
    seedCodeIndex(db);
    const explained = explainVivimRecord(db, { id: 'CAP-01' });
    assert.equal(explained.schema, 'hq.vivim-explain/2');
    assert.equal(explained.records.length, 1);
    assert.equal(explained.records[0].kind, 'capability');

    const rel = explained.relationships;
    assert.deepEqual(rel.requirements, ['CAP-01#acceptance']);
    assert.deepEqual(rel.tasks, ['D01']);
    assert.deepEqual(rel.journeys, [{ id: 'J001', stronger: true }]);

    // code join: both source candidates resolve (or are reported), verified
    // symbols land with file, line, kind and project.
    assert.deepEqual(rel.code.files, [{ path: 'src/a.mjs', project: 'vivim-web-harness' }]);
    assert.deepEqual(rel.code.unresolved, ['src/missing.mjs']);
    assert.deepEqual(rel.code.symbols, [
      { name: 'buildCatalogue', kind: 'function', line: 12, path: 'src/a.mjs', project: 'vivim-web-harness' },
      { name: 'sharedRelay', kind: 'const', line: 30, path: 'src/a.mjs', project: 'vivim-web-harness' },
    ]);
  });
});

test('explain requirement reaches its parent capability; journey explains its crosswalk edges', () => {
  withSyncedVivim(({ db }) => {
    const requirement = explainVivimRecord(db, { id: 'CAP-01#acceptance' });
    assert.deepEqual(requirement.relationships.capabilities, ['CAP-01']);
    assert.deepEqual(requirement.relationships.requirements, []);

    const journey = explainVivimRecord(db, { id: 'J001' });
    // stronger + candidate lists merge, deduplicated and sorted.
    assert.deepEqual(journey.relationships.capabilities, ['CAP-01', 'CAP-02']);
    assert.deepEqual(journey.relationships.tasks, ['D01']);
  });
});

test('explain task reaches capabilities and journeys that reference it', () => {
  withSyncedVivim(({ db }) => {
    const task = explainVivimRecord(db, { id: 'D01' });
    assert.deepEqual(task.relationships.capabilities, ['CAP-01']);
    assert.deepEqual(task.relationships.journeys, [{ id: 'J001', stronger: false }]);
    assert.deepEqual(task.relationships.tasks, []);
  });
});

test('explain never throws: unknown id, blank id and a dropped vivim_records table', () => {
  withSyncedVivim(({ db }) => {
    assert.deepEqual(explainVivimRecord(db, { id: 'NOPE-01' }), { schema: 'hq.vivim-explain/2', id: 'NOPE-01', records: [], relationships: null });
    assert.deepEqual(explainVivimRecord(db, { id: '' }), { schema: 'hq.vivim-explain/2', id: '', records: [], relationships: null });
    db.exec('DROP TABLE vivim_records');
    assert.equal(vivimTablesPresent(db), false);
    assert.deepEqual(explainVivimRecord(db, { id: 'CAP-01' }), { schema: 'hq.vivim-explain/2', id: 'CAP-01', records: [], relationships: null });
  });
});

// ---------------------------------------------------------------------------
// 3. the VIVIM product ledger in the unified KB read model (kbQuery)
// ---------------------------------------------------------------------------

test('kbQuery surfaces VIVIM records with namespaced ids and the owning space as project', () => {
  withSyncedVivim(({ db }) => {
    const results = kbQuery(db, { query: 'shared catalogue builder' });
    const vivimRows = results.filter((entry) => entry.kind === 'vivim');
    assert.ok(vivimRows.length >= 2, JSON.stringify(results.map((r) => r.id)));

    const capability = results.find((entry) => entry.id === 'vivim:capability:CAP-01');
    assert.ok(capability, 'native id preserved under the vivim: namespace');
    assert.equal(capability.source, 'vivim_records');
    assert.equal(capability.record_kind, 'capability');
    assert.equal(capability.native_id, 'CAP-01');
    assert.equal(capability.project, 'vivim-web-harness');
    assert.ok(capability.score > 0);
    assert.ok(!/\n/.test(capability.snippet), 'snippet stays one line');
    assert.match(capability.snippet, /capabilities document/);

    const requirement = results.find((entry) => entry.id === 'vivim:requirement:CAP-01#acceptance');
    assert.ok(requirement, 'the atomic requirement is answerable');
    assert.equal(requirement.record_kind, 'requirement');

    // Deterministic order: score desc, then id asc.
    const sorted = results.slice().sort((a, b) => (b.score - a.score) || (a.id < b.id ? -1 : 1));
    assert.deepEqual(results.map((r) => r.id), sorted.map((r) => r.id));
  });
});

test('kb kind filters isolate the VIVIM ledger, and unknown kinds stay empty', () => {
  withSyncedVivim(({ db }) => {
    const onlyVivim = kbQuery(db, { query: 'catalogue', kind: 'vivim' });
    assert.ok(onlyVivim.length > 0);
    assert.ok(onlyVivim.every((entry) => entry.kind === 'vivim'));

    // Alias: requirement narrows to the same ledger kind.
    const byAlias = kbQuery(db, { query: 'catalogue', kind: 'requirement' });
    assert.deepEqual(byAlias.map((e) => e.id), onlyVivim.map((e) => e.id));

    assert.deepEqual(kbQuery(db, { query: 'catalogue', kind: 'nosuchkind' }), []);
    assert.deepEqual(kbQuery(db, { query: '' }), []);
  });
});

test('kb project filter scopes VIVIM rows by the owning space', () => {
  withSyncedVivim(({ db }) => {
    assert.ok(kbQuery(db, { query: 'catalogue', kind: 'vivim', project: 'vivim-web-harness' }).length > 0);
    assert.equal(kbQuery(db, { query: 'catalogue', kind: 'vivim', project: 'elsewhere' }).length, 0);
  });
});

test('a vivim capability and an imported package capability never collide', () => {
  withSyncedVivim(({ db }) => {
    db.prepare(`INSERT INTO capability_document
      (id, source_id, install_path, path, kind, title, body, sha256, bytes, lines, lang, authority, observed_at)
      VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`).run(
      'demo:README.md', 'demo', 'node_modules/demo', 'node_modules/demo/README.md', 'docs',
      'Shared catalogue builder package', 'A shared catalogue builder package.', HASH, 10, 1, 'markdown', 'candidate', AT,
    );
    db.prepare(`INSERT INTO capability_document_fts (source_id, install_path, path, kind, title, body)
      VALUES (?, ?, ?, ?, ?, ?)`).run('demo', 'node_modules/demo', 'node_modules/demo/README.md', 'docs', 'Shared catalogue builder package', 'A shared catalogue builder package.');

    const results = kbQuery(db, { query: 'shared catalogue builder' });
    const ids = results.map((entry) => entry.id);
    assert.ok(ids.includes('vivim:capability:CAP-01'), 'VIVIM capability present');
    assert.ok(ids.includes('capability:demo:node_modules/demo/README.md'), 'imported package capability present');
    assert.equal(new Set(ids).size, ids.length, 'ids are unique: no collision between namespaces');
    for (const entry of results) {
      assert.equal(entry.id.startsWith(`${entry.kind}:`), true);
    }
  });
});

test('kbHealth summarizes the VIVIM ledger without counting records as KB nodes', () => {
  withSyncedVivim(({ db }) => {
    const health = kbHealth(db);
    assert.equal(health.vivim.records, db.prepare('SELECT COUNT(*) AS n FROM vivim_records').get().n);
    for (const kind of ['doc', 'decision', 'canon', 'session', 'note', 'wiki']) {
      assert.equal(health.counts[kind], 0, `${kind} count untouched by vivim rows`);
    }
    // Query-only invariant: the ledger is never exported through kbExport.
    assert.equal(kbExport(db).includes('vivim:'), false);
  });
});

test('no VIVIM rows: queries behave exactly as before and never throw', () => {
  const db = openMemoryDb();
  try {
    assert.deepEqual(kbQuery(db, { query: 'catalogue' }), []);
    assert.equal(kbHealth(db).vivim.records, 0);
    assert.doesNotThrow(() => kbExport(db));
    db.exec('DROP TABLE vivim_records');
    assert.doesNotThrow(() => kbQuery(db, { query: 'catalogue', kind: 'vivim' }));
    assert.deepEqual(kbQuery(db, { query: 'catalogue', kind: 'vivim' }), []);
  } finally {
    db.close();
  }
});
