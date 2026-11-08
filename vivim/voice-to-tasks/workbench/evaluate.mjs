// Offline evaluator for *synthetic transcript fixtures*. NOT model WER or hardware QA.
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { baselineProposals } from './core.mjs';
export function evaluate(fixtures) {
  if (!Array.isArray(fixtures) || !fixtures.length) throw new Error('FIXTURES_REQUIRED');
  let tp=0,fp=0,fn=0,zeroTaskFalsePositives=0;
  const cases=fixtures.map(f=>{
    const actual=baselineProposals(f.transcript,f.capture);
    const expected=f.expectedActions;
    if (!Array.isArray(expected)) throw new Error('EXPECTED_ACTIONS_REQUIRED');
    // Fixture gold is full evidence text, not paraphrased title: explicit task matching.
    const gold = new Set(expected), predicted=new Set(actual.map(x=>x.evidence));
    const hit=[...predicted].filter(x=>gold.has(x)).length;
    tp+=hit; fp+=predicted.size-hit; fn+=gold.size-hit;
    if (!gold.size && predicted.size) zeroTaskFalsePositives++;
    return {name:f.name,language:f.capture.locale,expected:gold.size,predicted:predicted.size,matched:hit,
      passed:hit===gold.size && predicted.size===gold.size};
  });
  return {classification:'SYNTHETIC_TRANSCRIPT_ONLY',cases,totals:{tp,fp,fn,
    precision:tp+fp ? tp/(tp+fp) : 1,recall:tp+fn ? tp/(tp+fn) : 1,
    f1:2*tp+fp+fn ? 2*tp/(2*tp+fp+fn) : 1,zeroTaskFalsePositives}};
}
// CLI accepts compact *text-only* fixture records. Synthetic hashes never imply real audio.
function expandSyntheticFixture(f) {
  if (!f || typeof f.name !== 'string' || typeof f.locale !== 'string' || typeof f.text !== 'string' ||
      !Array.isArray(f.expectedActions)) throw new Error('SYNTHETIC_FIXTURE_INVALID');
  const hash = s => createHash('sha256').update(s).digest('hex');
  const capture = { schema:'vivim.lab-audio-capture/1',noteId:'note-12345678',
    ownerId:'fixture-only',deviceId:'fixture-only',sha256:hash('NOT-AUDIO:'+f.name+f.text),
    mime:'audio/mp4',bytes:2048,capturedAt:'2026-11-08T15:00:00+01:00',
    locale:f.locale,timeZone:'Europe/Madrid',consentToHostTransfer:false };
  const transcript = {schema:'vivim.lab-transcript/1',noteId:capture.noteId,
    audioSha256:capture.sha256,text:f.text,engineDigest:hash('SYNTHETIC-ENGINE'),status:'draft'};
  return {...f,capture,transcript};
}
if (process.argv[1] && import.meta.url.endsWith(process.argv[1].replaceAll('\\','/'))) {
  const path=process.argv[2]; if(!path) throw new Error('Usage: node evaluate.mjs fixtures.json');
  const v=evaluate(JSON.parse(readFileSync(path,'utf8')).map(expandSyntheticFixture));
  console.log(JSON.stringify(v,null,2));
  if (v.cases.some(x=>!x.passed)) process.exitCode=1;
}
