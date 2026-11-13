import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, existsSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';
const root=resolve(dirname(fileURLToPath(import.meta.url)),'..');
const json=p=>JSON.parse(readFileSync(resolve(root,p),'utf8'));
test('catalog has broad, explicitly unverified coverage',()=>{
  const c=json('catalog/providers.json');
  assert.equal(c.schema,'vivim.labs.provider-catalog/0.1');
  assert.equal(c.verifiedAdapters,0);
  assert.ok(c.providers.length>=30);
  assert.equal(new Set(c.providers.map(p=>p.id)).size,c.providers.length);
  for(const p of c.providers){
    assert.equal(p.status,'CANDIDATE_NOT_WIRED');
    assert.ok(p.label&&p.family&&p.priority);
    assert.ok(Array.isArray(p.streams)&&p.streams.length);
    assert.ok(Array.isArray(p.modes)&&p.modes.length);
    assert.ok(!p.modes.some(m=>/api/i.test(m)),'no API transport candidates');
    assert.ok(p.sourceEvidence.startsWith('UNVERIFIED'));
  }
  for(const id of ['facebook','messenger','instagram','whatsapp','x','youtube','telegram','signal','discord','gmail','outlook']){
    assert.ok(c.providers.some(p=>p.id===id),id);
  }
});
test('Messenger has distinct read and Host-proposal messaging streams',()=>{
  const p=json('catalog/providers.json').providers.find(p=>p.id==='messenger');
  assert.deepEqual(p.webSurface.entry,'https://www.facebook.com/messages');
  assert.equal(p.interaction,'RESEARCH_ONLY_NO_SEND');
  assert.ok(p.streams.includes('threads')&&p.streams.includes('messages'));
  assert.ok(p.ui.includes('send-receipt')&&p.ui.includes('draft-composer'));
});
test('schemas encode proposal and provenance rather than issuing authority',()=>{
  const stream=json('contracts/stream-envelope.v0.schema.json');
  const outbound=json('contracts/host-action-request.v0.schema.json');
  assert.equal(stream.properties.schema.const,'vivim.labs.stream-envelope/0.1');
  assert.equal(outbound.properties.status.const,'PROPOSAL_ONLY');
  assert.equal(outbound.properties.approvalPolicy.properties.explicitConfirmation.const,true);
  assert.equal(outbound.properties.approvalPolicy.properties.requiresPreSendRecheck.const,true);
  assert.equal(stream.additionalProperties,false);
  assert.equal(outbound.additionalProperties,false);
});
test('UI is synthetic only; sending stays unavailable',()=>{
  for(const p of ['ui/index.html','ui/style.css','ui/app.mjs','ui/mock-facebook-thread.json'])assert.ok(existsSync(resolve(root,p)),p);
  const html=readFileSync(resolve(root,'ui/index.html'),'utf8');
  const js=readFileSync(resolve(root,'ui/app.mjs'),'utf8');
  assert.match(html,/Send unavailable/);
  assert.match(html,/disabled aria-disabled="true"/);
  assert.doesNotMatch(js,/\b(method\s*:\s*['"]POST['"]|fetch\(['"]https?:)/);
  assert.equal(json('ui/mock-facebook-thread.json').classification,'FABRICATED_NO_REAL_USER_DATA');
});
