// No scalar "wired=true" can be lifted to an effect permission, runtime or device observation.
export function semanticState(node) {
 const m=node?.metadata??{};const status=m.status??'unknown';
 return {identity:typeof m.semantic_id==='string'?m.semantic_id:null,registryStatus:['proposed','current','deprecated'].includes(status)?status:'unknown',acceptedMeaning:status==='current'&&typeof m.description==='string'&&m.description.trim().length>0,ownerAuthorityVerified:false};
}
export function commandState(node) {
 const m=node?.metadata??{};
 return {identity:typeof m.semantic_id==='string'?m.semantic_id:null,registryStatus:m.status??'unknown',handlerSourcePresent:Boolean(m.handler||m.dispatch),cliSyntax:typeof m.syntax==='string'?m.syntax:null,runtimeTested:false,deviceObserved:false,effectPermission:false,executionClaim:'UNVERIFIED',negativeIntentSafe:'NOT_TESTED'};
}
export function compilationState(node){const m=node?.metadata??{};return {kind:node?.kind,compilerVersion:m.compiler_version??'unknown',compiledInert:node?.kind==='compiled-wiki-program'&&m.status==='inert',handlerBound:false,ownerApproved:false};}
