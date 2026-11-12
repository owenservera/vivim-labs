#!/usr/bin/env node
/** Local synthetic-only bench — no Host, provider, or personal data. */
import {performance} from 'node:perf_hooks';
import {visualCompile} from '../src/compiler.mjs';
import {scanLexical,semanticSummary} from '../src/semantic-lens.mjs';
const INV={actions:[
 {id:'vivim.status.show',namespace:'host',title:'Show status',category:'system',risk:'R0',readiness:'source_wired',examples:['show status'],args:{}},
 {id:'ui.draft.edit',namespace:'tierflow-ui',title:'Edit draft',category:'ui',risk:'unknown',readiness:'source_declared',examples:[],args:{}}
]};
const probes=[];
for(let i=0;i<20;i++){
 probes.push(`email Pet${'e'.repeat(i%4)}`);
 probes.push(i%2?`do not email Peter ${i}`:`don't send email ${i}`);
 probes.push(`/ui.draft.${i%2?'edit':'unknown'}`);
 probes.push(`"never email Peter" ${i}`);
 probes.push(`Review TierFlow and repos-hq from ${2020+i%7}`);
}
const measurements=[],failures=[];
for(const [i,text] of probes.entries()){
 const t=performance.now(),p=visualCompile(text,INV),s=semanticSummary(text,p);
 measurements.push(performance.now()-t);
 if(p.executionPerformed!==false||p.authorization!=='not_assessed')failures.push([i,'effects/authority']);
 if((text.includes("don't")||text.includes('do not'))&&p.outcome!=='blocked_negated')failures.push([i,'negation']);
 if(text.startsWith('"never')&&p.outcome==='blocked_negated')failures.push([i,'quoted negative']);
 const ts=scanLexical(text);if(ts.some(x=>text.slice(x.start,x.end)!==x.text))failures.push([i,'span']);
 if(s.executable!=='not_assessed')failures.push([i,'permission truth']);
}
const sorted=measurements.toSorted((a,b)=>a-b);
const result={schema:'vivim.labs.synthetic-semantic-bench/1',kind:'local_in_process_only',probes:probes.length,
 p50Ms:+sorted[Math.floor(sorted.length*.5)].toFixed(3),p95Ms:+sorted[Math.floor(sorted.length*.95)].toFixed(3),
 maxMs:+sorted.at(-1).toFixed(3),failures,passed:failures.length===0};
console.log(JSON.stringify(result,null,2));
if(failures.length)process.exitCode=1;