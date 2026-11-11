/** Visual compiler only. Canonical command meaning comes from Host; this module adds UI annotations. */
import { getContacts } from './bridge.mjs';
const TOKENS=/"[^"]*"|'[^']*'|[\p{L}\p{N}_@#./+%-]+|→|[^\s]/gu;
const ICONS=Object.freeze({email:'✉',mail:'✉',contact:'♙',contacts:'♙',system:'◈',devices:'▣',control:'⌘',browser:'◎',ui:'▧',file:'▤',chat:'☷',message:'▤',search:'⌕',status:'◉',automation:'⏱',other:'⌘'});
const escapeRegExp=s=>s.replace(/[.*+?^${}()|[\]\\]/g,'\\$&');
const slug=s=>String(s).trim().toLowerCase().replace(/[^a-z0-9]+/g,'-').replace(/^-|-$/g,'');
export function annotate(text, action) {
  const re=new RegExp(TOKENS.source,'gu');let m;const tokens=[];
  while((m=re.exec(text))!==null){const v=m[0],lower=v.toLocaleLowerCase();
    const role=/^(@|#|\/)/.test(v)?'symbol':/^(to|for|about|with|from|into)$/i.test(v)?'connector':
      action?.language?.verbs?.some(w=>w.toLowerCase()===lower)?'verb':v.startsWith('"')||v.startsWith("'")?'literal':'text';
    tokens.push({start:m.index,end:m.index+v.length,text:v,role});
  }
  return tokens;
}
function shell(command, extra={}) {
  return {schema:'vivim.labs.visual-interpretation/1',text:command,outcome:'unknown',executionPerformed:false,
    source:'lab-visual',actionId:null,namespace:null,readiness:'unregistered',risk:'unknown',
    reading:'No registered command resolved.',canonical:'',tokens:annotate(command),
    symbols:[],slots:[],suggestions:[],menu:null,gaps:[],confidence:null,provenance:'unknown',...extra};
}
function emailContext(text){
  const m=text.match(/^\s*(?:email|e-mail|compose(?: an?)? email|send(?: an?)? email(?: to)?|write(?: an?)? email(?: to)?)\b\s*(.*)$/i);
  if(!m)return null;
  const tail=m[1]||'';
  // The recipient is the text before the optional about/subject/body phrase.
  const recipient=tail.replace(/^to\s+/i,'').split(/\s+(?:about|subject|saying|with\s+subject|body)\b/i)[0].trim();
  const prefixMatch=text.match(/^(\s*(?:email|e-mail|compose(?: an?)? email|send(?: an?)? email(?: to)?|write(?: an?)? email(?: to)?)\s*(?:to\s+)?)/i);
  const recipientStart=prefixMatch?prefixMatch[0].length:text.length-tail.length;
  return {recipient,recipientStart,recipientEnd:recipientStart+recipient.length};
}
export function visualCompile(text, inventory, hostResult=null, options={}) {
  const command=String(text||'').slice(0,1000);
  const hostActions=inventory.actions.filter(a=>a.namespace==='host');
  const result=shell(command);
  if(!command.trim())return {...result,outcome:'empty',reading:'Type a request to see its visual interpretation.'};
  const email=emailContext(command);
  if(email) {
    const reg=hostActions.find(a=>a.category==='email'||/^(email|mail)\./.test(a.id));
    const lookup=getContacts(email.recipient);
    const matches=lookup.contacts;
    const selected=matches.find(c=>c.name.toLowerCase()===email.recipient.toLowerCase()||c.email.toLowerCase()===email.recipient.toLowerCase());
    const slot={key:'recipient',label:selected?.name||email.recipient||'Choose a contact',value:selected?.id||null,
      role:'contact',state:selected?'fixture_match':email.recipient?'unresolved':'empty',icon:'♙'};
    result.actionId=reg?.id||null;
    result.namespace=reg?'host':'prototype'; result.readiness=reg?.readiness||'not_registered';
    result.risk=reg?.risk||'unknown'; result.outcome=selected?'visual_candidate':'slot_required';
    result.canonical=`✉ ${reg?.id||'/email (visual prototype)'} → ${selected?'@'+slug(selected.name):email.recipient?'@?':'@contact'}`;
    result.reading=selected?`Draft an email addressed to ${selected.name}. The lab has not sent anything.`:
      `Compose an email. ${email.recipient?'Resolve the recipient “'+email.recipient+'”.':'Select a contact as the recipient.'}`;
    result.slots=[slot,{key:'subject',label:'Subject',state:'not_provided',icon:'#'}];
    result.symbols=[{icon:'✉',kind:'action',label:'Email',registered:Boolean(reg)},
      {icon:'♙',kind:'entity',label:selected?.name||'Contact',registered:false}];
    result.menu={kind:'contacts',title:'Contacts',query:email.recipient,origin:lookup.origin,
      replace:{start:email.recipientStart,end:email.recipientEnd},items:matches.map(c=>({id:c.id,label:c.name,detail:c.email,insert:c.name}))};
    result.suggestions=matches.map(c=>({kind:'contact',text:c.name,detail:c.email}));
    result.tokens=annotate(command,{language:{verbs:['email','compose','send','write']}});
    if(!reg)result.gaps.push({code:'UNREGISTERED_EMAIL','message':'Email is a visual research prototype; Host catalog has no matching action.'});
    if(lookup.origin==='fictional_demo_fixture')result.gaps.push({code:'DEMO_CONTACTS','message':'Contact suggestions are fictional examples. They are not account contacts.'});
    if(!selected)result.gaps.push({code:'UNRESOLVED_RECIPIENT','message':'Recipient is not grounded to a trusted account contact.'});
    if(reg&&!reg.liveVerified)result.gaps.push({code:'HOST_UNVERIFIED','message':'Registered does not mean currently available or permitted.'});
    return result;
  }
  const candidateActionId=hostResult?.actionId||hostResult?.candidate?.actionId||null;
  let a=candidateActionId?hostActions.find(x=>x.id===candidateActionId):null;
  if(!a){
    // Discovery-only matching. Never invent an action or treat matching as a Host parse.
    const literal=command.trim().toLowerCase();
    a=hostActions.find(x=>x.examples.some(e=>e.trim().toLowerCase()===literal))||null;
  }
  if(hostResult){
    result.outcome=hostResult.outcome||'unknown';
    result.provenance='host.intent-planner.planText';
    result.confidence=hostResult.language?.feedback?.confidence??null;
    if(Array.isArray(hostResult.language?.feedback?.tokens))result.tokens=hostResult.language.feedback.tokens.map(t=>({...t,role:t.role||t.kind||'text'}));
    result.suggestions=(hostResult.language?.feedback?.suggestions||[]).map(s=>({kind:s.kind,text:s.text||s.display||'',detail:s.detail||''}));
    if(hostResult.refusal)result.gaps.push({code:hostResult.refusal.code||'REFUSED',message:hostResult.refusal.message||'Host refused interpretation'});
    if(hostResult.question)result.gaps.push({code:'CLARIFICATION',message:hostResult.question});
  }
  if(a){
    result.actionId=a.id;result.namespace=a.namespace;result.readiness=a.readiness;result.risk=a.risk;
    if(!hostResult)result.outcome='discovery_only';
    result.canonical='⌘ '+a.id;
    result.reading=a.language?.reading||a.title;
    result.symbols=[{icon:ICONS[a.category]||ICONS.other,kind:'action',label:a.title,registered:true}];
    result.slots=Object.entries(a.args||{}).map(([key,definition])=>({key,label:key.replaceAll('_',' '),state:'not_resolved',type:definition.type,icon:'@'}));
    if(!a.liveVerified)result.gaps.push({code:'NOT_LIVE_VERIFIED',message:'Host catalog source is not a runtime availability or permission receipt.'});
  } else if(!hostResult) {
    result.outcome='source_unavailable';
    result.gaps.push({code:'NO_LIVE_PLANNER',message:'Host planner is not connected; no authoritative interpretation performed.'});
  } else {
    result.reading=hostResult.question||hostResult.message||'No deterministic command selected. Clarify or choose a registered command.';
    result.canonical='? '+command;
  }
  if(hostResult?.language?.feedback?.canonical)result.canonical=hostResult.language.feedback.canonical;
  if(hostResult?.language?.feedback?.reading)result.reading=hostResult.language.feedback.reading;
  if(hostResult?.language?.feedback?.gaps) result.gaps.push(...hostResult.language.feedback.gaps.map(g=>({code:g.kind,message:g.hint||g.text})));
  return result;
}