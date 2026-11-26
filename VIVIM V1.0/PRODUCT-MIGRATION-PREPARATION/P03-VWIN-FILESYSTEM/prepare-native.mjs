// P03 R044 current-pin source join. Reads exact Git objects; no donor checkout or writes.
import {execFileSync} from 'node:child_process';
import {writeFileSync,mkdirSync,existsSync,realpathSync} from 'node:fs';
import {resolve,join,dirname,relative,isAbsolute,sep} from 'node:path';
import {createHash} from 'node:crypto';
const [labs,product,arbor,outArg]=process.argv.slice(2);
if(!labs||!product||!arbor||!outArg)throw Error('usage: node prepare-native.mjs LABS PRODUCT ARBOR NEW_SCRATCH');
const out=resolve(outArg),parent=realpathSync(dirname(out));if(existsSync(out))throw Error('DEST_EXISTS');
for(const r of [labs,product,arbor]){const rel=relative(realpathSync(r),parent);if(rel===''||(!isAbsolute(rel)&&rel!=='..'&&!rel.startsWith('..'+sep)))throw Error('DEST_IN_REPO');}
const l='ccd15368a20aa3651c64f73ac8571a07950ebfd4',p='bd8d33693a61021a81a2721f18fa87f3069e6836',a='b13b8d67b4adb84ec91612196c8b4257df71968b',w='VIVIM V1.0/WINDOWS-NATIVE-INTELLIGENCE/';
const entries=[...['Cargo.toml','probe.rs','test.mjs'].map(n=>[labs,l,w+'integration-packs/WNI04-R044-HQ-NATIVE-JOIN/'+n,n]),[labs,l,w+'integration-packs/WNI04-R043-NATIVE-PROPERTY/native_property.rs','native_property.rs'],...['file_metadata.rs','indexed_search.rs'].map(n=>[labs,l,w+'rust-starter/src/'+n,n]),[labs,l,w+'integration-packs/WNI04-R020-SEARCH/WindowsIndex.ps1','WindowsIndex.ps1'],[product,p,'core/programmability/src/source_digest.rs','source_digest.rs'],[product,p,'crates/protocol-sdk/src/campaign.rs','campaign-source.rs'],...['schema','capability-schema','indexer','search','util','walk','lang','git','extract','annotation-v2','semantic-registry','semantic-optimizers','rust-syntax-adapter'].map(n=>[arbor,a,'hq/lib/'+n+'.mjs','hq/lib/'+n+'.mjs'])];
const blob=b=>createHash('sha1').update('blob '+b.length+'\0').update(b).digest('hex');
const staged=entries.map(([repo,pin,path,dest])=>{const bytes=execFileSync('git',['-C',repo,'show',pin+':'+path],{maxBuffer:8*1024*1024});const expected=execFileSync('git',['-C',repo,'rev-parse',pin+':'+path],{encoding:'utf8'}).trim();if(blob(bytes)!==expected)throw Error('BLOB_MISMATCH '+path);return {pin,path,dest,blob:expected,bytes};});
mkdirSync(out);for(const {dest,bytes} of staged){const target=join(out,dest);mkdirSync(dirname(target),{recursive:true});writeFileSync(target,bytes,{flag:'wx'});}
writeFileSync(join(out,'MATERIALIZATION.json'),JSON.stringify(staged.map(({bytes,...r})=>r),null,2));
console.log(JSON.stringify({verified:staged.length,out}));
