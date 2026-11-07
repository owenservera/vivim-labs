#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
const dir = path.resolve('capability-observatory/data');
const load = name => JSON.parse(fs.readFileSync(path.join(dir,name),'utf8'));
const caps=load('capabilities.json'),dev=load('dev-preprod.json'),prod=load('production.json'),sources=load('source-catalog.json');
const fail=[];const ids=new Set();
for(const c of caps){if(!/^cap:[a-z0-9-]+$/.test(c.capability_id)) fail.push('invalid capability_id '+c.capability_id);if(ids.has(c.capability_id)) fail.push('duplicate '+c.capability_id);ids.add(c.capability_id);if(!Array.isArray(c.source_refs)||!c.source_refs.length) fail.push('no provenance '+c.capability_id)}
for(const [ledger,rows] of [['dev',dev],['prod',prod]])for(const row of rows){if(!ids.has(row.capability_id))fail.push(ledger+' dangling '+row.capability_id);if(ledger==='prod'&&['deployed','active_verified'].includes(row.state)&&(!row.build_sha||!row.evidence_refs?.length||!row.observed_at))fail.push('unsupported production claim '+row.capability_id)}
for(const s of sources)if(s.intake_state==='unlocated'&&s.measured_items!==null)fail.push('invented count '+s.id);
console.log(JSON.stringify({capabilities:caps.length,dev_rows:dev.length,production_rows:prod.length,sources:sources.length,unlocated_sources:sources.filter(s=>s.intake_state==='unlocated').length,errors:fail},null,2));
if(fail.length)process.exit(1);
