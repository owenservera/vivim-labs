// Byte-exact native ETW experiment; no production worktree writes.
import {execFileSync} from 'node:child_process';
import {writeFileSync,mkdirSync} from 'node:fs';
import {resolve,join} from 'node:path';
const d=resolve(process.argv[2]);mkdirSync(d,{recursive:false});
const lab=process.argv[3],arbor='ff1de3b70ca87d143301a7f934673acbf9e182e6',product='95d046c20d5af0a4e116d6fe6cacc9300203452c';
const root='VIVIM V1.0/WINDOWS-NATIVE-INTELLIGENCE';
const entries=[...['Cargo.toml','test-root.rs','etw_trace.rs'].map(n=>[lab,root+'/integration-packs/WNI02-R041-ETW-COMPILER/'+n,n]),...['etw.rs','etw_decode.rs','etw_record.rs'].map(n=>[lab,root+'/rust-starter/src/'+n,n]),...['workstation_process.rs','workstation_windows_job.rs'].map(n=>[arbor,'native/host-supervisor/src/'+n,n]),[product,'crates/protocol-sdk/src/campaign.rs','campaign.rs']];
for(const [pin,path,name] of entries){const data=execFileSync('git',['show',pin+':'+path],{maxBuffer:4*1024*1024});writeFileSync(join(d,name),data);const actual=execFileSync('git',['hash-object',join(d,name)],{encoding:'utf8'}).trim();const expected=execFileSync('git',['rev-parse',pin+':'+path],{encoding:'utf8'}).trim();if(actual!==expected)throw Error('BLOB_MISMATCH '+name);}
console.log('ETW_EXACT_BLOBS_OK '+entries.length);
