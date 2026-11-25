// Reproduce the R039 cross-campaign test from immutable source pins in a new scratch directory.
import {execFileSync} from 'node:child_process';
import {writeFileSync,mkdirSync,readFileSync} from 'node:fs';
import {resolve,join} from 'node:path';
const dest=resolve(process.argv[2]);
mkdirSync(dest,{recursive:false});
const lab='7ca571dc4e1cbf6023f61ef784fe5e1f8a755768',product='95d046c20d5af0a4e116d6fe6cacc9300203452c',owner='1ba61b437c8c3b2ee95292d999ab8e022f3396d7';
const root='VIVIM V1.0/WINDOWS-NATIVE-INTELLIGENCE',pack=root+'/integration-packs/WNI02-R034-SDK-ADMISSION/';
const files=[...['Cargo.toml','native-test-root.rs','campaign.rs','resource_campaign.rs','native-sampling.rs','meta-policy.rs','workload-profile.rs','R039-OVERLAY.json','owned-metrics-tests.rs','resource-domain.rs'].map(n=>[lab,pack+n,n]),...['scheduler.rs','resources.rs','hardware.rs'].map(n=>[lab,root+'/rust-starter/src/'+n,n]),...['protocol.rs','meta_language.rs','meta_resident.rs'].map(n=>[product,'crates/protocol-sdk/src/'+n,n]),...['workstation_process.rs','workstation_windows_job.rs','owned_campaign.rs'].map(n=>[owner,root+'/integration-packs/WNI02-R036-OWNED-ADMISSION/'+n,n])];
const git=(...args)=>execFileSync('git',args,{maxBuffer:4*1024*1024});
for(const [pin,path,n] of files){const bytes=git('show',pin+':'+path);writeFileSync(join(dest,n),bytes);const actual=git('hash-object',join(dest,n)).toString().trim();const expected=git('rev-parse',pin+':'+path).toString().trim();if(actual!==expected)throw Error('BLOB_MISMATCH '+n);}
const p=JSON.parse(readFileSync(join(dest,'R039-OVERLAY.json'),'utf8'));
for(const f of p.files){const file=join(dest,f.name);if(git('hash-object',file).toString().trim()!==f.blob)throw Error('SOURCE_DRIFT '+f.name);let s=readFileSync(file,'utf8');for(const [from,to] of f.edits){if(s.split(from).length!==2)throw Error('NON_UNIQUE '+f.name);s=s.replace(from,to);}writeFileSync(file,s,'utf8');}
console.log('R039_EXACT_MATERIALIZATION_OK '+files.length);
