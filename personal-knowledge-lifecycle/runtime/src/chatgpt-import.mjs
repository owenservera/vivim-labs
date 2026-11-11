// Official ChatGPT export ZIP -> normalized Host evidence batches.
// Labs integration staging; all writes are delegated to an authoritative Host sink.
// Intents and checkpoints share Host's existing transactional ledger; uncertain
// results are inspected, NEVER blindly replayed.
import { createHash } from 'node:crypto';
import path from 'node:path';
import { openZipArchive, fingerprintFile } from './zip-reader.mjs';
import { parseJsonObjectArray } from './json-array-stream.mjs';
import { normalizeChatGptConversations, batchMetadata } from './chatgpt-normalize.mjs';
export const ARCHIVE_JOB_SCHEMA='vivim.chatgpt-archive-job/1';
const hash=v=>createHash('sha256').update(v).digest('hex');
const fail=(code, message)=>Object.assign(new Error(message??code),{code});
const valid=s=>typeof s==='string'&&s.length>0&&s.length<=256&&!/[\x00-\x1f]/.test(s);
const clone=s=>structuredClone(s);

/**
 * @param {object} options
 * store: existing Host { read(key), transact(key, updater) }
 * authorize: Host authority check returning {allowed:true} for this exact scope;
 *            invoked at start and before EACH commit/reconciliation.
 * sink: Host's durable evidence writer {inspectBatch(key),commitBatch(packet)}.
 *       inspectBatch MUST distinguish confirmed-absent from uncertain.
 * afterCommitted: optional idempotent metadata accounting callback.
 */
