// Synthetic-only product surface projection validator. Not a live product export.
const KEYS=['graphId','semanticId','safeTitle','safeSummary','status','effectClass','permissionOwner','sourceEvidenceSummary','unknowns'];
const deny=/\b(?:api[_-]?key|password|private[_-]?key|bearer|cookie|token)\b|(?:\/mnt\/data\/|[a-z]:\\users\\|(?:^|\/)\.env(?:\.|$))/i;
export function projectProductFixture(record){
 if(!record||typeof record!=='object'||Array.isArray(record))throw Error('invalid_record');
 const out={};
 for(const k of KEYS){const v=record[k];
   if(v===undefined)continue;
   if(k==='unknowns'){
     if(!Array.isArray(v)||v.length>50||v.some(x=>typeof x!=='string'||x.length>160||deny.test(x)))throw Error('unsafe_unknowns');
     out[k]=[...v];
   }else{
     if(typeof v!=='string'||v.length>512||deny.test(v))throw Error('unsafe_product_field:'+k);
     out[k]=v;
   }
 }
 // Never inherit this from any source claim, even true in a fixture.
 out.actionExecutableVerified=false;
 out.schema='vivim.wave1.product-wiki/PROPOSED';
 out.ownerAuthority='NO_GRANT';
 return Object.freeze(out);
}
