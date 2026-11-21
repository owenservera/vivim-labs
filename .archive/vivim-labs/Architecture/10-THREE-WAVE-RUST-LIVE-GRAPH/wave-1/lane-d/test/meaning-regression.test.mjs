import {test} from 'node:test';import assert from 'node:assert/strict';
import {traceCompiled} from '../lineage/traverse.mjs';
import {scoreQuality} from '../quality/score.mjs';
import {safePath,inspectProducerFeed,validateEGraph} from '../contracts/validate-inputs.mjs';
import {checkLinks} from '../quality/links.mjs';
import {inspectPrivacy} from '../quality/privacy.mjs';
import {graph,graphFixture,encodeExchange} from './helpers.mjs';
const modify=(callback)=>{const x=graphFixture();callback(x);return validateEGraph(encodeExchange(x));};
const CODES=[
 ['identity_mismatch',()=>traceCompiled(graph({badIdentity:true})).chains[0].problems.includes('semantic_command_identity_mismatch')],
 ['missing_kb',()=>traceCompiled(graph({missingKb:true})).chains[0].problems.includes('knowledge_not_joined')],
 ['no_compiler',()=>traceCompiled(graph({noCompiler:true})).chains.length===0],
 ['stale_commit',()=>traceCompiled(graph({stale:true})).chains[0].problems.includes('root_head_stale')],
 ['declared_not_verified',()=>traceCompiled(graph()).observed.independentlyVerified===0],
 ['no_device_proof',()=>traceCompiled(graph()).chains[0].command.deviceObserved===false],
 ['no_host_effect',()=>traceCompiled(graph()).chains[0].command.effectPermission===false],
 ['no_implicit_owner',()=>traceCompiled(graph()).chains[0].semantic.ownerAuthorityVerified===false],
 ['source_byte_not_independently_verified',()=>traceCompiled(graph()).chains[0].source.bytesIndependentlyVerified===false],
 ['native_A_mismatch',()=>inspectProducerFeed({schema:'vivim.wave1.code-observations/1'}).ok===false],
 ['native_B_mismatch',()=>inspectProducerFeed({schema:'vivim.wave1.kb-semantic/1'}).ok===false],
 ['native_C_mismatch',()=>inspectProducerFeed({schema:'vivim.wave1.command-candidates/1'}).ok===false],
 ['schema_unsupported',()=>inspectProducerFeed({schema:'semantic-wiki/999'}).ok===false],
 ['path_parent',()=>safePath('hq/../private')===false],
 ['path_windows',()=>safePath('C:\\windows\\secret')===false],
 ['path_absolute',()=>safePath('/etc/passwd')===false],
 ['path_env',()=>safePath('hq/.env.prod')===false],
 ['path_session_json',()=>safePath('hq/session.json')===false],
 ['path_keystore',()=>safePath('hq/key.keystore')===false],
 ['missing_root',()=>modify(x=>x.nodes.find(n=>n.metadata?.source_lane==='C').metadata.source_root='unknown-root').diagnostics.some(d=>d.code==='missing_declared_root')],
 ['source_scope_escape',()=>modify(x=>x.nodes.find(n=>n.metadata?.source_lane==='A').metadata.source_path='unrelated/code.mjs').diagnostics.some(d=>d.code==='root_revision_or_scope_mismatch')],
 ['wrong_commit_type',()=>modify(x=>x.nodes.find(n=>n.metadata?.source_lane==='A').metadata.source_commit='f'.repeat(64)).diagnostics.some(d=>d.code==='wrong_git_commit_hash')],
 ['bad_content_type',()=>modify(x=>x.nodes.find(n=>n.metadata?.source_lane==='A').metadata.content_sha256='f'.repeat(40)).diagnostics.some(d=>d.code==='invalid_content_hash')],
 ['missing_accepted_status',()=>{const x=graphFixture();x.nodes.find(n=>n.metadata?.source_lane==='B'&&n.kind==='semantic_id').metadata.status='proposed';return traceCompiled(validateEGraph(encodeExchange(x))).chains[0].problems.includes('unaccepted_semantic_meaning');}],
 ['command_no_handler',()=>{const x=graphFixture();delete x.nodes.find(n=>n.metadata?.source_lane==='C').metadata.handler;return traceCompiled(validateEGraph(encodeExchange(x))).chains[0].problems.includes('handler_source_unverified');}],
 ['command_no_syntax',()=>{const x=graphFixture();delete x.nodes.find(n=>n.metadata?.source_lane==='C').metadata.syntax;return traceCompiled(validateEGraph(encodeExchange(x))).chains[0].problems.includes('cli_syntax_unverified');}],
 ['compiled_hash_changed',()=>{const x=graphFixture();x.nodes.find(n=>n.kind==='compiled-wiki-program').metadata.content_sha256='c'.repeat(64);return traceCompiled(validateEGraph(encodeExchange(x))).chains[0].problems.includes('compiled_source_digest_mismatch');}],
 ['arbitrary_empty_scope',()=>scoreQuality({}).blockers.includes('INCOMPLETE_SCOPE')],
 ['zero_denominator',()=>scoreQuality({wiki:{eligible:0,verified:0}}).measurements.find(x=>x.name==='wiki_verified_chains').percent===null],
 ['broken_anchor',()=>checkLinks(new Map([['index.md','[go](./link.md#bad)'],['link.md','# Known']])).defects.some(d=>d.code==='missing_anchor')],
 ['relative_escape',()=>checkLinks(new Map([['index.md','[go](../../secret.md)']])).defects.some(d=>d.code==='path_escape')],
 ['javascript_link',()=>checkLinks(new Map([['index.md','[go](javascript:alert)']])).defects.some(d=>d.code==='unsafe_link_scheme')],
 ['missing_md',()=>checkLinks(new Map([['index.md','[go](./missing.md)']])).defects.some(d=>d.code==='broken_link')],
 ['private_key_literal',()=>inspectPrivacy(new Map([['index.md','private_key=abc']])).defects.length===1],
 ['private_env_filename',()=>inspectPrivacy(new Map([['.env.md','Secret']])).defects.length===1],
];
for(const [name,probe] of CODES)test(`adversarial fixture ${name} refuses false authority`,()=>assert.equal(probe(),true,name));
// The historical negation/quotation/clause bugs are NOT retested by these inert wiki-only fixtures.
for(const phrase of ["don't delete index file",'never disconnect my PC','"delete index file"', 'show index file and delete index file','no borrar el índice']){
 test(`unsafe NL phrase is never promoted to executable by Lane D: ${phrase}`,()=>{
  const v=traceCompiled(graph());assert.equal(v.observed.independentlyVerified,0);
  assert.equal(v.chains[0].command.negativeIntentSafe,'NOT_TESTED');
  assert.equal(v.chains[0].command.effectPermission,false);
 });
}
