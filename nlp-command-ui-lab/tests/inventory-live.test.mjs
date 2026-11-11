import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync,mkdirSync,writeFileSync,rmSync} from 'node:fs';
import {join} from 'node:path';
import {tmpdir} from 'node:os';
import {collectInventory,paths} from '../src/bridge.mjs';
import {visualCompile} from '../src/compiler.mjs';

// Isolated source checkout fixtures: no real repository writes, no command execution.
test('all three mounted authority inventories are projected with independent identifiers and evidence',async()=>{
 const temp=mkdtempSync(join(tmpdir(),'vivim-composer-inventory-'));
 const host=join(temp,'host'),hq=join(temp,'hq'),tierflow=join(temp,'tierflow');
 for(const [folder,segments] of [[host,['apps','vivim-host','control']],[hq,['hq','live-graph','wave1','lane-c']],[tierflow,['src','ui']]]){
   mkdirSync(join(folder,...segments),{recursive:true});
 }
 const file=join(host,'apps/vivim-host/control/action-catalog.v1.json');
 const cat={schema:'vivim.control-action-catalog/1',version:6,actions:[{
   id:'vivim.status.show',title:'Show status',category:'system',risk:'R0',readiness:'verified',
   examples:['show status'],args:{},language:{verbs:['show']}
 }]};
 writeFileSync(file,JSON.stringify(cat));
 writeFileSync(join(hq,'hq/live-graph/wave1/lane-c/registry-adapter.mjs'),
 `export function inventory(){return {schema:'vivim.wave1.command-candidates/1',digest:'fixture-digest',summary:{actions:1,orphanHandlerKeys:[]},sourceFiles:[],records:[{actionId:'hq.wiki.search',purpose:'Search wiki',domain:'wiki',evidenceState:{handlerSource:'DISPATCH_SOURCE_PRESENT',runtime:'UNTESTED'},effects:{risk:'read'}}],cli:[{id:'hq.cli:wiki',syntax:'hq wiki'}]};}`);
 writeFileSync(join(tierflow,'src/ui/types.ts'),`export type UiCommandKind = 'ui.draft.edit' | 'ui.publish';`);
 // The adapter's root resolution is fixed at module load; test explicitly verifies missing mounts
 // are not magically turned into live registrations without the actual local checkouts.
 assert.ok(paths.host);
 const {pathToFileURL}=await import('node:url');
 const {spawnSync}=await import('node:child_process');
 const script=`import {collectInventory} from ${JSON.stringify(pathToFileURL(join(process.cwd(),'src/bridge.mjs')).href)};import {visualCompile} from ${JSON.stringify(pathToFileURL(join(process.cwd(),'src/compiler.mjs')).href)};const inv=await collectInventory();const p=visualCompile('/ui.publish',inv);console.log(JSON.stringify({inv,p}));`;
 try{
   const out=spawnSync(process.execPath,['--input-type=module','-e',script],{
     cwd:process.cwd(),encoding:'utf8',env:{...process.env,VIVIM_HARNESS:host,VIVIM_HQ:hq,VIVIM_TIERFLOW:tierflow}
   });
   assert.equal(out.status,0,out.stderr);
   const {inv,p}=JSON.parse(out.stdout);
   assert.equal(inv.counts.host,1);assert.equal(inv.counts.hq,1);
   assert.equal(inv.counts.cli,1);assert.equal(inv.counts.tierflow,2);
   assert.equal(inv.counts.all,5);
   assert.equal(inv.counts.liveVerified,0,'catalog verified is not a runtime receipt');
   assert.equal(new Set(inv.actions.map(a=>`${a.namespace}:${a.id}`)).size,5);
   assert.ok(inv.actions.every(a=>typeof a.visual?.sigil==='string'));
   assert.equal(p.outcome,'discovery_only');assert.equal(p.namespace,'tierflow-ui');assert.equal(p.executionPerformed,false);
 }finally{rmSync(temp,{recursive:true,force:true});}
});