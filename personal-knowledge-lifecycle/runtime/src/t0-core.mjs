// Labs staging implementation. Host owns authority, persistence, and effect receipts.
// This module does not drive Chrome, contact ChatGPT, or write a second graph.
import { createHash, randomUUID } from 'node:crypto';

export const SCHEMA = 'vivim.personal-acquisition/1';
export const LANES = Object.freeze(['export','profile','index','bootstrap','thread','archive']);
export const OPERATIONS = Object.freeze(['request_export','read_profile','read_index','run_bootstrap','capture_thread','import_archive','disclose_context']);
const LANE_OP = Object.freeze({export:'request_export',profile:'read_profile',index:'read_index',bootstrap:'run_bootstrap',thread:'capture_thread',archive:'import_archive'});
const STATES = Object.freeze(['ready','pending','action_required','uncertain','paused','complete','failed','revoked']);
const fail = (code, message) => Object.assign(new Error(message ?? code), {code});
const sha = value => createHash('sha256').update(value).digest('hex');
const plain = value => JSON.parse(JSON.stringify(value));
const check = (s, label, limit=256) => {
  if (typeof s !== 'string' || !s.trim() || s.length > limit || /[\x00-\x1f]/.test(s)) throw fail('INVALID_INPUT', label);
  return s;
};
const nowISO = now => {
  const time = now();
  const value = time instanceof Date ? time.toISOString() : new Date(time).toISOString();
  if (!value || value === 'Invalid Date') throw fail('CLOCK_INVALID');
  return value;
};
const initial = (worldId, accountId, at) => ({
  schema:SCHEMA,worldId,accountId,createdAt:at,revision:0,grants:{},
  lanes:Object.fromEntries(LANES.map(lane=>[lane,{status:'ready',cursor:null,processed:0,duplicates:0,receipt:null}])),
  evidence:{}, actions:{}, lastUpdatedAt:at
});

/**
 * Requires a Host-owned, transactional durable store, not a separate Labs database:
 * store.transact(key, updater) must atomically read, call updater(current|null)
 * and durably persist its returned JSON value. It MUST NOT retry a callback after
 * any external effect. All callbacks in this module are pure state transitions.
 */
