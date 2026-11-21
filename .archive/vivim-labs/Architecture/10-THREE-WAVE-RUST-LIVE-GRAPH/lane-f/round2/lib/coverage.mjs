const SEVERITY={critical:4,high:3,medium:2,low:1};
export function ratio(num,den,{eligible=true}={}){
 if(!Number.isInteger(num)||!Number.isInteger(den)||num<0||den<0||num>den)throw Error('INVALID_COVERAGE');
 if(!den)return {numerator:num,denominator:den,percent:null,status:eligible?'BLOCKED_EMPTY_DENOMINATOR':'NOT_APPLICABLE'};
 return {numerator:num,denominator:den,percent:Math.round(num*10000/den)/100,status:num===den?'MEASURED_COMPLETE':'MEASURED_PARTIAL'};
}
export function ledger({roots=[],domains=[],gaps=[]}={}){
 const issues=[],coverage=[],rootIds=new Set();
 for(const r of roots){
  if(!r.id||rootIds.has(r.id))issues.push({kind:'DUPLICATE_ROOT',id:r.id});rootIds.add(r.id);
  if(!/^[0-9a-f]{40}$/.test(r.commitSha??''))issues.push({kind:'ROOT_SHA_MISSING',id:r.id});
  const eligible=r.eligible,ignored=r.ignored,excluded=r.excluded,indexed=r.indexed;
  if(![eligible,ignored,excluded,indexed].every(Number.isInteger)||eligible<0||indexed>eligible)issues.push({kind:'ROOT_DENOMINATOR_MISSING',id:r.id});
  else coverage.push({rootId:r.id,...ratio(indexed,eligible),ignored,excluded});
 }
 if(!roots.length)issues.push({kind:'NO_SOURCE_ROOTS',id:'wave1'});
 const metrics={};for(const x of domains){try{metrics[x.name]=ratio(x.observed,x.eligible,{eligible:x.required!==false})}catch{issues.push({kind:'INVALID_DOMAIN_DENOMINATOR',id:x.name})}}
 for(const [name,value] of Object.entries(metrics))if(value.status==='BLOCKED_EMPTY_DENOMINATOR')issues.push({kind:'UNKNOWN_FOUNDATIONAL_COVERAGE',id:name});
 return {status:issues.length?'BLOCKED':'DENOMINATORS_RECORDED_NOT_RUNTIME_VERIFIED',coverage,metrics,issues,blindSpots:rankGaps(gaps)};
}
export function rankGaps(input=[]){
 const seen=new Set(),ranked=[];
 for(const g of input){const key=[g.kind,g.id,g.sourceSha||''].join('|');if(seen.has(key))continue;seen.add(key);
  const sev=SEVERITY[g.severity]??2,reach=Number.isFinite(g.downstreamReach)?Math.min(10,Math.max(0,g.downstreamReach)):1;
  const confidence=['observed-native','observed-device'].includes(g.evidenceClass)?1:g.evidenceClass==='static-source'?.75:.4;
  ranked.push({...g,score:Math.round(sev*(1+reach)*confidence*100)/100});
 }
 return ranked.sort((a,b)=>b.score-a.score||String(a.id).localeCompare(String(b.id)));
}
