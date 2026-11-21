#!/usr/bin/env node
// Lane F independent acceptance: read evidence, optionally run native tests, never self-certify Rust.
import {readFileSync,writeFileSync,mkdirSync,existsSync} from 'node:fs';
import {resolve,dirname,relative,isAbsolute,join} from 'node:path';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {performance} from 'node:perf_hooks';
import {pathToFileURL} from 'node:url';
export const SCHEMA='vivim.wave1.lane-f/1';
export const REQUIRED=['hq-test','cargo-test','rust-import','rust-noop','rust-restart','rust-invalid','rust-collision','rust-stale','rust-secret','where','what','talks-to','gap','wiki'];
const CATS=['exact','negation','quotation','conjunction','ambiguity','typo','missing-slot','unsupported','wrong-target'];
const PLACEMENTS=['RUST_NATIVE_NOW','RUST_ADAPTER','SQLITE_CANONICAL','SQLITE_DERIVED','KEEP_EXTERNAL_OWNER','DEFER_MEASUREMENT'];
const PROHIBITED=/(^|[\s/-])(reset|wipe|remove|delete|clean|publish|deploy|install|activate|push|force)([\s/-]|$)/i;
const hash=buf=>createHash('sha256').update(buf).digest('hex');
const error=e=>String(e?.message??e).slice(0,250);
const inside=(root,p)=>{const r=relative(root,p);return r===''||(!r.startsWith('..')&&!isAbsolute(r));};
const gap=(kind,id,detail,severity='high',evidence=null)=>({kind,id,detail,severity,evidence});
function evidence(root,name,label,missing){
 if(!name){missing.push(gap('untested',label,'No evidence path supplied','critical'));return null;}
 const p=resolve(root,name);if(!inside(root,p)){missing.push(gap('unsafe',label,'Evidence path escapes workspace','critical'));return null;}
 try{const b=readFileSync(p);if(b.length>32*1024*1024)throw Error('size limit');return {path:relative(root,p),sha256:hash(b),bytes:b.length,raw:b.toString('utf8')};}
 catch(e){missing.push(gap('untested',label,error(e),'critical'));return null;}
}
const json=(e,id,gaps)=>{if(!e)return null;try{return JSON.parse(e.raw);}catch(err){gaps.push(gap('untested',id,'Invalid JSON: '+error(err),'critical'));return null;}};
function commands(config,root,doRun,gaps){
 const seen=new Set(),out=[];const journal=config.files?.journal?resolve(root,config.files.journal):null;
 for(const c of config.commands??[]){
  if(!c.id||seen.has(c.id)){gaps.push(gap('duplicate',`command:${c.id}`,'Duplicate/empty command ID'));continue;}seen.add(c.id);
  const cwd=resolve(root,c.cwd??'.'),argv=c.argv;
  if(!inside(root,cwd)||!Array.isArray(argv)||!['cargo','node','bun'].includes(argv[0])||argv.some(a=>typeof a!=='string'||PROHIBITED.test(a))){out.push({id:c.id,status:'REFUSED'});gaps.push(gap('unsafe',`command:${c.id}`,'Unapproved executable, arguments, or cwd','critical'));continue;}
  if(!doRun){out.push({id:c.id,status:'NOT_EXECUTED'});continue;}
  const before=journal&&inside(root,journal)&&existsSync(journal)?hash(readFileSync(journal)):null,start=performance.now();
  const r=spawnSync(argv[0],argv.slice(1),{cwd,shell:false,encoding:'utf8',timeout:Math.min(c.timeoutMs??30000,120000),maxBuffer:1024*1024});
  const after=journal&&inside(root,journal)&&existsSync(journal)?hash(readFileSync(journal)):null;
  const expected=c.expectedExit??0,match=!c.expectedText||`${r.stdout??''}\n${r.stderr??''}`.includes(c.expectedText);
  const unchanged=!c.journalMustNotChange||before===after;
  const result={id:c.id,argv,exitCode:r.status??null,expectedExit:expected,status:!r.error&&r.status===expected&&match&&unchanged?'OK':'FAILED',durationMs:Math.round(performance.now()-start),journalBefore:before,journalAfter:after,stdout:String(r.stdout??'').slice(0,10000),stderr:String(r.stderr??'').slice(0,10000),error:r.error?error(r.error):null};
  // Never store raw credential-like values accidentally printed by an external CLI.
  for(const k of ['stdout','stderr'])result[k]=result[k].replace(/((token|password|secret|authorization|cookie)\s*[:=]\s*)[^\s]+/gi,'$1[REDACTED]');
  out.push(result);if(result.status!=='OK')gaps.push(gap('untested',`command:${c.id}`,'Non-passing native receipt','critical'));
 }
 for(const id of REQUIRED){const r=out.find(x=>x.id===id&&x.status==='OK');if(!r)gaps.push(gap('untested',`required:${id}`,'Missing successful native execution','critical'));
  if(r&&id==='rust-noop'&&(!r.stdout.includes('wrote_snapshot=false')||r.journalBefore!==r.journalAfter))gaps.push(gap('drift','rust-noop','No no-op + unchanged hash proof','critical'));
  if(r&&id.startsWith('rust-')&&['rust-invalid','rust-collision','rust-stale','rust-secret'].includes(id)&&(r.exitCode===0||r.journalBefore!==r.journalAfter))gaps.push(gap('unsafe',id,'Refusal must fail with unchanged journal','critical'));
 }
 return out;
}
export function assess(config,root,{execute=false}={}){
 if(config.schema!==SCHEMA)throw Error('Unexpected manifest schema');const gaps=[],files=config.files??{},roots=config.project?.roots??[];
 if(config.project?.id!=='vivim'||!roots.length)gaps.push(gap('unknown','project','One VIVIM project and root inventory not established','critical'));
 for(const r of roots){if(!/^[a-f0-9]{40}$/i.test(r.head??'')||r.indexed!==true)gaps.push(gap('untested',`root:${r.id}`,'Unverified source SHA or indexing receipt','critical'));}
 const reset=json(evidence(root,files.reset,'scoped-reset',gaps),'scoped-reset',gaps);
 if(!reset?.verified||!reset?.archiveVerified||!reset?.durableKbPreserved||!reset?.oldIndexRetired||!reset?.intentVerified)gaps.push(gap('untested','reset-safety','No local reset-preservation/intent proof','critical'));
 const gr=json(evidence(root,files.graph,'rust-graph',gaps),'rust-graph',gaps),wk=evidence(root,files.wiki,'wiki-output',gaps);
 const nodes=Array.isArray(gr?.nodes)?gr.nodes:[],edges=Array.isArray(gr?.edges)?gr.edges:[];
 const buckets=[nodes.filter(n=>/source|code|symbol/i.test(n.kind)),nodes.filter(n=>/semantic|knowledge|kb|decision/i.test(n.kind)),nodes.filter(n=>/command|action|cli|operation/i.test(n.kind))];
 if(buckets.some(x=>!x.length))gaps.push(gap('missing','A-B-C-in-Rust','Rust graph lacks independently evidenced code/KB/command domain','critical'));
 const ids=new Set(nodes.map(n=>n.id));for(const n of nodes)if(!n.sourceSha)gaps.push(gap('unknown',`source:${n.id}`,'Missing source anchor'));
 for(const e of edges){if(!ids.has(e.from)||!ids.has(e.to))gaps.push(gap('orphaned',`${e.from}->${e.to}`,'Missing graph endpoint'));if(!e.evidenceClass)gaps.push(gap('unknown',`edge:${e.from}->${e.to}`,'Static vs observed status unknown'));}
 let chain=null;for(const n of buckets[0])for(const e of edges.filter(e=>e.from===n.id&&buckets[1].some(k=>k.id===e.to))){const f=edges.find(f=>f.from===e.to&&buckets[2].some(c=>c.id===f.to));if(f){chain=[n.id,e.to,f.to];break;}if(chain)break;}
 if(!chain)gaps.push(gap('missing','source-kb-command-chain','Unproven Rust cross-domain traversal','critical'));
 if(chain&&!chain.every(id=>wk?.raw.includes(id)))gaps.push(gap('drift','wiki-chain','Wiki disagrees with Rust/source chain','critical'));
 for(const n of buckets[2].filter(n=>n.status==='declared'||n.wired===false))gaps.push(gap('declared-but-unwired',n.id,'No verified handler'));
 const nl=json(evidence(root,files.intent,'intent-safety',gaps),'intent-safety',gaps),cases=nl?.cases??[];
 for(const cat of CATS)if(!cases.some(c=>c.category===cat&&c.sourceSha&&c.observedStatus))gaps.push(gap('untested',`nl:${cat}`,'No source-pinned observed case','critical'));
 for(const c of cases)if(c.observedStatus!==c.expectedStatus||(c.formedExecutablePlan&&['negation','quotation','conjunction'].includes(c.category)))gaps.push(gap('unsafe',`nl:${c.id}`,'Incorrect or executable unsafe language result','critical'));
 const comp=json(evidence(root,files.compiler,'compiler-conformance',gaps),'compiler-conformance',gaps),tests=comp?.cases??[];
 if(!tests.length||tests.some(t=>t.pass!==true||!t.sourceSha))gaps.push(gap('untested','compiler-conformance','Native compiler/linker crosswalk not proven','critical'));
 const pl=json(evidence(root,files.placement,'placement-native',gaps),'placement-native',gaps),rows=pl?.components??[];
 const expected=['source-index','comment-compiler','kb-semantic','command-routing','rust-graph','sqlite-search','wiki','host-effects','ui-document'];
 const placements=expected.map(id=>{const r=rows.find(x=>x.id===id);if(!r?.owner||!r.evidence?.length)gaps.push(gap('untested',`placement:${id}`,'No measured owner/benchmark evidence','critical'));return {id,owner:r?.owner??'UNKNOWN',recommendation:r&&r.evidence?.length&&PLACEMENTS.includes(r.recommendation)?r.recommendation:'DEFER_MEASUREMENT',evidence:r?.evidence??[]};});
 const jr=evidence(root,files.journal,'rust-dev-journal',gaps);
 const receipts=commands(config,root,execute,gaps);
 const status=gaps.length?'BLOCKED':'PASS';
 return {status,receipts,gaps,graph:{nodes:nodes.length,edges:edges.length,chain},roots:{indexed:roots.filter(r=>r.indexed).length,total:roots.length},artifacts:{graph:gr?hash(JSON.stringify(gr)):null,wiki:wk?.sha256??null,journal:jr?.sha256??null},placement:placements,compiler:{cases:tests.length,passed:tests.filter(t=>t.pass&&t.sourceSha).length},intent:{cases:cases.length,unsafe:gaps.filter(g=>g.id.startsWith('nl:')&&g.kind==='unsafe').length}};
}
export function write(result,out){mkdirSync(out,{recursive:true});const emit=(n,value)=>writeFileSync(join(out,n),typeof value==='string'?value:JSON.stringify(value,null,2)+'\n');
 emit('graph-vs-target.json',{schema:SCHEMA,status:result.status,sourceRoots:result.roots,graph:result.graph,artifacts:result.artifacts,gaps:result.gaps});
 emit('blind-spot-register.json',{schema:SCHEMA,items:result.gaps});emit('raw-command-receipts.json',result.receipts);
 emit('compile-conformance.json',{schema:SCHEMA,status:result.status,...result.compiler});
 emit('intent-safety-baseline.json',{schema:SCHEMA,status:result.status,...result.intent});
 emit('rust-sqlite-placement.json',{schema:SCHEMA,status:result.status,components:result.placement});
 emit('rust-sqlite-placement.md',`# Native Rust vs SQLite\n\nStatus: ${result.status}\n\n`+result.placement.map(x=>`- ${x.id}: ${x.recommendation} (${x.owner}); evidence ${x.evidence.join(', ')||'MISSING'}`).join('\n')+'\n');
 emit('graph-vs-target.md',`# Wave 1 independent F verification\n\n**${result.status}**; indexed roots ${result.roots.indexed}/${result.roots.total}; Rust nodes/edges ${result.graph.nodes}/${result.graph.edges}; chain ${result.graph.chain?.join(' → ')??'NOT PROVEN'}.\n\nSee machine report for ${result.gaps.length} distinct findings.\n`);
 emit('WAVE-2-INPUT-DOSSIER.md',`# Wave 2 INPUT DOSSIER — not a Wave 2 design\n\nGate: ${result.status}.\n\n## Evidence gaps to resolve before design\n`+result.gaps.map(g=>`- ${g.id}: ${g.detail}`).join('\n')+'\n\nPreserve future typed visual-language source spans, clause and quotation structure, alternatives, effect preview, input/world revisions, and source/authority refs. Do not assign Wave 2 lanes until an executed integrated runtime has been evaluated.\n');
}
export function cli(args=process.argv.slice(2)){const i=args.indexOf('--manifest'),j=args.indexOf('--out');if(i<0||j<0||!args[i+1]||!args[j+1])throw Error('Usage: node acceptance.mjs --manifest RUN.json --out ./reports [--execute]');const path=resolve(args[i+1]),root=dirname(path),out=resolve(args[j+1]);if(!inside(root,out))throw Error('Reports must remain inside manifest workspace');const r=assess(JSON.parse(readFileSync(path,'utf8')),root,{execute:args.includes('--execute')});write(r,out);console.log(JSON.stringify({status:r.status,gaps:r.gaps.length,receipts:r.receipts.length}));process.exitCode=r.status==='PASS'?0:2;}
if(process.argv[1]&&import.meta.url===pathToFileURL(resolve(process.argv[1])).href){try{cli();}catch(e){console.error(error(e));process.exitCode=2;}}