/**
 * R005 visual compiler. No execution, approvals, personal lookup or intent authority.
 * Host result always wins when it refuses/clarifies. Everything else is a UI lens.
 */
import { getContacts } from './bridge.mjs';
import { affordanceFor } from './affordances.mjs';
import {scanLexical,detectNegation,commandMenu,resolveTypedReference,semanticSummary,projectHints} from './semantic-lens.mjs';
const slug=s=>String(s).trim().toLowerCase().replace(/[^a-z0-9]+/g,'-').replace(/^-|-$/g,'');
const STOP_OUTCOMES=new Set(['refused','clarification_required']);

export function annotate(text,action=null) {
  const verbs=(action?.language?.verbs||[]).map(x=>String(x).toLocaleLowerCase());
  return scanLexical(text).map(t=>({...t,role:t.role==='verb-candidate'&&verbs.includes(t.text.toLocaleLowerCase())?'verb':t.role}));
}
function shell(command) {
  return {schema:'vivim.labs.visual-interpretation/1',text:command,outcome:'unknown',executionPerformed:false,
    source:'lab-visual',actionId:null,namespace:null,readiness:'unregistered',risk:'unknown',
    reading:'No registered command resolved.',canonical:'',tokens:annotate(command),
    symbols:[],slots:[],suggestions:[],menu:null,gaps:[],confidence:null,provenance:'local_lexical_hint',
    authorization:'not_assessed',intentStatus:'unresolved',safety:null,projects:[],lens:null};
}
function emailContext(text){
  const m=text.match(/^\s*(?:email|e-mail|compose(?: an?)? email|send(?: an?)? email(?: to)?|write(?: an?)? email(?: to)?)\b\s*(.*)$/i);
  if(!m)return null;
  const tail=m[1]||'';
  const recipient=tail.replace(/^to\s+/i,'').split(/\s+(?:about|subject|saying|with\s+subject|body)\b/i)[0].trim();
  const prefixMatch=text.match(/^(\s*(?:email|e-mail|compose(?: an?)? email|send(?: an?)? email(?: to)?|write(?: an?)? email(?: to)?)\s*(?:to\s+)?)/i);
  const recipientStart=prefixMatch?prefixMatch[0].length:text.length-tail.length;
  return {recipient,recipientStart,recipientEnd:recipientStart+recipient.length};
}
function hostFeedback(result,host){
  if(!host)return;
  result.outcome=host.outcome||'unknown';
  result.provenance='host.intent-planner.planText';
  result.confidence=host.language?.feedback?.confidence??null;
  if(Array.isArray(host.language?.feedback?.tokens)) result.tokens=host.language.feedback.tokens.map(t=>({...t,role:t.role||t.kind||'text'}));
  result.suggestions=(host.language?.feedback?.suggestions||[]).map(s=>({kind:s.kind,text:s.text||s.display||'',detail:s.detail||''}));
  if(host.refusal)result.gaps.push({code:host.refusal.code||'HOST_REFUSAL',message:host.refusal.message||'Host refused interpretation'});
  if(host.question)result.gaps.push({code:'CLARIFICATION',message:host.question});
}
function finish(result,host,negation) {
  const feedback=host?.language?.feedback;
  if(feedback?.canonical && !negation.negated && !STOP_OUTCOMES.has(host.outcome))result.canonical=feedback.canonical;
  if(feedback?.reading && !negation.negated && !STOP_OUTCOMES.has(host.outcome))result.reading=feedback.reading;
  if(feedback?.gaps)result.gaps.push(...feedback.gaps.map(g=>({code:String(g.kind||'HOST_FEEDBACK'),message:g.hint||g.text||'Unspecified gap'})));
  result.safety=negation;
  result.projects=projectHints(result.text);
  result.intentStatus=result.actionId&&result.provenance==='host.intent-planner.planText'&&
    ['ready','approval_required','candidate'].includes(result.outcome)&&!negation.negated?'candidate':'unresolved';
  result.authorization='not_assessed'; // Lab cannot query operational actor grants.
  result.lens=semanticSummary(result.text,result);
  return result;
}

