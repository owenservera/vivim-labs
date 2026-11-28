// R013 contextual sufficiency: pure controller-side decision primitives.
// Inputs must be independently verified by the controller; never trust worker claims.
import crypto from 'node:crypto';
import {assess} from './COMPLETION-CHECK.mjs';
const hash=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
const hex=x=>typeof x==='string'&&/^[0-9a-f]{64}$/.test(x);
const num=x=>typeof x==='number'&&Number.isFinite(x)&&x>=0;
export function judge({contract,receipt,status,context,observed_at}){
 const result=assess({contract,receipt,status});
 if(result.decision!=='COMPLETE_VERIFIED')return {state:status==='WAITING_BUDGET'?'SUSPENDED_RESOURCES':'UNVERIFIED',reason:result.reason};
 if(!hex(context?.assumptions_digest)||!hex(context?.operating_envelope_digest)||!hex(context?.purpose_digest)||!hex(context?.evidence_standard_digest)||!Number.isFinite(Date.parse(observed_at)))return {state:'UNVERIFIED',reason:'CONTEXT_NOT_BOUND'};
 const expected={purpose_digest:hash(contract.purpose),operating_envelope_digest:hash(contract.operating_envelope),evidence_standard_digest:hash(contract.evidence_standard)};
 if(Object.entries(expected).some(([k,v])=>context[k]!==v)||receipt.context_digest!==hash(context))return {state:'UNVERIFIED',reason:'CONTEXT_MISMATCH'};
 const body={state:'FUNCTIONALLY_SUFFICIENT',contract_digest:contract.digest,candidate_digest:receipt.candidate_digest,context:structuredClone(context),receipt_digest:hash(receipt),observed_at,optional_count:contract.optional_backlog?.length??0};
 return {...body,judgment_digest:hash(body)};
}
export function optimize({judgment,benefit_low,benefit_high,cost_low,cost_high,minimum_net_value=0,budget_available,essential_gap=false}){
 if(essential_gap)return {decision:'REOPEN_REQUIRED',reason:'ESSENTIAL_GAP'};
 if(judgment?.state!=='FUNCTIONALLY_SUFFICIENT')return {decision:'FINISH_ESSENTIALS_FIRST'};
 if(budget_available===false)return {decision:'DEFER_RESOURCE_LIMIT',reason:'NOT_ECONOMIC_EVIDENCE'};
 if(budget_available!==true)return {decision:'DEFER_UNKNOWN_RESOURCES'};
 if(![benefit_low,benefit_high,cost_low,cost_high,minimum_net_value].every(num)||benefit_low>benefit_high||cost_low>cost_high)return {decision:'DEFER_UNKNOWN_VALUE'};
 // Costs and benefits must be supplied in comparable decision units.
 if(benefit_low-cost_high>=minimum_net_value&&benefit_low>cost_high)return {decision:'CONTINUE_OPTIONAL',reason:'ROBUST_POSITIVE_NET_VALUE'};
 if(benefit_high-cost_low<=minimum_net_value)return {decision:'DEFER_DIMINISHING_RETURNS',reason:'UPPER_BOUND_BELOW_THRESHOLD'};
 return {decision:'DEFER_OR_BOUNDED_EXPLORATION',reason:'VALUE_UNCERTAIN'};
}
export function reopen({judgment,contract,change}){
 if(judgment?.state!=='FUNCTIONALLY_SUFFICIENT'||judgment.contract_digest!==contract?.digest)throw Error('JUDGMENT_NOT_BOUND');
 if(!change||typeof change.reason!=='string'||!change.reason.trim()||!hex(change.evidence_digest)||!['PURPOSE','ENVELOPE','ASSUMPTIONS','CANDIDATE','EVIDENCE','ESSENTIAL_GAP','OPTIONAL_ONLY'].includes(change.kind))throw Error('CHANGE_NOT_VERIFIED');
 if(change.kind==='OPTIONAL_ONLY')return {state:'FUNCTIONALLY_SUFFICIENT',historical_judgment:judgment,affected:[],reason:change.reason};
 const ids=contract.mandatory.map(x=>x.id),known=new Set(ids);
 // Purpose changes and unknown impacts require full review. A trusted impact
 // analysis may otherwise identify directly affected criteria.
 let roots=change.kind==='PURPOSE'||change.impact_verified!==true||!Array.isArray(change.affected)||!change.affected.length?ids:change.affected;
 if(roots.some(x=>!known.has(x)))throw Error('UNKNOWN_OBLIGATION');
 const affected=new Set(roots);
 let expanded=true;while(expanded){expanded=false;for(const r of contract.mandatory){if(!affected.has(r.id)&&(r.depends_on??[]).some(d=>affected.has(d))){affected.add(r.id);expanded=true;}}}
 return {state:'SUFFICIENCY_INVALIDATED',historical_judgment:judgment,affected:[...affected].sort(),reason:change.reason,change_digest:hash(change),requires_new_qualification:true};
}
export const contextDigest=hash;
