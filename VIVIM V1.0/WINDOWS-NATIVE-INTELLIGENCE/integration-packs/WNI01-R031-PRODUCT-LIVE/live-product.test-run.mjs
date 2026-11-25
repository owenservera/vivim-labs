// Actual Product resident session with real HQ and Rust graph/SDK binaries.
// All source mutations are confined to a fresh owned temporary fixture.
import assert from 'node:assert/strict';
import {spawn,spawnSync} from 'node:child_process';
import {createInterface} from 'node:readline';
import {mkdtempSync,mkdirSync,writeFileSync,copyFileSync,readFileSync,rmSync} from 'node:fs';
import {resolve,join} from 'node:path';
import {createHash} from 'node:crypto';
import {pathToFileURL} from 'node:url';
import {setTimeout as delay} from 'node:timers/promises';
const [session,arbor,binary,tasks,fixtureParent]=process.argv.slice(2).map(p=>resolve(p));
const base=mkdtempSync(join(fixtureParent,'wni-live-')),source=join(base,'source'),data=join(base,'data');
mkdirSync(source);mkdirSync(data);mkdirSync(join(source,'protocol'));copyFileSync(tasks,join(source,'protocol','tasks.json'));
const git=(...args)=>{const r=spawnSync('git',['-C',source,...args],{windowsHide:true,encoding:'utf8'});if(r.status!==0)throw Error('FIXTURE_GIT_FAILED');};
const path='fixture.rs',file=join(source,path);
writeFileSync(file,'//! Owned fixture.\n// @hq:context Initial observation.\npub fn fixture()->u32{1}\n');writeFileSync(join(source,'stable.rs'),'//! Stable owned fixture.\n// @hq:context Stable source.\npub fn stable()->u32{3}\n');git('init','-q');git('add','fixture.rs','stable.rs','protocol/tasks.json');
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
 const stable=after.compiled.find(c=>c.path==='stable.rs').digest;
 const noChange=await ask({op:'refresh'});
 assert.equal(noChange.graphEpoch,after.graphEpoch);
 assert.equal(noChange.compiled.find(c=>c.path==='stable.rs').digest,stable);
 git('rm','-f','fixture.rs');
 let removed;
 for(let i=0;i<100;i++){await delay(100);removed=await ask({op:'status'});if(!removed.sourceObservation.dirty&&!removed.compiled.some(c=>c.path===path))break;}
 assert.equal(removed.sourceObservation.dirty,false);assert.equal(removed.compiled.some(c=>c.path===path),false);assert.equal(removed.failed.length,0);
 assert.ok(removed.graphEpoch>after.graphEpoch);assert.equal(removed.compiled.find(c=>c.path==='stable.rs').digest,stable);
 const deletedWiki=JSON.parse(readFileSync(removed.wiki.file,'utf8'));assert.equal(deletedWiki.digest,removed.wiki.digest);assert.ok(!deletedWiki.articles.some(a=>a.path?.endsWith('/fixture.rs.md')||a.path==='fixture.rs.md'));
 const {queryDevelopmentGraph}=await import(pathToFileURL(join(arbor,'hq/lib/kg-export.mjs')).href);
 const key='hqg1:'+createHash('sha256').update(['source-file','product-lab',path].join('\0')).digest('hex');
 const deletedGraph=queryDevelopmentGraph({root:data,journal:join(data,'.hq','kg-journal.hqj'),operation:'wiring',key});
 assert.equal(deletedGraph.state,'unavailable');assert.equal(deletedGraph.reason,'native_graph_exit_2');assert.match(deletedGraph.detail,/indexed source file absent/);assert.equal(deletedGraph.result?.source?.digest,undefined,'deleted source digest must not remain visible');
 const siblingKey='hqg1:'+createHash('sha256').update(['source-file','product-lab','stable.rs'].join('\0')).digest('hex');
 const siblingGraph=queryDevelopmentGraph({root:data,journal:join(data,'.hq','kg-journal.hqj'),operation:'wiring',key:siblingKey});assert.equal(siblingGraph.state,'indexed-snapshot');assert.equal(siblingGraph.epoch,removed.graphEpoch);assert.equal(siblingGraph.result?.source?.digest,stable);
 console.log(JSON.stringify({actualProduct:true,actualHq:true,actualNativeGraph:true,actualNativeSdk:true,automaticRefresh:true,sourceDigestChanged:true,graphEpochBefore:before.graphEpoch,graphEpochAfter:after.graphEpoch,wikiReadback:true,focusProof:true,unchangedGraphEpoch:noChange.graphEpoch,deletionGraphEpoch:removed.graphEpoch,deletionReadback:deletedGraph.state,deletedWikiArticle:true,unchangedSibling:true}));
 child.stdin.write('{"op":"quit"}\n');child.stdin.end();
 const exit=await new Promise((resolve,reject)=>{const t=setTimeout(()=>reject(Error('QUIT_TIMEOUT')),10000);child.once('exit',c=>{clearTimeout(t);resolve(c)});});assert.equal(exit,0);
}finally{lines?.close();if(child&&child.exitCode===null){child.kill();await new Promise(r=>child.once('exit',r));}rmSync(base,{recursive:true,force:true});}
