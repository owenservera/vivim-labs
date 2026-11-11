// Official ChatGPT export ZIP -> normalized Host evidence batches.
// Labs integration staging; all writes are delegated to an authoritative Host sink.
// Intents and checkpoints share Host's existing transactional ledger; uncertain
// results are inspected, NEVER blindly replayed.
import { createHash } from 'node:crypto';
import path from 'node:path';
import { openZipArchive } from './zip-reader.mjs';
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
 * sink: Host's durable evidence writer {inspectBatch(key,digest),commitBatch(packet)}.
 *       inspectBatch MUST distinguish confirmed-absent from uncertain.
 * staging: optional trusted Host storage adapter invoked after full stream CRC/EOF
 *          verification with {opId,objects,payloadDigest,grant,sourceId,archiveDigest,
 *          scopeId,worldId,accountId,provider}; failures abort before any admission.
 * afterCommitted: optional idempotent metadata accounting callback.
 */
export async function importChatGptExport(options={}) {
  const {archivePath,worldId,accountId,store,authorize,sink,staging=null,afterCommitted,
    batchSize=8,maxBatchBytes=1024*1024,maxObjects=10000,
    maxObjectBytes=512*1024,onProgress=()=>{},zipLimits={},signal=null,provider='chatgpt',scopeId=null}=options;
  if (!path.isAbsolute(archivePath??'')||!valid(worldId)||!valid(accountId)||provider!=='chatgpt')throw fail('IMPORT_IDENTITY_REQUIRED');
  const expectedScope=hash(JSON.stringify([worldId,accountId,provider]));
  if(scopeId!==null&&scopeId!==expectedScope)throw fail('IMPORT_SCOPE_MISMATCH');
  if (!store?.read||!store?.transact||typeof authorize!=='function'||!sink?.inspectBatch||!sink?.commitBatch)throw fail('IMPORT_PORTS_REQUIRED');
  if(staging!==null&&typeof staging!=='function')throw fail('IMPORT_STAGING_PORT_INVALID');
  if (!Number.isSafeInteger(batchSize)||batchSize<1||batchSize>128||!Number.isSafeInteger(maxBatchBytes)||maxBatchBytes<1024||maxBatchBytes>1024*1024||
    !Number.isSafeInteger(maxObjects)||maxObjects<1||maxObjects>10000||!Number.isSafeInteger(maxObjectBytes)||maxObjectBytes<128||maxObjectBytes>512*1024)throw fail('POLICY_LIMIT');
  const permit=async (sourceId, archiveDigest) => {
    const grant=await authorize({worldId,accountId,provider,scopeId:expectedScope,sourceId,archiveDigest,operation:'import_archive'});
    if (!grant||grant.allowed!==true||!valid(grant.grantId))throw fail('IMPORT_GRANT_REQUIRED');
    return grant;
  };
  const checkPause=()=>{if(signal?.aborted)throw fail('IMPORT_PAUSED');};
  checkPause();
  await permit('chatgpt-export:pending', 'pending');
  const caps={maxEntries:10000,maxDirectoryBytes:4*1024*1024,maxEntryBytes:32*1024*1024,
    maxTotalBytes:64*1024*1024,maxExpansionRatio:100};
  if(!zipLimits||typeof zipLimits!=='object'||Array.isArray(zipLimits)||Object.entries(zipLimits).some(([k,v])=>
    !(k in caps)||!Number.isSafeInteger(v)||v<1||v>caps[k]))throw fail('POLICY_LIMIT');
  const zip=await openZipArchive(archivePath,Object.assign(caps,zipLimits));
  try {
    if(zip.size>64*1024*1024)throw fail('POLICY_LIMIT');
    const candidates=zip.entries.filter(e=>e.name==='conversations.json'||e.name.endsWith('/conversations.json'));
    if (candidates.length!==1)throw fail(candidates.length?'IMPORT_AMBIGUOUS_CONVERSATIONS':'IMPORT_CONVERSATIONS_MISSING');
    const entry=candidates[0];
    if(!entry.size)throw fail('IMPORT_EMPTY_CONVERSATIONS');
    const archiveDigest=await zip.fingerprint({signal});
    const sourceId='chatgpt-export:'+hash(expectedScope+'\0'+archiveDigest);
    const key='chatgpt-import.'+hash(expectedScope+'\0'+sourceId);
    await permit(sourceId,archiveDigest);
    // Best-effort durable checkpoint: the Host ledger stays authoritative; the
    // in-memory mirror only fills gaps when the injected store cannot round-trip
    // (e.g. no-op test ledgers). It never replaces durable restart recovery.
    let memory=null;
    const modify=fn=>{
      const next=store.transact(key,old=>fn(old?clone(old):(memory?clone(memory):null)));
      memory=next?clone(next):memory;
      return next;
    };
    const first=()=>({schema:ARCHIVE_JOB_SCHEMA,worldId,accountId,provider,scopeId:expectedScope,sourceId,archiveDigest,
      entryName:entry.name,cursor:-1,processed:0,batches:0,pending:null,status:'running'});
    modify(old=>{
      if(!old)return first();
      if(old.schema!==ARCHIVE_JOB_SCHEMA||old.worldId!==worldId||old.accountId!==accountId||old.scopeId!==expectedScope||old.provider!==provider||old.archiveDigest!==archiveDigest||old.entryName!==entry.name)throw fail('IMPORT_CHECKPOINT_MISMATCH');
      return old;
    });
    let state=()=>store.read(key)??(memory?clone(memory):null);
    if(state().status==='complete'){
      checkPause();
      if(await zip.fingerprint({signal})!==archiveDigest)throw fail('ZIP_FILE_CHANGED');
      return {...state(),duplicateRun:true};
    }
    // Keep all selected data private until the ZIP verifier reaches EOF (including CRC).
    // Never expose early batches if the last stream chunk fails integrity.
    const staged=[];let stagedBytes=0;
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
      const digest=hash(JSON.stringify({scopeId:expectedScope,sourceId,firstIndex,lastIndex,records}));
      const batchKey='chatzip-'+hash(expectedScope+'\0'+sourceId+'\0'+firstIndex+'\0'+lastIndex+'\0'+digest);
      if(current.cursor>=lastIndex){buffer=[];bytes=0;firstIndex=-1;lastIndex=-1;return;}
      if(current.cursor!==firstIndex-1)throw fail('IMPORT_CURSOR_GAP');
      checkPause();
      const grant=await permit(sourceId,archiveDigest);
      if(staging){
        // Durable invisible staging through the trusted Host storage port precedes
        // any admission; full stream CRC/EOF plus fingerprint were already verified.
        await staging({opId:batchKey,objects:ordered,payloadDigest:hash(JSON.stringify(ordered)),
          grant,sourceId,archiveDigest,scopeId:expectedScope,worldId,accountId,provider});
      }
      modify(s=>{
        if(s.pending){if(s.pending.key!==batchKey||s.pending.digest!==digest)throw fail('IMPORT_PENDING_CONFLICT');}
        else s.pending={key:batchKey,digest,first:firstIndex,last:lastIndex};
        s.status='pending';return s;
      });
      // Inspect durable Host journal before any potentially duplicated effect.
      // The batch digest rides along so a published-job receipt can be rebuilt
      // on resume without a second content-independent identity store.
      let observed=await sink.inspectBatch(batchKey,digest);
      if(!observed||!['absent','committed','uncertain'].includes(observed.status))throw fail('IMPORT_INSPECTION_REQUIRED');
      if(observed.status==='uncertain')throw fail('IMPORT_COMMIT_UNCERTAIN');
      let receipt=observed.receipt;
      if(observed.status==='absent'){
        checkPause();
        await permit(sourceId,archiveDigest);
        receipt=await sink.commitBatch({key:batchKey,digest,sourceId,worldId,accountId,provider,scopeId:expectedScope,
          archiveDigest,range:{first:firstIndex,last:lastIndex},conversations:ordered});
      }
      if(!receipt||receipt.key!==batchKey||receipt.digest!==digest||receipt.verified!==true||receipt.status!=='committed')throw fail('IMPORT_RECEIPT_UNVERIFIED');
      await permit(sourceId,archiveDigest); // deny post-commit projection if revoked meanwhile
      if(typeof afterCommitted==='function')await afterCommitted({sourceId,batchId:batchKey,sourceRevision:archiveDigest,
        scopeId:expectedScope,provider,items:records,receipt,range:{first:firstIndex,last:lastIndex}});
      const result=modify(s=>{
        if(!s.pending||s.pending.key!==batchKey||s.pending.digest!==digest||s.cursor!==firstIndex-1)throw fail('IMPORT_PENDING_CONFLICT');
        s.cursor=lastIndex;s.processed+=records.length;s.batches++;s.pending=null;s.status='running';return s;
      });
      onProgress({processed:result.processed,batches:result.batches,cursor:result.cursor});
      buffer=[];bytes=0;firstIndex=-1;lastIndex=-1;
    }
    const stream=await zip.entryStream(entry,{signal});
    for await(const raw of parseJsonObjectArray(stream,{maxObjects,maxObjectBytes})){
      checkPause();
      index++;
      if(index<=state().cursor)continue;
      const [normalized]=normalizeChatGptConversations([raw],{accountId,limit:1,maxTextBytes:maxObjectBytes});
      const estimate=Buffer.byteLength(JSON.stringify(normalized));
      if(estimate>maxBatchBytes)throw fail('IMPORT_CONVERSATION_LIMIT');
      stagedBytes+=estimate;
      if(stagedBytes>8*1024*1024)throw fail('POLICY_LIMIT');
      staged.push({index,normalized,estimate});
    }
    checkPause();
    if(await zip.fingerprint()!==archiveDigest)throw fail('ZIP_FILE_CHANGED');
    for(const item of staged){
      checkPause();
      if(buffer.length&&(buffer.length>=batchSize||bytes+item.estimate>maxBatchBytes))await flush();
      if(firstIndex<0)firstIndex=item.index;
      buffer.push(item.normalized);lastIndex=item.index;bytes+=item.estimate;
    }
    await flush();
    await permit(sourceId,archiveDigest);
    const completed=modify(s=>{if(s.pending)throw fail('IMPORT_PENDING_CONFLICT');if(s.cursor!==index)throw fail('IMPORT_CURSOR_GAP');s.status='complete';return s;});
    return {...completed,duplicateRun:false};
  } finally { await zip.close(); }
}
