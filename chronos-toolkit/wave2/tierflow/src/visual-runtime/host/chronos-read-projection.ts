/** TierFlow consumes Chronos read models only; the Host owns time/effects/auth.
 * This is a defensive presentation projection, NOT a scheduler or permission check.
 * The caller must have authenticated the Host read before passing its reply here.
 */
export const CHRONOS_VISUAL_SCHEMA = 'vivim.chronos.visual-read/1' as const;
export type ChronosClockQuality = 'wall_clock_unverified' | 'ordered' | 'missing_timestamp' | 'wall_clock_regression';
export type ChronosConversationRead = Readonly<{
  schema:'vivim.chronos.conversation-read/1'; conversationId:string; sourceRevision:number;
  asOfMs:number; createdAtMs:number|null; updatedAtMs:number|null;
  clockQuality:ChronosClockQuality; lifecycleState:string; responseLatencyMs:null;
  effectAllowed:false; automaticRetryAllowed:false;
  turns:readonly Readonly<{id:string;phase:string;startedAtMs:number|null;
    lastUpdatedAtMs:number|null;wallOpenMs:number|null;authoritativeSendReceipt:false}>[];
}>;
export type ChronosSupervisorRead = Readonly<{
  schema:'vivim.chronos.supervisor-read/1'; observedAtMs:number; sourceLastSeq:number;
  durationAuthority:'wall_clock_estimate_only'; effectAllowed:false;dispatchAllowed:false;
  lanes:readonly Readonly<{laneId:string;attempt:number;phase:string;
    clockQuality:'ordered'|'wall_clock_regression';elapsedMs:number|null;lastLedgerSeq:number}>[];
}>;
export type ChronosVisualRead = Readonly<{
  schema:typeof CHRONOS_VISUAL_SCHEMA;
  source:'host-conversation'|'hq-supervisor';sourceRevision:number;
  status:'observed'|'degraded'; clockQuality:string;
  durationAuthority:'source_wall_clock_estimate_only'|'unavailable';
  elapsedMs:number|null; items:readonly Readonly<{kind:'turn'|'lane';phase:string;
    elapsedMs:number|null;clockQuality:string}>[];
  effectInvoked:false;canExecute:false;automaticRetryAllowed:false;
}>;
function record(x:unknown):x is Record<string,unknown>{
  return x !== null && typeof x === 'object' && !Array.isArray(x) &&
    (Object.getPrototypeOf(x) === Object.prototype || Object.getPrototypeOf(x) === null) &&
    Object.values(Object.getOwnPropertyDescriptors(x)).every(d=>'value' in d);
}
const safeMs=(v:unknown):v is number=>Number.isSafeInteger(v)&&typeof v==='number'&&v>=0;
const safeString=(v:unknown):v is string=>typeof v==='string'&&v.length>0&&v.length<=160&&
  !/[\r\n\u0000]/.test(v);
const safeOptionalMs=(v:unknown)=>v===null||safeMs(v);

/** Validate untrusted read model before projecting. No user/device IDs go to display. */
export function projectChronosConversationRead(input:unknown):ChronosVisualRead {
  if(!record(input)||input.schema!=='vivim.chronos.conversation-read/1'||
     !safeString(input.conversationId)||!safeMs(input.sourceRevision)||
     !safeMs(input.asOfMs)||!safeOptionalMs(input.createdAtMs)||!safeOptionalMs(input.updatedAtMs)||
     !safeString(input.clockQuality)||!safeString(input.lifecycleState)||
     input.effectAllowed!==false||input.automaticRetryAllowed!==false||input.responseLatencyMs!==null||
     !Array.isArray(input.turns)||input.turns.length>512) throw new Error('CHRONOS_UNTRUSTED_CONVERSATION_READ');
  const items=input.turns.map(turn=>{
    if(!record(turn)||!safeString(turn.id)||!safeString(turn.phase)||
      !safeOptionalMs(turn.startedAtMs)||!safeOptionalMs(turn.lastUpdatedAtMs)||
      !safeOptionalMs(turn.wallOpenMs)||turn.authoritativeSendReceipt!==false)
      throw new Error('CHRONOS_UNTRUSTED_TURN');
    const valid=turn.wallOpenMs===null ||
      (turn.startedAtMs!==null && turn.lastUpdatedAtMs!==null &&
       turn.lastUpdatedAtMs>=turn.startedAtMs && turn.wallOpenMs===turn.lastUpdatedAtMs-turn.startedAtMs);
    if(!valid) throw new Error('CHRONOS_DURATION_INCONSISTENT');
    return Object.freeze({kind:'turn' as const,phase:turn.phase,elapsedMs:turn.wallOpenMs,
      clockQuality:input.clockQuality as string});
  });
  const degraded=input.clockQuality!=='wall_clock_unverified';
  return Object.freeze({schema:CHRONOS_VISUAL_SCHEMA,source:'host-conversation',sourceRevision:input.sourceRevision,
    status:degraded?'degraded':'observed',clockQuality:input.clockQuality,
    durationAuthority:degraded?'unavailable':'source_wall_clock_estimate_only',
    elapsedMs:input.createdAtMs!==null && input.updatedAtMs!==null &&
      input.updatedAtMs>=input.createdAtMs && !degraded ? input.updatedAtMs-input.createdAtMs : null,
    items:Object.freeze(items),effectInvoked:false,canExecute:false,automaticRetryAllowed:false});
}

export function projectChronosSupervisorRead(input:unknown):ChronosVisualRead {
  if(!record(input)||input.schema!=='vivim.chronos.supervisor-read/1'||
    !safeMs(input.observedAtMs)||!safeMs(input.sourceLastSeq)||
    input.durationAuthority!=='wall_clock_estimate_only'||input.effectAllowed!==false||
    input.dispatchAllowed!==false||!Array.isArray(input.lanes)||input.lanes.length>2000)
    throw new Error('CHRONOS_UNTRUSTED_SUPERVISOR_READ');
  let degraded=false;
  const items=input.lanes.map(lane=>{
    if(!record(lane)||!safeString(lane.laneId)||!safeMs(lane.attempt)||lane.attempt===0||
      !safeString(lane.phase)||!['ordered','wall_clock_regression'].includes(String(lane.clockQuality))||
      !safeOptionalMs(lane.elapsedMs)||!safeMs(lane.lastLedgerSeq))
      throw new Error('CHRONOS_UNTRUSTED_LANE');
    if(lane.clockQuality!=='ordered') degraded=true;
    if(lane.clockQuality==='wall_clock_regression'&&lane.elapsedMs!==null)
      throw new Error('CHRONOS_DURATION_INCONSISTENT');
    return Object.freeze({kind:'lane' as const,phase:lane.phase,elapsedMs:lane.elapsedMs,
      clockQuality:lane.clockQuality as string});
  });
  return Object.freeze({schema:CHRONOS_VISUAL_SCHEMA,source:'hq-supervisor',sourceRevision:input.sourceLastSeq,
    status:degraded?'degraded':'observed',clockQuality:degraded?'wall_clock_regression':'ordered',
    durationAuthority:degraded?'unavailable':'source_wall_clock_estimate_only',elapsedMs:null,
    items:Object.freeze(items),effectInvoked:false,canExecute:false,automaticRetryAllowed:false});
}
