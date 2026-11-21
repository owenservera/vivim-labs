import test from 'node:test';
import assert from 'node:assert/strict';
import {compileSemanticWiki,SEMANTIC_WIKI_IR} from '../semantic-wiki-compiler.mjs';

const H='a'.repeat(64), C='b'.repeat(64);
const body=`/**
 * @hq:action vivim.status.show@1
 * @hq:capability vivim.status.read@1
 * @hq:contract vivim.status.request@1
 * @hq:effect host:read-only
 * @hq:evidence test:tests/status.test.mjs
 * @hq:context read-only status
 */
export function status() {}`;
const source=(id='status',text=body,change={})=>({
 sourceId:id,repoRef:'vivim-web-harness',path:'apps/host/status.mjs',
 contentHash:H,sourceText:text,symbolId:'status',...change
});
const catalog=()=>({version:6,sourceDigest:C,
 actions:[{id:'vivim.status.show',owner:'host',title:'Show VIVIM status',
 risk:'R0',readiness:'source_wired',args:{}}]});
const registry={lookup(id){
 return ['vivim.status.show@1','vivim.status.read@1','vivim.status.request@1']
  .includes(id)?{id,status:'current'}:null;
}};
const bindings=[{semanticAction:'vivim.status.show@1',
 hostActionId:'vivim.status.show',owner:'host'}];
const build=(changes={})=>compileSemanticWiki({sources:[source()],registry,
 hostCatalog:catalog(),bindings,...changes});

