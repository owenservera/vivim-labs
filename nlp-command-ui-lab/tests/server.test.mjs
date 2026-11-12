import test from 'node:test';
import assert from 'node:assert/strict';
import { createLabServer } from '../src/server.mjs';

test('standalone loopback server exposes read-only knowledge and command APIs without an effect endpoint',async()=>{
 const server=createLabServer();
 await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
 const port=server.address().port;
 try{
   const origin=`http://127.0.0.1:${port}`;
   const page=await fetch(origin+'/');assert.equal(page.status,200);
   const html=await page.text();assert.match(html,/Context forming/);assert.match(html,/semantic-spine/);assert.match(html,/data-density="xray"/);
   const lens=await fetch(origin+'/semantic-lens.mjs');assert.equal(lens.status,200);
   assert.match(lens.headers.get('content-type'),/javascript/);assert.match(await lens.text(),/semanticSummary/);
   assert.match(page.headers.get('content-security-policy'),/script-src 'self'/);
   const menuCall=await fetch(origin+'/api/interpret',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({text:'email Pet'})});
   const menu=await menuCall.json();assert.equal(menuCall.status,200);assert.equal(menu.menu.kind,'contacts');assert.equal(menu.authorization,'not_assessed');
   assert.equal(menu.executionPerformed,false);
   const protectedCall=await fetch(origin+'/api/interpret',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({text:"don't email Peter"})});
   const protect=await protectedCall.json();assert.equal(protect.outcome,'blocked_negated');assert.equal(protect.menu,null);
   const opts={method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({text:'repos-hq context graph',scope:'repos-hq'})};
   const context=await fetch(origin+'/api/context',opts);
   assert.equal(context.status,200);
   const result=await context.json();assert.equal(result.readOnly,true);assert.equal(result.persisted,false);
   assert.ok(['unavailable','results','no_matches'].includes(result.status));
   assert.equal((await fetch(origin+'/api/context',{...opts,headers:{...opts.headers,Origin:'https://evil.invalid'}})).status,403);
   assert.equal((await fetch(origin+'/api/execute',{...opts})).status,405);
   assert.equal((await fetch(origin+'/api/context',{...opts,body:JSON.stringify({text:'x',scope:'../../etc'})})).status,400);
 }finally{await new Promise(resolve=>server.close(resolve));}
});