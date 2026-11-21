import { test } from 'node:test';
import assert from 'node:assert/strict';
import { canonical, fingerprint, changedAnchors, anchorsCurrent, planningClaim, previewBinding, reusableCommandKey, boundedPacket, currentDraft } from './semantic-primitives.mjs';
const versions = { catalog:1, compiler:1, contract:1, capability:1, policy:1, rules:1, locale:'en' };
test('stable key order but exact literal values and domain separation', () => {
  assert.equal(canonical({ b:1, a:' x\n' }), canonical({ a:' x\n', b:1 }));
  assert.notEqual(fingerprint('meaning',' x'), fingerprint('meaning','x'));
  assert.notEqual(fingerprint('meaning','x'), fingerprint('contract','x'));
  assert.throws(() => canonical({ a:undefined })); assert.throws(() => canonical(1.5));
});
test('change set includes removed anchors, not unchanged dependencies', () => {
  assert.deepEqual(changedAnchors({ meaning:'a', contract:'b', removed:'c' }, { meaning:'d', contract:'b' }), ['meaning','removed']);
  assert.equal(anchorsCurrent({ contract:'b' }, { meaning:'d', contract:'b' }), true);
  assert.equal(anchorsCurrent({ contract:'b' }, {}), false); assert.equal(anchorsCurrent({}, {}), false);
});
test('planning requires authorized verified fresh claim-specific evidence', () => {
  const c={ origin:'observed', state:'verified', conflict:false, anchors:{ body:'a' } }, ok={ authorized:true, evidenceChecked:true };
  assert.equal(planningClaim(c,{ body:'a' },ok),true);
  for(const patch of [{origin:'inferred'},{state:'attested'},{conflict:true}]) assert.equal(planningClaim({...c,...patch},{body:'a'},ok),false);
  assert.equal(planningClaim(c,{body:'b'},ok),false); assert.equal(planningClaim(c,{body:'a'},{}),false);
});
test('round-trip and digest cover exact observable plus manifest and versions', () => {
  // Test adapters only; production must use the actual ARBOR compiler/renderer.
  const p={ observable:{ actionId:'wiki.edit',args:{content:'A\n"ñ"'} }, manifest:{revision:1}, versions, render:JSON.stringify, parse:JSON.parse };
  const a=previewBinding(p); assert.notEqual(a.bindingDigest,previewBinding({...p,manifest:{revision:2}}).bindingDigest);
  assert.notEqual(a.bindingDigest,previewBinding({...p,versions:{...versions,policy:2}}).bindingDigest);
  assert.throws(()=>previewBinding({...p,parse:()=>({})}),/ROUND_TRIP/); assert.throws(()=>previewBinding({...p,versions:{}}));
});
test('command reuse misses on dependency or scope change', () => {
  const p={scope:{owner:'owner-A'},template:'wiki.edit',parameters:{content:'string'},versions};
  assert.notEqual(reusableCommandKey(p),reusableCommandKey({...p,versions:{...versions,contract:2}}));
  assert.notEqual(reusableCommandKey(p),reusableCommandKey({...p,scope:{owner:'owner-B'}}));
  assert.throws(()=>reusableCommandKey({...p,versions:{}}));
});
test('packet refuses to truncate required semantic safety context', () => {
  const p={required:['contract'],optional:['body','x'],render:x=>x.join('|'),measure:x=>x.length,budget:10};
  assert.deepEqual(boundedPacket(p),{outcome:'ready',packet:'contract|x',cost:10});
  assert.deepEqual(boundedPacket({...p,budget:7}),{outcome:'budget_exceeded',minimum:8});
});
test('late draft results never overwrite newer revisions', () => {
  const d={requestId:'a',draftRevision:2,worldRevision:3,catalogRevision:4};
  assert.equal(currentDraft(d,d),true); assert.equal(currentDraft({...d,draftRevision:1},d),false); assert.equal(currentDraft({},{}),false);
});
