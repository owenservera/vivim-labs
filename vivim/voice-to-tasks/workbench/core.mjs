// VIVIM Labs: PURE, OFFLINE evaluation reference. No files, network, microphone,
// model downloads, provider calls, Host authority, or task writes occur here.
import { createHash } from 'node:crypto';

const fail = code => { const err = new Error(code); err.code = code; throw err; };
const isRecord = x => x !== null && typeof x === 'object' && !Array.isArray(x);
const exactKeys = (value, allowed) => isRecord(value) && Object.keys(value).every(k => allowed.includes(k));
const nonempty = (v, max = 200) => typeof v === 'string' && v.length > 0 && v.length <= max && v.trim() === v;
const digest = v => createHash('sha256').update(String(v), 'utf8').digest('hex');
const HEX64 = /^[a-f0-9]{64}$/;
const NOTE_ID = /^[a-z][a-z0-9:-]{7,100}$/;
const RFC3339 = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})$/;

// Lane A: metadata-only audio receipt, not an invitation to transmit audio.
export function checkCapture(capture) {
  if (!exactKeys(capture, ['schema','noteId','ownerId','deviceId','sha256','mime','bytes','capturedAt','locale','timeZone','consentToHostTransfer']) ||
      capture.schema !== 'vivim.lab-audio-capture/1' || !NOTE_ID.test(capture.noteId ?? '') ||
      !nonempty(capture.ownerId, 120) || !nonempty(capture.deviceId, 120) ||
      !HEX64.test(capture.sha256 ?? '') || capture.mime !== 'audio/mp4' ||
      !Number.isSafeInteger(capture.bytes) || capture.bytes < 64 || capture.bytes > 25 * 1024 * 1024 ||
      typeof capture.capturedAt !== 'string' || !RFC3339.test(capture.capturedAt) ||
      !Number.isFinite(Date.parse(capture.capturedAt)) ||
      !/^[a-z]{2,3}(?:-[A-Z]{2})?$/.test(capture.locale ?? '') ||
      !nonempty(capture.timeZone, 100) ||
      typeof capture.consentToHostTransfer !== 'boolean') fail('CAPTURE_INVALID');
  try { new Intl.DateTimeFormat('en-US',{timeZone:capture.timeZone}); } catch { fail('TIMEZONE_INVALID'); }
  return Object.freeze({ ...capture });
}

export function checkTranscript(record, capture) {
  checkCapture(capture);
  if (!exactKeys(record, ['schema','noteId','audioSha256','text','engineDigest','status']) ||
      record.schema !== 'vivim.lab-transcript/1' || record.status !== 'draft' ||
      record.noteId !== capture.noteId || record.audioSha256 !== capture.sha256 ||
      typeof record.text !== 'string' || record.text.length > 20000 ||
      !HEX64.test(record.engineDigest ?? '')) fail('TRANSCRIPT_INVALID');
  return Object.freeze({ ...record });
}

// Lane B: deterministic *date intention*, intentionally no dueInstant invention.
function localCalendarDate(instant, timeZone) {
  const parts = Object.fromEntries(new Intl.DateTimeFormat('en-US', {timeZone,
    year:'numeric',month:'2-digit',day:'2-digit'}).formatToParts(new Date(instant))
    .filter(x => ['year','month','day'].includes(x.type)).map(x=>[x.type,Number(x.value)]));
  const d = new Date(Date.UTC(parts.year, parts.month-1, parts.day));
  if (Number.isNaN(d.valueOf())) fail('REFERENCE_INSTANT_INVALID');
  return d;
}
export function interpretDuePhrase(phrase, { referenceInstant, timeZone, locale } = {}) {
  if (!nonempty(referenceInstant, 64) || !RFC3339.test(referenceInstant) ||
      !Number.isFinite(Date.parse(referenceInstant)) || !nonempty(timeZone,100) ||
      !nonempty(locale, 15)) fail('DATE_CONTEXT_INVALID');
  try { new Intl.DateTimeFormat('en-US',{timeZone}); } catch { fail('TIMEZONE_INVALID'); }
  if (phrase === null || phrase === undefined) return {kind:'none',dueDate:null,dueInstant:null};
  if (!nonempty(phrase,100)) fail('DUE_PHRASE_INVALID');
  const p = phrase.toLocaleLowerCase(locale).normalize('NFC');
  let days = null;
  if (/^(tomorrow|mañana|demà)$/.test(p)) days = 1;
  else if (/^(today|hoy|avui)$/.test(p)) days = 0;
  else if (/^(day after tomorrow|pasado mañana|demà passat)$/.test(p)) days = 2;
  if (days === null) return {kind:'clarification_required',dueDate:null,dueInstant:null,phrase};
  const d = localCalendarDate(referenceInstant, timeZone);
  d.setUTCDate(d.getUTCDate()+days);
  return {kind:'calendar_date',dueDate:d.toISOString().slice(0,10),dueInstant:null,phrase,timeZone};
}

