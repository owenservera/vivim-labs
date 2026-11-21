/**
 * PRE-G synthetic read projection ONLY. No ambient IO, persistence, routing,
 * corpus scanning, command execution, grant minting, or UI mutation.
 * Uses research/autowiki-screen-mirror contracts /0.1 without renaming HQ IDs.
 */
export const SCREEN_SCHEMA = 'vivim.screen-context/0.1';
export const PAGE_SCHEMA = 'vivim.autowiki-page/0.1';
export const DELTA_SCHEMA = 'vivim.autowiki-delta/0.1';
const REF = /^[A-Za-z0-9][A-Za-z0-9._:-]{0,190}$/;
const ITEM_STATUS = new Set(['observed','accepted-decision','proposal','historical','conflicted','stale']);
const AUTHORITY = new Set(['hq-development','host-personal','host-runtime','tierflow-world']);
const SCORE = {'accepted-decision':0,observed:1,conflicted:2,historical:3,proposal:4,stale:5};
const str = (v,n) => typeof v === 'string' ? v.slice(0,n) : '';
const valid = v => typeof v === 'string' && REF.test(v);
// HQ native document IDs include path separators and project markers (kb.mjs docNodeId).
// Never rewrite those IDs to fit an opaque Host token regex.
const nativeId = v => typeof v === 'string' && v.length > 0 && v.length <= 191 &&
  !/[\u0000-\u001f\u007f]/.test(v);
