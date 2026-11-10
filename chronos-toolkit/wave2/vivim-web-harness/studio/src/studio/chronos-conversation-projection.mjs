/**
 * Read-only Chronos adapter over the canonical conversation snapshot.
 * No browser actions, polling, timers, effects or fake message-send receipts.
 * Timestamps are copied only when explicitly present in the source snapshot.
 * Sensitive fields (prompts, replies, urls, tokens) are never included.
 */
export const CHRONOS_CONVERSATION_SCHEMA = 'vivim.chronos.conversation-read/1';
const MAX_TURNS = 512;
const ref = x => typeof x === 'string' && x.length > 0 && x.length <= 160 && !/[\r\n\0]/.test(x);
function instant(value) {
  if (typeof value !== 'string' || value.length > 40 || !/^\d{4}-\d{2}-\d{2}T/.test(value)) return null;
  const at = Date.parse(value);
  return Number.isSafeInteger(at) && new Date(at).toISOString() === new Date(value).toISOString() ? at : null;
}
function elapsed(start, end) { return start === null || end === null || end < start ? null : end - start; }

export function projectConversationChronos(view, { asOfMs } = {}) {
  if (!view || typeof view !== 'object' || !ref(view.id) ||
      !Number.isSafeInteger(view.revision) || view.revision < 0 ||
      !Number.isSafeInteger(asOfMs) || !Array.isArray(view.turns) || view.turns.length > MAX_TURNS)
    throw new Error('CHRONOS_CONVERSATION_SOURCE_INVALID');
  const createdAtMs = instant(view.createdAt), updatedAtMs = instant(view.updatedAt);
  const sourceClockQuality = createdAtMs === null || updatedAtMs === null ? 'missing_timestamp' :
    updatedAtMs < createdAtMs || asOfMs < createdAtMs ? 'wall_clock_regression' : 'wall_clock_unverified';
  const turns = view.turns.map(turn => {
    if (!turn || !ref(turn.id)) throw new Error('CHRONOS_TURN_SOURCE_INVALID');
    const startedAtMs = instant(turn.createdAt);
    const lastUpdatedAtMs = instant(turn.updatedAt);
    const typingStartedAtMs = instant(turn.typingStartedAt);
    return Object.freeze({ id:turn.id, phase:typeof turn.phase === 'string' ? turn.phase.slice(0,48) : 'unknown',
      startedAtMs, typingStartedAtMs, lastUpdatedAtMs,
      wallOpenMs:elapsed(startedAtMs,lastUpdatedAtMs),
      wallClockOnly:true, authoritativeSendReceipt:false });
  });
  return Object.freeze({schema:CHRONOS_CONVERSATION_SCHEMA,
    conversationId:view.id, sourceRevision:view.revision,
    source:'host:conversation-snapshot', asOfMs,
    createdAtMs, updatedAtMs, elapsedSinceCreationMs:elapsed(createdAtMs,asOfMs),
    clockQuality:sourceClockQuality,
    lifecycleState:typeof view.state === 'string' ? view.state.slice(0,48) : 'unknown',
    // Browser/reply timing requires independent source evidence; don't fake it from updatedAt.
    responseLatencyMs:null, activeTimeMs:null, effectAllowed:false, automaticRetryAllowed:false,
    turns:Object.freeze(turns) });
}
