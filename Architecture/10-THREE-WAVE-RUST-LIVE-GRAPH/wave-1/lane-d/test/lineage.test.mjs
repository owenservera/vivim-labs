import {test} from 'node:test';import assert from 'node:assert/strict';
import {traceCompiled} from '../lineage/traverse.mjs';import {sourceProvenance} from '../lineage/provenance.mjs';
import {semanticState,commandState} from '../lineage/states.mjs';
import {graph} from './helpers.mjs';
test('E 3-relationship compiler chain recognized, NEVER independent verifier',()=>{
 const v=traceCompiled(graph());assert.equal(v.chains.length,1);assert.equal(v.observed.independentlyVerified,0);
 assert(v.chains[0].linkedKb.length===1);assert.equal(v.chains[0].source.anchorPinned,true);
 assert.equal(v.chains[0].independentlyVerified,false);
 assert(v.chains[0].problems.includes('declaration_not_independent_execution_proof'));
});
test('different canonical command ID is not joined by name',()=>{
 const v=traceCompiled(graph({badIdentity:true}));assert(v.chains[0].problems.includes('semantic_command_identity_mismatch'));
});
test('orphan KB cannot satisfy trusted knowledge condition',()=>{
 const v=traceCompiled(graph({missingKb:true}));assert(v.chains[0].problems.includes('knowledge_not_joined'));
});
test('stale code source commit flags root mismatch',()=>{
 const v=traceCompiled(graph({stale:true}));assert(v.chains[0].problems.includes('root_head_stale'));
});
test('no compiler bridge refuses to produce chain',()=>{
 const v=traceCompiled(graph({noCompiler:true}));assert.equal(v.chains.length,0);assert.equal(v.status,'BLOCKED_NO_CHAIN');
});
test('no status type conflation: current semantic != handler permission',()=>{
 const v=traceCompiled(graph());assert.equal(v.chains[0].semantic.registryStatus,'current');assert.equal(v.chains[0].command.effectPermission,false);
});
test('source pin alone cannot certify original file bytes independently',()=>{
 const g=graph(),roots=g.nodes.filter(n=>n.kind==='source-root'),p=sourceProvenance(g.nodes.find(n=>n.kind==='file'),roots);
 assert.equal(p.anchorPinned,true);assert.equal(p.bytesIndependentlyVerified,false);
});
test('semantic description and wired status have separate owners',()=>{
 assert.equal(semanticState({metadata:{status:'proposed',description:'x',semantic_id:'a@1'}}).acceptedMeaning,false);
 assert.equal(commandState({metadata:{status:'wired',handler:'exists',syntax:'cmd'}}).runtimeTested,false);
});
