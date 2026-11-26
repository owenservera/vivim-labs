// @hq:context Native evidence joins the actual incumbent HQ index; no separate index.
// @hq:invariant Never attach a Windows observation to a stale or differently scoped source.
import assert from 'node:assert/strict';
import {mkdtempSync,mkdirSync,copyFileSync,appendFileSync,unlinkSync,rmSync} from 'node:fs';
import {join,resolve} from 'node:path';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {openMemoryDb} from './hq/lib/schema.mjs';
import {indexProject} from './hq/lib/indexer.mjs';
import {findFiles} from './hq/lib/search.mjs';
const here=fileURLToPath(new URL('.',import.meta.url));
const exe=resolve(process.env.VIVIM_NATIVE_SOURCE_PROBE||'');
const dir=mkdtempSync(join(here,'owned-hq-'));const db=openMemoryDb();
function invoke(args,{input,cwd=dir}={}){
 const r=spawnSync(exe,args,{cwd,input,encoding:'utf8',timeout:15000,maxBuffer:512*1024});
 if(r.status!==0)throw Error('NATIVE_PROBE_FAILED '+r.status+' '+(r.stderr||'').slice(0,240));
 return JSON.parse(r.stdout);
}
function index(){
 const r=indexProject({root:dir,db,project:{name:'wni-owned-sdk',path:dir},options:{useGit:false,maxFileBytes:256*1024}});
 assert.equal(r.errors.length,0,JSON.stringify(r.errors));return r;
}
function hq(){
 const r=findFiles(db,{project:'wni-owned-sdk',pattern:'**/campaign.rs',limit:20});
 return r.results.filter(x=>x.project==='wni-owned-sdk'&&x.path==='src/campaign.rs');
}
function bind(observation){
 const rows=hq();if(rows.length!==1)return null;
 const indexed=db.prepare('SELECT f.hash AS digest,f.size AS size FROM files f JOIN projects p ON p.id=f.project_id WHERE p.name=? AND f.path=?').get('wni-owned-sdk','src/campaign.rs');
 if(!indexed||indexed.digest!==observation.sourceDigest||Number(indexed.size)!==observation.size)return null;
 return {project:rows[0].project,path:rows[0].path,digest:indexed.digest,fileId:observation.fileId,size:observation.size,authority:'none'};
}
try{
 mkdirSync(join(dir,'src'));const path=join(dir,'src','campaign.rs');copyFileSync(join(here,'campaign-source.rs'),path);
 const first=index();assert.equal(first.added,1);
 const a=invoke(['--file']);assert.ok(bind(a));
 const script=join(here,'WindowsIndex.ps1');
 const ps=spawnSync('pwsh',['-NoProfile','-Command','. $env:VIVIM_WNI_INDEX_SCRIPT; Invoke-WniIndexedSearch -Root $env:VIVIM_WNI_FIXTURE_ROOT -Name campaign.rs -Limit 20 | ConvertTo-Json -Depth 8 -Compress'],{cwd:dir,env:{...process.env,VIVIM_WNI_INDEX_SCRIPT:script,VIVIM_WNI_FIXTURE_ROOT:dir},encoding:'utf8',timeout:20000,maxBuffer:512*1024});
 if(ps.status!==0)throw Error('WINDOWS_INDEX_ADAPTER_FAILED '+ps.status);
 const indexResult=invoke(['--index'],{input:ps.stdout});
 assert.equal(indexResult.complete,false);assert.ok(['ok','unavailable'].includes(indexResult.status));
 // HQ remains the authority even for successful incomplete index results.
 const acceptedIndexHits=indexResult.hits.filter(h=>h.relative.replaceAll('\\','/')==='src/campaign.rs'&&h.size===a.size&&bind(a)).length;
 appendFileSync(path,'\n// owned native readback change\n','utf8');
 const b=invoke(['--file']);assert.equal(a.fileId,b.fileId);assert.notEqual(a.sourceDigest,b.sourceDigest);
 assert.equal(bind(b),null,'stale HQ digest must not bind');
 const changed=index();assert.equal(changed.changed,1);assert.ok(bind(b));
 unlinkSync(path);const removed=index();assert.equal(removed.removed,1);assert.equal(hq().length,0);assert.equal(bind(b),null);
 console.log(JSON.stringify({event:'HQ_NATIVE_JOIN_OK',initialSize:a.size,updatedSize:b.size,identityPreserved:true,staleRefused:true,deletedRefused:true,windowsIndexStatus:indexResult.status,indexFailureStage:indexResult.stage??null,hresult:indexResult.hresult??null,acceptedIndexHits,hqAdded:first.added,hqChanged:changed.changed,hqRemoved:removed.removed}));
}finally{db.close();rmSync(dir,{recursive:true,force:true});}
