#!/usr/bin/env node
// Run only synthetic performance measurements of this Lane F parser. No native Rust/SQLite claim.
import {createHash} from 'node:crypto';
import {performance} from 'node:perf_hooks';
import {writeFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {parseExchange,nodeId} from './lib/hqkgx1.mjs';
import {quantile} from './lib/placement.mjs';
const digest=s=>createHash('sha256').update(s).digest('hex');
const nodes=Array.from({length:512},(_,i)=>({type:'node',id:nodeId('a:fixture-'+i),kind:'source',title:'synthetic-'+i,metadata:{source_lane:'A',source_root:'synthetic',source_path:'src/fixture-'+i+'.mjs',source_commit:'a'.repeat(40)}}));
const body=nodes.map(n=>JSON.stringify(n)).join('\n')+'\n';
const content=JSON.stringify({type:'manifest',schema:'repos-hq.hqkg-exchange/1',source:'synthetic-512',authority:'data-only; no executable code or provider actions',graph_sha256:digest(body),node_count:nodes.length,edge_count:0})+'\n'+body;
for(let i=0;i<5;i++)parseExchange(content);
const ms=[];for(let i=0;i<50;i++){const t=performance.now();parseExchange(content);ms.push(performance.now()-t)}
const result={schema:'vivim.wave1.lane-f.benchmark/1',status:'SYNTHETIC_CHECKER_ONLY',nodeCount:nodes.length,edgeCount:0,bytes:Buffer.byteLength(content),warmups:5,samples:50,p50Ms:quantile(ms,.5),p95Ms:quantile(ms,.95),rssBytes:process.memoryUsage().rss,compiler:'NOT_MEASURED',rust:'NOT_MEASURED',sqlite:'NOT_MEASURED',placementAuthorized:false};
if(process.argv[2])writeFileSync(resolve(process.argv[2]),JSON.stringify(result,null,2)+'\n',{flag:'wx'});
console.log(JSON.stringify(result));
