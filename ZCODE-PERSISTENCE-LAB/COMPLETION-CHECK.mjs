// Controller-side contract gate. This checks trusted evaluator receipts, not truth of agent claims.
const nonempty=x=>typeof x==='string'&&x.trim().length>0;
const hex=x=>typeof x==='string'&&/^[0-9a-f]{64}$/.test(x);
export function assess({contract,receipt,status}){
 const hold=(reason)=>({decision:'NOT_COMPLETE',reason});
 if(['WAITING_BUDGET','WAITING_AUTHORIZATION','WAITING_RECONCILIATION','FAILED_BOUNDED','CANCELLED'].includes(status))return hold(status);
 if(status!=='CANDIDATE_COMPLETE')return hold('NO_CANDIDATE');
 if(!contract||contract.frozen!==true||!nonempty(contract.id)||!Number.isInteger(contract.version)||contract.version<1||!hex(contract.digest)||!Array.isArray(contract.mandatory)||!contract.mandatory.length)return hold('NO_FROZEN_CONTRACT');
 const ids=contract.mandatory.map(x=>x.id);
 if(new Set(ids).size!==ids.length||contract.mandatory.some(x=>!nonempty(x.id)||!nonempty(x.behavior)||!nonempty(x.oracle)||!nonempty(x.falsifier)))return hold('INVALID_CRITERIA');
 if(!receipt||receipt.authority!=='controller-independent-evaluator'||receipt.contract_digest!==contract.digest||!hex(receipt.candidate_digest)||receipt.scope_review!=='PASS'||receipt.regressions!=='PASS'||receipt.failure_cases!=='PASS'||receipt.unresolved_mandatory!==0||receipt.unknown_effects!==0)return hold('INDEPENDENT_REVIEW_REQUIRED');
 if(!Array.isArray(receipt.criteria)||receipt.criteria.length!==ids.length||new Set(receipt.criteria.map(x=>x.id)).size!==ids.length)return hold('INCOMPLETE_RECEIPTS');
 for(const id of ids){const r=receipt.criteria.find(x=>x.id===id);if(!r||r.result!=='PASS'||r.candidate_digest!==receipt.candidate_digest||!hex(r.evidence_digest)||!nonempty(r.reproduction_command))return hold('MANDATORY_EVIDENCE_MISSING:'+id);}
 return {decision:'COMPLETE_VERIFIED',contract_id:contract.id,contract_version:contract.version,candidate_digest:receipt.candidate_digest,verified_criteria:ids.length};
}
// Only invoke with receipt obtained from independently controlled evaluator storage.
// This module does not authenticate caller or execute acceptance tests; self-authored receipts are not trusted.
