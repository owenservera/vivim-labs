import {test} from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync,mkdirSync,readFileSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join,resolve} from 'node:path';
import {spawnSync} from 'node:child_process';
import {buildAtlas,evidenceDigest,ARTIFACT_INDEX_SCHEMA,validateArtifactIndex} from '../src/adapter.mjs';
const origin='https://test.example';
const bundle=(...records)=>({format:'craft-anything-evidence-bundle',version:1,records,unknowns:[]});
function page(url,folder,outline){
 const r={subject:{kind:'website'},operation:'web_site_capture',params:{widths:[390]},
  result:{url,output_directory:folder,counts:{outline:outline.length}},
  confidence:'observed',authority:'our-run',environment:{clean_room:true,engine:'cdp',host:'test'},
  coverage:{status:'complete',examined:1,total:1},locations:[{kind:'url',value:url}],limitations:[],links:[]};
 r.id=evidenceDigest(r);mkdirSync(folder);writeFileSync(join(folder,'capture.json'),JSON.stringify({url,outline}));return r;
}
test('one source-linked index imports multiple captured app routes',()=>{
 const dir=mkdtempSync(join(tmpdir(),'craft-multi-'));
 const a=page(origin+'/chat',join(dir,'chat'),[{tag:'button',text:'Send'}]);
 const b=page(origin+'/settings',join(dir,'settings'),[{tag:'a',text:'Home'}]);
 const index={schema:ARTIFACT_INDEX_SCHEMA,captures:[{evidenceId:a.id,directory:join(dir,'chat')},{evidenceId:b.id,directory:join(dir,'settings')}],states:[]};
 const atlas=buildAtlas({bundle:bundle(a,b),siteId:'test',origin,artifactIndex:index,includeLabels:true,ownedTarget:true});
 assert.equal(atlas.controls.length,2);
 assert.equal(atlas.source.captureArtifacts,2);
 assert.equal(new Set(atlas.observations.filter(o=>o.kind==='website-capture').map(o=>o.pageRef)).size,2);
 assert.equal(JSON.stringify(atlas).includes('/settings'),false);
 assert.equal(JSON.stringify(atlas).includes('/chat'),false);
});
test('index requires each Craft evidence ID to match correct operation and folder',()=>{
 const dir=mkdtempSync(join(tmpdir(),'craft-idx-'));
 const a=page(origin+'/chat',join(dir,'chat'),[]);
 const index={schema:ARTIFACT_INDEX_SCHEMA,captures:[{evidenceId:a.id,directory:join(dir,'chat')},{evidenceId:a.id,directory:join(dir,'chat')}],states:[]};
 assert.throws(()=>buildAtlas({bundle:bundle(a),siteId:'test',origin,artifactIndex:index}),/ARTIFACT_INDEX_ENTRY_INVALID/);
 const wrong={...index,captures:[{evidenceId:'ev_'+'0'.repeat(64),directory:join(dir,'chat')}]};
 assert.throws(()=>buildAtlas({bundle:bundle(a),siteId:'test',origin,artifactIndex:wrong}),/ARTIFACT_INDEX_ENTRY_INVALID/);
 assert.throws(()=>buildAtlas({bundle:bundle(a),siteId:'test',origin,captureDir:join(dir,'chat'),artifactIndex:{schema:ARTIFACT_INDEX_SCHEMA,captures:[],states:[]}}),/AMBIGUOUS_ARTIFACT_SOURCE/);
});
test('multi-route CLI accepts exact index and never overwrites output',()=>{
 const dir=mkdtempSync(join(tmpdir(),'craft-idxcli-'));
 const a=page(origin+'/chat',join(dir,'chat'),[{tag:'button',text:'Send'}]);
 const index={schema:ARTIFACT_INDEX_SCHEMA,captures:[{evidenceId:a.id,directory:join(dir,'chat')}],states:[]};
 const input=join(dir,'bundle.json'),indexfile=join(dir,'index.json'),output=join(dir,'result');
 writeFileSync(input,JSON.stringify(bundle(a)));writeFileSync(indexfile,JSON.stringify(index));
 const cli=resolve(import.meta.dirname,'../src/cli.mjs');
 const args=[cli,'import','--bundle',input,'--site','test','--origin',origin,'--artifact-index',indexfile,'--out',output];
 const first=spawnSync(process.execPath,args,{encoding:'utf8'});
 assert.equal(first.status,0,first.stderr);
 assert.equal(JSON.parse(readFileSync(join(output,'atlas.json'))).source.captureArtifacts,1);
 const again=spawnSync(process.execPath,args,{encoding:'utf8'});
 assert.equal(again.status,1);
});
test('control count bounded by atlas schema across many captures',()=>{
 const dir=mkdtempSync(join(tmpdir(),'craft-large-'));
 const records=[],captures=[];
 for(let i=0;i<6;i++){
   const folder=join(dir,'capture-'+i);
   const r=page(origin+'/route-'+i,folder,Array.from({length:900},()=>({tag:'button'})));
   records.push(r);captures.push({evidenceId:r.id,directory:folder});
 }
 assert.throws(()=>buildAtlas({bundle:bundle(...records),siteId:'test',origin,
   artifactIndex:{schema:ARTIFACT_INDEX_SCHEMA,captures,states:[]}}),/ATLAS_ITEM_LIMIT_EXCEEDED/);
});
