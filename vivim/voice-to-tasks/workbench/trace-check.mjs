// Lane D: offline acceptance verifier for observed cross-device traces.
// This is NOT a task engine, sync implementation, Android UI, or Host effect writer.
const fail = code => { const e = new Error(code); e.code = code; throw e; };
const ACTIONS = new Set(['captured_local','queued','transcribed','proposed','reviewed','committed','reopened']);
const SOURCES = new Set(['android','desktop','host']);
export function verifyObservedTrace(trace) {
  if (!Array.isArray(trace) || trace.length < 2 || trace.length > 100) fail('TRACE_INVALID');
  const first=trace[0], noteId=first?.noteId, ownerId=first?.ownerId;
  if (typeof noteId !== 'string' || !noteId || typeof ownerId !== 'string' || !ownerId) fail('TRACE_IDENTITY');
  let phase=-1, hasReviewed=false, approved=[], receiptId=null, revision=null, reopened=false;
  const expected=['captured_local','queued','transcribed','proposed','reviewed','committed','reopened'];
  for (const e of trace) {
    if (!e || typeof e !== 'object' || Array.isArray(e) ||
        Object.keys(e).some(k=>!['noteId','ownerId','source','action','revision','approvedProposalIds','receiptId'].includes(k)) ||
        e.noteId!==noteId || e.ownerId!==ownerId || !SOURCES.has(e.source) || !ACTIONS.has(e.action) ||
        !Number.isSafeInteger(e.revision) || e.revision < 1) fail('TRACE_EVENT_INVALID');
    const i=expected.indexOf(e.action);
    if (i < phase || i > phase+1) fail('TRACE_UNAUTHORIZED_TRANSITION');
    if (i===phase && e.action !== 'queued' && e.action !== 'reopened') fail('TRACE_DUPLICATE_EVENT');
    if (i===0 && !['android','desktop'].includes(e.source)) fail('TRACE_CAPTURE_SOURCE');
    if (i===5 && e.source!=='host') fail('TRACE_HOST_COMMIT_REQUIRED');
    if (i===6 && !['android','desktop'].includes(e.source)) fail('TRACE_REOPEN_SOURCE');
    if (phase > -1 && e.revision < revision) fail('TRACE_STALE_REVISION');
    if (i===4) {
      if (!Array.isArray(e.approvedProposalIds) || new Set(e.approvedProposalIds).size!==e.approvedProposalIds.length ||
          !e.approvedProposalIds.every(x=>typeof x==='string'&&x.startsWith('vp-'))) fail('TRACE_REVIEW_INVALID');
      hasReviewed=true; approved=e.approvedProposalIds;
    }
    if (i===5) {
      if (!hasReviewed || !Array.isArray(approved) || typeof e.receiptId!=='string' || !e.receiptId.startsWith('receipt:'))
        fail('TRACE_UNVERIFIED_COMMIT');
      if (receiptId && receiptId!==e.receiptId) fail('TRACE_RECEIPT_CONFLICT');
      receiptId=e.receiptId;
    }
    if (i===6) reopened=true;
    phase=i;revision=e.revision;
  }
  return {classification:'OBSERVED_TRACE_CONTRACT_ONLY',noteId,ownerId,finalAction:expected[phase],
    receiptId,approvedCount:approved.length,reopened,passed:reopened&&Boolean(receiptId)};
}