export function createT0Coordinator({store,worldId,accountId,now=Date.now}={}) {
  check(worldId,'worldId');check(accountId,'accountId');
  if(typeof store?.transact!=='function'||typeof store?.read!=='function')throw fail('STORE_REQUIRED');
  const key='personal-acquisition.'+sha(worldId+'\0'+accountId);
  const timestamp=()=>nowISO(now);
  const read=()=>{const s=store.read(key);if(s && (s.schema!==SCHEMA||s.accountId!==accountId||s.worldId!==worldId))throw fail('SCOPE_MISMATCH');return s?plain(s):null;};
  const mutate=fn=>store.transact(key,raw=>{
    const s=raw?plain(raw):initial(worldId,accountId,timestamp());
    if(s.schema!==SCHEMA||s.worldId!==worldId||s.accountId!==accountId)throw fail('SCOPE_MISMATCH');
    const value=fn(s);
    value.revision+=1;value.lastUpdatedAt=timestamp();return value;
  });
  const assertGrant=(s,operation)=>{const g=s.grants[operation];if(!g||g.revokedAt||g.accountId!==accountId||g.worldId!==worldId||(g.expiresAt&&Date.parse(g.expiresAt)<=Date.parse(timestamp())))throw fail('GRANT_REQUIRED',operation);return g;};
  function start(){mutate(s=>s);return status();}
  function grant({operation,grantId=randomUUID(),expiresAt=null,scope='selected',approvedByOwner=false}={}){
    if(!OPERATIONS.includes(operation)||!approvedByOwner)throw fail('OWNER_APPROVAL_REQUIRED');
    check(grantId,'grantId');check(scope,'scope',128);
    if(expiresAt&&(!Number.isFinite(Date.parse(expiresAt))||Date.parse(expiresAt)<=Date.parse(timestamp())))throw fail('INVALID_EXPIRY');
    mutate(s=>{const existing=s.grants[operation];if(existing&&!existing.revokedAt)throw fail('GRANT_ALREADY_ACTIVE');s.grants[operation]={grantId,operation,worldId,accountId,scope,grantedAt:timestamp(),expiresAt,revokedAt:null};return s;});
    return status();
  }
  function revoke(operation){
    if(!OPERATIONS.includes(operation))throw fail('INVALID_OPERATION');
    mutate(s=>{
      if(s.grants[operation]&&!s.grants[operation].revokedAt)s.grants[operation].revokedAt=timestamp();
      for(const [lane,op] of Object.entries(LANE_OP))if(op===operation)s.lanes[lane].status='revoked';
      for(const a of Object.values(s.actions))if(a.operation===operation&&a.status==='pending')a.status='uncertain';
      return s;
    });return status();
  }
  // Requests are proposals, not external effects. An adapter receives these
  // only through an approved Host integration and must record outcome separately.
  function prepare({lane,actionId=randomUUID(),payloadDigest}={}){
    if(!LANES.includes(lane))throw fail('INVALID_LANE');
    check(actionId,'actionId');if(!/^[a-f0-9]{64}$/.test(payloadDigest??''))throw fail('INVALID_DIGEST');
    const operation=LANE_OP[lane];
    mutate(s=>{
      assertGrant(s,operation);
      const previous=s.actions[actionId];
      if(previous){if(previous.lane!==lane||previous.payloadDigest!==payloadDigest)throw fail('ACTION_CONFLICT');return s;}
      if(['pending','uncertain'].includes(s.lanes[lane].status))throw fail('LANE_IN_FLIGHT');
      s.actions[actionId]={actionId,lane,operation,payloadDigest,status:'pending',createdAt:timestamp(),receipt:null};
      s.lanes[lane].status='pending';return s;
    });return status().actions[actionId];
  }
  function reconcile({actionId,outcome,receipt}={}){
    check(actionId,'actionId');
    if(!['confirmed','uncertain','blocked'].includes(outcome))throw fail('INVALID_OUTCOME');
    if(outcome==='confirmed'&&(!receipt||typeof receipt!=='object'||receipt.actionId!==actionId||receipt.verified!==true||typeof receipt.effectRef!=='string'||!receipt.effectRef))throw fail('VERIFIED_RECEIPT_REQUIRED');
    mutate(s=>{
      const a=s.actions[actionId];if(!a)throw fail('ACTION_NOT_FOUND');
      if(a.status!=='pending'){if(a.status==='complete'&&outcome==='confirmed'&&a.receipt?.effectRef===receipt?.effectRef)return s;throw fail('RECONCILIATION_REQUIRED');}
      if(outcome==='confirmed')assertGrant(s,a.operation);
      a.status=outcome==='confirmed'?'complete':outcome==='uncertain'?'uncertain':'action_required';
      a.receipt=outcome==='confirmed'?{actionId,effectRef:receipt.effectRef,verified:true,observedAt:timestamp()}:null;
      s.lanes[a.lane].status=a.status;s.lanes[a.lane].receipt=a.receipt;return s;
    });return status().actions[actionId];
  }
  // Evidence intake is separate from browser effects; an actual Host evidence writer
  // must consume the receipt and content before treating anything as graph-backed.
  function recordBatch({lane,sourceId,batchId,sourceRevision,items,acceptedByHost=false}={}){
    if(!LANES.includes(lane))throw fail('INVALID_LANE');
    check(sourceId,'sourceId');check(batchId,'batchId');check(sourceRevision,'sourceRevision');
    if(!acceptedByHost)throw fail('HOST_ACCEPTANCE_REQUIRED');
    if(!Array.isArray(items)||items.length>256)throw fail('BATCH_LIMIT');
    const rows=items.map(item=>{
      check(item.nativeId,'nativeId');if(!/^[a-f0-9]{64}$/.test(item.contentDigest??''))throw fail('INVALID_DIGEST');
      return {nativeId:item.nativeId,contentDigest:item.contentDigest,kind:check(item.kind,'kind',64)};
    });
    mutate(s=>{
      assertGrant(s,LANE_OP[lane]);
      const batchKey=sha([lane,sourceId,sourceRevision,batchId].join('\0'));
      if(s.evidence['batch:'+batchKey]){if(s.evidence['batch:'+batchKey]!==sha(JSON.stringify(rows)))throw fail('BATCH_CONFLICT');return s;}
      let added=0,duplicate=0;
      for(const item of rows){
        const identity=sha([accountId,sourceId,item.nativeId,item.contentDigest].join('\0'));
        if(s.evidence['item:'+identity])duplicate++;
        else{s.evidence['item:'+identity]={sourceId,nativeId:item.nativeId,contentDigest:item.contentDigest,kind:item.kind,sourceRevision,at:timestamp()};added++;}
      }
      s.evidence['batch:'+batchKey]=sha(JSON.stringify(rows));
      s.lanes[lane].processed+=added;s.lanes[lane].duplicates+=duplicate;s.lanes[lane].cursor=batchId;
      return s;
    });return status().lanes[lane];
  }
  function status(){
    const s=read();if(!s)return null;
    // Do not leak internal evidence IDs to presentation callers.
    return {schema:s.schema,worldId:s.worldId,accountId:s.accountId,revision:s.revision,lastUpdatedAt:s.lastUpdatedAt,grants:plain(s.grants),lanes:plain(s.lanes),actions:plain(s.actions),evidenceCount:Object.keys(s.evidence).filter(k=>k.startsWith('item:')).length};
  }
  return Object.freeze({start,grant,revoke,prepare,reconcile,recordBatch,status});
}
