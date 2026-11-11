import test from 'node:test';
import assert from 'node:assert/strict';
import {createLabsFixture,createMemoryTestStore} from '../src/ports.mjs';
import {createT0Coordinator} from '../src/t0-core.mjs';
const digest='a'.repeat(64);
const receipt=actionId=>({actionId,verified:true,effectRef:'studio-confirmed-'+actionId});
test('explicit grants, confirmed effects and no blind repeat',()=>{
  const c=createLabsFixture();c.start();
  assert.throws(()=>c.prepare({lane:'export',actionId:'export-1',payloadDigest:digest}),{code:'GRANT_REQUIRED'});
  c.grant({operation:'request_export',approvedByOwner:true});
  c.prepare({lane:'export',actionId:'export-1',payloadDigest:digest});
  c.prepare({lane:'export',actionId:'export-1',payloadDigest:digest});
  assert.throws(()=>c.prepare({lane:'export',actionId:'export-2',payloadDigest:digest}),{code:'LANE_IN_FLIGHT'});
  assert.throws(()=>c.reconcile({actionId:'export-1',outcome:'confirmed'}),{code:'VERIFIED_RECEIPT_REQUIRED'});
  c.reconcile({actionId:'export-1',outcome:'confirmed',receipt:receipt('export-1')});
  assert.equal(c.status().lanes.export.status,'complete');
  assert.throws(()=>c.reconcile({actionId:'export-1',outcome:'uncertain'}),{code:'RECONCILIATION_REQUIRED'});
});
test('account scoped, durable reload and duplicate-safe batches',()=>{
  const store=createMemoryTestStore();
  const args={store,worldId:'owner-A',accountId:'account-A',now:()=>Date.UTC(2026,10,11)};
  const c=createT0Coordinator(args);c.start();c.grant({operation:'import_archive',approvedByOwner:true});
  const input={lane:'archive',sourceId:'zip-1',batchId:'batch-1',sourceRevision:'r1',acceptedByHost:true,items:[{nativeId:'thread-1',contentDigest:digest,kind:'conversation'}]};
  c.recordBatch(input);c.recordBatch(input);
  assert.equal(c.status().evidenceCount,1);
  assert.equal(createT0Coordinator(args).status().evidenceCount,1);
  assert.equal(createT0Coordinator({...args,accountId:'account-B'}).status(),null);
  assert.throws(()=>c.recordBatch({...input,items:[{...input.items[0],nativeId:'thread-2'}]}),{code:'BATCH_CONFLICT'});
});
test('revocation denies new effects while verified historical truth resolves uncertainty',()=>{
  const c=createLabsFixture();c.start();c.grant({operation:'read_profile',approvedByOwner:true});
  c.prepare({lane:'profile',actionId:'p1',payloadDigest:digest});c.revoke('read_profile');
  assert.equal(c.status().actions.p1.status,'uncertain');
  assert.equal(c.status().lanes.profile.status,'revoked');
  // Frozen contract (PREPARATION.md:56): a Host-verified receipt records historical
  // effect truth for uncertain actions even when the grant is revoked; the revoked
  // lane keeps its permission state and no new effect is authorized by resolution.
  const resolved=c.reconcile({actionId:'p1',outcome:'confirmed',receipt:receipt('p1')});
  assert.equal(resolved.status,'complete');
  assert.equal(resolved.receipt.effectRef,receipt('p1').effectRef);
  assert.equal(c.status().lanes.profile.status,'revoked');
  assert.throws(()=>c.prepare({lane:'profile',actionId:'p2',payloadDigest:digest}),{code:'GRANT_REQUIRED'});
  assert.throws(()=>c.recordBatch({lane:'profile',sourceId:'x',batchId:'1',sourceRevision:'1',acceptedByHost:true,items:[]}),{code:'GRANT_REQUIRED'});
  // Idempotent replay of the same complete action and effectRef stays accepted.
  assert.equal(c.reconcile({actionId:'p1',outcome:'confirmed',receipt:receipt('p1')}).status,'complete');
  assert.throws(()=>c.reconcile({actionId:'p1',outcome:'confirmed',receipt:{actionId:'p1',verified:true,effectRef:'different-ref'}}),{code:'RECONCILIATION_REQUIRED'});
});
test('source acceptance and bounds enforced',()=>{
  const c=createLabsFixture();c.start();c.grant({operation:'import_archive',approvedByOwner:true});
  assert.throws(()=>c.recordBatch({lane:'archive',sourceId:'x',batchId:'1',sourceRevision:'r',items:[]}),{code:'HOST_ACCEPTANCE_REQUIRED'});
  assert.throws(()=>c.recordBatch({lane:'archive',sourceId:'x',batchId:'1',sourceRevision:'r',acceptedByHost:true,items:Array.from({length:257},()=>({nativeId:'x',kind:'thread',contentDigest:digest}))}),{code:'BATCH_LIMIT'});
});