const VERBS = {
  en: /^(?:please\s+)?(?:email|send|call|check|review|pay|book|buy|order|schedule|finish|submit|fix|clean|contact|write|prepare|remind me to|i need to)\b/i,
  es: /^(?:por favor\s+)?(?:enviar|envía|manda|mandar|llamar|llama|revisar|revisa|pagar|paga|reservar|reserva|comprar|compra|preparar|prepara|contactar|contacta|tengo que)\b/i,
  ca: /^(?:si us plau\s+)?(?:enviar|envia|trucar|truca|revisar|revisa|pagar|paga|reservar|reserva|comprar|compra|preparar|prepara|he de)\b/i
};
const NEGATED = /^(?:(?:do not|don't|never|no|not|don't forget not to|no hagas|no facis|no envi[eï]s|no enviar)\b)/i;
const DUE = /\b((?:day after tomorrow|pasado mañana|demà passat|tomorrow|today|mañana|demà|hoy|avui)(?:\s+(?:morning|afternoon|evening|por la mañana|por la tarde|al matí))?|next\s+\w+|by\s+\w+|el\s+viernes|divendres)\b/iu;
function segments(text) {
  const result=[]; const re=/[^;.!?\n]+/g; let m;
  while ((m=re.exec(text)) !== null) {
    const s=m[0], left=s.length-s.trimStart().length, right=s.trimEnd().length;
    if (right > left) result.push({start:m.index+left,end:m.index+right,text:s.slice(left,right)});
  }
  return result;
}
// Narrow transparent baseline: NOT a generative substitute or production NLP.
export function baselineProposals(transcript, capture) {
  checkTranscript(transcript,capture);
  const verb = VERBS[capture.locale.split('-')[0]];
  if (!verb) return [];
  return segments(transcript.text).filter(s=>verb.test(s.text) && !NEGATED.test(s.text)).map(s=>{
    const dm=DUE.exec(s.text); const duePhrase=dm?.[0] ?? null;
    const due=interpretDuePhrase(duePhrase,{referenceInstant:capture.capturedAt,timeZone:capture.timeZone,locale:capture.locale});
    return {proposalId:'vp-'+digest([capture.noteId,s.start,s.end].join('\0')).slice(0,24),
      title:s.text,sourceSpan:[s.start,s.end],evidence:s.text,duePhrase,
      dueIntent:due,priority:null,status:'review_required'};
  });
}

// Lane C: untrusted model candidates must be source-bound. No unreviewed effects.
export function checkProposals(proposals, transcript, capture) {
  checkTranscript(transcript,capture);
  if (!Array.isArray(proposals) || proposals.length > 50) fail('PROPOSALS_INVALID');
  const seen=new Set();
  return proposals.map(p=>{
    if (!exactKeys(p,['proposalId','title','sourceSpan','evidence','duePhrase','dueIntent','priority','status']) ||
        !/^vp-[a-f0-9]{24}$/.test(p.proposalId??'') || seen.has(p.proposalId) ||
        !nonempty(p.title,500) || !Array.isArray(p.sourceSpan) || p.sourceSpan.length!==2 ||
        !p.sourceSpan.every(Number.isSafeInteger) || p.sourceSpan[0]<0 || p.sourceSpan[1]<=p.sourceSpan[0] ||
        p.sourceSpan[1]>transcript.text.length || p.evidence !== transcript.text.slice(...p.sourceSpan) ||
        !p.evidence.trim() || p.status!=='review_required' ||
        ![null,'low','normal','high'].includes(p.priority) ||
        (p.duePhrase!==null && (typeof p.duePhrase!=='string' || !p.evidence.includes(p.duePhrase))) ||
        !isRecord(p.dueIntent) || !['none','calendar_date','clarification_required'].includes(p.dueIntent.kind) ||
        p.dueIntent.dueInstant !== null) fail('PROPOSAL_UNGROUNDED');
    const correct=interpretDuePhrase(p.duePhrase,{referenceInstant:capture.capturedAt,timeZone:capture.timeZone,locale:capture.locale});
    if (p.dueIntent.kind !== correct.kind || p.dueIntent.dueDate !== correct.dueDate) fail('DUE_INTENT_MISMATCH');
    seen.add(p.proposalId); return Object.freeze({...p,sourceSpan:[...p.sourceSpan]});
  });
}

export function prepareOwnerReview({capture,transcript,proposals,approvedProposalIds,reviewRevision,ownerGrantRef}) {
  checkProposals(proposals,transcript,capture);
  if (!Array.isArray(approvedProposalIds) || !approvedProposalIds.every(x=>typeof x==='string') ||
      approvedProposalIds.length > proposals.length || new Set(approvedProposalIds).size!==approvedProposalIds.length ||
      !Number.isSafeInteger(reviewRevision) || reviewRevision < 1 ||
      !nonempty(ownerGrantRef,200)) fail('REVIEW_INVALID');
  const byId = new Map(proposals.map(p=>[p.proposalId,p]));
  const selected = approvedProposalIds.map(id=>{
    const p=byId.get(id); if (!p) fail('REVIEW_UNKNOWN_PROPOSAL');
    if (p.dueIntent.kind==='clarification_required') fail('REVIEW_UNRESOLVED_DATE');
    return {proposalId:id,title:p.title,dueDate:p.dueIntent.dueDate,priority:p.priority,
      sourceSha256:capture.sha256,sourceSpan:p.sourceSpan,
      idempotencyKey:digest([capture.ownerId,capture.noteId,id,reviewRevision].join('\0'))};
  });
  // Inert object only: ownerGrantRef is a *claim*, never authenticated here.
  return Object.freeze({schema:'vivim.lab-task-review-intent/1',status:'host_authorization_required',
    noteId:capture.noteId,ownerId:capture.ownerId,reviewRevision,ownerGrantRef,selected});
}
