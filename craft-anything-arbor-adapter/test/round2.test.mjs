import {test} from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync,mkdirSync,writeFileSync,readFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join,resolve} from 'node:path';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {buildAtlas,evidenceDigest,validateBundle} from '../src/adapter.mjs';
import {validateProfileBaseline,reconcileAtlas} from '../src/reconcile.mjs';
import {diffAtlases} from '../src/diff.mjs';
const origin='https://test.example';
const sha=x=>createHash('sha256').update(x).digest('hex');
const png=Buffer.from('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO2ufnoAAAAASUVORK5CYII=','base64');
const bundle=(...records)=>({format:'craft-anything-evidence-bundle',version:1,records,unknowns:[]});
function record(operation,result,params={}) {
 const r={subject:{kind:'website',name:'test'},operation,params,result,
  confidence:'observed',authority:'our-run',environment:{clean_room:true,engine:'cdp',host:'test'},
  limitations:[],coverage:{status:'partial',examined:1,total:2},
  locations:[{kind:'url',value:origin+'/'}],links:[],summary:'test'};
 r.id=evidenceDigest(r);return r;
}
const profile=(siteId='test',selectors=['button[type="submit"]'])=>({
 schema:'vivim.application-profile-baseline/1',target:{siteId,origin},profiles:[
  {owner:'studio',sourcePath:'studio/src/profiles/sites/test.mjs',features:[{id:'chat.send',surface:'chat',sourceStatus:'configured',selectors,matchTerms:['Send message']}]},
  {owner:'harness',sourcePath:'harness/src/providers/test.mjs',features:[{id:'chat.send',surface:'chat',sourceStatus:'configured',selectors:[...selectors,'textarea'],matchTerms:['Send message']}]}
 ]
});
function capture(dir, outline=[]) {
 const folder=join(dir,'capture');mkdirSync(folder);
 const content={url:origin+'/',outline,headings:[],links:[],forms:[],assets:{images:[]},a11y:{landmarks:{navigation:1},unnamed_controls:3,images_without_alt:0}};
 writeFileSync(join(folder,'capture.json'),JSON.stringify(content));
 return {folder,content};
}
test('Craft display-only variations deduplicate while hashed changes refuse',()=>{
 const r=record('web_site_capture',{url:origin+'/'});
 const copy={...r,summary:'Other summary',subject:{...r.subject,name:'New name'},carries_target_content:true};
 assert.equal(validateBundle(bundle(r,copy)).length,1);
 const bad={...r,result:{url:origin+'/',changed:true}};
 assert.throws(()=>validateBundle(bundle(r,bad)),/EVIDENCE_HASH_MISMATCH/);
});
test('HTML blueprint tag mapping is read-only; password and private text excluded',()=>{
 const dir=mkdtempSync(join(tmpdir(),'craft-atlas2-'));
 const {folder}=capture(dir,[{tag:'a',text:'Documentation',href:origin+'/docs'},
  {tag:'button',text:'Send message'},{tag:'input',type:'text',placeholder:'Write your prompt'},
  {tag:'input',type:'password',name:'access-token'}, {tag:'h1',text:'Top Secret'},{tag:'p',text:'private conversation content'}]);
 const r=record('web_site_capture',{url:origin+'/',output_directory:folder,counts:{outline:6,headings:0,links:0,forms:0,images:0}});
 const atlas=buildAtlas({bundle:bundle(r),siteId:'test',origin,captureDir:folder,ownedTarget:true,includeLabels:true});
 assert.deepEqual(atlas.controls.map(x=>x.role),['link','button','textbox','heading']);
 assert.equal(atlas.controls.find(x=>x.role==='heading').label,null);
 assert.equal(atlas.observations.find(x=>x.kind==='accessibility-summary').unnamedControls,3);
 assert.equal(atlas.controls.some(c=>JSON.stringify(c).includes('Top Secret')||JSON.stringify(c).includes('access-token')),false);
});
test('capture count and design mismatches refuse',()=>{
 const dir=mkdtempSync(join(tmpdir(),'craft-mismatch-'));
 const {folder}=capture(dir,[{tag:'button',text:'Hi'}]);
 const r=record('web_site_capture',{url:origin+'/',output_directory:folder,counts:{outline:10}});
 assert.throws(()=>buildAtlas({bundle:bundle(r),siteId:'test',origin,captureDir:folder}),/CAPTURE_COUNT_MISMATCH/);
});
test('Craft-attested screenshot digest verification and tamper refusal',()=>{
 const dir=mkdtempSync(join(tmpdir(),'craft-png-'));
 const {folder}=capture(dir,[{tag:'a',text:'Home'}]);
 writeFileSync(join(folder,'screenshot-390.png'),png);
 const r=record('web_site_capture',{url:origin+'/',output_directory:folder,counts:{outline:1},screenshots:[{file:'screenshot-390.png',sha256:sha(png)}]});
 const a=buildAtlas({bundle:bundle(r),siteId:'test',origin,captureDir:folder});
 assert.equal(a.observations.find(o=>o.kind==='capture-artifact').verifiedScreenshots,1);
 writeFileSync(join(folder,'screenshot-390.png'),Buffer.from('broken'));
 assert.throws(()=>buildAtlas({bundle:bundle(r),siteId:'test',origin,captureDir:folder}),/CAPTURE_ASSET_(SIZE_INVALID|NOT_PNG|DIGEST_MISMATCH)/);
});
test('state artifact rows must agree exactly with hashed Craft record',()=>{
 const dir=mkdtempSync(join(tmpdir(),'craft-states-')), folder=join(dir,'states');mkdirSync(folder);
 const row={index:0,kind:'tab',label:'Settings',changed:['tab selected']};
 const st={url:origin+'/',width:390,states:[row]};
 writeFileSync(join(folder,'states.json'),JSON.stringify(st));
 const r=record('web_page_states_capture',{url:origin+'/',found:1,states:[row],folder},{width:390});
 const a=buildAtlas({bundle:bundle(r),siteId:'test',origin,statesDir:folder});
 assert.equal(a.observations.find(o=>o.kind==='state-artifact').verifiedScreenshots,0);
 st.states[0]={...st.states[0],changed:['address changed']};writeFileSync(join(folder,'states.json'),JSON.stringify(st));
 assert.throws(()=>buildAtlas({bundle:bundle(r),siteId:'test',origin,statesDir:folder}),/STATES_ARTIFACT_INVALID/);
});
test('source baseline validates scope, discovers selector drift, refuses activation',()=>{
 const atlas=buildAtlas({bundle:bundle(record('web_site_capture',{url:origin+'/'})),siteId:'test',origin});
 const b=profile(); assert.equal(validateProfileBaseline(b,atlas),true);
 const result=reconcileAtlas({atlas,baseline:b});
 assert.equal(result.summary.sourceDivergences,1);
 assert.equal(result.summary.labelCorrespondences,0);
 assert.equal(result.features[0].observationStatus,'not-observed-in-scoped-capture');
 assert.equal(result.authority.executionAuthorized,false);
 assert.throws(()=>reconcileAtlas({atlas,baseline:profile('claude')}),/INVALID_OR_MISMATCHED_BASELINE/);
});
test('label hint cannot promote a selector or execution permission',()=>{
 const dir=mkdtempSync(join(tmpdir(),'craft-label-'));const {folder}=capture(dir,[{tag:'button',text:'Send message'}]);
 const r=record('web_site_capture',{url:origin+'/',output_directory:folder,counts:{outline:1}});
 const atlas=buildAtlas({bundle:bundle(r),siteId:'test',origin,captureDir:folder,ownedTarget:true,includeLabels:true});
 const result=reconcileAtlas({atlas,baseline:profile()});
 assert.equal(result.summary.labelCorrespondences,2);
 assert.ok(result.features.every(f=>f.observationStatus==='label-correspondence-unverified'));
 assert.equal(result.authority.activationPermitted,false);
});
test('diff is read-only and changes count but not inferred capability removal',()=>{
 const r=record('web_site_capture',{url:origin+'/'});
 const a=buildAtlas({bundle:bundle(r),siteId:'test',origin});
 const b=structuredClone(a);b.controls.push({sourceEvidenceId:r.id,position:0,role:'button',label:'Hello'});
 const d=diffAtlases(a,b);
 assert.equal(d.roleCounts.find(x=>x.role==='button').delta,1);
 assert.equal(d.status,'observation-difference-only');
 assert.throws(()=>diffAtlases(a,{...b,target:{...b.target,origin:'https://another.site'}}),/ATLAS_SCOPE_MISMATCH/);
});
test('CLI reconcile and diff use JSON files, no code execution',()=>{
 const dir=mkdtempSync(join(tmpdir(),'craft-cli2-'));
 const r=record('web_site_capture',{url:origin+'/'});
 const a=buildAtlas({bundle:bundle(r),siteId:'test',origin});
 const atlas=join(dir,'atlas.json'),baseline=join(dir,'baseline.json');
 writeFileSync(atlas,JSON.stringify(a));writeFileSync(baseline,JSON.stringify(profile()));
 const cli=resolve(import.meta.dirname,'../src/cli.mjs');
 const reconcile=spawnSync(process.execPath,[cli,'reconcile','--atlas',atlas,'--baseline',baseline,'--out',join(dir,'reconciliation')],{encoding:'utf8'});
 assert.equal(reconcile.status,0,reconcile.stderr);
 assert.equal(JSON.parse(readFileSync(join(dir,'reconciliation/reconciliation.json'),'utf8')).summary.sourceDivergences,1);
 const d=spawnSync(process.execPath,[cli,'diff','--before',atlas,'--after',atlas,'--out',join(dir,'comparison')],{encoding:'utf8'});
 assert.equal(d.status,0,d.stderr);
 assert.equal(JSON.parse(readFileSync(join(dir,'comparison/diff.json'),'utf8')).roleCounts.length,0);
});
