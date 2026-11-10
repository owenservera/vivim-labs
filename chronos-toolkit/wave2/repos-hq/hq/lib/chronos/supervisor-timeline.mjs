/**
 * Chronos read-only projection of the EXISTING HQ supervisor event ledger.
 * Does not create a scheduler, change admission, infer live-pid status or grant effects.
 * Durable event order is validated by the canonical supervisor replay implementation.
 */
import { replayEvents } from '../../supervisor/recovery.mjs';

export const CHRONOS_SUPERVISOR_SCHEMA = 'vivim.chronos.supervisor-read/1';
const STARTS = new Set(['STARTING', 'RESUMED']);
const ENDS = new Set(['EXPIRED', 'FENCED', 'QUARANTINED', 'FAILED', 'EXITED']);
const MAX_EVENTS = 10000;
const MAX_RESULT_LANES = 2000;

function instant(ts) {
  if (typeof ts !== 'string' || ts.length > 40 || !/^\d{4}-\d\d-\d\dT/.test(ts)) return null;
  const ms = Date.parse(ts);
  return Number.isSafeInteger(ms) ? ms : null;
}
function scopeId(id) {
  return typeof id === 'string' && id.length > 0 && id.length <= 160 && !/[\r\n\0]/.test(id);
}

/**
 * @param {Array<object>} events Existing supervisor ledger records, in canonical append order.
 * @param {{asOfMs:number, maxLanes?:number}} options Caller-supplied observation boundary (injection).
 */
export function projectSupervisorChronos(events, { asOfMs, maxLanes = MAX_RESULT_LANES } = {}) {
  if (!Array.isArray(events) || events.length > MAX_EVENTS ||
      !Number.isSafeInteger(asOfMs) || !Number.isInteger(maxLanes) || maxLanes < 1 || maxLanes > MAX_RESULT_LANES)
    throw new Error('CHRONOS_INVALID_READ_BOUNDS');
  // Reuse existing ledger validation and idempotency semantics. Never invent a status.
  const replay = replayEvents(events);
  const lanes = new Map();
  let excluded = 0;
  for (const event of events) {
    const at = instant(event.ts);
    if (at === null) throw new Error('CHRONOS_INVALID_EVENT_TIME');
    if (at > asOfMs) { excluded++; continue; }
    if (!scopeId(event.laneId) || !Number.isSafeInteger(Number(event.attempt)) || Number(event.attempt) < 1) continue;
    const attempt = Number(event.attempt);
    const key = `${event.laneId.length}:${event.laneId}:${attempt}`;
    let lane = lanes.get(key);
    if (!lane) {
      if (lanes.size >= maxLanes) throw new Error('CHRONOS_READ_CAPACITY');
      lane = { laneId:event.laneId, attempt, firstSeenAtMs:at, lastObservedAtMs:at,
        startedAtMs:null, endedAtMs:null, elapsedMs:null, phase:'unknown', lastLedgerSeq:0,
        clockQuality:'ordered', evidenceClass:'supervisor_event_ledger' };
      lanes.set(key,lane);
    }
    if (at < lane.lastObservedAtMs) lane.clockQuality = 'wall_clock_regression';
    lane.lastObservedAtMs = at;
    lane.lastLedgerSeq = event.seq;
    if (STARTS.has(event.event) && lane.startedAtMs === null) {
      lane.startedAtMs = at; lane.phase = 'running';
    }
    if (ENDS.has(event.event)) {
      lane.endedAtMs = at;
      lane.phase = event.event === 'EXITED' && Number(event.exitCode) === 0 ? 'completed' :
        event.event === 'QUARANTINED' ? 'quarantined' : 'closed_unverified';
    }
  }
  const result = [...lanes.values()].map(lane => {
    if (lane.startedAtMs !== null && lane.endedAtMs !== null && lane.clockQuality === 'ordered' && lane.endedAtMs >= lane.startedAtMs)
      lane.elapsedMs = lane.endedAtMs - lane.startedAtMs;
    else if (lane.startedAtMs !== null && lane.endedAtMs === null && lane.clockQuality === 'ordered' && asOfMs >= lane.startedAtMs)
      lane.elapsedMs = asOfMs - lane.startedAtMs;
    // Wall-clock-based elapsed is approximate and never used for lease enforcement.
    return Object.freeze(lane);
  });
  return Object.freeze({ schema:CHRONOS_SUPERVISOR_SCHEMA, observedAtMs:asOfMs,
    source:'hq:supervisor/events', sourceLastSeq:replay.lastSeq,
    excludedFutureObservations:excluded, durationAuthority:'wall_clock_estimate_only',
    effectAllowed:false, dispatchAllowed:false, lanes:Object.freeze(result) });
}
