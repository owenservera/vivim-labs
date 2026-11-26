// Exact donor fixture preparation. Source schemas are declarations, not a grant.
import {readFileSync,writeFileSync} from 'node:fs';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
const [materialized,arborRepo,out]=process.argv.slice(2);
if(!materialized||!arborRepo||!out)throw Error('USAGE: node fixture.mjs MATERIALIZED ARBOR_REPO OUTPUT');
const arb=join(materialized,'ARBOR'),labs=join(materialized,'Labs','VIVIM V1.0','WINDOWS-NATIVE-INTELLIGENCE');
const pin='b13b8d67b4adb84ec91612196c8b4257df71968b';
const catalogPath='apps/vivim-host/control/action-catalog.v1.json';
const raw=execFileSync('git',['-C',arborRepo,'show',pin+':'+catalogPath],{maxBuffer:8*1024*1024});
if(createHash('sha1').update('blob '+raw.length+'\0').update(raw).digest('hex')!=='03600add09203b1f7eb784577c2f0b4434a5ef64')throw Error('CATALOG_PIN');
const catalog=JSON.parse(raw);
const tunnel=readFileSync(join(arb,'tools/mcp/vivim-arbor/server/workstation-tunnel.cjs'),'utf8');
const profile=readFileSync(join(arb,'tools/mcp/vivim-arbor/server/tunnel-readonly.cjs'),'utf8');
const block=tunnel.match(/const definitions=\[([\s\S]*?)\n\];/)?.[1];if(!block)throw Error('SOURCE_DEFINITIONS');
const source=block.trim().split(/\r?\n/).map(line=>{
 const m=line.trim().match(/^\['([^']+)',(TERMINAL|[SFD]\+'[^']+'|'[^']+'),\[([^\]]*)\],(true|false)\],?$/);
 if(!m)throw Error('UNSUPPORTED_SOURCE_ROW '+line);
 const expr=m[2];const action=expr==='TERMINAL'?'workstation.session.terminal_start':expr.startsWith("'")?expr.slice(1,-1):({S:'workstation.session.',F:'workstation.file.',D:'workstation.directory.'}[expr[0]]+expr.slice(3,-1));
 return {name:m[1],action,fields:[...m[3].matchAll(/'([^']+)'/g)].map(x=>x[1]),mutating:m[4]==='true'};
});
for(const [name,fields,text] of [['vivim_tunnel_health',[],profile],['vivim_workstation_recipes',[],tunnel],['vivim_session_wait',['session_id','stdout_cursor','stderr_cursor','wait_ms'],tunnel]]){
 if(!text.includes("['"+name+"'"))throw Error('MISSING_TRANSPORT_TOOL '+name);
 source.push({name,action:null,fields,mutating:false});
}
// Snapshot of client-discovered typed tool contracts, 2026-11-26. No secrets.
const client=[
  {
    "name": "vivim_tunnel_health",
    "fields": []
  },
  {
    "name": "vivim_workstation_recipes",
    "fields": []
  },
  {
    "name": "vivim_terminal_start",
    "fields": [
      "request_json",
      "request_id"
    ]
  },
  {
    "name": "vivim_session_start",
    "fields": [
      "recipe",
      "request_id"
    ]
  },
  {
    "name": "vivim_session_status",
    "fields": [
      "session_id"
    ]
  },
  {
    "name": "vivim_session_output",
    "fields": [
      "session_id",
      "stdout_cursor",
      "stderr_cursor"
    ]
  },
  {
    "name": "vivim_session_cancel",
    "fields": [
      "session_id",
      "request_id"
    ]
  },
  {
    "name": "vivim_session_input",
    "fields": [
      "session_id",
      "input",
      "request_id"
    ]
  },
  {
    "name": "vivim_file_read",
    "fields": [
      "root_id",
      "path"
    ]
  },
  {
    "name": "vivim_file_inspect",
    "fields": [
      "root_id",
      "path"
    ]
  },
  {
    "name": "vivim_file_read_many",
    "fields": [
      "root_id",
      "path",
      "paths_json"
    ]
  },
  {
    "name": "vivim_file_search_selected",
    "fields": [
      "root_id",
      "path",
      "paths_json",
      "literal"
    ]
  },
  {
    "name": "vivim_directory_list",
    "fields": [
      "root_id",
      "path"
    ]
  },
  {
    "name": "vivim_directory_create",
    "fields": [
      "root_id",
      "path",
      "request_id"
    ]
  },
  {
    "name": "vivim_file_write",
    "fields": [
      "root_id",
      "path",
      "expected_digest",
      "content",
      "request_id"
    ]
  },
  {
    "name": "vivim_file_edit",
    "fields": [
      "root_id",
      "path",
      "expected_digest",
      "old_text",
      "new_text",
      "request_id"
    ]
  }
];
const {compile,normalize}=await import(pathToFileURL(join(labs,'capability-compiler.mjs')).href);
const candidates=compile(JSON.parse(readFileSync(join(labs,'CAPABILITY-CANDIDATES.json'),'utf8')));
const cases=candidates.map(row=>{
 const mcp={};for(const [k,f] of Object.entries(row.fields))mcp[k]=f.kind==='boolean'?true:f.kind==='drive'?'c:':f.kind==='integer'?f.min:'x'.repeat(f.min);
 if(row.host.risk==='R2')mcp.request_id='a'.repeat(32);
 const expectedMcp=normalize(row,mcp,'mcp'),app=expectedMcp.args,expectedApp=normalize(row,app,'app');
 return {row,mcp,app,expectedMcp,expectedApp};
});
writeFileSync(out,JSON.stringify({catalog,source,client,candidates,cases}),{flag:'wx'});
console.log('P06_ACTUAL_FIXTURE source='+source.length+' client='+client.length+' candidates='+candidates.length);
