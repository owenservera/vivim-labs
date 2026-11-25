'use strict';
const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),os=require('node:os'),path=require('node:path');
const {createWorkstationTools}=require('../server/workstation-tunnel.cjs');
test('resource projection is opt-in, uses existing Host plan, refuses stale grant',async t=>{
 const dir=fs.mkdtempSync(path.join(os.tmpdir(),'vivim-wni-'));t.after(()=>fs.rmSync(dir,{recursive:true,force:true}));
 const file=path.join(dir,'grant.json'),action='windows.system.resources.read',name='vivim_windows_system_resources_read',rootConfigSha='c'.repeat(64);
 const g={schema:'vivim.workstation-tunnel-grant/1',id:'a'.repeat(32),expiresAt:Date.now()+600000,port:43187,dataRoot:dir,ownerAuthModule:path.join(dir,'owner-auth.mjs'),ownerAuthSha256:'d'.repeat(64),recipeConfigSha:'b'.repeat(64),rootConfigSha,recipes:['node-version'],roots:['repo'],interactiveRecipes:[],allowWrites:false};
 const save=()=>fs.writeFileSync(file,JSON.stringify(g),{mode:0o600});
 let calls=0,wrong=false;
 const opts={directory:dir,credential:async()=> 'fixture-token',fetcher:async(url,o)=>{
  calls++;assert.ok(url.endsWith('/plan'));assert.deepEqual(JSON.parse(o.body),{actionId:action,args:{}});
  return new Response(JSON.stringify({outcome:'done',actionId:action,args:{},effect:'read',result:{schema:'vivim.windows.native-result/1',actionId:action,status:'ok',configSha:wrong?'d'.repeat(64):rootConfigSha,data:{}}}),{status:200});
 }};
 save();assert.ok(!createWorkstationTools(opts).some(x=>x[0]===name));
 g.nativeResources=true;save();
 const tool=createWorkstationTools(opts).find(x=>x[0]===name);assert.ok(tool);
 assert.equal((await tool[3]({})).actionId,action);wrong=true;await assert.rejects(tool[3]({}));wrong=false;
 assert.throws(()=>tool[3]({extra:'x'}));assert.equal(calls,2);
 fs.unlinkSync(file);await assert.rejects(tool[3]({}));assert.equal(calls,2);
});
