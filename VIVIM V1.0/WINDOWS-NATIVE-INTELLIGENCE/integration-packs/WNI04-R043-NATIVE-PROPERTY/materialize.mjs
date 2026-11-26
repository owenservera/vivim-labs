// Exact Git blob extraction to an isolated new directory.
import {execFileSync} from 'node:child_process';
import {writeFileSync,mkdirSync} from 'node:fs';
import {resolve,join} from 'node:path';
const d=resolve(process.argv[2]);mkdirSync(d);
const pin=process.argv[3],root='VIVIM V1.0/WINDOWS-NATIVE-INTELLIGENCE';
const entries=[...['Cargo.toml','lib.rs','native_property.rs'].map(n=>[root+'/integration-packs/WNI04-R043-NATIVE-PROPERTY/'+n,n]),[root+'/rust-starter/src/file_metadata.rs','file_metadata.rs']];
for(const [path,name] of entries){const data=execFileSync('git',['show',pin+':'+path],{maxBuffer:1024*1024});writeFileSync(join(d,name),data);const actual=execFileSync('git',['hash-object',join(d,name)],{encoding:'utf8'}).trim();const expected=execFileSync('git',['rev-parse',pin+':'+path],{encoding:'utf8'}).trim();if(actual!==expected)throw Error('BLOB_MISMATCH '+name);}
console.log('R043_EXACT_BLOBS_OK '+entries.length);
