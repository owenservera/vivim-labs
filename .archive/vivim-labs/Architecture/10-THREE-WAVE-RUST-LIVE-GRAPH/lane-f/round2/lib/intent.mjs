const CAT=['exact','negation','quotation','conjunction','ambiguity','typo','missing-slot','unsupported','wrong-target','span-revision'];
const neg=/\b(?:not|never|don't|dont|do not|cannot|can't|avoid|without|nunca|jamás|jamas|sin|ni|no|neither|nor|stop)\b/iu;
const clauses=/\b(?:and|or|then|before|after|but|y|o|luego|pero)\b|[;&|\r\n]/iu;
export const INTENT_CATEGORIES=Object.freeze(CAT);
export function structuralRisk(sentence){
 if(typeof sentence!=='string'||!sentence.trim()||sentence.length>2048)return {status:'INVALID_INPUT',executable:false};
 let quoted=false,begin=0,visible='',quotes=[];
 for(let i=0;i<sentence.length;i++){
  const char=sentence[i];
  if(char==='"'||char==='“'||char==='”'||char==="'"||char==='‘'||char==='’'){
    if((char==="'"||char==='’') && i>0 && i+1<sentence.length && /[\p{L}]/u.test(sentence[i-1]) && /[\p{L}]/u.test(sentence[i+1])) {visible+=quoted?' ':char;continue;}
    if(!quoted){quoted=char;begin=i;}else if(char===quoted || (quoted==='“'&&char==='”')||(quoted==='‘'&&char==='’')){quotes.push([begin,i+1]);quoted=false;}
    continue;
  }
  visible+=quoted?' ':char;
 }
 if(quoted)return {status:'UNBALANCED_QUOTE',executable:false,quotes};
 if(neg.test(visible))return {status:'NEGATED_REFUSAL',executable:false,quotes};
 if(clauses.test(visible))return {status:'COMPOUND_REFUSAL',executable:false,quotes};
 return {status:quotes.length?'QUOTED_DATA_CANDIDATE':'CANDIDATE_ONLY',executable:false,quotes};
}
export function evaluateIntentCases(cases,{minimum=100}={}){
 const findings=[],counts=Object.fromEntries(CAT.map(x=>[x,{total:0,observed:0,unsafe:0}]));
 if(!Array.isArray(cases)||cases.length<minimum)findings.push({code:'INSUFFICIENT_CORPUS',expected:minimum,actual:cases?.length??0});
 const seen=new Set();for(const c of cases??[]){
  if(!CAT.includes(c.category)){findings.push({code:'UNKNOWN_CATEGORY',id:c.id});continue}
  const bucket=counts[c.category];bucket.total++;
  if(!c.id||seen.has(c.id))findings.push({code:'DUPLICATE_CASE',id:c.id});seen.add(c.id);
  if(c.evidenceClass!=='observed-native'&&c.evidenceClass!=='observed-device'){findings.push({code:'NOT_NATIVE_OBSERVED',id:c.id});continue}
  bucket.observed++;
  if(!/^[0-9a-f]{64}$/.test(c.sourceSha256??'')||!Number.isInteger(c.exitCode)||typeof c.observedStatus!=='string')findings.push({code:'MISSING_NATIVE_RECEIPT',id:c.id});
  if(c.observedStatus!==c.expectedStatus){findings.push({code:'INTENT_MISMATCH',id:c.id});bucket.unsafe++}
  if(['negation','quotation','conjunction','ambiguity','wrong-target'].includes(c.category)&&c.executablePlan===true){findings.push({code:'UNSAFE_EXECUTABLE_INTENT',id:c.id});bucket.unsafe++}
 }
 for(const cat of CAT)if(!counts[cat].total)findings.push({code:'CATEGORY_UNCOVERED',category:cat});
 return {status:findings.length?'BLOCKED':'OBSERVED_CORPUS_PASSED_NOT_HOST_EFFECT_VERIFIED',counts,findings};
}
// Deterministic *test vocabulary*, not a claim of real registered NL command coverage.
export function buildSyntheticCorpus(){
 const samples=[['exact','show index file'],['negation',"don't delete index file"],['quotation','show "delete index file"'],['conjunction','show index file and delete index file'],['ambiguity','show file'],['typo','shwo indes flie'],['missing-slot','show'],['unsupported','mostrar índice'],['wrong-target','delete other project'],['span-revision','show graph on this revision']];
 return Array.from({length:120},(_,i)=>({id:`synthetic-${i+1}`,category:samples[i%samples.length][0],text:samples[i%samples.length][1],evidenceClass:'synthetic',expectedStatus:'UNVERIFIED',observedStatus:structuralRisk(samples[i%samples.length][1]).status}));
}
