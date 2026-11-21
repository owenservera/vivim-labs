// Actual E relation-aware read projection: source -> compiled wiki program -> B semantic -> C command.
// A text string, comment annotation, or producer's verified claim never certifies a real runtime.
import {validateEGraph} from '../contracts/validate-inputs.mjs';
import {sourceProvenance,edgeState} from './provenance.mjs';
import {semanticState,commandState,compilationState} from './states.mjs';
const REL={compile:'COMPILES_WIKI_PROGRAM',meaning:'DECLARES_INTENT_FOR',handler:'HAS_REGISTERED_HANDLER'};
const isB=n=>n?.metadata?.source_lane==='B';
const isC=n=>n?.metadata?.source_lane==='C';
const isA=n=>n?.metadata?.source_lane==='A';
const cmp=(a,b)=>String(a).localeCompare(String(b),'en');
export function traceCompiled(input) {
 const view=input?.nodes instanceof Map?input:validateEGraph(input);
 const nodes=view.nodes, edgeIndex=new Map();
 for(const edge of view.edges){const key=edge.from+'\0'+edge.relation; const rows=edgeIndex.get(key)??[];rows.push(edge);edgeIndex.set(key,rows);}
 const get=(from,relation)=>edgeIndex.get(from+'\0'+relation)??[];
 const roots=[...nodes.values()].filter(n=>n.kind==='source-root');
 const chains=[];const defects=[];
 for (const code of [...nodes.values()].filter(isA)) {
   const pins=sourceProvenance(code,roots);
   for (const ac of get(code.id,REL.compile)) {
     const compiled=nodes.get(ac.to);if(compiled?.kind!=='compiled-wiki-program'){defects.push({code:'compiler_target_wrong_kind',edge:ac.id});continue;}
     const compilation=compilationState(compiled);
     for (const cb of get(compiled.id,REL.meaning)) {
       const sem=nodes.get(cb.to);if(!isB(sem)||!sem.metadata?.semantic_id){defects.push({code:'semantic_identity_unbound',edge:cb.id});continue;}
       for(const bc of get(sem.id,REL.handler)) {
         const cmd=nodes.get(bc.to);if(!isC(cmd)){defects.push({code:'command_target_unregistered',edge:bc.id});continue;}
         const semantic=semanticState(sem),command=commandState(cmd);
         const identityAgrees=Boolean(semantic.identity && semantic.identity===command.identity);
         const e=[ac,cb,bc].map(edgeState);
         const declared=e.some(x=>x.claim==='declared');
         const linkedKb=[...view.edges].filter(x=>(x.from===sem.id||x.to===sem.id)&&[x.from,x.to].some(id=>nodes.get(id)?.metadata?.source_lane==='B' && nodes.get(id)?.kind!=='semantic_id'))
              .map(x=>({edge:x.id,knowledgeNode:(x.from===sem.id?x.to:x.from),evidence:edgeState(x)}));
         const problems=[...pins.defects];
         if(!identityAgrees)problems.push('semantic_command_identity_mismatch');
         if(!semantic.acceptedMeaning)problems.push('unaccepted_semantic_meaning');
         if(!command.handlerSourcePresent)problems.push('handler_source_unverified');
         if(!command.cliSyntax)problems.push('cli_syntax_unverified');
         if(!linkedKb.length)problems.push('knowledge_not_joined');
         if(!compilation.compiledInert)problems.push('compiler_inert_state_missing');
         if(!compiled.metadata?.content_sha256 || compiled.metadata.content_sha256!==code.metadata?.content_sha256)problems.push('compiled_source_digest_mismatch');
         if(e.some(x=>!x.evidence && x.claim!=='source-scoped'))problems.push('edge_evidence_missing');
         if(declared)problems.push('declaration_not_independent_execution_proof');
         chains.push({sourceId:code.id,compilerId:compiled.id,semanticId:sem.id,commandId:cmd.id,logicalSemanticId:semantic.identity,source:pins,semantic,command,compilation,linkedKb,edgeIds:[ac.id,cb.id,bc.id],edgeStates:e,status:'CANDIDATE_INERT_NEEDS_INDEPENDENT_PROOF',independentlyVerified:false,problems:[...new Set(problems)].sort(cmp)});
       }
     }
   }
 }
 if(!chains.length) defects.push({code:'no_compiled_source_semantic_command_chain'});
 return {schema:'vivim.wave1.lane-d.compiled-lineage/1',status:chains.length?'CANDIDATES_ONLY':'BLOCKED_NO_CHAIN',chains:chains.sort((a,b)=>cmp(a.sourceId,b.sourceId)||cmp(a.semanticId,b.semanticId)),defects,observed:{candidates:chains.length,independentlyVerified:0}};
}
