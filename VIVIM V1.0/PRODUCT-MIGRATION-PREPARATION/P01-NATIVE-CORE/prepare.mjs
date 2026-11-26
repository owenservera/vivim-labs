// P01 isolated assembly, using the incumbent parent materializer; no build or production writes.
// node prepare.mjs REPOS.json NEW_SCRATCH_DIRECTORY
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {existsSync,mkdirSync,readFileSync,realpathSync,writeFileSync} from 'node:fs';
import {dirname,isAbsolute,join,relative,resolve,sep} from 'node:path';
import {fileURLToPath} from 'node:url';

const here=dirname(fileURLToPath(import.meta.url));
const [rootsPath,destination]=process.argv.slice(2);
if(!rootsPath||!destination||process.argv.length!==4) throw Error('USAGE');
const readJson=p=>JSON.parse(readFileSync(p,'utf8'));
const roots=readJson(rootsPath);
const manifest=readJson(join(here,'MANIFEST.json'));
const closure=readJson(join(here,'closure.json'));
const adaptation=readJson(join(here,'adaptation.json'));
const hash=(bytes)=>createHash('sha256').update(bytes).digest('hex');
const inside=(root,path)=>{const r=relative(root,path);return r===''||(!isAbsolute(r)&&r!=='..'&&!r.startsWith('..'+sep));};
const canon=p=>{const s=realpathSync(p);const bs=String.fromCharCode(92),prefix=bs+bs+'?'+bs;if(process.platform==='win32'&&s.startsWith(prefix)){const rest=s.slice(prefix.length);if(rest.startsWith('UNC'+bs))return bs+bs+rest.slice(4);if(/^[A-Za-z]:/.test(rest))return rest;}return s;};
if(manifest.lane!=='P01-NATIVE-CORE'||manifest.status!=='preparation-only'||closure.schema!=='vivim.labs.p01.closure/1'||adaptation.schema!=='vivim.labs.p01.adaptation/1')throw Error('SCOPE');
const dest=join(canon(dirname(resolve(destination))),resolve(destination).split(sep).at(-1));
if(existsSync(dest))throw Error('DESTINATION_EXISTS');
for(const n of ['ARBOR','Product','Labs']){
  if(typeof roots[n]!=='string')throw Error('ROOT_REQUIRED '+n);
  const root=canon(roots[n]);
  const actual=canon(execFileSync('git',['-C',root,'rev-parse','--show-toplevel'],{windowsHide:true}).toString().trim());
  if(relative(root,actual)!==''||inside(root,dest)||inside(dest,root))throw Error('DESTINATION_OVERLAP_OR_ROOT '+n);
}
if(closure.donor.rev!==manifest.pins.ARBOR||closure.product.rev!==manifest.pins.Product)throw Error('CLOSURE_PIN_MISMATCH');
const testPath='VIVIM V1.0/PRODUCT-MIGRATION-PREPARATION/P01-NATIVE-CORE/tests/control_contract.rs';
const entries=new Map();
const add=e=>{const k=e.repository+'/'+e.path;const prev=entries.get(k);if(prev&&(prev.blob!==e.blob||prev.commit!==e.commit))throw Error('PIN_CONFLICT '+k);entries.set(k,e);};
for(const e of manifest.entries)add(e);
for(const [repository,group] of [['ARBOR',closure.donor],['Product',closure.product]]){
 for(const r of group.records){
  if(!r.path.startsWith('core/programmability/')&&!(repository==='ARBOR'&&r.path==='apps/vivim-host/control/action-catalog.v1.json'))throw Error('UNEXPECTED_CLOSURE_PATH '+r.path);
  add({repository,commit:group.rev,path:r.path,blob:r.blob,role:r.role||'exact-closure',expectedSha256:r.sha256});
 }
}
add({repository:'Labs',commit:adaptation.sourcePins.testCommit,path:testPath,blob:adaptation.sourcePins.testBlob,role:'Labs qualification fixture'});
const expanded={...manifest,pins:{...manifest.pins,Labs:adaptation.sourcePins.testCommit},entries:[...entries.values()]};
mkdirSync(dest);
const expandedPath=join(dest,'expanded-manifest.json'), materialized=join(dest,'materialized');
writeFileSync(expandedPath,JSON.stringify(expanded,null,2)+'\n',{flag:'wx'});
execFileSync(process.execPath,[join(here,'..','materialize.mjs'),expandedPath,materialized,resolve(rootsPath)],{stdio:'inherit',windowsHide:true});
const receipt=readJson(join(materialized,'MATERIALIZATION.json'));
for(const e of expanded.entries){
  const r=receipt.entries.find(x=>x.repository===e.repository&&x.path===e.path);
  if(!r||r.blob!==e.blob||e.expectedSha256&&r.sha256!==e.expectedSha256)throw Error('MATERIALIZATION_MISMATCH '+e.path);
}
const candidate=join(dest,'candidate');
const result=[];
const write=(path,bytes,origin)=>{
 if(!inside(candidate,path))throw Error('CANDIDATE_ESCAPE');
 mkdirSync(dirname(path),{recursive:true});writeFileSync(path,bytes,{flag:'wx'});
 if(!readFileSync(path).equals(bytes))throw Error('READBACK_MISMATCH '+path);
 result.push({path:relative(candidate,path).split(sep).join('/'),sha256:hash(bytes),origin});
};
const prefix='core/programmability/';
const candidateRule=adaptation.candidate.deterministicRules.find(r=>r.id==='P01-D03');
for(const r of closure.product.records){
 const rel=r.path.slice(prefix.length);
 const bytes=readFileSync(join(materialized,'Product',r.path));
 if(rel==='src/lib.rs'){
  if(r.blob!==candidateRule.targetBaseBlob)throw Error('STALE_ROOT');
  const source=bytes.toString('utf8');
  if(source.split(candidateRule.anchor).length!==2||source.includes('pub mod control_planning;')||source.includes('pub mod control_plan_presentation;'))throw Error('ROOT_ANCHOR_CONFLICT');
  write(join(candidate,rel),Buffer.concat([bytes,Buffer.from(candidateRule.append,'utf8')]),{repository:'Product',blob:r.blob,rule:'P01-D03'});
 }else write(join(candidate,rel),bytes,{repository:'Product',blob:r.blob});
}
for(const name of ['control_planning.rs','control_plan_presentation.rs']){
 const path=prefix+'src/'+name, r=closure.donor.records.find(x=>x.path===path);
 if(!r)throw Error('MISSING_DONOR '+path);
 write(join(candidate,'src',name),readFileSync(join(materialized,'ARBOR',path)),{repository:'ARBOR',blob:r.blob});
}
const lock=closure.donor.records.find(x=>x.path===prefix+'Cargo.lock');
write(join(candidate,'Cargo.lock'),readFileSync(join(materialized,'ARBOR',lock.path)),{repository:'ARBOR',blob:lock.blob,qualificationOnly:true});
write(join(candidate,'tests','control_contract.rs'),readFileSync(join(materialized,'Labs',testPath)),{repository:'Labs',blob:adaptation.sourcePins.testBlob});
writeFileSync(join(dest,'CANDIDATE.json'),JSON.stringify({schema:'vivim.labs.p01.candidate/1',lane:manifest.lane,status:'assembled-not-promoted',pins:expanded.pins,files:result,assurance:'Exact materialization and deterministic overlay only; no SDK conformance or execution authority.'},null,2)+'\n',{flag:'wx'});
console.log(JSON.stringify({candidate,files:result.length,status:'assembled'}));
