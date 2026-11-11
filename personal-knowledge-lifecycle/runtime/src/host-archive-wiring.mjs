// Dependency-injected bridge to the EXISTING Host ledger + authority + evidence sink.
// It deliberately does not invent a Host graph writer or Studio browser controller.
import {createHostLedgerStore} from './host-ledger-port.mjs';
import {buildPersonalGraphProposal} from './scoped-graph-proposal.mjs';
import {importChatGptExport} from './chatgpt-import.mjs';
export function createHostArchiveIngestion({ledger,host,coordinator,worldId,accountId}={}){
  if(!ledger||!host?.authorizeSource||!host?.inspectEvidenceBatch||!host?.commitEvidenceAndGraph||!host?.graphRevision||!coordinator?.recordBatch)throw new TypeError('Host authority, ledger, Rust graph bridge and coordinator required');
  const store=createHostLedgerStore({ledger});
  const authorize=packet=>host.authorizeSource(packet);
  const sink={
    inspectBatch:key=>host.inspectEvidenceBatch({key,worldId,accountId}),
    async commitBatch(packet){
      const graph=await host.graphRevision({worldId,accountId});
      if(!graph||!Number.isSafeInteger(graph.epoch)||graph.epoch<0||!Number.isSafeInteger(graph.sourceRevision)||graph.sourceRevision<1)throw Object.assign(new Error('Unverified Rust graph revision'),{code:'GRAPH_REVISION_UNVERIFIED'});
      const proposal=buildPersonalGraphProposal({owner:worldId,sourceId:packet.sourceId,
        provenanceRevision:graph.sourceRevision,provenanceDigest:packet.archiveDigest,
        expectedGraphEpoch:graph.epoch,conversations:packet.conversations,batchKey:packet.key});
      // The production Host adapter MUST authenticate source provenance, validate
      // the Rust graph proposal and commit evidence + graph with a single journal.
      return host.commitEvidenceAndGraph({...packet,graphProposal:proposal});
    }
  };
  return Object.freeze({
    importArchive:args=>importChatGptExport({worldId,accountId,store,authorize,sink,...args,
      afterCommitted:async({sourceId,batchId,sourceRevision,items,receipt})=>{
        if(!receipt?.verified||receipt?.status!=='committed')throw new Error('Host evidence receipt required');
        coordinator.recordBatch({lane:'archive',sourceId,batchId,sourceRevision,items,acceptedByHost:true});
      }})
  });
}
