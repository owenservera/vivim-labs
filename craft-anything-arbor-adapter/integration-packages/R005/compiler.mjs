import {createHash} from 'node:crypto';
import {TARGETS,OPERATIONS,ARBOR_PIN} from './targets.mjs';
export const SCHEMA='vivim.craft-provider-integration-proposal/1';
export const REVIEW_SCHEMA='vivim.craft-surface-review/1';
const sha=/^[a-f0-9]{64}$/;
const ev=/^ev_[a-f0-9]{64}$/;
const role=/^[a-z][a-z0-9-]{0,31}$/;
const obj=x=>x!==null&&typeof x==='object'&&!Array.isArray(x);
const inert=()=>({executionAuthorized:false,activationPermitted:false,sendPermission:false});
const fail=c=>{throw Object.assign(new Error(c),{code:c});};
const sorted=(a,b)=>a<b?-1:a>b?1:0;
const canonical=x=>Array.isArray(x)?'['+x.map(canonical).join(',')+']':obj(x)?'{'+Object.keys(x).sort().map(k=>JSON.stringify(k)+':'+canonical(x[k])).join(',')+'}':JSON.stringify(x);
const digest=x=>createHash('sha256').update(canonical(x)).digest('hex');
const key=(id,p)=>id+':'+p;
const bounded=(x,n)=>Array.isArray(x)&&x.length<=n;
const only=(x,keys)=>obj(x)&&Object.keys(x).every(k=>keys.includes(k));
const qualifiers=new Set(['observed','partial','restricted','not-applicable']);
function inspectAtlas(atlas){
 const t=TARGETS[atlas?.target?.siteId];
 if(!t||atlas.target.origin!==t.origin||atlas.schema!=='vivim.application-surface-atlas/0.1'||atlas.source?.producer!=='craft-anything'||atlas.source?.version!==1||!sha.test(atlas.source?.bundleDigest??'')||!bounded(atlas.evidence,2500)||!bounded(atlas.controls,5000)||!bounded(atlas.observations,10000)||!bounded(atlas.unknownIds,2500)||Object.keys(inert()).some(k=>atlas.authority?.[k]!==false))fail('ATLAS_SCOPE_OR_CONTRACT');
 const evidence=new Set(),controls=new Map(), observations=new Set();
 for(const e of atlas.evidence){if(!ev.test(e?.evidenceId??'')||evidence.has(e.evidenceId))fail('EVIDENCE_INVALID');evidence.add(e.evidenceId);}
 for(const c of atlas.controls){const k=key(c?.sourceEvidenceId,c?.position);if(!evidence.has(c?.sourceEvidenceId)||!Number.isSafeInteger(c.position)||c.position<0||!role.test(c.role??'')||c.effectAuthorized!==false||c.locatorVerified!==false||controls.has(k))fail('CONTROL_INVALID');controls.set(k,c.role);}
 for(const o of atlas.observations){if(!evidence.has(o?.evidenceId)||o.effectAuthorized!==false)fail('OBSERVATION_INVALID');observations.add(o.evidenceId);}
 return {t,evidence,controls,observations};
}
// The review is a human/authorized investigator's surface classification, NOT independent verification.
// No labels, selectors, raw URLs, account refs, or executable payloads are accepted or emitted.
export function compileProposal(atlas,review=null){
 const {t,evidence,controls,observations}=inspectAtlas(atlas);
 const atlasDigest=digest(atlas);
 const reviewed=new Map();
 if(review!==null){
  if(!only(review,['schema','siteId','atlasDigest','surfaces'])||review.schema!==REVIEW_SCHEMA||review.siteId!==t.siteId||review.atlasDigest!==atlasDigest||!bounded(review.surfaces,200))fail('REVIEW_SCOPE_OR_REVISION');
  for(const row of review.surfaces){
   if(!only(row,['surface','status','refs'])||!t.surfaces.includes(row.surface)||reviewed.has(row.surface)||!qualifiers.has(row.status)||!bounded(row.refs,100))fail('SURFACE_REVIEW_INVALID');
   const refs=[],seen=new Set();
   for(const r of row.refs){
    if(!only(r,['evidenceId','position'])||!evidence.has(r.evidenceId))fail('REVIEW_EVIDENCE_INVALID');
    const p=r.position??null,k=key(r.evidenceId,p);
    if(seen.has(k)||(p===null?!observations.has(r.evidenceId):!controls.has(k)))fail('REVIEW_REFERENCE_INVALID');
    seen.add(k);refs.push({evidenceId:r.evidenceId,position:p,role:p===null?null:controls.get(k)});
   }
   if(['observed','partial'].includes(row.status)&&!refs.length)fail('REVIEW_EVIDENCE_REQUIRED');
   if(['restricted','not-applicable'].includes(row.status)&&refs.length)fail('REVIEW_NONOBSERVATION_WITH_REFS');
   reviewed.set(row.surface,{status:row.status,refs:refs.sort((a,b)=>sorted(key(a.evidenceId,a.position),key(b.evidenceId,b.position)))});
  }
 }
 const coverage=t.surfaces.map(surface=>({surface,...(reviewed.get(surface)??{status:'unobserved',refs:[]})}));
 const operations=Object.entries(OPERATIONS).filter(([,op])=>t.surfaces.includes(op.surface)).map(([proposalKey,op])=>{
  const c=coverage.find(x=>x.surface===op.surface);
  return {proposalKey,surface:op.surface,effect:op.effect,inputFields:[...op.input],outputShape:op.output,
   observationStatus:c.status,evidenceRefs:c.refs,semanticId:null,actionId:null,binding:null,accountRef:null,
   liveVerified:false,publicInvocation:false,authority:inert()};
 }).sort((a,b)=>sorted(a.proposalKey,b.proposalKey));
 const base={schema:SCHEMA,target:{siteId:t.siteId,targetId:t.targetId,origin:t.origin,kind:t.kind},source:{arborPin:ARBOR_PIN,atlasDigest,bundleDigest:atlas.source.bundleDigest,sourceProfiles:t.sources,externalNativeVerificationReported:atlas.source.nativeVerified===true},coverage,operations,
  counts:{plannedSurfaces:coverage.length,reviewedSurfaces:reviewed.size,observedSurfaces:coverage.filter(c=>c.status==='observed').length,partialSurfaces:coverage.filter(c=>c.status==='partial').length,unobservedSurfaces:coverage.filter(c=>c.status==='unobserved').length},
  authority:inert(),status:'external-evidence-proposal-only'};
 return {...base,proposalDigest:digest(base)};
}
export function makeReviewTemplate(atlas){
 const {t}=inspectAtlas(atlas);
 return {schema:REVIEW_SCHEMA,siteId:t.siteId,atlasDigest:digest(atlas),surfaces:[]};
}
// Compare complete proposal revisions. This flags review invalidation, NOT a proven provider regression.
export function diffProposals(before,after){
 for(const p of [before,after]){
  if(p?.schema!==SCHEMA||!sha.test(p.proposalDigest??'')||!bounded(p.operations,200)||!bounded(p.coverage,200))fail('PROPOSAL_INVALID');
  const {proposalDigest,...base}=p;if(digest(base)!==proposalDigest||Object.keys(inert()).some(k=>p.authority?.[k]!==false))fail('PROPOSAL_DIGEST_INVALID');
 }
 if(before.target.siteId!==after.target.siteId||before.target.origin!==after.target.origin)fail('DELTA_SCOPE_MISMATCH');
 const old=new Map(before.operations.map(x=>[x.proposalKey,x]));
 const changed=after.operations.filter(x=>digest(x)!==digest(old.get(x.proposalKey)??null)).map(x=>x.proposalKey);
 const removed=before.operations.filter(x=>!after.operations.some(y=>y.proposalKey===x.proposalKey)).map(x=>x.proposalKey);
 const revisionChanged=before.source.atlasDigest!==after.source.atlasDigest;
 return {schema:'vivim.craft-provider-proposal-delta/1',siteId:after.target.siteId,before:before.proposalDigest,after:after.proposalDigest,
  revisionChanged,changed,removed,requiresReview:revisionChanged||changed.length>0||removed.length>0,
  status:'proposal-difference-only',authority:inert()};
}
