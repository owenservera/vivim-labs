const RELEVANT=/(semantic-language-kernel|intent-front-end|deterministic-executor|SHARED-KB|realtime-stateful-ui-blueprint|wiki|graph|sqlite|DATA-003|index|command)/i;
export function recoverTargets(inventory){
 if(inventory?.schema!=='vivim-labs.documentation-consolidation/1'||!Array.isArray(inventory.documents))throw Error('INVENTORY_SCHEMA');
 const docs=inventory.documents;
 if(docs.length!==inventory.summary?.total)throw Error('INVENTORY_COUNT');
 const moved=docs.filter(d=>d.status==='moved').length,retained=docs.filter(d=>d.status==='retained').length;
 if(moved!==inventory.summary.moved||retained!==inventory.summary.retained)throw Error('INVENTORY_MOVES');
 return {total:docs.length,moved,retained,targets:docs.filter(d=>RELEVANT.test(d.path)).map(d=>({id:`${d.repository}:${d.path}`,source:d.path,repository:d.repository,location:d.location,blobSha:d.blobSha,
   disposition:d.status,authority:'HISTORICAL_OR_CURRENT_UNRESOLVED',implementation:'NOT_VERIFIED',question:'Verify target authority, supersession and runtime evidence'}))};
}
