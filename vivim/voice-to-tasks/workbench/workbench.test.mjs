import test from 'node:test'; import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs'; import {spawnSync} from 'node:child_process'; import {createHash} from 'node:crypto';
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

test('E: standalone CLI consumes committed compact fixtures with zero external deps',()=>{
 const run=spawnSync(process.execPath,[new URL('./evaluate.mjs',import.meta.url).pathname,new URL('./fixtures.json',import.meta.url).pathname],{encoding:'utf8',timeout:5000});
 assert.equal(run.status,0,run.stderr); const report=JSON.parse(run.stdout);
 assert.equal(report.classification,'SYNTHETIC_TRANSCRIPT_ONLY'); assert.equal(report.cases.length,fixture.length);
 assert.equal(report.cases.every(c=>c.passed),true);
});
import {verifyObservedTrace} from './trace-check.mjs';
test('D: phone-to-Host-to-desktop trace requires review and Host receipt',()=>{
 const prefix={noteId:'note-12345678',ownerId:'owner-test',revision:1};
 const history=[
 {source:'android',action:'captured_local'},
 {source:'android',action:'queued'},
 {source:'host',action:'transcribed'},
 {source:'host',action:'proposed'},
 {source:'android',action:'reviewed',approvedProposalIds:['vp-1234567890abcdef12345678']},
 {source:'host',action:'committed',receiptId:'receipt:trusted-host-fixture'},
 {source:'desktop',action:'reopened'}].map(x=>({...prefix,...x}));
 const verified=verifyObservedTrace(history);assert.equal(verified.passed,true);
 assert.equal(verified.approvedCount,1);
 assert.throws(()=>verifyObservedTrace(history.filter(x=>x.action!=='reviewed')),{code:'TRACE_UNAUTHORIZED_TRANSITION'});
 assert.throws(()=>verifyObservedTrace(history.map(x=>x.action==='committed'?{...x,source:'android'}:x)),{code:'TRACE_HOST_COMMIT_REQUIRED'});
 assert.throws(()=>verifyObservedTrace(history.map(x=>x.action==='reopened'?{...x,ownerId:'different'}:x)),{code:'TRACE_EVENT_INVALID'});
 assert.throws(()=>verifyObservedTrace(history.map(x=>x.action==='committed'?{...x,receiptId:null}:x)),{code:'TRACE_UNVERIFIED_COMMIT'});
 const partial=verifyObservedTrace(history.slice(0,2));assert.equal(partial.finalAction,'queued');assert.equal(partial.passed,false);
});
