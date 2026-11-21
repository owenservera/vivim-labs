// hq/test/annotation-v2.test.mjs — additive v2 annotation grammar: records, diagnostics, comment scope.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import {
  ANNOTATION_V2_SCHEMA,
  SOURCE_LINK_SCHEMA,
  ANNOTATION_V2_TAGS,
  parseAnnotationV2,
  isVersionedId,
  isEffectClass,
  isEvidenceRef,
} from '../lib/annotation-v2.mjs';

// ---------------------------------------------------------------------------
// Fixture: one small registry with a current, a deprecated and a proposed ref.
// ---------------------------------------------------------------------------

const REGISTRY = {
  lookup(id) {
    if (id === 'billing.charge@1') return { id, status: 'current', kind: 'type' };
    if (id === 'billing.retry@1') return { id, status: 'current', kind: 'capability' };
    if (id === 'billing.proposal@1') return { id, status: 'proposed', kind: 'type' };
    if (id === 'legacy.refund@1') return { id, status: 'deprecated', kind: 'type' };
    return null;
  },
};

const OPTIONS = {
  projectId: 'demo',
  path: 'src/billing/retry.mjs',
  contentHash: 'sha256:9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08',
  symbolId: 'retryCharge',
  schemaVersion: '2.1',
  registry: REGISTRY,
};

const codes = (result) => result.records.map((r) => `${r.line}|${r.kind}|${r.value}`);
const findings = (result) => result.diagnostics.map((d) => `${d.line}|${d.code}|${d.value}`);

// ---------------------------------------------------------------------------
// 1. Contract surface
// ---------------------------------------------------------------------------

test('module exports the v2 contract surface', () => {
  assert.equal(ANNOTATION_V2_SCHEMA, 'hq.annotation/2');
  assert.equal(SOURCE_LINK_SCHEMA, 'hq.source-link/1');
  assert.deepEqual(ANNOTATION_V2_TAGS, [
    'type', 'capability', 'effect', 'evidence', 'contract', 'emits', 'consumes', 'action', 'test',
  ]);
});

test('a v2 comment block yields ordered records with provenance', () => {
  const source = [
    '/**',
    ' * @hq:context Billing retry boundary.',
    ' * @hq:type billing.charge@1',
    ' * @hq:capability billing.retry@1',
    ' * @hq:effect host:config-write',
    ' * @hq:evidence test:tests/billing.retry.test.mjs',
    ' */',
    'export function retryCharge() {}',
  ].join('\n');

  const result = parseAnnotationV2(source, OPTIONS);

  assert.equal(result.schema, 'hq.annotation/2');
  assert.equal(result.sourceLinkSchema, 'hq.source-link/1');
  assert.deepEqual(codes(result), [
    '3|type|billing.charge@1',
    '4|capability|billing.retry@1',
    '5|effect|host:config-write',
    '6|evidence|test:tests/billing.retry.test.mjs',
  ]);
  assert.deepEqual(result.diagnostics, []);

  assert.deepEqual(result.records[0], {
    kind: 'type',
    value: 'billing.charge@1',
    projectId: 'demo',
    path: 'src/billing/retry.mjs',
    contentHash: OPTIONS.contentHash,
    symbolId: 'retryCharge',
    line: 3,
    schemaVersion: '2.1',
    provenance: { kind: 'source-comment', path: 'src/billing/retry.mjs', line: 3 },
  });
  assert.deepEqual(Object.keys(result.records[0]), [
    'kind', 'value', 'projectId', 'path', 'contentHash', 'symbolId', 'line', 'schemaVersion', 'provenance',
  ]);

  // Defaults: no options at all means nulls, schemaVersion '2', and no symbolId key.
  const bare = parseAnnotationV2('// @hq:effect host:config-write', { registry: REGISTRY });
  assert.deepEqual(Object.keys(bare.records[0]), [
    'kind', 'value', 'projectId', 'path', 'contentHash', 'line', 'schemaVersion', 'provenance',
  ]);
  assert.equal(bare.records[0].projectId, null);
  assert.equal(bare.records[0].path, null);
  assert.equal(bare.records[0].contentHash, null);
  assert.equal(bare.records[0].schemaVersion, '2');
  assert.deepEqual(bare.records[0].provenance, { kind: 'source-comment', path: null, line: 1 });
});

