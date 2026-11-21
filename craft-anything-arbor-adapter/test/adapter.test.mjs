import {test} from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync, mkdirSync, writeFileSync, readFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {spawnSync} from 'node:child_process';
import {buildAtlas, canonicalJson, evidenceDigest, validateBundle} from '../src/adapter.mjs';

const origin='https://test.example';
function record(overrides={}) {
  const r={
    subject:{kind:'website',name:'Test app'},operation:'web_site_capture',
    params:{widths:[1440,390]},result:{url:origin+'/',counts:{outline:2,headings:1,links:0,forms:0,images:0},sign_in_page:false},
    confidence:'observed',authority:'our-run',environment:{clean_room:true,engine:'cdp',host:'windows-x86_64'},
    limitations:[{code:'one_state',message:'One state only'}],coverage:{status:'complete',examined:2,total:2},locations:[{kind:'url',value:origin+'/'}],links:[],summary:'browser capture',
    ...overrides
  };
  r.id=evidenceDigest(r);return r;
}
const bundle=(...records)=>({format:'craft-anything-evidence-bundle',version:1,records,unknowns:[]});

test('canonical sorting and source ID excludes display name and summary',()=>{
  assert.equal(canonicalJson({z:1,a:[{b:false,a:1}]}),'{"a":[{"a":1,"b":false}],"z":1}');
  const r=record();const changed={...r,summary:'other',subject:{...r.subject,name:'renamed'}};
  assert.equal(evidenceDigest(changed),r.id);
});
test('accepts valid records; identical repeats deduplicate',()=>{
  const r=record();assert.equal(validateBundle(bundle(r,r)).length,1);
  const a=buildAtlas({bundle:bundle(r,r),siteId:'test',origin});
  assert.equal(a.source.verifiedRecords,1); assert.equal(a.observations[0].kind,'website-capture');
  assert.equal(a.authority.executionAuthorized,false);
});
test('rejects changed hashed result, wrong schema, fabricated identity',()=>{
  const r=record();const bad={...r,result:{...r.result,url:'https://other.test'}};
  assert.throws(()=>validateBundle(bundle(bad)),/EVIDENCE_HASH_MISMATCH/);
  assert.throws(()=>validateBundle({...bundle(r),version:2}),/UNSUPPORTED_CRAFT_BUNDLE/);
  assert.throws(()=>buildAtlas({bundle:bundle(r),siteId:'Test',origin}),/INVALID_TARGET_SCOPE/);
});
test('rejects foreign browser origins and misleading source labels',()=>{
  const other=record({result:{url:'https://other.test/'}});
  assert.throws(()=>buildAtlas({bundle:bundle(other),siteId:'test',origin}),/MIXED_TARGET_ORIGINS/);
});
test('does not accept labels without an owned-target declaration',()=>{
  assert.throws(()=>buildAtlas({bundle:bundle(record()),siteId:'test',origin,includeLabels:true}),/LABELS_REQUIRE_OWNED_TARGET/);
});
test('imported capture contents require matching Craft-recorded path and origin',()=>{
  const dir=mkdtempSync(join(tmpdir(),'craft-bridge-'));
  const cap=join(dir,'capture');mkdirSync(cap);
  const r=record({result:{url:origin+'/',output_directory:cap,counts:{outline:1}}});
  writeFileSync(join(cap,'capture.json'),JSON.stringify({url:origin+'/',outline:[{role:'button',label:'Create project'},{role:'button',label:'email@example.com'}]}));
  const a=buildAtlas({bundle:bundle(r),siteId:'test',origin,captureDir:cap,includeLabels:true,ownedTarget:true});
  assert.equal(a.controls.length,2);assert.equal(a.controls[0].label,'Create project');assert.equal(a.controls[1].label,null);
  assert.equal(a.observations.find(x=>x.kind==='capture-artifact').artifactAttestedByCraftEvidence,false);
  assert.throws(()=>buildAtlas({bundle:bundle(r),siteId:'test',origin,captureDir:dir}),/CAPTURE_PROVENANCE_MISMATCH/);
});
test('end-to-end CLI writes research-only atlas/report and refuses output overwrite',()=>{
  const dir=mkdtempSync(join(tmpdir(),'craft-cli-'));
  const input=join(dir,'input.json'),output=join(dir,'output');
  writeFileSync(input,JSON.stringify(bundle(record())));
  const cli=join(import.meta.dirname,'..','src','cli.mjs');
  const args=[cli,'import','--bundle',input,'--site','test','--origin',origin,'--out',output];
  const first=spawnSync(process.execPath,args,{encoding:'utf8'});
  assert.equal(first.status,0,first.stderr);
  assert.equal(JSON.parse(readFileSync(join(output,'atlas.json'),'utf8')).authority.activationPermitted,false);
  const second=spawnSync(process.execPath,args,{encoding:'utf8'});
  assert.equal(second.status,1);
});