test('compiled IR is explicitly inert and source+catalog pinned',()=>{
 const v=build();assert.equal(v.schema,SEMANTIC_WIKI_IR);
 assert.equal(v.effectPerformed,false);assert.equal(v.catalog.version,6);
 assert.equal(v.catalog.digest,C);assert.equal(v.coverage.runtimeAcceptance,'not_evaluated');
});
test('existing HQ v2 parser extracts source annotation, never raw text',()=>{
 const v=build();assert.equal(v.sources.length,1);
 assert.ok(v.nodes.some(x=>x.kind==='action'&&x.ref==='vivim.status.show@1'));
 assert.ok(v.nodes.some(x=>x.kind==='capability'));
 assert.equal(JSON.stringify(v).includes('export function status'),false);
});
test('typed action links capability, contract, effect and evidence',()=>{
 const v=build();assert.equal(v.actions.length,1);
 assert.ok(v.edges.some(x=>x.kind==='supports_capability'));
 assert.ok(v.edges.some(x=>x.kind==='subject_to_contract'));
 assert.ok(v.edges.some(x=>x.kind==='declares_effect'));
 assert.ok(v.edges.some(x=>x.kind==='has_source_evidence'));
});
test('catalog mapping is descriptive only even for source-wired read risk R0',()=>{
 const v=build(),a=v.actions[0];
 assert.equal(a.hostActionId,'vivim.status.show');assert.equal(a.authority,'none');
 assert.equal(a.executable,false);assert.equal(a.executionPerformed,false);
 assert.equal(a.readiness,'source_wired');assert.equal(a.risk,'R0');
});
test('owner binding absent => no Host projection',()=>{
 const v=build({bindings:[]});assert.equal(v.actions.length,0);
 assert.ok(v.rejections.some(x=>x.reason==='no_owner_binding'));
 assert.ok(v.nodes.length>0);
});
test('unknown annotation semantic ID rejects entire source, not partial compile',()=>{
 const v=build({sources:[source('s','/**\n * @hq:action invented.command@1\n * @hq:effect host:write\n */')]});
 assert.equal(v.actions.length,0);assert.equal(v.nodes.length,0);
 assert.ok(v.diagnostics.some(x=>x.code==='ANNOTATION_unknown_ref'));
});
test('malformed effect cannot supply a partial capability projection',()=>{
 const v=build({sources:[source('s',body.replace('host:read-only','exec('))]});
 assert.equal(v.actions.length,0);
 assert.ok(v.rejections.some(x=>x.reason==='annotation_errors'));
});
test('duplicate explicit crosswalk fails closed',()=>{
 const v=build({bindings:[...bindings,...bindings]});
 assert.equal(v.actions.length,0);
 assert.ok(v.rejections.some(x=>x.reason==='ambiguous_owner_binding'));
});
test('crosswalk to wrong Host owner fails closed',()=>{
 const v=build({bindings:[{...bindings[0],owner:'other'}]});
 assert.equal(v.actions.length,0);
 assert.ok(v.rejections.some(x=>x.reason==='host_catalog_owner_mismatch'));
});
test('crosswalk to absent Host action is rejected',()=>{
 const v=build({bindings:[{...bindings[0],hostActionId:'unknown.action'}]});
 assert.equal(v.actions.length,0);
 assert.ok(v.rejections.some(x=>x.reason==='host_catalog_owner_mismatch'));
});
test('registry comment strings inside JS quotes do not become annotations',()=>{
 const v=build({sources:[source('s',`const q = "@hq:action vivim.status.show@1";\nconst x = 1;`)]});
 assert.equal(v.nodes.length,0);assert.equal(v.actions.length,0);
});
test('untrusted extra tag cannot redefine execution rights',()=>{
 const v=build({sources:[source('s',body.replace(' * @hq:context',
 ' * @hq:executor=run_shell_command\n * @hq:context'))]});
 assert.equal(v.actions.length,1);assert.equal(v.actions[0].executable,false);
});
test('secret source file path is rejected',()=>{
 const v=build({sources:[source('s',body,{path:'secrets/.env'})]});
 assert.equal(v.sources.length,0);assert.equal(v.actions.length,0);
 assert.ok(v.diagnostics.some(x=>x.code==='INVALID_OR_DUPLICATE_SOURCE'));
});
test('explicit source pin must be SHA256-shaped',()=>{
 const v=build({sources:[source('s',body,{contentHash:'unsafe'})]});
 assert.equal(v.actions.length,0);
});
test('duplicate source is refused and kept deterministic',()=>{
 const v=build({sources:[source('x'),source('x')]});
 assert.equal(v.sources.length,1);
 assert.ok(v.diagnostics.some(x=>x.code==='INVALID_OR_DUPLICATE_SOURCE'));
});
test('source order permuting does not change output',()=>{
 const a=build({sources:[source('a'),source('b')]});
 const b=build({sources:[source('b'),source('a')]});
 assert.deepEqual(a,b);
});
test('empty valid inputs never invent actions',()=>{
 const v=build({sources:[]});
 assert.equal(v.actions.length,0);assert.equal(v.nodes.length,0);
});
test('wrong catalog or registry shape throws before compilation',()=>{
 assert.throws(()=>build({hostCatalog:{}}));
 assert.throws(()=>build({registry:{}}));
 assert.throws(()=>build({hostCatalog:{...catalog(),actions:[catalog().actions[0],catalog().actions[0]]}}));
});
test('source parser diagnoses invalid reference without creating action projection',()=>{
 const v=build({sources:[source('s',body.replace('vivim.status.read@1','unknown.capability@1'))]});
 assert.equal(v.actions.length,0);assert.ok(v.coverage.rejectedSources>0);
});
test('no permission claims are created anywhere in machine IR',()=>{
 const v=build();
 const t=JSON.stringify(v);assert.equal(t.includes('"permissionGranted":true'),false);
 assert.equal(t.includes('"executable":true'),false);
 assert.equal(t.includes('"executionPerformed":true'),false);
});

test('two distinct action tags on same symbol cannot silently share contracts',()=>{
 const text=body.replace(' * @hq:capability',' * @hq:action vivim.status.read2@1\n * @hq:capability');
 const r={lookup(id){return (id==='vivim.status.read2@1'||registry.lookup(id))?{id,status:'current'}:null}};
 const v=build({sources:[source('s',text)],registry:r});
 assert.equal(v.actions.length,0);
 assert.ok(v.rejections.some(x=>x.reason==='ambiguous_action_symbol'));
});
test('compiled runtime and source digest flags cannot be mistaken for verification',()=>{
 const a=build().actions[0];
 assert.equal(a.sourceDigestVerified,false);assert.equal(a.runtimeVerified,false);
});
