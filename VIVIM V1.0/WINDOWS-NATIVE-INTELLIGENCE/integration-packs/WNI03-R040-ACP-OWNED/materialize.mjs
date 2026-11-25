// Native test source extraction uses byte-exact git objects, no production worktree edits.
import {execFileSync} from 'node:child_process';
import {writeFileSync,mkdirSync} from 'node:fs';
import {resolve,join} from 'node:path';
const d=resolve(process.argv[2]);mkdirSync(d,{recursive:true});
const lab=process.argv[3],arbor='ff1de3b70ca87d143301a7f934673acbf9e182e6';
const root='VIVIM V1.0/WINDOWS-NATIVE-INTELLIGENCE';
const entries=[...['Cargo.toml','test-root.rs','owned_acp.rs','fixture.rs'].map(n=>[lab,root+'/integration-packs/WNI03-R040-ACP-OWNED/'+n,n]),...['acp.rs','stream.rs','acp_transport.rs'].map(n=>[lab,root+'/rust-starter/src/'+n,n]),...['workstation_process.rs','workstation_windows_job.rs'].map(n=>[arbor,'native/host-supervisor/src/'+n,n])];
for(const [pin,path,name] of entries){const data=execFileSync('git',['show',pin+':'+path],{maxBuffer:4*1024*1024});writeFileSync(join(d,name),data);const actual=execFileSync('git',['hash-object',join(d,name)],{encoding:'utf8'}).trim();const expected=execFileSync('git',['rev-parse',pin+':'+path],{encoding:'utf8'}).trim();if(actual!==expected)throw Error('BLOB_MISMATCH '+name);}
console.log('ACP_EXACT_BLOBS_OK '+entries.length);
