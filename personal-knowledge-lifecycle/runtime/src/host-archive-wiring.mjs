// Dependency-injected bridge to the EXISTING Host ledger + authority + evidence sink.
// It deliberately does not invent a Host graph writer or Studio browser controller.
import {createHostLedgerStore} from './host-ledger-port.mjs';
import {buildPersonalGraphProposal, buildHqkgPersonalPacket} from './scoped-graph-proposal.mjs';
import {importChatGptExport} from './chatgpt-import.mjs';
import {createHash} from 'node:crypto';
const allowed=new Set(['archivePath','batchSize','maxBatchBytes','maxObjects','maxObjectBytes','zipLimits','onProgress','signal']);
const fail=code=>Object.assign(new Error(code),{code});
// Trusted storage contract (storage lane): forPrincipal port exposing authorize() ->
// opaque capability ID and stage({opId,capabilityId,objects}) with payload digest
// sha256(JSON.stringify(objects)); staging stays invisible until Host publication.
export function createHostArchiveIngestion({ledger,host,coordinator,worldId,accountId,storage=null}={}){
  if(!ledger||!host?.authorizeSource||!host?.inspectEvidenceBatch||!host?.commitEvidenceAndGraph||!host?.graphRevision||!coordinator?.recordBatch)throw new TypeError('Host authority, ledger, Rust graph bridge and coordinator required');
  if(storage!==null&&(typeof storage.authorize!=='function'||typeof storage.stage!=='function'))throw new TypeError('Trusted personal knowledge storage port must expose authorize and stage');
  const scopeId=createHash('sha256').update(JSON.stringify([worldId,accountId,'chatgpt'])).digest('hex');
  const owner='pk-'+scopeId;
  const store=createHostLedgerStore({ledger});
  const authorize=packet=>host.authorizeSource(packet);
  const sha=value=>createHash('sha256').update(value).digest('hex');
  // Durable V2 path: activates only when the trusted storage port exposes the full
  // finalize/commitGraph/reconcileGraph/inspect protocol (partial ports such as
  // {authorize,stage} keep the legacy evidence path). The storage job opId is the
  // 64-hex CLI opId (commitGraph passes it to rust.apply verbatim), derived
  // deterministically from the batch key so replays and restarts stay replay-stable.
  const durable=storage!==null&&['finalize','commitGraph','reconcileGraph','inspect','compact']
    .every(k=>typeof storage[k]==='function');
  const cliOpIdOf=batchKey=>sha(scopeId+'\0'+batchKey);
  const publishedReceipt=(batchKey,batchDigest)=>({status:'committed',receipt:{key:batchKey,digest:batchDigest,verified:true,status:'committed'}});
  // Crash/response-loss states reconcileGraph resolves with claimant ownership.
  const RECONCILABLE=new Set(['uncertain','admitted','graph_committing','graph_committed']);
  const durableSink={
    async inspectBatch(batchKey,batchDigest){
      const opId=cliOpIdOf(batchKey);
      const job=storage.inspect(opId);
      if(!job||!job.capabilityId)return {status:'absent'};
      if(job.state==='published')return publishedReceipt(batchKey,batchDigest);
      if(RECONCILABLE.has(job.state)){
        // Throws on lock/torn/orphan artifacts and CLAIM_FOREIGN; unresolved jobs
        // stay honestly uncertain and marker-absent returns to re-admission.
        const resolved=await storage.reconcileGraph({opId});
        if(resolved.state==='published')return publishedReceipt(batchKey,batchDigest);
        if(resolved.state==='integrity_verified')return {status:'absent'};
        if(resolved.state==='revoked')throw fail('IMPORT_PUBLICATION_REVOKED');
        throw fail('IMPORT_COMMIT_UNCERTAIN');
      }
      if(job.state==='revoked')throw fail('IMPORT_PUBLICATION_REVOKED');
      if(job.state==='erasing'||job.state==='erased')throw fail('IMPORT_DELETION_FENCED');
      return {status:'absent'}; // staging/integrity_verified: admission may proceed.
    },
    async commitBatch(packet){
      const opId=cliOpIdOf(packet.key);
      const job=storage.inspect(opId);
      const capabilityId=job?.capabilityId;
      if(!capabilityId)throw fail('IMPORT_STAGING_MISSING');
      const finalized=await storage.finalize({opId,capabilityId});
      if(finalized.state==='published')return {key:packet.key,digest:packet.digest,verified:true,status:'committed'};
      if(finalized.state!=='integrity_verified')throw fail('IMPORT_COMMIT_UNCERTAIN');
      const graphPacket=buildHqkgPersonalPacket({scope:scopeId,opId,
        payloadDigest:sha(JSON.stringify(packet.conversations)),conversations:packet.conversations});
      const proposal={nodes:graphPacket.nodes,edges:graphPacket.edges};
      let committed;
      try{
        committed=await storage.commitGraph({opId,proposal});
      }catch(error){
        // Frozen cadence (8 committed snapshots or >48 MiB projection): compact the
        // healthy journal through the durable-intent port, then retry exactly once.
        if(!(error&&(error.code==='COMPACTION_REQUIRED'||/COMPACTION_REQUIRED/.test(error.message??''))))throw error;
        await storage.compact({reason:'ingestion-cadence'});
        committed=await storage.commitGraph({opId,proposal});
      }
      if(committed.state==='published')return {key:packet.key,digest:packet.digest,verified:true,status:'committed'};
      if(committed.state==='revoked')throw fail('IMPORT_PUBLICATION_REVOKED');
      throw fail('IMPORT_COMMIT_UNCERTAIN');
    }
  };
  // Staging adapter: capability authorization and invisible staging through the
  // trusted storage port. The Host grant must carry the exact fields the storage
  // capability contract requires; missing fields fail closed, never defaulted.
  const staging=!storage?null:async({opId,objects,payloadDigest,grant,sourceId,archiveDigest})=>{
    if(!grant||grant.allowed!==true||typeof grant.grantId!=='string'||!grant.grantId)throw fail('STAGING_GRANT_REQUIRED');
    if(grant.destinationDigest===undefined||grant.grantRevision===undefined||grant.expiry===undefined)throw fail('STAGING_GRANT_INCOMPLETE');
    const capabilityId=await storage.authorize({operation:'import',payloadDigest,
      destinationDigest:grant.destinationDigest,grantId:grant.grantId,grantRevision:grant.grantRevision,
      sources:[{id:sourceId,revision:archiveDigest}],expiry:grant.expiry});
    if(typeof capabilityId!=='string'||!capabilityId)throw fail('STAGING_CAPABILITY_UNISSUED');
    // Same 64-hex identity space as the durable V2 commit path: the storage job
    // opId is the CLI opId, so finalize/commitGraph address the same job.
    return await storage.stage({opId:cliOpIdOf(opId),capabilityId,objects});
  };
  const sink=durable?durableSink:{
    inspectBatch:key=>host.inspectEvidenceBatch({key,worldId,accountId,provider:'chatgpt',scopeId}),
    inspectBatch:key=>host.inspectEvidenceBatch({key,worldId,accountId,provider:'chatgpt',scopeId}),
    async commitBatch(packet){
      const graph=await host.graphRevision({worldId,accountId,provider:'chatgpt',scopeId});
      if(!graph||!Number.isSafeInteger(graph.epoch)||graph.epoch<0||!Number.isSafeInteger(graph.sourceRevision)||graph.sourceRevision<1)throw Object.assign(new Error('Unverified Rust graph revision'),{code:'GRAPH_REVISION_UNVERIFIED'});
      const proposal=buildPersonalGraphProposal({owner,sourceId:packet.sourceId,
        provenanceRevision:graph.sourceRevision,provenanceDigest:packet.archiveDigest,
        expectedGraphEpoch:graph.epoch,conversations:packet.conversations,batchKey:packet.key});
      // Exact CLI packet (vivim.hqkg-personal/1): local ids, string properties, no
      // reserved keys. The Host MUST fill expectedBytes (and may re-recheck epoch)
      // from its trusted rust.inspect under the scope claim immediately before
      // apply; the CLI refuses stale epoch/bytes, so stale packets fail closed.
      const hqkgPacket=buildHqkgPersonalPacket({scope:scopeId,
        opId:createHash('sha256').update(scopeId+'\0'+packet.key).digest('hex'),
        payloadDigest:packet.digest,expectedEpoch:graph.epoch,
        ...(Number.isSafeInteger(graph.bytes)?{expectedBytes:graph.bytes}:{}),
        conversations:packet.conversations});
      // The production Host adapter MUST authenticate source provenance, validate
      // the Rust graph proposal and commit evidence + graph with a single journal.
      return host.commitEvidenceAndGraph({...packet,provider:'chatgpt',scopeId,graphProposal:proposal,hqkgPacket});
    }
  };
  return Object.freeze({
    importArchive:args=>{
      if(!args||typeof args!=='object'||Array.isArray(args)||Object.keys(args).some(k=>!allowed.has(k)))throw fail('IMPORT_RESERVED_OVERRIDE');
      return importChatGptExport({...args,worldId,accountId,provider:'chatgpt',scopeId,store,authorize,sink,staging,
      afterCommitted:async({sourceId,batchId,sourceRevision,items,receipt})=>{
        if(!receipt?.verified||receipt?.status!=='committed')throw new Error('Host evidence receipt required');
        coordinator.recordBatch({lane:'archive',scopeId,provider:'chatgpt',sourceId,batchId,sourceRevision,items,acceptedByHost:true});
      }});
    }
  });
}
