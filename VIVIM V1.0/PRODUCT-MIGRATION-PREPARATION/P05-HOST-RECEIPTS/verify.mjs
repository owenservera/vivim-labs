// Test-only package materializer. No Host listener, grants, or production writes.
import {readFileSync,writeFileSync,mkdirSync,copyFileSync,existsSync,realpathSync} from 'node:fs';
import {resolve,dirname,join,relative,isAbsolute} from 'node:path';
import {fileURLToPath} from 'node:url';
import {execFileSync,spawnSync} from 'node:child_process';
const here=dirname(fileURLToPath(import.meta.url));
const [targetArg,reposArg]=process.argv.slice(2);
if(!targetArg||!reposArg)throw Error('usage: node verify.mjs NEW_SCRATCH REPOS.json');
const repos=JSON.parse(readFileSync(reposArg,'utf8'));
const target=resolve(targetArg);if(existsSync(target))throw Error('scratch already exists');
const parent=realpathSync(dirname(target));const actual=join(parent,relative(dirname(target),target));
for(const key of ['ARBOR','Labs','Product']){const base=realpathSync(repos[key]);const rel=relative(base,actual);if(!rel||(!rel.startsWith('..')&&!isAbsolute(rel)))throw Error('scratch inside '+key);}
mkdirSync(actual);
const spec=JSON.parse(readFileSync(join(here,'closure.json'),'utf8'));
const selected=spec.entries.filter(e=>spec.isolatedSelected.includes(e.path));
const git=(repo,args,input)=>execFileSync('git',['-C',repo,...args],{input,maxBuffer:16*1024*1024});
for(const e of selected){const repo=repos[e.repository];const bytes=git(repo,['cat-file','blob',e.commit+':'+e.path]);const sha=git(repo,['hash-object','--stdin'],bytes).toString().trim();if(sha!==e.blob)throw Error('blob mismatch '+e.path);const dest=join(actual,e.repository,e.path);mkdirSync(dirname(dest),{recursive:true});writeFileSync(dest,bytes);}
const p=join(actual,'ARBOR','native','host-supervisor','src');
writeFileSync(join(p,'lib.rs'),'pub mod workstation_process;\n');
const recovery=selected.find(e=>e.path.endsWith('/rust-starter/src/recovery.rs'));
copyFileSync(join(actual,'Labs',recovery.path),join(p,'recovery.rs'));
writeFileSync(join(p,'lib.rs'),'pub mod workstation_process;\npub mod recovery;\n');
writeFileSync(join(dirname(p),'Cargo.toml'),"[package]\nname = \"vivim-host-supervisor\"\nversion = \"0.1.0\"\nedition = \"2021\"\npublish = false\n[dependencies]\nserde_json = \"1\"\n[target.'cfg(windows)'.dependencies]\nwindows-sys = { version = \"0.61.2\", features = [\"Win32_Foundation\", \"Win32_Security\", \"Win32_System_JobObjects\", \"Win32_System_Pipes\", \"Win32_System_Threading\"] }\n");
copyFileSync(join(here,'journal-regression.test.mjs'),join(actual,'journal-regression.test.mjs'));
const results=[];function run(program,args,env=process.env){const r=spawnSync(program,args,{cwd:actual,env,encoding:'utf8',timeout:300000,maxBuffer:8*1024*1024});const row={program,args,exit:r.status,error:r.error?.message??null,stdout:r.stdout,stderr:r.stderr};results.push(row);console.log(JSON.stringify(row));}
run(process.execPath,['--version']);
run(process.execPath,['--test',join(actual,'ARBOR','apps','vivim-host','test','workstation-terminal-policy.test.mjs'),join(actual,'ARBOR','apps','vivim-host','test','workstation-terminal-request.test.mjs'),join(actual,'journal-regression.test.mjs')],{...process.env,P05_JOURNAL:join(actual,'ARBOR','apps','vivim-host','control','journal.mjs')});
run('rustc',['--version']);run('cargo',['--version']);run('cargo',['test','--offline','--manifest-path',join(dirname(p),'Cargo.toml'),'--','--test-threads=1']);
writeFileSync(join(actual,'P05-RESULTS.json'),JSON.stringify({platform:process.platform,selected,results},null,2));
process.exitCode=results.some(r=>r.exit!==0)?1:0;
