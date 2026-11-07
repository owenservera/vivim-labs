#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { buildSnapshot } from '../src/observatory.mjs';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'../..');
const lane=path.resolve(root,'lane-d');
const args=process.argv.slice(2);
const arg=name=>{const i=args.indexOf(name);return i>=0?args[i+1]:null};
if(args.some(a=>a.startsWith('--')&&!['--hq','--out','--as-of','--watch'].includes(a)))throw Error('Unknown flag');
const read=file=>JSON.parse(fs.readFileSync(file,'utf8'));
const data=name=>read(path.join(root,'data',name));
const hqPath=arg('--hq')||process.env.VIVIM_HQ_CATALOGUE_EXPORT||null;
const out=path.resolve(arg('--out')||path.join(lane,'.local','snapshots'));
const nameFor=s=>'snapshot-'+s.snapshot_id.slice(7,23)+'.json';
const writeAtomic=(file,contents)=>{const tmp=file+'.tmp-'+process.pid;fs.writeFileSync(tmp,contents,{mode:0o600});fs.renameSync(tmp,file)};
function refresh(){
 let hq=null, missingReason='not_supplied';
 if(hqPath){if(fs.existsSync(hqPath))hq=read(hqPath);else missingReason='file_missing';}
 const snapshot=buildSnapshot({capabilities:data('capabilities.json'),dev:data('dev-preprod.json'),production:data('production.json'),sourceCatalog:data('source-catalog.json'),gates:read(path.join(lane,'data','release-gates.json')),hq,observedAt:arg('--as-of')||new Date().toISOString()});
 fs.mkdirSync(out,{recursive:true});
 const name=nameFor(snapshot),target=path.join(out,name),latest=path.join(out,'latest.json');
 if(!fs.existsSync(target))writeAtomic(target,JSON.stringify(snapshot,null,2)+'\n');
 writeAtomic(latest,JSON.stringify({schema:'vivim.labs.snapshot-pointer/1',snapshot_id:snapshot.snapshot_id,file:name,hq_state:snapshot.summary.hq_export,hq_missing_reason:hq?null:missingReason},null,2)+'\n');
 console.log(JSON.stringify({snapshot_id:snapshot.snapshot_id,file:target,latest,hq_state:snapshot.summary.hq_export,summary:snapshot.summary}));
}
refresh();
if(args.includes('--watch')){
 const watched=['capabilities.json','dev-preprod.json','production.json','source-catalog.json'].map(name=>path.join(root,'data',name)).concat([path.join(lane,'data','release-gates.json')],hqPath?[path.resolve(hqPath)]:[]);
 let pending=false;
 for(const file of watched)fs.watchFile(file,{interval:3000},()=>{if(pending)return;pending=true;setTimeout(()=>{pending=false;try{refresh()}catch(e){console.error('Refresh rejected, previous snapshot retained:',e.message)}},150)});
 console.log(JSON.stringify({watching:watched.map(p=>path.basename(p)),mode:'local-files-only'}));
}
