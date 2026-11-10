import {test} from 'node:test';
import assert from 'node:assert/strict';
import {projectSupervisorChronos} from '../lib/chronos/supervisor-timeline.mjs';
const time = n => new Date(1700000000000 + n).toISOString();
const evt = (seq,event,laneId,ms,ext={}) => ({seq,event,laneId,attempt:1,ts:time(ms),...ext});

test('Chronos supervisor read projection keeps replay authority and no effect',()=>{
  const events = [evt(1,'ADMITTED','lane-a',0,{idempotencyKey:'scope-a'}),
    evt(2,'STARTING','lane-a',100,{idempotencyKey:'scope-a'}),
    evt(3,'RUNNING','lane-a',200,{idempotencyKey:'scope-a',pid:33}),
    evt(4,'HEARTBEAT','lane-a',350,{idempotencyKey:'scope-a'}),
    evt(5,'EXITED','lane-a',500,{idempotencyKey:'scope-a',exitCode:0})];
  const copy=structuredClone(events);
  const out=projectSupervisorChronos(events,{asOfMs:1700000001000});
  assert.equal(out.schema,'vivim.chronos.supervisor-read/1');
  assert.equal(out.lanes[0].elapsedMs,400);
  assert.equal(out.lanes[0].phase,'completed');
  assert.equal(out.effectAllowed,false);
  assert.equal(out.dispatchAllowed,false);
  assert.deepEqual(events,copy);
  assert.equal(JSON.stringify(out).includes('scope-a'),false);
});

test('Chronos refuses corrupt replay sequence and invalid timestamps',()=>{
  const events=[evt(1,'ADMITTED','lane-a',0),evt(3,'RUNNING','lane-a',1)];
  assert.throws(()=>projectSupervisorChronos(events,{asOfMs:1700000001000}),/sequence|seq|out of order/i);
  assert.throws(()=>projectSupervisorChronos([evt(1,'ADMITTED','lane-a',0,{ts:'not-a-time'})],{asOfMs:1700000001000}),/INVALID_EVENT_TIME/);
});

test('Chronos future observation exclusion does not fabricate active duration',()=>{
  const events=[evt(1,'ADMITTED','lane-a',0),evt(2,'STARTING','lane-a',100),evt(3,'EXITED','lane-a',500,{exitCode:0})];
  const v=projectSupervisorChronos(events,{asOfMs:1700000000300});
  assert.equal(v.excludedFutureObservations,1);
  assert.equal(v.lanes[0].phase,'running');
  assert.equal(v.lanes[0].elapsedMs,200);
});

test('clock regression is visible rather than converted into authoritative duration',()=>{
  const events=[evt(1,'STARTING','lane-a',500),evt(2,'RUNNING','lane-a',100),evt(3,'EXITED','lane-a',600,{exitCode:0})];
  const v=projectSupervisorChronos(events,{asOfMs:1700000001000});
  assert.equal(v.lanes[0].clockQuality,'wall_clock_regression');
  assert.equal(v.lanes[0].elapsedMs,null);
});
