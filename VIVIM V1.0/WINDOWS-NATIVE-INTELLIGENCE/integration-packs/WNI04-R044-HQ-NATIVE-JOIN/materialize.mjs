// Byte-exact materialization of current HQ, Product digest, and the Labs native adapter.
import {execFileSync} from 'node:child_process';
import {writeFileSync,mkdirSync} from 'node:fs';
import {resolve,join,dirname} from 'node:path';
const d=resolve(process.argv[2]);mkdirSync(d);
const lab=process.argv[3],arbor='b13b8d67b4adb84ec91612196c8b4257df71968b',product='b02be1d52a2a141c37dbf63c2e24f61d628e1e73';
const root='VIVIM V1.0/WINDOWS-NATIVE-INTELLIGENCE';
const entries=[...['Cargo.toml','probe.rs','test.mjs'].map(n=>[lab,root+'/integration-packs/WNI04-R044-HQ-NATIVE-JOIN/'+n,n]),[lab,root+'/integration-packs/WNI04-R043-NATIVE-PROPERTY/native_property.rs','native_property.rs'],...['file_metadata.rs','indexed_search.rs'].map(n=>[lab,root+'/rust-starter/src/'+n,n]),[lab,root+'/integration-packs/WNI04-R020-SEARCH/WindowsIndex.ps1','WindowsIndex.ps1'],[product,'core/programmability/src/source_digest.rs','source_digest.rs'],[product,'crates/protocol-sdk/src/campaign.rs','campaign-source.rs'],...['schema','capability-schema','indexer','search','util','walk','lang','git','extract','annotation-v2','semantic-registry','semantic-optimizers','rust-syntax-adapter'].map(n=>[arbor,'hq/lib/'+n+'.mjs','hq/lib/'+n+'.mjs'])];
for(const [pin,path,name] of entries){const bytes=execFileSync('git',['show',pin+':'+path],{maxBuffer:4*1024*1024});const target=join(d,name);mkdirSync(dirname(target),{recursive:true});writeFileSync(target,bytes);const actual=execFileSync('git',['hash-object',target],{encoding:'utf8'}).trim();const expected=execFileSync('git',['rev-parse',pin+':'+path],{encoding:'utf8'}).trim();if(actual!==expected)throw Error('BLOB_MISMATCH '+name);}
console.log('R044_EXACT_BLOBS_OK '+entries.length);
