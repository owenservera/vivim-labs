import test from 'node:test';
import assert from 'node:assert/strict';
import {validateManifest,parseTree,fileStats,scan,markdown} from '../run-first-pass.mjs';

const commit='a'.repeat(40);
const baseline={
 schema:'vivim.labs.readiness-manifest/1',source_baselines:{main:commit},
 components:[
  {id:'E01',name:'Foo',category:'easy',source_branch:'main',source_path_prefix:'foo/',
    arbor_target:'tests/foo/',owner:'F',reported_evidence:'SOURCE_RESEARCH',disposition:'PROMOTE_TESTS'},
  {id:'C01',name:'Bar',category:'core',source_branch:'main',source_path_prefix:'bar/',
    arbor_target:'native/bar/',owner:'A',reported_evidence:'DESIGN_ONLY',disposition:'PREPARE_ADAPTER'}
 ]
};
const tree='100644 blob '+commit+'\tfoo/src/test.mjs\0'+
 '100644 blob '+commit+'\tfoo/tests/fixture.test.mjs\0'+
 '100644 blob '+commit+'\tbar/README.md\0';
test('validates pinned sha, unique ids and safe paths',()=>{
 assert.equal(validateManifest(baseline).components.length,2);
 assert.throws(()=>validateManifest({...baseline,components:[baseline.components[0],baseline.components[0]]}),/ID_INVALID/);
 assert.throws(()=>validateManifest({...baseline,components:[{...baseline.components[0],source_path_prefix:'../secrets'}]}),/PATH_UNSAFE/);
 assert.throws(()=>validateManifest({...baseline,source_baselines:{main:'HEAD'}}),/SOURCE_SHA_INVALID/);
});
test('traces only Git tracked blobs and test/doc counts',()=>{
 const rows=parseTree(tree);
 assert.equal(rows.length,3);
 assert.equal(fileStats(rows.filter(x=>x.path.startsWith('foo/')).map(x=>x.path)).test_or_fixture,1);
});
test('source presence never certifies integration or runtime/device readiness',()=>{
 const r=scan(baseline,()=>({ok:true,raw:tree}));
 assert.equal(r.components[0].first_pass_static,'SOURCE_TREE_PRESENT');
 assert.equal(r.components[0].fresh_test_execution,'UNRUN');
 assert.equal(r.components[0].host_wiring,'UNVERIFIED');
 assert.equal(r.components[1].product_build,'UNRUN');
 assert.match(markdown(r),/not executable runtime verification/i);
});
test('missing baseline or source is explicit unknown or missing, never false zero',()=>{
 const missing=scan(baseline,()=>({ok:false,error:'missing-commit'}));
 assert.equal(missing.components[0].first_pass_static,'REF_UNAVAILABLE');
 assert.equal(missing.baseline.main.tracked_files,null);
 const found=scan(baseline,()=>({ok:true,raw:tree.replace('bar/README.md','other/README.md')}));
 assert.equal(found.components[1].first_pass_static,'SOURCE_PATH_MISSING');
});
test('full real manifest parses and does not contain duplicate IDs',async()=>{
 const {readFileSync}=await import('node:fs');
 const manifest=JSON.parse(readFileSync(new URL('../components.json',import.meta.url),'utf8'));
 assert.ok(validateManifest(manifest).components.length>=35);
 const ids=manifest.components.map(x=>x.id);
 assert.equal(ids.length,new Set(ids).size);
});
