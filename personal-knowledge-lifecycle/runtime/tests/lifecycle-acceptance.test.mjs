// Independent synthetic regressions. These assert the frozen production requirements,
// not the old staging behavior. Red assertions must not be converted to skips.
import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtemp, writeFile, rm} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import path from 'node:path';
import {normalizeChatGptConversations} from '../src/chatgpt-normalize.mjs';
import {createHostArchiveIngestion} from '../src/host-archive-wiring.mjs';
import {importChatGptExport} from '../src/chatgpt-import.mjs';
import {makeZip, conversation} from './fixtures.mjs';

const normalized = raw => normalizeChatGptConversations([raw], {accountId:'synthetic-account'})[0];
const digest = raw => normalized(raw).contentDigest;
for (const [label, change] of [
  ['title', c => { c.title += ' revised'; }],
  ['selected branch', c => { c.current_node = 'n1'; }],
  ['omitted media', c => { c.mapping.n2.message.content.parts.push({image_asset_pointer:'synthetic-media-id'}); }],
  ['timestamp', c => { c.update_time += 1; }],
  ['message content', c => { c.mapping.n2.message.content.parts[0] += ' revised'; }],
]) {
  test(`v2 revision changes for ${label}`, () => {
    const original = conversation(1), changed = structuredClone(original); change(changed);
    assert.notEqual(digest(original), digest(changed), `${label} must be included in the canonical content revision`);
  });
}
test('overlapping exports normalize mapping order without changing content revision', () => {
  const a=conversation(2), b=structuredClone(a);
  b.mapping=Object.fromEntries(Object.entries(b.mapping).reverse());
  assert.equal(digest(a),digest(b),'serialization order is not a content revision');
  assert.equal(normalized(a).nativeId, normalized(b).nativeId);
});

function ledgerFixture() {
  const data=new Map();
  return {meta:{get:k=>data.get(k)??null,set:(k,v)=>data.set(k,v)},transaction:fn=>fn()};
}
for (const field of ['worldId','accountId','store','authorize','sink','coordinator']) {
  test(`public archive wrapper rejects reserved ${field} override before touching archive`, async () => {
    let called=0;
    const host={authorizeSource:async()=>{called++;return {allowed:true,grantId:'synthetic-grant'};},
      inspectEvidenceBatch:async()=>({status:'absent'}), graphRevision:async()=>({epoch:0,sourceRevision:1}),
      commitEvidenceAndGraph:async()=>{throw Error('unexpected graph write');}};
    const adapter=createHostArchiveIngestion({ledger:ledgerFixture(),host,
      coordinator:{recordBatch(){throw Error('unexpected projection');}},worldId:'synthetic-world',accountId:'synthetic-account'});
    await assert.rejects(async()=>adapter.importArchive({archivePath:path.join(tmpdir(),'synthetic-does-not-exist.zip'),
      [field]:field==='worldId'||field==='accountId'?'hostile':{}}), error => {
      assert.match(error.code??'', /OVERRIDE|RESERVED|INVALID_INPUT|UNEXPECTED|INPUT_INVALID/,
        'must reject untrusted reserved input, not fail later on missing archive or invalid replacement port');
      return true;
    });
    assert.equal(called,0,'reserved input rejection precedes authorization and file access');
  });
}

test('late corrupt CRC never commits graph/evidence or projects accepted batches', async () => {
  const dir=await mkdtemp(path.join(tmpdir(),'vivim-accept-crc-'));
  try {
    // Stored ZIP entry crosses many stream chunks; corruption is detected at final CRC,
    // after enough JSON objects for multiple old-style batch flushes.
    const rows=Array.from({length:96},(_,i)=>{
      const c=conversation(i);c.mapping.n2.message.content.parts=[`synthetic-${i}-`+'z'.repeat(8192)];return c;
    });
    const archivePath=path.join(dir,'synthetic.zip');
    const zip=makeZip([['conversations.json',JSON.stringify(rows),0]]);
    // Flip matching local/central CRCs as unsigned values; the shared fixture's
    // corruptCrc option uses signed XOR and overflows for some synthetic payloads.
    const wrong=(zip.readUInt32LE(14)^1)>>>0;
    zip.writeUInt32LE(wrong,14);
    const central=zip.readUInt32LE(zip.length-22+16);
    zip.writeUInt32LE(wrong,central+16);
    await writeFile(archivePath,zip);
    const values=new Map();let commits=0,projections=0;
    const store={read:k=>structuredClone(values.get(k)??null),transact(k,fn){const v=fn(this.read(k));values.set(k,structuredClone(v));return v;}};
    await assert.rejects(importChatGptExport({archivePath,worldId:'synthetic-world',accountId:'synthetic-account',store,
      authorize:async()=>({allowed:true,grantId:'synthetic-grant'}),batchSize:1,
      sink:{inspectBatch:async()=>({status:'absent'}),commitBatch:async p=>{
        commits++;return {key:p.key,digest:p.digest,verified:true,status:'committed'};
      }},afterCommitted:async()=>{projections++;}}),error=>/CRC|CHECKSUM/.test(error.code??error.message));
    assert.equal(commits,0,'full stream integrity must precede irreversible graph/evidence admission');
    assert.equal(projections,0,'late integrity failure must expose no accepted content');
  } finally {await rm(dir,{recursive:true,force:true});}
});
