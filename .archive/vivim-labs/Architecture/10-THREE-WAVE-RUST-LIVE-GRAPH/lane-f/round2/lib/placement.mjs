export const CHOICES=['RUST_NATIVE_NOW','RUST_ADAPTER','SQLITE_CANONICAL','SQLITE_DERIVED','KEEP_EXTERNAL_OWNER','DEFER_MEASUREMENT'];
export function quantile(values,p){
 if(!Array.isArray(values)||!values.length||values.some(x=>typeof x!=='number'||!Number.isFinite(x)||x<0))throw Error('INVALID_SAMPLES');
 const x=[...values].sort((a,b)=>a-b);const idx=Math.ceil(x.length*p)-1;return x[Math.max(0,Math.min(idx,x.length-1))];
}
export function analyzePlacement(components=[]){
 const rows=[],issues=[];
 for(const r of components){
  const observed=r.evidenceClass==='observed-native'&&Array.isArray(r.samplesMs)&&r.samplesMs.length>=30&&r.warmup>=5&&r.owner&&r.sourceSha&&CHOICES.includes(r.recommendation);
  const blocked=!observed||r.recommendation==='DEFER_MEASUREMENT';
  rows.push({id:r.id,owner:r.owner??'UNKNOWN',recommendation:blocked?'DEFER_MEASUREMENT':r.recommendation,
   p50Ms:observed?quantile(r.samplesMs,.5):null,p95Ms:observed?quantile(r.samplesMs,.95):null,sampleCount:r.samplesMs?.length??0,
   measurementStatus:observed?'MEASURED':'NOT_MEASURED',evidenceRef:r.evidenceRef??null});
  if(blocked)issues.push({code:'UNMEASURED_PLACEMENT',id:r.id});
 }
 if(!components.length)issues.push({code:'NO_COMPONENTS'});
 return {status:issues.length?'BLOCKED':'PLACEMENT_MEASURED_NOT_MIGRATION_AUTHORIZED',rows,issues};
}
