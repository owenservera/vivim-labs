// R006: pure bridge to the incumbent Studio read-only projection.
// Inject ARBOR's actual projectCraftIntelligence; never open browser or register actions.
import {compileProposal} from '../R005/compiler.mjs';
const fail=code=>{throw Object.assign(new Error(code),{code});};
const inert=()=>({executionAuthorized:false,activationPermitted:false,sendPermission:false});
const authority=a=>Object.keys(inert()).every(k=>a?.[k]===false);
const same=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
const fields=Object.freeze({
 cursor:{type:'string',minLength:1,maxLength:2048},
 query:{type:'string',minLength:1,maxLength:4096},
 conversationRef:{type:'string',minLength:1,maxLength:256},
 contentRef:{type:'string',minLength:1,maxLength:256},
 postRef:{type:'string',minLength:1,maxLength:256}
});
function inputSchema(op){
 const properties=Object.fromEntries(op.inputFields.map(name=>{
  if(!Object.hasOwn(fields,name))fail('UNDEFINED_INPUT_FIELD');
  return [name,{...fields[name]}];
 }));
 return {type:'object',additionalProperties:false,properties,required:op.inputFields.filter(x=>x!=='cursor')};
}
export function prepareStudioHandoff({atlas,review=null,reconciliation=null,projectCraftIntelligence}={}){
 if(typeof projectCraftIntelligence!=='function')fail('STUDIO_PROJECTOR_REQUIRED');
 const proposal=compileProposal(atlas,review);
 const p=projectCraftIntelligence(atlas,reconciliation);
 if(p?.schema!=='vivim.studio-craft-intelligence/1'||p.status!=='external-evidence-readonly'||
    p.target?.siteId!==proposal.target.siteId||p.target?.origin!==proposal.target.origin||
    p.source?.bundleDigest!==proposal.source.bundleDigest||p.source?.verificationAuthority!=='unattested-external-report'||
    !authority(p.authority)||!Array.isArray(p.evidence)||p.evidence.length!==atlas.evidence.length||
    p.counts?.controls!==atlas.controls.length||p.counts?.observations!==atlas.observations.length||
    p.counts?.evidence!==atlas.evidence.length||p.counts?.unknowns!==atlas.unknownIds.length)fail('STUDIO_PROJECTION_MISMATCH');
 const evidence=p.evidence.map(e=>e.evidenceId).sort();
 if(!same(evidence,atlas.evidence.map(e=>e.evidenceId).sort()))fail('STUDIO_EVIDENCE_MISMATCH');
 // We do not forward the external projection itself, labels, selectors, or raw artifacts.
 return {schema:'vivim.craft-studio-handoff/1',target:proposal.target,
  source:{proposalDigest:proposal.proposalDigest,atlasDigest:proposal.source.atlasDigest,bundleDigest:proposal.source.bundleDigest,arborPin:proposal.source.arborPin,
   projectionSchema:p.schema,verificationAuthority:'unattested-external-report'},
  coverage:proposal.coverage,counts:proposal.counts,
  candidates:proposal.operations.map(op=>({proposalKey:op.proposalKey,effect:op.effect,observationStatus:op.observationStatus,evidenceRefs:op.evidenceRefs,
   proposedInputSchema:inputSchema(op),proposedOutputShape:op.outputShape,
   canonicalSemanticId:null,hostActionId:null,providerBinding:null,accountScope:null,liveVerified:false,publicInvocation:false,authority:inert()})),
  authority:inert(),status:'awaiting-canonical-binding-and-independent-verification'};
}
