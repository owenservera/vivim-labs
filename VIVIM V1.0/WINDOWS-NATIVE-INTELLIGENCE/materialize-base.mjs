// Materialize exact Git object bytes into a NEW independent directory.
// Read-only source repository, no fetch, checkout, apply or deployment.
import {execFileSync} from 'node:child_process';
import {readFileSync,writeFileSync,mkdirSync,realpathSync,existsSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {resolve,dirname,join,relative,isAbsolute,sep} from 'node:path';
const [repository,specPath,output,...extra]=process.argv.slice(2);
if(!repository||!specPath||!output||extra.length)throw Error('Usage: node materialize-base.mjs <local Git repo> <PATCH-SPEC.json> <new independent output dir>');
const root=realpathSync(repository),dest=resolve(output);
if(existsSync(dest))throw Error('Output exists');
const parent=realpathSync(dirname(dest)),target=join(parent, dest.split(sep).at(-1));
const rel=relative(root,target);
if(!rel||(!isAbsolute(rel)&&rel!=='..'&&!rel.startsWith('..'+sep)))throw Error('Output must be outside source repository');
const spec=JSON.parse(readFileSync(specPath,'utf8'));
const hex=s=>typeof s==='string'&&/^[a-f0-9]{40}$/.test(s);
const safe=p=>typeof p==='string'&&p.length<=512&&!isAbsolute(p)&&!p.includes('\\')&&!p.includes(':')&&p.split('/').every(x=>x&&x!=='.'&&x!=='..');
if(spec.schema!=='vivim.labs.exact-overlay/1'||!hex(spec.baseCommit)||!Array.isArray(spec.patches)||spec.patches.length<1||spec.patches.length>256)throw Error('Invalid spec');
const git=(args)=>execFileSync('git',['-C',root,...args],{maxBuffer:16*1024*1024,windowsHide:true,stdio:['ignore','pipe','pipe']});
if(git(['rev-parse','--verify',spec.baseCommit+'^{commit}']).toString('utf8').trim()!==spec.baseCommit)throw Error('Commit not present');
const files=[],seen=new Set();let total=0;
for(const row of spec.patches){
 if(!safe(row.path)||!hex(row.baseBlob)||seen.has(row.path.toLowerCase()))throw Error('Invalid patch row');
 seen.add(row.path.toLowerCase());
 if(git(['rev-parse',spec.baseCommit+':'+row.path]).toString('utf8').trim()!==row.baseBlob)throw Error('Base blob mismatch '+row.path);
 const data=git(['cat-file','blob',row.baseBlob]);total+=data.length;
 if(total>64*1024*1024)throw Error('Overlay source too large');
 const sha=createHash('sha1').update('blob '+data.length+'\0').update(data).digest('hex');
 if(sha!==row.baseBlob)throw Error('Git blob bytes mismatch');
 files.push({path:row.path,data});
}
mkdirSync(target,{recursive:false});
for(const f of files){const p=join(target,f.path);mkdirSync(dirname(p),{recursive:true});writeFileSync(p,f.data,{flag:'wx'});}
console.log(JSON.stringify({baseCommit:spec.baseCommit,verifiedBlobs:files.length,bytes:total,sourceChanged:false}));
