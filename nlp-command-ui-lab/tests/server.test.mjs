import test from 'node:test';
import assert from 'node:assert/strict';
import { createLabServer } from '../src/server.mjs';

test('standalone loopback server exposes read-only knowledge and command APIs without an effect endpoint',async()=>{
 const server=createLabServer();
 await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
 const port=server.address().port;
 try{
   const origin=`http://127.0.0.1:${port}`;
   const page=await fetch(origin+'/');assert.equal(page.status,200);assert.match(await page.text(),/Context forming/);
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