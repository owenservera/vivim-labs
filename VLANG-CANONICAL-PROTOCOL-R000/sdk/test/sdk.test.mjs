import test from 'node:test';
import assert from 'node:assert/strict';
import { createVlangSdk, sourceSha256, VlangSdkError } from '../src/index.mjs';

const body = '// @hq:capability repo.inspect@1\nexport function inspect() {}\n';
const pin = { projectId: 'repo', path: 'src/inspect.ts', sourceSha256: sourceSha256(body) };
let parseCount = 0;
const ports = {
  parseAnnotations: async (_source, opt) => {
    parseCount++;
    return { schema: 'hq.annotation/2',
      records: [{ kind: 'capability', value: 'repo.inspect@1', projectId: opt.projectId,
        path: opt.path, contentHash: opt.contentHash, line: 1 }],
      diagnostics: [] };
  },
  composeContext: async ({ task }) => ({ task, schema: 'hq.context-bundle/1' }),
  verifyContext: async ({ pin: p }) => ({ ok: true, sourceSha256: p.sourceSha256 }),
};
const sdk = createVlangSdk(ports);
const fails = async (fn, code) => assert.rejects(fn, e => e instanceof VlangSdkError && e.code === code);

test('source hashes are exact; annotated facts confer no authority', async () => {
  const result = await sdk.inspectSource({ pin, source: body });
  assert.equal(result.scope, 'js-ts-comment-only');
  assert.equal(result.annotations[0].assurance, 'declared-unverified');
  assert.equal(result.annotations[0].executionAuthorized, false);
  assert.equal(result.executionAuthorized, false);
});
test('stale bytes are refused before calling HQ parser', async () => {
  const before = parseCount;
  await fails(() => sdk.inspectSource({ pin, source: body + ' ' }), 'STALE_SOURCE');
  assert.equal(parseCount, before);
});
test('rejects unsafe and unsupported paths', async () => {
  await fails(() => sdk.inspectSource({ pin: { ...pin, path: '../secret.ts' }, source: body }), 'UNSAFE_PATH');
  await fails(() => sdk.inspectSource({ pin: { ...pin, path: 'src/view.tsx' }, source: body }), 'UNSUPPORTED_LANGUAGE');
  await fails(() => sdk.inspectSource({ pin: { ...pin, path: '.env/local.ts' }, source: body }), 'UNSAFE_PATH');
});
test('Rust reads only leading comment header', async () => {
  const rs = '/// @hq:capability repo.inspect@1\nfn main() { let x = r#\"// @hq:capability unsafe@1\"#; }\n';
  let observed = null;
  const rust = createVlangSdk({ ...ports, parseAnnotations: async (s, opt) => {
    observed = s;
    return { schema: 'hq.annotation/2', records: [], diagnostics: [] };
  } });
  const result = await rust.inspectSource({ pin: { ...pin, path: 'src/lib.rs', sourceSha256: sourceSha256(rs) }, source: rs });
  assert.equal(result.scope, 'rust-file-header-only');
  assert.equal(observed, '/// @hq:capability repo.inspect@1');
});
test('parser records cannot escape their pinned source', async () => {
  const altered = createVlangSdk({ ...ports, parseAnnotations: async () => ({
    schema: 'hq.annotation/2', records: [{ projectId: 'someone-else' }], diagnostics: [],
  }) });
  await fails(() => altered.inspectSource({ pin, source: body }), 'UNBOUND_RECORD');
});
test('context requires independent freshness attestation', async () => {
  const result = await sdk.getVerifiedContext({ pin, task: 'inspect code' });
  assert.equal(result.contextVerified, true);
  assert.equal(result.executionAuthorized, false);
  const stale = createVlangSdk({ ...ports, verifyContext: async () => ({ ok: true, sourceSha256: '0'.repeat(64) }) });
  await fails(() => stale.getVerifiedContext({ pin, task: 'inspect code' }), 'UNVERIFIED_CONTEXT');
});
test('lineage is not inferred from path changes', () => {
  const next = { ...pin, sourceSha256: '1'.repeat(64) };
  assert.equal(sdk.compareSourcePins(pin, next).dependentClaims, 'requires-revalidation');
  assert.throws(() => sdk.compareSourcePins(pin, { ...next, path: 'src/renamed.ts' }), /IDENTITY_MISMATCH/);
});
test('no ambient port, approval, mutation or effects', () => {
  assert.throws(() => createVlangSdk({}), /MISSING_PORT/);
  assert.equal(Object.keys(sdk).sort().join(','), 'compareSourcePins,getVerifiedContext,inspectSource,schema');
});