// ---------------------------------------------------------------------------
// 2. v1 stays v1: zero records, zero diagnostics
// ---------------------------------------------------------------------------

test('a file with only v1 tags produces no records and no diagnostics', () => {
  const source = [
    '// @hq:context Only v1 tags here.',
    '// @hq:owner platform',
    '// @hq:task T-200',
    '// @hq:why Keeps the v1 surface untouched.',
    '// @hq:see src/other.mjs:12',
    '// @hq:invariant ordering must hold',
    '// @hq:todo(platform): add a test',
    '// @hq:risk data loss on replay',
    '// @hq:deprecated use retryCharge instead',
    '// @hq:see src/retry.mjs:30',
    'export function untouched() { return 1; }',
  ].join('\n');

  const result = parseAnnotationV2(source, OPTIONS);
  assert.equal(result.records.length, 0);
  assert.equal(result.diagnostics.length, 0);
});

test('an unknown @hq: tag is ignored without a diagnostic', () => {
  const result = parseAnnotationV2('// @hq:wat something arbitrary', OPTIONS);
  assert.deepEqual(result.records, []);
  assert.deepEqual(result.diagnostics, []);
});

// ---------------------------------------------------------------------------
// 3-5. Comment scope: strings, template literals, regex literals, bare code
// ---------------------------------------------------------------------------

test('a v2 tag inside a quoted string is never a record', () => {
  const source = [
    "const note = '@hq:type billing.charge@1';",
    'const other = "@hq:capability billing.retry@1";',
    "const url = 'https://example.com/* @hq:type billing.charge@1 */';",
  ].join('\n');

  const result = parseAnnotationV2(source, OPTIONS);
  assert.deepEqual(result.records, []);
  assert.deepEqual(result.diagnostics, []);
});

test('a v2 tag inside a template literal is never a record', () => {
  const source = [
    'const note = `@hq:type billing.charge@1 ${name}`;',
    'const multi = `',
    '@hq:effect host:config-write',
    '`;',
  ].join('\n');

  const result = parseAnnotationV2(source, OPTIONS);
  assert.deepEqual(result.records, []);
  assert.deepEqual(result.diagnostics, []);
});

test('a v2 tag inside a regex literal is never a record', () => {
  const result = parseAnnotationV2('const re = /@hq:type billing.charge@1/;', OPTIONS);
  assert.deepEqual(result.records, []);
  assert.deepEqual(result.diagnostics, []);
});

test('a v2 tag in bare code, not in a comment, is ignored', () => {
  const source = [
    'const meta = { tag: @hq:type billing.charge@1 };',
    'const inline = value; // @hq:effect host:config-write',
  ].join('\n');

  const result = parseAnnotationV2(source, OPTIONS);
  assert.deepEqual(codes(result), ['2|effect|host:config-write']);
  assert.deepEqual(result.diagnostics, []);
});

// ---------------------------------------------------------------------------
// 6-9. Registry validation
// ---------------------------------------------------------------------------

test('an unknown ref is emitted with an unknown_ref error', () => {
  const result = parseAnnotationV2('// @hq:type billing.unknown@1', OPTIONS);
  assert.deepEqual(codes(result), ['1|type|billing.unknown@1']);
  assert.deepEqual(findings(result), ['1|unknown_ref|billing.unknown@1']);
  assert.equal(result.diagnostics[0].severity, 'error');
  assert.equal(result.diagnostics[0].kind, 'type');
});

test('no registry supplied behaves exactly like an unknown ref', () => {
  const result = parseAnnotationV2('// @hq:capability billing.retry@1');
  assert.deepEqual(codes(result), ['1|capability|billing.retry@1']);
  assert.deepEqual(findings(result), ['1|unknown_ref|billing.retry@1']);
});

test('a registry without a lookup method behaves like an unknown ref', () => {
  const result = parseAnnotationV2('// @hq:action billing.retry@1', { registry: {} });
  assert.deepEqual(findings(result), ['1|unknown_ref|billing.retry@1']);
  assert.equal(result.records.length, 1);
});

