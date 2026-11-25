// Labs-only candidate projection; no execution, grant or authoritative registry.
export function projectCandidates(manifest, bindings={}) {
 if(manifest?.schema!=="vivim.labs.windows-extraction-pack/1"||!Array.isArray(manifest.descriptors))throw Error("INVALID_MANIFEST");
 const ids=new Set(),names=new Set();
 return manifest.descriptors.map(d=>{
  if(!/^windows\.[a-z][a-z0-9.]+$/.test(d.id)||d.semanticId!==d.id+"@1"||!/^vivim_windows_[a-z0-9_]+$/.test(d.mcp)||ids.has(d.id)||names.has(d.mcp)||!["R0","R2"].includes(d.risk)||d.readiness!=="proposed"||d.requiresLiveGrant!==true||!["mcp","text","desktop","mobile","visual"].every(s=>d.surfaces.includes(s))||!d.args||typeof d.args!=="object"||d.result!=="vivim.windows.native-result/1")throw Error("INVALID_DESCRIPTOR");
  ids.add(d.id);names.add(d.mcp);
  const b=Object.hasOwn(bindings,d.id)?bindings[d.id]:null;
  // Evidence statuses are declarations from a trusted integration harness, not grants.
  const wired=b?.hostActionId===d.id&&b?.dispatcher===true&&b?.nativeAdapter===true&&b?.schemaMatch===true&&b?.mcpTool===d.mcp&&b?.gpuiActionId===d.id;
  const candidateReady=wired&&b?.sourceEvidence===true;
  return {id:d.id,semanticId:d.semanticId,risk:d.risk,args:d.args,result:d.result,mcp:{name:d.mcp,hostActionId:d.id},app:{hostActionId:d.id,surfaces:d.surfaces.filter(s=>s!=="mcp")},coverage:candidateReady?"source_wiring_evidence_declared":"proposed",requiresLiveGrant:true,executionAuthorized:false};
 });
}
export function coverageSummary(rows){
 return {candidateCount:rows.length,mcpMapped:rows.filter(r=>r.mcp?.hostActionId===r.id).length,appMapped:rows.filter(r=>r.app?.hostActionId===r.id).length,sourceWiringEvidenceDeclared:rows.filter(r=>r.coverage==="source_wiring_evidence_declared").length,liveAccepted:0,executionAuthorized:false};
}
