import {HEX64} from './evidence.mjs';
const GRAMMARS=new Set(['hq-v1','hq-v2','hq-wiki-rust']);
export function evaluateCompiler(cases,{minimum=60,requireRealChain=true}={}){
 const findings=[],seen=new Set(),byGrammar={};let native=0,linked=0;
 if(!Array.isArray(cases)||cases.length<minimum)findings.push({code:'COMPILER_CORPUS_TOO_SMALL',actual:cases?.length??0,minimum});
 for(const c of cases??[]){
  if(!c.id||seen.has(c.id))findings.push({code:'COMPILER_DUPLICATE_CASE',id:c?.id});seen.add(c?.id);
  if(!GRAMMARS.has(c.grammar))findings.push({code:'COMPILER_GRAMMAR_UNKNOWN',id:c?.id});
  byGrammar[c.grammar]=(byGrammar[c.grammar]??0)+1;
  if(!HEX64.test(c.sourceSha256??'')||!Number.isInteger(c.sourceLine)||c.sourceLine<1)findings.push({code:'SOURCE_SPAN_UNVERIFIED',id:c.id});
  if(c.expectedOutcome!==c.observedOutcome)findings.push({code:'COMPILER_CONFORMANCE_MISMATCH',id:c.id});
  if(c.evidenceClass==='observed-native'){
    native++;
    if(c.linkedToSemantic===true&&c.linkedToRegisteredHandler===true&&c.sourceDigestVerified===true&&c.effectAuthorizationGranted!==true)linked++;
  }else findings.push({code:'SYNTHETIC_COMPILER_ONLY',id:c.id});
  if(c.effectAuthorizationGranted===true)findings.push({code:'COMPILER_CANNOT_GRANT_EFFECT_AUTHORITY',id:c.id});
 }
 for(const g of GRAMMARS)if(!byGrammar[g])findings.push({code:'GRAMMAR_NOT_COVERED',grammar:g});
 if(requireRealChain && !linked)findings.push({code:'REAL_COMPILED_SEMANTIC_HANDLER_CHAIN_MISSING'});
 return {status:findings.length?'BLOCKED':'NATIVE_COMPILER_CONFORMANCE_PROVISIONAL',counts:{total:cases?.length??0,native,linked,byGrammar},findings};
}
export function sampleCompilerVectors(){
 const tags=['@hq:context','@hq:type','@hq:wiki'];const grammar=['hq-v1','hq-v2','hq-wiki-rust'];
 return Array.from({length:60},(_,i)=>({id:'synthetic-comment-'+(i+1),grammar:grammar[i%3],text:(i%4===0?'"'+tags[i%3]+'"':'// '+tags[i%3]+' example'),sourceSha256:'a'.repeat(64),sourceLine:i+1,
   expectedOutcome:'UNEXECUTED',observedOutcome:'UNEXECUTED',evidenceClass:'synthetic',linkedToSemantic:false,linkedToRegisteredHandler:false,sourceDigestVerified:false}));
}
