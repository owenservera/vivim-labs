// Domain and scope denominators must be known; false confidence cannot be averaged away.
const metric=(name,numerator,denominator)=>{
 if(denominator===null||denominator===undefined) return {name,status:'NOT_MEASURED',numerator:numerator??null,denominator:null,percent:null};
 if(!Number.isSafeInteger(denominator)||denominator<0||!Number.isSafeInteger(numerator)||numerator<0||numerator>denominator) throw Error('invalid_coverage_denominator:'+name);
 if(denominator===0) return {name,status:'NOT_MEASURED',numerator,denominator,percent:null};
 return {name,status:numerator===denominator?'COMPLETE':'INCOMPLETE',numerator,denominator,percent:Math.round(numerator*10000/denominator)/100};
};
export function scoreQuality({roots=[],semantic=null,knowledge=null,commands=null,compiled=null,wiki=null,links=null,critical=[],independentWitness=false}={}) {
 if(!Array.isArray(roots)||!Array.isArray(critical))throw Error('invalid_quality_input');
 const scope=roots.map(x=>({rootId:x.id??null,...metric('eligible_files',x.indexed??null,x.eligible??null),excluded:x.excluded??null,declared:x.declared??null}));
 const measurements=[metric('semantic_source_bound',semantic?.linked??null,semantic?.eligible??null),metric('knowledge_linked',knowledge?.linked??null,knowledge?.eligible??null),metric('command_handler_source',commands?.sourceWired??null,commands?.eligible??null),metric('command_runtime_verified',commands?.runtimeVerified??null,commands?.eligible??null),metric('compiled_source_bound',compiled?.sourceBound??null,compiled?.eligible??null),metric('wiki_verified_chains',wiki?.verified??null,wiki?.eligible??null),metric('wiki_links_resolved',links?.resolved??null,links?.checked??null)];
 const blockers=[...new Set(critical)];
 if(!roots.length || scope.some(x=>x.status!=='COMPLETE'))blockers.push('INCOMPLETE_SCOPE');
 for (const k of ['semantic_source_bound','knowledge_linked','command_handler_source','compiled_source_bound','wiki_verified_chains']){
  const r=measurements.find(x=>x.name===k);
  if(r.status!=='COMPLETE')blockers.push('INCOMPLETE_'+k.toUpperCase());
 }
 if(!independentWitness)blockers.push('NO_INDEPENDENT_RUST_AND_WIKI_WITNESS');
 // No overall "score" is emitted; a single average cannot establish safety.
 return {schema:'vivim.wave1.lane-d.quality/2',status:blockers.length?'BLOCKED':'READY_FOR_EXTERNAL_GATE_REVIEW',notAnAuthorityGrant:true,scope,measurements,blockers:[...new Set(blockers)].sort()};
}
