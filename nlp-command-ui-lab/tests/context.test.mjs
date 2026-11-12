import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, mkdirSync, writeFileSync, rmSync, statSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { DatabaseSync } from 'node:sqlite';
import { deriveFocus, normalizeKnowledge, searchContext } from '../src/context.mjs';

test('focus identifies existing VIVIM repo names but does not grant command status',()=>{
 const r=deriveFocus('please explain the knowledge pipeline in repos HQ');
 assert.equal(r.scope,'repos-hq');assert.equal(r.scopeReason,'name_detected');
 assert.ok(r.query.includes('knowledge'));assert.equal(r.ready,true);
 assert.equal(deriveFocus('', 'auto').ready,false);
 assert.equal(deriveFocus('deploy the mobile UI','tierflow').scopeReason,'user_selected');
});

test('normalized knowledge is text-bounded, source-tagged and ignores unsupported kinds',()=>{
 const rows=normalizeKnowledge([{id:'a',kind:'wiki',title:'Wiki',snippet:'X'.repeat(900),project:'repos-hq'},{id:'b',kind:'secret',title:'not allowed'}]);
 assert.equal(rows.length,1);assert.equal(rows[0].snippet.length,320);assert.equal(rows[0].source,'hq.kb');
});

test('when actual HQ index is missing, the lens reports unavailable without fabricated evidence', async()=>{
 const v=await searchContext({text:'personal intelligence graph',hqRoot:'/this/not/a/repo'});
 assert.equal(v.status,'unavailable');assert.equal(v.items.length,0);assert.equal(v.persisted,false);assert.equal(v.readOnly,true);
});

test('uses existing HQ read APIs over a read-only SQLite index; personal evidence is opt-in',async()=>{
 const root=mkdtempSync(join(tmpdir(),'vivim-labs-context-'));
 try{
  mkdirSync(join(root,'hq/lib'),{recursive:true});mkdirSync(join(root,'.hq'));
  writeFileSync(join(root,'package.json'),'{"type":"module"}');
  writeFileSync(join(root,'hq/lib/registry.mjs'),"export const dbPath = root => root + '/.hq/index.db';");
  writeFileSync(join(root,'hq/lib/kb.mjs'),`export function kbQuery(db, {kind}) {
    const list=[{id:'wiki:1',kind:'wiki',title:'Shared context engine',snippet:'Graph and DCB',project:'repos-hq',score:7},
      {id:'note:secret',kind:'note',title:'Private note',snippet:'PRIVATE_TEST_DATA',project:'repos-hq',score:4}];
    return kind===null ? list : list.filter(x=>kind.includes(x.kind)); }`);
  writeFileSync(join(root,'hq/lib/search.mjs'),"export function searchContent(db){return {results:[{path:'hq/lib/context.mjs',project:'repos-hq',lang:'js',snippet:'Context query implementation',score:1,line:12}]};}");
  const db=new DatabaseSync(join(root,'.hq/index.db'));
  db.exec('CREATE TABLE projects (name TEXT, head TEXT, indexed_at TEXT)');
  db.prepare('INSERT INTO projects VALUES(?,?,?)').run('repos-hq','a'.repeat(40),'2026-11-11T12:00:00Z');db.close();
  const before=statSync(join(root,'.hq/index.db')).size;
  const a=await searchContext({text:'context graph',scope:'repos-hq',includePersonal:false,hqRoot:root});
  assert.equal(a.status,'results');assert.equal(a.source,'hq.local.sqlite');
  assert.equal(a.items.length,2);assert.equal(a.items.some(x=>x.kind==='note'),false);
  assert.equal(a.page.project.head,'a'.repeat(40));assert.equal(a.page.kind,'ephemeral_knowledge_page');
  const b=await searchContext({text:'context graph',scope:'repos-hq',includePersonal:true,hqRoot:root});
  assert.equal(b.items.some(x=>x.kind==='note'),true);
  assert.equal(b.items.find(x=>x.kind==='note').snippet,'PRIVATE_TEST_DATA');
  assert.equal(statSync(join(root,'.hq/index.db')).size,before);
 }finally{rmSync(root,{recursive:true,force:true});}
});