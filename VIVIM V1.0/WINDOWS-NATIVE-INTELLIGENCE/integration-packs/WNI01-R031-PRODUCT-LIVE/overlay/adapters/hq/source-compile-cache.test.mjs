import test from 'node:test';
import assert from 'node:assert/strict';
import {resolve} from 'node:path';
import {compileSourceV1} from './protocol-v1.mjs';
function fixture(){
 let digest='a'.repeat(64),generation=1,live=1,rev='r1',water='w1',source='dev1',stale=false,changeOnStatus=false,extracts=0,binds=0,status=0;
 const hq={sourceEvidence:({path,startLine,endLine})=>({status:stale?'stale':'indexed',path,sourceId:'project:'+path,devSourceId:source,currentHash:digest,indexedHash:digest,text:'// @hq:context fixture',lineStart:1,lineEnd:1,projectRevision:rev,indexWatermark:water})};
 const exchange={generation,batch:async commands=>commands.map(c=>{if(c==='status'){status++;if(changeOnStatus)stale=true;return 'OK generation='+live;}binds++;const a=c.split('\t');return ['OK vivim.meta-vlang.source/1',a[1],1,a[2],a[3],a[4],a[5],'unbound','source-assertion',a[6]].join('\t');})};
 const extractAnnotations=()=>{extracts++;return [{line:1,kind:'context',text:'fixture'}];};
 const parseAnnotationV2=()=>({records:[],diagnostics:[]});
 const args={hq,exchange,extractAnnotations,parseAnnotationV2,binary:resolve('fixture.exe'),project:'project',path:'src/lib.rs'};
 return {args,run:()=>compileSourceV1(args),stats:()=>({extracts,binds,status}),digest:v=>digest=v,rev:v=>{rev=v;water='w2'},source:v=>source=v,live:v=>live=v,stale:()=>stale=true,change:()=>changeOnStatus=true};
}
test('reuse exact source; rebase fresh index metadata',async()=>{const f=fixture();const a=await f.run();f.rev('r2');const b=await f.run();assert.deepEqual(a.declarations,b.declarations);assert.equal(b.projectRevision,'r2');assert.equal(b.indexWatermark,'w2');assert.deepEqual(f.stats(),{extracts:1,binds:1,status:1});});
test('source edit and source identity changes miss',async()=>{const f=fixture();await f.run();f.digest('b'.repeat(64));await f.run();f.source('dev2');await f.run();assert.equal(f.stats().extracts,3);});
test('no stale source reuse',async()=>{const f=fixture();await f.run();f.stale();await assert.rejects(f.run(),/STALE_OR_INCOMPLETE/);assert.equal(f.stats().status,0);});
test('source change during status is refused',async()=>{const f=fixture();await f.run();f.change();await assert.rejects(f.run(),/SOURCE_CHANGED/);});
test('live generation change is refused',async()=>{const f=fixture();await f.run();f.live(2);await assert.rejects(f.run(),/CACHE_GENERATION_CHANGED/);});
test('caller mutation does not poison artifacts; different parser misses',async()=>{const f=fixture();const a=await f.run();a.declarations[0].value='wrong';assert.equal((await f.run()).declarations[0].value,'fixture');f.args.parseAnnotationV2=()=>({records:[],diagnostics:[]});await f.run();assert.equal(f.stats().extracts,2);});