test('a throwing registry.lookup is caught and reported as unknown_ref', () => {
  const exploding = { lookup() { throw new Error('registry offline'); } };
  const result = parseAnnotationV2('// @hq:type billing.charge@1', { registry: exploding });
  assert.deepEqual(codes(result), ['1|type|billing.charge@1']);
  assert.deepEqual(findings(result), ['1|unknown_ref|billing.charge@1']);
});

test('a deprecated ref warns; a proposed ref is silent', () => {
  const source = [
    '// @hq:type legacy.refund@1',
    '// @hq:type billing.proposal@1',
    '// @hq:type billing.charge@1',
  ].join('\n');

  const result = parseAnnotationV2(source, OPTIONS);
  assert.deepEqual(codes(result), [
    '1|type|legacy.refund@1',
    '2|type|billing.proposal@1',
    '3|type|billing.charge@1',
  ]);
  assert.deepEqual(findings(result), ['1|deprecated_ref|legacy.refund@1']);
  assert.equal(result.diagnostics[0].severity, 'warning');
});

// ---------------------------------------------------------------------------
// 10. Grammar
// ---------------------------------------------------------------------------

test('each grammar family rejects its malformed value with no record', () => {
  const source = [
    '// @hq:type billing.charge',
    '// @hq:effect config-write',
    '// @hq:evidence tests/billing.retry.test.mjs',
    '// @hq:test',
    '// @hq:capability Billing.Retry@1',
    '// @hq:contract billing.retry@',
  ].join('\n');

  const result = parseAnnotationV2(source, OPTIONS);
  assert.deepEqual(result.records, []);
  assert.deepEqual(findings(result), [
    '1|invalid_value|billing.charge',
    '2|invalid_value|config-write',
    '3|invalid_value|tests/billing.retry.test.mjs',
    '4|invalid_value|',
    '5|invalid_value|Billing.Retry@1',
    '6|invalid_value|billing.retry@',
  ]);
  for (const finding of result.diagnostics) {
    assert.equal(finding.severity, 'error');
    assert.deepEqual(Object.keys(finding), ['code', 'severity', 'kind', 'value', 'line', 'message']);
  }
});

// ---------------------------------------------------------------------------
// 11. Secret paths
// ---------------------------------------------------------------------------

test('a secret-looking path segment is refused and never emitted', () => {
  const source = [
    '// @hq:evidence test:tests/.env-fixture',
    '// @hq:test test:tests\\SESSION.JSON',
    '// @hq:evidence test:tests/session.json',
    '// @hq:test test:keys/release-profile.json',
    '// @hq:evidence test:tests/billing.retry.test.mjs',
  ].join('\n');

  const result = parseAnnotationV2(source, OPTIONS);
  assert.deepEqual(codes(result), ['5|evidence|test:tests/billing.retry.test.mjs']);
  assert.deepEqual(findings(result), [
    '1|secret_path|test:tests/.env-fixture',
    '2|secret_path|test:tests\\SESSION.JSON',
    '3|secret_path|test:tests/session.json',
    '4|secret_path|test:keys/release-profile.json',
  ]);
  for (const finding of result.diagnostics) {
    assert.equal(finding.severity, 'error');
    assert.equal(finding.message.includes('.env-fixture'), false, 'message does not echo the refused path');
  }
});

test('a versioned id that merely contains a dot is not a secret path', () => {
  const result = parseAnnotationV2('// @hq:type billing.charge@1.2', OPTIONS);
  assert.deepEqual(codes(result), ['1|type|billing.charge@1.2']);
  // Unregistered (the fixture only knows @1), but refused as an unknown ref, never as a secret path.
  assert.deepEqual(findings(result), ['1|unknown_ref|billing.charge@1.2']);
});

// ---------------------------------------------------------------------------
// 12. Duplicates
// ---------------------------------------------------------------------------

test('a duplicate identical tag on one line keeps one record and warns', () => {
  const result = parseAnnotationV2('// @hq:type billing.charge@1 @hq:type billing.charge@1', OPTIONS);
  assert.deepEqual(codes(result), ['1|type|billing.charge@1']);
  assert.deepEqual(findings(result), ['1|duplicate_tag|billing.charge@1']);
  assert.equal(result.diagnostics[0].severity, 'warning');
});

