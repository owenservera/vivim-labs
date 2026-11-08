import {test} from 'node:test';
import assert from 'node:assert/strict';
import {projectConversationChronos} from '../../src/studio/chronos-conversation-projection.mjs';
const at = ms => new Date(1700000000000+ms).toISOString();
function view() {return {id:'convo-a',revision:3,state:'ready',createdAt:at(0),updatedAt:at(500),
  prompt:'SECRET_PRIVATE_PROMPT',reply:{text:'SECRET_PRIVATE_REPLY'},
  turns:[{id:'turn-a',createdAt:at(50),typingStartedAt:at(100),updatedAt:at(400),phase:'ready',
    reply:{text:'SECRET_PRIVATE_REPLY'},send:{evidence:{token:'SECRET_TOKEN'}}}]};}

test('conversation lifecycle read retains source times, no secret bodies or asserted send',()=>{
  const v=view(), original=structuredClone(v);
  const out=projectConversationChronos(v,{asOfMs:1700000001000});
  assert.equal(out.schema,'vivim.chronos.conversation-read/1');
  assert.equal(out.elapsedSinceCreationMs,1000);
  assert.equal(out.turns[0].wallOpenMs,350);
  assert.equal(out.responseLatencyMs,null);
  assert.equal(out.effectAllowed,false);
  assert.equal(out.turns[0].authoritativeSendReceipt,false);
  for(const secret of ['SECRET_PRIVATE_PROMPT','SECRET_PRIVATE_REPLY','SECRET_TOKEN'])
    assert.equal(JSON.stringify(out).includes(secret),false);
  assert.deepEqual(v,original);
});

test('missing timestamps stay missing rather than inventing start/stop',()=>{
  const v=view(); delete v.createdAt; delete v.turns[0].updatedAt;
  const out=projectConversationChronos(v,{asOfMs:1700000001000});
  assert.equal(out.clockQuality,'missing_timestamp');
  assert.equal(out.elapsedSinceCreationMs,null);
  assert.equal(out.turns[0].wallOpenMs,null);
});

test('clock regressions and malformed source are not promoted',()=>{
  const v=view(); v.updatedAt=at(-100);
  assert.equal(projectConversationChronos(v,{asOfMs:1700000001000}).clockQuality,'wall_clock_regression');
  assert.throws(()=>projectConversationChronos({...v,turns:new Array(513).fill({id:'x'})},{asOfMs:1700000001000}),/SOURCE_INVALID/);
  assert.throws(()=>projectConversationChronos(v,{asOfMs:NaN}),/SOURCE_INVALID/);
});
