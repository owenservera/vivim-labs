// P04 isolated source materializer. Node is preparation tooling only.
import {execFileSync} from 'node:child_process';
import {mkdirSync,writeFileSync,readFileSync,existsSync} from 'node:fs';
import {resolve,join,dirname,relative,isAbsolute} from 'node:path';
import {createHash} from 'node:crypto';
const [labArg,productArg,destArg]=process.argv.slice(2);
if(!labArg||!productArg||!destArg)throw Error('usage: node prepare.mjs LAB_REPO PRODUCT_REPO NEW_SCRATCH');
const lab=resolve(labArg),product=resolve(productArg),dest=resolve(destArg);
for(const root of [lab,product]){const r=relative(root,dest);if(!r||(!r.startsWith('..')&&!isAbsolute(r)))throw Error('destination inside repository');}
mkdirSync(dest,{recursive:false});
const L='ccd15368a20aa3651c64f73ac8571a07950ebfd4',P='bd8d33693a61021a81a2721f18fa87f3069e6836',O='1ba61b437c8c3b2ee95292d999ab8e022f3396d7';
const root='VIVIM V1.0/WINDOWS-NATIVE-INTELLIGENCE/',pack=root+'integration-packs/WNI02-R034-SDK-ADMISSION/',own=root+'integration-packs/WNI02-R036-OWNED-ADMISSION/';
const git=(r,...args)=>execFileSync('git',['-C',r,...args],{maxBuffer:32*1024*1024});
const hash=(bytes)=>createHash('sha1').update('blob '+bytes.length+'\0').update(bytes).digest('hex');
const rows=[];
function put(r,pin,path,out,expected){const bytes=git(r,'show',pin+':'+path);const actual=hash(bytes);const blob=git(r,'rev-parse',pin+':'+path).toString().trim();if(actual!==blob||(expected&&actual!==expected))throw Error('BLOB_MISMATCH '+path);mkdirSync(dirname(out),{recursive:true});writeFileSync(out,bytes,{flag:'wx'});rows.push({repository:r===lab?'Labs':'Product',pin,path,blob,destination:relative(dest,out)});}
const pd=join(dest,'product'),pkg=join(dest,'execution');mkdirSync(pkg);
for(const path of git(product,'ls-tree','-r','--name-only',P).toString().trim().split(/\r?\n/)){if(path)put(product,P,path,join(pd,path));}
for(const n of ['Cargo.toml','native-test-root.rs','resource_campaign.rs','native-sampling.rs','meta-policy.rs','workload-profile.rs','R039-OVERLAY.json','owned-metrics-tests.rs','resource-domain.rs'])put(lab,L,pack+n,join(pkg,n));
for(const n of ['scheduler.rs','resources.rs','hardware.rs'])put(lab,L,root+'rust-starter/src/'+n,join(pkg,n));
for(const n of ['workstation_process.rs','workstation_windows_job.rs','owned_campaign.rs'])put(lab,O,own+n,join(pkg,n));
put(product,P,'crates/protocol-sdk/src/campaign.rs',join(pkg,'campaign.rs'),'3ca396600bfaf8f51ba9636ded04d0e24f7dbd23');
const transforms=[];
function edit(n,from,to){const f=join(pkg,n);let s=readFileSync(f,'utf8');if(s.split(from).length!==2)throw Error('NON_UNIQUE '+n+' '+from);s=s.replace(from,to);writeFileSync(f,s);transforms.push({file:n,from,to});}
const overlay=JSON.parse(readFileSync(join(pkg,'R039-OVERLAY.json'),'utf8'));
for(const f of overlay.files){if(hash(readFileSync(join(pkg,f.name)))!==f.blob)throw Error('SOURCE_DRIFT '+f.name);for(const [a,b] of f.edits)edit(f.name,a,b);}
edit('Cargo.toml','[lib]','[dependencies]\nvivim-protocol-sdk = { path = "../product/crates/protocol-sdk" }\n[lib]');
edit('native-test-root.rs','mod campaign;','pub use vivim_protocol_sdk::{campaign,protocol,meta_language,meta_resident};');
edit('native-test-root.rs','mod protocol;\nmod meta_language;\nmod meta_resident;\n','');
edit('resource_campaign.rs','last:Option<Snapshot>,budget:Budget}','last:Option<Snapshot>,budget:Budget,clock:u64,limit:usize}');
edit('resource_campaign.rs','last:None,budget})','last:None,budget,clock:0,limit})');
edit('resource_campaign.rs','self.campaign.advance(now)?;','self.campaign.advance(now)?;self.clock=now;');
edit('resource_campaign.rs','self.campaign.pending_ids().filter(|id|!held.contains(id))','self.costs.keys().map(String::as_str).filter(|id|self.campaign.pending(id)&&!held.contains(id))');
edit('resource_campaign.rs','let mut deferred=Vec::new();','if self.held.len()>=self.limit&&!pending.is_empty(){return Ok(Admission::Deferred(pending.iter().map(|(id,_)|(id.clone(),Block::Parallel)).collect()))}\n  let mut deferred=Vec::new();');
edit('resource_campaign.rs','self.campaign.claim_selected(worker,now,ttl,Some(id))?.ok_or(Error::Invalid)?','self.campaign.claim_task(id,worker,now,ttl)?');
edit('resource_campaign.rs','self.campaign.submit(lease,now,report)?;Ok(())','self.campaign.advance(now)?;self.clock=now;self.campaign.submit(lease,now,report)?;Ok(())');
edit('resource_campaign.rs','self.campaign.abandon(lease);self.held.remove(&lease.token);Ok(())','if self.campaign.lease(&lease.task.id)==Some(lease){self.campaign.release(lease,self.clock)?;}\n  self.held.remove(&lease.token);Ok(())');
writeFileSync(join(dest,'MATERIALIZATION.json'),JSON.stringify({schema:'vivim.p04.materialization/1',product:P,labs:L,owner:O,rows,transforms,outputs:[...new Set(transforms.map(t=>t.file))].map(file=>({file,blob:hash(readFileSync(join(pkg,file)))}))},null,2));
console.log('P04_EXACT_MATERIALIZATION_OK '+rows.length+' blobs '+transforms.length+' edits');
