// P03: materialize exact current Product + selected VWin donors in fresh scratch.
import {execFileSync} from 'node:child_process';
import {readFileSync,writeFileSync,mkdirSync,existsSync,realpathSync} from 'node:fs';
import {resolve,join,dirname,relative,isAbsolute,sep} from 'node:path';
import {createHash} from 'node:crypto';
const [product,labs,output]=process.argv.slice(2);
if(!product||!labs||!output)throw Error('usage: node prepare.mjs PRODUCT_REPO LABS_REPO NEW_SCRATCH');
const out=resolve(output),parent=realpathSync(dirname(out));
if(existsSync(out))throw Error('DEST_EXISTS');
for(const root of [product,labs]){const rel=relative(realpathSync(root),parent);if(rel===''||(!isAbsolute(rel)&&rel!=='..'&&!rel.startsWith('..'+sep)))throw Error('DEST_IN_REPO');}
const p='bd8d33693a61021a81a2721f18fa87f3069e6836',l='ccd15368a20aa3651c64f73ac8571a07950ebfd4',w='VIVIM V1.0/WINDOWS-NATIVE-INTELLIGENCE/integration-packs/WNI01-R031-PRODUCT-LIVE/';
const hash=b=>createHash('sha1').update('blob '+b.length+'\0').update(b).digest('hex');
function read(repo,pin,path,expected){const b=execFileSync('git',['-C',repo,'show',pin+':'+path],{maxBuffer:8*1024*1024});const ref=execFileSync('git',['-C',repo,'rev-parse',pin+':'+path],{encoding:'utf8'}).trim();if(hash(b)!==ref||(expected&&ref!==expected))throw Error('BLOB_MISMATCH '+path);return b;}
const stage=[];
function patch(path,base,edits){const b=read(product,p,path,base);let t=new TextDecoder('utf-8',{fatal:true}).decode(b);for(const [from,to] of edits){if(t.split(from).length!==2)throw Error('ANCHOR_DRIFT '+path+' '+from);t=t.replace(from,()=>to);}stage.push([path,Buffer.from(t),hash(b)]);}
const spec=JSON.parse(read(labs,l,w+'PATCH-SPEC.json','57ce58cbde0dee7aebbb752fc7f47eee26d896c3'));
patch('adapters/hq/wiki-articles.mjs','ba9e26c794cabe8738043e1116d20e3e71ff8cc4',spec.patches.find(x=>x.path.endsWith('wiki-articles.mjs')).edits.map(e=>[e.from,e.to]));
patch('adapters/hq/dev-session.mjs','d3397eed888c4b800904b9e175928d93d40d6366',[
["import {publishArticles,readArticle,paneObservation} from './wiki-articles.mjs';","import {publishArticles,readArticle,paneObservation} from './wiki-articles.mjs';\nimport {createArticleCache} from './article-cache.mjs';\nimport {createRefreshLoop,observeRoot} from './refresh-loop.mjs';"],
["const [arbor,source,data,binary,metaBinary]=process.argv.slice(2);","const args=process.argv.slice(2);\nconst watching=args.at(-1)==='--watch';if(watching)args.pop();\nif(args.length<4||args.length>5||args.some(x=>x.startsWith('--')))throw Error('INVALID_SESSION_OPTION');\nconst [arbor,source,data,binary,metaBinary]=args;"],
["let state=null,dict=[],articleSet=null,selected='index';","const articleCache=createArticleCache();\nlet state=null,dict=[],articleSet=null,selected='index';"],
["definitions:dict,sourceRevision:index.generation});","definitions:dict,sourceRevision:index.generation,articleCache});"],
["try{\n if(metaBinary)meta=await startResidentMeta(metaBinary);\n emit({ok:true,op:'ready',result:await refresh()});","const loop=createRefreshLoop({refresh,observe:watching,onError:e=>emit({ok:false,op:'source-observation',error:e.message})});\nlet observer=null;\ntry{\n if(metaBinary)meta=await startResidentMeta(metaBinary);\n if(watching)observer=observeRoot({source,data,loop});\n emit({ok:true,op:'ready',result:await loop.enqueue(loop.perform)});"],
["if(req.op==='quit')break;let result;\n if(req.op==='refresh'||req.op==='source-changed')result=await refresh();\n else if(req.op==='status')result=state;\n else if(req.op==='wiki-index')","if(req.op==='quit')break;const result=await loop.enqueue(async()=>{\n if(req.op==='refresh'||req.op==='source-changed')return await loop.perform();\n if(req.op==='status')return {...state,sourceObservation:loop.status()};\n if(watching)loop.requireCurrent();\n let result;\n if(req.op==='wiki-index')"],
["else throw Error('UNKNOWN_OPERATION');\n emit({ok:true,op:req.op,result});","else throw Error('UNKNOWN_OPERATION');\n return result;});\n emit({ok:true,op:req.op,result});"],
["}finally{meta?.close();runtime.close();}","}finally{observer?.close();await loop.close();meta?.close();runtime.close();}"]
]);
for(const n of ['refresh-loop.mjs','refresh-loop.test.mjs','article-cache.mjs','article-cache.test.mjs']){const path='adapters/hq/'+n;if(execFileSync('git',['-C',product,'ls-tree',p,'--',path],{encoding:'utf8'}).trim())throw Error('ADD_COLLISION '+path);const b=read(labs,l,w+'overlay/'+path);stage.push([path,b,hash(b)]);}
for(const n of ['wiki-articles.test.mjs','protocol-v1.mjs','protocol-v1.test.mjs']){const path='adapters/hq/'+n,b=read(product,p,path);stage.push([path,b,hash(b)]);}
mkdirSync(out);for(const [path,b] of stage){const dst=join(out,path);mkdirSync(dirname(dst),{recursive:true});writeFileSync(dst,b,{flag:'wx'});}
writeFileSync(join(out,'MATERIALIZATION.json'),JSON.stringify(stage.map(([path,b,sourceBlob])=>({path,sourceBlob,outputBlob:hash(b)})),null,2));
console.log(JSON.stringify({verified:stage.length,patched:2,out}));
