import {test} from 'node:test';
import {strict as assert} from 'node:assert';
import {projectChronosConversationRead,projectChronosSupervisorRead} from '../../src/visual-runtime/host/chronos-read-projection.js';
const base={schema:'vivim.chronos.conversation-read/1',conversationId:'secret-identifier',sourceRevision:7,
 asOfMs:2000,createdAtMs:1000,updatedAtMs:1500,clockQuality:'wall_clock_unverified',lifecycleState:'ready',
 responseLatencyMs:null,effectAllowed:false,automaticRetryAllowed:false,
 turns:[{id:'turn-1',phase:'ready',startedAtMs:1100,lastUpdatedAtMs:1400,wallOpenMs:300,authoritativeSendReceipt:false,
 prompt:'NEVER-DISPLAY-PROMPT'}]};
test('conversation read turns into redacted inert UI data',()=>{
 const out=projectChronosConversationRead(base);
 assert.equal(out.elapsedMs,500);assert.equal(out.items[0].elapsedMs,300);
 assert.equal(out.canExecute,false);assert.equal(out.automaticRetryAllowed,false);
 assert.equal(JSON.stringify(out).includes('NEVER-DISPLAY-PROMPT'),false);
 assert.equal(JSON.stringify(out).includes('secret-identifier'),false);
});
test('forged or mismatched time is refused',()=>{
 assert.throws(()=>projectChronosConversationRead({...base,effectAllowed:true}),/UNTRUSTED/);
 assert.throws(()=>projectChronosConversationRead({...base,turns:[{...base.turns[0],wallOpenMs:999}]}),/INCONSISTENT/);
});
test('supervisor projection never elevates a duration to authority',()=>{
 const input={schema:'vivim.chronos.supervisor-read/1',observedAtMs:2000,sourceLastSeq:12,
  durationAuthority:'wall_clock_estimate_only',effectAllowed:false,dispatchAllowed:false,
  lanes:[{laneId:'private-id',attempt:1,phase:'completed',clockQuality:'ordered',elapsedMs:200,lastLedgerSeq:12}]};
 const out=projectChronosSupervisorRead(input);
 assert.equal(out.source,'hq-supervisor');assert.equal(out.items[0].elapsedMs,200);
 assert.equal(out.effectInvoked,false);
 assert.equal(JSON.stringify(out).includes('private-id'),false);
 assert.throws(()=>projectChronosSupervisorRead({...input,dispatchAllowed:true}),/UNTRUSTED/);
});
