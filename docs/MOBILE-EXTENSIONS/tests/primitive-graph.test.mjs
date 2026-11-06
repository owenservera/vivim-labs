// Research-only fixture validation. This test performs NO browser/device/Host action.
// Run: node --test docs/MOBILE-EXTENSIONS/tests/primitive-graph.test.mjs
import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const fixture = new URL('../fixtures/primitive-requirements.v0.json', import.meta.url);
const doc = JSON.parse(readFileSync(fixture, 'utf8'));
const items = doc.primitives;
const byId = new Map(items.map(p => [p.id, p]));

test('research fixture cannot become an execution authority', () => {
  assert.equal(doc.schema, 'vivim.lab.mobile-primitive-requirements/0');
  assert.equal(doc.status, 'research_only_not_runtime_registry');
  assert.equal(doc.authority, 'no_execution_granted');
  assert.ok(items.length >= 12);
  for (const p of items) {
    assert.equal(p.androidEvidence, 'not_device_tested', p.id);
    assert.ok(['none_in_fixture', 'pc_host_only'].includes(p.effectAuthority), p.id);
    assert.match(p.id, /^[a-z0-9][a-z0-9.-]+$/);
    assert.equal(p.sourceRepo, 'owenservera/vivim-web-harness');
    assert.ok(!p.sourcePath.includes('..') && !p.sourcePath.startsWith('/'), p.id);
    assert.ok(['android','host','studio','browser'].includes(p.currentOwner), p.id);
    assert.ok(Array.isArray(p.proposedAdapters) && p.proposedAdapters.length > 0, p.id);
  }
  assert.equal(byId.size, items.length, 'primitive IDs must be unique');
  assert.equal(byId.get('browser.send-once').effectAuthority, 'pc_host_only');
});

test('dependencies are explicit and acyclic', () => {
  const visited = new Set(), active = new Set();
  function visit(id) {
    assert.ok(byId.has(id), 'unknown primitive: '+id);
    assert.ok(!active.has(id), 'cycle at '+id);
    if (visited.has(id)) return;
    active.add(id);
    for (const dep of byId.get(id).dependsOn) visit(dep);
    active.delete(id);
    visited.add(id);
  }
  for (const p of items) visit(p.id);
  assert.equal(visited.size, items.length);
});

test('read-only extension sensor does not claim a mobile provider sender', () => {
  const p = byId.get('extension.passive-observe');
  assert.ok(p, 'passive extension source should be inventoried');
  assert.equal(p.currentOwner, 'browser');
  assert.equal(p.effectAuthority, 'none_in_fixture');
  assert.ok(!p.proposedAdapters.some(x => /effect|send|actuat/i.test(x)));
});