test('the same tag on two different lines is two records, not a duplicate', () => {
  const source = ['// @hq:type billing.charge@1', '// @hq:type billing.charge@1'].join('\n');
  const result = parseAnnotationV2(source, OPTIONS);
  assert.deepEqual(codes(result), ['1|type|billing.charge@1', '2|type|billing.charge@1']);
  assert.deepEqual(result.diagnostics, []);
});

// ---------------------------------------------------------------------------
// 13. Deterministic ordering
// ---------------------------------------------------------------------------

test('records and diagnostics are deterministically sorted', () => {
  const source = [
    '/* @hq:test test:tests/z.test.mjs @hq:evidence test:tests/a.test.mjs */',
    '// @hq:effect Bad-Class',
    '// @hq:type billing.unknown@1',
    '// @hq:capability billing.retry@1',
  ].join('\n');

  const result = parseAnnotationV2(source, OPTIONS);
  assert.deepEqual(codes(result), [
    '1|evidence|test:tests/a.test.mjs',
    '1|test|test:tests/z.test.mjs',
    '3|type|billing.unknown@1',
    '4|capability|billing.retry@1',
  ]);
  assert.deepEqual(findings(result), [
    '2|invalid_value|Bad-Class',
    '3|unknown_ref|billing.unknown@1',
  ]);

  assert.deepEqual(parseAnnotationV2(source, OPTIONS), result, 'same input, same output');
});

test('diagnostics on one line sort by code then value', () => {
  const source = '// @hq:effect Bad-Class @hq:type billing.unknown@1 @hq:type a.b@1';
  const result = parseAnnotationV2(source, { registry: { lookup: (id) => (id === 'a.b@1' ? { id, status: 'current' } : null) } });
  assert.deepEqual(findings(result), [
    '1|invalid_value|Bad-Class',
    '1|unknown_ref|billing.unknown@1',
  ]);
});

// ---------------------------------------------------------------------------
// 14. Malformed input never throws
// ---------------------------------------------------------------------------

test('malformed input returns a valid empty result instead of throwing', () => {
  for (const input of [null, undefined, 123, {}, [], true, '']) {
    const result = parseAnnotationV2(input);
    assert.equal(result.schema, 'hq.annotation/2');
    assert.equal(result.sourceLinkSchema, 'hq.source-link/1');
    assert.deepEqual(result.records, []);
    assert.deepEqual(result.diagnostics, []);
  }

  const noOptions = parseAnnotationV2('// @hq:type billing.charge@1', null);
  assert.deepEqual(findings(noOptions), ['1|unknown_ref|billing.charge@1']);
});

test('an unterminated block comment and an unterminated string do not throw', () => {
  assert.doesNotThrow(() => parseAnnotationV2('/* @hq:type billing.charge@1', OPTIONS));
  assert.doesNotThrow(() => parseAnnotationV2("const s = 'unterminated @hq:type billing.charge@1", OPTIONS));
});

// ---------------------------------------------------------------------------
// 15. Predicates
// ---------------------------------------------------------------------------

test('isVersionedId accepts name@version and rejects the rest', () => {
  for (const value of ['billing.retry@1', 'a@1', 'a.b-c_d@1.2.3', 'x@0']) {
    assert.equal(isVersionedId(value), true, value);
  }
  for (const value of ['billing.retry', 'billing.retry@', '@1', 'Billing@1', 'a b@1', 'a@v1', 'a@@1', 'a@1@2', '', null, undefined, 7]) {
    assert.equal(isVersionedId(value), false, String(value));
  }
});

test('isEffectClass accepts class:verb and rejects the rest', () => {
  for (const value of ['host:config-write', 'fs:read', 'net-egress:http-get', 'a:b']) {
    assert.equal(isEffectClass(value), true, value);
  }
  for (const value of ['config-write', 'host:', ':write', 'host:config write', 'Host:Write', 'host:write:extra', '', null, undefined]) {
    assert.equal(isEffectClass(value), false, String(value));
  }
});

test('isEvidenceRef requires the test: prefix and a non-empty remainder', () => {
  for (const value of ['test:tests/a.test.mjs', 'test:a']) {
    assert.equal(isEvidenceRef(value), true, value);
  }
  for (const value of ['tests/a.test.mjs', 'test:', 'test:   ', 'Test:a', '', null, undefined, 5]) {
    assert.equal(isEvidenceRef(value), false, String(value));
  }
});