const hash = x => { // identity hint only, NEVER a security/authorization digest
  let n=2166136261; for(const c of String(x)){n=Math.imul(n^c.charCodeAt(0),16777619);}
  return (n>>>0).toString(16).padStart(8,'0');
};
export function assertScreenContext(s) {
  if (!s || s.schema!==SCREEN_SCHEMA || !valid(s.contextId) || !Number.isSafeInteger(s.revision) || s.revision<1 ||
      typeof s.observedAt!=='string' || !s.source || !s.focus || !s.scope || !s.coverage ||
      !valid(s.scope.ownerRef) || !valid(s.scope.worldRef) || !valid(s.scope.policyRevision) ||
      s.scope.authority!=='host-trusted-scope' || s.scope.purpose!=='autowiki-read' ||
      !['native','bound-web','manual','unsupported'].includes(s.source.kind) ||
      !Array.isArray(s.focus.objectRefs) || !Array.isArray(s.focus.projectCandidates) ||
      !Array.isArray(s.focus.topicTerms) || !['none','present-unsubmitted','unknown','redacted'].includes(s.focus.draft?.state) ||
      !['complete','partial','unavailable','denied'].includes(s.coverage.status) ||
      !Array.isArray(s.coverage.unavailableSources)) throw new TypeError('INVALID_SCREEN_CONTEXT');
  return s;
}
function authorized(s, trusted) {
  // Scope fields in ScreenContext are comparisons, NOT a grant. Only readScope
  // from an injected trusted Host broker is capable of authorizing this read.
  return !!trusted && trusted.authorized===true && trusted.revoked!==true &&
    trusted.ownerRef===s.scope.ownerRef && trusted.worldRef===s.scope.worldRef &&
    trusted.policyRevision===s.scope.policyRevision && trusted.purpose==='autowiki-read' &&
    Array.isArray(trusted.allowedAudiences) && trusted.allowedAudiences.length>0;
}
function focusOf(s, denied=false) {
  const projects=s.focus.projectCandidates.filter(p=>p && nativeId(p.ref));
  const trusted=projects.filter(p=>p.state==='trusted');
  const ambiguous=projects.length>1 || (projects.length && trusted.length!==1);
  if(denied) return {title:'Access unavailable',projectRef:null,state:'unresolved',source:'heuristic'};
  return {title:str(s.focus.title,180),projectRef:ambiguous?null:(trusted[0]?.ref||null),
    state:ambiguous?'unresolved':trusted.length===1?'trusted':'unresolved',
    source:trusted.length===1 ? (trusted[0].evidence==='bound-thread'?'bound-thread':'native-state') : 'heuristic'};
}
function emptyPage(s,status,readers=[],omissions=[],now=new Date().toISOString(),focus=focusOf(s,true)) {
  const rev='projection:'+hash(s.contextId+':'+s.revision+':'+status);
  return {schema:PAGE_SCHEMA,pageId:'auto:'+hash(s.contextId+':'+s.scope.ownerRef+':'+s.scope.worldRef),
    contextId:s.contextId,contextRevision:s.revision,projectionRevision:rev,asOf:now,status,focus,
    sections:[],items:[],coverage:{state:status==='denied'?'denied':'unknown',
      screenCoverage:'not released',corpusCoverage:'not released',readerStates:readers},
    sourceRevisions:[],omissions,readOnly:true,effectPerformed:false};
}
function cleanRecord(r,s,scope) {
  // Secondary narrowing on already-authorized read-port results; never trust
  // a provider snippet or a source notice to grant source access.
  if(!r || !nativeId(r.id) || !nativeId(r.source?.nativeRef) || !nativeId(r.source?.revision) ||
     !nativeId(r.source?.digest) || !AUTHORITY.has(r.authority) || !ITEM_STATUS.has(r.status) ||
     r.ownerRef!==scope.ownerRef || r.worldRef!==scope.worldRef ||
     !scope.allowedAudiences.includes(r.audience) ||
     (r.projectRef && !s.focus.projectCandidates.some(p=>p.state==='trusted' && p.ref===r.projectRef))) return null;
  const src={nativeRef:r.source.nativeRef,revision:r.source.revision,digest:r.source.digest};
  if(valid(r.source.repoRef))src.repoRef=r.source.repoRef;
  if(typeof r.source.path==='string')src.path=str(r.source.path,500);
  if(Number.isSafeInteger(r.source.line)&&r.source.line>0)src.line=r.source.line;
  return {id:r.id,kind:str(r.kind,64)||'source',title:str(r.title,180),
    summary:str(r.summary,512),authority:r.authority,source:src,status:r.status,
    rankReasons:Array.isArray(r.rankReasons)?r.rankReasons.filter(x=>[
      'active-object','active-project','active-topic','recent-source-event','accepted-decision',
      'verified-runtime','related-graph','source-coverage','manual-pinned-focus'].includes(x)).slice(0,8):[],
    sourceTimes:{occurredAt:r.sourceTimes?.occurredAt||null,indexedAt:r.sourceTimes?.indexedAt||null,
      verifiedAt:r.sourceTimes?.verifiedAt||null}};
}
function draftItem(s) {
  if(s.focus.draft?.state!=='present-unsubmitted' || !valid(s.focus.draft.digest) ||
     !nativeId(s.source.viewRef))return null;
  return {id:'draft:'+hash(s.contextId+':'+s.revision),kind:'draft_intent',
    title:'Unsubmitted editor draft',summary:'Provisional input, not an accepted decision.',
    authority:'tierflow-world',source:{nativeRef:s.source.viewRef,
      revision:str(s.source.viewRevision||String(s.revision),191),digest:s.focus.draft.digest},
    status:'proposal',rankReasons:[],sourceTimes:{occurredAt:null,indexedAt:null,verifiedAt:null}};
}
export function autoWikiDelta(before,after) {
  if(!before || !after || before.schema!==PAGE_SCHEMA || after.schema!==PAGE_SCHEMA)throw new TypeError('INVALID_PAGE');
  const same=before.contextId===after.contextId && before.contextRevision===after.contextRevision;
  const kind=after.status==='denied'?'revoked':!same?'resnapshot-required':'changed';
  const old=new Map(before.items.map(i=>[i.id,i]));
  const next=new Map(after.items.map(i=>[i.id,i]));
  return {schema:DELTA_SCHEMA,contextId:after.contextId,contextRevision:after.contextRevision,
    priorProjectionRevision:before.projectionRevision,nextProjectionRevision:after.projectionRevision,
    kind,removedItemIds:[...old.keys()].filter(id=>!next.has(id)).slice(0,50),
    changedItemIds:[...next.keys()].filter(id=>!old.has(id)||
      JSON.stringify(old.get(id))!==JSON.stringify(next.get(id))).slice(0,50),
    reason:kind==='resnapshot-required'?'screen-revision-changed':kind==='revoked'?'scope-revoked':'source-delta'};
}
export function createAutoWikiMirror({readScope,readSources,now=()=>new Date().toISOString()}={}) {
  if(typeof readScope!=='function'||typeof readSources!=='function'||typeof now!=='function')
    throw new TypeError('TRUSTED_READ_PORTS_REQUIRED');
  let generation=0, current=null;
  const close=()=>{generation++;current=null;return {closed:true};};
  async function open(screen) {
    const s=assertScreenContext(screen);
    const seq=++generation, stamp=now();
    const restricted=s.source.kind==='unsupported'||s.source.bindingState==='denied'||
      (s.source.kind==='bound-web' && (s.source.bindingState!=='bound'||!s.source.conversationRef));
    let scope;
    try{scope=await readScope();}catch{scope=null;}
    if(seq!==generation)return emptyPage(s,'stale',[],[],stamp);
    if(restricted||s.coverage.status==='denied'||!authorized(s,scope))
      return (current=emptyPage(s,'denied',[{readerId:'host',state:'denied'}],[],stamp));
    const focus=focusOf(s);
    if(s.focus.projectCandidates.length>0 && focus.projectRef===null)
      return (current=emptyPage(s,'partial',[{readerId:'project',state:'not-run'}],
        [{reason:'unresolved-identity',count:s.focus.projectCandidates.length}],stamp,focus));
    if(s.coverage.status==='unavailable')
      return (current=emptyPage(s,'unavailable',[{readerId:'screen',state:'unavailable'}],[],stamp,focus));
    // Query is bounded metadata; never pass raw draft text or web page HTML.
    let result;
    try{result=await readSources({contextId:s.contextId,contextRevision:s.revision,
      ownerRef:scope.ownerRef,worldRef:scope.worldRef,policyRevision:scope.policyRevision,
      focus:{objectRefs:s.focus.objectRefs.slice(0,12),projectRef:focus.projectRef,
        topicTerms:s.focus.topicTerms.slice(0,18)},limit:20});}
    catch{result={status:'unavailable',readerId:'hq',records:[]};}
    if(seq!==generation)return emptyPage(s,'stale',[{readerId:'screen',state:'stale'}],[],stamp,focus);
    let verified;
    try{verified=await readScope();}catch{verified=null;}
    if(seq!==generation)return emptyPage(s,'stale',[{readerId:'screen',state:'stale'}],[],stamp,focus);
    if(!authorized(s,verified)||scope.sessionRef!==verified.sessionRef||
       scope.deviceRef!==verified.deviceRef)
      return (current=emptyPage(s,'denied',[{readerId:'host',state:'denied'}],[],stamp));
    if(result?.contextRevision!==undefined && result.contextRevision!==s.revision)
      return (current=emptyPage(s,'stale',[{readerId:str(result.readerId,191)||'hq',state:'stale'}],[],stamp,focus));
    const readerId=valid(result?.readerId)?result.readerId:'hq';
    if(!['ready','partial'].includes(result?.status))
      return (current=emptyPage(s,'unavailable',[{readerId,state:result?.status==='denied'?'denied':'unavailable'}],[],stamp,focus));
    const input=Array.isArray(result.records)?result.records.slice(0,200):[];
    const seen=new Set(),items=[];
    for(const r of input) {
      const item=cleanRecord(r,s,verified);
      if(item&&!seen.has(item.id)){seen.add(item.id);items.push(item);}
    }
    const rejected=input.length-items.length;
    items.sort((a,b)=>(SCORE[a.status]-SCORE[b.status])||a.id.localeCompare(b.id,'en'));
    const draft=draftItem(s);
    if(draft && !seen.has(draft.id))items.push(draft);
    const capped=items.slice(0,20);
    const partial=result.status==='partial'||s.coverage.status==='partial'||
      s.source.kind==='bound-web'||s.source.contentCoverage==='currently-rendered-only'||
      rejected>0||items.length>20;
    const revisions=(Array.isArray(result.sourceRevisions)?result.sourceRevisions:[])
      .filter(r=>nativeId(r?.sourceId)&&nativeId(r?.revision))
      .slice(0,20).map(r=>({sourceId:r.sourceId,revision:r.revision,
        freshness:['current','stale','unknown','unavailable'].includes(r.freshness)?r.freshness:'unknown'}));
    const projectionRevision='projection:'+hash(s.contextId+':'+s.revision+':'+
      revisions.map(r=>r.sourceId+':'+r.revision).join('|')+':'+capped.map(x=>x.id+':'+x.source.revision).join('|'));
    const omission=[];
    if(rejected)omission.push({reason:'scope-filtered',count:rejected});
    if(items.length>20)omission.push({reason:'budget',count:items.length-20});
    const page={schema:PAGE_SCHEMA,
      pageId:'auto:'+hash(s.contextId+':'+s.scope.ownerRef+':'+s.scope.worldRef),
      contextId:s.contextId,contextRevision:s.revision,projectionRevision,asOf:stamp,
      status:partial?'partial':'ready',focus,
      sections:capped.length?[{id:'sources',title:'Source-linked context',
        itemIds:capped.map(x=>x.id),status:partial?'partial':'ready'}]:[],
      items:capped,coverage:{state:partial?'partial':'complete-for-authorized-registered-sources',
        screenCoverage:s.source.kind==='bound-web'?'currently-rendered-only':s.source.contentCoverage,
        corpusCoverage:partial?'bounded-and-not-exhaustive':'authorized-registered-sources-only',
        readerStates:[{readerId,state:result.status}]},
      sourceRevisions:revisions,omissions:omission,readOnly:true,effectPerformed:false};
    current=page; return page;
  }
  return Object.freeze({open,close,current:()=>current});
}