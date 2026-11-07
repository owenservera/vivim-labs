import {test} from 'node:test';import assert from 'node:assert/strict';
import {inspectProducerFeed,validateEGraph,safePath} from '../contracts/validate-inputs.mjs';
import {graph,graphFixture,encodeExchange} from './helpers.mjs';
const native={A:'vivim.wave1.code-observations/1',B:'vivim.wave1.kb-semantic/1',C:'vivim.wave1.command-candidates/1'};
for(const [lane,schema] of Object.entries(native))test(`native ${lane} producer is NOT direct E feed`,()=>{
 const r=inspectProducerFeed({schema});assert.equal(r.ok,false);assert.equal(r.lane,lane);assert.equal(r.status,'BLOCKED_CONTRACT_CROSSWALK');
});
test('unknown producer schema refuses',()=>assert.equal(inspectProducerFeed({schema:'invented/99'}).reason,'unsupported_feed_schema'));
test('E source-records requires lane/nodes/edges, but accepted shape is not verified',()=>{
 assert.equal(inspectProducerFeed({schema:'vivim.wave1.source-records/1'}).ok,false);
 assert.equal(inspectProducerFeed({schema:'vivim.wave1.source-records/1',lane:'A',nodes:[],edges:[]}).status,'STRUCTURE_ONLY_NOT_VERIFIED');
});
test('E graph fields accepted without silent Round 1 key substitution',()=>{
 const v=validateEGraph(graph());assert.equal(v.nodes.size,7);assert.equal(v.status,'BLOCKED_CONTRACT_OR_PROVENANCE');
 assert(v.diagnostics.some(x=>x.code==='missing_edge_evidence'));
 assert(!v.diagnostics.some(x=>x.code==='missing_declared_root'));
});
test('E graph source commit must be pinned to declared root',()=>{
 const v=validateEGraph(graph({stale:true}));assert(v.diagnostics.some(x=>x.code==='root_revision_or_scope_mismatch'));
});
test('invalid 40-byte and 64-byte provenance not conflated',()=>{
 const x=graphFixture();x.nodes.find(n=>n.metadata?.source_lane==='A').metadata.content_sha256='f'.repeat(40);
 assert(validateEGraph(encodeExchange(x)).diagnostics.some(x=>x.code==='invalid_content_hash'));
});
test('missing root does not manufacture scope',()=>{
 const x=graphFixture();x.nodes.find(n=>n.metadata?.source_lane==='C').metadata.source_root='ghost';
 assert(validateEGraph(encodeExchange(x)).diagnostics.some(x=>x.code==='missing_declared_root'));
});
test('strict source scope rejects traversal, secret and windows paths',()=>{
 for(const p of ['../src/a.mjs','C:\\evil','/etc/passwd','hq/.env','hq/session.json','hq//x'])assert.equal(safePath(p),false,p);
 assert.equal(safePath('hq/lib/real.mjs'),true);
});
