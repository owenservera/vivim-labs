// @hq:context Reuse Labs capability compiler and incumbent Host catalogue validator.
// @hq:invariant All candidates remain disabled; native planner and GPUI only present inert decisions.
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {compile,normalize} from './capability-compiler.mjs';
const read=n=>readFileSync(new URL(n,import.meta.url),'utf8');
const manifest=JSON.parse(read('./CAPABILITY-CANDIDATES.json'));
const current=JSON.parse(read('./action-catalog.v1.json'));
const hostSource=read('./intent-planner.mjs');
// Exact contiguous pure validator region, without importing effectful Host dependencies.
const start=hostSource.indexOf('const err = code');
const end=hostSource.indexOf('export function loadControlCatalog()');
if(start<0||end<=start)throw Error('VALIDATOR_SOURCE_DRIFT');
const validator=hostSource.slice(start,end);
const {validateControlCatalog}=await import('data:text/javascript;base64,'+Buffer.from(validator).toString('base64'));
validateControlCatalog(current);
const rows=compile(manifest);
const ids=new Set(current.actions.map(a=>a.id));
const additions=rows.filter(r=>!ids.has(r.id));
const collisions=rows.filter(r=>ids.has(r.id)).map(r=>r.id);
if(collisions.length)throw Error('EXISTING_ACTION_RECONCILIATION_REQUIRED '+collisions.join(','));
const combined={...current,actions:[...current.actions,...additions.map(r=>r.host)]};
validateControlCatalog(combined);
const cases=rows.map(row=>{
 const mcp={},app={};
 for(const [k,f] of Object.entries(row.fields)){
  if(f.kind==='boolean'){mcp[k]=true;app[k]='enabled';}
  else if(f.kind==='drive'){mcp[k]='c:';app[k]='C:';}
  else if(f.kind==='integer'){mcp[k]=f.max;app[k]=String(f.max);}
  else{mcp[k]='x'.repeat(f.min);app[k]=mcp[k];}
 }
 if(row.host.risk==='R2')mcp.request_id='a'.repeat(32);
 const a=normalize(row,mcp,'mcp'),b=normalize(row,app,'app');
 if(JSON.stringify(a.args)!==JSON.stringify(b.args)||a.executionAuthorized||b.executionAuthorized||row.mcp.registered||row.app.enabled)throw Error('PARITY_OR_AUTHORITY');
 if(Object.hasOwn(a.args,'request_id'))throw Error('TRANSPORT_LEAK');
 const required=row.mcp.inputSchema.required.slice().sort();
 if(JSON.stringify(required)!==JSON.stringify(Object.keys(mcp).sort()))throw Error('MCP_SCHEMA');
 for(const k of Object.keys(app)){
  const bad={...app,[k]:' '+app[k]};let rejected=false;
  try{normalize(row,bad,'app')}catch{rejected=true}
  if(!rejected)throw Error('TRIM_NOT_REJECTED '+row.id+' '+k);
 }
 return {action:row.host,args:a.args,mcp:row.mcp,app:row.app};
});
const out={schema:'vivim.labs.capability-native-parity/1',hostVersion:current.version,hostRows:current.actions.length,candidateRows:rows.length,combinedRows:combined.actions.length,validatorSha256:createHash('sha256').update(validator).digest('hex'),cases};
writeFileSync(new URL('./native-cases.json',import.meta.url),JSON.stringify(out,null,2)+'\n');
console.log('HOST_CATALOG_VALIDATE_OK existing='+current.actions.length+' candidates='+rows.length+' combined='+combined.actions.length+' collisions='+collisions.length);
