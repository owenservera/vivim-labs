// All graph/wiki test data here is SYNTHETIC. No production pass is implied.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdtempSync, mkdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { parseExchange, readWikiPages, assessWiki, main } from './wiki-oracle.mjs';
const id = s => 'hqg1:'+createHash('sha256').update(s).digest('hex');
const key = s => createHash('sha256').update(s).digest('hex');
const node = (name,kind,metadata) => ({type:'node',id:id(name),kind,title:name,metadata});
const edge = (name,from,to,relation='defines',metadata={link_status:'verified',evidence_ref:'test:synthetic'})=>({type:'edge',id:id(name),from:id(from),to:id(to),relation,weight:1,metadata});
const baseline = () => ([
  node('code','code',{source_repo:'repos-hq',source_path:'hq/lib/action-router.mjs',source_sha:'a'.repeat(40)}),
  node('kb','knowledge',{description:'Reviewed decision',status:'accepted'}),
  node('semantic','semantic',{semantic_id:'vivim.graph.inspect@1',description:'Inspect graph',status:'current'}),
  node('command','command',{action_id:'vivim.graph.inspect@1',status:'wired',syntax:'hq graph inspect',handler:'hq/bin/hq.mjs'}),
  edge('code-sem','code','semantic'),
  edge('sem-cmd','semantic','command','binds'),
]);
const wikiGood = '# Graph inspection\n\n' + [
 'hq/lib/action-router.mjs', 'a'.repeat(40), 'vivim.graph.inspect@1', 'hq graph inspect',
].join('\n\n') + '\n';
function exchange(records=baseline()) {
  const payload=records.map(r=>JSON.stringify(r)).join('\n')+'\n';
  const head={type:'manifest',schema:'repos-hq.hqkg-exchange/1',source:'test:synthetic',authority:'data-only; no executable code or provider actions',graph_sha256:key(payload),node_count:records.filter(x=>x.type==='node').length,edge_count:records.filter(x=>x.type==='edge').length};
  return JSON.stringify(head)+'\n'+payload;
}
const graph = (rows=baseline())=>parseExchange(exchange(rows));
const pages = (body=wikiGood)=>new Map([['integration.md',body],['index.md','# Index\n\n[Integration](./integration.md)\n']]);
const quality = (rows=baseline(),p=pages())=>assessWiki({exchange:graph(rows),pages:p});