export async function importChatGptExport(options={}) {
  const {archivePath,worldId,accountId,store,authorize,sink,afterCommitted,
    batchSize=8,maxBatchBytes=32*1024*1024,maxObjects=1_000_000,
    maxObjectBytes=32*1024*1024,onProgress=()=>{},zipLimits={}}=options;
  if (!path.isAbsolute(archivePath??'')||!valid(worldId)||!valid(accountId))throw fail('IMPORT_IDENTITY_REQUIRED');
  if (!store?.read||!store?.transact||typeof authorize!=='function'||!sink?.inspectBatch||!sink?.commitBatch)throw fail('IMPORT_PORTS_REQUIRED');
  if (!Number.isSafeInteger(batchSize)||batchSize<1||batchSize>128||!Number.isSafeInteger(maxBatchBytes)||maxBatchBytes<1024)throw fail('IMPORT_BOUNDS');
  const permit=async (sourceId, archiveDigest) => {
    const grant=await authorize({worldId,accountId,sourceId,archiveDigest,operation:'import_archive'});
    if (!grant||grant.allowed!==true||!valid(grant.grantId))throw fail('IMPORT_GRANT_REQUIRED');
    return grant;
  };
  await permit('chatgpt-export:pending', 'pending');
  const zip=await openZipArchive(archivePath,zipLimits);
  try {
    const candidates=zip.entries.filter(e=>e.name==='conversations.json'||e.name.endsWith('/conversations.json'));
    if (candidates.length!==1)throw fail(candidates.length?'IMPORT_AMBIGUOUS_CONVERSATIONS':'IMPORT_CONVERSATIONS_MISSING');
    const entry=candidates[0];
    if(!entry.size)throw fail('IMPORT_EMPTY_CONVERSATIONS');
    const archiveDigest=await fingerprintFile(archivePath);
    const sourceId='chatgpt-export:'+hash(accountId+'\0'+archiveDigest);
    const key='chatgpt-import.'+hash(worldId+'\0'+accountId+'\0'+sourceId);
    await permit(sourceId,archiveDigest);
    const modify=fn=>store.transact(key,old=>fn(old?clone(old):null));
    const first=()=>({schema:ARCHIVE_JOB_SCHEMA,worldId,accountId,sourceId,archiveDigest,
      entryName:entry.name,cursor:-1,processed:0,batches:0,pending:null,status:'running'});
    modify(old=>{
      if(!old)return first();
      if(old.schema!==ARCHIVE_JOB_SCHEMA||old.worldId!==worldId||old.accountId!==accountId||old.archiveDigest!==archiveDigest||old.entryName!==entry.name)throw fail('IMPORT_CHECKPOINT_MISMATCH');
      return old;
    });
    let state=()=>store.read(key);
    if(state().status==='complete')return {...state(),duplicateRun:true};
    let buffer=[],firstIndex=-1,lastIndex=-1,bytes=0,index=-1;
    async function flush(){
      if(!buffer.length)return;
      const current=state();
      const ordered=buffer.slice().sort((a,b)=>{
        const av=Date.parse(a.updatedAt??a.createdAt??'')||0;
        const bv=Date.parse(b.updatedAt??b.createdAt??'')||0;
        return bv-av;
      });
      const records=batchMetadata(ordered);
      const digest=hash(JSON.stringify({sourceId,firstIndex,lastIndex,records}));
      const batchKey='chatzip-'+hash(sourceId+'\0'+firstIndex+'\0'+lastIndex+'\0'+digest);
      if(current.cursor>=lastIndex){buffer=[];bytes=0;return;}
      if(current.cursor!==firstIndex-1)throw fail('IMPORT_CURSOR_GAP');
      await permit(sourceId,archiveDigest);
      modify(s=>{
        if(s.pending){if(s.pending.key!==batchKey||s.pending.digest!==digest)throw fail('IMPORT_PENDING_CONFLICT');}
        else s.pending={key:batchKey,digest,first:firstIndex,last:lastIndex};
        s.status='pending';return s;
      });
      // Inspect durable Host journal before any potentially duplicated effect.
      let observed=await sink.inspectBatch(batchKey);
      if(!observed||!['absent','committed','uncertain'].includes(observed.status))throw fail('IMPORT_INSPECTION_REQUIRED');
      if(observed.status==='uncertain')throw fail('IMPORT_COMMIT_UNCERTAIN');
      let receipt=observed.receipt;
      if(observed.status==='absent'){
        await permit(sourceId,archiveDigest);
        receipt=await sink.commitBatch({key:batchKey,digest,sourceId,worldId,accountId,
          archiveDigest,range:{first:firstIndex,last:lastIndex},conversations:ordered});
      }
      if(!receipt||receipt.key!==batchKey||receipt.digest!==digest||receipt.verified!==true||receipt.status!=='committed')throw fail('IMPORT_RECEIPT_UNVERIFIED');
      await permit(sourceId,archiveDigest); // deny post-commit projection if revoked meanwhile
      if(typeof afterCommitted==='function')await afterCommitted({sourceId,batchId:batchKey,sourceRevision:archiveDigest,
        items:records,receipt,range:{first:firstIndex,last:lastIndex}});
      const result=modify(s=>{
        if(!s.pending||s.pending.key!==batchKey||s.pending.digest!==digest||s.cursor!==firstIndex-1)throw fail('IMPORT_PENDING_CONFLICT');
        s.cursor=lastIndex;s.processed+=records.length;s.batches++;s.pending=null;s.status='running';return s;
      });
      onProgress({processed:result.processed,batches:result.batches,cursor:result.cursor});
      buffer=[];bytes=0;firstIndex=-1;lastIndex=-1;
    }
    const stream=await zip.entryStream(entry);
    for await(const raw of parseJsonObjectArray(stream,{maxObjects,maxObjectBytes})){
      index++;
      if(index<=state().cursor)continue;
      const [normalized]=normalizeChatGptConversations([raw],{accountId,limit:1,maxTextBytes:maxObjectBytes});
      const estimate=Buffer.byteLength(JSON.stringify(normalized));
      if(estimate>maxBatchBytes)throw fail('IMPORT_CONVERSATION_LIMIT');
      if(buffer.length&&(buffer.length>=batchSize||bytes+estimate>maxBatchBytes))await flush();
      if(firstIndex<0)firstIndex=index;
      buffer.push(normalized);lastIndex=index;bytes+=estimate;
    }
    await flush();
    await permit(sourceId,archiveDigest);
    const completed=modify(s=>{if(s.pending)throw fail('IMPORT_PENDING_CONFLICT');if(s.cursor!==index)throw fail('IMPORT_CURSOR_GAP');s.status='complete';return s;});
    return {...completed,duplicateRun:false};
  } finally { await zip.close(); }
}
