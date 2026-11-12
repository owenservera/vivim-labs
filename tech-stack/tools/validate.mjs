#!/usr/bin/env node
import { readFileSync, existsSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
const root=dirname(dirname(fileURLToPath(import.meta.url)));
const j=x=>JSON.parse(readFileSync(join(root,x),'utf8'));
const pins=j('catalog/source-pins.json'), cats=j('catalog/current-stack.json'), ws=j('catalog/workspaces.json');
const dep=j('catalog/dependency-edges.json'), watch=j('tracking/release-watch.json');
const opt=j('tracking/optimization-backlog.json'), nxt=j('tracking/adoption-horizon.json');
const fail=[], assert=(b,m)=>{if(!b)fail.push(m)};
assert(/^[a-f0-9]{40}$/.test(pins.commit),'source commit must be SHA40');
assert(pins.commit===cats.source_commit && pins.commit===ws.source_commit && pins.commit===dep.source_commit,'source commits disagree');
assert(pins.commit===watch.source_commit && pins.commit===opt.source_commit,'watch/backlog pin mismatch');
const ids=cats.components.map(c=>c.id), unique=new Set(ids);
assert(unique.size===ids.length,'duplicate technology ids');
for(const c of cats.components){
 assert(c.latest_release===null || (typeof c.latest_release==='string'&&c.latest_checked_at),'unverified latest claim: '+c.id);
 assert(c.resolved_version===null || typeof c.resolved_version==='string','bad resolved version: '+c.id);
 assert(c.source_paths.length>0,'missing source: '+c.id);
 assert(existsSync(join(root,'technologies',c.id,'README.md')),'missing research folder: '+c.id);
}
assert(new Set(watch.items.map(x=>x.id)).size===watch.items.length,'duplicate watch ids');
assert(watch.items.length===ids.length && watch.items.every(x=>unique.has(x.id)),'release watch coverage mismatch');
for(const w of ws.entries)assert(typeof w.path==='string'&&w.path.length>0,'workspace path missing');
const nodes=new Set(dep.nodes);
for(const e of dep.edges)assert(nodes.has(e.from)&&nodes.has(e.to),'edge endpoints missing: '+e.from+' -> '+e.to);
assert(new Set(opt.entries.map(x=>x.id)).size===opt.entries.length,'duplicate optimization ids');
for(const c of nxt.items)assert(opt.entries.some(o=>o.id===c.related_optimization),'unlinked horizon item: '+c.id);
assert(pins.root_lockfiles['Cargo.lock']==='ABSENT_AT_PIN' && pins.root_lockfiles['bun.lock']==='ABSENT_AT_PIN','root lock evidence changed without source recapture');
if(fail.length){console.error(JSON.stringify({status:'FAIL',checks:fail},null,2));process.exitCode=1;}
else console.log(JSON.stringify({status:'PASS',pin:pins.commit,technologies:ids.length,workspaces:ws.entries.length,edges:dep.edges.length,watch:watch.items.length,optimizations:opt.entries.length,horizon:nxt.items.length},null,2));