test('HQKGX1 transport: valid manifest and counts',()=>{
 const g=graph(); assert.equal(g.nodes.length,4); assert.equal(g.edges.length,2);
});
test('HQKGX1 refuses body tamper and missing final newline',()=>{
 assert.throws(()=>parseExchange(exchange().replace('Inspect graph','Modify graph')),/exchange_digest_mismatch/);
 assert.throws(()=>parseExchange(exchange().slice(0,-1)),/invalid_exchange_size_or_newline/);
});
test('HQKGX1 refuses unsupported authority',()=>{
 const s=exchange().replace('data-only; no executable code or provider actions','allow actions');
 assert.throws(()=>parseExchange(s),/invalid_authority/);
});
test('HQKGX1 refuses duplicate node and edge identities',()=>{
 const b=baseline(); assert.throws(()=>graph([...b.slice(0,4),b[1],...b.slice(4)]),/duplicate_node/);
 assert.throws(()=>graph([...b,b[5]]),/duplicate_edge/);
});
test('HQKGX1 refuses missing endpoints',()=>{
 const b=baseline();b[5].to=id('absent');assert.throws(()=>graph(b),/orphan_edge/);
});
test('HQKGX1 refuses unsafe metadata and noncanonical identity',()=>{
 const b=baseline();b[0].metadata.api_key='should not be read';assert.throws(()=>graph(b),/secret_metadata_key/);
 const c=baseline();c[0].id='bogus';assert.throws(()=>graph(c),/invalid_record_id/);
});
test('quality oracle grades a fully linked SYNTHETIC page but does not self-certify runtime',()=>{
 const v=quality();assert.equal(v.totals.verifiedWikiChains,1);assert.equal(v.totals.domains.knowledge,1);
 assert.equal(v.verdict,'BLOCKED');assert.equal(v.rustEvidenceIndependentlyVerified,false);
});
test('a claimed runtime receipt can only produce READY_FOR_INDEPENDENT_VERIFICATION',()=>{
 const g=graph();const v=assessWiki({exchange:g,pages:pages(),rustRuntimeEvidence:{observed:true,exchangeDigest:g.manifest.graph_sha256,journalSha:'b'.repeat(64)}});
 assert.equal(v.verdict,'READY_FOR_INDEPENDENT_VERIFICATION');assert.equal(v.rustEvidenceIndependentlyVerified,false);
});
test('false semantic description remains a missing meaning defect',()=>{
 const b=baseline();delete b[2].metadata.description;
 const v=quality(b);assert.equal(v.totals.verifiedWikiChains,0);assert(v.defects.some(x=>x.code==='missing_semantic_description'));
});
test('declared command is not a wired command',()=>{
 const b=baseline();b[3].metadata.status='declared';
 const v=quality(b);assert.equal(v.totals.verifiedWikiChains,0);assert(v.defects.some(x=>x.code==='unwired_or_unverified_command'));
});
test('source without commit provenance cannot pass',()=>{
 const b=baseline();delete b[0].metadata.source_sha;
 const v=quality(b);assert.equal(v.totals.verifiedWikiChains,0);assert(v.defects.some(x=>x.code==='missing_source_provenance'));
});
test('no false chain from bare code and command presence',()=>{
 const v=quality(baseline().slice(0,4));assert.equal(v.totals.potentialChains,0);
 assert(v.defects.some(x=>x.code==='no_code_semantic_command_chain'));
});
test('unverified edge and index-only wiki do not pass',()=>{
 const b=baseline();b[5].metadata={link_status:'declared'};
 const v=quality(b,pages('# Index only\n'));assert.equal(v.totals.verifiedWikiChains,0);
 assert(v.defects.some(x=>x.code==='unverified_relationship'));
 assert(v.defects.some(x=>x.code==='missing_cross_domain_wiki_page'));
});
test('broken Markdown links are reported without fetching external URLs',()=>{
 const p=pages();p.set('index.md','[Missing](./missing.md)\n[External](https://example.com/data.md)\n');
 const v=quality(baseline(),p);assert.equal(v.defects.filter(x=>x.code==='broken_wiki_link').length,1);
});
test('input and page ordering do not change content-derived quality verdict',()=>{
 const a=quality();const rev=baseline();const b=quality([...rev.slice(0,4).reverse(),...rev.slice(4).reverse()],new Map([...pages()].reverse()));
 assert.deepEqual(a.totals,b.totals);assert.deepEqual(a.defects,b.defects);
});
test('wiki reader refuses symlinks and ignores secret paths',()=>{
 const dir=mkdtempSync(join(tmpdir(),'wiki-oracle-'));
 try {writeFileSync(join(dir,'index.md'),'# Hello\n');writeFileSync(join(dir,'.env.md'),'secret');
   assert.deepEqual([...readWikiPages(dir).keys()],['index.md']);
   symlinkSync(join(dir,'index.md'),join(dir,'shortcut.md'));assert.throws(()=>readWikiPages(dir),/wiki_symlink_refused/);
 } finally {rmSync(dir,{recursive:true,force:true});}
});
test('CLI creates a unique JSON report, returns blocked exit code, refuses overwrite',()=>{
 const dir=mkdtempSync(join(tmpdir(),'wiki-oracle-cli-'));
 try {const input=join(dir,'exchange.jsonl'),wiki=join(dir,'wiki'),out=join(dir,'result.json');mkdirSync(wiki);writeFileSync(join(wiki,'integration.md'),wikiGood);writeFileSync(input,exchange());
   assert.equal(main(['--exchange',input,'--wiki-dir',wiki,'--report',out]),2);
   assert.equal(JSON.parse(readFileSync(out,'utf8')).verdict,'BLOCKED');
   assert.throws(()=>main(['--exchange',input,'--wiki-dir',wiki,'--report',out]),/report_exists_refuse_overwrite/);
 } finally {rmSync(dir,{recursive:true,force:true});}
});

test('quality has auditable five-dimensional denominators',()=>{
 const v=quality();assert.equal(v.quality.dimensions.length,5);
 assert.equal(v.quality.score,80);assert.equal(v.quality.dimensions[4].name,'linked_knowledge_records');
 assert.equal(v.quality.dimensions[4].numerator,0);
});
test('unknown knowledge status and orphaned record are explicit integration defects',()=>{
 const b=baseline();b[1].metadata.status='mystery';
 const v=quality(b);
 assert(v.defects.some(x=>x.code==='unknown_knowledge_status'));
 assert(v.defects.some(x=>x.code==='orphan_knowledge_record'));
});