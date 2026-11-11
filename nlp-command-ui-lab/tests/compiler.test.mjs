import test from 'node:test';
import assert from 'node:assert/strict';
import { annotate,visualCompile } from '../src/compiler.mjs';
const inventory={actions:[],revision:'fixture'};
test('email opens contact menu immediately, without inventing action',()=>{
 const p=visualCompile('email ',inventory);
 assert.equal(p.outcome,'slot_required');assert.equal(p.namespace,'prototype');assert.equal(p.actionId,null);
 assert.equal(p.menu.kind,'contacts');assert.equal(p.executionPerformed,false);assert.ok(p.gaps.some(x=>x.code==='UNREGISTERED_EMAIL'));
});
test('typing recipient narrows matches and Tab span accurately',()=>{
 const p=visualCompile('email Pet',inventory);assert.equal(p.menu.query,'Pet');assert.deepEqual(p.menu.items.map(x=>x.label),['Peter Miller','Petra Ruiz']);
 assert.equal(p.text.slice(p.menu.replace.start,p.menu.replace.end),'Pet');
});
test('one fictional demo match still never creates live identity',()=>{
 const p=visualCompile('email Peter Miller',inventory);assert.equal(p.menu.origin,'fictional_demo_fixture');assert.equal(p.outcome,'visual_candidate');assert.equal(p.executionPerformed,false);
 assert.ok(p.gaps.some(x=>x.code==='DEMO_CONTACTS'));
});
test('Host result supplies authoritative id and readiness separately from execution',()=>{
 const inv={actions:[{id:'vivim.status.show',namespace:'host',title:'Status',category:'system',risk:'R0',readiness:'source_wired',liveVerified:false,args:{},examples:['show status']}]};
 const p=visualCompile('show status',inv,{outcome:'ready',actionId:'vivim.status.show',language:{feedback:{reading:'Show VIVIM status'}}});
 assert.equal(p.actionId,'vivim.status.show');assert.equal(p.provenance,'host.intent-planner.planText');assert.equal(p.readiness,'source_wired');assert.equal(p.executionPerformed,false);
 assert.ok(p.gaps.some(x=>x.code==='NOT_LIVE_VERIFIED'));
});
test('unrecognized text without Host planner is not falsely interpreted',()=>{
 const p=visualCompile('delete everything',inventory);assert.equal(p.outcome,'source_unavailable');assert.equal(p.actionId,null);assert.equal(p.executionPerformed,false);
});
test('token positions preserve original text',()=>{
 const text='email "Peter Miller" → subject';const tokens=annotate(text);
 assert.ok(tokens.some(t=>t.text==='"Peter Miller"'));
 for(const t of tokens)assert.equal(text.slice(t.start,t.end),t.text);
});