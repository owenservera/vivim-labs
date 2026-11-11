import test from 'node:test';import assert from 'node:assert/strict';
import {buildPersonalGraphProposal,personalPrefix} from '../src/scoped-graph-proposal.mjs';
import {normalizeChatGptConversations} from '../src/chatgpt-normalize.mjs';
import {conversation} from './fixtures.mjs';
const digest='b'.repeat(64);
const input=(owner='owner-a')=>({owner,sourceId:'chatgpt-export:archive-a',provenanceRevision:1,provenanceDigest:digest,expectedGraphEpoch:7,batchKey:'batch-1',conversations:normalizeChatGptConversations([conversation(0)],{accountId:'account-a'})});
test('scoped Rust proposal puts nodes before edges and preserves provenance',()=>{
 const result=buildPersonalGraphProposal(input());
 assert.equal(result.counts.nodes,3);assert.equal(result.counts.edges,3);
 assert.equal(result.expected.graphEpoch,7);
 assert.ok(result.changes.ops.slice(0,3).every(op=>op.op==='PutNode'));
 assert.ok(result.changes.ops.slice(3).every(op=>op.op==='PutEdge'));
 assert.ok(result.changes.ops.every(op=>(op.node??op.edge).id.startsWith('personal:7:owner-a/')));
 assert.ok(result.changes.ops.every(op=>(op.node??op.edge).properties['hqkg.provenance.digest']===digest));
 assert.ok(result.changes.ops.filter(op=>op.op==='PutNode').every(op=>op.node.sources.includes('chatgpt-export:archive-a')));
 assert.equal(buildPersonalGraphProposal(input()).changes.changeSetId,result.changes.changeSetId);
});
test('owner scope cannot collide and UTF8 byte lengths match Rust',()=>{
 assert.equal(personalPrefix('ñ'),'personal:2:ñ/');
 const a=buildPersonalGraphProposal(input('owner-a'));
 const b=buildPersonalGraphProposal(input('owner-b'));
 assert.notEqual(a.changes.ops[0].node.id,b.changes.ops[0].node.id);
});
test('invalid or cross-source provenance cannot be proposed',()=>{
 assert.throws(()=>buildPersonalGraphProposal({...input(),provenanceDigest:'x'}),{code:'GRAPH_PROVENANCE_INVALID'});
 assert.throws(()=>buildPersonalGraphProposal({...input(),owner:''}),{code:'GRAPH_OWNER_INVALID'});
 const invalid=input(); invalid.conversations[0].messages[0].contentDigest='bad';
 assert.throws(()=>buildPersonalGraphProposal(invalid),{code:'GRAPH_MESSAGE_INVALID'});
});
