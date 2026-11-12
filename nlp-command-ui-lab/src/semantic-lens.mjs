/**
 * R005: Pure, zero-effect semantic presentation primitives.
 * This file is intentionally NOT an intent router or authority. It can suggest
 * how to visualize input; only the Host may establish an operational intent.
 */
export const LENS_SCHEMA = 'vivim.labs.semantic-lens/1';
export const DENSITIES = Object.freeze(['quiet', 'assisted', 'xray']);
const TOKEN = /"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'|[\p{L}]+(?:['’][\p{L}]+)?|[\p{L}\p{N}_@#./+%-]+|→|[^\s]/gu;
const NEGATIONS = /^(?:don't|don’t|dont|do not|never|no|not|without|stop|cancel|avoid)$/i;
const LINKS = /^(?:to|for|about|with|from|into|on|at|before|after|then)$/i;
const COMMANDS = /^(?:email|e-mail|send|write|compose|show|list|open|find|search|create|update|delete|remove|publish|rollback|schedule|run|build|change|review|summarize|explain)$/i;
const PROJECTS = Object.freeze([
  ['tierflow',/\btierflow\b/i], ['repos-hq',/\brepos[- ]?hq\b/i],
  ['vivim-web-harness',/\b(?:vivim[- ]web[- ]harness|web harness)\b/i],
  ['vivim-labs',/\bvivim[- ]labs\b/i]
]);
const normalize = value => String(value ?? '').toLocaleLowerCase('en-US');

/** Non-overlapping source spans, preserving user UTF-16 offsets for textarea edits. */
export function scanLexical(text='') {
  const source = String(text).slice(0,1000);
  const tokens=[]; const scanner=new RegExp(TOKEN.source,'gu');let m;
  while((m=scanner.exec(source))!==null){
    const part=m[0];let role='text';
    if(part.startsWith('"')||part.startsWith("'"))role='literal';
    else if(NEGATIONS.test(part))role='negation';
    else if(LINKS.test(part))role='connector';
    else if(/^\/[a-z0-9_.-]+$/i.test(part))role='command-reference';
    else if(/^@/.test(part))role='entity-reference';
    else if(/^#/.test(part))role='tag';
    else if(COMMANDS.test(part))role='verb-candidate';
    else if(/^\d{4}-\d\d-\d\d$/.test(part))role='date';
    tokens.push({text:part,start:m.index,end:m.index+part.length,role});
  }
  return tokens;
}

/** Negation scope is conservative: identify a syntactic negative, never discard it. */
export function detectNegation(text='') {
  const source=String(text); const candidates=scanLexical(source);
  for(let i=0;i<candidates.length;i++){
    const t=candidates[i],next=candidates[i+1];
    const isPair=normalize(t.text)==='do' && normalize(next?.text)==='not';
    const negated=NEGATIONS.test(t.text)||isPair;
    if(!negated)continue;
    const boundary=source.slice(isPair?next.end:t.end).search(/[.!?;\n]/);
    const end=boundary===-1?source.length:(isPair?next.end:t.end)+boundary;
    return {negated:true,start:t.start,end,cue:isPair?'do not':t.text,reason:'Negative instruction detected; do not silently convert it to a positive command.'};
  }
  return {negated:false,start:null,end:null,cue:null,reason:null};
}

/** A soft project hint is not a grounded resource ID. */
export function projectHints(text='') {
  return PROJECTS.filter(([,rx])=>rx.test(text)).map(([name])=>({name,status:'lexical_hint'}));
}
const lower = s=>String(s||'').toLowerCase();
export function commandQuery(text='') {
  // Command discovery only for an explicit leading slash, never quoted text.
  const m=String(text).match(/^\s*\/([a-z0-9._-]*)$/i);
  return m?m[1]:null;
}
function scoreCommand(a,needle){
  const id=lower(a.id),title=lower(a.title),category=lower(a.category),q=lower(needle);
  if(!q)return 1;
  if(id===q)return 10;
  if(id.startsWith(q))return 9;
  if(id.split('.').pop()===q)return 8;
  if(title.startsWith(q))return 7;
  if(id.split('.').some(x=>x.startsWith(q)))return 6;
  if(category.startsWith(q))return 5;
  if(title.includes(q))return 4;
  if(id.includes(q))return 3;
  return 0;
}
/** Suggestions belong to the real source-defined command census, never a fabricated list. */
export function commandMenu(text,inventory,{limit=8}={}) {
  const needle=commandQuery(text);if(needle===null)return null;
  const actions=Array.isArray(inventory?.actions)?inventory.actions:[];
  const seen=new Set();
  const items=actions.map(a=>({a,score:scoreCommand(a,needle)})).filter(x=>x.score>0)
    .sort((a,b)=>b.score-a.score||String(a.a.namespace).localeCompare(String(b.a.namespace))||String(a.a.id).localeCompare(String(b.a.id)))
    .filter(x=>{const key=x.a.namespace+':'+x.a.id;if(seen.has(key))return false;seen.add(key);return true;})
    .slice(0,Math.max(1,Math.min(20,limit))).map(({a})=>({
      id:`${a.namespace}:${a.id}`,label:a.title||a.id,detail:`${a.namespace} · ${a.readiness||'unknown'} · ${a.id}`,
      insert:`/${a.id}`,identity:{namespace:a.namespace,id:a.id},verified:false,
      icon:a.visual?.icon||'⌘',sigil:a.visual?.sigil||null
    }));
  return {kind:'commands',title:'Registered command discovery',origin:'source_census_not_runtime_authority',
    query:needle,replace:{start:String(text).indexOf('/'),end:String(text).length},items};
}

export function resolveTypedReference(text,inventory) {
  const m=String(text).match(/^\s*\/([a-z0-9._-]+)$/i);if(!m)return null;
  const candidates=(inventory?.actions||[]).filter(a=>a.id===m[1]);
  return candidates.length===1?candidates[0]:null;
}

/** Tiny affordances for a compact strip: never call a permission check. */
export function semanticSummary(text,preview=null) {
  const tokens=scanLexical(text);const negation=detectNegation(text),projects=projectHints(text);
  const isQuotedOnly=tokens.length===1&&tokens[0].role==='literal';
  const sourceOutcome=preview?.outcome||'lexical_only';
  const understood=preview?.actionId && preview?.provenance==='host.intent-planner.planText' &&
    ['ready','approval_required','candidate'].includes(sourceOutcome) && !negation.negated
    ? 'candidate':'unresolved';
  const executable='not_assessed'; // The lab is never the operational authority.
  const signals=[];
  if(negation.negated)signals.push({key:'negation',icon:'⛔',label:'Negation preserved',tone:'warning'});
  else if(isQuotedOnly)signals.push({key:'literal',icon:'❝',label:'Quoted text',tone:'neutral'});
  else if(preview?.actionId)signals.push({key:'action',icon:preview?.symbols?.[0]?.icon||'⌘',label:preview.actionId,tone:'info'});
  else if(commandQuery(text)!==null)signals.push({key:'discover',icon:'⌕',label:'Command discovery',tone:'neutral'});
  else if(tokens.some(t=>t.role==='verb-candidate'))signals.push({key:'verb',icon:'⌘',label:'Possible action',tone:'neutral'});
  for(const project of projects.slice(0,2))signals.push({key:'project:'+project.name,icon:'▣',label:project.name+'?',tone:'neutral'});
  if(preview?.menu?.kind==='contacts')signals.push({key:'contact',icon:'♙',label:preview.slots?.[0]?.label||'Recipient?',tone:'warning'});
  if(preview?.menu?.kind==='commands')signals.push({key:'commands',icon:'⌕',label:`${preview.menu.items.length} source matches`,tone:'neutral'});
  if(!signals.length&&tokens.length)signals.push({key:'undetermined',icon:'?',label:'Unresolved',tone:'neutral'});
  return {schema:LENS_SCHEMA,signals:signals.slice(0,4),tokens,negation,projects,
    understood,executable,sourceOutcome,provenance:preview?.provenance||'local_lexical_hint',
    caveat:'Visual-only; availability and authorization are not assessed by the lab.'};
}

/** Detect meaningful changes without comparing irrelevant changing DOM state. */
export function meaningDelta(before,after) {
  if(!before)return {kind:'initial',label:'First interpretation'};
  const prior=semanticSummary(before.text,before),next=semanticSummary(after.text,after);
  if(prior.negation.negated!==next.negation.negated)return {kind:'safety',label:next.negation.negated?'Negation detected':'Negation removed'};
  if(before.actionId!==after.actionId)return {kind:'action',label:after.actionId?'Action candidate changed':'Action candidate cleared'};
  const oldSlots=JSON.stringify(before.slots||[]),newSlots=JSON.stringify(after.slots||[]);
  if(oldSlots!==newSlots)return {kind:'slot',label:'Arguments changed'};
  if(before.outcome!==after.outcome)return {kind:'status',label:'Understanding updated'};
  return {kind:'stable',label:'Meaning unchanged'};
}

/** Applies a chosen suggestion to the exact source substring; rejects stale views. */
export function replaceSuggestion(source,menu,item) {
  if(!menu?.replace||!item||typeof item.insert!=='string')return null;
  const {start,end}=menu.replace;
  if(!Number.isInteger(start)||!Number.isInteger(end)||start<0||end<start||end>source.length)return null;
  if(menu.kind==='commands' && commandQuery(source)===null)return null;
  if(menu.kind==='contacts' && typeof menu.query==='string' &&
     normalize(source.slice(start,end))!==normalize(menu.query))return null;
  const text=source.slice(0,start)+item.insert+source.slice(end);
  if(text.length>1000)return null;
  return {text,start,end: start+item.insert.length,caret:start+item.insert.length,
    undo:{text:source,expected:text,caret:end}};
}