export function visualCompile(text,inventory={},hostResult=null) {
  const command=String(text||'').slice(0,1000);
  const actions=Array.isArray(inventory.actions)?inventory.actions:[];
  const hostActions=actions.filter(a=>a.namespace==='host');
  const result=shell(command);
  const negation=detectNegation(command);
  if(!command.trim()){
    result.outcome='empty';result.reading='Type a request to see its visual interpretation.';
    return finish(result,hostResult,negation);
  }
  hostFeedback(result,hostResult);
  if(negation.negated) {
    // The lab errs on the side of preserving the negative and never redirects to a positive action.
    result.outcome=STOP_OUTCOMES.has(hostResult?.outcome)?hostResult.outcome:'blocked_negated';
    result.canonical='⛔ '+command;
    result.reading='A negated or cancelling expression was detected. The visual compiler will not convert this into a positive instruction. '+
      (hostResult?.question||hostResult?.refusal?.message||'Confirm the intended scope before any execution outside this lab.');
    result.symbols=[{icon:'⛔',kind:'modifier',label:'Negation',registered:false}];
    result.gaps.push({code:'NEGATION_SCOPE',message:negation.reason});
    return finish(result,hostResult,negation);
  }
  if(STOP_OUTCOMES.has(hostResult?.outcome)){
    result.canonical='? '+command;
    result.reading=hostResult.refusal?.message||hostResult.question||'Host requires a clarification.';
    result.menu=null;
    return finish(result,hostResult,negation);
  }

  const explicit=resolveTypedReference(command,inventory);
  const discovery=commandMenu(command,inventory);
  if(discovery) {
    result.menu=discovery;
    result.outcome='discovery_only';
    result.provenance='read_only_command_inventory';
    result.canonical=explicit?'⌘ '+explicit.namespace+':'+explicit.id:'⌕ /'+discovery.query;
    result.reading=explicit?`Selected a source-registered ${explicit.namespace} command (${explicit.id}). This is a read-only discovery reference, not an execution request.`:
      `Looking for source-registered commands matching “${discovery.query}”. ${discovery.items.length} suggestion(s) in the current inventory. No operational permission was checked.`;
    if(explicit){
      result.actionId=explicit.id;result.namespace=explicit.namespace;result.readiness=explicit.readiness;
      result.risk=explicit.risk;
      result.symbols=[{icon:explicit.visual?.icon||affordanceFor(explicit).icon,kind:'action',label:explicit.title,registered:true,sigil:explicit.visual?.sigil||null}];
    }else result.symbols=[{icon:'⌕',kind:'discovery',label:'Search commands',registered:false}];
    result.gaps.push({code:'DISCOVERY_ONLY',message:'Typing a source command reference cannot invoke a Host, HQ or TierFlow operation.'});
    return finish(result,hostResult,negation);
  }

  const email=emailContext(command);
  if(email){
    const reg=hostActions.find(a=>a.category==='email'||/^(email|mail)\./.test(a.id));
    const lookup=getContacts(email.recipient);const matches=lookup.contacts;
    const selected=matches.find(c=>c.name.toLowerCase()===email.recipient.toLowerCase()||c.email.toLowerCase()===email.recipient.toLowerCase());
    // Source lookup is for visual example only. A matching fictional contact is NEVER a grounded identity.
    const slot={key:'recipient',label:selected?.name||email.recipient||'Choose a contact',value:selected?.id||null,
      role:'contact',state:selected?'fixture_match':email.recipient?'unresolved':'empty',icon:'♙'};
    result.actionId=reg?.id||null;
    result.namespace=reg?'host':'prototype';result.readiness=reg?.readiness||'not_registered';result.risk=reg?.risk||'unknown';
    result.outcome=selected?'visual_candidate':'slot_required';
    result.canonical=`✉ ${reg?.id||'/email (visual prototype)'} → ${selected?'@'+slug(selected.name):email.recipient?'@?':'@contact'}`;
    result.reading=selected?`Draft an email addressed to ${selected.name} (fictional fixture). Identity is not verified; the lab has not sent anything.`:
      `Compose an email. ${email.recipient?'Resolve the recipient “'+email.recipient+'”.':'Select a contact as the recipient.'}`;
    result.slots=[slot,{key:'subject',label:'Subject',state:'not_provided',icon:'#'}];
    result.symbols=[{icon:'✉',kind:'action',label:'Email',registered:Boolean(reg)},
      {icon:'♙',kind:'entity',label:selected?.name||'Contact',registered:false}];
    result.menu={kind:'contacts',title:'Contacts',query:email.recipient,origin:lookup.origin,
      replace:{start:email.recipientStart,end:email.recipientEnd},items:matches.map(c=>({id:c.id,label:c.name,detail:c.email,insert:c.name}))};
    result.suggestions=matches.map(c=>({kind:'contact',text:c.name,detail:c.email}));
    result.tokens=annotate(command,{language:{verbs:['email','compose','send','write']}});
    if(!reg)result.gaps.push({code:'UNREGISTERED_EMAIL',message:'Email is a visual research prototype; Host catalog has no matching action.'});
    if(lookup.origin==='fictional_demo_fixture')result.gaps.push({code:'DEMO_CONTACTS',message:'Contact suggestions are fictional examples. They are not account contacts.'});
    if(!selected)result.gaps.push({code:'UNRESOLVED_RECIPIENT',message:'Recipient is not grounded to a trusted account contact.'});
    if(reg&&!reg.liveVerified)result.gaps.push({code:'HOST_UNVERIFIED',message:'Registered does not mean currently available or permitted.'});
    return finish(result,hostResult,negation);
  }

  const candidateActionId=hostResult?.actionId||hostResult?.candidate?.actionId||null;
  let action=candidateActionId?hostActions.find(x=>x.id===candidateActionId):null;
  if(!action){
    const literal=command.trim().toLowerCase();
    action=hostActions.find(x=>(x.examples||[]).some(e=>e.trim().toLowerCase()===literal))||null;
  }
  if(action){
    result.actionId=action.id;result.namespace='host';result.readiness=action.readiness;result.risk=action.risk;
    if(!hostResult)result.outcome='discovery_only';
    result.canonical='⌘ '+action.id;
    result.reading=action.language?.reading||action.title;
    result.symbols=[{icon:action.visual?.icon||affordanceFor(action).icon,kind:'action',label:action.title,registered:true,sigil:action.visual?.sigil||null}];
    result.slots=Object.entries(action.args||{}).map(([key,definition])=>({key,label:key.replaceAll('_',' '),state:'not_resolved',type:definition.type,icon:'@'}));
    if(!action.liveVerified)result.gaps.push({code:'NOT_LIVE_VERIFIED',message:'Host catalog registration is not a runtime receipt or grant.'});
  } else if(!hostResult){
    result.outcome='source_unavailable';result.reading='No authoritative Host interpretation was available; the visual lab can display only lexical hints.';
    result.gaps.push({code:'NO_LIVE_PLANNER',message:'Host planner is not connected; no authoritative interpretation performed.'});
  } else {
    result.reading=hostResult.question||hostResult.message||'No deterministic command selected. Clarify or choose a registered command.';
    result.canonical='? '+command;
  }
  return finish(result,hostResult,negation);
}