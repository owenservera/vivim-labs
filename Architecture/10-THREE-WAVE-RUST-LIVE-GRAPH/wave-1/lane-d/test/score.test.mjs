import {test} from 'node:test';import assert from 'node:assert/strict';import {scoreQuality} from '../quality/score.mjs';
const complete={roots:[{id:'hq',eligible:10,indexed:10,excluded:2,declared:12}],semantic:{eligible:3,linked:3},knowledge:{eligible:2,linked:2},commands:{eligible:5,sourceWired:5,runtimeVerified:0},compiled:{eligible:1,sourceBound:1},wiki:{eligible:1,verified:1},links:{checked:10,resolved:10},independentWitness:false};
test('all high percentages cannot pass without independent witness',()=>{
 const x=scoreQuality(complete);assert.equal(x.status,'BLOCKED');assert(x.blockers.includes('NO_INDEPENDENT_RUST_AND_WIKI_WITNESS'));
});
test('no roots is incomplete, not implicitly 100%',()=>assert(scoreQuality({...complete,roots:[]}).blockers.includes('INCOMPLETE_SCOPE')));
test('zero denominator explicitly NOT_MEASURED',()=>{
 const x=scoreQuality({...complete,compiled:{eligible:0,sourceBound:0}});assert.equal(x.measurements[4].status,'NOT_MEASURED');assert.equal(x.measurements[4].percent,null);
});
test('unknown denominator explicitly NOT_MEASURED',()=>{
 const x=scoreQuality({...complete,knowledge:null});assert.equal(x.measurements[1].status,'NOT_MEASURED');
});
test('overreported numerator hard refuses',()=>assert.throws(()=>scoreQuality({...complete,wiki:{eligible:1,verified:2}}),/invalid_coverage_denominator/));
test('incomplete root is a critical blocker',()=>{
 const x=scoreQuality({...complete,roots:[{id:'hq',eligible:10,indexed:9}]});assert(x.blockers.includes('INCOMPLETE_SCOPE'));
});
test('fatal source and crosswalk issue overrides all metrics',()=>{
 const x=scoreQuality({...complete,critical:['UNSAFE_SOURCE_ALIAS'],independentWitness:true});assert(x.blockers.includes('UNSAFE_SOURCE_ALIAS'));
});
