// Read-only validation or write NEW isolated overlay tree. Never modifies ARBOR.
import {readFileSync,writeFileSync,mkdirSync,existsSync,lstatSync,realpathSync} from 'node:fs';
import {resolve,join,dirname,isAbsolute,relative,sep} from 'node:path';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
const home=dirname(fileURLToPath(import.meta.url));
const spec=JSON.parse(readFileSync(join(home,'PATCH-SPEC.json'),'utf8'));
const [source,out,...extra]=process.argv.slice(2);
if(!source||extra.length)throw Error('Usage: node prepare-overlay.mjs <ARBOR checkout> [new-output-directory]');
const base=realpathSync(source),target=out?resolve(out):null;
if(target&&(existsSync(target)||target===base||target.startsWith(base+sep)))throw Error('Choose a new output directory outside ARBOR');
const safe=p=>typeof p==='string'&&!isAbsolute(p)&&p.split('/').every(x=>x&&x!=='.'&&x!=='..'&&!x.includes('\\')&&!x.includes(':'));
const staged=[];
for(const f of spec.patches){
 if(!safe(f.path))throw Error('Unsafe patch path');
 const path=join(base,f.path),real=realpathSync(path);
 if(!real.startsWith(base+sep)||lstatSync(path).isSymbolicLink())throw Error('Unexpected source path');
 const bytes=readFileSync(path),blob=createHash('sha1').update('blob '+bytes.length+'\0').update(bytes).digest('hex');
 if(blob!==f.baseBlob)throw Error('BASE_BLOB_DRIFT '+f.path);
 let text=new TextDecoder('utf-8',{fatal:true}).decode(bytes);
 for(const e of f.edits){if(text.split(e.from).length!==2)throw Error('ANCHOR_DRIFT '+f.path);text=text.replace(e.from,()=>e.to);}
 staged.push([f.path,text]);
}
for(const f of spec.additions){
 if(!safe(f.path)||!safe(f.source)||existsSync(join(base,f.path)))throw Error('ADD_COLLISION '+f.path);
 staged.push([f.path,readFileSync(join(home,f.source))]);
}
if(target){mkdirSync(target,{recursive:false});for(const [path,content] of staged){const p=join(target,path);mkdirSync(dirname(p),{recursive:true});writeFileSync(p,content,{flag:'wx'});}}
console.log(JSON.stringify({baseCommit:spec.baseCommit,verifiedFiles:spec.patches.length,newFiles:spec.additions.length,overlayWritten:!!target,sourceChanged:false}));
