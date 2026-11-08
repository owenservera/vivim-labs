import test from 'node:test'; import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs'; import {createHash} from 'node:crypto';
import {checkCapture,checkTranscript,baselineProposals,checkProposals,interpretDuePhrase,prepareOwnerReview} from './core.mjs';
import {evaluate} from './evaluate.mjs';
const sha=x=>createHash('sha256').update(x).digest('hex');
const fixture=JSON.parse(readFileSync(new URL('./fixtures.json',import.meta.url),'utf8'));
function inputs(f=fixture[0]) {
 const capture={schema:'vivim.lab-audio-capture/1',noteId:'note-12345678',ownerId:'owner-test',deviceId:'android-test',sha256:sha(f.text),mime:'audio/mp4',bytes:2048,
  capturedAt:'2026-11-08T15:00:00+01:00',locale:f.locale,timeZone:'Europe/Madrid',consentToHostTransfer:false};
 const transcript={schema:'vivim.lab-transcript/1',noteId:capture.noteId,audioSha256:capture.sha256,text:f.text,engineDigest:sha('fixture-engine'),status:'draft'};
 return {capture,transcript};
}
test('A: strict capture envelope; no raw audio bytes or hidden transfer',()=>{
 const {capture}=inputs(); assert.equal(checkCapture(capture).consentToHostTransfer,false);
 for (const patch of [{audio:'secret'}, {bytes:0}, {bytes:30000000}, {mime:'audio/wav'}, {sha256:'bad'}, {timeZone:'Mars/Phobos'}, {consentToHostTransfer:'yes'}])
   assert.throws(()=>checkCapture({...capture,...patch}));
});
test('A: transcript cannot claim different source or engine',()=>{
 const {capture,transcript}=inputs(); assert.deepEqual(checkTranscript(transcript,capture),transcript);
 for(const patch of [{noteId:'other-note'}, {audioSha256:sha('wrong')},{engineDigest:'none'}, {status:'committed'}, {tokens:['secrets']}, {text:'x'.repeat(20001)}])
   assert.throws(()=>checkTranscript({...transcript,...patch},capture));
});
test('B: deterministic date intent never invents a timestamp',()=>{
 const c={referenceInstant:'2026-11-08T15:00:00+01:00',timeZone:'Europe/Madrid',locale:'en-US'};
 assert.deepEqual(interpretDuePhrase(null,c),{kind:'none',dueDate:null,dueInstant:null});
 assert.deepEqual(interpretDuePhrase('tomorrow',c).dueDate,'2026-11-09');
 assert.equal(interpretDuePhrase('by Friday',c).kind,'clarification_required');
 assert.equal(interpretDuePhrase('tomorrow morning',c).kind,'clarification_required');
 assert.equal(interpretDuePhrase('tomorrow',c).dueInstant,null);
});
test('B: multilingual relative dates and DST boundary are calendar-safe',()=>{
 const common={timeZone:'Europe/Madrid',referenceInstant:'2026-03-28T23:30:00+01:00'};
 assert.equal(interpretDuePhrase('mañana',{...common,locale:'es-ES'}).dueDate,'2026-03-29');
 assert.equal(interpretDuePhrase('demà passat',{...common,locale:'ca-ES'}).dueDate,'2026-03-30');
 assert.equal(interpretDuePhrase('pasado mañana',{...common,locale:'es-ES'}).dueDate,'2026-03-30');
});
test('B: explicit-baseline extraction and source spans',()=>{
 const {capture,transcript}=inputs();const p=baselineProposals(transcript,capture);assert.equal(p.length,3);
 assert.equal(p[0].dueIntent.dueDate,'2026-11-09');assert.equal(p[1].dueIntent.kind,'clarification_required');
 assert.deepEqual(checkProposals(p,transcript,capture),p);
});
test('C: reject hallucinated or tampered model proposals',()=>{
 const {capture,transcript}=inputs(), p=baselineProposals(transcript,capture);
 const fail=alter=>assert.throws(()=>checkProposals([alter,...p.slice(1)],transcript,capture));
 fail({...p[0],evidence:'different words'}); fail({...p[0],sourceSpan:[1,10]});
 fail({...p[0],duePhrase:'2030'});fail({...p[0],dueIntent:{kind:'calendar_date',dueDate:'2030-01-01',dueInstant:null}});
 assert.throws(()=>checkProposals([p[0],p[0]],transcript,capture));
});
test('C: review is inert; requires owner authorization and resolved dates',()=>{
 const {capture,transcript}=inputs();const p=baselineProposals(transcript,capture);
 const params={capture,transcript,proposals:p,approvedProposalIds:[p[0].proposalId,p[2].proposalId],reviewRevision:1,ownerGrantRef:'claim-only-owner-ref'};
 const approved=prepareOwnerReview(params);assert.equal(approved.status,'host_authorization_required');
 assert.equal(approved.selected.length,2);assert.equal(approved.selected[0].dueDate,'2026-11-09');
 assert.equal(approved.selected[1].dueDate,null);
 assert.deepEqual(prepareOwnerReview(params),approved);
 assert.throws(()=>prepareOwnerReview({...params,approvedProposalIds:[p[1].proposalId]}),{code:'REVIEW_UNRESOLVED_DATE'});
 assert.throws(()=>prepareOwnerReview({...params,approvedProposalIds:[p[0].proposalId,p[0].proposalId]}));
 assert.throws(()=>prepareOwnerReview({...params,approvedProposalIds:['vp-'+sha('ghost').slice(0,24)]}));
 assert.throws(()=>prepareOwnerReview({...params,ownerGrantRef:''}));
});
test('D/E: synthetic evaluation exposes false positives; never claims WER',()=>{
 const batch=fixture.map(f=>{const {capture,transcript}=inputs(f);return {name:f.name,capture,transcript,expectedActions:f.expectedActions}});
 const r=evaluate(batch);assert.equal(r.classification,'SYNTHETIC_TRANSCRIPT_ONLY');
 assert.equal(r.cases.length,fixture.length);assert.ok(r.totals.f1>=0&&r.totals.f1<=1);
 assert.equal('wer' in r.totals,false);assert.equal(r.cases.some(c=>!c.passed),false);
});
