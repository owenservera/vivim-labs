import assert from 'node:assert/strict';
import {resolve} from 'node:path';
import {startResidentProtocol} from './resident-protocol.mjs';
const runtime=await startResidentProtocol(resolve(process.argv[2]));
try{
 const h='a'.repeat(64);
 const commands=Array.from({length:64},(_,i)=>['bind',runtime.generation,'product:fixture.rs',h,i+1,'context','fixture'].join('\t'));
 const t0=performance.now();const a=await runtime.batch(commands);const cold=performance.now()-t0;
 assert.ok(a.every(s=>s.startsWith('OK vivim.meta-vlang.source/1')));
 await runtime.batch(['definitions','describe annotation.context']);
 const t1=performance.now();const b=await runtime.batch(commands);const warm=performance.now()-t1;
 assert.deepEqual(a,b);const stats=runtime.bindCacheStats();assert.equal(stats.hits,64);assert.equal(stats.misses,64);assert.equal(stats.checks,1);
 console.log(JSON.stringify({actualProductSdk:true,rows:64,coldMs:Math.round(cold*100)/100,warmMs:Math.round(warm*100)/100,nativeWarmRoundTrips:1,exactOutput:true,stats}));
}finally{runtime.close();}
