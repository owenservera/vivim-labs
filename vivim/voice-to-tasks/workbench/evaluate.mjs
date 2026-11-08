// Offline evaluator for *synthetic transcript fixtures*. NOT model WER or hardware QA.
import { readFileSync } from 'node:fs';
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
if (process.argv[1] && import.meta.url.endsWith(process.argv[1].replaceAll('\\','/'))) {
  const path=process.argv[2]; if(!path) throw new Error('Usage: node evaluate.mjs fixtures.json');
  const v=evaluate(JSON.parse(readFileSync(path,'utf8')));
  console.log(JSON.stringify(v,null,2));
  if (v.cases.some(x=>!x.passed)) process.exitCode=1;
}
