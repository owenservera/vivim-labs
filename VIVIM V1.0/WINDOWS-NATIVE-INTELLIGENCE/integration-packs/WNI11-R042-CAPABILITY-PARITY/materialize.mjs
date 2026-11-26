// Materialize immutable donor sources in a distinct Labs scratch directory.
import {execFileSync} from 'node:child_process';
import {writeFileSync,mkdirSync} from 'node:fs';
import {resolve,join} from 'node:path';
const dest=resolve(process.argv[2]);mkdirSync(dest);
const lab=process.argv[3],arbor='b13b8d67b4adb84ec91612196c8b4257df71968b';
const root='VIVIM V1.0/WINDOWS-NATIVE-INTELLIGENCE';
const entries=[...['Cargo.toml','lib.rs','parity.rs','generate.mjs'].map(n=>[lab,root+'/integration-packs/WNI11-R042-CAPABILITY-PARITY/'+n,n]),...['capability-compiler.mjs','capability-compiler.test.mjs','CAPABILITY-CANDIDATES.json'].map(n=>[lab,root+'/'+n,n]),...['intent-planner.mjs','action-catalog.v1.json'].map(n=>[arbor,'apps/vivim-host/control/'+n,n]),...['control_planning.rs','control_plan_presentation.rs'].map(n=>[arbor,'core/programmability/src/'+n,n])];
for(const [pin,path,n] of entries){const bytes=execFileSync('git',['show',pin+':'+path],{maxBuffer:4*1024*1024});writeFileSync(join(dest,n),bytes);const actual=execFileSync('git',['hash-object',join(dest,n)],{encoding:'utf8'}).trim();const expected=execFileSync('git',['rev-parse',pin+':'+path],{encoding:'utf8'}).trim();if(actual!==expected)throw Error('BLOB_MISMATCH '+n);}
console.log('R042_EXACT_BLOBS_OK '+entries.length);
