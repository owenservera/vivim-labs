// Preparation-only byte-exact acquisition. No network, donor edits, builds or effects.
// Usage: node materialize.mjs MANIFEST.json NEW_SCRATCH_DIRECTORY REPOS.json
// REPOS.json maps ARBOR, Labs, Product to existing local Git repositories.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {existsSync, mkdirSync, readFileSync, realpathSync, writeFileSync} from 'node:fs';
import {dirname, isAbsolute, join, relative, resolve, sep} from 'node:path';
const [manifestPath, destination, rootsPath] = process.argv.slice(2);
if(!manifestPath || !destination || !rootsPath || process.argv.length!==5) throw Error('USAGE');
const m=JSON.parse(readFileSync(manifestPath,'utf8'));
const roots=JSON.parse(readFileSync(rootsPath,'utf8'));
const sha=/^[a-f0-9]{40}$/;
const names=new Set(['ARBOR','Labs','Product']);
const pathOk=p=>typeof p==='string' && p.length<=512 && p.split('/').every(x=>x && x!=='.' && x!=='..' && x.toLowerCase()!=='.git' && !/[\\\\:\\x00-\\x1f<>|?*"]/u.test(x) && !/[. ]$/.test(x) && !/^(con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\\.|$)/i.test(x));
const within=(root,p)=>{const r=relative(root,p);return r===''||(!isAbsolute(r)&&r!=='..'&&!r.startsWith('..'+sep));};
const git=(cwd,args)=>execFileSync('git',['-C',cwd,...args],{maxBuffer:16*1024*1024,windowsHide:true});
if(m.schema!=='vivim.labs.product-preparation/1'||!Array.isArray(m.entries)||m.entries.length<1||m.entries.length>500||m.status!=='preparation-only')throw Error('MANIFEST');
if(existsSync(destination))throw Error('DESTINATION_EXISTS');
const dest=join(realpathSync(dirname(resolve(destination))),resolve(destination).split(sep).at(-1));
const local={};
for(const r of names){
 if(typeof roots[r]!=='string')throw Error('ROOT_REQUIRED '+r);
 const root=realpathSync(roots[r]);
 const actual=realpathSync(git(root,['rev-parse','--show-toplevel']).toString().trim());
 if(actual!==root)throw Error('ROOT_NOT_GIT_TOPLEVEL '+r);
 if(within(root,dest)||within(dest,root))throw Error('DESTINATION_OVERLAPS_REPOSITORY '+r);
 local[r]=root;
}
const seen=new Set();const staged=[];let total=0;
for(const e of m.entries){
 if(!names.has(e.repository)||!sha.test(e.commit)||!sha.test(e.blob)||!pathOk(e.path)||m.pins[e.repository]!==e.commit)throw Error('INVALID_ENTRY');
 const rel=e.repository+'/'+e.path;const key=rel.toLowerCase();if(seen.has(key))throw Error('DUPLICATE_DESTINATION');seen.add(key);
 const cwd=local[e.repository];const spec=e.commit+':'+e.path;
 const actual=git(cwd,['rev-parse','--verify',spec]).toString().trim();
 if(actual!==e.blob)throw Error('PIN_MISMATCH '+rel);
 if(git(cwd,['cat-file','-t',e.blob]).toString().trim()!=='blob')throw Error('NOT_BLOB '+rel);
 const bytes=git(cwd,['cat-file','blob',e.blob]);
 const computed=createHash('sha1').update('blob '+bytes.length+'\0').update(bytes).digest('hex');
 if(computed!==e.blob)throw Error('BLOB_DIGEST_MISMATCH '+rel);
 total+=bytes.length;if(total>64*1024*1024)throw Error('TOTAL_LIMIT');
 staged.push({rel,bytes,receipt:{...e,materializedPath:rel,size:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')}});
}
mkdirSync(dest);
for(const f of staged){const target=join(dest,f.rel);if(!within(dest,target))throw Error('SCOPE_ESCAPE');mkdirSync(dirname(target),{recursive:true});writeFileSync(target,f.bytes,{flag:'wx'});const readback=readFileSync(target);if(!readback.equals(f.bytes))throw Error('READBACK_MISMATCH '+f.rel);}
const receipt={schema:'vivim.labs.preparation-materialization/1',lane:m.lane,entries:staged.map(x=>x.receipt),bytes:total,assurance:'exact-source-readback-only; not dependency closure, compilation, conformance or migration acceptance'};
writeFileSync(join(dest,'MATERIALIZATION.json'),JSON.stringify(receipt,null,2)+'\n',{flag:'wx'});
console.log(JSON.stringify({lane:m.lane,files:staged.length,bytes:total,status:'materialized-readback-verified'}));
