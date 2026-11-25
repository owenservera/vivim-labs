import assert from 'node:assert/strict';
import {resolve} from 'node:path';
import {createHash} from 'node:crypto';
import {pathToFileURL} from 'node:url';
import {startResidentProtocol} from './resident-protocol.mjs';
import {compileSourceV1} from './protocol-v1.mjs';
const [binary,extractPath,parserPath]=process.argv.slice(2);
const {extractAnnotations:extract}=await import(pathToFileURL(resolve(extractPath)).href);
const {parseAnnotationV2:parse}=await import(pathToFileURL(resolve(parserPath)).href);
const runtime=await startResidentProtocol(resolve(binary));
let extracts=0,parses=0,wire=0,rev='r1';
const source=Array.from({length:32},(_,i)=>'// @hq:context Fixture observation '+i).join('\n');
const hash=createHash('sha256').update(source).digest('hex');
const hq={sourceEvidence:({path,endLine})=>({status:'indexed',path,sourceId:'fixture:'+path,devSourceId:'owned-fixture',currentHash:hash,indexedHash:hash,text:endLine===1?source.split('\n')[0]:source,lineStart:1,lineEnd:endLine===1?1:32,projectRevision:rev,indexWatermark:rev})};
const exchange={generation:runtime.generation,batch:async c=>{wire+=c.length;return runtime.batch(c);}};
const args={hq,exchange,binary:resolve(binary),project:'fixture',path:'fixture.rs',extractAnnotations:(...a)=>{extracts++;return extract(...a)},parseAnnotationV2:(...a)=>{parses++;return parse(...a)}};
try{
 let t=performance.now();const cold=await compileSourceV1(args),coldMs=performance.now()-t,coldWire=wire;
 assert.ok(cold.declarations.length>0);rev='r2';t=performance.now();const warm=await compileSourceV1(args),warmMs=performance.now()-t;
 assert.deepEqual(warm.declarations,cold.declarations);assert.deepEqual(warm.diagnostics,cold.diagnostics);assert.equal(warm.projectRevision,'r2');assert.equal(extracts,1);assert.equal(parses,1);assert.equal(wire-coldWire,1);
 console.log(JSON.stringify({actualProductSdk:true,actualArborExtractors:true,fixtureHq:true,declarations:cold.declarations.length,coldMs,warmMs,coldWire,warmWire:wire-coldWire,extracts,parses,exactOutput:true,freshIndexMetadata:true}));
}finally{runtime.close();}
