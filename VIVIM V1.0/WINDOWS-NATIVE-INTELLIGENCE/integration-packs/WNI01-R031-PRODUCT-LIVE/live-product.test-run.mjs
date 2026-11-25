// Actual Product resident session with real HQ and Rust graph/SDK binaries.
// All source mutations are confined to a fresh owned temporary fixture.
import assert from 'node:assert/strict';
import {spawn,spawnSync} from 'node:child_process';
import {createInterface} from 'node:readline';
import {mkdtempSync,mkdirSync,writeFileSync,copyFileSync,readFileSync,rmSync} from 'node:fs';
import {resolve,join} from 'node:path';
import {setTimeout as delay} from 'node:timers/promises';
const [session,arbor,binary,tasks,fixtureParent]=process.argv.slice(2).map(p=>resolve(p));
const base=mkdtempSync(join(fixtureParent,'wni-live-')),source=join(base,'source'),data=join(base,'data');
mkdirSync(source);mkdirSync(data);mkdirSync(join(source,'protocol'));copyFileSync(tasks,join(source,'protocol','tasks.json'));
const git=(...args)=>{const r=spawnSync('git',['-C',source,...args],{windowsHide:true,encoding:'utf8'});if(r.status!==0)throw Error('FIXTURE_GIT_FAILED');};
const path='fixture.rs',file=join(source,path);
writeFileSync(file,'//! Owned fixture.\n// @hq:context Initial observation.\npub fn fixture()->u32{1}\n');git('init','-q');git('add','fixture.rs','protocol/tasks.json');
let child,lines;
try{
 child=spawn(process.execPath,[session,arbor,source,data,binary,'--watch'],{windowsHide:true,stdio:['pipe','pipe','pipe'],env:process.env});
 child.stderr.resume();lines=createInterface({input:child.stdout,crlfDelay:Infinity});
 const iter=lines[Symbol.asyncIterator]();
 const next=async()=>{let timer;try{const r=await Promise.race([iter.next(),new Promise((_,reject)=>timer=setTimeout(()=>reject(Error('SESSION_TIMEOUT')),120000))]);if(r.done)throw Error('SESSION_CLOSED');const v=JSON.parse(r.value);if(!v.ok)throw Error(v.error||'SESSION_FAILED');return v;}finally{clearTimeout(timer);}};
 const ask=async op=>{child.stdin.write(JSON.stringify(op)+'\n');return (await next()).result;};
 const ready=await next();assert.equal(ready.op,'ready');
 const before=await ask({op:'status'});assert.ok(before.compiled.some(c=>c.path===path));assert.equal(before.failed.length,0);
 const old=before.compiled.find(c=>c.path===path).digest;
 writeFileSync(file,'//! Owned fixture revised.\n// @hq:context Updated observation.\npub fn fixture()->u32{2}\n');
 let after;
 for(let i=0;i<100;i++){await delay(100);after=await ask({op:'status'});if(!after.sourceObservation.dirty&&after.compiled.some(c=>c.path===path&&c.digest!==old))break;}
 assert.ok(after.compiled.some(c=>c.path===path&&c.digest!==old));assert.equal(after.sourceObservation.dirty,false);
 assert.ok(after.index.generation>before.index.generation);assert.ok(after.graphEpoch>before.graphEpoch);
 assert.notEqual(after.wiki.digest,before.wiki.digest);assert.equal(after.failed.length,0);
 const wiki=JSON.parse(readFileSync(after.wiki.file,'utf8'));assert.equal(wiki.digest,after.wiki.digest);assert.equal(wiki.graphEpoch,after.graphEpoch);
 assert.ok(JSON.stringify(wiki).includes('Updated observation'));
 const focus=await ask({op:'focus',path,task:'Understand fixture function',startLine:1,endLine:3});
 assert.equal(focus.schema,'vivim.live-development-context/1');assert.equal(focus.protocol.sourceDigest,after.compiled.find(c=>c.path===path).digest);assert.equal(focus.graph.epoch,after.graphEpoch);
 console.log(JSON.stringify({actualProduct:true,actualHq:true,actualNativeGraph:true,actualNativeSdk:true,automaticRefresh:true,sourceDigestChanged:true,graphEpochBefore:before.graphEpoch,graphEpochAfter:after.graphEpoch,wikiReadback:true,focusProof:true}));
 child.stdin.write('{"op":"quit"}\n');child.stdin.end();
 const exit=await new Promise((resolve,reject)=>{const t=setTimeout(()=>reject(Error('QUIT_TIMEOUT')),10000);child.once('exit',c=>{clearTimeout(t);resolve(c)});});assert.equal(exit,0);
}finally{lines?.close();if(child&&child.exitCode===null){child.kill();await new Promise(r=>child.once('exit',r));}rmSync(base,{recursive:true,force:true});}
