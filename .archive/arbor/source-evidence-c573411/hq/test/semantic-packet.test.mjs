import test from 'node:test';
import assert from 'node:assert/strict';
import { selectMandatorySources, semanticPacketManifest } from '../lib/context.mjs';
import { estimateTokens } from '../lib/util.mjs';

const ids=['test.action@1','test.contract@1'];
const candidate=(ref,values,cost=100,state='indexed',resolution='current')=>({
  ref,estimatedTokens:cost,contentHash:'a'.repeat(64),
  semanticReferences:values.map((semanticId,line)=>({semanticId,line:line+1,annotationKind:'consumes',
    resolution,semanticAnchor:{state,anchorId:ref+':'+semanticId,fingerprint:'b'.repeat(64)}}))
});

test('mandatory coverage precedes relevance and selects deterministic minimum cost including manifest',()=>{
  const a=candidate('p:a.mjs',[ids[0]]);
  const b=candidate('p:b.mjs',[ids[1]]);
  const c=candidate('p:c.mjs',ids,20);
  const exact=20+estimateTokens(JSON.stringify(semanticPacketManifest(ids,[c])));
  for(const order of [[a,b,c],[c,b,a],[b,a,c]]){
    const result=selectMandatorySources(order,ids,{capacity:exact,maxFiles:3});
    assert.equal(result.status,'complete');
    assert.deepEqual(result.selected,[c]);
  }
  const insufficient=selectMandatorySources([a,b,c],ids,{capacity:exact-1,maxFiles:3});
  assert.equal(insufficient.status,'budget_exceeded');
  assert.deepEqual(insufficient.selected,[]);
  assert.equal(selectMandatorySources([a,b],ids,{capacity:10000,maxFiles:1}).status,'budget_exceeded');
});

test('missing, ambiguous and deprecated dependencies cannot be presented as complete',()=>{
  for(const bad of [candidate('p:bad',[ids[1]],20,'ambiguous'),candidate('p:bad',[ids[1]],20,'indexed','deprecated'),
    candidate('p:bad',[ids[1]],20,'indexed','invalid_kind')]){
    const result=selectMandatorySources([candidate('p:a',[ids[0]]),bad],ids,{capacity:10000,maxFiles:3});
    assert.equal(result.status,'incomplete');
    assert.deepEqual(result.missing,[ids[1]]);
    assert.deepEqual(result.selected,[]);
  }
  const manifest=semanticPacketManifest(ids,[candidate('p:a',ids)]);
  assert.equal(manifest.planningEligible,false);
  assert.equal(manifest.mandatoryConstraints.status,'incomplete');
  assert.equal(manifest.sourceTextTreatment,'untrusted-data');
});
