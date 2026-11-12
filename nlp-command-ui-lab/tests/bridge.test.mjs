import test from 'node:test';
import assert from 'node:assert/strict';
import { collectInventory,getContacts } from '../src/bridge.mjs';
test('contacts are bounded and fixture is explicitly labelled',()=>{
 const x=getContacts('Pet');assert.equal(x.origin,'fictional_demo_fixture');assert.equal(x.contacts.length,2);
 for(const c of x.contacts)assert.equal(c.verified,false);
});
test('missing canonical registries produce gaps rather than invented records',async()=>{
 const x=await collectInventory();assert.equal(x.schema,'vivim.labs.command-inventory/1');assert.ok(Array.isArray(x.actions));
 assert.ok(x.gaps.some(g=>g.code==='HQ_CROSSWALK_GAP'));
 assert.ok(x.gaps.some(g=>g.code==='AVAILABILITY_EVIDENCE_GAP'));
});